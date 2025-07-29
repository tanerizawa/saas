use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::infrastructure::web::AppState;
use crate::services::onboarding::{OnboardingRequest, OnboardingService, OnboardingStatus, OnboardingStep};
use crate::domain::value_objects::UserId;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OnboardingRequestDto {
    pub user_email: String,
    pub user_full_name: String,
    pub user_phone: Option<String>,
    pub company_name: String,
    pub company_type: String,
    pub business_description: String,
    pub company_address: String,
    pub company_phone: String,
    pub company_email: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct OnboardingStatusResponse {
    pub user_id: String,
    pub company_id: String,
    pub current_step: String,
    pub completed_steps: Vec<String>,
    pub next_actions: Vec<String>,
    pub completion_percentage: u8,
    pub estimated_completion_date: Option<String>,
}

impl From<OnboardingStatus> for OnboardingStatusResponse {
    fn from(status: OnboardingStatus) -> Self {
        Self {
            user_id: status.user_id.to_string(),
            company_id: status.company_id.to_string(),
            current_step: format!("{:?}", status.current_step),
            completed_steps: status.completed_steps.iter().map(|s| format!("{:?}", s)).collect(),
            next_actions: status.next_actions,
            completion_percentage: status.completion_percentage,
            estimated_completion_date: status.estimated_completion_date.map(|d| d.to_rfc3339()),
        }
    }
}

/// Start UMKM onboarding process
/// 
/// Creates a new UMKM onboarding workflow with automated steps
#[utoipa::path(
    post,
    path = "/api/v1/onboarding/start",
    request_body = OnboardingRequestDto,
    responses(
        (status = 201, description = "Onboarding started successfully", body = OnboardingStatusResponse),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User or company already exists")
    ),
    tag = "Onboarding"
)]
pub async fn start_onboarding(
    State(state): State<AppState>,
    Json(payload): Json<OnboardingRequestDto>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let onboarding_service = OnboardingService::new(
        state.email_service(),
        state.onboarding_repository(),
    );

    let request = OnboardingRequest {
        user_email: payload.user_email,
        user_full_name: payload.user_full_name,
        user_phone: payload.user_phone,
        company_name: payload.company_name,
        company_type: payload.company_type,
        business_description: payload.business_description,
        company_address: payload.company_address,
        company_phone: payload.company_phone,
        company_email: payload.company_email,
        tax_id: payload.tax_id,
    };

    match onboarding_service.start_onboarding(request).await {
        Ok(status) => {
            let response = OnboardingStatusResponse::from(status);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to start onboarding: {}", e);
            let error_response = serde_json::json!({
                "error": "Onboarding Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

/// Get onboarding status
/// 
/// Retrieves the current onboarding status for a user
#[utoipa::path(
    get,
    path = "/api/v1/onboarding/status/{user_id}",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Onboarding status retrieved", body = OnboardingStatusResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Onboarding"
)]
pub async fn get_onboarding_status(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let onboarding_service = OnboardingService::new(
        state.email_service(),
        state.onboarding_repository(),
    );

    let user_id = match UserId::parse(&user_id) {
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

    match onboarding_service.get_onboarding_status(&user_id).await {
        Ok(status) => {
            let response = OnboardingStatusResponse::from(status);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get onboarding status: {}", e);
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

/// Complete onboarding step
/// 
/// Marks a specific onboarding step as completed
#[utoipa::path(
    post,
    path = "/api/v1/onboarding/complete-step/{user_id}/{step}",
    params(
        ("user_id" = String, Path, description = "User ID"),
        ("step" = String, Path, description = "Onboarding step to complete")
    ),
    responses(
        (status = 200, description = "Step completed successfully", body = OnboardingStatusResponse),
        (status = 400, description = "Invalid step or user ID"),
        (status = 404, description = "User not found")
    ),
    tag = "Onboarding"
)]
pub async fn complete_onboarding_step(
    State(state): State<AppState>,
    Path((user_id, step)): Path<(String, String)>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let onboarding_service = OnboardingService::new(
        state.email_service(),
        state.onboarding_repository(),
    );

    let user_id = match UserId::parse(&user_id) {
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

    let onboarding_step = match step.as_str() {
        "email_verification" => OnboardingStep::EmailVerification,
        "company_information" => OnboardingStep::CompanyInformation,
        "document_upload" => OnboardingStep::DocumentUpload,
        "initial_payment" => OnboardingStep::InitialPayment,
        "account_activation" => OnboardingStep::AccountActivation,
        _ => {
            let error_response = serde_json::json!({
                "error": "Invalid Step",
                "message": format!("Unknown onboarding step: {}", step),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    match onboarding_service.complete_step(&user_id, onboarding_step).await {
        Ok(status) => {
            let response = OnboardingStatusResponse::from(status);
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to complete onboarding step: {}", e);
            let error_response = serde_json::json!({
                "error": "Step Completion Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

/// Get onboarding checklist
/// 
/// Returns the complete onboarding checklist with step descriptions
#[utoipa::path(
    get,
    path = "/api/v1/onboarding/checklist",
    responses(
        (status = 200, description = "Onboarding checklist retrieved")
    ),
    tag = "Onboarding"
)]
pub async fn get_onboarding_checklist(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let onboarding_service = OnboardingService::new(
        state.email_service(),
        state.onboarding_repository(),
    );
    let checklist = onboarding_service.get_onboarding_checklist();

    let response_checklist: Vec<serde_json::Value> = checklist
        .into_iter()
        .map(|(step, description, completed)| {
            serde_json::json!({
                "step": format!("{:?}", step),
                "description": description,
                "completed": completed,
                "order": step.get_order()
            })
        })
        .collect();

    let response = serde_json::json!({
        "checklist": response_checklist,
        "total_steps": response_checklist.len(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    (StatusCode::OK, Json(response))
}
