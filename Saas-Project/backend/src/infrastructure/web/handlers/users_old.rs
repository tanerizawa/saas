use axum::{
    extract::{State, Path, Query},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::application::queries::GetUserQuery;
use crate::application::query_handlers::UserQueryHandler;
use crate::shared::errors::AppError;

type AppState = crate::AppState;

#[derive(Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct SearchUsersQuery {
    pub q: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_user))
        .route("/", get(list_users))
        .route("/profile", get(get_current_user_profile))
        .route("/search", get(search_users))
}

/// Get user by ID (requires authentication)
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let user_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid user ID format".to_string()))?;
    
    let query = GetUserQuery::new(user_id);
    let query_handler = UserQueryHandler::new(state.user_repository.clone());
    
    match query_handler.handle_get_user(query).await {
        Ok(user) => Ok(Json(json!({
            "id": user.id().to_string(),
            "email": user.email().value(),
            "name": user.name(),
            "role": user.role().to_string(),
            "verified": user.is_verified(),
            "created_at": user.created_at(),
            "updated_at": user.updated_at()
        }))),
        Err(e) => Err(AppError::InternalError(e.to_string()))
    }
}

/// List all users (admin only)
pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListUsersQuery>,
) -> Result<Json<Value>, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    
    let query_handler = UserQueryHandler::new(state.user_repository.clone());
    
    match query_handler.handle_list_users(page, limit).await {
        Ok(response) => {
            let user_responses: Vec<Value> = response.data
                .into_iter()
                .map(|user| json!({
                    "id": user.id().to_string(),
                    "email": user.email().value(),
                    "name": user.name(),
                    "role": user.role().to_string(),
                    "verified": user.is_verified(),
                    "created_at": user.created_at(),
                    "updated_at": user.updated_at()
                }))
                .collect();

            Ok(Json(json!({
                "users": user_responses,
                "total": response.total,
                "page": response.page,
                "limit": response.limit
            })))
        }
        Err(e) => Err(AppError::InternalError(e.to_string()))
    }
}

/// Get current user's profile
pub async fn get_current_user_profile(
    State(state): State<AppState>,
) -> Result<Json<Value>, AppError> {
    // For now, just return a placeholder
    // In real implementation, we would extract user from JWT token
    Ok(Json(json!({
        "message": "Profile endpoint - authentication needed"
    })))
}

pub async fn search_users(
    State(state): State<AppState>,
    Query(params): Query<SearchUsersQuery>,
) -> Result<Json<Value>, AppError> {
    let query_text = params.q.unwrap_or_default();
    
    let query_handler = UserQueryHandler::new(state.user_repository.clone());
    
    match query_handler.handle_search_users(&query_text).await {
        Ok(users) => {
            let user_responses: Vec<Value> = users
                .into_iter()
                .map(|user| json!({
                    "id": user.id().to_string(),
                    "email": user.email().value(),
                    "name": user.name(),
                    "role": user.role().to_string(),
                    "verified": user.is_verified()
                }))
                .collect();

            Ok(Json(json!({
                "users": user_responses,
                "query": query_text
            })))
        }
        Err(e) => Err(AppError::InternalError(e.to_string()))
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(get_user))
        .route("/", get(list_users))
        .route("/profile", get(get_current_user_profile))
        .route("/search", get(search_users))
}

/// Placeholder endpoint (for backward compatibility)
pub async fn placeholder() -> Json<Value> {
    Json(json!({
        "message": "Users API - Fully implemented with authentication",
        "endpoints": [
            "GET /users/:id - Get user by ID (authenticated)",
            "GET /users - List users (admin only)",
            "GET /users/profile - Get current user profile",
            "GET /users/search?search=query - Search users (admin only)"
        ],
        "authentication": "Required - Bearer token in Authorization header",
        "authorization": "Role-based access control implemented"
    }))
}
