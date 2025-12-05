/// The commitment type for inclusion commitments
pub const INCLUSION_COMMITMENT_TYPE: u64 = 1;

/// The constraint type for inclusion constraints
pub const INCLUSION_CONSTRAINT_TYPE: u64 = 1;

/// Maximum number of constraints per slot
pub const MAX_CONSTRAINTS_PER_SLOT: usize = 256;

/// Number of slots to query for delegated slots
pub const DELEGATED_SLOTS_QUERY_RANGE: u64 = 64;

/// Number of seconds before the next slot to trigger posting SignedConstraints
pub const CONSTRAINT_TRIGGER_OFFSET: i64 = 2;
