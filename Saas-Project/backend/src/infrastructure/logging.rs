use crate::config::Config;
use std::env;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::UtcTime},
    prelude::*,
    EnvFilter,
};

/// Configure structured logging based on environment
pub fn setup_logging(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Build filter based on configuration
    let filter = match config.environment.as_str() {
        "production" => EnvFilter::new("info,sqlx=warn,actix_web=info"),
        "staging" => EnvFilter::new("debug,sqlx=info,actix_web=info"),
        _ => EnvFilter::new("debug,sqlx=debug,actix_web=debug"),
    };

    // Build formatting layer
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(UtcTime::rfc_3339())
        .with_span_events(FmtSpan::CLOSE)
        .json();

    // Initialize subscriber with filters and formatters
    if config.environment == "production" {
        // Production: JSON logging for better parsing
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    } else {
        // Development: Pretty formatting for human readability
        tracing_subscriber::registry()
            .with(filter)
            .with(
                tracing_subscriber::fmt::layer()
                    .with_timer(UtcTime::rfc_3339())
                    .with_span_events(FmtSpan::CLOSE)
                    .pretty(),
            )
            .init();
    }

    Ok(())
}

/// Add request tracing middleware for HTTP requests
pub fn setup_request_tracing() -> impl actix_web::dev::Middleware {
    use actix_web::middleware::Logger;
    
    Logger::new("%r %s %b %D ms")
        .log_target("actix_web")
}
