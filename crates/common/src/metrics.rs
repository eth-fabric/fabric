use std::time::Instant;

use prometheus::{HistogramVec, IntCounterVec};

/// Generic helper that knows how to record HTTP metrics:
/// - requests_total
/// - request_duration_seconds
/// - responses_total
///
/// It does not care whether it is client or server.
#[derive(Clone, Copy)]
pub struct HttpMetrics {
    pub requests: &'static IntCounterVec,
    pub responses: &'static IntCounterVec,
    pub latency: &'static HistogramVec,
}

impl HttpMetrics {
    pub fn start(&self, endpoint: &'static str, method: &'static str) -> Instant {
        self.requests.with_label_values(&[endpoint, method]).inc();
        Instant::now()
    }

    pub fn finish_status(
        &self,
        endpoint: &'static str,
        method: &'static str,
        status: u16,
        start: Instant,
    ) {
        let status_str = status.to_string();

        self.latency
            .with_label_values(&[endpoint, method])
            .observe(start.elapsed().as_secs_f64());

        self.responses
            .with_label_values(&[endpoint, method, &status_str])
            .inc();
    }

    pub fn finish_label(
        &self,
        endpoint: &'static str,
        method: &'static str,
        status_label: &str,
        start: Instant,
    ) {
        self.latency
            .with_label_values(&[endpoint, method])
            .observe(start.elapsed().as_secs_f64());

        self.responses
            .with_label_values(&[endpoint, method, status_label])
            .inc();
    }
}
