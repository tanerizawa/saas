// Infrastructure repositories module
// PostgreSQL implementations of domain repositories

pub mod account_repository;
pub mod cached_license_repository;
pub mod company_repository;
pub mod license_repository;
pub mod postgres_user_repository;
pub mod in_memory_user_repository;
pub mod transaction_repository;

// Export only one LicenseRepository trait - the one from cached_license_repository
pub use cached_license_repository::CachedLicenseRepository;
pub use cached_license_repository::LicenseRepository;
pub use company_repository::PostgresCompanyRepository;
// pub use license_repository::PostgresLicenseRepositoryImpl;
pub use postgres_user_repository::PostgresUserRepository;
