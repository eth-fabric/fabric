use lazy_static::lazy_static;
use prometheus::{
	HistogramVec, IntCounterVec, Registry, register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
};

use common::metrics::HttpMetrics;

pub const CLIENT_REGISTRY_NAME: &str = "constraints-client";
pub const SERVER_REGISTRY_NAME: &str = "constraints-server";

lazy_static! {
	pub static ref CONSTRAINTS_CLIENT_REGISTRY: Registry =
		Registry::new_custom(Some(CLIENT_REGISTRY_NAME.to_string()), None).unwrap();

	// Constraints client metrics (for HTTP calls to relay)
	pub static ref CONSTRAINTS_CLIENT_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"constraints_client_requests_total",
		"Total HTTP requests to relay by endpoint and method",
		&["endpoint", "method"],
		CONSTRAINTS_CLIENT_REGISTRY
	)
	.unwrap();

	pub static ref CONSTRAINTS_CLIENT_RESPONSES_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
		"constraints_client_responses_total",
		"Total HTTP responses from relay by endpoint, method, and status",
		&["endpoint", "method", "status"],
		CONSTRAINTS_CLIENT_REGISTRY
	)
	.unwrap();

	pub static ref CONSTRAINTS_CLIENT_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
		"constraints_client_latency_seconds",
		"HTTP request latency to relay in seconds by endpoint and method",
		&["endpoint", "method"],
		CONSTRAINTS_CLIENT_REGISTRY
	)
	.unwrap();
	pub static ref CONSTRAINTS_SERVER_METRICS_REGISTRY: Registry =
		Registry::new_custom(Some(SERVER_REGISTRY_NAME.to_string()), None).unwrap();
	pub static ref CONSTRAINTS_SERVER_REQUESTS_TOTAL: IntCounterVec =
		register_int_counter_vec_with_registry!(
			"http_requests_total",
			"Total number of HTTP requests",
			&["endpoint", "method"],
			CONSTRAINTS_SERVER_METRICS_REGISTRY
		)
		.unwrap();
	pub static ref CONSTRAINTS_SERVER_RESPONSES_TOTAL: IntCounterVec =
		register_int_counter_vec_with_registry!(
			"http_responses_total",
			"Total number of HTTP responses by status",
			&["endpoint", "method", "status"],
			CONSTRAINTS_SERVER_METRICS_REGISTRY
		)
		.unwrap();
	pub static ref CONSTRAINTS_SERVER_REQUEST_LATENCY_SECONDS: HistogramVec =
		register_histogram_vec_with_registry!(
			"http_request_duration_seconds",
			"Request latency in seconds",
			&["endpoint", "method"],
			CONSTRAINTS_SERVER_METRICS_REGISTRY
		)
		.unwrap();
}

// helper for server side
pub fn server_http_metrics() -> HttpMetrics {
	HttpMetrics {
		requests: &CONSTRAINTS_SERVER_REQUESTS_TOTAL,
		responses: &CONSTRAINTS_SERVER_RESPONSES_TOTAL,
		latency: &CONSTRAINTS_SERVER_REQUEST_LATENCY_SECONDS,
	}
}

// and similarly for client side if you want to share the same helper:
pub fn client_http_metrics() -> HttpMetrics {
	HttpMetrics {
		requests: &CONSTRAINTS_CLIENT_REQUESTS_TOTAL,
		responses: &CONSTRAINTS_CLIENT_RESPONSES_TOTAL,
		latency: &CONSTRAINTS_CLIENT_LATENCY_SECONDS,
	}
}
