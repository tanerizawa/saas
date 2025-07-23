use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::finance::{FinancialAccount, FinancialAccountRepository},
    shared::errors::AppError,
};

#[async_trait]
impl<A: FinancialAccountRepository + Send + Sync + 'static> FinancialAccountRepository for Arc<A> {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<FinancialAccount>, AppError> {
        self.as_ref().find_by_id(id).await
    }

    async fn list_by_company(&self, company_id: Uuid) -> Result<Vec<FinancialAccount>, AppError> {
        self.as_ref().list_by_company(company_id).await
    }

    async fn create(&self, account: &FinancialAccount) -> Result<FinancialAccount, AppError> {
        self.as_ref().create(account).await
    }

    async fn update(&self, account: &FinancialAccount) -> Result<FinancialAccount, AppError> {
        self.as_ref().update(account).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.as_ref().delete(id).await
    }
}
