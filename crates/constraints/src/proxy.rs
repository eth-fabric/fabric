use std::sync::Arc;

use axum::{
	body::Body,
	extract::{Request, State},
	http::StatusCode,
	response::Response,
};
use reqwest::Client;
use tracing::{error, info};

/// Trait for types that can provide proxy state
pub trait ProxyState: Send + Sync + 'static {
	fn server_url(&self) -> &str;
	fn http_client(&self) -> &Client;
}

/// Proxy handler for forwarding unmatched requests to downstream relay
pub async fn proxy_handler<A>(State(state): State<Arc<A>>, req: Request) -> Result<Response, StatusCode>
where
	A: ProxyState,
{
	let method = req.method().clone();
	let uri = req.uri().clone();
	let path = uri.path();
	let query = uri.query().map(|q| format!("?{}", q)).unwrap_or_default();

	// Build downstream relay URL
	let downstream_full_url = format!("{}{}{}", state.server_url(), path, query);
	info!("Proxying {} {} to {}", method, path, downstream_full_url);

	// Extract headers and body
	let headers = req.headers().clone();
	let body_bytes = match axum::body::to_bytes(req.into_body(), usize::MAX).await {
		Ok(bytes) => bytes,
		Err(e) => {
			error!("Failed to read request body: {}", e);
			return Err(StatusCode::BAD_REQUEST);
		}
	};

	// Build downstream request
	let mut downstream_req = state.http_client().request(method.clone(), &downstream_full_url);

	// Forward headers (excluding host and connection-related headers)
	for (key, value) in headers.iter() {
		let key_str = key.as_str();
		if key_str != "host" && key_str != "connection" && !key_str.starts_with("x-forwarded") {
			if let Ok(val) = value.to_str() {
				downstream_req = downstream_req.header(key_str, val);
			}
		}
	}

	// Add body if present
	if !body_bytes.is_empty() {
		downstream_req = downstream_req.body(body_bytes.to_vec());
	}

	// Send request
	let response = match downstream_req.send().await {
		Ok(resp) => resp,
		Err(e) => {
			error!("Failed to proxy request to downstream relay: {}", e);
			return Err(StatusCode::BAD_GATEWAY);
		}
	};

	// Build response
	let status = response.status();
	let mut response_builder = Response::builder().status(status);

	// Copy response headers
	for (key, value) in response.headers() {
		response_builder = response_builder.header(key, value);
	}

	// Get response body
	let body_bytes = match response.bytes().await {
		Ok(bytes) => bytes,
		Err(e) => {
			error!("Failed to read downstream relay response body: {}", e);
			return Err(StatusCode::BAD_GATEWAY);
		}
	};

	// Build final response
	match response_builder.body(Body::from(body_bytes)) {
		Ok(response) => {
			info!("Proxy response: {} for {} {}", status, method, path);
			Ok(response)
		}
		Err(e) => {
			error!("Failed to build response: {}", e);
			Err(StatusCode::INTERNAL_SERVER_ERROR)
		}
	}
}
