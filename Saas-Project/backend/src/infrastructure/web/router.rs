use axum::{
    routing::get,
    Router, 
    extract::State,
    http::StatusCode, 
    response::IntoResponse, 
    Json
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use tower_http::services::ServeDir;

use crate::application::services::AuthService;
use crate::domain::repositories::{UserRepository, CompanyRepository, LicenseRepository};
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub company_repository: Arc<dyn CompanyRepository + Send + Sync>,
    pub license_repository: Arc<dyn LicenseRepository + Send + Sync>,
    pub auth_service: Arc<dyn AuthService + Send + Sync>,
    pub config: AppConfig,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
        // Add other path handlers here as they're implemented
    ),
    components(
        schemas(HealthResponse)
        // Add other schemas here as they're implemented
    ),
    tags(
        (name = "Health", description = "API health endpoints")
        // Add other tags here
    )
)]
struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    // Build the main router
    let api_router = Router::new()
        .route("/health", get(health))
        .with_state(state);

    // Create the swagger documentation
    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api-docs/openapi.json", ApiDoc::openapi());
    
    // Combine the API router with Swagger UI and static file serving
    Router::new()
        .nest("/api", api_router)
        .merge(swagger)
        .nest_service(
            "/docs", 
            ServeDir::new("api-docs")
        )
}

// API Handlers

/// Health check endpoint
/// 
/// Returns the current status of the API
#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "API is healthy", body = HealthResponse)
    )
)]
async fn health() -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    (StatusCode::OK, Json(response))
}

// Response models

/// Health check response
#[derive(serde::Serialize, utoipa::ToSchema)]
pub struct HealthResponse {
    /// API status
    status: String,
    
    /// API version
    version: String,
    
    /// Current timestamp
    timestamp: String,
}

// Error handler
pub async fn handle_error(err: axum::extract::rejection::JsonRejection) -> impl IntoResponse {
    let status = StatusCode::BAD_REQUEST;
    
    let body = serde_json::json!({
        "error": "Bad Request",
        "message": err.to_string(),
        "status": status.as_u16(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    (status, Json(body))
}
