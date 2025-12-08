use common::logging::setup_logging;
use common::storage::create_database;
use constraints::server::build_constraints_router_with_proxy;
use eyre::Result;
use inclusion::relay::{
    config::RelayConfig,
    services::{lookahead_manager::LookaheadManager, server::RelayServer},
    state::RelayState,
};
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

fn setup_state(path: &str) -> Result<RelayState> {
    // Read config .toml file
    let content = std::fs::read_to_string(path)?;
    let config: RelayConfig = toml::from_str(&content)?;
    info!("Loaded relay config");

    // Setup logging
    setup_logging(&config.log_level)?;

    // Initialize database
    let db = create_database(config.db_path.as_str())
        .map_err(|e| eyre::eyre!("Failed to create database: {}", e))?;

    Ok(RelayState::new(db, config))
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Get config path from command line arguments
    let config_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "config/relay.config.toml".to_string());

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
    tokio::spawn(async move {
        if let Err(e) = lookahead_manager.run().await {
            tracing::error!("Lookahead manager error: {}", e);
        }
    });

    // Run relay server (this will block until shutdown)
    info!("Starting relay server on {}", server_url);
    let listener = TcpListener::bind(server_url).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
