#![allow(dead_code)]

use std::sync::Arc;

use super::queries::{GetUserQuery, ListLicensesQuery};
use crate::domain::entities::User;
use crate::domain::repositories::{LicenseRepository, UserRepository};
use crate::shared::errors::AppResult;
use crate::shared::types::PaginatedResponse;

pub struct UserQueryHandler {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserQueryHandler {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }

    pub async fn handle_get_user(&self, query: GetUserQuery) -> AppResult<Option<User>> {
        let user_id = crate::domain::value_objects::UserId::from_uuid(query.user_id);
        self.user_repository.find_by_id(&user_id).await
    }

    pub async fn handle_list_users(
        &self,
        page: u32,
        limit: u32,
    ) -> AppResult<PaginatedResponse<User>> {
        let offset = (page.saturating_sub(1) * limit) as i32;

        let users = self
            .user_repository
            .list_all(Some(limit as i32), Some(offset))
            .await?;

        let total = self.user_repository.count_all().await? as u64;

        Ok(PaginatedResponse::new(users, total, page, limit))
    }

    pub async fn handle_search_users(&self, email_query: &str) -> AppResult<Vec<User>> {
        self
            .user_repository
            .search(email_query, None, None)
            .await
    }
}

pub struct LicenseQueryHandler {
    license_repository: Option<Arc<dyn LicenseRepository + Send + Sync>>,
}

impl LicenseQueryHandler {
    pub fn new() -> Self {
        Self {
            license_repository: None, // Will be injected when license repo is implemented
        }
    }

    pub fn with_repository(license_repository: Arc<dyn LicenseRepository + Send + Sync>) -> Self {
        Self {
            license_repository: Some(license_repository),
        }
    }

    pub async fn handle_list_licenses(&self, _query: ListLicensesQuery) -> AppResult<Vec<String>> {
        if let Some(repo) = &self.license_repository {
            let licenses = repo.search_licenses("", None).await?;
            Ok(licenses.into_iter().map(|l| l.title).collect())
        } else {
            Ok(vec![])
        }
    }

    pub async fn handle_get_user_licenses(&self, user_id: uuid::Uuid) -> AppResult<Vec<String>> {
        if let Some(repo) = &self.license_repository {
            let licenses = repo.get_licenses_by_user(user_id).await?;
            Ok(licenses.into_iter().map(|l| l.title).collect())
        } else {
            Ok(vec![])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use uuid::Uuid;

    #[test]
    fn test_query_handler_construction() {
        let _license_handler = LicenseQueryHandler::new();
        // User handler test will be added when repository mock is available
    }

    #[test]
    fn test_license_handler_placeholder() {
        let handler = LicenseQueryHandler::new();
        // Test that handler can be constructed without repository for now
        assert!(handler.license_repository.is_none());
    }

    #[tokio::test]
    async fn test_list_licenses_placeholder() {
        let handler = LicenseQueryHandler::new();
        let query = ListLicensesQuery::new();
        let result = handler.handle_list_licenses(query).await;

        assert!(result.is_ok());
        let licenses = result.unwrap();
        assert!(licenses.is_empty());
    }
}
