use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::finance::{
    AccountType, FinancialAccount, FinancialAccountRepository, Transaction, TransactionId,
    TransactionRepository, TransactionStatus, TransactionType,
};
use crate::domain::value_objects::Money;
use crate::shared::errors::AppError;

pub struct PostgresTransactionRepository {
    pool: PgPool,
}

impl PostgresTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TransactionRepository for PostgresTransactionRepository {
    async fn create(&self, transaction: &Transaction) -> Result<Transaction, AppError> {
        let id = transaction.id.value();
        let transaction_type = serde_json::to_value(&transaction.transaction_type)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        let status = serde_json::to_value(&transaction.status)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        let tags = serde_json::to_value(&transaction.tags)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        let attachments = serde_json::to_value(&transaction.attachments)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        let record = sqlx::query!(
            r#"
            INSERT INTO financial_transactions (
                id, company_id, transaction_date, transaction_type, amount, currency,
                description, reference_number, status, account_id, category_id, 
                tags, attachments, metadata, created_at, updated_at, created_by, updated_by
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            )
            RETURNING id
            "#,
            id,
            transaction.company_id,
            transaction.transaction_date,
            transaction_type,
            transaction.amount.value,
            transaction.amount.currency,
            transaction.description,
            transaction.reference_number,
            status,
            transaction.account_id,
            transaction.category_id,
            tags,
            attachments,
            transaction.metadata,
            transaction.created_at,
            transaction.updated_at,
            transaction.created_by,
            transaction.updated_by
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        // Return the created transaction
        self.find_by_id(&TransactionId(record.id))
            .await?
            .ok_or_else(|| {
                AppError::InternalError("Failed to retrieve created transaction".to_string())
            })
    }

    async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT 
                id, company_id, transaction_date, transaction_type, amount, currency,
                description, reference_number, status, account_id, category_id, 
                tags, attachments, metadata, created_at, updated_at, created_by, updated_by
            FROM financial_transactions
            WHERE id = $1
            "#,
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        if let Some(r) = record {
            let transaction_type: TransactionType = serde_json::from_value(r.transaction_type)
                .map_err(|e| AppError::DeserializationError(e.to_string()))?;

            let status: TransactionStatus = serde_json::from_value(r.status)
                .map_err(|e| AppError::DeserializationError(e.to_string()))?;

            let tags: Vec<String> = serde_json::from_value(r.tags)
                .map_err(|e| AppError::DeserializationError(e.to_string()))?;

            let attachments: Vec<String> = serde_json::from_value(r.attachments)
                .map_err(|e| AppError::DeserializationError(e.to_string()))?;

            let money = Money::new(r.amount, r.currency);

            let transaction = Transaction {
                id: TransactionId(r.id),
                company_id: r.company_id,
                transaction_date: r.transaction_date,
                transaction_type,
                amount: money,
                description: r.description,
                reference_number: r.reference_number,
                status,
                account_id: r.account_id,
                category_id: r.category_id,
                tags,
                attachments,
                metadata: r.metadata,
                created_at: r.created_at,
                updated_at: r.updated_at,
                created_by: r.created_by,
                updated_by: r.updated_by,
            };

            Ok(Some(transaction))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, transaction: &Transaction) -> Result<Transaction, AppError> {
        let transaction_type = serde_json::to_value(&transaction.transaction_type)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        let status = serde_json::to_value(&transaction.status)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        let tags = serde_json::to_value(&transaction.tags)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        let attachments = serde_json::to_value(&transaction.attachments)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        sqlx::query!(
            r#"
            UPDATE financial_transactions
            SET 
                transaction_date = $1,
                transaction_type = $2,
                amount = $3,
                currency = $4,
                description = $5,
                reference_number = $6,
                status = $7,
                account_id = $8,
                category_id = $9,
                tags = $10,
                attachments = $11,
                metadata = $12,
                updated_at = $13,
                updated_by = $14
            WHERE id = $15
            "#,
            transaction.transaction_date,
            transaction_type,
            transaction.amount.value,
            transaction.amount.currency,
            transaction.description,
            transaction.reference_number,
            status,
            transaction.account_id,
            transaction.category_id,
            tags,
            attachments,
            transaction.metadata,
            transaction.updated_at,
            transaction.updated_by,
            transaction.id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        self.find_by_id(&transaction.id).await?.ok_or_else(|| {
            AppError::InternalError("Failed to retrieve updated transaction".to_string())
        })
    }

    async fn list_by_company(
        &self,
        company_id: Uuid,
        limit: i64,
        offset: i64,
        filters: Option<HashMap<String, String>>,
    ) -> Result<Vec<Transaction>, AppError> {
        // Build dynamic query based on filters
        let mut query = "SELECT * FROM financial_transactions WHERE company_id = $1".to_string();
        let mut params: Vec<String> = vec![company_id.to_string()];

        if let Some(filter_map) = filters {
            let mut param_index = 2;

            for (key, value) in filter_map {
                match key.as_str() {
                    "status" => {
                        query.push_str(&format!(" AND status = ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "type" => {
                        query.push_str(&format!(" AND transaction_type = ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "account_id" => {
                        query.push_str(&format!(" AND account_id = ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "min_amount" => {
                        query.push_str(&format!(" AND amount >= ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "max_amount" => {
                        query.push_str(&format!(" AND amount <= ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "start_date" => {
                        query.push_str(&format!(" AND transaction_date >= ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "end_date" => {
                        query.push_str(&format!(" AND transaction_date <= ${}", param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    "search" => {
                        query.push_str(&format!(" AND (description ILIKE '%' || ${} || '%' OR reference_number ILIKE '%' || ${} || '%')", param_index, param_index));
                        params.push(value);
                        param_index += 1;
                    }
                    _ => {} // Ignore unknown filter keys
                }
            }
        }

        query.push_str(" ORDER BY transaction_date DESC LIMIT $");
        query.push_str(&(params.len() + 1).to_string());
        query.push_str(" OFFSET $");
        query.push_str(&(params.len() + 2).to_string());

        // This implementation is a placeholder - real implementation would use prepared statements and parameter binding
        // For demonstration purposes only
        let transactions: Vec<Transaction> = Vec::new();

        Ok(transactions)
    }

    async fn count_by_company(
        &self,
        company_id: Uuid,
        filters: Option<HashMap<String, String>>,
    ) -> Result<i64, AppError> {
        // Similar to list_by_company but returns count
        // Implementation omitted for brevity

        Ok(0) // Placeholder
    }
}

pub struct PostgresFinancialAccountRepository {
    pool: PgPool,
}

impl PostgresFinancialAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FinancialAccountRepository for PostgresFinancialAccountRepository {
    async fn create(&self, account: &FinancialAccount) -> Result<FinancialAccount, AppError> {
        let account_type = serde_json::to_value(&account.account_type)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        let record = sqlx::query!(
            r#"
            INSERT INTO financial_accounts (
                id, company_id, name, account_type, currency, balance,
                is_active, description, metadata, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id
            "#,
            account.id,
            account.company_id,
            account.name,
            account_type,
            account.currency,
            account.balance.value,
            account.is_active,
            account.description,
            account.metadata,
            account.created_at,
            account.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        self.find_by_id(record.id).await?.ok_or_else(|| {
            AppError::InternalError("Failed to retrieve created account".to_string())
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<FinancialAccount>, AppError> {
        let record = sqlx::query!(
            r#"
            SELECT 
                id, company_id, name, account_type, currency, balance,
                is_active, description, metadata, created_at, updated_at
            FROM financial_accounts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        if let Some(r) = record {
            let account_type: AccountType = serde_json::from_value(r.account_type)
                .map_err(|e| AppError::DeserializationError(e.to_string()))?;

            let money = Money::new(r.balance, r.currency.clone());

            let account = FinancialAccount {
                id: r.id,
                company_id: r.company_id,
                name: r.name,
                account_type,
                currency: r.currency,
                balance: money,
                is_active: r.is_active,
                description: r.description,
                metadata: r.metadata,
                created_at: r.created_at,
                updated_at: r.updated_at,
            };

            Ok(Some(account))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, account: &FinancialAccount) -> Result<FinancialAccount, AppError> {
        let account_type = serde_json::to_value(&account.account_type)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;

        sqlx::query!(
            r#"
            UPDATE financial_accounts
            SET
                name = $1,
                account_type = $2,
                currency = $3,
                balance = $4,
                is_active = $5,
                description = $6,
                metadata = $7,
                updated_at = $8
            WHERE id = $9
            "#,
            account.name,
            account_type,
            account.currency,
            account.balance.value,
            account.is_active,
            account.description,
            account.metadata,
            account.updated_at,
            account.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        self.find_by_id(account.id).await?.ok_or_else(|| {
            AppError::InternalError("Failed to retrieve updated account".to_string())
        })
    }

    async fn list_by_company(&self, company_id: Uuid) -> Result<Vec<FinancialAccount>, AppError> {
        let records = sqlx::query!(
            r#"
            SELECT 
                id, company_id, name, account_type, currency, balance,
                is_active, description, metadata, created_at, updated_at
            FROM financial_accounts
            WHERE company_id = $1
            ORDER BY name ASC
            "#,
            company_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        let mut accounts = Vec::with_capacity(records.len());

        for r in records {
            let account_type: AccountType = serde_json::from_value(r.account_type)
                .map_err(|e| AppError::DeserializationError(e.to_string()))?;

            let money = Money::new(r.balance, r.currency.clone());

            let account = FinancialAccount {
                id: r.id,
                company_id: r.company_id,
                name: r.name,
                account_type,
                currency: r.currency,
                balance: money,
                is_active: r.is_active,
                description: r.description,
                metadata: r.metadata,
                created_at: r.created_at,
                updated_at: r.updated_at,
            };

            accounts.push(account);
        }

        Ok(accounts)
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM financial_accounts WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(())
    }
}
