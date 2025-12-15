use alloy::consensus::TxEnvelope;
use alloy::primitives::{Address, B256, Bytes};
use alloy::rlp::Decodable;
use alloy::rpc::types::beacon::relay::SubmitBlockRequest as AlloySubmitBlockRequest;
use alloy::rpc::types::beacon::{BlsPublicKey, BlsSignature};
use axum::http::HeaderMap;
use common::utils::decode_pubkey;
use eyre::{Result, eyre};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

/// A constraint with its type and payload
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Constraint {
	pub constraint_type: u64,
	pub payload: Bytes,
}
/// A delegation message from proposer to gateway
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Delegation {
	pub proposer: BlsPublicKey,
	pub delegate: BlsPublicKey,
	pub committer: Address,
	pub slot: u64,
	pub metadata: Bytes,
}

/// A signed delegation with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SignedDelegation {
	pub message: Delegation,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: BlsSignature,
}

/// A constraints message containing multiple constraints
#[derive(Debug, Clone, Serialize, Deserialize, Default, Encode, Decode)]
pub struct ConstraintsMessage {
	pub proposer: BlsPublicKey,
	pub delegate: BlsPublicKey,
	pub slot: u64,
	pub constraints: Vec<Constraint>,
	pub receivers: Vec<BlsPublicKey>,
}

/// A signed constraints message with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SignedConstraints {
	pub message: ConstraintsMessage,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: BlsSignature,
}

/// Constraint capabilities response
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ConstraintCapabilities {
	pub constraint_types: Vec<u64>,
}

/// Proofs of constraint validity for a block
#[derive(Debug, Clone, Serialize, Deserialize, Default, Encode, Decode)]
pub struct ConstraintProofs {
	pub constraint_types: Vec<u64>,
	pub payloads: Vec<Bytes>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct SubmitBlockRequestWithProofs {
	#[serde(flatten)]
	pub message: AlloySubmitBlockRequest,
	pub proofs: ConstraintProofs,
}

impl SubmitBlockRequestWithProofs {
	pub fn slot(&self) -> u64 {
		self.message.bid_trace().slot
	}

	pub fn into_block_request(self) -> AlloySubmitBlockRequest {
		self.message
	}

	pub fn transactions(&self) -> Result<Vec<TxEnvelope>> {
		// Extract transaction bytes from the appropriate variant
		let tx_bytes_list = match &self.message {
			AlloySubmitBlockRequest::Electra(request) => {
				&request.execution_payload.payload_inner.payload_inner.transactions
			}
			AlloySubmitBlockRequest::Fulu(request) => {
				&request.execution_payload.payload_inner.payload_inner.transactions
			}
			AlloySubmitBlockRequest::Deneb(request) => {
				&request.execution_payload.payload_inner.payload_inner.transactions
			}
			AlloySubmitBlockRequest::Capella(request) => &request.execution_payload.payload_inner.transactions,
		};

		// Decode transactions
		let mut transactions = Vec::new();

		for tx_bytes in tx_bytes_list {
			let tx =
				TxEnvelope::decode(&mut tx_bytes.as_ref()).map_err(|e| eyre!("Failed to decode transaction: {}", e))?;
			transactions.push(tx);
		}

		if transactions.is_empty() {
			return Err(eyre!("No transactions in execution payload"));
		}

		Ok(transactions)
	}
}

pub struct AuthorizationContext {
	pub signature: Option<BlsSignature>,
	pub public_key: Option<BlsPublicKey>,
	pub nonce: Option<u64>,
	pub signing_id: Option<B256>,
}

/// Extract and parse BLS signature, public key, nonce, and signing_id from headers
impl AuthorizationContext {
	pub fn from_headers(headers: &HeaderMap) -> Result<AuthorizationContext> {
		// Extract headers
		let signature = match headers.get("X-Receiver-Signature") {
			Some(signature_header) => {
				let signature_str =
					signature_header.to_str().map_err(|_| eyre!("Invalid X-Receiver-Signature header"))?;
				let bls_signature = BlsSignature::new(
					signature_str
						.strip_prefix("0x")
						.unwrap_or(signature_str)
						.as_bytes()
						.try_into()
						.map_err(|e| eyre!("Invalid BLS signature: {:?}", e))?,
				);
				Some(bls_signature)
			}
			None => None,
		};

		let public_key = match headers.get("X-Receiver-PublicKey") {
			Some(public_key_header) => {
				let public_key_str =
					public_key_header.to_str().map_err(|_| eyre!("Invalid X-Receiver-PublicKey header"))?;
				let public_key = decode_pubkey(public_key_str)?;
				Some(public_key)
			}
			None => None,
		};

		let signing_id = match headers.get("X-Receiver-SigningId") {
			Some(signing_id_header) => {
				let signing_id_str =
					signing_id_header.to_str().map_err(|_| eyre!("Invalid X-Receiver-SigningId header"))?;
				let signing_id =
					B256::from_slice(signing_id_str.strip_prefix("0x").unwrap_or(signing_id_str).as_bytes());
				Some(signing_id)
			}
			None => None,
		};

		let nonce = match headers.get("X-Receiver-Nonce") {
			Some(nonce_header) => {
				let nonce_str = nonce_header.to_str().map_err(|_| eyre!("Invalid X-Receiver-Nonce header"))?;
				Some(nonce_str.parse::<u64>().map_err(|e| eyre!("Invalid nonce format: {}", e))?)
			}
			None => None,
		};

		Ok(AuthorizationContext { signature, public_key, nonce, signing_id })
	}
}
/// Response wrapper for GET /delegations
#[derive(Serialize, Deserialize)]
pub struct DelegationsResponse {
	pub delegations: Vec<SignedDelegation>,
}

/// Response wrapper for GET /constraints
#[derive(Serialize, Deserialize)]
pub struct ConstraintsResponse {
	pub constraints: Vec<SignedConstraints>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::Bytes;

	#[test]
	fn test_constraint_capabilities() {
		let capabilities = ConstraintCapabilities { constraint_types: vec![1, 2, 3, 4, 5] };

		assert_eq!(capabilities.constraint_types.len(), 5);
		assert_eq!(capabilities.constraint_types[0], 1);
		assert_eq!(capabilities.constraint_types[4], 5);
	}

	/// Test serialization and deserialization of constraint types
	#[test]
	fn test_constraint_serialization() {
		let constraint = Constraint { constraint_type: 1, payload: Bytes::from(vec![1, 2, 3, 4, 5, 6, 7, 8]) };

		// Test JSON serialization
		let json = serde_json::to_string(&constraint).unwrap();
		let deserialized: Constraint = serde_json::from_str(&json).unwrap();

		assert_eq!(constraint.constraint_type, deserialized.constraint_type);
		assert_eq!(constraint.payload, deserialized.payload);
	}

	// todo more unit tests
}
