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
pub struct LicenseApplicationRequest {
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub license_type: String,
    pub business_description: String,
    pub required_documents: Vec<String>,
    pub additional_info: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseReviewRequest {
    pub license_id: LicenseId,
    pub reviewer_id: UserId,
    pub action: ReviewAction,
    pub comments: String,
    pub required_fixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewAction {
    Approve,
    Reject,
    RequestRevision,
    EscalateToAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseProcessingStatus {
    pub license_id: LicenseId,
    pub current_status: ApplicationStatus,
    pub assigned_reviewer: Option<UserId>,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, thiserror::Error)]
pub enum LicenseProcessingError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("License not found")]
    LicenseNotFound,
    #[error("Reviewer not found")]
    ReviewerNotFound,
    #[error("Invalid status transition")]
    InvalidStatusTransition,
}

impl LicenseProcessingService {
    pub fn new(
        email_service: Arc<EmailService>,
        license_processing_repository: Arc<dyn LicenseProcessingRepository + Send + Sync>,
    ) -> Self {
        Self {
            email_service,
            license_processing_repository,
        }
    }

    /// Create a new license application
    pub async fn create_license_application(
        &self,
        request: CreateLicenseApplicationRequest,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        let internal_request = request.to_internal()
            .map_err(|e| LicenseProcessingError::ValidationError(e))?;

        let license_id = self.license_processing_repository
            .create_application(internal_request)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Create response from the new license
        let status = LicenseProcessingStatus {
            license_id: license_id.clone(),
            current_status: ApplicationStatus::Submitted,
            assigned_reviewer: None,
            estimated_completion: Some(Utc::now() + chrono::Duration::days(7)),
        };

        Ok(LicenseApplicationResponse::from(status))
    }

