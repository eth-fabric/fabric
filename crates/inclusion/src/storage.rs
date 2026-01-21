use alloy::primitives::B256;
use alloy::rpc::types::beacon::BlsPublicKey;
use commitments::types::SignedCommitment;
use constraints::types::{Constraint, SignedConstraints};
use eyre::Result;
use rocksdb::{Direction, IteratorMode};

use common::storage::{
	DatabaseContext,
	db::{DbOp, TypedDbExt, scan_slot_range_kind, slot_prefix},
};

use crate::types::SignedCommitmentAndConstraint;

/// 1-byte table tags so everything shares the same RocksDB instance.
const KIND_SIGNED_CONSTRAINT: u8 = b'B';
const KIND_CONSTRAINT: u8 = b'C';
const KIND_SIGNED_COMMITMENT: u8 = b'D';
const KIND_LOOKAHEAD: u8 = b'E';
const KIND_SIGNED_CONSTRAINTS_POSTED: u8 = b'F';

/// Key for a single SignedConstraints.
/// Layout: [ 'B' ][ slot_be ]
pub fn signed_constraint_key(slot: u64) -> [u8; 1 + 8] {
	let mut key = [0u8; 1 + 8];
	key[0] = KIND_SIGNED_CONSTRAINT;
	key[1..].copy_from_slice(&slot.to_be_bytes());
	key
}

/// Key for a single Constraint
/// Layout: [ 'C' ][ slot_be ]
pub fn constraint_key(slot: u64, request_hash: &B256) -> [u8; 1 + 8 + 32] {
	let mut key = [0u8; 1 + 8 + 32];
	key[0] = KIND_CONSTRAINT;
	key[1..9].copy_from_slice(&slot.to_be_bytes());
	key[9..].copy_from_slice(request_hash.as_slice());
	key
}

/// Key for a SignedCommitment (and paired Constraint).
/// Layout: [ 'D' ][ request_hash (32 bytes) ]
pub fn signed_commitment_key(request_hash: &B256) -> [u8; 1 + 32] {
	let mut key = [0u8; 1 + 32];
	key[0] = KIND_SIGNED_COMMITMENT;
	key[1..].copy_from_slice(request_hash.as_slice());
	key
}

/// Key for a proposer BLS public key for a specific slot.
/// Layout: [ 'E' ][ slot_be ]
pub fn lookahead_key(slot: u64) -> [u8; 1 + 8] {
	let mut key = [0u8; 1 + 8];
	key[0] = KIND_LOOKAHEAD;
	key[1..].copy_from_slice(&slot.to_be_bytes());
	key
}

/// Key for a constraints posted flag for a specific slot.
/// Layout: [ 'F' ][ slot_be ]
pub fn signed_constraints_finalized_key(slot: u64) -> [u8; 1 + 8] {
	let mut key = [0u8; 1 + 8];
	key[0] = KIND_SIGNED_CONSTRAINTS_POSTED;
	key[1..].copy_from_slice(&slot.to_be_bytes());
	key
}
pub trait InclusionDbExt {
	fn store_signed_constraints(&self, constraint: &SignedConstraints) -> Result<()>;

	fn get_signed_constraints(&self, slot: u64) -> Result<Option<SignedConstraints>>;

	fn get_signed_constraints_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<(u64, SignedConstraints)>>;

	fn store_signed_commitment_and_constraint(
		&self,
		slot: u64,
		request_hash: &B256,
		commitment: &SignedCommitment,
		constraint: &Constraint,
	) -> Result<()>;

	fn get_signed_commitment(&self, request_hash: &B256) -> Result<Option<SignedCommitmentAndConstraint>>;

	fn get_constraints_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<(u64, B256, Constraint)>>;

	fn finalize_signed_constraints(&self, slot: u64) -> Result<()>;
	fn signed_constraints_finalized(&self, slot: u64) -> Result<bool>;
}

impl InclusionDbExt for DatabaseContext {
	fn store_signed_constraints(&self, constraint: &SignedConstraints) -> Result<()> {
		let slot = constraint.message.slot;
		let key = signed_constraint_key(slot);
		self.put_json(&key, constraint)
	}

	fn get_signed_constraints(&self, slot: u64) -> Result<Option<SignedConstraints>> {
		let key = signed_constraint_key(slot);
		self.get_json(&key)
	}

	fn get_signed_constraints_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<(u64, SignedConstraints)>> {
		scan_slot_range_kind::<SignedConstraints>(self, KIND_SIGNED_CONSTRAINT, start_slot, end_slot)
	}

	fn finalize_signed_constraints(&self, slot: u64) -> Result<()> {
		let key = signed_constraints_finalized_key(slot);
		self.put_json(&key, &true)
	}

