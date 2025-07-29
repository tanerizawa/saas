use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::infrastructure::web::AppState;
use crate::services::license_processing::{
    LicenseProcessingService, LicenseProcessingStatus,
    LicenseProcessingError,
};
use crate::services::license_processing_models::{
    CreateLicenseApplicationRequest, ReviewLicenseRequest,
};
use crate::services::{LicenseApplicationResponse, ProcessingStatisticsResponse};
use crate::infrastructure::repositories::license_processing_repository::{
    ReviewDecision, ApplicationStatus,
};
use crate::domain::value_objects::{LicenseId, UserId, CompanyId};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LicenseApplicationDto {
    pub user_id: String,
    pub company_id: String,
    pub license_type: String,
    pub business_description: String,
    pub required_documents: Vec<String>,
    pub additional_info: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LicenseReviewDto {
    pub license_id: String,
    pub reviewer_id: String,
    pub action: String, // "approve", "reject", "request_revision", "escalate"
    pub comments: String,
    pub required_fixes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LicenseProcessingStatusResponse {
    pub license_id: String,
    pub current_status: String,
    pub processing_steps: Vec<String>,
    pub assigned_reviewer: Option<String>,
    pub estimated_completion: Option<String>,
    pub priority_level: String,
    pub workflow_stage: String,
}

impl From<LicenseProcessingStatus> for LicenseProcessingStatusResponse {
    fn from(status: LicenseProcessingStatus) -> Self {
        Self {
            license_id: status.license_id.to_string(),
            current_status: format!("{:?}", status.current_status),
            processing_steps: status.processing_steps.iter().map(|s| format!("{:?}", s)).collect(),
            assigned_reviewer: status.assigned_reviewer.map(|r| r.to_string()),
            estimated_completion: status.estimated_completion.map(|d| d.to_rfc3339()),
            priority_level: format!("{:?}", status.priority_level),
            workflow_stage: format!("{:?}", status.workflow_stage),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignedLicensesQuery {
    pub stage: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Submit a new license application
/// 
/// Creates a new license application and starts the processing workflow
#[utoipa::path(
    post,
    path = "/api/v1/licenses/applications",
    request_body = LicenseApplicationDto,
    responses(
        (status = 201, description = "License application submitted successfully", body = LicenseProcessingStatusResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "License Processing"
)]
pub async fn submit_license_application(
    State(state): State<AppState>,
    Json(payload): Json<LicenseApplicationDto>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let license_service = LicenseProcessingService::new(
        state.email_service(),
        state.license_processing_repository(),
    );

    let user_id = match UserId::parse(&payload.user_id) {
        Ok(id) => id,
        Err(_) => {
            let error_response = serde_json::json!({
                "error": "Invalid User ID",
                "message": "Invalid UUID format",
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    let company_id = match CompanyId::from_uuid(uuid::Uuid::parse_str(&payload.company_id).unwrap_or_default()) {
        id => id,
    };

    let request = CreateLicenseApplicationRequest {
        user_id: user_id.to_string(),
        company_id: payload.company_id,
        license_type: payload.license_type,
        business_description: payload.business_description,
        required_documents: payload.required_documents,
        additional_info: payload.additional_info,
        priority: None,
    };

    match license_service.create_license_application(request).await {
        Ok(status) => {
            let response = LicenseProcessingStatusResponse::from(status);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to submit license application: {}", e);
            let error_response = serde_json::json!({
                "error": "Application Submission Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

/// Process a license review
/// 
/// Allows staff and admins to review and take action on license applications
#[utoipa::path(
    post,
    path = "/api/v1/licenses/review",
    request_body = LicenseReviewDto,
    responses(
        (status = 200, description = "License review processed successfully", body = LicenseProcessingStatusResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "License Processing"
)]
pub async fn process_license_review(
    State(state): State<AppState>,
    Json(payload): Json<LicenseReviewDto>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let license_service = LicenseProcessingService::new(
        state.email_service(),
        state.license_processing_repository(),
    );

    let license_id = match LicenseId::new() {
        // TODO: Parse from payload.license_id
        id => id,
    };

    let reviewer_id = match UserId::parse(&payload.reviewer_id) {
        Ok(id) => id,
        Err(_) => {
            let error_response = serde_json::json!({
                "error": "Invalid Reviewer ID",
                "message": "Invalid UUID format",
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    let action = match payload.action.as_str() {
        "approve" => ReviewAction::Approve,
        "reject" => ReviewAction::Reject,
        "request_revision" => ReviewAction::RequestRevision,
        "escalate" => ReviewAction::EscalateToAdmin,
        _ => {
            let error_response = serde_json::json!({
                "error": "Invalid Action",
                "message": format!("Unknown review action: {}", payload.action),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    let review_request = LicenseReviewRequest {
        license_id,
        reviewer_id,
        action,
        comments: payload.comments,
        required_fixes: payload.required_fixes,
    };

    // TODO: Get reviewer user from JWT token or database
    let reviewer = crate::domain::entities::User::new(
        crate::domain::value_objects::Email::new("reviewer@example.com").unwrap(),
        "dummy_hash".to_string(),
        "Reviewer".to_string(),
        crate::domain::entities::UserRole::AdminStaff,
    );

    match license_service.process_review(review_request).await {
        Ok(status) => {
            let response = LicenseProcessingStatusResponse::from(status);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to process license review: {}", e);
            let error_response = serde_json::json!({
                "error": "Review Processing Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

/// Get license processing status
/// 
/// Retrieves the current processing status of a license application
#[utoipa::path(
    get,
    path = "/api/v1/licenses/status/{license_id}",
    params(
        ("license_id" = String, Path, description = "License ID")
    ),
    responses(
        (status = 200, description = "License status retrieved", body = LicenseProcessingStatusResponse),
        (status = 404, description = "License not found")
    ),
    tag = "License Processing"
)]
pub async fn get_license_status(
    State(state): State<AppState>,
    Path(license_id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let license_service = LicenseProcessingService::new(
        state.email_service(),
        state.license_processing_repository(),
    );

    // TODO: Parse license_id properly
    let license_id = LicenseId::new();

    match license_service.get_processing_status(&license_id).await {
        Ok(status) => {
            let response = LicenseProcessingStatusResponse::from(status);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get license status: {}", e);
            let error_response = serde_json::json!({
                "error": "Status Retrieval Failed",
                "message": e.to_string(),
                "status": 404,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/// Get assigned licenses for a reviewer
/// 
/// Returns licenses assigned to a specific reviewer/staff member
#[utoipa::path(
    get,
    path = "/api/v1/licenses/assigned/{reviewer_id}",
    params(
        ("reviewer_id" = String, Path, description = "Reviewer User ID")
    ),
    responses(
        (status = 200, description = "Assigned licenses retrieved"),
        (status = 404, description = "Reviewer not found")
    ),
    tag = "License Processing"
)]
pub async fn get_assigned_licenses(
    State(state): State<AppState>,
    Path(reviewer_id): Path<String>,
    Query(query): Query<AssignedLicensesQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let license_service = LicenseProcessingService::new(
        state.email_service(),
        state.license_processing_repository(),
    );

    let reviewer_id = match UserId::parse(&reviewer_id) {
        Ok(id) => id,
        Err(_) => {
            let error_response = serde_json::json!({
                "error": "Invalid Reviewer ID",
                "message": "Invalid UUID format",
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    // TODO: Parse stage from query parameter
    let stage = None;

    match license_service.get_assigned_licenses(&reviewer_id, stage).await {
        Ok(licenses) => {
            let response_licenses: Vec<LicenseProcessingStatusResponse> = licenses
                .into_iter()
                .map(LicenseProcessingStatusResponse::from)
                .collect();

            let response = serde_json::json!({
                "licenses": response_licenses,
                "total": response_licenses.len(),
                "limit": query.limit.unwrap_or(50),
                "offset": query.offset.unwrap_or(0),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get assigned licenses: {}", e);
            let error_response = serde_json::json!({
                "error": "Retrieval Failed",
                "message": e.to_string(),
                "status": 404,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/// Get license processing statistics
/// 
/// Returns processing statistics for reporting and dashboard purposes
#[utoipa::path(
    get,
    path = "/api/v1/licenses/statistics",
    responses(
        (status = 200, description = "Processing statistics retrieved"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "License Processing"
)]
pub async fn get_processing_statistics(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let license_service = LicenseProcessingService::new(
        state.email_service(),
        state.license_processing_repository(),
    );

    // For statistics, we need to pass a user_id - let's use a placeholder for now
    let user_id = UserId::new(); // This should come from authentication context
    match license_service.get_processing_statistics(user_id).await {
        Ok(stats) => {
            let response = serde_json::json!({
                "statistics": stats,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get processing statistics: {}", e);
            let error_response = serde_json::json!({
                "error": "Statistics Retrieval Failed",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}
