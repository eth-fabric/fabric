use axum::response::IntoResponse;
use lazy_static::lazy_static;
use prometheus::{
    Encoder, HistogramVec, IntCounterVec, Registry, TextEncoder,
    register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
};

pub const REGISTRY_NAME: &str = "commitments-rpc-service-metrics";

lazy_static! {
    pub static ref COMMITMENTS_METRICS_REGISTRY: Registry =
        Registry::new_custom(Some(REGISTRY_NAME.to_string()), None).unwrap();
    pub static ref RPC_CALLS_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
        "rpc_calls_total",
        "Total number of JSON-RPC method calls",
        &["method"],
        COMMITMENTS_METRICS_REGISTRY
    )
    .unwrap();
    pub static ref RPC_RESPONSES_TOTAL: IntCounterVec = register_int_counter_vec_with_registry!(
        "rpc_responses_total",
        "Total number of JSON-RPC responses by status",
        &["method", "status"],
        COMMITMENTS_METRICS_REGISTRY
    )
    .unwrap();
    pub static ref RPC_LATENCY_SECONDS: HistogramVec = register_histogram_vec_with_registry!(
        "rpc_request_duration_seconds",
        "JSON-RPC method latency in seconds",
        &["method"],
        COMMITMENTS_METRICS_REGISTRY
    )
    .unwrap();
}

pub async fn metrics_handler() -> axum::response::Response {
    let metric_families = COMMITMENTS_METRICS_REGISTRY.gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    if encoder.encode(&metric_families, &mut buffer).is_err() {
        return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, encoder.format_type())
        .body(axum::body::Body::from(buffer))
        .unwrap()
}
