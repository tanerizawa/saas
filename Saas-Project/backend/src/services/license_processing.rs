use crate::domain::entities::{User, UserRole};
use crate::domain::value_objects::{LicenseId, UserId, CompanyId};
use crate::infrastructure::repositories::{LicenseProcessingRepository};
use crate::infrastructure::repositories::license_processing_repository::{
    LicenseApplication, LicenseReview, ApplicationStatus, ReviewDecision, 
    PriorityLevel, ProcessingStatistics,
    LicenseApplicationCreate, LicenseReviewCreate
};
use crate::services::{EmailService, LicenseApplicationResponse, ProcessingStatisticsResponse};
use crate::services::license_processing_models::{
    CreateLicenseApplicationRequest, ReviewLicenseRequest
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use std::fmt;
use uuid::Uuid;

#[derive(Clone)]
pub struct LicenseProcessingService {
    email_service: Arc<EmailService>,
    license_processing_repository: Arc<dyn LicenseProcessingRepository + Send + Sync>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseProcessingStatus {
    pub license_id: LicenseId,
    pub current_status: ApplicationStatus,
    pub assigned_reviewer: Option<UserId>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewAction {
    pub action_type: String,
    pub requires_documents: Option<Vec<String>>,
    pub next_step: Option<String>,
}

#[derive(Debug, Clone)]
pub enum LicenseProcessingError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
    UnauthorizedAccess(String),
    WorkflowError(String),
    ExternalServiceError(String),
}

impl fmt::Display for LicenseProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LicenseProcessingError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            LicenseProcessingError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            LicenseProcessingError::NotFound(msg) => write!(f, "Not found: {}", msg),
            LicenseProcessingError::UnauthorizedAccess(msg) => write!(f, "Unauthorized: {}", msg),
            LicenseProcessingError::WorkflowError(msg) => write!(f, "Workflow error: {}", msg),
            LicenseProcessingError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
        }
    }
}

impl std::error::Error for LicenseProcessingError {}

impl LicenseProcessingService {
    pub fn new(
        email_service: Arc<EmailService>,
        license_processing_repository: Arc<dyn LicenseProcessingRepository + Send + Sync>
    ) -> Self {
        Self {
            email_service,
            license_processing_repository,
        }
    }

    // Core License Application Operations
    pub async fn create_license_application(
        &self,
        request: CreateLicenseApplicationRequest,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        // Validate request data
        self.validate_application_request(&request)?;

        // Create application data
        let application_data = LicenseApplicationCreate {
            user_id: request.user_id,
            company_id: request.company_id,
            license_type: request.license_type.clone(),
            business_description: request.business_description.clone(),
            required_documents: request.required_documents.clone(),
            additional_info: request.additional_info.clone(),
            status: ApplicationStatus::Submitted,
            priority: self.calculate_priority(&request),
            submitted_at: Utc::now(),
        };

        // Create application in repository
        let application = self.license_processing_repository
            .create_application(application_data)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Send acknowledgment email
        self.send_application_acknowledgment(&application).await?;

        // Create response
        Ok(LicenseApplicationResponse {
            id: application.id.to_string(),
            company_id: application.company_id.to_string(),
            license_type: application.license_type.clone(),
            status: format!("{:?}", application.status),
            priority: format!("{:?}", application.priority),
            current_stage: application.current_stage,
            total_stages: application.total_stages,
            assigned_reviewer_id: application.assigned_reviewer_id.map(|id| id.to_string()),
            estimated_completion: self.calculate_estimated_completion(&application)
                .map(|dt| dt.to_rfc3339()),
            created_at: application.created_at.to_rfc3339(),
            updated_at: application.updated_at.to_rfc3339(),
            application_data: application.application_data,
        })
    }

