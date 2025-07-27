use axum::{
    extract::State,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::entities::{User, UserRole, UserStatus};
use crate::domain::value_objects::{Email, UserId};
use crate::infrastructure::web::AppState;
use crate::shared::errors::AppResult;

// Create the auth router
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

// Request and response models

#[derive(Debug, Deserialize, Validate)]
pub struct UserRegistrationRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    
    #[validate(length(min = 2, message = "Name must be at least 2 characters"))]
    pub full_name: String,
    
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    pub status: String,
    pub phone: Option<String>,
    pub email_verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            email: user.email.to_string(),
            full_name: user.full_name,
            role: user.role.to_string(),
            status: user.status.to_string(),
            phone: user.phone,
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}

// Handler implementations

/// Register a new user
/// 
/// Creates a new user account with the provided information
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = UserRegistrationRequest,
    responses(
        (status = 201, description = "User created successfully", body = UserResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Email already exists")
    )
)]
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<UserRegistrationRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Validate the request
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Validation Error",
                "message": validation_errors.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        ));
    }
    
    // Check if email already exists
    let email = Email::new(&payload.email).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid Input",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    let existing_user = state.user_repository.find_by_email(&email).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    if existing_user.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "Conflict",
                "message": "Email already in use",
                "status": 409,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        ));
    }
    
    // Hash the password
    let password_hash = state.auth_service.hash_password(&payload.password).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    // Create the user
    let user = User {
        id: UserId::new(),
        email,
        password_hash,
        full_name: payload.full_name,
        phone: payload.phone,
        role: UserRole::UmkmOwner, // Default role for new users
        status: UserStatus::PendingVerification, // Default status
        email_verified_at: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Save the user
    state.user_repository.save(&user).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    // Return the created user
    let user_response = UserResponse::from(user);
    
    Ok((StatusCode::CREATED, Json(user_response)))
}

/// User login
/// 
/// Authenticates a user and returns a JWT token
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Invalid credentials"),
        (status = 401, description = "Unauthorized")
    )
)]
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Validate the request
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Validation Error",
                "message": validation_errors.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        ));
    }
    
    // Find the user by email
    let email = Email::new(&payload.email).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "Invalid Input",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    let user = state.user_repository.find_by_email(&email).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    let user = user.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized",
                "message": "Invalid email or password",
                "status": 401,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    // Verify the password
    let is_valid = state.auth_service.verify_password(&user.password_hash, &payload.password).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized",
                "message": "Invalid email or password",
                "status": 401,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        ));
    }
    
    // Check if the user is active
    if user.status != UserStatus::Active {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized",
                "message": "Account is not active",
                "status": 401,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        ));
    }
    
    // Generate a JWT token
    let token = state.auth_service.generate_token(&user.id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Server Error",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            })),
        )
    })?;
    
    // Return the token and user info
    let response = LoginResponse {
        token,
        user: UserResponse::from(user),
    };
    
    Ok((StatusCode::OK, Json(response)))
}
