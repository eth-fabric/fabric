use std::net::IpAddr;

use cb_common::commit::client::SignerClient;
use cb_common::config::StartSignerConfig;
use cb_common::types::{BlsPublicKey, Jwt, ModuleId};
use cb_common::utils::{bls_pubkey_from_hex, random_jwt_secret};
use cb_signer::service::SigningService;
use eyre::Result;
use inclusion::constants::INCLUSION_CONSTRAINT_TYPE;
use reqwest::Url;
use serde::Deserialize;
use tokio::time::sleep;
use toml_edit::DocumentMut;
use tracing::info;

/// Pure data struct for simulation configuration loaded from TOML
#[derive(Debug, Deserialize)]
pub struct SimulationConfig {
	// Chain name
	chain: String,

	// Log level
	log_level: String,

	// Module name
	gateway_module_name: String,
	proposer_module_name: String,

	// Module signing IDs
	gateway_module_signing_id: String,
	proposer_module_signing_id: String,

	// Database paths
	db_path: String,

	// --- Key paths ----
	gateway_default_bls_key: String,
	proposer_consensus_key: String,
	proxy_key_dir: String,
	keys_path: String,
	secrets_path: String,

	// --- Service URLs ----
	proposer_signer_host: String,
	proposer_signer_port: u16,
	gateway_signer_host: String,
	gateway_signer_port: u16,
	beacon_host: String,
	beacon_port: u16,
	execution_client_host: String,
	execution_client_port: u16,
	gateway_host: String,
	gateway_port: u16,
	gateway_metrics_host: String,
	gateway_metrics_port: u16,
	relay_host: String,
	relay_port: u16,

	// Gateway specific
	delegation_check_interval_seconds: u64,
	constraints_receivers: Vec<String>,

	// Proposer specific
	lookahead_check_interval_seconds: u64,

	// Relay specific
	lookahead_update_interval: u64,
	downstream_relay_host: String,
	downstream_relay_port: u16,

	// Spammer specific
	spammer_mode: String,
	spammer_interval_secs: u64,
	spammer_private_key: String,
	slasher_address: String,
}

impl SimulationConfig {
	pub fn new(config_path: &str) -> eyre::Result<Self> {
		let content = std::fs::read_to_string(config_path)?;
		let config: SimulationConfig = toml::from_str(&content)?;
		Ok(config)
	}
}

/// Builder for simulation environment setup
/// Manages all state and orchestration through chainable methods
pub struct SimulationBuilder {
	config: SimulationConfig,
	// JWT tokens
	admin_jwt: Option<Jwt>,
	gateway_jwt: Option<Jwt>,
	proposer_jwt: Option<Jwt>,
	// Module identifiers
	gateway_module_id: Option<ModuleId>,
	proposer_module_id: Option<ModuleId>,
	// Config file paths
	gateway_cb_config: Option<String>,
	proposer_cb_config: Option<String>,
	gateway_signer_cb_config: Option<String>,
	proposer_signer_cb_config: Option<String>,
	relay_config: Option<String>,
	spammer_config: Option<String>,
	gateway_db_path: Option<String>,
	proposer_db_path: Option<String>,
	relay_db_path: Option<String>,
	// Env file paths
	gateway_env_file: Option<String>,
	proposer_env_file: Option<String>,
	gateway_signer_env_file: Option<String>,
	proposer_signer_env_file: Option<String>,
	relay_env_file: Option<String>,
	spammer_env_file: Option<String>,
	beacon_mock_env_file: Option<String>,
	// Runtime state
	proposer_signer_url: Option<Url>,
	gateway_signer_url: Option<Url>,
	gateway_bls_proxy: Option<String>,
	gateway_committer_address: Option<String>,
}

impl SimulationBuilder {
	pub fn new(config: SimulationConfig) -> Self {
		Self {
			config,
			admin_jwt: None,
			gateway_jwt: None,
			proposer_jwt: None,
			gateway_module_id: None,
			proposer_module_id: None,
			gateway_cb_config: None,
			proposer_cb_config: None,
			gateway_signer_cb_config: None,
			proposer_signer_cb_config: None,
			relay_config: None,
			spammer_config: None,
			gateway_db_path: None,
			proposer_db_path: None,
			relay_db_path: None,
			gateway_env_file: None,
			proposer_env_file: None,
			gateway_signer_env_file: None,
			proposer_signer_env_file: None,
			relay_env_file: None,
			spammer_env_file: None,
			beacon_mock_env_file: None,
			proposer_signer_url: None,
			gateway_signer_url: None,
			gateway_bls_proxy: None,
			gateway_committer_address: None,
		}
	}

