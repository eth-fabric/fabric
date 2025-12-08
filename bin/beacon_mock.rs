use axum::{Json, Router, extract::Path, routing::get};
use clap::Parser;
use lookahead::types::{ProposerDutiesResponse, ValidatorDuty};
use lookahead::utils::{epoch_to_first_slot, epoch_to_last_slot};
use eyre::Result;
use tracing::info;

/// Mock beacon node that returns alternating proposer duties
#[derive(Parser, Debug)]
#[command(name = "beacon-mock")]
#[command(about = "Mock beacon node for testing")]
struct Args {
	/// Beacon API URL to bind to (e.g., http://localhost:5052)
	#[arg(long, default_value = "http://0.0.0.0:5052")]
	url: String,

	/// Proposer BLS public key (hex format with 0x prefix)
	proposer_bls_key: String,
}

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

	Json(ProposerDutiesResponse { execution_optimistic: false, finalized: true, data: duties })
}

#[tokio::main]
async fn main() -> Result<()> {
	common::logging::setup("info".into())?;
	// Parse command line arguments
	let args = Args::parse();

	// Parse the URL to extract host and port
	let url = args.url.parse::<url::Url>().map_err(|e| eyre::eyre!("Invalid URL '{}': {}", args.url, e))?;

	let host = url.host_str().unwrap_or("0.0.0.0");
	let port = url.port().unwrap_or(5052);
	let bind_addr = format!("{}:{}", host, port);

	info!("Mock Beacon Node Server");
	info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	info!("Listening on: {}", bind_addr);
	info!("Proposer key: {}", args.proposer_bls_key);
	info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
	info!("Endpoint: GET /eth/v1/validator/duties/proposer/{{epoch}}");
	info!("Pattern: Even slots = proposer key, Odd slots = zero key");

	// Build router with proposer key as shared state
	let app = Router::new()
		.route("/eth/v1/validator/duties/proposer/{epoch}", get(get_proposer_duties_handler))
		.with_state(args.proposer_bls_key);

	// Bind to the specified address
	let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

	info!("Mock Beacon Node server ready");

	// Start server
	axum::serve(listener, app).await?;

	Ok(())
}
