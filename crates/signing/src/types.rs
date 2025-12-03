use alloy::primitives::U256;
use alloy::sol;

/// Binding of the MessageType enum, defined here:
/// https://github.com/eth-fabric/urc/blob/304e59f967dd8fdf4342c2f776f789e7c99b8ef9/src/IRegistry.sol#L99
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MessageType {
    Reserved = 0,
    Registration = 1,
    Delegation = 2,
    Commitment = 3,
    Constraints = 4,
}

impl MessageType {
    pub fn to_uint256(self) -> U256 {
        U256::from(self as u64)
    }
}

sol! {
    struct SolCommitmentRequest {
        uint64 commitment_type;
        bytes payload;
        address slasher;
    }
}

sol! {
    struct SolCommitment {
        uint64 commitment_type;
        bytes payload;
        bytes32 request_hash;
        address slasher;
    }
}