    pub async fn assign_reviewer(
        &self,
        license_id: LicenseId,
        reviewer_id: UserId,
        assigning_user_id: UserId,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        // Validate permissions
        self.validate_reviewer_permissions(assigning_user_id, reviewer_id).await?;

        // Update application
        let updated_application = self.license_processing_repository
            .assign_reviewer(license_id, reviewer_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Notify reviewer
        self.notify_reviewer_assignment(reviewer_id, &updated_application).await?;

        // Create response
        Ok(LicenseApplicationResponse {
            id: updated_application.id.to_string(),
            company_id: updated_application.company_id.to_string(),
            license_type: updated_application.license_type.clone(),
            status: format!("{:?}", updated_application.status),
            priority: format!("{:?}", updated_application.priority),
            current_stage: updated_application.current_stage,
            total_stages: updated_application.total_stages,
            assigned_reviewer_id: updated_application.assigned_reviewer_id.map(|id| id.to_string()),
            estimated_completion: self.calculate_estimated_completion(&updated_application)
                .map(|dt| dt.to_rfc3339()),
            created_at: updated_application.created_at.to_rfc3339(),
            updated_at: updated_application.updated_at.to_rfc3339(),
            application_data: updated_application.application_data,
        })
    }

    pub async fn process_review(
        &self,
        request: ReviewLicenseRequest,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        // Get current application
        let application = self.license_processing_repository
            .get_application(request.license_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?
            .ok_or_else(|| LicenseProcessingError::NotFound("License application not found".to_string()))?;

        // Validate reviewer permissions
        self.validate_review_permissions(&request, &application)?;

        // Create review record
        let review_data = LicenseReviewCreate {
            license_id: request.license_id,
            reviewer_id: request.reviewer_id,
            decision: request.decision,
            comments: request.comments.clone(),
            review_date: Utc::now(),
            recommendations: request.recommendations.clone(),
        };

        let review = self.license_processing_repository
            .create_review(review_data)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Update application status based on review
        let new_status = self.determine_new_status(&review, &application);
        let updated_application = self.license_processing_repository
            .update_status(request.license_id, new_status)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Handle workflow actions based on decision
        match review.decision {
            ReviewDecision::Approve => {
                self.approve_license(&updated_application).await?;
            },
            ReviewDecision::Reject => {
                self.reject_license(&updated_application, &review).await?;
            },
            ReviewDecision::RequestRevision => {
                self.request_revision(&updated_application, &review).await?;
            },
            ReviewDecision::Escalate => {
                self.escalate_to_admin(&updated_application, &review).await?;
            },
        }

        // Create response
        Ok(LicenseApplicationResponse {
            id: updated_application.id.to_string(),
            company_id: updated_application.company_id.to_string(),
            license_type: updated_application.license_type.clone(),
            status: format!("{:?}", updated_application.status),
            priority: format!("{:?}", updated_application.priority),
            current_stage: updated_application.current_stage,
            total_stages: updated_application.total_stages,
            assigned_reviewer_id: updated_application.assigned_reviewer_id.map(|id| id.to_string()),
            estimated_completion: self.calculate_estimated_completion(&updated_application)
                .map(|dt| dt.to_rfc3339()),
            created_at: updated_application.created_at.to_rfc3339(),
            updated_at: updated_application.updated_at.to_rfc3339(),
            application_data: updated_application.application_data,
        })
    }

    pub async fn get_processing_statistics(
        &self,
        user_id: UserId,
    ) -> Result<ProcessingStatisticsResponse, LicenseProcessingError> {
        // Get statistics from repository
        let stats = self.license_processing_repository
            .get_processing_statistics()
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Transform to response format
        Ok(ProcessingStatisticsResponse {
            total_applications: stats.total_applications,
            pending_applications: stats.pending_applications,
            approved_applications: stats.approved_applications,
            rejected_applications: stats.rejected_applications,
            average_processing_time_hours: stats.average_processing_time_hours,
            applications_by_priority: stats.applications_by_priority,
        })
    }

    // Helper Methods
    fn validate_application_request(
        &self,
        request: &CreateLicenseApplicationRequest,
    ) -> Result<(), LicenseProcessingError> {
        if request.license_type.is_empty() {
            return Err(LicenseProcessingError::ValidationError(
                "License type is required".to_string()
            ));
        }

        if request.business_description.len() < 50 {
            return Err(LicenseProcessingError::ValidationError(
                "Business description must be at least 50 characters".to_string()
            ));
        }

        if request.required_documents.is_empty() {
            return Err(LicenseProcessingError::ValidationError(
                "At least one required document must be specified".to_string()
            ));
        }

        Ok(())
    }

    fn calculate_priority(&self, request: &CreateLicenseApplicationRequest) -> PriorityLevel {
        // Business logic for priority calculation
        if request.license_type.contains("emergency") {
            PriorityLevel::High
        } else if request.license_type.contains("standard") {
            PriorityLevel::Medium
        } else {
            PriorityLevel::Low
        }
    }

    fn calculate_estimated_completion(&self, application: &LicenseApplication) -> Option<DateTime<Utc>> {
        let base_days = match application.priority {
            PriorityLevel::High => 7,
            PriorityLevel::Medium => 14,
            PriorityLevel::Low => 30,
        };

        Some(application.created_at + chrono::Duration::days(base_days))
    }

    fn get_next_steps(&self, application: &LicenseApplication) -> Vec<String> {
        match application.status {
            ApplicationStatus::Submitted => vec![
                "Application received and is being reviewed".to_string(),
                "You will be notified when a reviewer is assigned".to_string(),
            ],
            ApplicationStatus::UnderReview => vec![
                "Application is currently under review".to_string(),
                "Please wait for reviewer feedback".to_string(),
            ],
            ApplicationStatus::RequiredDocuments => vec![
                "Additional documents are required".to_string(),
                "Check your email for specific requirements".to_string(),
            ],
            ApplicationStatus::Processing => vec![
                "Application is being processed".to_string(),
                "Final decision will be communicated soon".to_string(),
            ],
            ApplicationStatus::PendingDocuments => vec![
                "Waiting for document submission".to_string(),
                "Please submit required documents to continue".to_string(),
            ],
            ApplicationStatus::Approved => vec![
                "Congratulations! Your license has been approved".to_string(),
                "License certificate will be sent via email".to_string(),
            ],
            ApplicationStatus::Rejected => vec![
                "Application has been rejected".to_string(),
                "Review rejection reasons and consider reapplying".to_string(),
            ],
            ApplicationStatus::OnHold => vec![
                "Application is temporarily on hold".to_string(),
                "Please contact support for more information".to_string(),
            ],
            ApplicationStatus::Cancelled => vec![
                "Application has been cancelled".to_string(),
                "You may submit a new application if needed".to_string(),
            ],
        }
    }

    async fn send_application_acknowledgment(
        &self,
        application: &LicenseApplication,
    ) -> Result<(), LicenseProcessingError> {
        // Email sending logic would go here
        // For now, we'll just return Ok
        Ok(())
    }

    async fn validate_reviewer_permissions(
        &self,
        assigning_user_id: UserId,
        reviewer_id: UserId,
    ) -> Result<(), LicenseProcessingError> {
        // Permission validation logic would go here
        // For now, we'll just return Ok
        Ok(())
    }

    async fn notify_reviewer_assignment(
        &self,
        reviewer_id: UserId,
        application: &LicenseApplication,
    ) -> Result<(), LicenseProcessingError> {
        // Notification logic would go here
        Ok(())
    }

    fn validate_review_permissions(
        &self,
        request: &ReviewLicenseRequest,
        application: &LicenseApplication,
    ) -> Result<(), LicenseProcessingError> {
        // Validation logic would go here
        Ok(())
    }

    fn determine_new_status(
        &self,
        review: &LicenseReview,
        application: &LicenseApplication,
    ) -> ApplicationStatus {
        match review.decision {
            ReviewDecision::Approve => ApplicationStatus::Approved,
            ReviewDecision::Reject => ApplicationStatus::Rejected,
            ReviewDecision::RequestRevision => ApplicationStatus::RequiredDocuments,
            ReviewDecision::Escalate => ApplicationStatus::UnderReview,
        }
    }

    async fn approve_license(
        &self,
        application: &LicenseApplication,
    ) -> Result<(), LicenseProcessingError> {
        // License approval workflow
        Ok(())
    }

    async fn reject_license(
        &self,
        application: &LicenseApplication,
        review: &LicenseReview,
    ) -> Result<(), LicenseProcessingError> {
        // License rejection workflow
        Ok(())
    }

    async fn request_revision(
        &self,
        application: &LicenseApplication,
        review: &LicenseReview,
    ) -> Result<(), LicenseProcessingError> {
        // Revision request workflow
        Ok(())
    }

    async fn put_on_hold(
        &self,
        application: &LicenseApplication,
        review: &LicenseReview,
    ) -> Result<(), LicenseProcessingError> {
        // On hold workflow
        Ok(())
    }

    async fn escalate_to_admin(
        &self,
        application: &LicenseApplication,
        review: &LicenseReview,
    ) -> Result<(), LicenseProcessingError> {
        // Admin escalation workflow
        Ok(())
    }
}