	pub fn setup_directories(&mut self) -> Result<&mut Self> {
		// Config paths
		std::fs::create_dir_all("config/simulation")?;
		std::fs::create_dir_all("config/docker")?;

		// Key paths
		std::fs::create_dir_all(&self.config.proxy_key_dir)?;
		std::fs::create_dir_all(&self.config.keys_path)?;
		std::fs::create_dir_all(&self.config.secrets_path)?;

		// DB paths
		self.gateway_db_path = Some(format!("{}/gateway", self.config.db_path));
		self.proposer_db_path = Some(format!("{}/proposer", self.config.db_path));
		self.relay_db_path = Some(format!("{}/relay", self.config.db_path));
		std::fs::create_dir_all(self.gateway_db_path.clone().unwrap())?;
		std::fs::create_dir_all(self.proposer_db_path.clone().unwrap())?;
		std::fs::create_dir_all(self.relay_db_path.clone().unwrap())?;
		Ok(self)
	}

	pub fn initialize_jwts(&mut self) -> Result<&mut Self> {
		self.gateway_jwt = Some(Jwt(random_jwt_secret()));
		self.proposer_jwt = Some(Jwt(random_jwt_secret()));
		self.admin_jwt = Some(Jwt(random_jwt_secret()));
		Ok(self)
	}

	pub fn initialize_module_ids(&mut self) -> Result<&mut Self> {
		self.gateway_module_id = Some(ModuleId(self.config.gateway_module_name.clone()));
		self.proposer_module_id = Some(ModuleId(self.config.proposer_module_name.clone()));
		Ok(self)
	}

	pub fn initialize_paths(&mut self, docker: bool) -> Result<&mut Self> {
		// Set signer URL
		self.proposer_signer_url = Some(Url::parse(&format!(
			"http://{host}:{port}",
			host = self.config.proposer_signer_host.parse::<IpAddr>().expect("Failed to parse proposer signer host"),
			port = self.config.proposer_signer_port
		))?);
		self.gateway_signer_url = Some(Url::parse(&format!(
			"http://{host}:{port}",
			host = self.config.gateway_signer_host.parse::<IpAddr>().expect("Failed to parse gateway signer host"),
			port = self.config.gateway_signer_port
		))?);

		let dest = if docker { "config/docker" } else { "config/simulation" };

		// Set config file paths
		self.gateway_signer_cb_config = Some(format!("{}/gateway-signer.toml", dest));
		self.proposer_signer_cb_config = Some(format!("{}/proposer-signer.toml", dest));
		self.gateway_cb_config = Some(format!("{}/gateway.toml", dest));
		self.proposer_cb_config = Some(format!("{}/proposer.toml", dest));
		self.relay_config = Some(format!("{}/relay.toml", dest));
		self.spammer_config = Some(format!("{}/spammer.toml", dest));

		// Set env file paths
		self.gateway_env_file = Some(format!("{}/gateway.env", dest));
		self.proposer_env_file = Some(format!("{}/proposer.env", dest));
		self.gateway_signer_env_file = Some(format!("{}/gateway-signer.env", dest));
		self.proposer_signer_env_file = Some(format!("{}/proposer-signer.env", dest));
		self.relay_env_file = Some(format!("{}/relay.env", dest));
		self.spammer_env_file = Some(format!("{}/spammer.env", dest));
		self.beacon_mock_env_file = Some(format!("{}/beacon-mock.env", dest));
		Ok(self)
	}

