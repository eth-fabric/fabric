use eyre::{Result, eyre};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::constants::DELEGATED_SLOTS_QUERY_RANGE;
use crate::gateway::state::GatewayState;
use crate::storage::DelegationsDbExt;
use constraints::client::ConstraintsClient;
use lookahead::utils::current_slot;

/// Delegation manager that monitors delegated slots
pub struct DelegationManager {
    state: Arc<GatewayState>,
}

impl DelegationManager {
    /// Create a new delegation task
    pub async fn new(state: Arc<GatewayState>) -> Self {
        Self { state }
    }

    /// Run the delegation task continuously
    pub async fn run(&self) -> Result<()> {
        info!(
            "Starting delegation task with {}s polling interval",
            self.state.delegation_check_interval_seconds
        );

        loop {
            if let Err(e) = self.update_delegations().await {
                error!("Error in delegation check: {}", e);
            }

            sleep(Duration::from_secs(
                self.state.delegation_check_interval_seconds,
            ))
            .await;
        }
    }

    /// Check delegations for upcoming slots
    async fn update_delegations(&self) -> Result<()> {
        let current_slot = current_slot(&self.state.chain);
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
        let delegations = self.state.constraints_client.get_delegations(slot).await?;

        // It's assumed there is only one delegation for a given slot
        match delegations.first() {
            Some(delegation) => {
                info!("Retrieved delegation from relay for slot {}", slot);

                if delegation.message.delegate != self.state.gateway_public_key {
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
