#![allow(dead_code)]

use axum::{
    extract::{State, Path, Query},
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
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

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let user_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid user ID format".to_string()))?;
    
    let query = GetUserQuery::new(user_id);
    let query_handler = UserQueryHandler::new(state.user_repository.clone());
    
    match query_handler.handle_get_user(query).await {
        Ok(Some(user)) => Ok(Json(json!({
            "id": user.id.to_string(),
            "email": user.email.value(),
            "name": user.full_name,
            "role": format!("{:?}", user.role),
            "verified": user.email_verified_at.is_some(),
            "created_at": user.created_at,
            "updated_at": user.updated_at
        }))),
        Ok(None) => Err(AppError::NotFound("User not found".to_string())),
        Err(e) => Err(AppError::InternalError(e.to_string()))
    }
}

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
                    "id": user.id.to_string(),
                    "email": user.email.value(),
                    "name": user.full_name,
                    "role": format!("{:?}", user.role),
                    "verified": user.email_verified_at.is_some(),
                    "created_at": user.created_at,
                    "updated_at": user.updated_at
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

pub async fn get_current_user_profile(
    State(_state): State<AppState>,
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
                    "id": user.id.to_string(),
                    "email": user.email.value(),
                    "name": user.full_name,
                    "role": format!("{:?}", user.role),
                    "verified": user.email_verified_at.is_some()
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