	pub fn write_env_files(&mut self, docker: bool) -> Result<&mut Self> {
		let signer_url =
			if docker { "http://gateway-signer:20000".to_string() } else { self.gateway_signer_url.clone().unwrap().to_string() };

		// Gateway .env file
		let config_path = if docker { "config.toml".to_string() } else { self.gateway_cb_config.clone().unwrap() };
		let gateway_env_content = format!(
			"# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CB_CONFIG={config_path}\n\
             CB_MODULE_ID={module_id}\n\
             CB_SIGNER_JWT={jwt}\n\
             CB_SIGNER_URL={signer_url}\n\
             RUST_LOG={log_level}\n",
			config_path = config_path,
			module_id = self.gateway_module_id.clone().unwrap(),
			jwt = self.gateway_jwt.clone().unwrap(),
			signer_url = signer_url,
			log_level = self.config.log_level
		);
		std::fs::write(self.gateway_env_file.clone().unwrap(), gateway_env_content)?;

		// Proposer .env file
		let signer_url =
		if docker { "http://proposer-signer:20000".to_string() } else { self.proposer_signer_url.clone().unwrap().to_string() };
		let config_path = if docker { "config.toml".to_string() } else { self.proposer_cb_config.clone().unwrap() };
		let proposer_env_content = format!(
			"# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CB_CONFIG={config_path}\n\
             CB_MODULE_ID={module_id}\n\
             CB_SIGNER_JWT={jwt}\n\
             CB_SIGNER_URL={signer_url}\n\
             RUST_LOG={log_level}\n",
			config_path = config_path,
			module_id = self.proposer_module_id.clone().unwrap(),
			jwt = self.proposer_jwt.clone().unwrap(),
			signer_url = signer_url,
			log_level = self.config.log_level
		);
		std::fs::write(self.proposer_env_file.clone().unwrap(), proposer_env_content)?;

		// Gateway Signer .env file
		let config_path = if docker { "config.toml".to_string() } else { self.gateway_signer_cb_config.clone().unwrap() };
		let cb_jwts = format!(
			"{gateway_module_id}={gateway_jwt},{proposer_module_id}={proposer_jwt}",
			gateway_module_id = self.gateway_module_id.clone().unwrap(),
			gateway_jwt = self.gateway_jwt.clone().unwrap(),
			proposer_module_id = self.proposer_module_id.clone().unwrap(),
			proposer_jwt = self.proposer_jwt.clone().unwrap()
		);
		let signer_env_content = format!(
			"# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CB_CONFIG={config_path}\n\
             CB_JWTS={cb_jwts}\n\
             CB_SIGNER_ADMIN_JWT={admin_jwt}\n\
             RUST_LOG={log_level}\n",
			config_path = config_path,
			cb_jwts = cb_jwts,
			admin_jwt = self.admin_jwt.clone().unwrap(),
			log_level = self.config.log_level
		);
		std::fs::write(self.gateway_signer_env_file.clone().unwrap(), signer_env_content)?;

		// Proposer Signer .env file
		let config_path = if docker { "config.toml".to_string() } else { self.proposer_signer_cb_config.clone().unwrap() };
		let cb_jwts = format!(
			"{gateway_module_id}={gateway_jwt},{proposer_module_id}={proposer_jwt}",
			gateway_module_id = self.gateway_module_id.clone().unwrap(),
			gateway_jwt = self.gateway_jwt.clone().unwrap(),
			proposer_module_id = self.proposer_module_id.clone().unwrap(),
			proposer_jwt = self.proposer_jwt.clone().unwrap()
		);
		let signer_env_content = format!(
			"# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CB_CONFIG={config_path}\n\
             CB_JWTS={cb_jwts}\n\
             CB_SIGNER_ADMIN_JWT={admin_jwt}\n\
             RUST_LOG={log_level}\n",
			config_path = config_path,
			cb_jwts = cb_jwts,
			admin_jwt = self.admin_jwt.clone().unwrap(),
			log_level = self.config.log_level
		);
		std::fs::write(self.proposer_signer_env_file.clone().unwrap(), signer_env_content)?;

		// Relay .env file
		let config_path = if docker { "config.toml".to_string() } else { self.relay_config.clone().unwrap() };
		let relay_env_content = format!(
			"# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CONFIG_PATH={config_path}\n\
             RUST_LOG={log_level}\n",
			config_path = config_path,
			log_level = self.config.log_level
		);
		std::fs::write(self.relay_env_file.clone().unwrap(), relay_env_content)?;

		// Spammer .env file
		let config_path = if docker { "config.toml".to_string() } else { self.spammer_config.clone().unwrap() };
		let spammer_env_content = format!(
			"# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CONFIG_PATH={config_path}\n\
             RUST_LOG={log_level}\n",
			config_path = config_path,
			log_level = self.config.log_level
		);
		std::fs::write(self.spammer_env_file.clone().unwrap(), spammer_env_content)?;

		// Beacon mock .env file
		let beacon_mock_env_content = format!(
			"# Simulation environment variables\n\
				# Generated by simulation-setup binary\n\n\
				BEACON_HOST={beacon_host}\n\
				BEACON_PORT={beacon_port}\n\
				PROPOSER_KEY={proposer_key}\n\
				RUST_LOG={log_level}\n",
			beacon_host = self.config.beacon_host,
			beacon_port = self.config.beacon_port,
			proposer_key = self.config.proposer_consensus_key,
			log_level = self.config.log_level
		);
		std::fs::write(self.beacon_mock_env_file.clone().unwrap(), beacon_mock_env_content)?;

		Ok(self)
	}

