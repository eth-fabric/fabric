/// Health check endpoint
pub const HEALTH: &str = "/health";

/// Store delegation endpoint
pub const DELEGATION: &str = "/delegation";

/// Get delegations for a specific slot
pub const DELEGATIONS_SLOT: &str = "/delegations/{slot}";

/// Store constraints endpoint
pub const CONSTRAINTS: &str = "/constraints";

/// Get constraints for a specific slot
pub const CONSTRAINTS_SLOT: &str = "/constraints/v0/relay/constraints/{slot}";

/// Get capabilities endpoint
pub const CAPABILITIES: &str = "/constraints/v0/builder/capabilities";

/// Submit block with proofs endpoint
pub const BLOCKS_WITH_PROOFS: &str = "/constraints/v0/relay/blocks_with_proofs";

/// Downstream builder API submit block endpoint for proxying (optional)
pub const LEGACY_SUBMIT_BLOCK: &str = "/eth/v1/builder/blocks";
