use eyre::Result;
use std::sync::Arc;
use tracing::{error, info};

use commit_boost::prelude::load_commit_module_config;

use common::storage::create_database;
use constraints::client::ConstraintsClient;
use inclusion::proposer::{
    config::ProposerConfig, delegation_manager::DelegationManager, state::ProposerState,
};
use lookahead::utils::current_slot;

async fn setup_state() -> Result<ProposerState> {
    // Load configuration using commit-boost's config loader
    let commit_config = load_commit_module_config::<ProposerConfig>()
        .map_err(|e| eyre::eyre!("Failed to load commit module config: {}", e))?;

    info!("Loaded config");

    let config = commit_config.extra.clone();

    // Initialize database
    let db = create_database(config.db_path.as_str())
        .map_err(|e| eyre::eyre!("Failed to create database: {}", e))?;

    // Initialize state
    let state = ProposerState::new(db, commit_config);

    info!("Proposer configuration:");
    info!("  Gateway BLS key: {}", state.gateway_public_key);
    info!(
        "  Gateway committer address (ECDSA): {}",
        state.gateway_address
    );
    info!("  Constraints URL: {}", config.relay_addr);
    info!("  Beacon API URL: {}", config.beacon_api_url);
    info!("  Module signing ID: {}", config.module_signing_id);
    info!("  Chain: {}", state.chain);
    info!(
        "  Delegation pollling interval: {} seconds",
        config.lookahead_check_interval_seconds
    );

    // Test constraints server health
    match state.constraints_client.health_check().await {
        Ok(true) => info!("Relay health check passed"),
        _ => return Err(eyre::eyre!("Relay health check failed")),
    }

    Ok(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // Setup state
    let state = setup_state().await?;

    // Clone before move
    let chain = state.chain.clone();
    let lookahead_check_interval_seconds = state.lookahead_check_interval_seconds;

    // Launch delegation manager
    let delegation_manager = DelegationManager::new(Arc::new(state));

    // Launch delegation manager loop
    info!("Starting proposer delegation loop");

    // Set up polling interval
    let mut poll_interval = tokio::time::interval(std::time::Duration::from_secs(
        lookahead_check_interval_seconds,
    ));

    loop {
        poll_interval.tick().await;

        let current_slot = current_slot(&chain);
        info!(
            "Checking proposer duties for current slot: {}",
            current_slot
        );

        // Process lookahead to find and post delegations
        match delegation_manager.process_lookahead().await {
            Ok(()) => {
                info!("Lookahead processed successfully for slot {}", current_slot);
            }
            Err(e) => {
                error!(
                    "Error processing lookahead for slot {}: {}",
                    current_slot, e
                );
            }
        }
    }
}
