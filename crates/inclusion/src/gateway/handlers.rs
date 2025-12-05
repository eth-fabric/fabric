use crate::metrics::{RPC_CALLS_TOTAL, RPC_LATENCY_SECONDS, RPC_RESPONSES_TOTAL};
use jsonrpsee::Extensions;
use jsonrpsee::core::RpcResult;
use std::time::Instant;
use tracing::{error, info, instrument};

use super::utils;
use crate::CommitmentsServerState;
use alloy::primitives::B256;
use commit_boost::prelude::commit::request::EncryptionScheme;
use common::config::GatewayConfig;
use common::constants::INCLUSION_COMMITMENT_TYPE;
use common::types::commitments::Offering;
use common::types::{CommitmentRequest, FeeInfo, SignedCommitment, SlotInfo, SlotInfoResponse};

#[instrument(name = "commitment_request", skip(_context, _extensions))]
pub async fn commitment_request_handler<T: GatewayConfig>(
	params: jsonrpsee::types::Params<'_>,
	_context: std::sync::Arc<CommitmentsServerState<T>>,
	_extensions: Extensions,
) -> RpcResult<SignedCommitment> {
	const METHOD: &str = "commitmentRequest";
	RPC_CALLS_TOTAL.with_label_values(&[METHOD]).inc();
	let start = Instant::now();
	info!("Processing commitment request");
	let request: CommitmentRequest = params.one()?;

	// Validate the commitment request and extract the inclusion payload
	let inclusion_payload = match utils::validate_commitment_request(&request) {
		Ok(payload) => payload,
		Err(e) => {
			error!("Invalid commitment request: {}", e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid commitment request",
				Some(format!("{}", e)),
			));
		}
	};

	let slot = inclusion_payload.slot;

	// Get the delegation for this slot
	let signed_delegation = match _context.delegations_database().get_delegation_for_slot(slot) {
		Ok(Some(delegation)) => {
			info!("Found delegation for slot {}, proceeding with commitment", slot);
			delegation
		}
		Ok(None) => {
			error!("No delegation found for slot {}, cannot create commitment", slot);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"No delegation for slot",
				Some(format!("No delegation found for slot {}", slot)),
			));
		}
		Err(e) => {
			error!("Failed to get delegation for slot {}: {}", slot, e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32603, // Internal error
				"Failed to get delegation",
				Some(format!("{}", e)),
			));
		}
	};

	// Parse module_signing_id from config
	let module_signing_id = {
		let config_guard = _context.commit_config.lock().await;
		match config_guard.extra.module_signing_id().parse::<B256>() {
			Ok(id) => id,
			Err(e) => {
				error!("Failed to parse module_signing_id from config: {}", e);
				RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
				RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
				return Err(jsonrpsee::types::error::ErrorObject::owned(
					-32603, // Internal error
					"Invalid module_signing_id configuration",
					Some(format!("{}", e)),
				));
			}
		}
	};

	// Create and sign the commitment using ECDSA with commit-boost
	let commit_config = _context.commit_config.clone();
	let committer_address = signed_delegation.message.committer;
	let signed_commitment =
		match utils::create_signed_commitment(&request, commit_config, committer_address, &module_signing_id).await {
			Ok(commitment) => commitment,
			Err(e) => {
				error!("Failed to create signed commitment: {}", e);
				RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
				RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
				return Err(jsonrpsee::types::error::ErrorObject::owned(
					-32602, // Invalid params
					"Failed to create signed commitment",
					Some(format!("{}", e)),
				));
			}
		};

	// Create the constraint before storing anything to avoid race conditions
	let constraint = match utils::create_constraint_from_commitment_request(&request, slot) {
		Ok(constraint) => constraint,
		Err(e) => {
			error!("Failed to create constraint from commitment request: {}", e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32603, // Internal error
				"Failed to create constraint",
				Some(format!("{}", e)),
			));
		}
	};

	// Store both commitment and constraint atomically to prevent race conditions
	let request_hash = signed_commitment.commitment.request_hash;
	if let Err(e) = _context.commitments_database().store_commitment_and_constraint(
		slot,
		&request_hash,
		&signed_commitment,
		&constraint,
	) {
		error!("Failed to store commitment and constraint atomically: {}", e);
		RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
		RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
		return Err(jsonrpsee::types::error::ErrorObject::owned(
			-32603, // Internal error
			"Failed to store commitment and constraint",
			Some(format!("{}", e)),
		));
	}

	info!("Commitment request processed successfully with constraint saved");
	RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "ok"]).inc();
	RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
	Ok(signed_commitment)
}

