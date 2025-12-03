use alloy::primitives::{Address, B256, Bytes, Signature};
use serde::{Deserialize, Serialize};

/// Request for a new SignedCommitment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitmentRequest {
	pub commitment_type: u64,
	pub payload: Bytes,
	pub slasher: Address,
}

/// Core commitment data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commitment {
	pub commitment_type: u64,
	pub payload: Bytes,
	pub request_hash: B256,
	pub slasher: Address,
}

/// A commitment with its ECDSA signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCommitment {
	pub commitment: Commitment,
	pub nonce: u64,
	pub signing_id: B256,
	pub signature: Signature,
}

/// Information about offerings for a specific chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offering {
	pub chain_id: u64,
	pub commitment_types: Vec<u64>,
}

/// Information about a specific slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotInfo {
	pub slot: u64,
	pub offerings: Vec<Offering>,
}

/// Fee information for a commitment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeInfo {
	pub fee_payload: Bytes, // opaque fee payload
	pub commitment_type: u64,
}

/// Fee payload for an inclusion preconf request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeePayload {
	pub request_hash: B256,
	pub price_gwei: u64,
}