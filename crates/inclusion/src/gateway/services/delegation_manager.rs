use eyre::{Result, eyre};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::gateway::state::GatewayState;
use crate::storage::DelegationsDbExt;
use crate::{constants::DELEGATED_SLOTS_QUERY_RANGE, gateway::config::GatewayConfig};
use commit_boost::prelude::Chain;
use constraints::client::{ConstraintsClient, HttpConstraintsClient};
use lookahead::utils::current_slot;

/// Delegation manager that monitors delegated slots
pub struct DelegationManager {
    state: Arc<GatewayState>,
    constraints_client: HttpConstraintsClient,
    config: GatewayConfig,
    chain: Chain,
}

impl DelegationManager {
    /// Create a new delegation task
    /// Create a new constraint manager
    pub async fn new(state: Arc<GatewayState>) -> Self {
        // Client for sending constraints to the relay
        let constraints_client =
            HttpConstraintsClient::new(state.relay_url.to_string(), state.api_key.clone());

        // Copy to avoid needing the lock in the future
        let config = state.config.lock().await.extra.clone();
        let chain = state.config.lock().await.chain.clone();
        Self {
            state,
            constraints_client,
            config,
            chain,
        }
    }

    /// Run the delegation task continuously
    pub async fn run(&self) -> Result<()> {
        info!(
            "Starting delegation task with {}s polling interval",
            self.config.delegation_check_interval_seconds
        );

        loop {
            if let Err(e) = self.update_delegations().await {
                error!("Error in delegation check: {}", e);
            }

            sleep(Duration::from_secs(
                self.config.delegation_check_interval_seconds,
            ))
            .await;
        }
    }

    /// Check delegations for upcoming slots
    async fn update_delegations(&self) -> Result<()> {
        let current_slot = current_slot(&self.chain);
        let lookahead_end = current_slot + DELEGATED_SLOTS_QUERY_RANGE;

        info!(
            "Checking delegations for slots {} to {}",
            current_slot, lookahead_end
        );

        // Batch read known delegated slots
        let delegated_slots = self
            .state
            .db
            .get_delegations_in_range(current_slot, lookahead_end)?
            .into_iter()
            .map(|(slot, _)| slot)
            .collect::<Vec<u64>>();

        // Check each slot in the lookahead window
        for slot in current_slot..=lookahead_end {
            if delegated_slots.contains(&slot) {
                debug!("Slot {} already has delegations, skipping", slot);
                continue;
            }

            // Process but don't return on error to continue processing other slots
            if let Err(e) = self.get_delegations_from_relay(slot).await {
                warn!("Failed to process delegations for slot {}: {}", slot, e);
            }
        }

        Ok(())
    }

    /// Use the constraints API to get delegations for a specific slot
    async fn get_delegations_from_relay(&self, slot: u64) -> Result<()> {
        let delegations = self.constraints_client.get_delegations(slot).await?;

        // It's assumed there is only one delegation for a given slot
        match delegations.first() {
            Some(delegation) => {
                info!("Retrieved delegation from relay for slot {}", slot);

                if delegation.message.delegate != self.config.delegate_public_key {
                    info!(
                        "Delegation for slot {} does not match gateway public key",
                        slot
                    );
                    return Err(eyre!(
                        "Delegation for slot {} does not match gateway public key",
                        slot
                    ));
                }

                // Store delegation in the database to prevent reprocessing
                self.state.db.store_delegation(&delegation)?;

                info!("Successfully stored delegation for slot {}", slot);

                Ok(())
            }
            None => Ok(()),
        }
    }
}
