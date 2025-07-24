#![allow(dead_code)]

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::licenses::{
        ApplicationStatus, ApplicationStatusHistory, License, LicenseDocument, LicenseType,
        PriorityLevel,
    },
    domain::entities::UserRole,
    infrastructure::repositories::license_repository::LicenseStatistics,
    // infrastructure::repositories::LicenseRepository,
    infrastructure::web::middleware::auth::AuthenticatedUser,
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct CreateLicenseRequest {
    pub license_type: LicenseType,
    pub company_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<PriorityLevel>,
    pub estimated_processing_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLicenseRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<PriorityLevel>,
    pub estimated_processing_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ApproveLicenseRequest {
    pub license_number: String,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub issuing_authority: String,
    pub admin_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RejectLicenseRequest {
    pub rejection_reason: String,
    pub admin_notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LicenseQueryParams {
    pub status: Option<ApplicationStatus>,
    pub license_type: Option<LicenseType>,
    pub search: Option<String>,
    pub company_id: Option<Uuid>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct LicenseResponse {
    pub license: License,
    pub documents: Vec<LicenseDocument>,
    pub status_history: Vec<ApplicationStatusHistory>,
}

#[derive(Debug, Serialize)]
pub struct LicenseListResponse {
    pub licenses: Vec<License>,
    pub total_count: i64,
    pub has_more: bool,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_license))
        .route("/", get(get_user_licenses))
        .route("/search", get(search_licenses))
        .route("/statistics", get(get_license_statistics))
        .route("/:id", get(get_license_by_id))
        .route("/:id", put(update_license))
        .route("/:id", delete(delete_license))
        .route("/:id/submit", post(submit_license))
        .route("/:id/approve", post(approve_license))
        .route("/:id/reject", post(reject_license))
        .route("/:id/documents", get(get_license_documents))
        .route("/:id/documents", post(upload_license_document))
        .route("/:id/status-history", get(get_license_status_history))
}

// Create a new license application
async fn create_license(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Json(request): Json<CreateLicenseRequest>,
) -> Result<Json<License>, StatusCode> {
    // Create new license in draft status
    let license = License::new(
        request.license_type,
        request.company_id,
        *user.user_id.as_uuid(), // Convert UserId to Uuid
        request.title,
        request.description,
    );

    match app_state.license_repository.create_license(&license).await {
        Ok(created_license) => Ok(Json(created_license)),
        Err(e) => {
            tracing::error!("Failed to create license: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get user's licenses
async fn get_user_licenses(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<LicenseQueryParams>,
) -> Result<Json<Vec<License>>, StatusCode> {
    // Filter by specific criteria or get all user licenses
    let licenses = if let Some(status) = params.status {
        // Get by status (need to filter by user)
        match app_state
            .license_repository
            .get_licenses_by_status(status)
            .await
        {
            Ok(all_licenses) => all_licenses
                .into_iter()
                .filter(|license| license.user_id == *user.user_id.as_uuid())
                .collect(),
            Err(e) => {
                tracing::error!("Failed to get licenses by status: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else if let Some(license_type) = params.license_type {
        // Get by type (need to filter by user)
        match app_state
            .license_repository
            .get_licenses_by_type(license_type)
            .await
        {
            Ok(all_licenses) => all_licenses
                .into_iter()
                .filter(|license| license.user_id == *user.user_id.as_uuid())
                .collect(),
            Err(e) => {
                tracing::error!("Failed to get licenses by type: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else if let Some(company_id) = params.company_id {
        // Get by company
        match app_state
            .license_repository
            .get_licenses_by_company(company_id)
            .await
        {
            Ok(licenses) => licenses
                .into_iter()
                .filter(|license| license.user_id == *user.user_id.as_uuid()) // Ensure user owns the licenses
                .collect(),
            Err(e) => {
                tracing::error!("Failed to get licenses by company: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    } else {
        // Get all user licenses
        match app_state
            .license_repository
            .get_licenses_by_user(*user.user_id.as_uuid())
            .await
        {
            Ok(licenses) => licenses,
            Err(e) => {
                tracing::error!("Failed to get user licenses: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    Ok(Json(licenses))
}

// Get license by ID with full details
async fn get_license_by_id(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
) -> Result<Json<LicenseResponse>, StatusCode> {
    // Get license
    let license = match app_state
        .license_repository
        .get_license_by_id(license_id)
        .await
    {
        Ok(Some(license)) => {
            // Check if user owns this license
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get documents
    let documents = match app_state
        .license_repository
        .get_documents_by_license(license_id)
        .await
    {
        Ok(docs) => docs,
        Err(e) => {
            tracing::error!("Failed to get license documents: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Get status history
    let status_history = match app_state
        .license_repository
        .get_status_history_by_license(license_id)
        .await
    {
        Ok(history) => history,
        Err(e) => {
            tracing::error!("Failed to get status history: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(LicenseResponse {
        license,
        documents,
        status_history,
    }))
}

// Update license (only in draft status)
async fn update_license(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
    Json(request): Json<UpdateLicenseRequest>,
) -> Result<Json<License>, StatusCode> {
    // Get existing license
    let mut license = match app_state
        .license_repository
        .get_license_by_id(license_id)
        .await
    {
        Ok(Some(license)) => {
            // Check ownership
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            // Only allow updates in draft status
            if license.application_status != ApplicationStatus::Draft {
                return Err(StatusCode::BAD_REQUEST);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Update fields
    if let Some(title) = request.title {
        license.title = title;
    }
    if let Some(description) = request.description {
        license.description = Some(description);
    }
    if let Some(priority) = request.priority {
        license.priority = priority;
    }
    if let Some(processing_days) = request.estimated_processing_days {
        license.estimated_processing_days = Some(processing_days);
    }

    license.updated_at = Utc::now();

    // Save updated license
    match app_state.license_repository.update_license(&license).await {
        Ok(updated_license) => Ok(Json(updated_license)),
        Err(e) => {
            tracing::error!("Failed to update license: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Delete license (only in draft status)
async fn delete_license(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // Get existing license to check ownership and status
    let _license = match app_state
        .license_repository
        .get_license_by_id(license_id)
        .await
    {
        Ok(Some(license)) => {
            // Check ownership
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            // Only allow deletion in draft status
            if license.application_status != ApplicationStatus::Draft {
                return Err(StatusCode::BAD_REQUEST);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Delete license
    match app_state
        .license_repository
        .delete_license(license_id)
        .await
    {
        Ok(true) => Ok(StatusCode::NO_CONTENT),
        Ok(false) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to delete license: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Submit license application
async fn submit_license(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
) -> Result<Json<License>, StatusCode> {
    // Check license ownership
    let _license = match app_state
        .license_repository
        .get_license_by_id(license_id)
        .await
    {
        Ok(Some(license)) => {
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Submit application
    match app_state
        .license_repository
        .submit_license_application(license_id, *user.user_id.as_uuid())
        .await
    {
        Ok(updated_license) => Ok(Json(updated_license)),
        Err(e) => {
            tracing::error!("Failed to submit license application: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Approve license (admin only)
async fn approve_license(
    State(app_state): State<AppState>,
    admin_user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
    Json(request): Json<ApproveLicenseRequest>,
) -> Result<Json<License>, StatusCode> {
    if admin_user.role != UserRole::SuperAdmin && admin_user.role != UserRole::AdminStaff {
        return Err(StatusCode::FORBIDDEN);
    }

    match app_state
        .license_repository
        .approve_license(
            license_id,
            *admin_user.user_id.as_uuid(),
            request.license_number,
            request.issue_date,
            request.expiry_date,
            request.issuing_authority,
            request.admin_notes,
        )
        .await
    {
        Ok(approved_license) => Ok(Json(approved_license)),
        Err(e) => {
            tracing::error!("Failed to approve license: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Reject license (admin only)
async fn reject_license(
    State(app_state): State<AppState>,
    admin_user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
    Json(request): Json<RejectLicenseRequest>,
) -> Result<Json<License>, StatusCode> {
    if admin_user.role != UserRole::SuperAdmin && admin_user.role != UserRole::AdminStaff {
        return Err(StatusCode::FORBIDDEN);
    }

    match app_state
        .license_repository
        .reject_license(
            license_id,
            *admin_user.user_id.as_uuid(),
            request.rejection_reason,
            request.admin_notes,
        )
        .await
    {
        Ok(rejected_license) => Ok(Json(rejected_license)),
        Err(e) => {
            tracing::error!("Failed to reject license: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Search licenses
async fn search_licenses(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Query(params): Query<LicenseQueryParams>,
) -> Result<Json<Vec<License>>, StatusCode> {
    let search_query = params.search.unwrap_or_default();

    match app_state
        .license_repository
        .search_licenses(&search_query, Some(*user.user_id.as_uuid()))
        .await
    {
        Ok(licenses) => Ok(Json(licenses)),
        Err(e) => {
            tracing::error!("Failed to search licenses: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get license documents
async fn get_license_documents(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
) -> Result<Json<Vec<LicenseDocument>>, StatusCode> {
    // Check license ownership
    let _license = match app_state
        .license_repository
        .get_license_by_id(license_id)
        .await
    {
        Ok(Some(license)) => {
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match app_state
        .license_repository
        .get_documents_by_license(license_id)
        .await
    {
        Ok(documents) => Ok(Json(documents)),
        Err(e) => {
            tracing::error!("Failed to get license documents: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Upload license document (placeholder - will need multipart form handling)
async fn upload_license_document(
    State(_app_state): State<AppState>,
    _user: AuthenticatedUser,
    Path(_license_id): Path<Uuid>,
) -> Result<Json<LicenseDocument>, StatusCode> {
    // TODO: Implement file upload handling with multipart forms
    // This will require additional dependencies like tower-http for multipart
    // For now, return not implemented
    Err(StatusCode::NOT_IMPLEMENTED)
}

// Get license status history
async fn get_license_status_history(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
    Path(license_id): Path<Uuid>,
) -> Result<Json<Vec<ApplicationStatusHistory>>, StatusCode> {
    // Check license ownership
    let _license = match app_state
        .license_repository
        .get_license_by_id(license_id)
        .await
    {
        Ok(Some(license)) => {
            if license.user_id != *user.user_id.as_uuid() {
                return Err(StatusCode::FORBIDDEN);
            }
            license
        }
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to get license: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match app_state
        .license_repository
        .get_status_history_by_license(license_id)
        .await
    {
        Ok(history) => Ok(Json(history)),
        Err(e) => {
            tracing::error!("Failed to get status history: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Get license statistics for user
async fn get_license_statistics(
    State(app_state): State<AppState>,
    user: AuthenticatedUser,
) -> Result<Json<LicenseStatistics>, StatusCode> {
    match app_state
        .license_repository
        .get_license_statistics(Some(*user.user_id.as_uuid()))
        .await
    {
        Ok(statistics) => Ok(Json(statistics)),
        Err(e) => {
            tracing::error!("Failed to get license statistics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
