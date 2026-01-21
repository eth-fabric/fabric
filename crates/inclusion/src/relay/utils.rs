use alloy::primitives::Address;
use alloy::rpc::types::beacon::BlsPublicKey;
use common::storage::DatabaseContext;
use eyre::{Result, eyre};
use tracing::info;

use commit_boost::prelude::Chain;
use constraints::types::{
	Constraint, ConstraintProofs, ConstraintsMessage, Delegation, SignedConstraints, SignedDelegation,
	SubmitBlockRequestWithProofs,
};
use lookahead::utils::current_slot;
use proposer::storage::DelegationsDbExt;
use signing::signer::verify_bls;
use urc::utils::{get_constraints_message_signing_root, get_delegation_signing_root};

use crate::constants::{INCLUSION_CONSTRAINT_TYPE, MAX_CONSTRAINTS_PER_SLOT};
use crate::proofs::{InclusionProof, verify_constraints};
use crate::storage::LookaheadDbExt;
use crate::types::InclusionPayload;

/// Verify BLS signature on a SignedConstraints message using the delegate public key from the message
pub fn verify_constraints_signature(signed_constraints: &SignedConstraints, chain: &Chain) -> Result<()> {
	// Get the message hash for signature verification
	let signing_root = get_constraints_message_signing_root(&signed_constraints.message)?;

	// Use the delegate public key from the message for verification
	verify_bls(
		chain.clone(),
		&signed_constraints.message.delegate,
		&signing_root,
		&signed_constraints.signature,
		&signed_constraints.signing_id,
		signed_constraints.nonce,
	)
}

/// Verify BLS signature on a SignedDelegation message using the proposer public key from the message
pub fn verify_delegation_signature(signed_delegation: &SignedDelegation, chain: &Chain) -> Result<()> {
	// Get the signing root for signature verification
	let signing_root = get_delegation_signing_root(&signed_delegation.message)?;

	// Use the proposer public key from the message for verification
	verify_bls(
		chain.clone(),
		&signed_delegation.message.proposer,
		&signing_root,
		&signed_delegation.signature,
		&signed_delegation.signing_id,
		signed_delegation.nonce,
	)
}

/// Validate delegation message structure
pub fn validate_delegation_message(delegation: &Delegation, chain: &Chain) -> Result<()> {
	// Check that committer address is not zero
	if delegation.committer == Address::ZERO {
		return Err(eyre!("Invalid committer address"));
	}

	// Check that the delegation slot has not already elapsed
	if delegation.slot <= current_slot(chain) {
		return Err(eyre!("Delegation slot has already elapsed"));
	}

	Ok(())
}

/// Validate a constraints message
/// Checks that the constraints slot has not already elapsed
pub fn validate_constraints_message(message: &ConstraintsMessage, chain: &Chain) -> Result<()> {
	// Check that the constraints slot has not already elapsed
	if message.slot <= current_slot(chain) {
		return Err(eyre::eyre!("Constraints slot has already elapsed"));
	}

	Ok(())
}

/// Validate that the given public key is the scheduled proposer for the given slot
/// Reads from the proposer lookahead stored in the database
pub fn validate_is_proposer(pubkey: &BlsPublicKey, slot: u64, db: &DatabaseContext) -> Result<()> {
	// Look up the expected proposer from the lookahead database
	match db.get_proposer_bls_key(slot)? {
		Some(expected_proposer) => {
			// Compare the provided pubkey with the expected proposer
			if pubkey == &expected_proposer {
				info!("Proposer validation successful for slot {}", slot);
				Ok(())
			} else {
				Err(eyre!(
					"Proposer validation failed for slot {}: provided pubkey does not match expected proposer",
					slot
				))
			}
		}
		None => Err(eyre!("No proposer lookahead found for slot {}, rejecting validation", slot)),
	}
}

/// Validate that the supplied gateway public key is delegated to for the given slot
pub fn validate_is_gateway(gateway: &BlsPublicKey, slot: u64, db: &DatabaseContext) -> Result<()> {
	// Get the delegation for the given slot
	let delegation = db.get_delegation(slot)?.ok_or(eyre!("No delegation found for slot {}", slot))?;

	// Check that the delegation is for the expected gateway
	if delegation.message.delegate != *gateway {
		return Err(eyre!("Delegation for slot {} is not for the supplied gateway public key", slot));
	}

	Ok(())
}

pub fn handle_proof_validation(
	block_request: &SubmitBlockRequestWithProofs,
	signed_constraints: SignedConstraints,
) -> Result<()> {
	if block_request.proofs.constraint_types.len() != block_request.proofs.payloads.len() {
		return Err(eyre!("Constraint types and payloads length mismatch"));
	}

	if block_request.proofs.constraint_types.len() > MAX_CONSTRAINTS_PER_SLOT {
		return Err(eyre!(
			"Too many proofs: {} exceeds maximum of {}",
			block_request.proofs.constraint_types.len(),
			MAX_CONSTRAINTS_PER_SLOT
		));
	}

	// We first verify the proof corresponds to the constraints
	verify_proof_completeness(&block_request.proofs, &signed_constraints.message.constraints)?;
	info!("Proofs correspond to constraints");

	// We then verify the validity of the proofs
	// For now we assume all constraints are inclusion constraints
	verify_constraints(&block_request.message, &block_request.proofs)?;

	info!("Proofs verified successfully");

	Ok(())
}

