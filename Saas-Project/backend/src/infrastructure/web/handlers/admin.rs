// Admin dashboard handlers
#![allow(dead_code)]

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::json;

/// Placeholder handler for admin endpoints
pub async fn placeholder() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Admin endpoint - coming soon",
        "status": "placeholder"
    })))
}

pub fn routes() -> Router {
    Router::new()
        .route("/dashboard", get(|| async { "Admin dashboard" }))
        .route("/users", get(|| async { "Manage users" }))
        .route("/licenses/pending", get(|| async { "Pending licenses" }))
        .route("/reports", get(|| async { "System reports" }))
        .route("/settings", get(|| async { "System settings" }))
}
