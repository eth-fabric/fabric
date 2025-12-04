//! Logging setup helpers shared across binaries/services.

use eyre::Result;
use tracing_subscriber::EnvFilter;

/// Initialize tracing-subscriber with the provided log level.
///
/// This configures an EnvFilter using either RUST_LOG or the provided level,
/// enables thread IDs and names, and disables target names for cleaner output.
pub fn setup(log_level: &str) -> Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    Ok(())
}
