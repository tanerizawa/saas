use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    domain::finance::{Transaction, TransactionId, TransactionRepository},
    shared::errors::AppError,
};

#[async_trait]
impl<T: TransactionRepository + Send + Sync + 'static> TransactionRepository for Arc<T> {
    async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, AppError> {
        self.as_ref().find_by_id(id).await
    }

    async fn list_by_company(
        &self,
        company_id: uuid::Uuid,
        limit: i64,
        offset: i64,
        filters: Option<HashMap<String, String>>,
    ) -> Result<Vec<Transaction>, AppError> {
        self.as_ref()
            .list_by_company(company_id, limit, offset, filters)
            .await
    }

    async fn count_by_company(
        &self,
        company_id: uuid::Uuid,
        filters: Option<HashMap<String, String>>,
    ) -> Result<i64, AppError> {
        self.as_ref().count_by_company(company_id, filters).await
    }

    async fn create(&self, transaction: &Transaction) -> Result<Transaction, AppError> {
        self.as_ref().create(transaction).await
    }

    async fn update(&self, transaction: &Transaction) -> Result<Transaction, AppError> {
        self.as_ref().update(transaction).await
    }
}
