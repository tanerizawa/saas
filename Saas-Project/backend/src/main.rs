use axum::{response::Json, routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::{info, instrument};

// Import modules
mod config;
mod domain;
mod infrastructure;
mod services;
mod shared;

use config::AppConfig;
use domain::repositories::{CompanyRepository, UserRepository};
use infrastructure::{
    database::DatabaseManager,
    repositories::{
        LicenseRepository, PostgresCompanyRepository, PostgresLicenseRepositoryImpl,
        PostgresUserRepository,
    },
    web::handlers,
};
use services::auth::AuthService;
use shared::errors::AppError;

// Simple AppContext for fresh setup
#[derive(Clone)]
pub struct AppContext {
    pub config: AppConfig,
    pub db: DatabaseManager,
    pub auth_service: AuthService,
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub company_repository: Arc<dyn CompanyRepository + Send + Sync>,
    pub license_repository: Arc<dyn LicenseRepository + Send + Sync>,
}

// Implement the AppStateType trait for AppContext
impl infrastructure::web::handlers::AppStateType for AppContext {
    fn company_repository(
        &self,
    ) -> &Arc<dyn domain::repositories::CompanyRepository + Send + Sync> {
        &self.company_repository
    }

    fn user_repository(&self) -> &Arc<dyn domain::repositories::UserRepository + Send + Sync> {
        &self.user_repository
    }

    fn license_repository(
        &self,
    ) -> &Arc<dyn infrastructure::repositories::LicenseRepository + Send + Sync> {
        &self.license_repository
    }

    fn auth_service(&self) -> &services::auth::AuthService {
        &self.auth_service
    }

    fn config(&self) -> &config::AppConfig {
        &self.config
    }

    fn cache_service(&self) -> &Option<infrastructure::cache::CacheService> {
        &None // No cache for fresh setup
    }
}

// Use the AppState type alias from the handlers module
pub use infrastructure::web::handlers::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("🚀 Starting SaaS UMKM Platform Backend (Fresh Setup)");

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("⚙️  Configuration loaded");

    // Initialize database
    let db = DatabaseManager::new(&config.database_url, 10).await?;
    info!("🗄️  Database connected");

    // Initialize auth service
    let auth_service = AuthService::new(config.jwt_secret.clone());
    info!("🔐 Auth service initialized");

    // Initialize repositories
    let user_repository = Arc::new(PostgresUserRepository::new(db.pool().clone()));
    let company_repository = Arc::new(PostgresCompanyRepository::new(db.pool().clone()));
    let license_repository = Arc::new(PostgresLicenseRepositoryImpl::new(db.pool().clone()));

    info!("📊 Repositories initialized");

    // Create application context
    let app_state = Arc::new(AppContext {
        config: config.clone(),
        db,
        auth_service,
        user_repository,
        company_repository,
        license_repository,
    });

    // Build application router
    let app = create_app(app_state.clone()).await;

    // Start server
    let addr = format!("{}:{}", config.app_host, config.app_port);
    info!("🌐 Server starting on {}", addr);

    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument(skip(state))]
async fn create_app(state: AppState) -> Router {
    // Build the router with middleware
    let mut router = Router::new().layer(CorsLayer::permissive());

    // Add routes
    router = router
        .route("/health", get(health_check))
        .nest_service("/uploads", ServeDir::new(state.config().upload_dir.clone()))
        .nest("/api/v1", create_api_routes());

    router.with_state(state)
}

fn create_api_routes() -> Router<AppState> {
    Router::new()
        // Authentication routes
        .route(
            "/auth/register",
            axum::routing::post(handlers::auth::register),
        )
        .route("/auth/login", axum::routing::post(handlers::auth::login))
        .route(
            "/auth/refresh",
            axum::routing::post(handlers::auth::refresh_token),
        )
        .route(
            "/auth/logout",
            axum::routing::post(handlers::auth::logout),
        )
        .route(
            "/auth/reset-password",
            axum::routing::post(handlers::auth::request_password_reset),
        )
        .route(
            "/auth/health", 
            axum::routing::get(handlers::auth::health_check)
        )
        // User routes
        .route("/users/profile", axum::routing::get(handlers::users::get_current_user_profile))
        // TODO: Fix companies middleware authentication issues
        // .route("/users/companies", axum::routing::get(handlers::companies::get_my_companies))
        
        // TODO: Company management routes - fix authentication middleware
        // .nest("/companies", handlers::companies::routes())
        
        // TODO: Financial management routes - disabled until fixed  
        // .nest("/finance", handlers::finance_v2::router())
        
        // Admin routes
        .nest("/admin", Router::new()
            .route("/placeholder", axum::routing::get(handlers::admin::placeholder))
        )
}

async fn health_check() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "saas-umkm-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
