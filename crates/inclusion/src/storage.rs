use alloy::primitives::B256;
use commit_boost::prelude::BlsPublicKey;
use commitments::types::SignedCommitment;
use constraints::types::{Constraint, SignedConstraints, SignedDelegation};
use eyre::Result;
use rocksdb::{Direction, IteratorMode};
use serde::de::DeserializeOwned;

use common::storage::{DatabaseContext, db::TypedDbExt};

use crate::types::SignedCommitmentAndConstraint;

/// 1-byte table tags so everything shares the same RocksDB instance.
const KIND_DELEGATION: u8 = b'D';
const KIND_CONSTRAINT: u8 = b'K';
const KIND_COMMITMENT: u8 = b'C';
const KIND_PROPOSER: u8 = b'P';

/// Key for a single SignedDelegation.
/// Layout: [ 'D' ][ slot_be ]
pub fn delegation_key(slot: u64) -> [u8; 1 + 8] {
    let mut key = [0u8; 1 + 8];
    key[0] = KIND_DELEGATION;
    key[1..].copy_from_slice(&slot.to_be_bytes());
    key
}

/// Key for a single SignedConstraints.
/// Layout: [ 'K' ][ slot_be ]
pub fn constraint_key(slot: u64) -> [u8; 1 + 8] {
    let mut key = [0u8; 1 + 8];
    key[0] = KIND_CONSTRAINT;
    key[1..].copy_from_slice(&slot.to_be_bytes());
    key
}

/// Key for a SignedCommitment (and paired Constraint).
/// Layout: [ 'C' ][ slot_be ][ request_hash (32 bytes) ]
pub fn commitment_key(slot: u64, request_hash: &B256) -> [u8; 1 + 8 + 32] {
    let mut key = [0u8; 1 + 8 + 32];
    key[0] = KIND_COMMITMENT;
    key[1..9].copy_from_slice(&slot.to_be_bytes());
    key[9..].copy_from_slice(request_hash.as_slice());
    key
}

/// Key for a proposer BLS public key for a specific slot.
/// Layout: [ 'P' ][ slot_be ]
pub fn proposer_key(slot: u64) -> [u8; 1 + 8] {
    let mut key = [0u8; 1 + 8];
    key[0] = KIND_PROPOSER;
    key[1..].copy_from_slice(&slot.to_be_bytes());
    key
}

/// Prefix key for a range starting at a given slot for a given kind.
/// Layout: [ kind ][ slot_be ]
pub fn slot_prefix(kind: u8, slot: u64) -> [u8; 1 + 8] {
    let mut key = [0u8; 1 + 8];
    key[0] = kind;
    key[1..].copy_from_slice(&slot.to_be_bytes());
    key
}

fn scan_slot_range_kind<T>(
    db: &DatabaseContext,
    kind: u8,
    start_slot: u64,
    end_slot: u64,
) -> Result<Vec<(u64, T)>>
where
    T: DeserializeOwned,
{
    if start_slot > end_slot {
        return Ok(Vec::new());
    }

    let start_key = slot_prefix(kind, start_slot);
    let inner: &rocksdb::DB = &*db.inner();

    let iter = inner.iterator(IteratorMode::From(&start_key, Direction::Forward));
    let mut out = Vec::new();

    for item in iter {
        let (key, value) = item?;

        if key.len() < 1 + 8 {
            continue;
        }

        // kind at index 0
        let k = key[0];
        if k != kind {
            // different logical table prefix, stop
            break;
        }

        // slot in bytes 1..9
        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&key[1..9]);
        let slot = u64::from_be_bytes(slot_bytes);

        if slot < start_slot {
            continue;
        }
        if slot > end_slot {
            break;
        }

        let value_t = serde_json::from_slice::<T>(&value)?;
        out.push((slot, value_t));
    }

    Ok(out)
}
pub trait DelegationsDbExt {
    fn put_delegation(&self, delegation: &SignedDelegation) -> Result<()>;
    fn get_delegation(&self, slot: u64) -> Result<Option<SignedDelegation>>;
    fn get_delegations_in_range(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<(u64, SignedDelegation)>>;
}

impl DelegationsDbExt for DatabaseContext {
    fn put_delegation(&self, delegation: &SignedDelegation) -> Result<()> {
        let slot = delegation.message.slot; // adjust to your real field
        let key = delegation_key(slot);
        self.put_json(&key, delegation)
    }

    fn get_delegation(&self, slot: u64) -> Result<Option<SignedDelegation>> {
        let key = delegation_key(slot);
        self.get_json(&key)
    }

    fn get_delegations_in_range(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<(u64, SignedDelegation)>> {
        scan_slot_range_kind::<SignedDelegation>(self, KIND_DELEGATION, start_slot, end_slot)
    }
}

pub trait ConstraintsDbExt {
    fn put_signed_constraints(&self, constraint: &SignedConstraints) -> Result<()>;
    fn get_signed_constraints(&self, slot: u64) -> Result<Option<SignedConstraints>>;
    fn get_signed_constraints_in_range(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<(u64, SignedConstraints)>>;
}

impl ConstraintsDbExt for DatabaseContext {
    fn put_signed_constraints(&self, constraint: &SignedConstraints) -> Result<()> {
        let slot = constraint.message.slot;
        let key = constraint_key(slot);
        self.put_json(&key, constraint)
    }

