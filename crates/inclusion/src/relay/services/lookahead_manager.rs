use alloy::rpc::types::beacon::BlsPublicKey;
use eyre::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

use crate::storage::LookaheadDbExt;
use lookahead::utils::{current_slot, epoch_to_first_slot, epoch_to_last_slot, slot_to_epoch};

use crate::relay::state::RelayState;

/// Delegation manager that monitors lookahead duties and signs delegations
pub struct LookaheadManager {
	state: Arc<RelayState>,
}

impl LookaheadManager {
	/// Create a new lookahead manager
	pub fn new(state: Arc<RelayState>) -> Self {
		Self { state }
	}

	/// Run the proposer lookahead task continuously
	pub async fn run(&self) -> Result<()> {
		info!("Starting lookahead manager with {}s update interval", self.state.lookahead_update_interval);

		loop {
			if let Err(e) = self.process_lookahead().await {
				error!("Error updating proposer lookahead: {}", e);
			}

			sleep(Duration::from_secs(self.state.lookahead_update_interval)).await;
		}
	}

	/// Update the proposer lookahead for upcoming slots
	async fn process_lookahead(&self) -> Result<()> {
		// Calculate current epoch
		let current_epoch = slot_to_epoch(current_slot(&self.state.chain));

		// Populate each epoch in the range
		for epoch in current_epoch..=current_epoch + 1 {
			self.populate_lookahead(epoch, None).await?;
		}

		info!("Lookahead updated for epochs {} to {}", current_epoch, current_epoch + 1);

		Ok(())
	}

	/// Populate the proposer lookahead for a specific epoch
	/// This is a public method that can be called from tests or for manual population
	/// If proposer_key is provided, all slots in the epoch will use that key (useful for testing)
	/// Otherwise, fetch proposer duties from the beacon node
	pub async fn populate_lookahead(&self, epoch: u64, proposer_key: Option<BlsPublicKey>) -> Result<()> {
		// Calculate the slot range for this epoch
		let start_slot = epoch_to_first_slot(epoch);
		let end_slot = epoch_to_last_slot(epoch);

		match proposer_key {
			Some(key) => {
				// If a test proposer key is provided, use it for all slots in the epoch
				for slot in start_slot..=end_slot {
					self.state.db.store_proposer_bls_key(slot, &key)?;
				}
			}
			None => {
				// Otherwise, fetch proposer duties from the beacon node
				let duties = self.state.beacon_client.get_proposer_duties(epoch).await?;

				for duty in duties.data {
					let slot = duty.parse_slot()?;
					let pubkey = duty.parse_pubkey()?;
					self.state.db.store_proposer_bls_key(slot, &pubkey)?;
				}
			}
		}

		Ok(())
	}
}
