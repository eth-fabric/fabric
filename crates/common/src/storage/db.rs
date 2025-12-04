use std::fmt;
use std::fmt::Write as _;
use std::sync::Arc;

use eyre::Result;
use rocksdb::{DB, WriteBatch};
use serde::{Serialize, de::DeserializeOwned};

/// Basic database operation used for batch writes.
#[derive(Debug, Clone)]
pub enum DbOp {
    Put { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
}

/// Thin wrapper around RocksDB that provides a stable, generic API.
///
/// Domain crates should build extension traits on top of this type.
#[derive(Clone)]
pub struct DatabaseContext {
    inner: Arc<DB>,
}

impl DatabaseContext {
    /// Create a new DatabaseContext from an Arc<DB>.
    pub fn new(inner: Arc<DB>) -> Self {
        Self { inner }
    }

    /// Expose the underlying DB if a crate really needs low level access.
    pub fn inner(&self) -> &Arc<DB> {
        &self.inner
    }

    /// Get a raw value by key.
    ///
    /// Returns Ok(Some(bytes)) if the key exists, Ok(None) if it does not.
    pub fn get_raw(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.inner.get(key)?)
    }

    /// Put a raw value by key.
    pub fn put_raw(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.inner.put(key, value)?;
        Ok(())
    }

    /// Delete a raw key.
    pub fn delete_raw(&self, key: &[u8]) -> Result<()> {
        self.inner.delete(key)?;
        Ok(())
    }

    /// Apply a batch of operations atomically.
    pub fn batch_write_raw(&self, ops: impl IntoIterator<Item = DbOp>) -> Result<()> {
        let mut batch = WriteBatch::default();
        for op in ops {
            match op {
                DbOp::Put { key, value } => batch.put(key, value),
                DbOp::Delete { key } => batch.delete(key),
            }
        }
        self.inner.write(batch)?;
        Ok(())
    }

    /// Convenience helper for reading many keys.
    ///
    /// This is implemented as a simple loop for now.
    pub fn multi_get_raw<'a>(
        &self,
        keys: impl IntoIterator<Item = &'a [u8]>,
    ) -> Result<Vec<Option<Vec<u8>>>> {
        let mut out = Vec::new();
        for key in keys {
            out.push(self.inner.get(key)?);
        }
        Ok(out)
    }

    pub fn healthcheck(&self) -> Result<()> {
        self.inner.put(b"healthcheck", b"ok")?;
        let value = self.inner.get(b"healthcheck")?;
        if value != Some(b"ok".to_vec()) {
            return Err(eyre::eyre!("Healthcheck failed"));
        }
        self.inner.delete(b"healthcheck")?;
        Ok(())
    }
}

/// Helper for building namespaced keys like:
/// "prefix:part1:part2:part3".
pub fn key_with_prefix<I, T>(prefix: &str, parts: I) -> Vec<u8>
where
    I: IntoIterator<Item = T>,
    T: fmt::Display,
{
    let mut s = String::new();
    s.push_str(prefix);

    for part in parts {
        s.push(':');
        let _ = write!(&mut s, "{}", part);
    }

    s.into_bytes()
}

/// Extension trait for typed reads and writes using serde.
///
/// Domain crates can choose their own encoding by defining their own
/// extension traits if they do not want JSON.
pub trait TypedDbExt {
    fn put_json<T: Serialize>(&self, key: &[u8], value: &T) -> Result<()>;
    fn get_json<T: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<T>>;
}

impl TypedDbExt for DatabaseContext {
    fn put_json<T: Serialize>(&self, key: &[u8], value: &T) -> Result<()> {
        let bytes = serde_json::to_vec(value)?;
        self.put_raw(key, &bytes)
    }

    fn get_json<T: DeserializeOwned>(&self, key: &[u8]) -> Result<Option<T>> {
        match self.get_raw(key)? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use rocksdb::Options;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tempfile::TempDir;

    // Helper to create an in-memory-ish temporary DB for tests.
    fn new_temp_db() -> Result<DatabaseContext> {
        let tmp_dir = TempDir::new()?;
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, tmp_dir.path())?;
        Ok(DatabaseContext::new(Arc::new(db)))
    }

    #[test]
    fn healthcheck_works() -> Result<()> {
        let db = new_temp_db()?;
        db.healthcheck()?;
        Ok(())
    }

    #[test]
    fn raw_put_get_delete_roundtrip() -> Result<()> {
        let db = new_temp_db()?;

        let key = b"foo";
        let value = b"bar";

        // put_raw and get_raw
        db.put_raw(key, value)?;
        let got = db.get_raw(key)?;
        assert_eq!(got, Some(value.to_vec()));

        // delete_raw
        db.delete_raw(key)?;
        let got_after_delete = db.get_raw(key)?;
        assert_eq!(got_after_delete, None);

        Ok(())
    }

