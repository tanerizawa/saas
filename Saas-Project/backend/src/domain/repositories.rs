// Repository traits for data access abstraction (Hexagonal Architecture)
#![allow(dead_code)]

use async_trait::async_trait;
use crate::domain::value_objects::*;
use crate::domain::entities::*;
use crate::domain::companies::Company;
use crate::shared::errors::AppResult;

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
    async fn save(&self, user: &User) -> AppResult<()>;
    async fn delete(&self, id: &UserId) -> AppResult<()>;
    // Add new methods for pagination and search
    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>>;
    async fn count_all(&self) -> AppResult<i64>;
    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>>;
}

#[async_trait]
pub trait LicenseRepository {
    async fn find_by_id(&self, id: &LicenseId) -> AppResult<Option<License>>;
    async fn find_by_user_id(&self, user_id: &UserId) -> AppResult<Vec<License>>;
    async fn save(&self, license: &License) -> AppResult<()>;
    async fn delete(&self, id: &LicenseId) -> AppResult<()>;
}

#[async_trait]
pub trait BusinessRepository {
    async fn find_by_id(&self, id: &BusinessId) -> AppResult<Option<Business>>;
    async fn find_by_owner_id(&self, owner_id: &UserId) -> AppResult<Vec<Business>>;
    async fn save(&self, business: &Business) -> AppResult<()>;
    async fn delete(&self, id: &BusinessId) -> AppResult<()>;
}

#[async_trait]
pub trait CompanyRepository {
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