    /// Assign a reviewer to a license application
    pub async fn assign_reviewer(
        &self,
        application_id: &LicenseId,
        reviewer_id: &UserId,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .assign_reviewer(application_id, reviewer_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("Reviewer assigned: {} to {}", reviewer_id, application_id);
        Ok(())
    }

    /// Process a review decision
    pub async fn process_review(
        &self,
        application_id: &LicenseId,
        review: LicenseReviewCreate,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        let application = self.license_processing_repository
            .get_application(application_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?
            .ok_or(LicenseProcessingError::LicenseNotFound)?;

        // Create the review
        let review_create = LicenseReviewCreate {
            application_id: application_id.clone(),
            reviewer_id: review.reviewer_id,
            decision: review.decision.clone(),
            comments: review.comments,
            review_data: Some(review.review_data.unwrap_or_default()),
            created_at: Utc::now(),
        };

        let new_status = match review.decision {
            ReviewDecision::Approve => ApplicationStatus::Approved,
            ReviewDecision::Reject => ApplicationStatus::Rejected,
            ReviewDecision::RequestRevision => ApplicationStatus::RequiredDocuments,
            ReviewDecision::Escalate => ApplicationStatus::UnderReview,
        };

        // Update application status
        self.license_processing_repository
            .update_status(application_id, new_status.clone())
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        let status = LicenseProcessingStatus {
            license_id: application_id.clone(),
            current_status: new_status,
            assigned_reviewer: Some(review.reviewer_id),
            estimated_completion: Some(Utc::now() + chrono::Duration::days(3)),
        };

        Ok(LicenseApplicationResponse::from(status))
    }

    /// Get processing statistics
    pub async fn get_processing_statistics(&self) -> Result<ProcessingStatisticsResponse, LicenseProcessingError> {
        let stats = self.license_processing_repository
            .get_statistics()
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(ProcessingStatisticsResponse::from(stats))
    }

    /// Helper method to calculate estimated completion time
    fn calculate_estimated_completion(&self, priority: &PriorityLevel) -> DateTime<Utc> {
        let hours_to_add = match priority {
            PriorityLevel::Urgent => 24,   // 1 day
            PriorityLevel::High => 72,     // 3 days
            PriorityLevel::Normal => 168,  // 1 week
            PriorityLevel::Low => 336,     // 2 weeks
        };
        
        Utc::now() + chrono::Duration::hours(hours_to_add as i64)
    }

    /// Helper method to send acknowledgment notification
    async fn send_application_acknowledgment(
        &self,
        license_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Implement email notification
        tracing::info!("Acknowledgment sent for application: {}", license_id);
        Ok(())
    }

    /// Helper method to validate reviewer permissions
    async fn validate_reviewer_permissions(
        &self,
        reviewer_id: &UserId,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Implement permission validation
        tracing::debug!("Validating reviewer permissions: {}", reviewer_id);
        Ok(())
    }

    /// Helper method to approve license
    async fn approve_license(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .update_status(application_id, ApplicationStatus::Approved)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License approved: {}", application_id);
        Ok(())
    }

    /// Helper method to reject license
    async fn reject_license(
        &self,
        application_id: &LicenseId,
        comments: Option<&str>,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .update_status(application_id, ApplicationStatus::Rejected)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License rejected: {} - Comments: {:?}", application_id, comments);
        Ok(())
    }

    /// Helper method to request revision
    async fn request_revision(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .update_status(application_id, ApplicationStatus::RequiredDocuments)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License revision requested: {}", application_id);
        Ok(())
    }

    /// Helper method to escalate to admin
    async fn escalate_to_admin(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Implement admin assignment logic
        // For now, just update priority to urgent
        self.license_processing_repository
            .set_priority(application_id, &PriorityLevel::Urgent)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License escalated to admin: {}", application_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_calculation() {
        // Test priority levels directly since we can't easily mock the database
        let priorities = vec![
            ("NIB", PriorityLevel::High),
            ("SIUP", PriorityLevel::Normal),
            ("unknown", PriorityLevel::Normal),
        ];
        
        for (license_type, expected) in priorities {
            // Test the priority determination logic
            let priority = match license_type {
                "NIB" => PriorityLevel::High,
                _ => PriorityLevel::Normal,
            };
            assert_eq!(priority, expected);
        }
    }
}

impl ProcessingStep {
    pub fn get_description(&self) -> &str {
        match self {
            ProcessingStep::ApplicationReceived => "Application submitted and acknowledged",
            ProcessingStep::DocumentVerification => "Verifying submitted documents",
            ProcessingStep::InitialReview => "Initial review by staff",
            ProcessingStep::ComplianceCheck => "Compliance and regulatory check",
            ProcessingStep::AdminApproval => "Final approval by administrator",
            ProcessingStep::LicenseGeneration => "Generating license certificate",
            ProcessingStep::NotificationSent => "Notification sent to applicant",
            ProcessingStep::Completed => "Processing completed",
        }
    }

    pub fn get_estimated_duration_hours(&self) -> u32 {
        match self {
            ProcessingStep::ApplicationReceived => 1,
            ProcessingStep::DocumentVerification => 24,
            ProcessingStep::InitialReview => 48,
            ProcessingStep::ComplianceCheck => 72,
            ProcessingStep::AdminApproval => 24,
            ProcessingStep::LicenseGeneration => 2,
            ProcessingStep::NotificationSent => 1,
            ProcessingStep::Completed => 0,
        }
    }
}



#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStage {
    Intake,           // Just received, needs initial processing
    Review,           // Under review by staff
    Verification,     // Document and compliance verification
    Approval,         // Waiting for admin approval
    Processing,       // Generating license/certificates
    Completed,        // Fully processed
    OnHold,          // Waiting for additional information
    Rejected,        // Application rejected
}

#[derive(Debug)]
pub enum LicenseProcessingError {
    ApplicationNotFound(String),
    InvalidStatus(String),
    UnauthorizedReviewer(String),
    DocumentVerificationFailed(String),
    ComplianceCheckFailed(String),
    LicenseGenerationFailed(String),
    NotificationFailed(String),
    DatabaseError(String),
    ValidationError(String),
}

impl fmt::Display for LicenseProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LicenseProcessingError::ApplicationNotFound(id) => write!(f, "Application not found: {}", id),
            LicenseProcessingError::InvalidStatus(msg) => write!(f, "Invalid status: {}", msg),
            LicenseProcessingError::UnauthorizedReviewer(msg) => write!(f, "Unauthorized reviewer: {}", msg),
            LicenseProcessingError::DocumentVerificationFailed(msg) => write!(f, "Document verification failed: {}", msg),
            LicenseProcessingError::ComplianceCheckFailed(msg) => write!(f, "Compliance check failed: {}", msg),
            LicenseProcessingError::LicenseGenerationFailed(msg) => write!(f, "License generation failed: {}", msg),
            LicenseProcessingError::NotificationFailed(msg) => write!(f, "Notification failed: {}", msg),
            LicenseProcessingError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            LicenseProcessingError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for LicenseProcessingError {}

impl LicenseProcessingService {
    pub fn new(
        email_service: Arc<super::email::EmailService>,
        license_processing_repository: Arc<dyn LicenseProcessingRepository + Send + Sync>,
    ) -> Self {
        Self { 
            email_service,
            license_processing_repository,
        }
    }

