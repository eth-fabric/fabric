use axum::{Router, routing::get};
use eyre::Result;
use jsonrpsee::server::{RpcModule, Server};
use reqwest::Url;

use super::metrics::server_metrics_handler;
use crate::rpc::CommitmentsRpcServer;

/// Extra info the server harness needs from a handler.
///
/// Every implementation that wants to use `run_commitments_rpc_server` must
/// provide these two methods.
pub trait CommitmentsServerInfo {
	fn server_url(&self) -> Url;
	fn metrics_url(&self) -> Url;
}

pub async fn run_commitments_rpc_server<H>(handlers: H) -> Result<()>
where
	H: CommitmentsRpcServer + CommitmentsServerInfo + Clone + Send + Sync + 'static,
{
	// Get urls from the handler
	let server_url: Url = handlers.server_url();
	let metrics_url: Url = handlers.metrics_url();

	// Get socket addresses
	let server_socket = server_url.socket_addrs(|| None)?;
	let server_socket = server_socket.first().ok_or(eyre::eyre!("Failed to get first socket address"))?;

	let metrics_socket = metrics_url.socket_addrs(|| None)?;
	let metrics_socket = *metrics_socket.first().ok_or(eyre::eyre!("Failed to get first socket address"))?;

	let server = Server::builder().build(server_socket).await?;
	let module: RpcModule<_> = handlers.into_rpc();

	let addr = server.local_addr()?;
	tracing::info!("Starting Commitments RPC server on {}", addr);

	// Spawn metrics server
	tokio::spawn(async move {
		let app = Router::new().route("/metrics", get(server_metrics_handler));
		match tokio::net::TcpListener::bind(metrics_socket).await {
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