	pub async fn generate_proxy_keys(&mut self, docker: bool) -> Result<&mut Self> {
		// Load signer config
		dotenv::from_filename(self.gateway_signer_env_file.clone().unwrap())?;

		// Force correct CB_CONFIG path since docker changes
		let path = if docker { "config/docker/signer.toml" } else { "config/simulation/signer.toml" };
		unsafe {
			std::env::set_var("CB_CONFIG", path);
		}
		let signer_config = StartSignerConfig::load_from_env()?;
		info!("Signer config loaded: {:?}", signer_config);

		// Launch signer server
		let signer_server_handle = tokio::spawn(async move { SigningService::run(signer_config).await });

		// Wait for signer server to start
		sleep(std::time::Duration::from_secs(5)).await;

		// Generate proxy keys
		let (bls_proxy, ecdsa_proxy) = self.generate_gateway_proxy_keys().await?;

		// Kill signer server
		signer_server_handle.abort();

		// Store proxy keys
		self.gateway_bls_proxy = Some(bls_proxy.to_string());
		self.gateway_committer_address = Some(ecdsa_proxy);

		Ok(self)
	}

	pub fn write_signer_config(&mut self, gateway: bool) -> Result<&mut Self> {
		let path = if gateway { self.gateway_signer_cb_config.clone().unwrap() } else { self.proposer_signer_cb_config.clone().unwrap() };
		let mut doc = self.cb_config(gateway);

		doc.push_str(&format!(
			r#"# Gateway Module configuration
[[modules]]
id = "{gateway_module_name}"
signing_id = "{gateway_module_signing_id}"
type = "commit"
docker_image = "n/a"
env_file = "n/a"

# Proposer Module configuration
[[modules]]
id = "{proposer_module_name}"
signing_id = "{proposer_module_signing_id}"
type = "commit"
docker_image = "n/a"
env_file = "n/a""#,
			gateway_module_name = self.config.gateway_module_name,
			gateway_module_signing_id = self.config.gateway_module_signing_id,
			proposer_module_name = self.config.proposer_module_name,
			proposer_module_signing_id = self.config.proposer_module_signing_id
		));

		let toml = doc.parse::<DocumentMut>().expect("invalid signer toml");
		std::fs::write(path, toml.to_string())?;
		Ok(self)
	}

	pub fn write_gateway_config(&mut self, docker: bool) -> Result<&mut Self> {
		let mut doc = self.cb_config(true);

		let relay_host = if docker { "relay" } else { self.config.relay_host.as_str() };

		doc.push_str(&format!(
			r#"
# -- Commitments server configuration --

# Gateway Module configuration
[[modules]]
id = "{gateway_module_name}"
signing_id = "{gateway_module_signing_id}"
type = "commit"
docker_image = "n/a"
env_file = "n/a"

# Address of the Commitments RPC server
rpc_host = "{rpc_host}"

# Port of the Commitments RPC server
rpc_port = {rpc_port}

# Host of the metrics server
metrics_host = "{metrics_host}"

# Port of the metrics server
metrics_port = {metrics_port}

# Path to the rocksdb database file location
db_path = "{db_path}"

# Host of the Relay server (constraints API)
relay_host = "{relay_host}"

# Port of the Relay server (constraints API)
relay_port = {relay_port}

# Execution client configuration
execution_client_host = "{execution_client_host}"
execution_client_port = {execution_client_port}

# Constraints receivers (set to gateway BLS proxy key for now)
constraints_receivers = [] # todo add once builder can sign x-receiver headers

# Module signing ID for this gateway instance
module_signing_id = "{module_signing_id}"

# Commitments-specific configuration
log_level = "info"

# How often to check for new delegations
delegation_check_interval_seconds = {delegation_check_interval_seconds}

# Gateway public key for signing constraints
gateway_public_key = "{gateway_public_key}"
"#,
			gateway_module_name = self.config.gateway_module_name,
			gateway_module_signing_id = self.config.gateway_module_signing_id,
			rpc_host = self.config.gateway_host,
			rpc_port = self.config.gateway_port,
			metrics_host = self.config.gateway_metrics_host,
			metrics_port = self.config.gateway_metrics_port,
			db_path = self.gateway_db_path.clone().unwrap(),
			relay_host = relay_host,
			relay_port = self.config.relay_port,
			execution_client_host = self.config.execution_client_host,
			execution_client_port = self.config.execution_client_port,
			// constraints_receivers = self.config.constraints_receivers.join(","),
			module_signing_id = self.config.gateway_module_signing_id,
			delegation_check_interval_seconds = self.config.delegation_check_interval_seconds,
			gateway_public_key = self.gateway_bls_proxy.clone().expect("gateway BLS proxy key not set")
		));

		let toml = doc.parse::<DocumentMut>().expect("invalid gateway toml");
		std::fs::write(self.gateway_cb_config.clone().unwrap(), toml.to_string())?;

		Ok(self)
	}

