use prometheus::{register_counter_vec, register_histogram_vec, CounterVec, HistogramVec};
use once_cell::sync::Lazy;

// HTTP metrics
pub static HTTP_REQUESTS_COUNTER: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "http_requests_total",
        "Total number of HTTP requests",
        &["method", "endpoint", "status_code"]
    )
    .unwrap()
});

pub static HTTP_REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds",
        &["method", "endpoint"]
    )
    .unwrap()
});

// Domain metrics - License operations
pub static LICENSE_OPERATIONS: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "domain_license_operations_total",
        "Total number of license domain operations",
        &["operation", "license_type"]
    )
    .unwrap()
});

// Domain metrics - Company operations
pub static COMPANY_OPERATIONS: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "domain_company_operations_total",
        "Total number of company domain operations",
        &["operation"]
    )
    .unwrap()
});

// Auth metrics
pub static AUTH_OPERATIONS: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "domain_auth_operations_total",
        "Total number of authentication operations",
        &["operation", "status"]
    )
    .unwrap()
});

pub static AUTH_ACTIVE_SESSIONS: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "domain_auth_active_sessions",
        "Number of currently active user sessions",
        &[]
    )
    .unwrap()
});

// Database metrics
pub static DB_QUERIES_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "db_query_duration_seconds",
        "Database query duration in seconds",
        &["query_type", "status"]
    )
    .unwrap()
});

pub static DB_CONNECTIONS: Lazy<CounterVec> = Lazy::new(|| {
    register_counter_vec!(
        "db_connections_total",
        "Total number of database connection operations",
        &["operation"]
    )
    .unwrap()
});

// Helper functions for tracking metrics

pub fn track_http_request(method: &str, endpoint: &str, status_code: &str) {
    HTTP_REQUESTS_COUNTER
        .with_label_values(&[method, endpoint, status_code])
        .inc();
}

pub fn track_http_request_duration<F, R>(method: &str, endpoint: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let timer = HTTP_REQUEST_DURATION
        .with_label_values(&[method, endpoint])
        .start_timer();
    let result = f();
    timer.observe();
    result
}

pub fn track_license_operation(operation: &str, license_type: &str) {
    LICENSE_OPERATIONS
        .with_label_values(&[operation, license_type])
        .inc();
}

pub fn track_company_operation(operation: &str) {
    COMPANY_OPERATIONS.with_label_values(&[operation]).inc();
}

pub fn track_auth_operation(operation: &str, status: &str) {
    AUTH_OPERATIONS
        .with_label_values(&[operation, status])
        .inc();
}

pub fn track_db_query<F, R>(query_type: &str, f: F) -> Result<R, String>
where
    F: FnOnce() -> Result<R, String>,
{
    let timer = DB_QUERIES_DURATION
        .with_label_values(&[query_type, "in_progress"])
        .start_timer();
    
    match f() {
        Ok(result) => {
            drop(timer);
            DB_QUERIES_DURATION
                .with_label_values(&[query_type, "success"])
                .observe(timer.stop_and_record());
            Ok(result)
        }
        Err(e) => {
            drop(timer);
            DB_QUERIES_DURATION
                .with_label_values(&[query_type, "error"])
                .observe(timer.stop_and_record());
            Err(e)
        }
    }
}
