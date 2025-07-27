use async_trait::async_trait;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::finance_simplified::{Transaction, Account, TransactionType};
use crate::infrastructure::cache::CacheService;
use std::sync::Arc;
use tracing::{error, info};

/// Repository for financial operations
#[async_trait]
pub trait FinancialRepository: Send + Sync {
    /// Creates a new transaction
    async fn create_transaction(&self, transaction: Transaction) -> Result<Uuid, sqlx::Error>;
    
    /// Gets a transaction by ID
    async fn get_transaction_by_id(&self, id: Uuid) -> Result<Option<Transaction>, sqlx::Error>;
    
    /// Gets transactions by user ID with pagination
    async fn get_transactions_by_user(
        &self,
        user_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<Transaction>, i64), sqlx::Error>;
    
    /// Creates a new account
    async fn create_account(&self, account: Account) -> Result<Uuid, sqlx::Error>;
    
    /// Gets an account by ID
    async fn get_account_by_id(&self, id: Uuid) -> Result<Option<Account>, sqlx::Error>;
    
    /// Gets accounts by user ID
    async fn get_accounts_by_user(&self, user_id: Uuid) -> Result<Vec<Account>, sqlx::Error>;
    
    /// Updates an account balance
    async fn update_account_balance(
        &self,
        account_id: Uuid,
        amount: f64,
        transaction_type: TransactionType,
    ) -> Result<(), sqlx::Error>;
}

/// PostgreSQL implementation of FinancialRepository
pub struct PostgresFinancialRepository {
    pool: Pool<Postgres>,
    cache: Option<Arc<CacheService>>,
}

impl PostgresFinancialRepository {
    /// Creates a new PostgresFinancialRepository without cache
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            cache: None,
        }
    }
    
    /// Creates a new PostgresFinancialRepository with cache
    pub fn new_with_cache(pool: Pool<Postgres>, cache: Arc<CacheService>) -> Self {
        Self {
            pool,
            cache: Some(cache),
        }
    }
    
    /// Get transaction cache key
    fn transaction_cache_key(&self, id: Uuid) -> String {
        format!("transaction:{}", id)
    }
    
    /// Get account cache key
    fn account_cache_key(&self, id: Uuid) -> String {
        format!("account:{}", id)
    }
    
    /// Get user transactions cache key
    fn user_transactions_cache_key(&self, user_id: Uuid, page: u32, limit: u32) -> String {
        format!("user:{}:transactions:{}:{}", user_id, page, limit)
    }
    
    /// Get user accounts cache key
    fn user_accounts_cache_key(&self, user_id: Uuid) -> String {
        format!("user:{}:accounts", user_id)
    }
    
    /// Invalidate user transactions cache
    async fn invalidate_user_transactions_cache(&self, user_id: Uuid) {
        if let Some(cache) = &self.cache {
            let _ = cache.delete(&format!("user:{}:transactions:*", user_id)).await;
        }
    }
    
    /// Invalidate user accounts cache
    async fn invalidate_user_accounts_cache(&self, user_id: Uuid) {
        if let Some(cache) = &self.cache {
            let _ = cache.delete(&self.user_accounts_cache_key(user_id)).await;
        }
    }
}

