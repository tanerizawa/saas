use axum::{
    routing::{get, post, put},
    Router,
};

use crate::infrastructure::web::AppState;
use crate::infrastructure::web::handlers::{
    onboarding_handler, license_processing_handler, system_config_handler
};

pub fn create_onboarding_routes() -> Router<AppState> {
    Router::new()
        .route("/start", post(onboarding_handler::start_onboarding))
        .route("/status/:user_id", get(onboarding_handler::get_onboarding_status))
        .route("/complete-step/:user_id/:step", post(onboarding_handler::complete_onboarding_step))
        .route("/checklist", get(onboarding_handler::get_onboarding_checklist))
}

pub fn create_license_processing_routes() -> Router<AppState> {
    Router::new()
        .route("/applications", post(license_processing_handler::submit_license_application))
        .route("/review", post(license_processing_handler::process_license_review))
        .route("/status/:license_id", get(license_processing_handler::get_license_status))
        .route("/assigned/:reviewer_id", get(license_processing_handler::get_assigned_licenses))
        .route("/statistics", get(license_processing_handler::get_processing_statistics))
}

pub fn create_system_config_routes() -> Router<AppState> {
    Router::new()
        .route("/groups", get(system_config_handler::get_all_config_groups))
        .route("/groups/:group_name", get(system_config_handler::get_config_group))
        .route("/groups/:group_name", put(system_config_handler::update_config_group))
        .route("/get/:key", get(system_config_handler::get_config_value))
        .route("/set", put(system_config_handler::update_config_value))
        .route("/reset/:key", post(system_config_handler::reset_config_to_default))
        .route("/export", get(system_config_handler::export_config))
        .route("/import", post(system_config_handler::import_config))
}

pub fn create_business_logic_routes() -> Router<AppState> {
    Router::new()
        .nest("/onboarding", create_onboarding_routes())
        .nest("/licenses", create_license_processing_routes())
        .nest("/system/config", create_system_config_routes())
}