    fn get_signed_constraints(&self, slot: u64) -> Result<Option<SignedConstraints>> {
        let key = constraint_key(slot);
        self.get_json(&key)
    }

    fn get_signed_constraints_in_range(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<(u64, SignedConstraints)>> {
        scan_slot_range_kind::<SignedConstraints>(self, KIND_CONSTRAINT, start_slot, end_slot)
    }
}

pub trait CommitmentsDbExt {
    fn store_signed_commitment_and_constraint(
        &self,
        slot: u64,
        request_hash: &B256,
        commitment: &SignedCommitment,
        constraint: &Constraint,
    ) -> Result<()>;

    fn get_signed_commitment_and_constraint(
        &self,
        slot: u64,
        request_hash: &B256,
    ) -> Result<Option<SignedCommitmentAndConstraint>>;

    fn get_signed_commitment_and_constraints_in_range(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<(u64, B256, SignedCommitmentAndConstraint)>>;
}

impl CommitmentsDbExt for DatabaseContext {
    fn store_signed_commitment_and_constraint(
        &self,
        slot: u64,
        request_hash: &B256,
        commitment: &SignedCommitment,
        constraint: &Constraint,
    ) -> Result<()> {
        let commitment_key = commitment_key(slot, request_hash);

        let signed_commitment_and_constraint = SignedCommitmentAndConstraint {
            commitment: commitment.clone(),
            constraint: constraint.clone(),
        };

        self.put_json(&commitment_key, &signed_commitment_and_constraint)
    }

    fn get_signed_commitment_and_constraint(
        &self,
        slot: u64,
        request_hash: &B256,
    ) -> Result<Option<SignedCommitmentAndConstraint>> {
        let key = commitment_key(slot, request_hash);
        self.get_json(&key)
    }

    fn get_signed_commitment_and_constraints_in_range(
        &self,
        start_slot: u64,
        end_slot: u64,
    ) -> Result<Vec<(u64, B256, SignedCommitmentAndConstraint)>> {
        if start_slot > end_slot {
            return Ok(Vec::new());
        }

        let start_key = slot_prefix(KIND_COMMITMENT, start_slot);
        let inner: &rocksdb::DB = &*self.inner();

        let iter = inner.iterator(IteratorMode::From(&start_key, Direction::Forward));
        let mut out = Vec::new();

        for item in iter {
            let (key, value) = item?;

            if key.len() < 1 + 8 + 32 {
                continue;
            }

            // kind at 0
            if key[0] != KIND_COMMITMENT {
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

            let result = serde_json::from_slice::<SignedCommitmentAndConstraint>(&value)?;
            out.push((slot, request_hash, result));
        }

        Ok(out)
    }
}

pub trait LookaheadDbExt {
    fn put_proposer_bls_key(&self, slot: u64, key: &BlsPublicKey) -> Result<()>;
    fn get_proposer_bls_key(&self, slot: u64) -> Result<Option<BlsPublicKey>>;
}

impl LookaheadDbExt for DatabaseContext {
    fn put_proposer_bls_key(&self, slot: u64, key: &BlsPublicKey) -> Result<()> {
        let db_key = proposer_key(slot);
        self.put_json(&db_key, key)
    }

    fn get_proposer_bls_key(&self, slot: u64) -> Result<Option<BlsPublicKey>> {
        let key = proposer_key(slot);
        self.get_json(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        TestValue {
            id,
            payload: format!("value-{id}"),
        }
    }

    #[test]
    fn delegation_key_layout_is_correct() {
        let slot = 42u64;
        let key = delegation_key(slot);

        assert_eq!(key.len(), 1 + 8);
        assert_eq!(key[0], KIND_DELEGATION);

        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&key[1..9]);
        let parsed = u64::from_be_bytes(slot_bytes);
        assert_eq!(parsed, slot);
    }

    #[test]
    fn constraint_key_layout_is_correct() {
        let slot = 123u64;
        let key = constraint_key(slot);

        assert_eq!(key.len(), 1 + 8);
        assert_eq!(key[0], KIND_CONSTRAINT);

        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&key[1..9]);
        let parsed = u64::from_be_bytes(slot_bytes);
        assert_eq!(parsed, slot);
    }

    #[test]
    fn proposer_key_layout_is_correct() {
        let slot = 999u64;
        let key = proposer_key(slot);

        assert_eq!(key.len(), 1 + 8);
        assert_eq!(key[0], KIND_PROPOSER);

        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&key[1..9]);
        let parsed = u64::from_be_bytes(slot_bytes);
        assert_eq!(parsed, slot);
    }

