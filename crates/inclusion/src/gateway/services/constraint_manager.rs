use constraints::types::{Constraint, ConstraintsMessage, SignedDelegation};
use eyre::Result;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::constants::CONSTRAINT_TRIGGER_OFFSET;
use crate::gateway::state::GatewayState;
use crate::gateway::utils::sign_constraints_message;
use crate::storage::{DelegationsDbExt, InclusionDbExt};
use constraints::client::ConstraintsClient;
use lookahead::utils::{current_slot, time_until_next_slot};

/// Constraint manager that monitors delegated slots and triggers constraint processing
pub struct ConstraintManager {
    state: Arc<GatewayState>,
}

impl ConstraintManager {
    /// Create a new constraint manager
    pub async fn new(state: Arc<GatewayState>) -> Self {
        Self { state }
    }

    /// Run the constraints task continuously
    pub async fn run(&self) -> Result<()> {
        info!("Starting constraints task - monitoring delegated slots");

        loop {
            if let Err(e) = self.check_and_process_constraints().await {
                error!("Error in constraints check: {}", e);
            }

            // Sleep for a short interval before checking again
            sleep(Duration::from_millis(100)).await;
        }
    }

    /// Check for delegated slots and process constraints if needed
    async fn check_and_process_constraints(&self) -> Result<()> {
        let current_slot = current_slot(&self.state.chain);
        let target_slot = current_slot + 1;

        // Check if target slot is delegated
        match self.state.db.get_delegation(target_slot) {
            Ok(Some(delegation)) => {
                // Check if constraints have already been finalized for this slot to prevent reprocessing
                match self.state.db.signed_constraints_finalized(target_slot) {
                    Ok(true) => {
                        info!(
                            "Constraints already posted for slot {}, skipping",
                            target_slot
                        );
                        // Sleep for a longer interval since we don't need to process this slot
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        return Ok(());
                    }
                    Ok(false) => {
                        // Calculate time until trigger offset before target slot starts
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let trigger_time = (time_until_next_slot(&self.state.chain)
                            - CONSTRAINT_TRIGGER_OFFSET)
                            as u64;

                        if now >= trigger_time {
                            // Time to process constraints for this slot
                            info!(
                                "Triggering constraints processing for slot {} ({} seconds before slot start)",
                                target_slot, CONSTRAINT_TRIGGER_OFFSET
                            );
                            if let Err(e) = self.post_constraints(target_slot, delegation).await {
                                warn!(
                                    "Failed to process constraints for slot {}: {}",
                                    target_slot, e
                                );
                            }
                        } else {
                            // Wait until it's time to trigger
                            let wait_duration = trigger_time - now;
                            info!(
                                "Slot {} is delegated, waiting {} seconds until trigger time",
                                target_slot, wait_duration
                            );
                            tokio::time::sleep(Duration::from_secs(wait_duration)).await;

                            // Now process constraints
                            info!("Triggering constraints processing for slot {}", target_slot);
                            if let Err(e) = self.post_constraints(target_slot, delegation).await {
                                warn!(
                                    "Failed to process constraints for slot {}: {}",
                                    target_slot, e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Failed to check constraint posted status for slot {}: {}",
                            target_slot, e
                        );
                        // Continue with processing despite the error
                    }
                }
            }
            Ok(None) => {
                // Target slot is not delegated, nothing to do
                // Sleep for a longer interval to avoid busy waiting
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => {
                error!(
                    "Failed to check delegation status for slot {}: {}",
                    target_slot, e
                );
                // Sleep briefly before retrying
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }

        Ok(())
    }

    /// Process constraints for a specific slot
    async fn post_constraints(&self, slot: u64, delegation: SignedDelegation) -> Result<()> {
        // Get constraints for the specific slot
        let constraints: Vec<Constraint> = self
            .state
            .db
            .get_constraints_in_range(slot, slot)?
            .into_iter()
            .map(|(_, _, constraint)| constraint)
            .collect();

        if constraints.is_empty() {
            info!("No constraints found for slot {}", slot);
            return Ok(());
        }

        let constraints_message = ConstraintsMessage {
            proposer: delegation.message.proposer.clone(),
            delegate: delegation.message.delegate.clone(),
            slot,
            constraints,
            receivers: self.state.constraints_receivers.clone(),
        };

        // Sign the constraints message with the gateway public key
        let signed_constraints = sign_constraints_message(
            &constraints_message,
            &mut self.state.signer_client.clone(),
            delegation.message.delegate,
            &self.state.module_signing_id,
            self.state.chain,
        )
        .await?;

        // Send to relay using the client
        self.state
            .constraints_client
            .post_constraints(&signed_constraints)
            .await?;

        info!("Successfully posted constraints for slot {} to relay", slot);

        // Mark constraints as posted for this slot to prevent reprocessing
        self.state.db.finalize_signed_constraints(slot)?;

        info!("Successfully finalized constraints for slot {}", slot);

        Ok(())
    }
}
