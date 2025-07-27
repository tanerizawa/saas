use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    body::Body,
};
use std::collections::HashMap;
use std::{
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing::{error, warn};

// For shared state
use crate::infrastructure::web::handlers::AppState;

// Simple in-memory rate limiter
// For production, this would be better implemented with Redis
#[derive(Clone)]
pub struct RateLimiter {
    // Map of IP addresses to a list of request timestamps
    requests: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    // Maximum requests allowed in the window
    max_requests: usize,
    // Time window in seconds
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_secs,
        }
    }

    async fn is_rate_limited(&self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);

        let mut requests = self.requests.lock().await;

        // Get or insert the IP address
        let timestamps = requests.entry(ip).or_insert_with(Vec::new);

        // Remove timestamps outside the window
        timestamps.retain(|&timestamp| now.duration_since(timestamp) < window);

        // Check if the request count exceeds the limit
        if timestamps.len() >= self.max_requests {
            return true;
        }

        // Add the current timestamp
        timestamps.push(now);

        false
    }

    // Cleanup old entries periodically (called from a background task)
    pub async fn cleanup(&self) {
        let now = Instant::now();
        let window = Duration::from_secs(self.window_secs);

        let mut requests = self.requests.lock().await;

        // Remove old timestamps for all IPs
        for timestamps in requests.values_mut() {
            timestamps.retain(|&timestamp| now.duration_since(timestamp) < window);
        }

        // Remove empty entries
        requests.retain(|_, timestamps| !timestamps.is_empty());
    }
}

// Extract client IP from headers or socket
fn get_client_ip(headers: &HeaderMap, connection_info: Option<IpAddr>) -> Option<IpAddr> {
    // Try X-Forwarded-For header first (if behind proxy/load balancer)
    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            // Get the first IP in the list
            if let Some(ip_str) = forwarded_str.split(',').next() {
                if let Ok(ip) = ip_str.trim().parse::<IpAddr>() {
                    return Some(ip);
                }
            }
        }
    }

    // Fall back to connection info (direct client IP)
    connection_info
}

// Middleware function for rate limiting
pub async fn rate_limit(
    State(state): State<AppState>,
    headers: HeaderMap,
    connection_info: Option<IpAddr>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip rate limiting if rate limiter not configured
    let rate_limiter = match &state.config().rate_limiter {
        Some(limiter) => limiter.clone(),
        None => return Ok(next.run(request).await),
    };

    // Get client IP
    let client_ip = match get_client_ip(&headers, connection_info) {
        Some(ip) => ip,
        None => {
            error!("Failed to determine client IP address");
            return Ok(StatusCode::BAD_REQUEST.into_response());
        }
    };

    // Check if client is rate limited
    if rate_limiter.is_rate_limited(client_ip).await {
        warn!("Rate limit exceeded for IP: {}", client_ip);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    // Continue with the request
    Ok(next.run(request).await)
}

// Background task to clean up old rate limiter entries
pub async fn cleanup_rate_limiter(rate_limiter: RateLimiter) {
    let cleanup_interval = Duration::from_secs(60); // Clean up once per minute

    loop {
        tokio::time::sleep(cleanup_interval).await;
        rate_limiter.cleanup().await;
    }
}
