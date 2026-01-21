use constraints::types::SignedDelegation;
use eyre::Result;

use common::storage::{
	DatabaseContext,
	db::{TypedDbExt, scan_slot_range_kind},
};

/// 1-byte table tags so everything shares the same RocksDB instance.
const KIND_SIGNED_DELEGATION: u8 = b'A';

/// Key for a single SignedDelegation.
/// Layout: [ 'A' ][ slot_be ]
pub fn signed_delegation_key(slot: u64) -> [u8; 1 + 8] {
	let mut key = [0u8; 1 + 8];
	key[0] = KIND_SIGNED_DELEGATION;
	key[1..].copy_from_slice(&slot.to_be_bytes());
	key
}

pub trait DelegationsDbExt {
	fn store_delegation(&self, delegation: &SignedDelegation) -> Result<()>;
	fn get_delegation(&self, slot: u64) -> Result<Option<SignedDelegation>>;
	fn get_delegations_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<(u64, SignedDelegation)>>;
	fn is_delegated(&self, slot: u64) -> Result<bool>;
}

impl DelegationsDbExt for DatabaseContext {
	fn store_delegation(&self, delegation: &SignedDelegation) -> Result<()> {
		let slot = delegation.message.slot; // adjust to your real field
		let key = signed_delegation_key(slot);
		self.put_json(&key, delegation)
	}

	fn get_delegation(&self, slot: u64) -> Result<Option<SignedDelegation>> {
		let key = signed_delegation_key(slot);
		self.get_json(&key)
	}

	fn get_delegations_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<(u64, SignedDelegation)>> {
		scan_slot_range_kind::<SignedDelegation>(self, KIND_SIGNED_DELEGATION, start_slot, end_slot)
	}

	fn is_delegated(&self, slot: u64) -> Result<bool> {
		Ok(self.get_delegation(slot)?.is_some())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use common::storage::db::slot_prefix;
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
	fn signed_delegation_key_layout_is_correct() {
		let slot = 42u64;
		let key = signed_delegation_key(slot);

		assert_eq!(key.len(), 1 + 8);
		assert_eq!(key[0], KIND_SIGNED_DELEGATION);

		let mut slot_bytes = [0u8; 8];
		slot_bytes.copy_from_slice(&key[1..9]);
		let parsed = u64::from_be_bytes(slot_bytes);
		assert_eq!(parsed, slot);
	}

	#[test]
	fn slot_prefix_layout_is_correct() {
		let slot = 1234u64;
		let key = slot_prefix(KIND_SIGNED_DELEGATION, slot);

		assert_eq!(key.len(), 1 + 8);
		assert_eq!(key[0], KIND_SIGNED_DELEGATION);

		let mut slot_bytes = [0u8; 8];
		slot_bytes.copy_from_slice(&key[1..9]);
		let parsed = u64::from_be_bytes(slot_bytes);
		assert_eq!(parsed, slot);
	}

	#[test]
	fn scan_slot_range_kind_empty_db_returns_empty() -> Result<()> {
		let db = new_temp_db()?;

		let result = super::scan_slot_range_kind::<TestValue>(&db, KIND_SIGNED_DELEGATION, 10, 20)?;

		assert!(result.is_empty());
		Ok(())
	}

	#[test]
	fn scan_slot_range_kind_filters_by_slot_range() -> Result<()> {
		let db = new_temp_db()?;

		// Insert some values manually using raw keys.
		// Two delegations, two constraints, mixed slots.
		let v1 = make_test_value(1);
		let v2 = make_test_value(2);

		// Delegations at slots 5 and 15
		db.put_json(&signed_delegation_key(5), &v1)?;
		db.put_json(&signed_delegation_key(15), &v2)?;

		// Scan delegations in [0, 100]
		let delegations = super::scan_slot_range_kind::<TestValue>(&db, KIND_SIGNED_DELEGATION, 0, 100)?;
		assert_eq!(delegations.len(), 2);
		assert_eq!(delegations[0], (5, v1));
		assert_eq!(delegations[1], (15, v2.clone()));

		// Scan delegations in [6, 14] should only return 0
		let delegations_mid = super::scan_slot_range_kind::<TestValue>(&db, KIND_SIGNED_DELEGATION, 6, 14)?;
		assert_eq!(delegations_mid.len(), 0);

		// Scan delegations in [6, 15] should return only slot 15 (inclusive).
		let delegations_mid2 = super::scan_slot_range_kind::<TestValue>(&db, KIND_SIGNED_DELEGATION, 6, 15)?;
		assert_eq!(delegations_mid2.len(), 1);
		assert_eq!(delegations_mid2[0], (15, v2.clone()));

		Ok(())
	}
}
