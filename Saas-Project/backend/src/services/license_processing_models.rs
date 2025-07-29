use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Request Models for License Processing API

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLicenseApplicationRequest {
    pub user_id: String,
    pub company_id: String,
    pub license_type: String,
    pub business_description: String,
    pub required_documents: Vec<String>,
    pub additional_info: HashMap<String, String>,
    pub priority: Option<String>, // "urgent", "high", "normal", "low"
}

#[derive(Debug, Serialize, Deserialize)]  
pub struct ReviewLicenseRequest {
    pub reviewer_id: String,
    pub decision: String, // "approve", "reject", "request_revision", "escalate"
    pub comments: Option<String>,
    pub review_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePriorityRequest {
    pub priority: String, // "urgent", "high", "normal", "low"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String, // "submitted", "under_review", "required_documents", "approved", "rejected", "cancelled"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignReviewerRequest {
    pub reviewer_id: String,
}

// Response Models for License Processing API

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseApplicationResponse {
    pub id: String,
    pub company_id: String,
    pub license_type: String,
    pub status: String,
    pub priority: String,
    pub current_stage: i32,
    pub total_stages: i32,
    pub assigned_reviewer_id: Option<String>,
    pub estimated_completion: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub application_data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingStatisticsResponse {
    pub total_applications: i64,
    pub pending_applications: i64,
    pub approved_applications: i64,
    pub rejected_applications: i64,
    pub average_processing_time_hours: f64,
    pub applications_by_priority: HashMap<String, i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowStatusResponse {
    pub application_id: String,
    pub current_stage: i32,
    pub total_stages: i32,
    pub stage_name: String,
    pub next_actions: Vec<String>,
    pub estimated_completion: Option<String>,
    pub can_advance: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewerWorkloadResponse {
    pub reviewer_id: String,
    pub active_applications: i64,
    pub completed_reviews_count: i64,
    pub average_review_time_hours: f64,
    pub capacity_status: String, // "available", "busy", "overloaded"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationListResponse {
    pub applications: Vec<LicenseApplicationResponse>,
    pub total_count: i64,
    pub page: u32,
    pub limit: u32,
}

// Utility functions for conversions

impl CreateLicenseApplicationRequest {
    pub fn to_internal(&self) -> Result<crate::infrastructure::repositories::license_processing_repository::LicenseApplicationCreate, String> {
        use crate::domain::value_objects::{CompanyId, UserId};
        use crate::infrastructure::repositories::license_processing_repository::{PriorityLevel, ApplicationStatus};

        let user_id = UserId::parse(&self.user_id)
            .map_err(|_| "Invalid user ID format".to_string())?;

        let company_id = CompanyId::from_uuid(
            Uuid::parse_str(&self.company_id)
                .map_err(|_| "Invalid company ID format".to_string())?
        );

        let priority = match self.priority.as_deref().unwrap_or("normal") {
            "urgent" => PriorityLevel::High,
            "high" => PriorityLevel::High,
            "normal" => PriorityLevel::Medium,
            "low" => PriorityLevel::Low,
            _ => return Err("Invalid priority level".to_string()),
        };

        Ok(crate::infrastructure::repositories::license_processing_repository::LicenseApplicationCreate {
            user_id,
            company_id,
            license_type: self.license_type.clone(),
            business_description: self.business_description.clone(),
            required_documents: self.required_documents.clone(),
            additional_info: self.additional_info.clone(),
            status: ApplicationStatus::Submitted,
            priority,
            submitted_at: chrono::Utc::now(),
        })
    }
}

impl ReviewLicenseRequest {
    pub fn to_internal(&self, license_id: crate::domain::value_objects::LicenseId) -> Result<crate::infrastructure::repositories::license_processing_repository::LicenseReviewCreate, String> {
        use crate::domain::value_objects::UserId;
        use crate::infrastructure::repositories::license_processing_repository::ReviewDecision;

        let reviewer_id = UserId::parse(&self.reviewer_id)
            .map_err(|_| "Invalid reviewer ID format".to_string())?;

        let decision = match self.decision.as_str() {
            "approve" => ReviewDecision::Approve,
            "reject" => ReviewDecision::Reject,
            "request_revision" => ReviewDecision::RequestRevision,
            "escalate" => ReviewDecision::Escalate,
            _ => return Err("Invalid decision".to_string()),
        };

        Ok(crate::infrastructure::repositories::license_processing_repository::LicenseReviewCreate {
            license_id,
            reviewer_id,
            decision,
            comments: self.comments.clone().unwrap_or_default(),
            review_date: chrono::Utc::now(),
            recommendations: None,
        })
    }
}

impl From<&crate::infrastructure::repositories::license_processing_repository::LicenseApplication> for LicenseApplicationResponse {
    fn from(app: &crate::infrastructure::repositories::license_processing_repository::LicenseApplication) -> Self {
        use crate::infrastructure::repositories::license_processing_repository::{ApplicationStatus, PriorityLevel};
        
        let status = match &app.status {
            ApplicationStatus::Submitted => "submitted",
            ApplicationStatus::UnderReview => "under_review", 
            ApplicationStatus::RequiredDocuments => "required_documents",
            ApplicationStatus::Processing => "processing",
            ApplicationStatus::PendingDocuments => "pending_documents",
            ApplicationStatus::Approved => "approved",
            ApplicationStatus::Rejected => "rejected",
            ApplicationStatus::OnHold => "on_hold",
            ApplicationStatus::Cancelled => "cancelled",
        };

        let priority = match &app.priority {
            PriorityLevel::High => "urgent",
            PriorityLevel::High => "high",
            PriorityLevel::Medium => "normal",
            PriorityLevel::Low => "low",
        };

        Self {
            id: app.id.to_string(),
            company_id: app.company_id.to_string(),
            license_type: app.license_type.clone(),
            status: status.to_string(),
            priority: priority.to_string(),
            current_stage: app.current_stage,
            total_stages: app.total_stages,
            assigned_reviewer_id: app.assigned_reviewer_id.as_ref().map(|id| id.to_string()),
            estimated_completion: app.estimated_completion.map(|dt| dt.to_rfc3339()),
            created_at: app.created_at.to_rfc3339(),
            updated_at: app.updated_at.to_rfc3339(),
            application_data: app.application_data.clone(),
        }
    }
}

impl From<&crate::infrastructure::repositories::license_processing_repository::ProcessingStatistics> for ProcessingStatisticsResponse {
    fn from(stats: &crate::infrastructure::repositories::license_processing_repository::ProcessingStatistics) -> Self {
        Self {
            total_applications: stats.total_applications,
            pending_applications: stats.pending_applications,
            approved_applications: stats.approved_applications,
            rejected_applications: stats.rejected_applications,
            average_processing_time_hours: stats.average_processing_time_hours,
            applications_by_priority: stats.applications_by_priority.clone(),
        }
    }
}
