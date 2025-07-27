use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::domain::entities::UserRole;
use crate::domain::value_objects::UserId;
use crate::shared::errors::AppError;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
    pub company_id: uuid::Uuid,
    pub role: UserRole,
}

use crate::infrastructure::web::handlers::AppState;

/// JWT Authentication middleware
pub async fn require_auth(
    State(ctx): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        });

    let token = auth_header.ok_or_else(|| {
        AppError::Unauthorized("Missing or invalid authorization header".to_string())
    })?;

    // Validate token
    let user_id = ctx
        .auth_service()
        .extract_user_id(token)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

    let user_role = ctx
        .auth_service()
        .extract_user_role(token)
        .map_err(|_| AppError::Unauthorized("Invalid token claims".to_string()))?;

    // Get company_id from the token claims or user service
    let company_id = ctx
        .auth_service()
        .extract_company_id(token)
        .unwrap_or_else(|_| uuid::Uuid::new_v4()); // Default fallback

    // Add authenticated user to request extensions
    request.extensions_mut().insert(AuthenticatedUser {
        user_id,
        company_id,
        role: user_role,
    });

    Ok(next.run(request).await)
}

/// Extract authenticated user from request
#[allow(dead_code)]
pub fn extract_user(request: &Request) -> Result<&AuthenticatedUser, AppError> {
    request
        .extensions()
        .get::<AuthenticatedUser>()
        .ok_or_else(|| AppError::Unauthorized("User not authenticated".to_string()))
}

// Implement FromRequestParts so AuthenticatedUser can be used as an extractor
#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    "User not authenticated".to_string(),
                )
            })
    }
}
