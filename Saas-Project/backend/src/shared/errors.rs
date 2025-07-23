// Application error types and handling
// Centralized error management following Rust best practices

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("External API error: {0}")]
    ExternalApi(String),
    
    #[error("File processing error: {0}")]
    FileProcessing(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, error_code) = match &self {
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
                "DATABASE_ERROR",
            ),
            AppError::Authentication(msg) => (
                StatusCode::UNAUTHORIZED,
                msg.clone(),
                "AUTHENTICATION_ERROR",
            ),
            AppError::Authorization(msg) => (
                StatusCode::FORBIDDEN,
                msg.clone(),
                "AUTHORIZATION_ERROR",
            ),
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                msg.clone(),
                "VALIDATION_ERROR",
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg.clone(),
                "NOT_FOUND",
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                msg.clone(),
                "CONFLICT",
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                msg.clone(),
                "UNAUTHORIZED",
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                msg.clone(),
                "FORBIDDEN",
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg.clone(),
                "BAD_REQUEST",
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
                "INTERNAL_ERROR",
            ),
            AppError::ExternalApi(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("External service error: {}", msg),
                "EXTERNAL_API_ERROR",
            ),
            AppError::FileProcessing(msg) => (
                StatusCode::BAD_REQUEST,
                msg.clone(),
                "FILE_PROCESSING_ERROR",
            ),
            AppError::RateLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                "Rate limit exceeded".to_string(),
                "RATE_LIMIT_EXCEEDED",
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
                "INTERNAL_ERROR",
            ),
            AppError::NotImplemented(msg) => (
                StatusCode::NOT_IMPLEMENTED,
                msg.clone(),
                "NOT_IMPLEMENTED",
            ),
        };

        // Log internal errors for debugging (don't expose sensitive info to client)
        if matches!(self, AppError::Database(_) | AppError::Internal(_)) {
            tracing::error!("Internal error: {:?}", self);
        }

        let body = Json(json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        }));

        (status, body).into_response()
    }
}

// Convenience type alias
pub type AppResult<T> = Result<T, AppError>;
