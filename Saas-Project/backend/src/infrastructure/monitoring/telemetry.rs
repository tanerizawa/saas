use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use prometheus::{Encoder, TextEncoder};
use std::sync::Arc;

// Initialize telemetry and return prometheus registry
pub fn init_telemetry() -> prometheus::Registry {
    // Create a new registry
    let registry = prometheus::Registry::new();

    // Register default collectors
    prometheus::default_registry()
        .register(Box::new(
            prometheus::process_collector::ProcessCollector::for_self(),
        ))
        .expect("Failed to register process collector");

    registry
}

// Handler for metrics endpoint
pub async fn metrics_handler(
    State(registry): State<Arc<prometheus::Registry>>,
) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", encoder.format_type())
        .body(buffer)
        .unwrap()
}
