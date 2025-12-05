pub mod db;

use eyre::{Context, Result};
use rocksdb::{DB, Options};
use std::sync::Arc;

pub use db::DatabaseContext;

/// Create a RocksDB database at the specified path
pub fn create_database(database_path: &str) -> Result<Arc<DatabaseContext>> {
    // Create database directory if it doesn't exist
    std::fs::create_dir_all(database_path)
        .with_context(|| format!("Failed to create database directory: {}", database_path))?;

    // Configure RocksDB options
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);

    // Open the database
    let db = DB::open(&opts, database_path)
        .with_context(|| format!("Failed to open RocksDB database at: {}", database_path))?;

    tracing::info!("RocksDB database opened successfully at: {}", database_path);

    let db_context = DatabaseContext::new(Arc::new(db));
    db_context.healthcheck()?;

    tracing::info!("RocksDB database healthcheck passed");

    Ok(Arc::new(db_context))
}