/// Verifies that the proofs cover all the constraints
/// Assumes that the constraints are sorted by constraint type
pub fn verify_proof_completeness(proofs: &ConstraintProofs, constraints: &[Constraint]) -> Result<()> {
	if proofs.constraint_types.len() != constraints.len() {
		return Err(eyre!(
			"Constraint types length mismatch, received {} constraints, expected {}",
			proofs.constraint_types.len(),
			constraints.len()
		));
	}

	let matching_constraint_types =
		proofs.constraint_types.iter().zip(constraints.iter()).all(|(n, t)| *n == t.constraint_type);

	if !matching_constraint_types {
		return Err(eyre!("Constraint types mismatch"));
	}

	for (proof, constraint) in proofs.payloads.iter().zip(constraints.iter()) {
		match constraint.constraint_type {
			INCLUSION_CONSTRAINT_TYPE => {
				let proof = InclusionProof::from_bytes(proof)?;
				let payload = InclusionPayload::abi_decode(&constraint.payload)?;
				let tx_hash = payload.tx_hash()?;
				if proof.tx_hash != tx_hash {
					return Err(eyre!("Transaction hash mismatch"));
				}
			}
			_ => {
				return Err(eyre!("Unsupported constraint type {:?}", constraint.constraint_type));
			}
		}
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::Bytes;
	use alloy::primitives::hex;
	use alloy::rpc::types::beacon::BlsPublicKey;

	#[test]
	fn test_validate_delegation_message_zero_committer() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();
		let chain = Chain::Mainnet;

		let delegation = Delegation {
			proposer: BlsPublicKey::new(valid_bls_key.clone().try_into().unwrap()),
			delegate: BlsPublicKey::new(valid_bls_key.try_into().unwrap()),
			committer: Address::ZERO,
			slot: 12345,
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		assert!(validate_delegation_message(&delegation, &chain).is_err());
	}

	#[test]
	fn test_validate_delegation_message_slot_elapsed() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let chain = Chain::Mainnet;

		// Get current slot and try to delegate a slot that has already elapsed
		let current_slot = current_slot(&chain);

		let delegation = Delegation {
			proposer: BlsPublicKey::new(valid_bls_key.clone().try_into().unwrap()),
			delegate: BlsPublicKey::new(valid_bls_key.try_into().unwrap()),
			committer: "0x1234567890123456789012345678901234567890".parse().unwrap(),
			slot: current_slot - 1, // Slot in the past
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		let result = validate_delegation_message(&delegation, &chain);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("already elapsed"));
	}

	#[test]
	fn test_validate_delegation_message_future_slot() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let chain = Chain::Mainnet;

		// Get current slot and try to delegate to a future slot
		let current_slot = current_slot(&chain);

		let delegation = Delegation {
			proposer: BlsPublicKey::new(valid_bls_key.clone().try_into().unwrap()),
			delegate: BlsPublicKey::new(valid_bls_key.try_into().unwrap()),
			committer: "0x1234567890123456789012345678901234567890".parse().unwrap(),
			slot: current_slot + 10, // Future slot
			metadata: Bytes::from(vec![0x01, 0x02]),
		};

		let result = validate_delegation_message(&delegation, &chain);
		assert!(result.is_ok());
	}

	#[test]
	fn test_validate_constraints_message_slot_elapsed() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let chain = Chain::Mainnet;

		// Get current slot and try to create constraints for a slot that has already elapsed
		let current_slot = current_slot(&chain);

		let constraints_message = ConstraintsMessage {
			proposer: BlsPublicKey::new(valid_bls_key.clone().try_into().unwrap()),
			delegate: BlsPublicKey::new(valid_bls_key.try_into().unwrap()),
			slot: current_slot - 1, // Slot in the past
			constraints: vec![],
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message, &chain);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("already elapsed"));
	}

	#[test]
	fn test_validate_constraints_message_current_slot() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let chain = Chain::Mainnet;

		// Get current slot and try to create constraints for the current slot
		let current_slot = current_slot(&chain);

		let constraints_message = ConstraintsMessage {
			proposer: BlsPublicKey::new(valid_bls_key.clone().try_into().unwrap()),
			delegate: BlsPublicKey::new(valid_bls_key.try_into().unwrap()),
			slot: current_slot, // Current slot
			constraints: vec![],
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message, &chain);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("already elapsed"));
	}

	#[test]
	fn test_validate_constraints_message_future_slot() {
		// Use a valid BLS public key
		let valid_bls_key = hex::decode(
			"af6e96c0eccd8d4ae868be9299af737855a1b08d57bccb565ea7e69311a30baeebe08d493c3fea97077e8337e95ac5a6",
		)
		.unwrap();

		let chain = Chain::Mainnet;

		// Get current slot and try to create constraints for a future slot
		let current_slot = current_slot(&chain);

		let constraints_message = ConstraintsMessage {
			proposer: BlsPublicKey::new(valid_bls_key.clone().try_into().unwrap()),
			delegate: BlsPublicKey::new(valid_bls_key.try_into().unwrap()),
			slot: current_slot + 10, // Future slot
			constraints: vec![],
			receivers: vec![],
		};

		let result = validate_constraints_message(&constraints_message, &chain);
		assert!(result.is_ok());
	}
}
