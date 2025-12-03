// use commit_boost::prelude::{BlsPublicKey, BlsSignature};
// use ::urc::registry::BLS::{G1Point, G2Point};
use alloy::primitives::{Address, B256, Bytes, keccak256};
use alloy::sol_types::SolValue;

use crate::types::{MessageType, SolCommitmentRequest, SolCommitment};
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


// // ===== Shared conversion helpers for BLS <-> EVM types =====
// /// Converts a pubkey to its corresponding affine G1 point form for EVM precompile usage
// pub fn convert_pubkey_to_g1_point(pubkey: &BlsPublicKey) -> Result<G1Point> {
// 	let pubkey_byes = pubkey.serialize();
// 	let mut pubkey_affine = blst_p1_affine::default();
// 	let uncompress_result = unsafe { blst_p1_uncompress(&mut pubkey_affine, pubkey_byes.as_ptr()) };
// 	match uncompress_result {
// 		BLST_ERROR::BLST_SUCCESS => Ok(()),
// 		_ => Err(eyre::eyre!("Error converting pubkey to affine point: {uncompress_result:?}")),
// 	}?;
// 	let (x_a, x_b) = convert_fp_to_uint256_pair(&pubkey_affine.x);
// 	let (y_a, y_b) = convert_fp_to_uint256_pair(&pubkey_affine.y);
// 	Ok(G1Point { x_a, x_b, y_a, y_b })
// }

// /// Converts a signature to its corresponding affine G2 point form for EVM precompile usage
// pub fn convert_signature_to_g2_point(signature: &BlsSignature) -> Result<G2Point> {
// 	let signature_bytes = signature.serialize();
// 	let mut signature_affine = blst_p2_affine::default();
// 	let uncompress_result = unsafe { blst_p2_uncompress(&mut signature_affine, signature_bytes.as_ptr()) };
// 	match uncompress_result {
// 		BLST_ERROR::BLST_SUCCESS => Ok(()),
// 		_ => Err(eyre::eyre!("Error converting signature to affine point: {uncompress_result:?}")),
// 	}?;
// 	let (x_c0_a, x_c0_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[0]);
// 	let (x_c1_a, x_c1_b) = convert_fp_to_uint256_pair(&signature_affine.x.fp[1]);
// 	let (y_c0_a, y_c0_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[0]);
// 	let (y_c1_a, y_c1_b) = convert_fp_to_uint256_pair(&signature_affine.y.fp[1]);
// 	Ok(G2Point { x_c0_a, x_c0_b, x_c1_a, x_c1_b, y_c0_a, y_c0_b, y_c1_a, y_c1_b })
// }

// /// Converts a blst_fp to a pair of B256, as used in G1Point
// pub fn convert_fp_to_uint256_pair(fp: &blst_fp) -> (B256, B256) {
// 	let mut fp_bytes = [0u8; 48];
// 	unsafe {
// 		blst_bendian_from_fp(fp_bytes.as_mut_ptr(), fp);
// 	}
// 	let mut high_bytes = [0u8; 32];
// 	high_bytes[16..].copy_from_slice(&fp_bytes[0..16]);
// 	let high = B256::from(high_bytes);
// 	let mut low_bytes = [0u8; 32];
// 	low_bytes.copy_from_slice(&fp_bytes[16..48]);
// 	let low = B256::from(low_bytes);
// 	(high, low)
// }

#[cfg(test)]
mod tests {
	use super::*;
	use eyre::Result;

	#[test]
	fn test_get_commitment_request_signing_root() -> Result<()> {
		let commitment_request =
			CommitmentRequest { commitment_type: 1, payload: Bytes::new(), slasher: Address::ZERO };
		assert_eq!(
			get_commitment_request_signing_root(&commitment_request).to_string(),
			"0xf61a6130b6ebfffcb3738e03fe820e4b883b623ec3ab7657ffbf385b2e94edba"
		);
		Ok(())
	}

	#[test]
	fn test_get_commitment_signing_root() -> Result<()> {
		let commitment =
			Commitment { commitment_type: 1, payload: Bytes::new(), request_hash: B256::ZERO, slasher: Address::ZERO };

		assert_eq!(
			get_commitment_signing_root(&commitment).to_string(),
			"0x9770f15c80e37efd7af931b39a8b67e01003b923ee5d808b5a87619ebdf30da1"
		);
		Ok(())
	}
}
