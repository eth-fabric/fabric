use alloy::primitives::{B256, Bytes, keccak256};
use alloy::sol_types::SolValue;

use crate::types::{MessageType, SolCommitment, SolCommitmentRequest};
use commitments::types::spec::{Commitment, CommitmentRequest};

/// Hashes a commitment request as expected by solidity
pub fn get_commitment_request_signing_root(request: &CommitmentRequest) -> B256 {
    let encoded = Bytes::from(SolCommitmentRequest::abi_encode(&SolCommitmentRequest {
        commitment_type: request.commitment_type,
        payload: request.payload.clone(),
        slasher: request.slasher,
    }));

    keccak256(&encoded)
}

/// Hashes a commitment as expected by solidity
pub fn get_commitment_signing_root(commitment: &Commitment) -> B256 {
    let commitment_evm = SolCommitment {
        commitment_type: commitment.commitment_type,
        payload: commitment.payload.clone(),
        request_hash: commitment.request_hash,
        slasher: commitment.slasher,
    };

    // Rust equivalent of abi.encode(message_type, commitment) in Solidity
    let encoded = (MessageType::Commitment.to_uint256(), commitment_evm).abi_encode_params();
    keccak256(&encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Address;
    use eyre::Result;

    #[test]
    fn test_get_commitment_request_signing_root() -> Result<()> {
        let commitment_request = CommitmentRequest {
            commitment_type: 1,
            payload: Bytes::new(),
            slasher: Address::ZERO,
        };
        assert_eq!(
            get_commitment_request_signing_root(&commitment_request).to_string(),
            "0xf61a6130b6ebfffcb3738e03fe820e4b883b623ec3ab7657ffbf385b2e94edba"
        );
        Ok(())
    }

    #[test]
    fn test_get_commitment_signing_root() -> Result<()> {
        let commitment = Commitment {
            commitment_type: 1,
            payload: Bytes::new(),
            request_hash: B256::ZERO,
            slasher: Address::ZERO,
        };

        assert_eq!(
            get_commitment_signing_root(&commitment).to_string(),
            "0x9770f15c80e37efd7af931b39a8b67e01003b923ee5d808b5a87619ebdf30da1"
        );
        Ok(())
    }
}