#[instrument(name = "commitment_result", skip(_context, _extensions))]
pub fn commitment_result_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: &CommitmentsServerState<T>,
	_extensions: &Extensions,
) -> RpcResult<SignedCommitment> {
	const METHOD: &str = "commitmentResult";
	RPC_CALLS_TOTAL.with_label_values(&[METHOD]).inc();
	let start = Instant::now();
	info!("Processing commitment result request");
	let request_hash: B256 = params.one()?;

	// Retrieve the commitment from the database using just the request hash
	match _context.commitments_database().get_commitment_by_hash(&request_hash) {
		Ok(Some(signed_commitment)) => {
			info!("Commitment result request processed successfully");
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "ok"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			Ok(signed_commitment)
		}
		Ok(None) => {
			error!("Commitment not found for request hash: {}", request_hash);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Commitment not found",
				Some(format!("No commitment found for request hash: {}", request_hash)),
			))
		}
		Err(e) => {
			error!("Failed to retrieve commitment from database: {}", e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			Err(jsonrpsee::types::error::ErrorObject::owned(
				-32603, // Internal error
				"Database error",
				Some(format!("{}", e)),
			))
		}
	}
}

#[instrument(name = "slots", skip(_context, _extensions))]
pub fn slots_handler<T>(
	_params: jsonrpsee::types::Params<'_>,
	_context: &CommitmentsServerState<T>,
	_extensions: &Extensions,
) -> RpcResult<SlotInfoResponse> {
	const METHOD: &str = "slots";
	RPC_CALLS_TOTAL.with_label_values(&[METHOD]).inc();
	let start = Instant::now();
	info!("Processing slots request");

	// Get current slot
	let current_slot = _context.slot_timer().get_current_slot();
	info!("Current slot: {}", current_slot);

	// Get chain ID once from commit config
	let chain_id_uint = _context
		.commit_config
		.try_lock()
		.map_err(|_| {
			jsonrpsee::types::error::ErrorObject::owned(
				-32603, // Internal error
				"Failed to access commit config",
				Some("Could not acquire lock on commit config".to_string()),
			)
		})?
		.chain
		.id();

	// Convert Uint<256> to u64
	let chain_id = chain_id_uint.to::<u64>();

	// Get all delegated slots in range with a single database query
	let delegated_slots =
		match _context.delegations_database().get_delegated_slots_in_range(current_slot, current_slot + 64) {
			Ok(slots) => slots,
			Err(e) => {
				error!("Failed to get delegated slots: {}", e);
				RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
				RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
				return Err(jsonrpsee::types::error::ErrorObject::owned(
					-32603, // Internal error
					"Failed to get delegated slots",
					Some(format!("{}", e)),
				));
			}
		};

	// Build slot info for each delegated slot
	let mut slots = Vec::new();
	for slot in delegated_slots {
		info!("Found delegation for slot {}", slot);

		// Create offering with chain ID and commitment type
		let offering = Offering { chain_id, commitment_types: vec![INCLUSION_COMMITMENT_TYPE] };

		// Create slot info
		let slot_info = SlotInfo { slot, offerings: vec![offering] };

		slots.push(slot_info);
	}

	info!("Found {} slots with delegations", slots.len());
	let response = SlotInfoResponse { slots };

	info!("Slots request processed successfully");
	RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "ok"]).inc();
	RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
	Ok(response)
}

