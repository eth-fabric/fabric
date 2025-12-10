mod bindings;
pub mod utils;

use alloy::primitives::{Address, B256, U256};
use alloy::rpc::types::beacon::{BlsPublicKey, BlsSignature};

/// Binding of the MessageType enum, defined here:
/// https://github.com/eth-fabric/urc/blob/304e59f967dd8fdf4342c2f776f789e7c99b8ef9/src/IRegistry.sol#L99
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum MessageType {
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

/// URC registration message
pub struct Registration {
	pub owner: Address,
}

/// Signed registration used for URC.register
pub struct SignedRegistration {
	pub pubkey: BlsPublicKey,
	pub signature: BlsSignature,
	pub nonce: u64,
}

/// Container for URC register() call parameters
pub struct URCRegisterInputs {
	pub registrations: Vec<SignedRegistration>,
	pub owner: Address,
	pub signing_id: B256,
}
