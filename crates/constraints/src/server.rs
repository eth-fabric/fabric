use std::sync::Arc;

use axum::{
	Json, Router,
	extract::{Path, State},
	http::{HeaderMap, StatusCode},
	response::IntoResponse,
	routing::{get, post},
};
use axum_reverse_proxy::ReverseProxy;
use reqwest::Client;
use tracing::{error, info};

use crate::api::ConstraintsApi;
use crate::metrics::server_http_metrics;
use crate::routes;
use crate::types::{AuthorizationContext, SignedConstraints, SignedDelegation, SubmitBlockRequestWithProofs};


/// Build an Axum router for the Constraints REST API,
/// using any implementation of `ConstraintsApi`.
pub fn build_constraints_router<A>(api: A) -> Router
where
	A: ConstraintsApi,
{
	let state = Arc::new(api);

	Router::new()
		.route(routes::HEALTH, get(health::<A>))
		.route(routes::CAPABILITIES, get(get_capabilities::<A>))
		.route(routes::CONSTRAINTS, post(post_constraints::<A>))
		.route(routes::CONSTRAINTS_SLOT, get(get_constraints::<A>))
		.route(routes::DELEGATION, post(post_delegation::<A>))
		.route(routes::DELEGATIONS_SLOT, get(get_delegations::<A>))
		.route(routes::BLOCKS_WITH_PROOFS, post(post_blocks_with_proofs::<A>))
		.with_state(state)
}

pub trait ProxyState: Send + Sync + 'static {
    fn server_url(&self) -> &str;
    fn http_client(&self) -> &Client;
}

/// Build an Axum router for the Constraints REST API with a proxy fallback,
/// using any implementation of `ConstraintsApi` and `ProxyState`.
pub fn build_constraints_router_with_proxy<A>(api: A) -> Router
where
	A: ConstraintsApi + ProxyState,
{
	let state = Arc::new(api);

	// This forwards every path and query to the downstream server URL.
    let proxy = ReverseProxy::new("/", state.server_url());

	Router::new()
		.route(routes::HEALTH, get(health::<A>))
		.route(routes::CAPABILITIES, get(get_capabilities::<A>))
		.route(routes::CONSTRAINTS, post(post_constraints::<A>))
		.route(routes::CONSTRAINTS_SLOT, get(get_constraints::<A>))
		.route(routes::DELEGATION, post(post_delegation::<A>))
		.route(routes::DELEGATIONS_SLOT, get(get_delegations::<A>))
		.route(routes::BLOCKS_WITH_PROOFS, post(post_blocks_with_proofs::<A>))
		.fallback_service(proxy)
		.with_state(state)
}

// ---------- Handlers ----------

// GET /health
async fn health<A>(State(api): State<Arc<A>>) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::HEALTH;
	const METHOD: &str = "GET";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match api.health_check().await {
		Ok(()) => {
			metrics.finish_status(ENDPOINT, METHOD, 200, start);
			StatusCode::OK
		}
		Err(_) => {
			metrics.finish_status(ENDPOINT, METHOD, 500, start);
			StatusCode::INTERNAL_SERVER_ERROR
		}
	}
}

// GET /capabilities
async fn get_capabilities<A>(State(api): State<Arc<A>>) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::CAPABILITIES;
	const METHOD: &str = "GET";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match api.get_capabilities().await {
		Ok(capabilities) => {
			metrics.finish_status(ENDPOINT, METHOD, 200, start);
			(StatusCode::OK, Json(capabilities)).into_response()
		}
		Err(e) => {
			metrics.finish_status(ENDPOINT, METHOD, 500, start);
			(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to fetch capabilities: {e}")).into_response()
		}
	}
}

// POST /constraints
async fn post_constraints<A>(State(api): State<Arc<A>>, Json(body): Json<SignedConstraints>) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::CONSTRAINTS;
	const METHOD: &str = "POST";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match api.post_constraints(body).await {
		Ok(()) => {
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::OK.as_u16(), start);
			StatusCode::OK.into_response()
		}
		Err(e) => {
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
			(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to store constraints: {e}")).into_response()
		}
	}
}

// GET /constraints/{slot}
async fn get_constraints<A>(State(api): State<Arc<A>>, Path(slot): Path<u64>, headers: HeaderMap) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::CONSTRAINTS_SLOT;
	const METHOD: &str = "GET";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match AuthorizationContext::from_headers(&headers) {
		Ok(auth) => match api.get_constraints(slot, auth).await {
			Ok(constraints) => {
				metrics.finish_status(ENDPOINT, METHOD, StatusCode::OK.as_u16(), start);
				(StatusCode::OK, Json(constraints)).into_response()
			}
			Err(e) => {
				metrics.finish_status(ENDPOINT, METHOD, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
				(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to get constraints for slot {slot}: {e}"))
					.into_response()
			}
		},
		Err(e) => {
			return (StatusCode::BAD_REQUEST, format!("failed to get constraints for slot {slot}: {e}"))
				.into_response();
		}
	}
}

// POST /delegation
async fn post_delegation<A>(State(api): State<Arc<A>>, Json(body): Json<SignedDelegation>) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::DELEGATION;
	const METHOD: &str = "POST";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match api.post_delegation(body).await {
		Ok(()) => {
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::OK.as_u16(), start);
			StatusCode::OK.into_response()
		}
		Err(e) => {
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
			(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to store delegation: {e}")).into_response()
		}
	}
}

// GET /delegations/{slot}
async fn get_delegations<A>(State(api): State<Arc<A>>, Path(slot): Path<u64>) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::DELEGATIONS_SLOT;
	const METHOD: &str = "GET";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match api.get_delegations(slot).await {
		Ok(delegations) => {
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::OK.as_u16(), start);
			(StatusCode::OK, Json(delegations)).into_response()
		}
		Err(e) => {
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
			(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to get delegations for slot {slot}: {e}"))
				.into_response()
		}
	}
}

// POST /blocks_with_proofs
async fn post_blocks_with_proofs<A>(
	State(api): State<Arc<A>>,
	headers: HeaderMap,
	Json(body): Json<SubmitBlockRequestWithProofs>,
) -> impl IntoResponse
where
	A: ConstraintsApi,
{
	const ENDPOINT: &str = routes::BLOCKS_WITH_PROOFS;
	const METHOD: &str = "POST";

	let metrics = server_http_metrics();
	let start = metrics.start(ENDPOINT, METHOD);

	match api.post_blocks_with_proofs(body, headers).await {
		Ok(()) => {
			info!("Blocks with proofs submitted successfully");
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::OK.as_u16(), start);
			StatusCode::OK.into_response()
		}
		Err(e) => {
			error!("Failed to submit blocks with proofs: {e}");
			metrics.finish_status(ENDPOINT, METHOD, StatusCode::INTERNAL_SERVER_ERROR.as_u16(), start);
			(StatusCode::INTERNAL_SERVER_ERROR, format!("failed to submit blocks with proofs: {e}")).into_response()
		}
	}
}