#[instrument(name = "fee", skip(_context, _extensions))]
pub async fn fee_handler<T: GatewayConfig>(
	params: jsonrpsee::types::Params<'_>,
	_context: std::sync::Arc<CommitmentsServerState<T>>,
	_extensions: Extensions,
) -> RpcResult<FeeInfo> {
	const METHOD: &str = "fee";
	RPC_CALLS_TOTAL.with_label_values(&[METHOD]).inc();
	let start = Instant::now();
	info!("Processing fee request");
	let request: CommitmentRequest = params.one()?;

	// Use helper function to calculate fee using RPC calls
	let fee_info = match utils::calculate_fee_info(&request, &_context).await {
		Ok(fee_info) => fee_info,
		Err(e) => {
			error!("Failed to calculate fee info: {}", e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid request",
				Some(format!("{}", e)),
			));
		}
	};

	// Store fee information in the database
	let request_hash = match request.request_hash() {
		Ok(hash) => hash,
		Err(e) => {
			error!("Failed to calculate request hash: {}", e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid request",
				Some(format!("{}", e)),
			));
		}
	};

	if let Err(e) = _context.commitments_database().store_fee_info(&request_hash, &fee_info) {
		error!("Failed to store fee info in database: {}", e);
		RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
		RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
		return Err(jsonrpsee::types::error::ErrorObject::owned(
			-32603, // Internal error
			"Failed to store fee info",
			Some(format!("{}", e)),
		));
	}

	info!("Fee request processed successfully");
	RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "ok"]).inc();
	RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
	Ok(fee_info)
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GenerateProxyKeyRequest {
	pub bls_public_key: String,
	pub encryption_scheme: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GenerateProxyKeyResponse {
	pub signed_delegation: serde_json::Value, // Will contain the SignedProxyDelegation
	pub encryption_scheme: String,
}

#[instrument(name = "generate_proxy_key", skip(_context, _extensions))]
pub async fn generate_proxy_key_handler<T>(
	params: jsonrpsee::types::Params<'_>,
	_context: std::sync::Arc<CommitmentsServerState<T>>,
	_extensions: Extensions,
) -> RpcResult<GenerateProxyKeyResponse> {
	const METHOD: &str = "generateProxyKey";
	RPC_CALLS_TOTAL.with_label_values(&[METHOD]).inc();
	let start = Instant::now();
	info!("Processing generate proxy key request");
	let request: GenerateProxyKeyRequest = params.one()?;

	// Parse BLS public key
	let bls_public_key = match common::utils::bls_pubkey_from_hex(&request.bls_public_key) {
		Ok(key) => key,
		Err(e) => {
			error!("Invalid BLS public key format: {}", e);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid BLS public key format",
				Some(format!("{}", e)),
			));
		}
	};

	// Parse encryption scheme
	let encryption_scheme = match request.encryption_scheme.to_lowercase().as_str() {
		"ecdsa" => EncryptionScheme::Ecdsa,
		"bls" => EncryptionScheme::Bls,
		_ => {
			error!("Invalid encryption scheme: {}", request.encryption_scheme);
			RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
			RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
			return Err(jsonrpsee::types::error::ErrorObject::owned(
				-32602, // Invalid params
				"Invalid encryption scheme",
				Some(format!("Expected 'ecdsa' or 'bls', got: {}", request.encryption_scheme)),
			));
		}
	};

	// Generate proxy key
	let commit_config = _context.commit_config.clone();
	let signed_delegation = {
		let mut commit_config = commit_config.lock().await;
		match encryption_scheme {
			EncryptionScheme::Ecdsa => {
				let delegation =
					commit_config.signer_client.generate_proxy_key_ecdsa(bls_public_key).await.map_err(|e| {
						error!("Failed to generate ECDSA proxy key: {}", e);
						RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
						RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
						jsonrpsee::types::error::ErrorObject::owned(
							-32603, // Internal error
							"Failed to generate ECDSA proxy key",
							Some(format!("{}", e)),
						)
					})?;
				serde_json::to_value(delegation).map_err(|e| {
					error!("Failed to serialize ECDSA delegation: {}", e);
					jsonrpsee::types::error::ErrorObject::owned(
						-32603, // Internal error
						"Failed to serialize delegation",
						Some(format!("{}", e)),
					)
				})?
			}
			EncryptionScheme::Bls => {
				let delegation =
					commit_config.signer_client.generate_proxy_key_bls(bls_public_key).await.map_err(|e| {
						error!("Failed to generate BLS proxy key: {}", e);
						RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
						RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
						jsonrpsee::types::error::ErrorObject::owned(
							-32603, // Internal error
							"Failed to generate BLS proxy key",
							Some(format!("{}", e)),
						)
					})?;
				serde_json::to_value(delegation).map_err(|e| {
					error!("Failed to serialize BLS delegation: {}", e);
					RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "error"]).inc();
					RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
					jsonrpsee::types::error::ErrorObject::owned(
						-32603, // Internal error
						"Failed to serialize delegation",
						Some(format!("{}", e)),
					)
				})?
			}
		}
	};

	info!("Proxy key generated successfully");
	RPC_RESPONSES_TOTAL.with_label_values(&[METHOD, "ok"]).inc();
	RPC_LATENCY_SECONDS.with_label_values(&[METHOD]).observe(start.elapsed().as_secs_f64());
	Ok(GenerateProxyKeyResponse { signed_delegation, encryption_scheme: request.encryption_scheme })
}
