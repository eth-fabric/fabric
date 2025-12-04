use std::net::SocketAddr;

use axum::{Router, routing::get};
use eyre::Result;
use jsonrpsee::server::{RpcModule, Server};

use super::metrics::server_metrics_handler;
use crate::rpc::CommitmentsRpcServer;

/// Extra info the server harness needs from a handler.
///
/// Every implementation that wants to use `run_commitments_rpc_server` must
/// provide these two methods.
pub trait CommitmentsServerInfo {
    fn server_addr(&self) -> SocketAddr;
    fn metrics_addr(&self) -> SocketAddr;
}

pub async fn run_commitments_rpc_server<H>(handlers: H) -> Result<()>
where
    H: CommitmentsRpcServer + CommitmentsServerInfo + Clone + Send + Sync + 'static,
{
    // Get addresses from the handler
    let server_addr: SocketAddr = handlers.server_addr();
    let metrics_addr: SocketAddr = handlers.metrics_addr();

    let server = Server::builder().build(server_addr).await?;
    let module: RpcModule<_> = handlers.into_rpc();

    let addr = server.local_addr()?;
    tracing::info!("Starting Commitments RPC server on {}", addr);

    // Spawn metrics server
    tokio::spawn(async move {
        let app = Router::new().route("/metrics", get(server_metrics_handler));
        match tokio::net::TcpListener::bind(metrics_addr).await {
            Ok(listener) => {
                if let Err(e) = axum::serve(listener, app).await {
                    tracing::error!("metrics server error: {}", e);
                }
            }
            Err(e) => tracing::error!("failed to bind metrics listener: {}", e),
        }
    });

    let handle = server.start(module);
    handle.stopped().await;

    Ok(())
}
