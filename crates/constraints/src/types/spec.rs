use alloy::consensus::TxEnvelope;
use alloy::primitives::{Address, B256, Bytes};
use alloy::rlp::Decodable;
use alloy::rpc::types::beacon::relay::SubmitBlockRequest as AlloySubmitBlockRequest;
use eyre::{Result, eyre};
use serde::{Deserialize, Serialize};

use commit_boost::prelude::{BlsPublicKey, BlsSignature};

/// A constraint with its type and payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: u64,
    pub payload: Bytes,
}
/// A delegation message from proposer to gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub proposer: BlsPublicKey,
    pub delegate: BlsPublicKey,
    pub committer: Address,
    pub slot: u64,
    pub metadata: Bytes,
}

/// A signed delegation with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedDelegation {
    pub message: Delegation,
    pub nonce: u64,
    pub signing_id: B256,
    pub signature: BlsSignature,
}

/// A constraints message containing multiple constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintsMessage {
    pub proposer: BlsPublicKey,
    pub delegate: BlsPublicKey,
    pub slot: u64,
    pub constraints: Vec<Constraint>,
    pub receivers: Vec<BlsPublicKey>,
}

/// A signed constraints message with BLS signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedConstraints {
    pub message: ConstraintsMessage,
    pub nonce: u64,
    pub signing_id: B256,
    pub signature: BlsSignature,
}

/// Constraint capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintCapabilities {
    pub constraint_types: Vec<u64>,
}

/// Proofs of constraint validity for a block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintProofs {
    pub constraint_types: Vec<u64>,
    pub payloads: Vec<Bytes>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                &request
                    .execution_payload
                    .payload_inner
                    .payload_inner
                    .transactions
            }
            AlloySubmitBlockRequest::Fulu(request) => {
                &request
                    .execution_payload
                    .payload_inner
                    .payload_inner
                    .transactions
            }
            AlloySubmitBlockRequest::Deneb(request) => {
                &request
                    .execution_payload
                    .payload_inner
                    .payload_inner
                    .transactions
            }
            AlloySubmitBlockRequest::Capella(request) => {
                &request.execution_payload.payload_inner.transactions
            }
        };

        // Decode transactions
        let mut transactions = Vec::new();

        for tx_bytes in tx_bytes_list {
            let tx = TxEnvelope::decode(&mut tx_bytes.as_ref())
                .map_err(|e| eyre!("Failed to decode transaction: {}", e))?;
            transactions.push(tx);
        }

        if transactions.is_empty() {
            return Err(eyre!("No transactions in execution payload"));
        }

        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;

    #[test]
    fn test_constraint_capabilities() {
        let capabilities = ConstraintCapabilities {
            constraint_types: vec![1, 2, 3, 4, 5],
        };

        assert_eq!(capabilities.constraint_types.len(), 5);
        assert_eq!(capabilities.constraint_types[0], 1);
        assert_eq!(capabilities.constraint_types[4], 5);
    }

    /// Test serialization and deserialization of constraint types
    #[test]
    fn test_constraint_serialization() {
        let constraint = Constraint {
            constraint_type: 1,
            payload: Bytes::from(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        };

        // Test JSON serialization
        let json = serde_json::to_string(&constraint).unwrap();
        let deserialized: Constraint = serde_json::from_str(&json).unwrap();

        assert_eq!(constraint.constraint_type, deserialized.constraint_type);
        assert_eq!(constraint.payload, deserialized.payload);
    }

    // todo more unit tests
}
