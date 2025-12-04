use axum::response::{IntoResponse, Response};
use lazy_static::lazy_static;
use prometheus::{
    Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder,
    register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
};

use common::metrics::HttpMetrics;

pub const COMMITMENTS_CLIENT_REGISTRY_NAME: &str = "commitments-client";
pub const COMMITMENTS_SERVER_REGISTRY_NAME: &str = "commitments-server";

lazy_static! {
    pub static ref COMMITMENTS_CLIENT_REGISTRY: Registry =
        Registry::new_custom(Some(COMMITMENTS_CLIENT_REGISTRY_NAME.to_string()), None).unwrap();

    // Commitments client metrics
    pub static ref COMMITMENTS_CLIENT_REQUESTS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
        "commitments_client_requests_total",
        "Total HTTP requests to relay by endpoint and method",
        &["endpoint", "method"],
        COMMITMENTS_CLIENT_REGISTRY
    )
    .unwrap();

    pub static ref COMMITMENTS_CLIENT_RESPONSES_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
        "commitments_client_responses_total",
        "Total HTTP responses from relay by endpoint, method, and status",
        &["endpoint", "method", "status"],
        COMMITMENTS_CLIENT_REGISTRY
    )
    .unwrap();

    pub static ref COMMITMENTS_CLIENT_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
        "commmitments_client_latency_seconds",
        "HTTP request latency to relay in seconds by endpoint and method",
        &["endpoint", "method"],
        COMMITMENTS_CLIENT_REGISTRY
    )
    .unwrap();
    pub static ref COMMITMENTS_SERVER_METRICS_REGISTRY: Registry =
        Registry::new_custom(Some(COMMITMENTS_SERVER_REGISTRY_NAME.to_string()), None).unwrap();
    pub static ref COMMITMENTS_SERVER_REQUESTS_TOTAL: IntCounterVec =
        register_int_counter_vec_with_registry!(
            "http_requests_total",
            "Total number of HTTP requests",
            &["endpoint", "method"],
            COMMITMENTS_SERVER_METRICS_REGISTRY
        )
        .unwrap();
    pub static ref COMMITMENTS_SERVER_RESPONSES_TOTAL: IntCounterVec =
        register_int_counter_vec_with_registry!(
            "http_responses_total",
            "Total number of HTTP responses by status",
            &["endpoint", "method", "status"],
            COMMITMENTS_SERVER_METRICS_REGISTRY
        )
        .unwrap();
    pub static ref COMMITMENTS_SERVER_REQUEST_LATENCY_SECONDS: HistogramVec =
        register_histogram_vec_with_registry!(
            "http_request_duration_seconds",
            "Request latency in seconds",
            &["endpoint", "method"],
            COMMITMENTS_SERVER_METRICS_REGISTRY
        )
        .unwrap();
}

// helper for server side
pub fn server_http_metrics() -> HttpMetrics {
    HttpMetrics {
        requests: &COMMITMENTS_SERVER_REQUESTS_TOTAL,
        responses: &COMMITMENTS_SERVER_RESPONSES_TOTAL,
        latency: &COMMITMENTS_SERVER_REQUEST_LATENCY_SECONDS,
    }
}

// and similarly for client side if you want to share the same helper:
pub fn client_http_metrics() -> HttpMetrics {
    HttpMetrics {
        requests: &COMMITMENTS_CLIENT_REQUESTS_TOTAL,
        responses: &COMMITMENTS_CLIENT_RESPONSES_TOTAL,
        latency: &COMMITMENTS_CLIENT_LATENCY_SECONDS,
    }
}

pub async fn server_metrics_handler() -> Response {
    let metric_families = COMMITMENTS_SERVER_METRICS_REGISTRY.gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    if encoder.encode(&metric_families, &mut buffer).is_err() {
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    Response::builder()
        .status(axum::http::StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, encoder.format_type())
        .body(axum::body::Body::from(buffer))
        .unwrap()
}