#[async_trait]
impl FinancialRepository for PostgresFinancialRepository {
    async fn create_transaction(&self, transaction: Transaction) -> Result<Uuid, sqlx::Error> {
        let transaction_id = sqlx::query!(
            r#"
            INSERT INTO transactions (
                id, user_id, account_id, amount, transaction_type, 
                description, category, created_at, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
            transaction.id,
            transaction.user_id,
            transaction.account_id,
            transaction.amount,
            transaction.transaction_type.to_string(),
            transaction.description,
            transaction.category,
            transaction.created_at,
            transaction.status.to_string()
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        // Update account balance
        self.update_account_balance(
            transaction.account_id,
            transaction.amount,
            transaction.transaction_type,
        ).await?;
        
        // Invalidate cache
        self.invalidate_user_transactions_cache(transaction.user_id).await;
        
        Ok(transaction_id)
    }
    
    async fn get_transaction_by_id(&self, id: Uuid) -> Result<Option<Transaction>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            if let Some(transaction) = cache
                .get::<Transaction>(&self.transaction_cache_key(id))
                .await
                .ok()
                .flatten()
            {
                return Ok(Some(transaction));
            }
        }
        
        // Query database
        let maybe_transaction = sqlx::query!(
            r#"
            SELECT 
                id, user_id, account_id, amount, transaction_type, 
                description, category, created_at, status
            FROM transactions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|row| Transaction {
            id: row.id,
            user_id: row.user_id,
            account_id: row.account_id,
            amount: row.amount,
            transaction_type: TransactionType::from_str(&row.transaction_type),
            description: row.description,
            category: row.category,
            created_at: row.created_at,
            status: row.status.parse().unwrap_or_default(),
        });
        
        // Store in cache if found
        if let (Some(cache), Some(transaction)) = (&self.cache, &maybe_transaction) {
            let _ = cache
                .set(&self.transaction_cache_key(id), transaction, Some(300))
                .await;
        }
        
        Ok(maybe_transaction)
    }
    
    async fn get_transactions_by_user(
        &self,
        user_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<Transaction>, i64), sqlx::Error> {
        let cache_key = self.user_transactions_cache_key(user_id, page, limit);
        
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            if let Some((transactions, total)) = cache
                .get::<(Vec<Transaction>, i64)>(&cache_key)
                .await
                .ok()
                .flatten()
            {
                return Ok((transactions, total));
            }
        }
        
        // Calculate offset
        let offset = (page - 1) * limit;
        
        // Get total count
        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM transactions
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);
        
        // Get transactions
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, user_id, account_id, amount, transaction_type, 
                description, category, created_at, status
            FROM transactions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Map rows to transactions
        let transactions = rows
            .into_iter()
            .map(|row| Transaction {
                id: row.id,
                user_id: row.user_id,
                account_id: row.account_id,
                amount: row.amount,
                transaction_type: TransactionType::from_str(&row.transaction_type),
                description: row.description,
                category: row.category,
                created_at: row.created_at,
                status: row.status.parse().unwrap_or_default(),
            })
            .collect();
        
        // Store in cache
        if let Some(cache) = &self.cache {
            let _ = cache
                .set(&cache_key, &(transactions.clone(), total), Some(60))
                .await;
        }
        
        Ok((transactions, total))
    }
    
    async fn create_account(&self, account: Account) -> Result<Uuid, sqlx::Error> {
        let account_id = sqlx::query!(
            r#"
            INSERT INTO accounts (
                id, user_id, name, account_type, balance, 
                currency, created_at, updated_at, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
            account.id,
            account.user_id,
            account.name,
            account.account_type.to_string(),
            account.balance,
            account.currency,
            account.created_at,
            account.updated_at,
            account.status.to_string()
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        // Invalidate cache
        self.invalidate_user_accounts_cache(account.user_id).await;
        
        Ok(account_id)
    }
    
    async fn get_account_by_id(&self, id: Uuid) -> Result<Option<Account>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            if let Some(account) = cache
                .get::<Account>(&self.account_cache_key(id))
                .await
                .ok()
                .flatten()
            {
                return Ok(Some(account));
            }
        }
        
        // Query database
        let maybe_account = sqlx::query!(
            r#"
            SELECT 
                id, user_id, name, account_type, balance, 
                currency, created_at, updated_at, status
            FROM accounts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(|row| Account {
            id: row.id,
            user_id: row.user_id,
            name: row.name,
            account_type: row.account_type.parse().unwrap_or_default(),
            balance: row.balance,
            currency: row.currency,
            created_at: row.created_at,
            updated_at: row.updated_at,
            status: row.status.parse().unwrap_or_default(),
        });
        
        // Store in cache if found
        if let (Some(cache), Some(account)) = (&self.cache, &maybe_account) {
            let _ = cache
                .set(&self.account_cache_key(id), account, Some(300))
                .await;
        }
        
        Ok(maybe_account)
    }
    
    async fn get_accounts_by_user(&self, user_id: Uuid) -> Result<Vec<Account>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            if let Some(accounts) = cache
                .get::<Vec<Account>>(&self.user_accounts_cache_key(user_id))
                .await
                .ok()
                .flatten()
            {
                return Ok(accounts);
            }
        }
        
        // Query database
        let rows = sqlx::query!(
            r#"
            SELECT 
                id, user_id, name, account_type, balance, 
                currency, created_at, updated_at, status
            FROM accounts
            WHERE user_id = $1
            ORDER BY name ASC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        // Map rows to accounts
        let accounts = rows
            .into_iter()
            .map(|row| Account {
                id: row.id,
                user_id: row.user_id,
                name: row.name,
                account_type: row.account_type.parse().unwrap_or_default(),
                balance: row.balance,
                currency: row.currency,
                created_at: row.created_at,
                updated_at: row.updated_at,
                status: row.status.parse().unwrap_or_default(),
            })
            .collect();
        
        // Store in cache
        if let Some(cache) = &self.cache {
            let _ = cache
                .set(&self.user_accounts_cache_key(user_id), &accounts, Some(120))
                .await;
        }
        
        Ok(accounts)
    }
    
    async fn update_account_balance(
        &self,
        account_id: Uuid,
        amount: f64,
        transaction_type: TransactionType,
    ) -> Result<(), sqlx::Error> {
        // Calculate balance change based on transaction type
        let balance_change = match transaction_type {
            TransactionType::Deposit => amount,
            TransactionType::Withdrawal => -amount,
            TransactionType::Transfer => 0.0, // Transfers handled separately
        };
        
        // Update account balance
        sqlx::query!(
            r#"
            UPDATE accounts
            SET balance = balance + $1, updated_at = $2
            WHERE id = $3
            "#,
            balance_change,
            Utc::now(),
            account_id
        )
        .execute(&self.pool)
        .await?;
        
        // Invalidate account cache
        if let Some(cache) = &self.cache {
            let _ = cache.delete(&self.account_cache_key(account_id)).await;
            
            // Also get account to invalidate user accounts cache
            if let Ok(Some(account)) = self.get_account_by_id(account_id).await {
                self.invalidate_user_accounts_cache(account.user_id).await;
            }
        }
        
        Ok(())
    }
}

