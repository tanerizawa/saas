use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

use crate::application::services::AuthService;
use crate::domain::repositories::UserRepository;
use crate::shared::errors::AppError;
use crate::infrastructure::web::AppState;

/// Authentication middleware that validates JWT tokens
/// 
/// This middleware extracts the Bearer token from the Authorization header,
/// validates it, and adds the user ID to the request extensions.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Extract the token from the Authorization header
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|auth_header| {
            let auth_header = auth_header.to_str().ok()?;
            let parts: Vec<&str> = auth_header.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                Some(parts[1])
            } else {
                None
            }
        })
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Unauthorized",
                    "message": "Missing or invalid Authorization header",
                    "status": 401,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            )
        })?;

    // Validate the token and extract the user ID
    let user_id = state
        .auth_service
        .validate_token(token)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Unauthorized",
                    "message": "Invalid or expired token",
                    "status": 401,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            )
        })?;

    // Add the user ID to the request extensions
    req.extensions_mut().insert(user_id);

    // Continue to the next handler
    Ok(next.run(req).await)
}

/// Role-based access control middleware
/// 
/// This middleware checks if the authenticated user has one of the required roles.
/// It must be used after the auth_middleware.
pub async fn rbac_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
    allowed_roles: Vec<&'static str>,
) -> Result<Response, impl IntoResponse> {
    // Get the user ID from the request extensions (set by auth_middleware)
    let user_id = req
        .extensions()
        .get::<String>()
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Unauthorized",
                    "message": "Authentication required",
                    "status": 401,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            )
        })?;

    // Parse the user ID into a UserId
    let user_id = user_id.parse().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": "Invalid user ID format",
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;

    // Get the user from the repository
    let user = state
        .user_repository
        .find_by_id(&user_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Server Error",
                    "message": "Failed to fetch user details",
                    "status": 500,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Unauthorized",
                    "message": "User not found",
                    "status": 401,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                })),
            )
        })?;

    // Check if the user has one of the allowed roles
    let user_role = user.role.to_string();
    if !allowed_roles.contains(&user_role.as_str()) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "Forbidden",
                "message": "Insufficient permissions",
                "status": 403,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        ));
    }

    // Continue to the next handler
    Ok(next.run(req).await)
}

/// Rate limiting middleware
/// 
/// This middleware implements rate limiting based on client IP or user ID.
pub async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // In a real implementation, this would use Redis or a similar store to track request rates
    // For now, we'll just pass through all requests
    Ok(next.run(req).await)
}

/// Logging middleware
/// 
/// This middleware logs all incoming requests and their responses.
pub async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let path = req.uri().path().to_owned();
    let method = req.method().clone();
    let start_time = std::time::Instant::now();

    // Continue to the next handler
    let response = next.run(req).await;

    // Log the request after it's been processed
    let duration = start_time.elapsed();
    let status = response.status();
    
    tracing::info!(
        "Request: {} {} - Status: {} - Duration: {:?}",
        method,
        path,
        status,
        duration
    );

    response
}
