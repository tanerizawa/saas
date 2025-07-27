use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::domain::entities::{User, UserRole};
use crate::domain::value_objects::Email;
use crate::infrastructure::web::middleware::auth::AuthenticatedUser;

// Use the AppState from the handlers module
use super::AppState;

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub role: String,
    pub is_verified: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub role: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub status: String,
}

/// User registration endpoint
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Parse email
    let email = Email::new(&payload.email).map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid email format",
                "details": err
            })),
        )
    })?;

    // Parse role or default to UmkmOwner
    let role = match payload.role.as_deref() {
        Some("admin_staff") => UserRole::AdminStaff,
        Some("super_admin") => UserRole::SuperAdmin,
        _ => UserRole::UmkmOwner,
    };

    // Hash password
    let password_hash = state
        .auth_service()
        .hash_password(&payload.password)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to process password",
                    "details": err.to_string()
                })),
            )
        })?;

    // Create user
    let user = User::new(email, password_hash, payload.full_name, role);

    // Save user to database
    state.user_repository().save(&user).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create user",
                "details": err.to_string()
            })),
        )
    })?;

    Ok(Json(json!({
        "message": "User registered successfully",
        "user_id": user.id.to_string(),
        "email_verification_required": true
    })))
}

/// User login endpoint
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Parse email
    let email = Email::new(&payload.email).map_err(|err| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid email format",
                "details": err
            })),
        )
    })?;

    // Find user by email
    let mut user = state
        .user_repository()
        .find_by_email(&email)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database error",
                    "details": err.to_string()
                })),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Invalid credentials"
                })),
            )
        })?;

    // Verify password
    let is_valid = state
        .auth_service()
        .verify_password(&payload.password, &user.password_hash)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal server error"
                })),
            )
        })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid credentials"
            })),
        ));
    }

    // Check if user can login (active and verified)
    if !user.can_login() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Account not active or email not verified"
            })),
        ));
    }

    // Update last login
    user.update_last_login();

    // Save updated user back to database
    state.user_repository().save(&user).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update login time",
                "details": err.to_string()
            })),
        )
    })?;

    // Generate tokens
    let tokens = state.auth_service().generate_tokens(&user).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to generate tokens",
                "details": err.to_string()
            })),
        )
    })?;

    Ok(Json(json!({
        "access_token": tokens.access_token,
        "refresh_token": tokens.refresh_token,
        "expires_at": tokens.expires_at,
        "user": {
            "id": user.id.as_uuid().to_string(),
            "email": user.email.as_str(),
            "full_name": user.full_name,
            "role": user.role.to_string()
        }
    })))
}

/// Get current user profile
pub async fn get_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<UserProfileResponse>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.user_id;

    let user_data = state
        .user_repository()
        .find_by_id(&user_id)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database error",
                    "details": err.to_string()
                })),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User not found"})),
            )
        })?;

    let response = UserProfileResponse {
        id: user_data.id.to_string(),
        email: user_data.email.as_str().to_string(),
        role: user_data.role.to_string(),
        is_verified: user_data.email_verified_at.is_some(),
        created_at: user_data.created_at,
    };

    Ok(Json(response))
}
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let refresh_token = payload
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "refresh_token is required"})),
            )
        })?;

    // Validate refresh token
    let claims = state
        .auth_service()
        .validate_token(refresh_token)
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid refresh token"})),
            )
        })?;

    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map(crate::domain::value_objects::UserId)
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid refresh token"})),
            )
        })?;

    let user = state
        .user_repository()
        .find_by_id(&user_id)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error", "details": err.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid refresh token"})),
            )
        })?;

    let tokens = state
        .auth_service()
        .generate_tokens(&user)
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to generate tokens", "details": err.to_string()})),
            )
        })?;

    if let Some(cache) = state.cache_service() {
        if let Ok(new_claims) = state.auth_service().validate_token(&tokens.refresh_token) {
            let ttl = new_claims.exp - chrono::Utc::now().timestamp();
            let key = format!("refresh:{}:{}", user_id, new_claims.jti);
            let _ = cache.set(&key, &tokens.refresh_token, Some(ttl as u64)).await;
        }
    }

    Ok(Json(json!({
        "access_token": tokens.access_token,
        "refresh_token": tokens.refresh_token,
        "expires_at": tokens.expires_at,
    })))
}

/// Logout endpoint
pub async fn logout(
    State(state): State<AppState>,
    auth_user: crate::infrastructure::web::middleware::auth::AuthenticatedUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if let Some(cache) = state.cache_service() {
        let pattern = format!("refresh:{}:*", auth_user.user_id);
        let _ = cache.delete_by_pattern(&pattern).await;
    }

    Ok(Json(json!({
        "message": "Logged out successfully"
    })))
}

/// Request password reset
pub async fn request_password_reset(
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Missing email"
                })),
            )
        })?;

    Ok(Json(json!({
        "message": "If an account with this email exists, a password reset link has been sent"
    })))
}

/// Health check for auth service
pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "authentication",
        "timestamp": chrono::Utc::now()
    }))
}
