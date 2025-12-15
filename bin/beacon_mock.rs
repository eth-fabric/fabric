use alloy::primitives::B256;
use axum::{Json, Router, extract::Path, routing::get};
use eyre::Result;
use lookahead::constants::PROPOSER_DUTIES_ROUTE;
use lookahead::types::{ProposerDutiesResponse, ValidatorDuty};
use lookahead::utils::{epoch_to_first_slot, epoch_to_last_slot};
use tracing::info;

/// Handler for proposer duties endpoint
async fn get_proposer_duties_handler(
	Path(epoch): Path<u64>,
	axum::extract::State(proposer_key): axum::extract::State<String>,
) -> Json<ProposerDutiesResponse> {
	// Calculate slot range for epoch (32 slots per epoch)
	let start_slot = epoch_to_first_slot(epoch);
	let end_slot = epoch_to_last_slot(epoch);

	info!("Getting proposer duties for epoch {} from slot {} to slot {}", epoch, start_slot, end_slot);

	// Generate alternating duties
	// Even slots: use provided proposer key
	// Odd slots: use default random key
	let duties: Vec<ValidatorDuty> = (start_slot..=end_slot)
		.map(|slot| {
			let is_even = slot % 2 == 0;
			let pubkey = if is_even {
				proposer_key.clone()
			} else {
				// Default random but valid BLS public key
				"0x879d322fb401a2638b6217cab6e9bf954e6df9b18e0c302f3bdc00551a8ac308459d8a79eb54f0f272e6b648ee4d03b3"
					.to_string()
			};

			ValidatorDuty { validator_index: slot.to_string(), pubkey, slot: slot.to_string() }
		})
		.collect();

	Json(ProposerDutiesResponse {
		execution_optimistic: false,
		dependent_root: B256::from_slice(&[0; 32]),
		data: duties,
	})
}

#[tokio::main]
async fn main() -> Result<()> {
	// Read env vars
	let log_level = std::env::var("RUST_LOG").unwrap_or("info".to_string());
	let host = std::env::var("BEACON_HOST").expect("BEACON_HOST environment variable not set");
	let port = std::env::var("BEACON_PORT").expect("BEACON_PORT environment variable not set");
	let proposer_key = std::env::var("PROPOSER_KEY").expect("PROPOSER_KEY environment variable not set");
	common::logging::setup_logging(&log_level)?;

	let bind_addr = format!("{}:{}", host, port);

	info!("Mock Beacon Node Server");
	info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	info!("Listening on: {}", bind_addr);
	info!("Proposer key: {}", proposer_key);
	info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	info!("Endpoint: GET /eth/v1/validator/duties/proposer/{{epoch}}");
	info!("Pattern: Even slots = proposer key, Odd slots = random key 0x87d322...");

	// Build router with proposer key as shared state
	let app = Router::new()
		.route(
			format!("/{}/{{epoch}}", PROPOSER_DUTIES_ROUTE).as_str(),
			// PROPOSER_DUTIES_ROUTE,
			get(get_proposer_duties_handler),
		)
		.with_state(proposer_key);

	// Bind to the specified address
	let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

	info!("Mock Beacon Node server ready");

	// Start server
	axum::serve(listener, app).await?;

	Ok(())
}
