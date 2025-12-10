use common::storage::create_database;
use constraints::server::build_constraints_router_with_proxy;
use eyre::Result;
use inclusion::relay::{
	config::RelayConfig,
	services::{lookahead_manager::LookaheadManager, server::RelayServer},
	state::RelayState,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

fn setup_state(path: &str) -> Result<RelayState> {
	// Read config .toml file
	let content = std::fs::read_to_string(path)?;
	let config: RelayConfig = toml::from_str(&content)?;

	info!("Loaded relay config");

	// Initialize database
	let db = create_database(config.db_path.as_str()).map_err(|e| eyre::eyre!("Failed to create database: {}", e))?;

	Ok(RelayState::new(db, config))
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
	// Get config path from command line arguments
	let config_path = std::env::var("CONFIG_PATH").expect("CONFIG_PATH environment variable not set");

	// Setup logging
	common::logging::setup_logging(&std::env::var("RUST_LOG").expect("RUST_LOG environment variable not set"))?;

	// Setup state
	let state = Arc::new(setup_state(config_path.as_str())?);

	// Copy before move
	let server_url = format!("{}:{}", state.host, state.port);

	// Create lookahead manager
	let lookahead_manager = LookaheadManager::new(Arc::clone(&state));

	// Create relay server
	let relay_server = RelayServer::new(state);

	// Build constraints router with proxy fallback
	let router = build_constraints_router_with_proxy(relay_server);

	info!("Starting lookahead manager");
	let lookahead_manager_handle = tokio::spawn(async move {
		if let Err(e) = lookahead_manager.run().await {
			tracing::error!("Lookahead manager error: {}", e);
		}
	});

	// Run relay server (this will block until shutdown)
	info!("Starting relay server on {}", server_url);
	let listener = TcpListener::bind(server_url).await?;
	let relay_server_handle = tokio::spawn(async move {
		if let Err(e) = axum::serve(listener, router).await {
			tracing::error!("Relay server error: {}", e);
		}
	});

	// Wait for shutdown signals
	common::utils::wait_for_signal().await?;
	info!("Shutdown signal received, stopping tasks");

	// Kill tasks
	lookahead_manager_handle.abort();
	relay_server_handle.abort();

	Ok(())
}