	pub fn write_proposer_config(&mut self, docker: bool) -> Result<&mut Self> {
		let mut doc = self.cb_config(false);

		let relay_host = if docker { "relay" } else { self.config.relay_host.as_str() };
		let beacon_api_host = if docker { "beacon-mock" } else { self.config.beacon_host.as_str() };

		doc.push_str(&format!(
			r#"
# -- Proposer configuration --

# Proposer Module configuration
[[modules]]
id = "{proposer_module_name}"
signing_id = "{proposer_module_signing_id}"
type = "commit"
docker_image = "n/a"
env_file = "n/a"

# Path to the rocksdb database file location
db_path = "{db_path}"

# Gateway public key to delegate to
gateway_public_key = "{gateway_public_key}"

# Gateway committer EOA address
gateway_address = "{gateway_address}"

# Host of the Relay server (constraints API)
relay_host = "{relay_host}"

# Port of the Relay server (constraints API)
relay_port = {relay_port}

# Host of the Beacon API for fetching proposer duties
beacon_api_host = "{beacon_api_host}"

# Port of the Beacon API for fetching proposer duties
beacon_api_port = {beacon_api_port}

# How often to check for new delegations
lookahead_check_interval_seconds = {lookahead_check_interval_seconds}

# Module signing ID for this proposer instance
module_signing_id = "{module_signing_id}"
"#,
			proposer_module_name = self.config.proposer_module_name,
			proposer_module_signing_id = self.config.proposer_module_signing_id,
			db_path = self.proposer_db_path.clone().unwrap(),
			gateway_public_key = self.gateway_bls_proxy.clone().expect("gateway BLS proxy key not set"),
			gateway_address = self.gateway_committer_address.clone().expect("gateway address not set"),
			relay_host = relay_host,
			relay_port = self.config.relay_port,
			beacon_api_host = beacon_api_host,
			beacon_api_port = self.config.beacon_port,
			lookahead_check_interval_seconds = self.config.lookahead_check_interval_seconds,
			module_signing_id = self.config.proposer_module_signing_id
		));

		let toml = doc.parse::<DocumentMut>().expect("invalid proposer toml");
		std::fs::write(self.proposer_cb_config.clone().unwrap(), toml.to_string())?;

		Ok(self)
	}

	pub fn write_relay_config(&mut self, docker: bool) -> Result<&mut Self> {
		let beacon_api_host = if docker { "beacon-mock" } else { self.config.beacon_host.as_str() };

		let doc = format!(
			r#"# This file is automatically generated by the simulation-setup binary
# -- Relay configuration --\n
chain = "{chain}"
host = "{relay_host}"
port = {relay_port}
db_path = "{db_path}"
constraint_capabilities = [{constraint_capabilities}]
beacon_api_host = "{beacon_api_host}"
beacon_api_port = {beacon_api_port}
lookahead_update_interval = {lookahead_update_interval}
downstream_relay_host = "{downstream_relay_host}"
downstream_relay_port = {downstream_relay_port}
"#,
			chain = self.config.chain,
			relay_host = self.config.relay_host,
			relay_port = self.config.relay_port,
			db_path = self.relay_db_path.clone().unwrap(),
			constraint_capabilities = INCLUSION_CONSTRAINT_TYPE,
			beacon_api_host = beacon_api_host,
			beacon_api_port = self.config.beacon_port,
			lookahead_update_interval = self.config.lookahead_update_interval,
			downstream_relay_host = self.config.downstream_relay_host,
			downstream_relay_port = self.config.downstream_relay_port,
		);

		let toml = doc.parse::<DocumentMut>().expect("invalid relay toml");
		std::fs::write(self.relay_config.clone().unwrap(), toml.to_string())?;

		Ok(self)
	}

