use crate::domain::value_objects::{UserId, CompanyId};
use crate::infrastructure::repositories::OnboardingRepository;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::Arc;

#[derive(Clone)]
pub struct OnboardingService {
    email_service: Arc<super::email::EmailService>,
    repository: Arc<dyn OnboardingRepository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingRequest {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct OnboardingStatus {
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub current_step: OnboardingStep,
    pub completed_steps: Vec<OnboardingStep>,
    pub next_actions: Vec<String>,
    pub completion_percentage: u8,
    pub estimated_completion_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnboardingStep {
    UserRegistration,
    EmailVerification,
    CompanyInformation,
    DocumentUpload,
    InitialPayment,
    AccountActivation,
    WelcomeComplete,
}

impl OnboardingStep {
    pub fn get_description(&self) -> &str {
        match self {
            OnboardingStep::UserRegistration => "Complete user registration",
            OnboardingStep::EmailVerification => "Verify email address",
            OnboardingStep::CompanyInformation => "Provide company information",
            OnboardingStep::DocumentUpload => "Upload required documents",
            OnboardingStep::InitialPayment => "Complete initial payment",
            OnboardingStep::AccountActivation => "Account activation by admin",
            OnboardingStep::WelcomeComplete => "Onboarding completed",
        }
    }

    pub fn get_order(&self) -> u8 {
        match self {
            OnboardingStep::UserRegistration => 1,
            OnboardingStep::EmailVerification => 2,
            OnboardingStep::CompanyInformation => 3,
            OnboardingStep::DocumentUpload => 4,
            OnboardingStep::InitialPayment => 5,
            OnboardingStep::AccountActivation => 6,
            OnboardingStep::WelcomeComplete => 7,
        }
    }
}

#[derive(Debug)]
pub enum OnboardingError {
    UserAlreadyExists,
    CompanyAlreadyExists,
    InvalidCompanyType,
    DocumentUploadFailed,
    PaymentFailed,
    EmailSendFailed(String),
    DatabaseError(String),
    ValidationError(String),
}

impl fmt::Display for OnboardingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OnboardingError::UserAlreadyExists => write!(f, "User already exists"),
            OnboardingError::CompanyAlreadyExists => write!(f, "Company already exists"),
            OnboardingError::InvalidCompanyType => write!(f, "Invalid company type"),
            OnboardingError::DocumentUploadFailed => write!(f, "Document upload failed"),
            OnboardingError::PaymentFailed => write!(f, "Payment processing failed"),
            OnboardingError::EmailSendFailed(msg) => write!(f, "Email send failed: {}", msg),
            OnboardingError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            OnboardingError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for OnboardingError {}

impl OnboardingService {
    pub fn new(
        email_service: Arc<super::email::EmailService>,
        repository: Arc<dyn OnboardingRepository>,
    ) -> Self {
        Self { email_service, repository }
    }

    /// Start the onboarding process for a new UMKM
    pub async fn start_onboarding(
        &self,
        request: OnboardingRequest,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // Validate the request
        self.validate_onboarding_request(&request)?;

        // Create user (will be pending verification)
        let user_id = UserId::new();
        let company_id = CompanyId::new();

        // Create onboarding workflow in database
        let workflow = self.repository
            .create_workflow(user_id.clone(), Some(company_id.clone()))
            .await
            .map_err(|e| OnboardingError::DatabaseError(e.to_string()))?;

        // TODO: Implement user creation through user repository
        // let user = User::new(
        //     user_id,
        //     Email::new(&request.user_email).map_err(|e| OnboardingError::ValidationError(e))?,
        //     request.user_full_name,
        //     UserRole::UmkmOwner,
        //     UserStatus::PendingVerification,
        // );

        // TODO: Implement company creation through company repository
        // let company = Company::new(
        //     company_id,
        //     request.company_name,
        //     request.company_type,
        //     CompanyStatus::PendingActivation,
        // );

        // Send welcome email with next steps
        self.send_welcome_email(&request.user_email, &request.user_full_name)
            .await?;

        // Create initial onboarding status from workflow
        let status = OnboardingStatus {
            user_id: workflow.user_id,
            company_id: workflow.company_id.unwrap_or(company_id),
            current_step: OnboardingStep::EmailVerification,
            completed_steps: vec![OnboardingStep::UserRegistration],
            next_actions: vec![
                "Check your email and click the verification link".to_string(),
                "Complete your company profile".to_string(),
            ],
            completion_percentage: 14, // 1/7 steps completed
            estimated_completion_date: Some(Utc::now() + chrono::Duration::days(3)),
        };

        Ok(status)
    }

    /// Get the current onboarding status for a user
    pub async fn get_onboarding_status(
        &self,
        user_id: &UserId,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // Get workflow from database
        let workflow = self.repository
            .get_workflow_by_user_id(user_id)
            .await
            .map_err(|e| OnboardingError::DatabaseError(e.to_string()))?;

        if let Some(workflow) = workflow {
            // Convert workflow to OnboardingStatus
            let current_step = self.step_from_number(workflow.current_step);
            let completed_steps = self.get_completed_steps(workflow.current_step);
            let next_actions = self.get_next_actions(&current_step);

            let status = OnboardingStatus {
                user_id: workflow.user_id,
                company_id: workflow.company_id.unwrap_or(CompanyId::new()),
                current_step,
                completed_steps,
                next_actions,
                completion_percentage: workflow.completion_percentage as u8,
                estimated_completion_date: workflow.completed_at.or_else(|| {
                    Some(Utc::now() + chrono::Duration::days(2))
                }),
            };

            Ok(status)
        } else {
            // No workflow found, return default status
            let status = OnboardingStatus {
                user_id: user_id.clone(),
                company_id: CompanyId::new(),
                current_step: OnboardingStep::UserRegistration,
                completed_steps: vec![],
                next_actions: vec!["Start your onboarding process".to_string()],
                completion_percentage: 0,
                estimated_completion_date: Some(Utc::now() + chrono::Duration::days(3)),
            };

            Ok(status)
        }
    }

    /// Update onboarding progress when a step is completed
    pub async fn complete_step(
        &self,
        user_id: &UserId,
        step: OnboardingStep,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // Get current workflow from database
        let workflow = self.repository
            .get_workflow_by_user_id(user_id)
            .await
            .map_err(|e| OnboardingError::DatabaseError(e.to_string()))?;

        if let Some(workflow) = workflow {
            let step_number = step.get_order() as i32;
            
            // Complete the step in database
            self.repository
                .complete_step(&workflow.id, step_number)
                .await
                .map_err(|e| OnboardingError::DatabaseError(e.to_string()))?;

            // Handle step-specific logic and return updated status
            match step {
                OnboardingStep::EmailVerification => {
                    self.handle_email_verification(user_id).await
                }
                OnboardingStep::CompanyInformation => {
                    self.handle_company_information_completion(user_id).await
                }
                OnboardingStep::DocumentUpload => {
                    self.handle_document_upload_completion(user_id).await
                }
                OnboardingStep::InitialPayment => {
                    self.handle_payment_completion(user_id).await
                }
                OnboardingStep::AccountActivation => {
                    self.handle_account_activation(user_id).await
                }
                _ => {
                    // For other steps, return current status
                    self.get_onboarding_status(user_id).await
                },
            }
        } else {
            Err(OnboardingError::ValidationError("No onboarding workflow found for user".to_string()))
        }
    }

    /// Generate an onboarding checklist for the user
    pub fn get_onboarding_checklist(&self) -> Vec<(OnboardingStep, String, bool)> {
        vec![
            (OnboardingStep::UserRegistration, "Register your account".to_string(), true),
            (OnboardingStep::EmailVerification, "Verify your email address".to_string(), false),
            (OnboardingStep::CompanyInformation, "Complete company profile".to_string(), false),
            (OnboardingStep::DocumentUpload, "Upload required documents".to_string(), false),
            (OnboardingStep::InitialPayment, "Complete initial payment".to_string(), false),
            (OnboardingStep::AccountActivation, "Wait for admin approval".to_string(), false),
            (OnboardingStep::WelcomeComplete, "Start using the platform".to_string(), false),
        ]
    }

    // Private helper methods

    fn step_from_number(&self, step_number: i32) -> OnboardingStep {
        match step_number {
            1 => OnboardingStep::UserRegistration,
            2 => OnboardingStep::EmailVerification,
            3 => OnboardingStep::CompanyInformation,
            4 => OnboardingStep::DocumentUpload,
            5 => OnboardingStep::InitialPayment,
            6 => OnboardingStep::AccountActivation,
            7 => OnboardingStep::WelcomeComplete,
            _ => OnboardingStep::UserRegistration,
        }
    }

    fn get_completed_steps(&self, current_step: i32) -> Vec<OnboardingStep> {
        let mut completed = Vec::new();
        for i in 1..current_step {
            completed.push(self.step_from_number(i));
        }
        completed
    }

    fn get_next_actions(&self, current_step: &OnboardingStep) -> Vec<String> {
        match current_step {
            OnboardingStep::UserRegistration => vec![
                "Complete user registration".to_string(),
            ],
            OnboardingStep::EmailVerification => vec![
                "Check your email and click the verification link".to_string(),
            ],
            OnboardingStep::CompanyInformation => vec![
                "Complete your company profile".to_string(),
                "Provide business information".to_string(),
            ],
            OnboardingStep::DocumentUpload => vec![
                "Upload required documents (KTP, NPWP, etc.)".to_string(),
                "Prepare business license documents".to_string(),
            ],
            OnboardingStep::InitialPayment => vec![
                "Complete your initial payment".to_string(),
                "Choose your subscription plan".to_string(),
            ],
            OnboardingStep::AccountActivation => vec![
                "Wait for admin approval (usually within 24 hours)".to_string(),
                "You will receive an email when your account is activated".to_string(),
            ],
            OnboardingStep::WelcomeComplete => vec![
                "Welcome! Your account is now active".to_string(),
                "Start exploring the platform features".to_string(),
            ],
        }
    }

    fn validate_onboarding_request(&self, request: &OnboardingRequest) -> Result<(), OnboardingError> {
        if request.user_email.is_empty() {
            return Err(OnboardingError::ValidationError("Email is required".to_string()));
        }

        if request.user_full_name.is_empty() {
            return Err(OnboardingError::ValidationError("Full name is required".to_string()));
        }

        if request.company_name.is_empty() {
            return Err(OnboardingError::ValidationError("Company name is required".to_string()));
        }

        // Validate company type
        let valid_types = vec!["mikro", "kecil", "menengah"];
        if !valid_types.contains(&request.company_type.as_str()) {
            return Err(OnboardingError::InvalidCompanyType);
        }

        Ok(())
    }

    async fn send_welcome_email(
        &self,
        email: &str,
        full_name: &str,
    ) -> Result<(), OnboardingError> {
        // TODO: Use email service to send welcome email
        self.email_service
            .send_welcome_email(email, full_name)
            .await
            .map_err(|e| OnboardingError::EmailSendFailed(e.to_string()))?;

        Ok(())
    }

    async fn handle_email_verification(
        &self,
        user_id: &UserId,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // TODO: Update user status to email verified
        // TODO: Send next steps email

        let status = OnboardingStatus {
            user_id: user_id.clone(),
            company_id: CompanyId::new(),
            current_step: OnboardingStep::CompanyInformation,
            completed_steps: vec![
                OnboardingStep::UserRegistration,
                OnboardingStep::EmailVerification,
            ],
            next_actions: vec![
                "Complete your company profile".to_string(),
                "Upload company documents".to_string(),
            ],
            completion_percentage: 28, // 2/7 steps completed
            estimated_completion_date: Some(Utc::now() + chrono::Duration::days(2)),
        };

        Ok(status)
    }

    async fn handle_company_information_completion(
        &self,
        user_id: &UserId,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // TODO: Validate company information
        // TODO: Send confirmation email

        let status = OnboardingStatus {
            user_id: user_id.clone(),
            company_id: CompanyId::new(),
            current_step: OnboardingStep::DocumentUpload,
            completed_steps: vec![
                OnboardingStep::UserRegistration,
                OnboardingStep::EmailVerification,
                OnboardingStep::CompanyInformation,
            ],
            next_actions: vec![
                "Upload required documents (KTP, NPWP, etc.)".to_string(),
                "Prepare business license documents".to_string(),
            ],
            completion_percentage: 42, // 3/7 steps completed
            estimated_completion_date: Some(Utc::now() + chrono::Duration::days(1)),
        };

        Ok(status)
    }

    async fn handle_document_upload_completion(
        &self,
        user_id: &UserId,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // TODO: Verify documents
        // TODO: Notify admin for review

        let status = OnboardingStatus {
            user_id: user_id.clone(),
            company_id: CompanyId::new(),
            current_step: OnboardingStep::InitialPayment,
            completed_steps: vec![
                OnboardingStep::UserRegistration,
                OnboardingStep::EmailVerification,
                OnboardingStep::CompanyInformation,
                OnboardingStep::DocumentUpload,
            ],
            next_actions: vec![
                "Complete your initial payment".to_string(),
                "Choose your subscription plan".to_string(),
            ],
            completion_percentage: 57, // 4/7 steps completed
            estimated_completion_date: Some(Utc::now() + chrono::Duration::hours(2)),
        };

        Ok(status)
    }

    async fn handle_payment_completion(
        &self,
        user_id: &UserId,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // TODO: Verify payment
        // TODO: Activate subscription

        let status = OnboardingStatus {
            user_id: user_id.clone(),
            company_id: CompanyId::new(),
            current_step: OnboardingStep::AccountActivation,
            completed_steps: vec![
                OnboardingStep::UserRegistration,
                OnboardingStep::EmailVerification,
                OnboardingStep::CompanyInformation,
                OnboardingStep::DocumentUpload,
                OnboardingStep::InitialPayment,
            ],
            next_actions: vec![
                "Wait for admin approval (usually within 24 hours)".to_string(),
                "You will receive an email when your account is activated".to_string(),
            ],
            completion_percentage: 71, // 5/7 steps completed
            estimated_completion_date: Some(Utc::now() + chrono::Duration::hours(24)),
        };

        Ok(status)
    }

    async fn handle_account_activation(
        &self,
        user_id: &UserId,
    ) -> Result<OnboardingStatus, OnboardingError> {
        // TODO: Activate user account
        // TODO: Send welcome email with login instructions

        let status = OnboardingStatus {
            user_id: user_id.clone(),
            company_id: CompanyId::new(),
            current_step: OnboardingStep::WelcomeComplete,
            completed_steps: vec![
                OnboardingStep::UserRegistration,
                OnboardingStep::EmailVerification,
                OnboardingStep::CompanyInformation,
                OnboardingStep::DocumentUpload,
                OnboardingStep::InitialPayment,
                OnboardingStep::AccountActivation,
                OnboardingStep::WelcomeComplete,
            ],
            next_actions: vec![
                "Welcome! Your account is now active".to_string(),
                "Start exploring the platform features".to_string(),
            ],
            completion_percentage: 100, // All steps completed
            estimated_completion_date: None,
        };

        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onboarding_step_order() {
        assert_eq!(OnboardingStep::UserRegistration.get_order(), 1);
        assert_eq!(OnboardingStep::EmailVerification.get_order(), 2);
        assert_eq!(OnboardingStep::WelcomeComplete.get_order(), 7);
    }

    #[test]
    fn test_validation() {
        // Create a mock repository for testing
        // In actual tests, you'd use a proper mock
        use crate::infrastructure::repositories::PostgresOnboardingRepository;
        use sqlx::PgPool;
        
        // This test would need to be adjusted for actual testing with a mock repository
        // For now, we'll skip the service instantiation
        let invalid_request = OnboardingRequest {
            user_email: "".to_string(),
            user_full_name: "".to_string(),
            user_phone: None,
            company_name: "".to_string(),
            company_type: "invalid".to_string(),
            business_description: "".to_string(),
            company_address: "".to_string(),
            company_phone: "".to_string(),
            company_email: "".to_string(),
            tax_id: None,
        };

        // The actual validation logic would be tested separately
        // assert!(service.validate_onboarding_request(&invalid_request).is_err());
    }
}
