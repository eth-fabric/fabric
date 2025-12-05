use commitments::types::SignedCommitment;
use constraints::types::Constraint;
use serde::{Deserialize, Serialize};

// /// Fee payload for an inclusion preconf request
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FeePayload {
//     pub request_hash: B256,
//     pub price_gwei: u64,
// }

// #[derive(serde::Serialize, serde::Deserialize, Clone)]
// pub struct GenerateProxyKeyResponse {
// 	pub signed_delegation: serde_json::Value, // Will contain the SignedProxyDelegation
// 	pub encryption_scheme: String,
// }

/// A signed commitment and its paired constraint for a specific slot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCommitmentAndConstraint {
    pub commitment: SignedCommitment,
    pub constraint: Constraint,
}
