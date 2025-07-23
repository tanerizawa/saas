use axum::{
    response::Json,
    http::StatusCode,
};
use serde_json::json;

/// Placeholder handler for file endpoints
pub async fn placeholder() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({
        "message": "Files endpoint - coming soon",
        "status": "placeholder"
    })))
}
