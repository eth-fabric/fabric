use alloy::consensus::{SignableTransaction, Signed, TxEip1559, TxEnvelope};
use alloy::eips::eip2718::Encodable2718;
use alloy::primitives::{Address, Bytes, TxKind, U256};
use alloy::signers::{SignerSync, local::PrivateKeySigner};
use commitments::client::CommitmentsHttpClient;
use eyre::{Result, WrapErr};
use lookahead::utils::current_slot_estimate;
use serde::Deserialize;
use std::time::Duration;
use tokio::time;
use tracing::{error, info};

use commit_boost::prelude::Chain;

use commitments::types::{CommitmentRequest, SignedCommitment};
use inclusion::constants::INCLUSION_COMMITMENT_TYPE;
use inclusion::types::InclusionPayload;
use urc::utils::get_commitment_request_signing_root;

/// Configuration for the spammer
#[derive(Debug, Deserialize)]
struct SpammerConfig {
    /// Mode: "one-shot" or "continuous"
    mode: String,
    /// Gateway RPC (commitments) host
    gateway_host: String,
    /// Gateway RPC (commitments) port
    gateway_port: u16,
    /// Interval between requests in seconds (only used in continuous mode)
    interval_secs: u64,
    /// Sender private key (must have ETH balance for test transactions)
    sender_private_key: String,
    /// Slasher contract address (optional, random if not provided)
    slasher_address: Option<String>,
    /// Chain ID for transactions
    chain: Chain,
}

/// Generate a valid signed transaction
fn generate_signed_transaction(
    config: &SpammerConfig,
    signer: &PrivateKeySigner,
    nonce: u64,
) -> Result<Bytes> {
    // Create EIP-1559 transaction with random recipient
    let tx = TxEip1559 {
        chain_id: config
            .chain
            .id()
            .try_into()
            .expect("Chain ID conversion failed"),
        nonce,
        gas_limit: 21000,
        max_fee_per_gas: 20000000000,
        max_priority_fee_per_gas: 2000000000,
        to: TxKind::Call(Address::random()), // Random recipient address
        value: U256::from(100000000),
        input: Bytes::new(),
        access_list: Default::default(),
    };

    // Sign the transaction
    let encoded_tx = tx.encoded_for_signing();
    let signature = signer
        .sign_message_sync(&encoded_tx)
        .wrap_err("Failed to sign transaction")?;

    // Create signed transaction envelope
    let signed_tx = Signed::new_unhashed(tx, signature);
    let tx_envelope = TxEnvelope::Eip1559(signed_tx);

    // RLP encode
    let mut encoded = Vec::new();
    tx_envelope.encode_2718(&mut encoded);

    Ok(Bytes::from(encoded))
}

/// Create a commitment request
fn create_commitment_request(
    config: &SpammerConfig,
    signed_tx: Bytes,
) -> Result<CommitmentRequest> {
    // Get current slot
    let current_slot = current_slot_estimate(config.chain.genesis_time_sec());

    // Create inclusion payload
    let inclusion_payload = InclusionPayload {
        slot: current_slot,
        signed_tx,
    };

    // ABI encode the payload
    let payload = inclusion_payload
        .abi_encode()
        .wrap_err("Failed to encode inclusion payload")?;

    // Parse or generate slasher address
    let slasher = if let Some(addr_str) = &config.slasher_address {
        addr_str
            .parse::<Address>()
            .wrap_err("Failed to parse slasher address")?
    } else {
        Address::random()
    };

    Ok(CommitmentRequest {
        commitment_type: INCLUSION_COMMITMENT_TYPE,
        payload,
        slasher,
    })
}

/// Send a commitment request via RPC
async fn send_commitment_request(
    gateway_url: &str,
    request: &CommitmentRequest,
) -> Result<SignedCommitment> {
    let commitments_client = CommitmentsHttpClient::new(gateway_url)?;
    commitments_client.commitment_request(request.clone()).await
}

