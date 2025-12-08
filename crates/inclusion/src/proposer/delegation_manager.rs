use crate::proposer::state::ProposerState;
use crate::proposer::utils::create_signed_delegation;
use crate::storage::DelegationsDbExt;
use alloy::rpc::types::beacon::BlsPublicKey;
use constraints::client::ConstraintsClient;
use eyre::{Context, Result};
use lookahead::utils::{current_slot, slot_to_epoch};
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Delegation manager that monitors lookahead duties and signs delegations
pub struct DelegationManager {
    state: Arc<ProposerState>,
}

impl DelegationManager {
    /// Create a new delegation manager
    pub fn new(state: Arc<ProposerState>) -> Self {
        Self { state }
    }

    /// Get all consensus BLS public keys from the signer client
    pub async fn get_consensus_keys(&self) -> Result<Vec<BlsPublicKey>> {
        let response = self
            .state
            .signer_client
            .clone()
            .get_pubkeys()
            .await
            .context("Failed to get public keys from signer")?;

        Ok(response
            .keys
            .iter()
            .map(|map| BlsPublicKey::new(map.consensus.serialize()))
            .collect())
    }

    /// Process proposer lookahead to find upcoming duties and sign delegations
    ///
    /// This function checks the beacon chain for proposer duties in the current and next epoch.
    /// If the configured proposer is assigned to a slot, it creates, signs, and posts a delegation
    /// to the relay.
    pub async fn process_lookahead(&self) -> Result<()> {
        let our_pubkeys = self.get_consensus_keys().await?;

        if our_pubkeys.is_empty() {
            warn!("No consensus keys found in signer");
            return Ok(());
        }

        info!(
            "Processing lookahead for {} consensus key(s)",
            our_pubkeys.len()
        );

        // Calculate current epoch
        let current_epoch = slot_to_epoch(current_slot(&self.state.chain));

        // Check duties for both current and next epoch
        for epoch in [current_epoch, current_epoch + 1] {
            self.process_epoch_duties(epoch, &our_pubkeys).await?;
        }

        Ok(())
    }

    /// Process duties for a specific epoch
    async fn process_epoch_duties(
        &self,
        epoch: u64,
        our_pubkeys: &[BlsPublicKey],
    ) -> Result<usize> {
        // Get proposer duties for this epoch
        let duties = self
            .state
            .beacon_client
            .get_proposer_duties(epoch)
            .await
            .context("Failed to get proposer duties")?;

        let mut posted_count = 0;

        // Check each duty to see if it's for one of our proposers
        for duty in duties.data {
            let duty_pubkey = duty.parse_pubkey()?;
            let duty_slot = duty.parse_slot()?;

            debug!("Duty pubkey: {:?}, Duty slot: {}", duty_pubkey, duty_slot);

            // Only process duties that:
            // 1. Match one of our proposer keys
            // 2. Are in the future (slot > current_slot)
            if our_pubkeys.contains(&duty_pubkey) && duty_slot > current_slot(&self.state.chain) {
                info!("Found proposer duty for slot {}", duty_slot);
                let existing_delegation = self.state.db.get_delegation(duty_slot)?;

                if existing_delegation.is_some() {
                    warn!(
                        "Delegation already exists for slot {}. Skipping to prevent equivocation. Existing delegation: proposer={:?}",
                        duty_slot,
                        existing_delegation.unwrap().message.proposer
                    );
                    continue;
                }

                // No existing delegation, proceed to create and sign
                let signed_delegation = create_signed_delegation(
                    &mut self.state.signer_client.clone(),
                    &duty_pubkey,
                    &self.state.gateway_public_key,
                    duty_slot,
                    &self.state.gateway_address,
                    &self.state.module_signing_id,
                    &self.state.chain,
                )
                .await?;

                info!("Signed delegation: {:?}", signed_delegation);

                // Store before sending to prevent equivocation
                self.state.db.store_delegation(&signed_delegation)?;

                info!("Stored delegation for slot {}", duty_slot);

                // Post to relay
                self.state
                    .constraints_client
                    .post_delegation(&signed_delegation)
                    .await?;

                info!("Posted delegation for slot {}", duty_slot);

                posted_count += 1;
            }
        }

        Ok(posted_count)
    }
}