/// Mock implementation for testing
#[cfg(test)]
pub struct MockFinancialRepository {
    transactions: Vec<Transaction>,
    accounts: Vec<Account>,
}

#[cfg(test)]
impl MockFinancialRepository {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new(),
            accounts: Vec::new(),
        }
    }
}

#[cfg(test)]
#[async_trait]
impl FinancialRepository for MockFinancialRepository {
    async fn create_transaction(&self, transaction: Transaction) -> Result<Uuid, sqlx::Error> {
        Ok(transaction.id)
    }
    
    async fn get_transaction_by_id(&self, id: Uuid) -> Result<Option<Transaction>, sqlx::Error> {
        Ok(self.transactions.iter().find(|t| t.id == id).cloned())
    }
    
    async fn get_transactions_by_user(
        &self,
        user_id: Uuid,
        page: u32,
        limit: u32,
    ) -> Result<(Vec<Transaction>, i64), sqlx::Error> {
        let transactions = self.transactions.iter()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect::<Vec<_>>();
        
        let total = transactions.len() as i64;
        let start = ((page - 1) * limit) as usize;
        let end = (start + limit as usize).min(transactions.len());
        
        Ok((transactions[start..end].to_vec(), total))
    }
    
    async fn create_account(&self, account: Account) -> Result<Uuid, sqlx::Error> {
        Ok(account.id)
    }
    
    async fn get_account_by_id(&self, id: Uuid) -> Result<Option<Account>, sqlx::Error> {
        Ok(self.accounts.iter().find(|a| a.id == id).cloned())
    }
    
    async fn get_accounts_by_user(&self, user_id: Uuid) -> Result<Vec<Account>, sqlx::Error> {
        Ok(self.accounts.iter()
            .filter(|a| a.user_id == user_id)
            .cloned()
            .collect())
    }
    
    async fn update_account_balance(
        &self,
        _account_id: Uuid,
        _amount: f64,
        _transaction_type: TransactionType,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }
}
