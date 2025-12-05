use serde::{Deserialize, Serialize};

/// Configuration for Beacon API integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconApiConfig {
	/// Primary beacon node endpoint (e.g., Alchemy)
	pub primary_endpoint: String,
	/// Fallback beacon node endpoints
	pub fallback_endpoints: Vec<String>,
	/// Request timeout in seconds
	pub request_timeout_secs: u64,
	/// Beacon chain genesis time (Unix timestamp)
	pub genesis_time: u64,
}

// ==================== Configuration Trait System ====================

/// Gateway configuration trait
/// Defines the interface for gateway orchestration configuration
pub trait GatewayConfig {
	// Commitments server configuration
	fn server_host(&self) -> &str;
	fn server_port(&self) -> u16;
	fn commitments_database_path(&self) -> &str;
	fn log_level(&self) -> &str;

	// Gateway orchestration configuration
	fn relay_url(&self) -> &str;
	fn constraints_api_key(&self) -> Option<&str>;
	fn genesis_timestamp(&self) -> u64;
	fn delegation_database_path(&self) -> &str;
	fn execution_endpoint_url(&self) -> &str;
	fn execution_request_timeout_secs(&self) -> u64;
	fn execution_max_retries(&self) -> u32;
	fn constraints_receivers(&self) -> &[String];
	fn module_signing_id(&self) -> &str;
}

/// Proposer configuration trait
/// Defines the interface for proposer service configuration
pub trait ProposerConfig {
	fn delegate_bls_public_key(&self) -> &str;
	fn relay_url(&self) -> &str;
	fn beacon_api_url(&self) -> &str;
	fn genesis_timestamp(&self) -> u64;
	fn module_signing_id(&self) -> &str;
}

/// Relay configuration trait
/// Defines the interface for relay service configuration
pub trait RelayConfig {
	fn port(&self) -> u16;
	fn database_path(&self) -> &str;
	fn log_level(&self) -> &str;
	fn genesis_timestamp(&self) -> u64;
	fn constraint_capabilities(&self) -> &[u64];
}

// ==================== Concrete Inclusion Preconf Configurations ====================

/// Gateway configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionGatewayConfig {
	// Commitments server configuration
	pub commitments_server_host: String,
	pub commitments_server_port: u16,
	pub commitments_database_path: String,

	/// Gateway orchestration configuration
	pub relay_url: String,
	pub constraints_api_key: Option<String>,
	pub genesis_timestamp: u64,
	pub delegation_database_path: String,

	/// Execution client configuration
	pub execution_endpoint_url: String,
	pub execution_request_timeout_secs: u64,
	pub execution_max_retries: u32,

	/// Constraints receivers
	pub constraints_receivers: Vec<String>,
	pub delegate_public_key: String,

	/// Module signing ID for this gateway instance
	pub module_signing_id: String,

	// Commitments-specific configuration
	pub log_level: String,
	pub enable_method_tracing: bool,
	pub traced_methods: Vec<String>,

	/// Delegation task configuration
	pub delegation_check_interval_seconds: u64,
	pub delegation_lookahead_window: u64,
}

impl GatewayConfig for InclusionGatewayConfig {
	fn server_host(&self) -> &str {
		&self.commitments_server_host
	}

	fn server_port(&self) -> u16 {
		self.commitments_server_port
	}

	fn commitments_database_path(&self) -> &str {
		&self.commitments_database_path
	}

	fn log_level(&self) -> &str {
		&self.log_level
	}

	fn relay_url(&self) -> &str {
		&self.relay_url
	}

	fn constraints_api_key(&self) -> Option<&str> {
		self.constraints_api_key.as_deref()
	}

	fn genesis_timestamp(&self) -> u64 {
		self.genesis_timestamp
	}

	fn delegation_database_path(&self) -> &str {
		&self.delegation_database_path
	}

	fn execution_endpoint_url(&self) -> &str {
		&self.execution_endpoint_url
	}

	fn execution_request_timeout_secs(&self) -> u64 {
		self.execution_request_timeout_secs
	}

	fn execution_max_retries(&self) -> u32 {
		self.execution_max_retries
	}

	fn constraints_receivers(&self) -> &[String] {
		&self.constraints_receivers
	}

	fn module_signing_id(&self) -> &str {
		&self.module_signing_id
	}
}

impl InclusionGatewayConfig {
	/// Get delegation check interval in seconds
	pub fn delegation_check_interval_seconds(&self) -> u64 {
		self.delegation_check_interval_seconds
	}

	/// Get delegation lookahead window in slots
	pub fn delegation_lookahead_window(&self) -> u64 {
		self.delegation_lookahead_window
	}
}

/// Proposer configuration for inclusion preconfs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InclusionProposerConfig {
	pub delegate_bls_public_key: String,
	pub committer_address: String,
	pub relay_url: String,
	pub relay_api_key: Option<String>,
	pub beacon_api_url: String,
	pub beacon_genesis_timestamp: u64,
	pub poll_interval_seconds: u64,
	pub module_signing_id: String,
}

impl ProposerConfig for InclusionProposerConfig {
	fn delegate_bls_public_key(&self) -> &str {
		&self.delegate_bls_public_key
	}

	fn relay_url(&self) -> &str {
		&self.relay_url
	}

	fn beacon_api_url(&self) -> &str {
		&self.beacon_api_url
	}

	fn genesis_timestamp(&self) -> u64 {
		self.beacon_genesis_timestamp
	}

	fn module_signing_id(&self) -> &str {
		&self.module_signing_id
	}
}
