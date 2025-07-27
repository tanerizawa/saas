use axum::{http::StatusCode, response::Json};
use serde_json::json;

/// Placeholder handler for finance endpoints
pub async fn placeholder() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Finance endpoint - coming soon",
        "status": "placeholder"
    })))
}
