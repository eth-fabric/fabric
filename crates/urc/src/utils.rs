use alloy::primitives::{B256, Bytes, keccak256};
use alloy::rpc::types::beacon::{BlsPublicKey, BlsSignature};
use alloy::sol;
use alloy::sol_types::{SolCall, SolValue};
use blst::{
	BLST_ERROR, blst_bendian_from_fp, blst_fp, blst_p1_affine, blst_p1_uncompress, blst_p2_affine, blst_p2_uncompress,
};
// use commit_boost::prelude::{BlsPublicKey, BlsSignature};
use eyre::{Result, eyre};

use crate::bindings::i_registry::{
	BLS::{G1Point, G2Point},
	IRegistry::{SignedRegistration as SolSignedRegistration, registerCall as SolRegisterCall},
	ISlasher::{Commitment as SolCommitment, Delegation as SolDelegation},
};

use crate::{MessageType, Registration, SignedRegistration, URCRegisterInputs};
use commitments::types::{Commitment, CommitmentRequest};
use constraints::types::{ConstraintsMessage, Delegation};

/// Converts a pubkey to its corresponding affine G1 point form for EVM precompile usage
fn convert_pubkey_to_g1_point(pubkey: &BlsPublicKey) -> Result<G1Point> {
	let mut pubkey_affine = blst_p1_affine::default();
	let uncompress_result = unsafe { blst_p1_uncompress(&mut pubkey_affine, pubkey.as_ptr()) };
	match uncompress_result {
		BLST_ERROR::BLST_SUCCESS => Ok(()),
		_ => Err(eyre::eyre!("Error converting pubkey to affine point: {uncompress_result:?}")),
	}?;
	let (x_a, x_b) = convert_fp_to_uint256_pair(&pubkey_affine.x);
	let (y_a, y_b) = convert_fp_to_uint256_pair(&pubkey_affine.y);
	Ok(G1Point { x_a, x_b, y_a, y_b })
}