    #[test]
    fn batch_write_and_multi_get_work() -> Result<()> {
        let db = new_temp_db()?;

        let ops = vec![
            DbOp::Put {
                key: b"k1".to_vec(),
                value: b"v1".to_vec(),
            },
            DbOp::Put {
                key: b"k2".to_vec(),
                value: b"v2".to_vec(),
            },
            DbOp::Put {
                key: b"k3".to_vec(),
                value: b"v3".to_vec(),
            },
        ];

        db.batch_write_raw(ops)?;

        let keys: Vec<&[u8]> = vec![b"k1", b"k2", b"k3", b"k4"];
        let values = db.multi_get_raw(keys.iter().map(|k| k.as_ref()))?;

        assert_eq!(values[0].as_deref(), Some(b"v1".as_ref()));
        assert_eq!(values[1].as_deref(), Some(b"v2".as_ref()));
        assert_eq!(values[2].as_deref(), Some(b"v3".as_ref()));
        assert_eq!(values[3], None);

        // Now delete k2 and check again via a batch
        db.batch_write_raw([DbOp::Delete {
            key: b"k2".to_vec(),
        }])?;

        let values2 = db.multi_get_raw(vec![b"k1", b"k2"].iter().map(|k| k.as_ref()))?;
        assert_eq!(values2[0].as_deref(), Some(b"v1".as_ref()));
        assert_eq!(values2[1], None);

        Ok(())
    }

    #[test]
    fn key_with_prefix_builds_expected_format() {
        let key = key_with_prefix("commitment", [123u64, 456u64]);
        let key_str = String::from_utf8(key).unwrap();
        assert_eq!(key_str, "commitment:123:456");
    }

    #[test]
    fn typed_json_roundtrip() -> Result<()> {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct MyValue {
            a: u32,
            b: String,
        }

        let db = new_temp_db()?;
        let key = b"typed:example";

        let value = MyValue {
            a: 42,
            b: "hello".into(),
        };

        db.put_json(key, &value)?;
        let loaded: Option<MyValue> = db.get_json(key)?;

        assert_eq!(loaded, Some(value));

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Simulated "extension crate" example
    //
    // In reality this would live in another crate, e.g. `constraints`:
    //
    //   // constraints/src/db_ext.rs
    //   use common_db::{DatabaseContext, TypedDbExt, key_with_prefix};
    //   ...
    //
    // We put it here under a module so the tests can compile and run.
    // ------------------------------------------------------------------------

    mod dummy_constraints_crate {
        use super::*;

        /// Domain type that only the "constraints" crate knows.
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        pub struct DummyConstraint {
            pub id: u64,
            pub payload: String,
        }

        /// Extension trait that adds domain specific helpers on top of DatabaseContext.
        ///
        /// This trait would be public in your constraints crate:
        ///
        ///   pub trait ConstraintsDbExt { ... }
        ///
        /// Callers import the trait and then call these methods directly on DatabaseContext.
        pub trait DummyConstraintsDbExt {
            fn store_constraint(&self, constraint: &DummyConstraint) -> Result<()>;
            fn load_constraint(&self, id: u64) -> Result<Option<DummyConstraint>>;
            fn delete_constraint(&self, id: u64) -> Result<()>;
        }

        // Key encoding logic is local to the constraints crate.
        fn constraint_key(id: u64) -> Vec<u8> {
            key_with_prefix("constraint", [id])
        }

        impl DummyConstraintsDbExt for DatabaseContext {
            fn store_constraint(&self, constraint: &DummyConstraint) -> Result<()> {
                let key = constraint_key(constraint.id);
                self.put_json(&key, constraint)
            }

            fn load_constraint(&self, id: u64) -> Result<Option<DummyConstraint>> {
                let key = constraint_key(id);
                self.get_json(&key)
            }

            fn delete_constraint(&self, id: u64) -> Result<()> {
                let key = constraint_key(id);
                self.delete_raw(&key)
            }
        }

        #[test]
        fn extension_trait_roundtrip_works() -> Result<()> {
            let db = super::new_temp_db()?;

            let c = DummyConstraint {
                id: 7,
                payload: "hello-constraints".to_string(),
            };

            // These methods come from the extension trait.
            db.store_constraint(&c)?;

            let loaded = db.load_constraint(7)?;
            assert_eq!(loaded, Some(c.clone()));

            db.delete_constraint(7)?;
            let loaded_after_delete = db.load_constraint(7)?;
            assert_eq!(loaded_after_delete, None);

            Ok(())
        }
    }
}
