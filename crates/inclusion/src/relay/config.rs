use commit_boost::prelude::Chain;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayConfig {
	/// Chain spec (either name or path to spec file)
	pub chain: Chain,

	/// Host of the Relay server (constraints API)
	pub host: String,

	/// Port of the Relay server (constraints API)
	pub port: u16,

	/// Path to the rocksdb database file location
	pub db_path: String,

	/// Supported constraint types
	pub constraint_capabilities: Vec<u64>,

	/// Host of the Beacon API for fetching proposer duties
	pub beacon_api_host: String,

	/// Port of the Beacon API for fetching proposer duties
	pub beacon_api_port: u16,

	/// How often to update the lookahead window
	pub lookahead_update_interval: u64,

	/// Host of the downstream relay for proxying unhandled requests
	pub downstream_relay_host: String,

	/// Port of the downstream relay for proxying unhandled requests
	pub downstream_relay_port: u16,
}