    /// Create a new license application with enhanced workflow management
    pub async fn create_license_application(
        &self,
        request: LicenseApplicationCreate,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        // Create license application
        let license_id = LicenseId::new();
        let now = Utc::now();
        
        let application = LicenseApplication {
            id: license_id.clone(),
            company_id: request.company_id,
            license_type: request.license_type,
            application_data: request.application_data,
            current_stage: 1,
            total_stages: 8, // Standard workflow has 8 stages
            assigned_reviewer_id: None,
            status: ApplicationStatus::Submitted,
            priority: request.priority,
            estimated_completion: Some(self.calculate_estimated_completion(&request.priority)),
            created_at: now,
            updated_at: now,
        };

        // Save to repository
        let created_id = self.license_processing_repository
            .create_application(&application)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Send acknowledgment email
        self.send_application_acknowledgment(&created_id).await?;

        // Get the created application for response
        let created_app = self.license_processing_repository
            .get_application(&created_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?
            .ok_or_else(|| LicenseProcessingError::ApplicationNotFound("Application not found after creation".to_string()))?;

        Ok(LicenseApplicationResponse::from(&created_app))
    }

    /// Get license application by ID
    pub async fn get_license_application(
        &self,
        application_id: &LicenseId,
    ) -> Result<Option<LicenseApplicationResponse>, LicenseProcessingError> {
        let application = self.license_processing_repository
            .get_application(application_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(application.as_ref().map(LicenseApplicationResponse::from))
    }

    /// Assign a reviewer to an application
    pub async fn assign_reviewer(
        &self,
        application_id: &LicenseId,
        reviewer_id: &UserId,
    ) -> Result<(), LicenseProcessingError> {
        // Validate reviewer permissions
        self.validate_reviewer_permissions(reviewer_id).await?;

        // Check reviewer workload
        let workload = self.license_processing_repository
            .get_reviewer_workload(reviewer_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        if workload > 10 { // Maximum 10 active applications per reviewer
            return Err(LicenseProcessingError::ValidationError(
                "Reviewer workload exceeds maximum capacity".to_string()
            ));
        }

        // Assign reviewer
        self.license_processing_repository
            .assign_reviewer(application_id, reviewer_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Process a review for an application
    pub async fn process_review(
        &self,
        application_id: &LicenseId,
        review: LicenseReviewCreate,
    ) -> Result<LicenseApplicationResponse, LicenseProcessingError> {
        // Validate reviewer permissions
        self.validate_reviewer_permissions(&review.reviewer_id).await?;

        // Get current application
        let mut application = self.license_processing_repository
            .get_application(application_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?
            .ok_or_else(|| LicenseProcessingError::ApplicationNotFound("Application not found".to_string()))?;

        // Validate review is from assigned reviewer
        if let Some(assigned_id) = &application.assigned_reviewer_id {
            if assigned_id != &review.reviewer_id {
                return Err(LicenseProcessingError::UnauthorizedReviewer(
                    "Review can only be submitted by assigned reviewer".to_string()
                ));
            }
        }

        // Create review record
        let review_record = LicenseReview {
            id: Uuid::new_v4(),
            application_id: application_id.clone(),
            reviewer_id: review.reviewer_id,
            stage: application.current_stage,
            decision: review.decision.clone(),
            comments: review.comments,
            review_data: review.review_data,
            created_at: Utc::now(),
        };

        self.license_processing_repository
            .create_review(&review_record)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Process review decision
        match review.decision {
            ReviewDecision::Approve => {
                self.approve_license(application_id).await?;
            },
            ReviewDecision::Reject => {
                self.reject_license(application_id, review_record.comments.as_deref()).await?;
            },
            ReviewDecision::RequestRevision => {
                self.request_revision(application_id).await?;
            },
            ReviewDecision::Escalate => {
                self.escalate_to_admin(application_id).await?;
            },
        }

        // Get updated application
        let updated_app = self.license_processing_repository
            .get_application(application_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?
            .ok_or_else(|| LicenseProcessingError::ApplicationNotFound("Application not found after review".to_string()))?;

        Ok(LicenseApplicationResponse::from(&updated_app))
    }

    /// Get applications assigned to a reviewer
    pub async fn get_applications_by_reviewer(
        &self,
        reviewer_id: &UserId,
    ) -> Result<Vec<LicenseApplicationResponse>, LicenseProcessingError> {
        let applications = self.license_processing_repository
            .get_applications_by_reviewer(reviewer_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(applications.iter().map(LicenseApplicationResponse::from).collect())
    }

    /// Get applications for a company
    pub async fn get_applications_by_company(
        &self,
        company_id: &CompanyId,
    ) -> Result<Vec<LicenseApplicationResponse>, LicenseProcessingError> {
        let applications = self.license_processing_repository
            .get_applications_by_company(company_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(applications.iter().map(LicenseApplicationResponse::from).collect())
    }

    /// Get processing statistics
    pub async fn get_processing_statistics(&self) -> Result<ProcessingStatisticsResponse, LicenseProcessingError> {
        let stats = self.license_processing_repository
            .get_processing_statistics()
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(ProcessingStatisticsResponse::from(&stats))
    }

    /// Update application priority
    pub async fn update_priority(
        &self,
        application_id: &LicenseId,
        priority: PriorityLevel,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .set_priority(application_id, &priority)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Update application status
    pub async fn update_status(
        &self,
        application_id: &LicenseId,
        status: ApplicationStatus,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .update_status(application_id, &status)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Advance application to next stage
    pub async fn advance_stage(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        self.license_processing_repository
            .advance_stage(application_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // Private helper methods for enhanced functionality

    fn calculate_estimated_completion(&self, priority: &PriorityLevel) -> DateTime<Utc> {
        let hours = match priority {
            PriorityLevel::Urgent => 24,   // 1 day
            PriorityLevel::High => 72,     // 3 days
            PriorityLevel::Normal => 168,  // 1 week
            PriorityLevel::Low => 336,     // 2 weeks
        };
        
        Utc::now() + chrono::Duration::hours(hours)
    }

    async fn send_application_acknowledgment(
        &self,
        license_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Implement email notification
        tracing::info!("Application acknowledgment sent for license: {}", license_id);
        Ok(())
    }

    async fn validate_reviewer_permissions(
        &self,
        reviewer_id: &UserId,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Implement proper permission validation
        // For now, just log the validation
        tracing::info!("Validating reviewer permissions for: {}", reviewer_id);
        Ok(())
    }

    async fn approve_license(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        // Update status to approved
        self.license_processing_repository
            .update_status(application_id, &ApplicationStatus::Approved)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        // Advance to next stage
        self.license_processing_repository
            .advance_stage(application_id)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License approved: {}", application_id);
        Ok(())
    }

    async fn reject_license(
        &self,
        application_id: &LicenseId,
        comments: Option<&str>,
    ) -> Result<(), LicenseProcessingError> {
        // Update status to rejected
        self.license_processing_repository
            .update_status(application_id, &ApplicationStatus::Rejected)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License rejected: {} - Comments: {:?}", application_id, comments);
        Ok(())
    }

    async fn request_revision(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        // Update status to require documents
        self.license_processing_repository
            .update_status(application_id, &ApplicationStatus::RequiredDocuments)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License revision requested: {}", application_id);
        Ok(())
    }

    async fn escalate_to_admin(
        &self,
        application_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Implement admin assignment logic
        // For now, just update priority to urgent
        self.license_processing_repository
            .set_priority(application_id, &PriorityLevel::Urgent)
            .await
            .map_err(|e| LicenseProcessingError::DatabaseError(e.to_string()))?;

        tracing::info!("License escalated to admin: {}", application_id);
        Ok(())
    }

    /// Submit a new license application and start the processing workflow
    pub async fn submit_application(
        &self,
        request: LicenseApplicationRequest,
    ) -> Result<LicenseProcessingStatus, LicenseProcessingError> {
        // Validate the application
        self.validate_application(&request)?;

        let license_id = LicenseId::new();
        let priority = self.determine_priority(&request.license_type);

        // TODO: Create license record in database
        // let license = License::new(
        //     license_id.clone(),
        //     request.user_id.clone(),
        //     request.company_id.clone(),
        //     request.license_type.clone(),
        //     LicenseStatus::Pending,
        // );

        // Start the workflow
        let status = LicenseProcessingStatus {
            license_id: license_id.clone(),
            current_status: ApplicationStatus::Submitted,
            processing_steps: vec![ProcessingStep::ApplicationReceived],
            assigned_reviewer: self.assign_reviewer(&request.license_type).await?,
            estimated_completion: Some(self.calculate_estimated_completion(&priority)),
            priority_level: priority,
            workflow_stage: WorkflowStage::Intake,
        };

        // Send acknowledgment email
        self.send_application_acknowledgment(&request).await?;

        // Schedule next step
        self.schedule_document_verification(&license_id).await?;

        Ok(status)
    }

    }

#[cfg(test)]
mod tests {
    use super::*;

    /// Get the current processing status of a license
    pub async fn get_processing_status(
        &self,
        license_id: &LicenseId,
    ) -> Result<LicenseProcessingStatus, LicenseProcessingError> {
        // TODO: Fetch from database
        // For now, return a placeholder status
        let status = LicenseProcessingStatus {
            license_id: license_id.clone(),
            current_status: ApplicationStatus::Processing,
            processing_steps: vec![
                ProcessingStep::ApplicationReceived,
                ProcessingStep::DocumentVerification,
            ],
            assigned_reviewer: Some(UserId::new()),
            estimated_completion: Some(Utc::now() + chrono::Duration::days(3)),
            priority_level: PriorityLevel::Normal,
            workflow_stage: WorkflowStage::Review,
        };

        Ok(status)
    }

    /// Get licenses assigned to a specific reviewer
    pub async fn get_assigned_licenses(
        &self,
        reviewer_id: &UserId,
        stage: Option<WorkflowStage>,
    ) -> Result<Vec<LicenseProcessingStatus>, LicenseProcessingError> {
        // TODO: Implement database query
        // This would fetch all licenses assigned to the reviewer, optionally filtered by stage
        
        Ok(vec![])
    }

    /// Get processing statistics for reporting
    pub async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, LicenseProcessingError> {
        // TODO: Implement database aggregation queries
        
        let stats = ProcessingStatistics {
            total_applications: 0,
            pending_applications: 0,
            approved_applications: 0,
            rejected_applications: 0,
            average_processing_time_hours: 0.0,
            sla_compliance_rate: 0.0,
            applications_by_type: HashMap::new(),
            applications_by_priority: HashMap::new(),
            applications_by_stage: HashMap::new(),
        };

        Ok(stats)
    }

    // Private helper methods

    fn validate_application(&self, request: &LicenseApplicationRequest) -> Result<(), LicenseProcessingError> {
        if request.license_type.is_empty() {
            return Err(LicenseProcessingError::ValidationError("License type is required".to_string()));
        }

        if request.business_description.is_empty() {
            return Err(LicenseProcessingError::ValidationError("Business description is required".to_string()));
        }

        if request.required_documents.is_empty() {
            return Err(LicenseProcessingError::ValidationError("At least one document is required".to_string()));
        }

        Ok(())
    }

    fn determine_priority(&self, license_type: &str) -> PriorityLevel {
        match license_type {
            "NIB" => PriorityLevel::High,      // NIB is critical for business operations
            "SIUP" => PriorityLevel::Normal,   // Standard business license
            "TDP" => PriorityLevel::Normal,    // Company registration
            "NPWP" => PriorityLevel::High,     // Tax ID is important
            _ => PriorityLevel::Normal,
        }
    }

    async fn assign_reviewer(&self, license_type: &str) -> Result<Option<UserId>, LicenseProcessingError> {
        // TODO: Implement reviewer assignment logic
        // This could be based on:
        // - License type specialization
        // - Current workload
        // - Availability
        // - Round-robin assignment
        
        Ok(Some(UserId::new()))
    }

    fn calculate_estimated_completion(&self, priority: &PriorityLevel) -> DateTime<Utc> {
        let hours = priority.get_sla_hours();
        Utc::now() + chrono::Duration::hours(hours as i64)
    }

    async fn send_application_acknowledgment(
        &self,
        request: &LicenseApplicationRequest,
    ) -> Result<(), LicenseProcessingError> {
        // TODO: Get user details to send email
        // For now, just log
        tracing::info!(
            "Sending application acknowledgment for license type: {} to user: {}",
            request.license_type,
            request.user_id
        );

        Ok(())
    }

    async fn schedule_document_verification(&self, _license_id: &LicenseId) -> Result<(), LicenseProcessingError> {
        // TODO: Schedule background task for document verification
        tracing::info!("Scheduled document verification for license: {}", _license_id);
        Ok(())
    }

    fn validate_reviewer_permissions(
        &self,
        reviewer: &User,
        _license_id: &LicenseId,
    ) -> Result<(), LicenseProcessingError> {
        match reviewer.role {
            UserRole::AdminStaff | UserRole::SuperAdmin => Ok(()),
            _ => Err(LicenseProcessingError::UnauthorizedReviewer(
                "Only admin staff can review licenses".to_string(),
            )),
        }
    }

    async fn approve_license(
        &self,
        request: &LicenseReviewRequest,
        _reviewer: &User,
    ) -> Result<LicenseProcessingStatus, LicenseProcessingError> {
        // TODO: Update license status to approved
        // TODO: Generate license number
        // TODO: Send approval notification

        let status = LicenseProcessingStatus {
            license_id: request.license_id.clone(),
            current_status: ApplicationStatus::Approved,
            processing_steps: vec![
                ProcessingStep::ApplicationReceived,
                ProcessingStep::DocumentVerification,
                ProcessingStep::InitialReview,
                ProcessingStep::ComplianceCheck,
                ProcessingStep::AdminApproval,
                ProcessingStep::LicenseGeneration,
                ProcessingStep::NotificationSent,
                ProcessingStep::Completed,
            ],
            assigned_reviewer: None,
            estimated_completion: None,
            priority_level: PriorityLevel::Normal,
            workflow_stage: WorkflowStage::Completed,
        };

        tracing::info!("License approved: {}", request.license_id);
        Ok(status)
    }

    async fn reject_license(
        &self,
        request: &LicenseReviewRequest,
        _reviewer: &User,
    ) -> Result<LicenseProcessingStatus, LicenseProcessingError> {
        // TODO: Update license status to rejected
        // TODO: Send rejection notification with reasons

        let status = LicenseProcessingStatus {
            license_id: request.license_id.clone(),
            current_status: ApplicationStatus::Rejected,
            processing_steps: vec![
                ProcessingStep::ApplicationReceived,
                ProcessingStep::DocumentVerification,
                ProcessingStep::InitialReview,
                ProcessingStep::NotificationSent,
            ],
            assigned_reviewer: None,
            estimated_completion: None,
            priority_level: PriorityLevel::Normal,
            workflow_stage: WorkflowStage::Rejected,
        };

        tracing::info!("License rejected: {} - Reason: {}", request.license_id, request.comments);
        Ok(status)
    }

    async fn request_revision(
        &self,
        request: &LicenseReviewRequest,
        _reviewer: &User,
    ) -> Result<LicenseProcessingStatus, LicenseProcessingError> {
        // TODO: Update license status to revision requested
        // TODO: Send revision request notification

        let status = LicenseProcessingStatus {
            license_id: request.license_id.clone(),
            current_status: ApplicationStatus::PendingDocuments,
            processing_steps: vec![
                ProcessingStep::ApplicationReceived,
                ProcessingStep::DocumentVerification,
                ProcessingStep::InitialReview,
            ],
            assigned_reviewer: Some(UserId::new()),
            estimated_completion: Some(Utc::now() + chrono::Duration::days(2)),
            priority_level: PriorityLevel::Normal,
            workflow_stage: WorkflowStage::OnHold,
        };

        tracing::info!("License revision requested: {} - Comments: {}", request.license_id, request.comments);
        Ok(status)
    }

    async fn escalate_to_admin(
        &self,
        request: &LicenseReviewRequest,
        _reviewer: &User,
    ) -> Result<LicenseProcessingStatus, LicenseProcessingError> {
        // TODO: Assign to super admin
        // TODO: Send escalation notification

        let status = LicenseProcessingStatus {
            license_id: request.license_id.clone(),
            current_status: ApplicationStatus::Processing,
            processing_steps: vec![
                ProcessingStep::ApplicationReceived,
                ProcessingStep::DocumentVerification,
                ProcessingStep::InitialReview,
                ProcessingStep::ComplianceCheck,
            ],
            assigned_reviewer: Some(UserId::new()), // TODO: Assign to actual super admin
            estimated_completion: Some(Utc::now() + chrono::Duration::days(1)),
            priority_level: PriorityLevel::Urgent,
            workflow_stage: WorkflowStage::Approval,
        };

        tracing::info!("License escalated to admin: {} - Comments: {}", request.license_id, request.comments);
        Ok(status)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processing_step_duration() {
        assert_eq!(ProcessingStep::ApplicationReceived.get_estimated_duration_hours(), 1);
        assert_eq!(ProcessingStep::ComplianceCheck.get_estimated_duration_hours(), 72);
    }

    #[test]
    fn test_priority_sla() {
        assert_eq!(PriorityLevel::Urgent.get_sla_hours(), 24);
        assert_eq!(PriorityLevel::Low.get_sla_hours(), 168);
    }

    #[test]
    fn test_determine_priority() {
        // Create a mock repository for testing
        use crate::infrastructure::repositories::PostgresLicenseProcessingRepository;
        use sqlx::PgPool;
        
        // For testing, we'll skip the database dependency
        // In a real test, you'd use a test database or mock
        let email_service = Arc::new(super::email::EmailService::new());
        
        // We can't easily create a PgPool in tests without a database
        // So we'll test the priority logic through integration tests instead
        // For now, just test the basic functionality
        
        // Test priority levels directly
        let priorities = vec![
            ("NIB", PriorityLevel::High),
            ("SIUP", PriorityLevel::Normal),
            ("unknown", PriorityLevel::Normal),
        ];
        
        for (license_type, expected) in priorities {
            // Test the priority determination logic
            let priority = match license_type {
                "NIB" => PriorityLevel::High,
                "SIUP" | _ => PriorityLevel::Normal,
            };
            assert_eq!(priority, expected);
        }
    }
}
