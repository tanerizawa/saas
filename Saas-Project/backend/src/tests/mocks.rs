#[cfg(test)]
pub mod mocks {
    use async_trait::async_trait;
    use mockall::mock;
    use mockall::predicate::*;
    use uuid::Uuid;
    
    use crate::application::services::AuthService;
    use crate::domain::entities::{User, Company, License};
    use crate::domain::repositories::{UserRepository, CompanyRepository, LicenseRepository};
    use crate::domain::value_objects::{UserId, CompanyId, LicenseId, Email};
    use crate::shared::errors::{AppResult, AppError};
    
    // Mock User Repository
    mock! {
        pub UserRepository {}
        
        #[async_trait]
        impl UserRepository for UserRepository {
            async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>>;
            async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
            async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>>;
            async fn count_all(&self) -> AppResult<i64>;
            async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>>;
            async fn save(&self, user: &User) -> AppResult<()>;
            async fn delete(&self, id: &UserId) -> AppResult<()>;
        }
    }
    
    // Mock Company Repository
    mock! {
        pub CompanyRepository {}
        
        #[async_trait]
        impl CompanyRepository for CompanyRepository {
            async fn find_by_id(&self, id: &uuid::Uuid) -> AppResult<Option<Company>>;
            async fn find_by_owner_id(&self, owner_id: &uuid::Uuid) -> AppResult<Vec<Company>>;
            async fn find_by_nib(&self, nib: &str) -> AppResult<Option<Company>>;
            async fn save(&self, company: &Company) -> AppResult<()>;
            async fn update(&self, company: &Company) -> AppResult<()>;
            async fn delete(&self, id: &uuid::Uuid) -> AppResult<()>;
            async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<Company>>;
            async fn count_by_owner(&self, owner_id: &uuid::Uuid) -> AppResult<i64>;
            async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<Company>>;
        }
    }
    
    // Mock License Repository
    mock! {
        pub LicenseRepository {}
        
        #[async_trait]
        impl LicenseRepository for LicenseRepository {
            async fn find_by_id(&self, id: &LicenseId) -> AppResult<Option<License>>;
            async fn find_by_user_id(&self, user_id: &UserId) -> AppResult<Vec<License>>;
            async fn save(&self, license: &License) -> AppResult<()>;
            async fn delete(&self, id: &LicenseId) -> AppResult<()>;
        }
    }
    
    // Mock Auth Service
    mock! {
        pub AuthService {}
        
        #[async_trait]
        impl AuthService for AuthService {
            async fn hash_password(&self, password: &str) -> AppResult<String>;
            async fn verify_password(&self, hash: &str, password: &str) -> AppResult<bool>;
            async fn generate_token(&self, user_id: &UserId) -> AppResult<String>;
            async fn validate_token(&self, token: &str) -> AppResult<String>;
        }
    }
}
