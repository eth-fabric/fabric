use std::net::IpAddr;

use cb_common::commit::client::SignerClient;
use cb_common::config::StartSignerConfig;
use cb_common::types::{BlsPublicKey, Jwt, ModuleId};
use cb_common::utils::{bls_pubkey_from_hex, random_jwt_secret};
use cb_signer::service::SigningService;
use eyre::Result;
use inclusion::constants::INCLUSION_CONSTRAINT_TYPE;
use serde::Deserialize;
use tokio::time::sleep;
use toml_edit::DocumentMut;
use tracing::info;
use url::Url;

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
    signer_host: String,
    signer_port: u16,
    beacon_mock_host: String,
    beacon_mock_port: u16,
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

    // Proposer specific
    lookahead_check_interval_seconds: u64,

    // Relay specific
    lookahead_update_interval: u64,
    downstream_relay_url: String,

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
    signer_cb_config: Option<String>,
    relay_config: Option<String>,
    spammer_config: Option<String>,
    // Env file paths
    gateway_env_file: Option<String>,
    proposer_env_file: Option<String>,
    signer_env_file: Option<String>,
    relay_env_file: Option<String>,
    spammer_env_file: Option<String>,
    beacon_mock_env_file: Option<String>,
    // Runtime state
    signer_url: Option<Url>,
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
            signer_cb_config: None,
            relay_config: None,
            spammer_config: None,
            gateway_env_file: None,
            proposer_env_file: None,
            signer_env_file: None,
            relay_env_file: None,
            spammer_env_file: None,
            beacon_mock_env_file: None,
            signer_url: None,
            gateway_bls_proxy: None,
            gateway_committer_address: None,
        }
    }

    pub fn setup_directories(&mut self) -> Result<&mut Self> {
        std::fs::create_dir_all("config/simulation")?;
        std::fs::create_dir_all(&self.config.db_path)?;
        std::fs::create_dir_all(&self.config.proxy_key_dir)?;
        std::fs::create_dir_all(&self.config.keys_path)?;
        std::fs::create_dir_all(&self.config.secrets_path)?;
        Ok(self)
    }

    pub fn initialize_jwts(&mut self) -> &mut Self {
        self.gateway_jwt = Some(Jwt(random_jwt_secret()));
        self.proposer_jwt = Some(Jwt(random_jwt_secret()));
        self.admin_jwt = Some(Jwt(random_jwt_secret()));
        self
    }

    pub fn initialize_module_ids(&mut self) -> &mut Self {
        self.gateway_module_id = Some(ModuleId(self.config.gateway_module_name.clone()));
        self.proposer_module_id = Some(ModuleId(self.config.proposer_module_name.clone()));
        self
    }

    pub fn initialize_paths(&mut self) -> &mut Self {
        // Set signer URL
        self.signer_url = Some(
            Url::parse(&format!(
                "http://{host}:{port}",
                host = self
                    .config
                    .signer_host
                    .parse::<IpAddr>()
                    .expect("Failed to parse signer host"),
                port = self.config.signer_port
            ))
            .expect("Failed to parse signer URL"),
        );

        // Set config file paths
        self.signer_cb_config = Some("config/simulation/signer.toml".to_string());
        self.gateway_cb_config = Some("config/simulation/gateway.toml".to_string());
        self.proposer_cb_config = Some("config/simulation/proposer.toml".to_string());
        self.relay_config = Some("config/simulation/relay.toml".to_string());
        self.spammer_config = Some("config/simulation/spammer.toml".to_string());

        // Set env file paths
        self.gateway_env_file = Some("config/simulation/gateway.env".to_string());
        self.proposer_env_file = Some("config/simulation/proposer.env".to_string());
        self.signer_env_file = Some("config/simulation/signer.env".to_string());
        self.relay_env_file = Some("config/simulation/relay.env".to_string());
        self.spammer_env_file = Some("config/simulation/spammer.env".to_string());
        self.beacon_mock_env_file = Some("config/simulation/beacon-mock.env".to_string());
        self
    }

    pub fn write_env_files(&mut self) -> Result<&mut Self> {
        // Gateway .env file
        let gateway_env_content = format!(
            "# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CB_CONFIG={}\n\
             CB_MODULE_ID={}\n\
             CB_SIGNER_JWT={}\n\
             CB_SIGNER_URL={}\n\
             RUST_LOG={}\n",
            self.gateway_cb_config.clone().unwrap(),
            self.gateway_module_id.clone().unwrap(),
            self.gateway_jwt.clone().unwrap(),
            self.signer_url.clone().unwrap(),
            self.config.log_level
        );
        std::fs::write(self.gateway_env_file.clone().unwrap(), gateway_env_content)?;

        // Proposer .env file
        let proposer_env_content = format!(
            "# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CB_CONFIG={}\n\
             CB_MODULE_ID={}\n\
             CB_SIGNER_JWT={}\n\
             CB_SIGNER_URL={}\n\
             RUST_LOG={}\n",
            self.proposer_cb_config.clone().unwrap(),
            self.proposer_module_id.clone().unwrap(),
            self.proposer_jwt.clone().unwrap(),
            self.signer_url.clone().unwrap(),
            self.config.log_level
        );
        std::fs::write(
            self.proposer_env_file.clone().unwrap(),
            proposer_env_content,
        )?;

        // Signer .env file
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
             CB_CONFIG={}\n\
             CB_JWTS={}\n\
             CB_SIGNER_ADMIN_JWT={}\n\
             RUST_LOG={}\n",
            self.signer_cb_config.clone().unwrap(),
            cb_jwts,
            self.admin_jwt.clone().unwrap(),
            self.config.log_level
        );
        std::fs::write(self.signer_env_file.clone().unwrap(), signer_env_content)?;

        // Relay .env file
        let relay_env_content = format!(
            "# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CONFIG_PATH={}\n\
             RUST_LOG={}\n",
            self.relay_config.clone().unwrap(),
            self.config.log_level
        );
        std::fs::write(self.relay_env_file.clone().unwrap(), relay_env_content)?;

        // Spammer .env file
        let spammer_env_content = format!(
            "# Simulation environment variables\n\
             # Generated by simulation-setup binary\n\n\
             CONFIG_PATH={}\n\
             RUST_LOG={}\n",
            self.spammer_config.clone().unwrap(),
            self.config.log_level
        );
        std::fs::write(self.spammer_env_file.clone().unwrap(), spammer_env_content)?;

        // Spammer .env file
        let beacon_mock_env_content = format!(
            "# Simulation environment variables\n\
				# Generated by simulation-setup binary\n\n\
				BEACON_HOST={}\n\
				BEACON_PORT={}\n\
				PROPOSER_KEY={}\n\
				RUST_LOG={}\n",
            self.config.beacon_mock_host,
            self.config.beacon_mock_port,
            self.config.proposer_consensus_key,
            self.config.log_level
        );
        std::fs::write(
            self.beacon_mock_env_file.clone().unwrap(),
            beacon_mock_env_content,
        )?;

        Ok(self)
    }

    pub fn write_signer_config(&mut self) -> Result<&mut Self> {
        let mut doc = self.cb_config();

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
        std::fs::write(self.signer_cb_config.clone().unwrap(), toml.to_string())?;
        Ok(self)
    }

    pub async fn generate_proxy_keys(&mut self) -> Result<&mut Self> {
        // Load signer config
        dotenv::from_filename(self.signer_env_file.clone().unwrap())?;
        let signer_config = StartSignerConfig::load_from_env()?;
        info!("Signer config loaded: {:?}", signer_config);

        // Launch signer server
        let signer_server_handle =
            tokio::spawn(async move { SigningService::run(signer_config).await });

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

    pub fn write_gateway_config(&mut self) -> Result<&mut Self> {
        let mut doc = self.cb_config();

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
constraints_receivers = ["{constraints_receivers}"]

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
            db_path = format!("{}/gateway", self.config.db_path),
            relay_host = self.config.relay_host,
            relay_port = self.config.relay_port,
            execution_client_host = self.config.execution_client_host,
            execution_client_port = self.config.execution_client_port,
            constraints_receivers = self
                .gateway_bls_proxy
                .clone()
                .expect("gateway BLS proxy key not set"),
            module_signing_id = self.config.gateway_module_signing_id,
            delegation_check_interval_seconds = self.config.delegation_check_interval_seconds,
            gateway_public_key = self.config.gateway_default_bls_key
        ));

        let toml = doc.parse::<DocumentMut>().expect("invalid gateway toml");
        std::fs::write(self.gateway_cb_config.clone().unwrap(), toml.to_string())?;

        Ok(self)
    }

    pub fn write_proposer_config(&mut self) -> Result<&mut Self> {
        let mut doc = self.cb_config();

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
            db_path = format!("{}/proposer", self.config.db_path),
            gateway_public_key = self
                .gateway_bls_proxy
                .clone()
                .expect("gateway BLS proxy key not set"),
            gateway_address = self
                .gateway_committer_address
                .clone()
                .expect("gateway address not set"),
            relay_host = self.config.relay_host,
            relay_port = self.config.relay_port,
            beacon_api_host = self.config.beacon_mock_host,
            beacon_api_port = self.config.beacon_mock_port,
            lookahead_check_interval_seconds = self.config.lookahead_check_interval_seconds,
            module_signing_id = self.config.proposer_module_signing_id
        ));

        let toml = doc.parse::<DocumentMut>().expect("invalid proposer toml");
        std::fs::write(self.proposer_cb_config.clone().unwrap(), toml.to_string())?;

        Ok(self)
    }

    pub fn write_relay_config(&mut self) -> Result<&mut Self> {
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
downstream_relay_url = "{downstream_relay_url}"
log_level = "{log_level}"
"#,
            chain = self.config.chain,
            relay_host = self.config.relay_host,
            relay_port = self.config.relay_port,
            db_path = format!("{}/relay", self.config.db_path),
            constraint_capabilities = INCLUSION_CONSTRAINT_TYPE,
            beacon_api_host = self.config.beacon_mock_host,
            beacon_api_port = self.config.beacon_mock_port,
            lookahead_update_interval = self.config.lookahead_update_interval,
            downstream_relay_url = self.config.downstream_relay_url,
            log_level = self.config.log_level,
        );

        let toml = doc.parse::<DocumentMut>().expect("invalid relay toml");
        std::fs::write(self.relay_config.clone().unwrap(), toml.to_string())?;

        Ok(self)
    }

    pub fn write_spammer_config(&mut self) -> Result<&mut Self> {
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
            gateway_host = self.config.gateway_host,
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

    fn cb_config(&self) -> String {
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
port = {port}

[signer.local.loader]
format = "lighthouse"
keys_path = "{keys_path}"
secrets_path = "{secrets_path}"

[signer.local.store]
proxy_dir = "{proxy_key_dir}"
		"#,
            chain = self.config.chain,
            port = self.config.signer_port,
            keys_path = self.config.keys_path,
            secrets_path = self.config.secrets_path,
            proxy_key_dir = self.config.proxy_key_dir
        )
    }

    async fn launch_signer_client(&self) -> Result<SignerClient> {
        let signer_url = self.signer_url.clone().unwrap();
        let client = SignerClient::new(
            signer_url,
            None,
            Jwt(self.gateway_jwt.clone().unwrap().to_string()),
            ModuleId(self.gateway_module_id.clone().unwrap().to_string()),
        )?;
        Ok(client)
    }

    async fn generate_gateway_proxy_keys(&self) -> Result<(BlsPublicKey, String)> {
        let mut client = self.launch_signer_client().await?;

        let gateway_bls_key = bls_pubkey_from_hex(self.config.gateway_default_bls_key.as_str())?;

        let bls_proxy = client
            .generate_proxy_key_bls(gateway_bls_key.clone())
            .await?;
        println!("BLS proxy: {:?}", bls_proxy);

        let ecdsa_proxy = client.generate_proxy_key_ecdsa(gateway_bls_key).await?;
        println!("ECDSA proxy: {:?}", ecdsa_proxy);

        Ok((
            bls_proxy.message.proxy,
            ecdsa_proxy.message.proxy.to_checksum(None),
        ))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting simulation setup");

    SimulationBuilder::new(SimulationConfig::new("config/simulation.config.toml")?)
        .setup_directories()?
        .initialize_jwts()
        .initialize_module_ids()
        .initialize_paths()
        .write_env_files()?
        .write_signer_config()?
        .generate_proxy_keys()
        .await?
        .write_gateway_config()?
        .write_proposer_config()?
        .write_relay_config()?
        .write_spammer_config()?;

    info!("Simulation setup complete");
    Ok(())
}