    #[test]
    fn commitment_key_layout_is_correct() {
        let slot = 7u64;
        let hash_bytes = [0x11u8; 32];
        let request_hash = B256::from(hash_bytes);
        let key = commitment_key(slot, &request_hash);

        assert_eq!(key.len(), 1 + 8 + 32);
        assert_eq!(key[0], KIND_COMMITMENT);

        // slot
        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&key[1..9]);
        let parsed_slot = u64::from_be_bytes(slot_bytes);
        assert_eq!(parsed_slot, slot);

        // hash
        let mut parsed_hash_bytes = [0u8; 32];
        parsed_hash_bytes.copy_from_slice(&key[9..9 + 32]);
        assert_eq!(parsed_hash_bytes, hash_bytes);
    }

    #[test]
    fn slot_prefix_layout_is_correct() {
        let slot = 1234u64;
        let key = slot_prefix(KIND_DELEGATION, slot);

        assert_eq!(key.len(), 1 + 8);
        assert_eq!(key[0], KIND_DELEGATION);

        let mut slot_bytes = [0u8; 8];
        slot_bytes.copy_from_slice(&key[1..9]);
        let parsed = u64::from_be_bytes(slot_bytes);
        assert_eq!(parsed, slot);
    }

    #[test]
    fn scan_slot_range_kind_empty_db_returns_empty() -> Result<()> {
        let db = new_temp_db()?;

        let result = super::scan_slot_range_kind::<TestValue>(&db, KIND_DELEGATION, 10, 20)?;

        assert!(result.is_empty());
        Ok(())
    }

    #[test]
    fn scan_slot_range_kind_with_start_greater_than_end_is_empty() -> Result<()> {
        let db = new_temp_db()?;

        let result = super::scan_slot_range_kind::<TestValue>(&db, KIND_DELEGATION, 20, 10)?;

        assert!(result.is_empty());
        Ok(())
    }

    #[test]
    fn scan_slot_range_kind_filters_by_kind_and_slot_range() -> Result<()> {
        let db = new_temp_db()?;

        // Insert some values manually using raw keys.
        // Two delegations, two constraints, mixed slots.
        let v1 = make_test_value(1);
        let v2 = make_test_value(2);
        let v3 = make_test_value(3);
        let v4 = make_test_value(4);

        // Delegations at slots 5 and 15
        db.put_json(&delegation_key(5), &v1)?;
        db.put_json(&delegation_key(15), &v2)?;

        // Constraints at slots 10 and 20
        db.put_json(&constraint_key(10), &v3)?;
        db.put_json(&constraint_key(20), &v4)?;

        // Scan delegations in [0, 100]
        let delegations = super::scan_slot_range_kind::<TestValue>(&db, KIND_DELEGATION, 0, 100)?;
        assert_eq!(delegations.len(), 2);
        assert_eq!(delegations[0], (5, v1));
        assert_eq!(delegations[1], (15, v2.clone()));

        // Scan constraints in [0, 100]
        let constraints = super::scan_slot_range_kind::<TestValue>(&db, KIND_CONSTRAINT, 0, 100)?;
        assert_eq!(constraints.len(), 2);
        assert_eq!(constraints[0], (10, v3));
        assert_eq!(constraints[1], (20, v4));

        // Scan delegations in [6, 14] should only return slot 15? No, that is out of range.
        let delegations_mid =
            super::scan_slot_range_kind::<TestValue>(&db, KIND_DELEGATION, 6, 14)?;
        assert_eq!(delegations_mid.len(), 0);

        // Scan delegations in [6, 15] should return only slot 15.
        let delegations_mid2 =
            super::scan_slot_range_kind::<TestValue>(&db, KIND_DELEGATION, 6, 15)?;
        assert_eq!(delegations_mid2.len(), 1);
        assert_eq!(delegations_mid2[0], (15, v2.clone()));

        Ok(())
    }

    #[test]
    fn commitments_range_scan_works_with_mixed_data() -> Result<()> {
        let db = new_temp_db()?;

        // We will emulate SignedCommitment with TestValue here, stored under commitment keys.
        let c1 = make_test_value(101);
        let c2 = make_test_value(102);
        let c3 = make_test_value(103);

        let h1 = B256::from([0x01u8; 32]);
        let h2 = B256::from([0x02u8; 32]);
        let h3 = B256::from([0x03u8; 32]);

        // Slots: 10, 20, 30
        let key1 = commitment_key(10, &h1);
        let key2 = commitment_key(20, &h2);
        let key3 = commitment_key(30, &h3);

        // Store as raw JSON values.
        let v1 = serde_json::to_vec(&c1)?;
        let v2 = serde_json::to_vec(&c2)?;
        let v3 = serde_json::to_vec(&c3)?;
        db.batch_write_raw(vec![
            DbOp::Put {
                key: key1.to_vec(),
                value: v1,
            },
            DbOp::Put {
                key: key2.to_vec(),
                value: v2,
            },
            DbOp::Put {
                key: key3.to_vec(),
                value: v3,
            },
        ])?;

        // Now use the same logic as get_signed_commitment_and_constraints_in_range, but decode as TestValue.
        let start_key = slot_prefix(KIND_COMMITMENT, 10);
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

            if key[0] != KIND_COMMITMENT {
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
}
