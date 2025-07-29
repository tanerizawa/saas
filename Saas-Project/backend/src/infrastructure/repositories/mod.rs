// Infrastructure repositories module
// PostgreSQL implementations of domain repositories

pub mod account_repository;
pub mod cached_license_repository;
pub mod company_repository;
pub mod license_repository;
pub mod postgres_user_repository;
pub mod transaction_repository;

// Business workflow repositories
pub mod onboarding_repository;
pub mod system_config_repository;
pub mod email_repository;
pub mod license_processing_repository;

// Export implementations
pub use company_repository::PostgresCompanyRepository;
pub use license_repository::PostgresLicenseRepositoryImpl;
pub use postgres_user_repository::PostgresUserRepository;
// Re-export the LicenseRepository trait from cached_license_repository
pub use cached_license_repository::LicenseRepository;

// Export business workflow repositories
pub use onboarding_repository::{OnboardingRepository, PostgresOnboardingRepository, OnboardingWorkflow, OnboardingStep};
pub use system_config_repository::{SystemConfigRepository, PostgresSystemConfigRepository, SystemConfigGroup, SystemConfigSetting};
pub use email_repository::{EmailRepository, PostgresEmailRepository, EmailTemplate, EmailLog};
pub use license_processing_repository::{PostgresLicenseProcessingRepository, LicenseProcessingRepository};
