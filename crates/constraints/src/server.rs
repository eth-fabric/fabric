use std::{sync::Arc, time::Duration};

use axum::{
	Json, Router,
	body::Body,
	extract::{Path, State},
	http::{HeaderMap, Request, StatusCode},
	response::IntoResponse,
	routing::{get, post},
};
use axum_reverse_proxy::ReverseProxy;
use reqwest::Client;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{Level, Span, error, info};

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

#[derive(Clone)]
struct DownstreamUrl(String);

/// Build an Axum router for the Constraints REST API with a proxy fallback,
/// using any implementation of `ConstraintsApi` and `ProxyState`.
pub fn build_constraints_router_with_proxy<A>(api: A) -> Router
where
	A: ConstraintsApi + ProxyState,
{
	let state = Arc::new(api);
	let downstream_url = state.server_url().to_string();

	// This forwards every path and query to the downstream server URL.
	let proxy = ReverseProxy::new("/", downstream_url.as_str());

	// Add logging to proxy requests
	let proxy = ServiceBuilder::new()
		.map_request(move |mut req: Request<Body>| {
			let path = req.uri().path();
			let query = req.uri().query().map(|q| format!("?{q}")).unwrap_or_default();
			let downstream_full_url = format!("{}{}{}", downstream_url.as_str(), path, query);

			req.extensions_mut().insert(DownstreamUrl(downstream_full_url));
			req
		})
		.layer(
			TraceLayer::new_for_http()
				.make_span_with(|req: &Request<_>| {
					let downstream =
						req.extensions().get::<DownstreamUrl>().map(|d| d.0.as_str()).unwrap_or("<missing>");

					tracing::span!(
						Level::INFO,
						"proxy",
						method = %req.method(),
						uri = %req.uri(),
						downstream = %downstream,
					)
				})
				.on_response(|res: &axum::http::Response<_>, latency: Duration, _span: &Span| {
					info!(
						status = %res.status(),
						latency_ms = latency.as_millis(),
						"proxied"
					);
				}),
		)
		.service(proxy);

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
