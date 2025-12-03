pub mod beacon;
pub mod commitments;
pub mod constraints;
pub mod database;
pub mod inclusion;
pub mod responses;
pub mod urc;

// Re-export all types for easy access
pub use beacon::{
	BeaconTiming, ProposerDutiesResponse, ValidatorData, ValidatorDuty, ValidatorInfo, ValidatorResponse,
};
pub use commitments::{Commitment, CommitmentRequest, FeeInfo, SignedCommitment, SlotInfo};
pub use constraints::{
	Constraint, ConstraintCapabilities, ConstraintProofs, ConstraintsMessage, Delegation, SignedConstraints,
	SignedDelegation, SubmitBlockRequestWithProofs,
};

pub use database::DatabaseContext;
pub use inclusion::InclusionPayload;
pub use responses::{
	GetDelegationsResponse, HealthResponse, ProcessConstraintsResponse, ProcessDelegationsResponse, SlotInfoResponse,
};
pub use urc::{Registration, SignedRegistration, URCRegisterInputs};

use ::urc::registry::BLS::{G1Point, G2Point};
use alloy::primitives::B256;
use blst::*;
use commit_boost::prelude::{BlsPublicKey, BlsSignature};
use eyre::Result;

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
	pub fn to_uint256(self) -> alloy::primitives::U256 {
		alloy::primitives::U256::from(self as u64)
	}
}

// ===== Shared conversion helpers for BLS <-> EVM types =====
/// Converts a pubkey to its corresponding affine G1 point form for EVM precompile usage
pub fn convert_pubkey_to_g1_point(pubkey: &BlsPublicKey) -> Result<G1Point> {
	let pubkey_byes = pubkey.serialize();
	let mut pubkey_affine = blst_p1_affine::default();
	let uncompress_result = unsafe { blst_p1_uncompress(&mut pubkey_affine, pubkey_byes.as_ptr()) };
	match uncompress_result {
		BLST_ERROR::BLST_SUCCESS => Ok(()),
		_ => Err(eyre::eyre!("Error converting pubkey to affine point: {uncompress_result:?}")),
	}?;
	let (x_a, x_b) = convert_fp_to_uint256_pair(&pubkey_affine.x);
	let (y_a, y_b) = convert_fp_to_uint256_pair(&pubkey_affine.y);
	Ok(G1Point { x_a, x_b, y_a, y_b })
}

/// Converts a signature to its corresponding affine G2 point form for EVM precompile usage
pub fn convert_signature_to_g2_point(signature: &BlsSignature) -> Result<G2Point> {
	let signature_bytes = signature.serialize();
	let mut signature_affine = blst_p2_affine::default();
	let uncompress_result = unsafe { blst_p2_uncompress(&mut signature_affine, signature_bytes.as_ptr()) };
	match uncompress_result {
		BLST_ERROR::BLST_SUCCESS => Ok(()),
		_ => Err(eyre::eyre!("Error converting signature to affine point: {uncompress_result:?}")),
	}?;
	let (x_c0_a, x_c0_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[0]);
	let (x_c1_a, x_c1_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[1]);
	let (y_c0_a, y_c0_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[0]);
	let (y_c1_a, y_c1_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[1]);
	Ok(G2Point { x_c0_a, x_c0_b, x_c1_a, x_c1_b, y_c0_a, y_c0_b, y_c1_a, y_c1_b })
}

/// Converts a blst_fp to a pair of B256, as used in G1Point
pub fn convert_fp_to_uint256_pair(fp: &blst_fp) -> (B256, B256) {
	let mut fp_bytes = [0u8; 48];
	unsafe {
		blst_bendian_from_fp(fp_bytes.as_mut_ptr(), fp);
	}
	let mut high_bytes = [0u8; 32];
	high_bytes[16..].copy_from_slice(&fp_bytes[0..16]);
	let high = B256::from(high_bytes);
	let mut low_bytes = [0u8; 32];
	low_bytes.copy_from_slice(&fp_bytes[16..48]);
	let low = B256::from(low_bytes);
	(high, low)
}