	fn signed_constraints_finalized(&self, slot: u64) -> Result<bool> {
		let key = signed_constraints_finalized_key(slot);
		let flag: Option<bool> = self.get_json(&key)?;
		Ok(flag.unwrap_or(false))
	}

	fn store_signed_commitment_and_constraint(
		&self,
		slot: u64,
		request_hash: &B256,
		commitment: &SignedCommitment,
		constraint: &Constraint,
	) -> Result<()> {
		let signed_commitment_key = signed_commitment_key(request_hash);
		let constraint_key = constraint_key(slot, request_hash);

		self.batch_write_raw(vec![
			DbOp::Put { key: signed_commitment_key.to_vec(), value: serde_json::to_vec(commitment)? },
			DbOp::Put { key: constraint_key.to_vec(), value: serde_json::to_vec(constraint)? },
		])
	}

	fn get_signed_commitment(&self, request_hash: &B256) -> Result<Option<SignedCommitmentAndConstraint>> {
		let key = signed_commitment_key(request_hash);
		self.get_json(&key)
	}

	fn get_constraints_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<(u64, B256, Constraint)>> {
		if start_slot > end_slot {
			return Ok(Vec::new());
		}

		let start_key = slot_prefix(KIND_CONSTRAINT, start_slot);
		let inner: &rocksdb::DB = &*self.inner();

		let iter = inner.iterator(IteratorMode::From(&start_key, Direction::Forward));
		let mut out = Vec::new();

		for item in iter {
			let (key, value) = item?;

			if key.len() < 1 + 8 + 32 {
				continue;
			}

			// kind at 0
			if key[0] != KIND_CONSTRAINT {
				break;
			}

			// slot in 1..9
			let mut slot_bytes = [0u8; 8];
			slot_bytes.copy_from_slice(&key[1..9]);
			let slot = u64::from_be_bytes(slot_bytes);
			if slot < start_slot {
				continue;
			}
			if slot > end_slot {
				break;
			}

			// request_hash in 9..41
			let mut hash_bytes = [0u8; 32];
			hash_bytes.copy_from_slice(&key[9..9 + 32]);
			let request_hash = B256::from(hash_bytes);

			let result = serde_json::from_slice::<Constraint>(&value)?;
			out.push((slot, request_hash, result));
		}

		Ok(out)
	}
}

pub trait LookaheadDbExt {
	fn store_proposer_bls_key(&self, slot: u64, key: &BlsPublicKey) -> Result<()>;
	fn get_proposer_bls_key(&self, slot: u64) -> Result<Option<BlsPublicKey>>;
}

impl LookaheadDbExt for DatabaseContext {
	fn store_proposer_bls_key(&self, slot: u64, key: &BlsPublicKey) -> Result<()> {
		let db_key = lookahead_key(slot);
		self.put_json(&db_key, key)
	}

