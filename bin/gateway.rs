use commit_boost::prelude::load_commit_module_config;
use commitments::server::run_commitments_rpc_server;
use common::storage::create_database;
use eyre::Result;
use inclusion::gateway::config::GatewayConfig;
use inclusion::gateway::services::{
    constraint_manager::ConstraintManager, delegation_manager::DelegationManager, rpc::GatewayRpc,
};
use inclusion::gateway::state::GatewayState;
use std::sync::Arc;
use tracing::{error, info};

fn setup_state() -> Result<GatewayState> {
    // Load gateway configuration using commit-boost's config loader
    let commit_config = load_commit_module_config::<GatewayConfig>()
        .map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

    let config = commit_config.extra.clone();

    // Initialize database
    let db = create_database(config.db_path.as_str())
        .map_err(|e| eyre::eyre!("Failed to create database: {}", e))?;

    Ok(GatewayState::new(db, commit_config))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting gateway service (commitments server + gateway tasks)");

    // Setup state
    let state = Arc::new(setup_state()?);

    // Create tasks
    let rpc_server = GatewayRpc::new(Arc::clone(&state));
    let delegation_manager = DelegationManager::new(Arc::clone(&state));
    let constraint_manager = ConstraintManager::new(Arc::clone(&state));

    // Spawn RPC server
    let rpc_handle = tokio::spawn(async move {
        if let Err(e) = run_commitments_rpc_server(rpc_server).await {
            error!("Commitments RPC server exited with error: {e:?}");
        } else {
            info!("Commitments RPC server stopped");
        }
    });

    // Spawn delegation task
    let delegation_handle = tokio::spawn(async move {
        if let Err(e) = delegation_manager.run().await {
            error!("Delegation task exited with error: {e:?}");
        } else {
            info!("Delegation task stopped");
        }
    });

    // Spawn constraints task
    let constraints_handle = tokio::spawn(async move {
        if let Err(e) = constraint_manager.run().await {
            error!("Constraints task exited with error: {e:?}");
        } else {
            info!("Constraints task stopped");
        }
    });

    // Wait for Docker shutdown signals (SIGINT/SIGTERM)
    common::utils::wait_for_signal().await?;
    info!("Shutdown signal received, stopping tasks");

    // Kill tasks
    rpc_handle.abort();
    delegation_handle.abort();
    constraints_handle.abort();

    Ok(())
}
