use commit_boost::prelude::StartCommitModuleConfig;
use common::storage::DatabaseContext;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::gateway::config::GatewayConfig;

/// Server state that provides access to shared resources for all RPC handlers
/// This holds runtime resources (database connections, RPC clients, timers) needed by the commitments service
#[derive(Clone)]
pub struct GatewayState {
    /// Storage
    pub db: DatabaseContext,
    /// Commit module configuration for commit-boost operations (Arc<Mutex> for thread safety)
    pub config: Arc<Mutex<StartCommitModuleConfig<GatewayConfig>>>,
    /// Relay URL for constraints communication
    pub relay_url: SocketAddr,
    /// API key for relay authentication
    pub api_key: Option<String>,
}

impl GatewayState {
    /// Create a new commitments server state with the given database contexts and commit config
    pub fn new(
        db: DatabaseContext,
        config: StartCommitModuleConfig<GatewayConfig>,
        relay_url: SocketAddr,
        api_key: Option<String>,
    ) -> Self {
        Self {
            db,
            config: Arc::new(Mutex::new(config)),
            relay_url,
            api_key,
        }
    }

    /// Get a reference to the database
    pub fn db(&self) -> &DatabaseContext {
        &self.db
    }
}
