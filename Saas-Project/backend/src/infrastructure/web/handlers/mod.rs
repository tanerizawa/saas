// HTTP request handlers organized by domain
use std::sync::Arc;

// Define a simple type alias for AppState for use in handlers
pub type AppState = Arc<dyn AppStateType>;

// AppStateType needs to expose all the fields that handlers will access
pub trait AppStateType: Send + Sync {
    fn company_repository(&self) -> &Arc<dyn crate::domain::repositories::CompanyRepository + Send + Sync>;
    fn user_repository(&self) -> &Arc<dyn crate::domain::repositories::UserRepository + Send + Sync>;
    fn license_repository(&self) -> &Arc<dyn crate::infrastructure::repositories::LicenseRepository + Send + Sync>;
    fn auth_service(&self) -> &crate::services::auth::AuthService;
    fn config(&self) -> &AppConfig;
    fn cache_service(&self) -> &Option<crate::infrastructure::cache::CacheService>;
}

// Import the AppConfig type
use crate::config::AppConfig;

pub mod admin;
pub mod auth;
pub mod users;
// TODO: Fix trait bounds compilation issues - companies handlers need auth middleware fixes
// pub mod companies;