/// Run in one-shot mode
async fn run_one_shot(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<()> {
    info!("Running in one-shot mode");

    // Generate transaction with nonce 0
    let signed_tx = generate_signed_transaction(config, signer, 0)?;
    info!("Generated signed transaction ({} bytes)", signed_tx.len());

    // Create commitment request
    let request = create_commitment_request(config, signed_tx)?;
    let signing_hash = get_commitment_request_signing_root(&request);
    info!("Created commitment request with hash: {:?}", signing_hash);

    // Send request
    let gateway_url = format!("http://{}:{}", config.gateway_host, config.gateway_port);
    info!("Sending commitment request to {}", gateway_url);
    let response = send_commitment_request(gateway_url.as_str(), &request).await?;

    info!("✓ Commitment request successful!");
    info!("  Request hash: {:?}", response.commitment.request_hash);
    info!("  Commitment type: {}", response.commitment.commitment_type);
    info!("  Slasher: {:?}", response.commitment.slasher);

    Ok(())
}

/// Run in continuous mode
async fn run_continuous(config: &SpammerConfig, signer: &PrivateKeySigner) -> Result<()> {
    info!(
        "Running in continuous mode (interval: {}s)",
        config.interval_secs
    );

    let mut interval = time::interval(Duration::from_secs(config.interval_secs));
    let mut nonce = 0u64;

    let gateway_url = format!("http://{}:{}", config.gateway_host, config.gateway_port);

    let mut shutdown = Box::pin(common::utils::wait_for_signal());
    loop {
        tokio::select! {
            _ = interval.tick() => {
                info!("--- Sending commitment request #{} ---", nonce + 1);
                match generate_signed_transaction(config, signer, nonce) {
                    Ok(signed_tx) => {
                        info!("Generated signed transaction ({} bytes)", signed_tx.len());
                        match create_commitment_request(config, signed_tx) {
                            Ok(request) => {
                                let signing_root = get_commitment_request_signing_root(&request);
                                info!("Request hash: {:?}", signing_root);
                                match send_commitment_request(gateway_url.as_str(), &request).await {
                                    Ok(response) => {
                                        info!("Commitment request successful!");
                                        info!("Signing root: {:?}", response.commitment.request_hash);
                                        nonce += 1;
                                    }
                                    Err(e) => {
                                        error!("✗ Failed to send commitment request: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("✗ Failed to create commitment request: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("✗ Failed to generate signed transaction: {}", e);
                    }
                }
            }
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
    // Read env vars
    let log_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
    let config_path =
        std::env::var("CONFIG_PATH").expect("CONFIG_PATH environment variable not set");

    // Setup logging
    common::logging::setup_logging(&log_level)?;

    info!("Loading configuration from: {}", config_path);

    // Load configuration
    let config_content =
        std::fs::read_to_string(config_path).wrap_err("Failed to read config file")?;
    let config: SpammerConfig =
        toml::from_str(&config_content).wrap_err("Failed to parse config file")?;

    info!("Configuration loaded successfully");
    info!("  Mode: {}", config.mode);
    info!(
        "  Gateway URL: {}:{}",
        config.gateway_host, config.gateway_port
    );
    info!("  Chain ID: {}", config.chain.id());

    // Parse sender private key
    let signer = config
        .sender_private_key
        .parse::<PrivateKeySigner>()
        .wrap_err("Failed to parse sender private key")?;
    let sender_address = signer.address();
    info!("Sender address: {:?}", sender_address);

    // Run based on mode
    match config.mode.as_str() {
        "one-shot" => run_one_shot(&config, &signer).await?,
        "continuous" => run_continuous(&config, &signer).await?,
        _ => {
            return Err(eyre::eyre!(
                "Invalid mode '{}'. Must be 'one-shot' or 'continuous'",
                config.mode
            ));
        }
    }

    Ok(())
}