	fn get_proposer_bls_key(&self, slot: u64) -> Result<Option<BlsPublicKey>> {
		let key = lookahead_key(slot);
		self.get_json(&key)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use alloy::primitives::Bytes;
	use common::storage::db::DbOp;
	use eyre::Result;
	use rocksdb::Options;
	use serde::{Deserialize, Serialize};
	use std::sync::Arc;
	use tempfile::TempDir;

	// Simple helper to create an ephemeral DB wrapped in DatabaseContext.
	fn new_temp_db() -> Result<DatabaseContext> {
		let tmp_dir = TempDir::new()?;
		let mut opts = Options::default();
		opts.create_if_missing(true);
		let db = rocksdb::DB::open(&opts, tmp_dir.path())?;
		Ok(DatabaseContext::new(Arc::new(db)))
	}

	// A simple type to test scan_slot_range_kind without depending on the real
	// SignedDelegation / SignedConstraints / SignedCommitment structs.
	#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
	struct TestValue {
		id: u64,
		payload: String,
	}

	fn make_test_value(id: u64) -> TestValue {
		TestValue { id, payload: format!("value-{id}") }
	}

	#[test]
	fn signed_constraint_key_layout_is_correct() {
		let slot = 123u64;
		let key = signed_constraint_key(slot);

		assert_eq!(key.len(), 1 + 8);
		assert_eq!(key[0], KIND_SIGNED_CONSTRAINT);

		let mut slot_bytes = [0u8; 8];
		slot_bytes.copy_from_slice(&key[1..9]);
		let parsed = u64::from_be_bytes(slot_bytes);
		assert_eq!(parsed, slot);
	}

	#[test]
	fn proposer_key_layout_is_correct() {
		let slot = 999u64;
		let key = lookahead_key(slot);

		assert_eq!(key.len(), 1 + 8);
		assert_eq!(key[0], KIND_LOOKAHEAD);

		let mut slot_bytes = [0u8; 8];
		slot_bytes.copy_from_slice(&key[1..9]);
		let parsed = u64::from_be_bytes(slot_bytes);
		assert_eq!(parsed, slot);
	}

	#[test]
	fn signed_commitment_key_layout_is_correct() {
		let hash_bytes = [0x11u8; 32];
		let request_hash = B256::from(hash_bytes);
		let key = signed_commitment_key(&request_hash);

		assert_eq!(key.len(), 1 + 32);
		assert_eq!(key[0], KIND_SIGNED_COMMITMENT);

		// hash
		let mut parsed_hash_bytes = [0u8; 32];
		parsed_hash_bytes.copy_from_slice(&key[1..33]);
		assert_eq!(parsed_hash_bytes, hash_bytes);
	}

	#[test]
	fn signed_constraints_finalized_key_layout_is_correct() {
		let slot = 999u64;
		let key = signed_constraints_finalized_key(slot);

		assert_eq!(key.len(), 1 + 8);
		assert_eq!(key[0], KIND_SIGNED_CONSTRAINTS_POSTED);

		let mut slot_bytes = [0u8; 8];
		slot_bytes.copy_from_slice(&key[1..9]);
		let parsed = u64::from_be_bytes(slot_bytes);
		assert_eq!(parsed, slot);
	}

	#[test]
	fn constraints_range_scan_works_with_mixed_data() -> Result<()> {
		let db = new_temp_db()?;

		// We will emulate Constraint with TestValue here, stored under constraint keys.
		let c1 = make_test_value(101);
		let c2 = make_test_value(102);
		let c3 = make_test_value(103);

		let h1 = B256::from([0x01u8; 32]);
		let h2 = B256::from([0x02u8; 32]);
		let h3 = B256::from([0x03u8; 32]);

		// Slots: 10, 20, 30
		let key1 = constraint_key(10, &h1);
		let key2 = constraint_key(20, &h2);
		let key3 = constraint_key(30, &h3);

		// Store as raw JSON values.
		let v1 = serde_json::to_vec(&c1)?;
		let v2 = serde_json::to_vec(&c2)?;
		let v3 = serde_json::to_vec(&c3)?;
		db.batch_write_raw(vec![
			DbOp::Put { key: key1.to_vec(), value: v1 },
			DbOp::Put { key: key2.to_vec(), value: v2 },
			DbOp::Put { key: key3.to_vec(), value: v3 },
		])?;

		// Now use the same logic as get_constraints_in_range, but decode as TestValue.
		let start_key = slot_prefix(KIND_CONSTRAINT, 10);
		let inner: &rocksdb::DB = &*db.inner();
		let iter = inner.iterator(IteratorMode::From(&start_key, Direction::Forward));

		let mut slots = Vec::new();
		let mut hashes = Vec::new();
		let mut values = Vec::new();

		for item in iter {
			let (key, value) = item?;

			if key.len() < 1 + 8 + 32 {
				continue;
			}

			if key[0] != KIND_CONSTRAINT {
				break;
			}

			let mut slot_bytes = [0u8; 8];
			slot_bytes.copy_from_slice(&key[1..9]);
			let slot = u64::from_be_bytes(slot_bytes);

			if slot > 30 {
				break;
			}

			let mut hash_bytes = [0u8; 32];
			hash_bytes.copy_from_slice(&key[9..9 + 32]);
			let hash = B256::from(hash_bytes);

			let decoded: TestValue = serde_json::from_slice(&value)?;
			slots.push(slot);
			hashes.push(hash);
			values.push(decoded);
		}

		assert_eq!(slots, vec![10, 20, 30]);
		assert_eq!(values, vec![c1, c2, c3]);
		assert_eq!(hashes[0], h1);
		assert_eq!(hashes[1], h2);
		assert_eq!(hashes[2], h3);

		Ok(())
	}

	#[test]
	fn constraints_range_scan_single_slot() -> Result<()> {
		let db = new_temp_db()?;

		let c = Constraint { constraint_type: 1, payload: Bytes::from([0x01u8; 32]) };
		let h = B256::from([0x01u8; 32]);
		let key = constraint_key(10, &h);

		db.put_json(&key, &c)?;

		// Verify slot scan works for a single slot
		let slots = super::scan_slot_range_kind::<Constraint>(&db, KIND_CONSTRAINT, 10, 10)?;
		assert_eq!(slots.len(), 1);
		assert_eq!(slots[0].0, 10);
		assert_eq!(slots[0].1.constraint_type, c.constraint_type);
		assert_eq!(slots[0].1.payload, c.payload);

		// Verify get_constraints_in_range works for a single slot
		let constraints = db.get_constraints_in_range(10, 10)?;
		assert_eq!(constraints.len(), 1);
		assert_eq!(constraints[0].0, 10);
		// no bytes32 stored since we used put_json
		assert_eq!(constraints[0].2.constraint_type, c.constraint_type);
		assert_eq!(constraints[0].2.payload, c.payload);

		Ok(())
	}
}
