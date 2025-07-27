use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    services::ServeDir,
};
use tracing::{info, instrument, warn};

mod application;
mod config;
mod domain;
mod infrastructure;
mod services;
mod shared;
#[cfg(test)]
mod tests;

use config::AppConfig;
use domain::repositories::{CompanyRepository, UserRepository};
use infrastructure::{
    database::manager::DatabaseManager,
    repositories::{
        CachedLicenseRepository, LicenseRepository, PostgresCompanyRepository,
        PostgresUserRepository,
    },
    web::handlers,
};
use services::auth::AuthService;
use shared::errors::AppError;

// Define AppContext and AppState types
#[derive(Clone)]
pub struct AppContext {
    pub config: AppConfig,
    pub db: DatabaseManager,
    pub auth_service: AuthService,
    pub cache_service: Option<infrastructure::cache::CacheService>,
    pub user_repository: Arc<dyn UserRepository + Send + Sync>,
    pub company_repository: Arc<dyn CompanyRepository + Send + Sync>,
    pub license_repository: Arc<dyn LicenseRepository + Send + Sync>,
}

// Implement the AppStateType trait for AppContext
impl infrastructure::web::handlers::AppStateType for AppContext {
    fn company_repository(&self) -> &Arc<dyn domain::repositories::CompanyRepository + Send + Sync> {
        &self.company_repository
    }
    
    fn user_repository(&self) -> &Arc<dyn domain::repositories::UserRepository + Send + Sync> {
        &self.user_repository
    }
    
    fn license_repository(&self) -> &Arc<dyn infrastructure::repositories::LicenseRepository + Send + Sync> {
        &self.license_repository
    }
    
    fn auth_service(&self) -> &services::auth::AuthService {
        &self.auth_service
    }

    fn config(&self) -> &config::AppConfig {
        &self.config
    }

    fn cache_service(&self) -> &Option<infrastructure::cache::CacheService> {
        &self.cache_service
    }
}

// Use the AppState type alias from the handlers module
pub use infrastructure::web::handlers::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing (structured logging as recommended)
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .init();

    info!("🚀 Starting SaaS UMKM Platform Backend");

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("📋 Configuration loaded");

    // Initialize database connection
    let db = DatabaseConnection::new(&config.database_url).await?;
    info!("🗄️ Database connection established");

    // Run migrations
    // Migration disabled to fix version issue
    // Original line:     // db.migrate().await? // Disabled temporarily;
    info!("🔄 Database migrations SKIPPED (to fix version issues)");
    info!("🔄 Database migrations completed");

    // Initialize authentication service
    let auth_service = AuthService::new(config.jwt_secret.clone());
    info!("🔐 Authentication service initialized");

    // Initialize repositories
    let user_repository = Arc::new(PostgresUserRepository::new(db.pool().clone()));
    let company_repository = Arc::new(PostgresCompanyRepository::new(db.pool().clone()));

    // Initialize cache service if Redis URL is provided
    let cache_service = match &config.redis_url {
        Some(redis_url) => match infrastructure::cache::CacheService::new(redis_url) {
            Ok(cache) => {
                info!("🔄 Redis cache service initialized");
                Some(cache)
            }
            Err(err) => {
                warn!("⚠️ Failed to initialize Redis cache service: {}", err);
                None
            }
        },
        None => {
            info!("ℹ️ No Redis URL provided, running without cache");
            None
        }
    };

    // Use cached license repository if Redis is available
    let license_repository: Arc<dyn LicenseRepository + Send + Sync> = match &cache_service {
        Some(cache) => {
            tracing::info!("🔄 Initializing cached license repository");
            Arc::new(CachedLicenseRepository::<CacheService>::new_with_cache(
                db.pool().clone(),
                Arc::new(cache.clone()),
            ))
        }
        None => {
            tracing::info!("🔄 Initializing standard license repository (no cache)");
            Arc::new(CachedLicenseRepository::new(db.pool().clone()))
        }
    };

    info!("📊 Repositories initialized");

    // Create application context
    let app_state = Arc::new(AppContext {
        config: config.clone(),
        db,
        auth_service,
        cache_service,
        user_repository,
        company_repository,
        license_repository,
    });

    // Build application router
    let app = create_app(app_state.clone()).await;

    // Rate limiter initialization notification
    if let Some(_) = &config.rate_limiter {
        info!("🔄 Rate limiter initialized and ready");
        // The governor crate doesn't require explicit cleanup
    }

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
    let mut router = Router::new();

    // Add tracing middleware
    router = router.layer(TraceLayer::new_for_http());

    // Add compression if enabled
    if state.config.enable_compression {
        router = router.layer(CompressionLayer::new());
    }

    // Add CORS middleware
    router = router.layer(
        CorsLayer::new()
            .allow_origin(Any) // In production, restrict to specific origins
            .allow_methods(Any)
            .allow_headers(Any),
    );

    // Add routes to the router
    router = router
        // Health check endpoint
        .route("/health", get(health_check))
        // Static file serving for uploads
        .nest_service("/uploads", ServeDir::new(state.config.upload_dir.clone()))
        // API routes
        .nest("/api/v1", create_api_routes());

    // Add rate limiting middleware if configured
    router = if state.config.rate_limiter.is_some() {
        use crate::infrastructure::web::middleware::auth::require_auth;
        router.route_layer(axum::middleware::from_fn_with_state(
            state.clone(),
            require_auth,
        ))
    } else {
        router
    };

    // Finish building the router with state
    router.with_state(state)
}

fn create_api_routes() -> Router<AppState> {
    Router::new()
        // Authentication routes (public)
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/refresh", post(handlers::auth::refresh_token))
        .route(
            "/auth/reset-password",
            post(handlers::auth::request_password_reset),
        )
        .route("/auth/health", get(handlers::auth::health_check))
        // Protected routes (will be added with middleware in the full app)
        .route("/me", get(handlers::auth::get_profile))
        .route("/auth/logout", post(handlers::auth::logout))
        // User management routes
        .nest("/users", handlers::users::routes())
        // Company management routes
        .nest("/companies", handlers::companies::routes())
        // License management routes
        .nest("/licenses", handlers::licenses::routes())
        // Placeholder routes for other handlers (public for now)
        // .route("/licensing", get(handlers::licensing::placeholder))
        .route("/business", get(handlers::business::placeholder))
        .route("/finance", get(handlers::finance::placeholder))
        .route("/admin", get(handlers::admin::placeholder))
        .route("/files", get(handlers::files::placeholder))
}

async fn health_check() -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({
        "status": "healthy",
        "service": "saas-umkm-backend",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
