pub const PROPOSER_DUTIES_ROUTE: &str = "eth/v1/validator/duties/proposer";

pub const VALIDATOR_STATUS_ROUTE: &str = "eth/v1/beacon/states/head/validators";

/// Ethereum slot duration in seconds
pub const SLOT_DURATION_SECONDS: u64 = 12;

/// Ethereum slot duration in milliseconds
pub const SLOT_DURATION_MS: u64 = SLOT_DURATION_SECONDS * 1000;

/// Slots per epoch
pub const SLOTS_PER_EPOCH: u64 = 32;