	pub fn write_spammer_config(&mut self, docker: bool) -> Result<&mut Self> {
		let gateway_host = if docker { "gateway" } else { self.config.gateway_host.as_str() };

		let doc = format!(
			r#"# This file is automatically generated by the simulation-setup binary
# -- Spammer configuration --

# Mode can be either "one-shot" or "continuous"
mode = "{mode}"
chain = "{chain}"
gateway_host = "{gateway_host}"
gateway_port = {gateway_port}
interval_secs = {interval_secs}
sender_private_key = "{sender_private_key}"
slasher_address = "{slasher_address}"
"#,
			mode = self.config.spammer_mode,
			chain = self.config.chain,
			gateway_host = gateway_host,
			gateway_port = self.config.gateway_port,
			interval_secs = self.config.spammer_interval_secs,
			sender_private_key = self.config.spammer_private_key,
			slasher_address = self.config.slasher_address,
		);

		let toml = doc.parse::<DocumentMut>().expect("invalid spammer toml");
		std::fs::write(self.spammer_config.clone().unwrap(), toml.to_string())?;

		Ok(self)
	}

	// --- Private helper methods ---

	fn cb_config(&self, gateway: bool) -> String {
		let host = if gateway { self.config.gateway_signer_host.clone() } else { self.config.proposer_signer_host.clone() };
		let port = if gateway { self.config.gateway_signer_port } else { self.config.proposer_signer_port };
		format!(
			r#"# This file is automatically generated by the simulation-setup binary
# It contains necessary configuration data as required by commit-boost

# Chain configuration
chain = "{chain}"

# PBS configuration
[pbs]
port = 18850
with_signer = true

# Relays configuration (required by commit-boost config structure)
[[relays]]
url = "https://0xafa4c6985aa049fb79dd37010438cfebeb0f2bd42b115b89dd678dab0670c1de38da0c4e9138c9290a398ecd9a0b3110@boost-relay-hoodi.flashbots.net"

# Metrics configuration
[metrics]
enabled = true

# Signer configuration
[signer]
host = "{host}"
port = {port}

[signer.local.loader]
format = "lighthouse"
keys_path = "{keys_path}"
secrets_path = "{secrets_path}"

[signer.local.store]
proxy_dir = "{proxy_key_dir}"
		"#,
			chain = self.config.chain,
			host = host,
			port = port,
			keys_path = self.config.keys_path,
			secrets_path = self.config.secrets_path,
			proxy_key_dir = self.config.proxy_key_dir
		)
	}

	/// Assumes the gateway signer is used
	async fn launch_signer_client(&self) -> Result<SignerClient> {
		let signer_url = self.gateway_signer_url.clone().unwrap();
		let client = SignerClient::new(
			signer_url,
			None,
			Jwt(self.gateway_jwt.clone().unwrap().to_string()),
			ModuleId(self.gateway_module_id.clone().unwrap().to_string()),
		)?;
		Ok(client)
	}

	/// Assumes the gateway signer is used
	async fn generate_gateway_proxy_keys(&self) -> Result<(BlsPublicKey, String)> {
		let mut client = self.launch_signer_client().await?;

		let gateway_bls_key = bls_pubkey_from_hex(self.config.gateway_default_bls_key.as_str())?;

		let bls_proxy = client.generate_proxy_key_bls(gateway_bls_key.clone()).await?;
		println!("BLS proxy: {:?}", bls_proxy);

		let ecdsa_proxy = client.generate_proxy_key_ecdsa(gateway_bls_key).await?;
		println!("ECDSA proxy: {:?}", ecdsa_proxy);

		Ok((bls_proxy.message.proxy, ecdsa_proxy.message.proxy.to_checksum(None)))
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt::init();
	info!("Starting simulation setup");

	let docker = std::env::var("DOCKER").is_ok();
	let base_config_path = if docker { "config/docker.config.toml" } else { "config/simulation.config.toml" };

	SimulationBuilder::new(SimulationConfig::new(base_config_path)?)
		.setup_directories()?
		.initialize_jwts()?
		.initialize_module_ids()?
		.initialize_paths(docker)?
		.write_env_files(docker)?
		.write_signer_config(true)?
		.write_signer_config(false)?
		.generate_proxy_keys(docker)
		.await?
		.write_gateway_config(docker)?
		.write_proposer_config(docker)?
		.write_relay_config(docker)?
		.write_spammer_config(docker)?;

	info!("Simulation setup complete");
	Ok(())
}