/// Converts a signature to its corresponding affine G2 point form for EVM precompile usage
fn convert_signature_to_g2_point(signature: &BlsSignature) -> Result<G2Point> {
	let mut signature_affine = blst_p2_affine::default();
	let uncompress_result = unsafe { blst_p2_uncompress(&mut signature_affine, signature.as_ptr()) };
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
fn convert_fp_to_uint256_pair(fp: &blst_fp) -> (B256, B256) {
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

/// Hashes a commitment request as expected by solidity
pub fn get_commitment_request_signing_root(request: &CommitmentRequest) -> B256 {
	sol! {
		struct SolCommitmentRequest {
			uint64 commitment_type;
			bytes payload;
			address slasher;
		}
	}
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
		commitmentType: commitment.commitment_type,
		payload: commitment.payload.clone(),
		requestHash: commitment.request_hash,
		slasher: commitment.slasher,
	};

	// Rust equivalent of keccak256(abi.encode(message_type, commitment)) in Solidity
	keccak256((MessageType::Commitment.to_uint256(), commitment_evm).abi_encode_params())
}

pub fn get_delegation_signing_root(delegation: &Delegation) -> Result<B256> {
	// Convert the pubkeys to G1 points
	let proposer = convert_pubkey_to_g1_point(&delegation.proposer).map_err(|e| {
		eyre!("Error converting proposer pubkey {} to G1 point: {e:?}", delegation.proposer.to_string())
	})?;
	let delegate = convert_pubkey_to_g1_point(&delegation.delegate).map_err(|e| {
		eyre!("Error converting delegate pubkey {} to G1 point: {e:?}", delegation.delegate.to_string())
	})?;
	let delegation_evm = SolDelegation {
		proposer: proposer,
		delegate: delegate,
		committer: delegation.committer,
		slot: delegation.slot,
		metadata: delegation.metadata.clone(),
	};

	// Rust equivalent of keccak256(abi.encode(message_type, delegation)) in Solidity
	Ok(keccak256((MessageType::Delegation.to_uint256(), delegation_evm).abi_encode_params()))
}

pub fn get_constraints_message_signing_root(constraints: &ConstraintsMessage) -> Result<B256> {
	sol! {
		struct SolConstraint {
			uint64 constraintType;
			bytes payload;
		}

		struct SolConstraintsMessage {
			G1Point proposer;
			G1Point delegate;
			uint64 slot;
			SolConstraint[] constraints;
			G1Point[] receivers;
		}
	}

	// Convert the pubkeys to G1 points
	let proposer = convert_pubkey_to_g1_point(&constraints.proposer).map_err(|e| {
		eyre!("Error converting proposer pubkey {} to G1 point: {e:?}", constraints.proposer.to_string())
	})?;
	let delegate = convert_pubkey_to_g1_point(&constraints.delegate).map_err(|e| {
		eyre!("Error converting delegate pubkey {} to G1 point: {e:?}", constraints.delegate.to_string())
	})?;

	// Convert the ConstraintsMessage to EVM format
	let constraints_message_evm = SolConstraintsMessage {
		proposer,
		delegate,
		slot: constraints.slot,
		constraints: constraints
			.constraints
			.iter()
			.map(|c| SolConstraint { constraintType: c.constraint_type, payload: c.payload.clone() })
			.collect(),
		receivers: constraints.receivers.iter().map(convert_pubkey_to_g1_point).collect::<Result<Vec<_>, _>>()?,
	};

	// Rust equivalent of keccak256(abi.encode(message_type, constraints)) in Solidity
	Ok(keccak256((MessageType::Constraints.to_uint256(), constraints_message_evm).abi_encode_params()))
}

pub fn get_registration_signing_root(registration: &Registration) -> B256 {
	sol! {
		struct SolRegistration {
			address owner;
		}
	}
	let registration_evm = SolRegistration { owner: registration.owner };
	keccak256((MessageType::Registration.to_uint256(), registration_evm).abi_encode_params())
}

fn get_signed_registration_sol_type(registration: &SignedRegistration) -> Result<SolSignedRegistration> {
	let pubkey = convert_pubkey_to_g1_point(&registration.pubkey)?;
	let signature = convert_signature_to_g2_point(&registration.signature)?;
	let signed_registration = SolSignedRegistration { pubkey, signature, nonce: registration.nonce };
	Ok(signed_registration)
}

pub fn abi_encode_urc_register_inputs(inputs: &URCRegisterInputs) -> Result<Bytes> {
	let registrations =
		inputs.registrations.iter().map(get_signed_registration_sol_type).collect::<Result<Vec<_>>>()?;

	let register_call = SolRegisterCall { registrations, owner: inputs.owner, signingId: inputs.signing_id };
	let encoded = register_call.abi_encode();
	Ok(Bytes::from(encoded))
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::{Address, U256, hex};
	use common::utils::decode_pubkey;
	use constraints::types::Constraint;
	use eyre::Result;

	fn bls_pubkey_from_hex(hex_str: &str) -> BlsPublicKey {
		decode_pubkey(hex_str).expect("Failed to decode public key")
	}

	#[test]
	fn test_message_type_to_uint256() {
		assert_eq!(MessageType::Reserved.to_uint256(), U256::from(0));
		assert_eq!(MessageType::Registration.to_uint256(), U256::from(1));
		assert_eq!(MessageType::Delegation.to_uint256(), U256::from(2));
		assert_eq!(MessageType::Commitment.to_uint256(), U256::from(3));
		assert_eq!(MessageType::Constraints.to_uint256(), U256::from(4));
	}

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

	#[test]
	fn test_get_delegation_signing_root() -> Result<()> {
		let proposer = bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		);

		let delegate = bls_pubkey_from_hex(
			"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
		);

		let delegation = Delegation {
			proposer,
			delegate,
			committer: hex!("0x1111111111111111111111111111111111111111").into(),
			slot: 5,
			metadata: Bytes::from("some-metadata-here"),
		};
		assert_eq!(
			get_delegation_signing_root(&delegation).unwrap().to_string(),
			"0xcd9aca062121f6f50df1bfd7e74e2b023a5a0d9e1387447568a2119db5022e1b"
		);
		Ok(())
	}

	#[test]
	fn test_get_constraints_message_signing_root() -> Result<()> {
		let proposer = bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		);
		let delegate = bls_pubkey_from_hex(
			"0xaf53b192a82ec1229e8fce4f99cb60287ce33896192b6063ac332b36fbe87ba1b2936bbc849ec68a0132362ab11a7754",
		);

		// Create test BLS public keys
		let receivers = vec![bls_pubkey_from_hex(
			"0xaf6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)];

		let constraints_message = ConstraintsMessage {
			proposer,
			delegate,
			slot: 67890,
			constraints: vec![
				Constraint { constraint_type: 1, payload: Bytes::from(vec![0x01, 0x02]) },
				Constraint { constraint_type: 2, payload: Bytes::from(vec![0x03, 0x04]) },
			],
			receivers,
		};

		assert_eq!(
			get_constraints_message_signing_root(&constraints_message).unwrap().to_string(),
			"0xb27bb26406c8fe6cf9e5bb1723d7dd2b06e4d32efc0cb0419dc57cc6c4b0ca87"
		);
		Ok(())
	}
}
