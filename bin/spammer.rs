use alloy::consensus::{SignableTransaction, Signed, TxEnvelope};
use alloy::eips::eip2718::Encodable2718;
use alloy::network::{Ethereum, TransactionBuilder};
use alloy::primitives::{Address, Bytes, TxHash, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::{SignerSync, local::PrivateKeySigner};
use commitments::client::CommitmentsHttpClient;
use eyre::{Result, WrapErr};
use lookahead::utils::current_slot_estimate;
use reqwest::Url;
use serde::Deserialize;
use std::time::Duration;
use tracing::{error, info};

use commit_boost::prelude::Chain;

use commitments::types::{CommitmentRequest, SignedCommitment};
use inclusion::constants::INCLUSION_COMMITMENT_TYPE;
use inclusion::types::InclusionPayload;

pub const SLOTS_IN_FUTURE_TO_SEND_COMMITMENT_REQUEST: u64 = 2;

/// Configuration for the spammer
#[derive(Debug, Deserialize)]
struct SpammerConfig {
	/// Mode: "one-shot" or "continuous"
	mode: String,
	/// Gateway RPC (commitments) host
	gateway_host: String,
	/// Gateway RPC (commitments) port
	gateway_port: u16,
	/// Execution client host
	execution_client_host: String,
	/// Execution client port
	execution_client_port: u16,
	/// Slasher contract address (optional, random if not provided)
	slasher_address: Option<String>,
	/// Chain spec
	chain: Chain,
}

/// Generate a valid signed transaction, returning encoded bytes, tx hash, and nonce
async fn generate_signed_transaction(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<(Bytes, TxHash, u64)> {
	// Create execution client
	let execution_client_url =
		Url::parse(format!("http://{}:{}", config.execution_client_host, config.execution_client_port).as_str())?;
	let execution_client = ProviderBuilder::new().network::<Ethereum>().connect_http(execution_client_url);

	let nonce = execution_client.get_transaction_count(signer.address()).await?;
	let chain_id = config.chain.id().to::<u64>();

	let latest_block = execution_client
		.get_block_by_number(alloy::eips::BlockNumberOrTag::Latest)
		.await?
		.ok_or_else(|| eyre::eyre!("Failed to get latest block"))?;
	let base_fee =
		latest_block.header.base_fee_per_gas.ok_or_else(|| eyre::eyre!("No base fee in block (pre-EIP-1559?)"))?;
	let max_fee_per_gas = base_fee * 100000; // A lot more to avoid getting outbid

	let balance = execution_client.get_balance(signer.address()).await?;
	info!("Sender address: {:?}", signer.address());
	info!("Sender balance: {} wei", balance);
	info!("Base fee: {} wei, max_fee_per_gas: {} wei", base_fee, max_fee_per_gas);
	info!("Max cost: {} wei", U256::from(21000) * U256::from(max_fee_per_gas) + U256::from(1));

	// Create EIP-1559 transaction with random recipient
	let tx = execution_client
		.transaction_request()
		.from(signer.address())
		.to(Address::random())
		.value(U256::from(1))
		.gas_limit(21000)
		.nonce(nonce)
		.max_priority_fee_per_gas(max_fee_per_gas.into())
		.max_fee_per_gas(max_fee_per_gas.into())
		.with_chain_id(chain_id)
		.build_1559()?;

	// Sign the transaction hash
	let signature_hash = tx.signature_hash();
	let signature = signer.sign_hash_sync(&signature_hash).wrap_err("Failed to sign transaction")?;

	// Create signed transaction envelope
	let signed_tx = Signed::new_unhashed(tx, signature);
	let tx_envelope = TxEnvelope::Eip1559(signed_tx);

	// Capture tx hash before encoding
	let tx_hash = *tx_envelope.tx_hash();

	// RLP encode
	let mut encoded = Vec::new();
	tx_envelope.encode_2718(&mut encoded);

	Ok((Bytes::from(encoded), tx_hash, nonce))
}

/// Create a commitment request, returning the request and target slot
fn create_commitment_request(config: &SpammerConfig, signed_tx: Bytes) -> Result<(CommitmentRequest, u64)> {
	// Get current slot
	let current_slot = current_slot_estimate(config.chain.genesis_time_sec());

	// Send the commitment for a future slot
	let target_slot = current_slot + SLOTS_IN_FUTURE_TO_SEND_COMMITMENT_REQUEST;

	// Create inclusion payload
	let inclusion_payload = InclusionPayload { slot: target_slot, signed_tx };

	// ABI encode the payload
	let payload = inclusion_payload.abi_encode().wrap_err("Failed to encode inclusion payload")?;

	// Parse or generate slasher address
	let slasher = if let Some(addr_str) = &config.slasher_address {
		addr_str.parse::<Address>().wrap_err("Failed to parse slasher address")?
	} else {
		Address::random()
	};

	Ok((CommitmentRequest { commitment_type: INCLUSION_COMMITMENT_TYPE, payload, slasher }, target_slot))
}

/// Send a commitment request via RPC
async fn send_commitment_request(gateway_url: Url, request: &CommitmentRequest) -> Result<SignedCommitment> {
	let commitments_client = CommitmentsHttpClient::new(gateway_url)?;
	commitments_client.commitment_request(request.clone()).await
}

async fn create_and_send_commitment_request(
	config: &SpammerConfig,
	signer: &PrivateKeySigner,
) -> Result<(SignedCommitment, TxHash, u64, u64)> {
	let gateway_url = Url::parse(format!("http://{}:{}", config.gateway_host, config.gateway_port).as_str())?;
	let (tx, tx_hash, nonce) = generate_signed_transaction(config, signer).await?;
	let (request, target_slot) = create_commitment_request(config, tx)?;
	let response = send_commitment_request(gateway_url, &request).await?;
	Ok((response, tx_hash, target_slot, nonce))
}

/// Run in one-shot mode
async fn run_one_shot(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<()> {
	match create_and_send_commitment_request(config, signer).await {
		Ok((response, tx_hash, target_slot, nonce)) => {
			info!("Commitment request successful!");
			info!("Target slot: {}, nonce: {}, tx_hash: {:?}, request_hash: {:?}", target_slot, nonce, tx_hash, response.commitment.request_hash);
		}
		Err(e) => {
			error!("✗ Failed to create and send commitment request: {}", e);
		}
	}
	Ok(())
}

/// Run in continuous mode (one transaction per slot)
async fn run_continuous(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<()> {
	info!("Running in continuous mode (one transaction per slot)");

	let mut last_sent_slot: Option<u64> = None;
	let mut shutdown = Box::pin(common::utils::wait_for_signal());

	loop {
		let current_slot = lookahead::utils::current_slot(&config.chain);

		// Only send if we haven't sent for this slot yet
		if last_sent_slot != Some(current_slot) {
			match create_and_send_commitment_request(config, signer).await {
				Ok((response, tx_hash, target_slot, nonce)) => {
					info!(
						"Sent at slot {}, target slot {}: nonce {}, tx_hash {:?}, request_hash {:?}",
						current_slot, target_slot, nonce, tx_hash, response.commitment.request_hash
					);
					last_sent_slot = Some(current_slot);
				}
				Err(e) => {
					error!("Slot {}: Failed: {}", current_slot, e);
					// Still mark as sent to avoid retry spam on persistent errors
					last_sent_slot = Some(current_slot);
				}
			}
		}

		// Wait until the next slot
		let sleep_ms = lookahead::utils::time_until_next_slot_ms(&config.chain);
		let sleep_duration = Duration::from_millis(sleep_ms.max(100) as u64);

		tokio::select! {
			_ = tokio::time::sleep(sleep_duration) => {}
			_ = &mut shutdown => {
				info!("Shutdown signal received, stopping spammer loop");
				break;
			}
		}
	}
	Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
	// Setup logging
	common::logging::setup_logging(&std::env::var("RUST_LOG").expect("RUST_LOG environment variable not set"))?;

	let sender_private_key =
		std::env::var("SENDER_PRIVATE_KEY").expect("SENDER_PRIVATE_KEY environment variable not set");

	let config_path = std::env::var("CONFIG_PATH").expect("CONFIG_PATH environment variable not set");

	info!("Loading configuration from: {}", config_path);

	// Load configuration
	let config_content = std::fs::read_to_string(config_path).wrap_err("Failed to read config file")?;
	let config: SpammerConfig = toml::from_str(&config_content).wrap_err("Failed to parse config file")?;

	info!("Configuration loaded successfully");
	info!("  Mode: {}", config.mode);
	info!("  Gateway URL: {}:{}", config.gateway_host, config.gateway_port);
	info!("  Chain ID: {}", config.chain.id());

	// Parse sender private key
	let signer = sender_private_key.parse::<PrivateKeySigner>().wrap_err("Failed to parse sender private key")?;
	let sender_address = signer.address();
	info!("Sender address: {:?}", sender_address);

	// Run based on mode
	match config.mode.as_str() {
		"one-shot" => run_one_shot(&config, &signer).await?,
		"continuous" => run_continuous(&config, &signer).await?,
		_ => {
			return Err(eyre::eyre!("Invalid mode '{}'. Must be 'one-shot' or 'continuous'", config.mode));
		}
	}

	Ok(())
}
