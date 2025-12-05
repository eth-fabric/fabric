use commit_boost::prelude::StartCommitModuleConfig;
use common::slot_timer::SlotTimer;
use common::types::database::DatabaseContext;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Server state that provides access to shared resources for all RPC handlers
/// This holds runtime resources (database connections, RPC clients, timers) needed by the commitments service
#[derive(Clone)]
pub struct CommitmentsServerState<T> {
	/// Database context for commitments and constraints
	pub commitments_database: DatabaseContext,
	/// Database context for delegations
	pub delegations_database: DatabaseContext,
	/// Commit module configuration for commit-boost operations (Arc<Mutex> for thread safety)
	pub commit_config: Arc<Mutex<StartCommitModuleConfig<T>>>,
	/// Relay URL for constraints communication
	pub relay_url: String,
	/// API key for relay authentication
	pub api_key: Option<String>,
	/// Slot timer for Ethereum slot timing
	pub slot_timer: SlotTimer,
}

// Manual Debug implementation since ExecutionApiClient might not derive Debug properly
impl<T> std::fmt::Debug for CommitmentsServerState<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CommitmentsServerState")
			.field("commitments_database", &self.commitments_database)
			.field("delegations_database", &self.delegations_database)
			.field("relay_url", &self.relay_url)
			.field("slot_timer", &self.slot_timer)
			.finish()
	}
}

impl<T> CommitmentsServerState<T> {
	/// Create a new commitments server state with the given database contexts and commit config
	pub fn new(
		commitments_database: DatabaseContext,
		delegations_database: DatabaseContext,
		commit_config: StartCommitModuleConfig<T>,
		relay_url: String,
		api_key: Option<String>,
		slot_timer: SlotTimer,
	) -> Self {
		Self {
			commitments_database,
			delegations_database,
			commit_config: Arc::new(Mutex::new(commit_config)),
			relay_url,
			api_key,
			slot_timer,
		}
	}

	/// Get a reference to the commitments database
	pub fn commitments_database(&self) -> &DatabaseContext {
		&self.commitments_database
	}

	/// Get a reference to the delegations database
	pub fn delegations_database(&self) -> &DatabaseContext {
		&self.delegations_database
	}

	/// Get a reference to the slot timer
	pub fn slot_timer(&self) -> &SlotTimer {
		&self.slot_timer
	}
}
