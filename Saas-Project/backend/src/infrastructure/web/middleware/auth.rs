use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::str::FromStr;

use crate::domain::value_objects::UserId;
use crate::domain::entities::UserRole;
use crate::services::auth::{AuthService, Claims};

// Authentication struct for handlers
#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub role: UserRole,
    pub claims: Claims,
}

// Authentication error responses
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authorization token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Authorization token expired"),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    TokenExpired,
}

// JWT Token extractor implementation
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AuthService: Clone,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // Parse Bearer token
        let _token = if auth_header.starts_with("Bearer ") {
            &auth_header[7..]
        } else {
            return Err(AuthError::InvalidToken);
        };

        // This would normally extract the auth service from state
        // For now, we'll return an error since the full middleware setup needs more work
        Err(AuthError::InvalidToken)
    }
}

// Simpler approach: Create a basic auth check function for handlers
pub async fn extract_user_from_token(token: &str, auth_service: &AuthService) -> Result<AuthenticatedUser, AuthError> {
    let claims = auth_service
        .validate_token(token)
        .map_err(|_| AuthError::InvalidToken)?;

    // Parse user_id from claims
    let user_id = UserId::parse(&claims.sub).map_err(|_| AuthError::InvalidToken)?;
    
    // Parse role from claims  
    let role = UserRole::from_str(&claims.role).map_err(|_| AuthError::InvalidToken)?;

    Ok(AuthenticatedUser {
        user_id,
        role,
        claims,
    })
}
