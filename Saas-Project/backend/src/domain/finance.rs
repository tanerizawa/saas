// Finance domain module - Enhanced for Phase 6
// Contains comprehensive financial models and operations

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use sqlx::types::BigDecimal;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::value_objects::{Currency, Money};
use crate::shared::errors::AppError;

// ----------------
// Value Objects
// ----------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    Income,
    Expense,
    Transfer,
    Investment,
    Loan,
    LoanRepayment,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Draft,
    Pending,
    Completed,
    Failed,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Cash,
    Bank,
    CreditCard,
    Receivable,
    Payable,
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Cash => write!(f, "Cash"),
            AccountType::Bank => write!(f, "Bank"),
            AccountType::CreditCard => write!(f, "CreditCard"),
            AccountType::Receivable => write!(f, "Receivable"),
            AccountType::Payable => write!(f, "Payable"),
            AccountType::Asset => write!(f, "Asset"),
            AccountType::Liability => write!(f, "Liability"),
            AccountType::Equity => write!(f, "Equity"),
            AccountType::Revenue => write!(f, "Revenue"),
            AccountType::Expense => write!(f, "Expense"),
        }
    }
}

// ----------------
// Entities
// ----------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: TransactionId,
    pub company_id: Uuid,
    pub transaction_date: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub amount: Money,
    pub description: String,
    pub reference_number: Option<String>,
    pub status: TransactionStatus,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub attachments: Vec<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Option<Uuid>,
}

impl Transaction {
    pub fn new(
        company_id: Uuid,
        transaction_date: DateTime<Utc>,
        transaction_type: TransactionType,
        amount: Money,
        description: String,
        account_id: Uuid,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: TransactionId::new(),
            company_id,
            transaction_date,
            transaction_type,
            amount,
            description,
            reference_number: None,
            status: TransactionStatus::Draft,
            account_id,
            category_id: None,
            tags: Vec::new(),
            attachments: Vec::new(),
            metadata: None,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: None,
        }
    }

    pub fn complete(&mut self) -> Result<(), AppError> {
        match self.status {
            TransactionStatus::Draft | TransactionStatus::Pending => {
                self.status = TransactionStatus::Completed;
                self.updated_at = Utc::now();
                Ok(())
            }
            TransactionStatus::Completed => Ok(()),
            _ => Err(AppError::Validation(
                "Transaction cannot be completed from its current status".to_string(),
            )),
        }
    }

    pub fn cancel(&mut self) -> Result<(), AppError> {
        match self.status {
            TransactionStatus::Draft | TransactionStatus::Pending => {
                self.status = TransactionStatus::Cancelled;
                self.updated_at = Utc::now();
                Ok(())
            }
            TransactionStatus::Completed => Err(AppError::Validation(
                "Completed transactions cannot be cancelled directly, create a refund instead"
                    .to_string(),
            )),
            _ => Err(AppError::Validation(
                "Transaction cannot be cancelled from its current status".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialAccount {
    pub id: Uuid,
    pub company_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub account_type: String,
    pub currency: Currency,
    pub balance: Money,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FinancialAccount {
    pub fn new(
        company_id: Uuid,
        name: String,
        account_type: String,
        currency: Currency,
        initial_balance: Money,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            company_id,
            name,
            account_type,
            currency,
            balance: initial_balance,
            is_active: true,
            description: None,
            metadata: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    pub fn update_balance(&mut self, amount: Money) -> Result<(), AppError> {
        if self.currency != amount.currency {
            return Err(AppError::Validation(format!(
                "Currency mismatch. Account currency: {}, transaction currency: {}",
                self.currency, amount.currency
            )));
        }

        self.balance += amount;
        self.updated_at = Utc::now();
        Ok(())
    }
}

// ----------------
// Repository Interfaces
// ----------------

#[async_trait::async_trait]
pub trait TransactionRepository: Send + Sync {
    async fn create(&self, transaction: &Transaction) -> Result<Transaction, AppError>;
    async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>, AppError>;
    async fn update(&self, transaction: &Transaction) -> Result<Transaction, AppError>;
    async fn list_by_company(
        &self,
        company_id: Uuid,
        limit: i64,
        offset: i64,
        filters: Option<HashMap<String, String>>,
    ) -> Result<Vec<Transaction>, AppError>;
    async fn count_by_company(
        &self,
        company_id: Uuid,
        filters: Option<HashMap<String, String>>,
    ) -> Result<i64, AppError>;
}

#[async_trait::async_trait]
pub trait FinancialAccountRepository: Send + Sync {
    async fn create(&self, account: &FinancialAccount) -> Result<FinancialAccount, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<FinancialAccount>, AppError>;
    async fn update(&self, account: &FinancialAccount) -> Result<FinancialAccount, AppError>;
    async fn list_by_company(&self, company_id: Uuid) -> Result<Vec<FinancialAccount>, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}

// ----------------
// Domain Services
// ----------------

pub struct FinancialService<T: TransactionRepository, A: FinancialAccountRepository> {
    transaction_repository: T,
    account_repository: A,
}

impl<T: TransactionRepository, A: FinancialAccountRepository> FinancialService<T, A> {
    pub fn new(transaction_repository: T, account_repository: A) -> Self {
        Self {
            transaction_repository,
            account_repository,
        }
    }

    pub async fn execute_transaction(
        &self,
        transaction: &mut Transaction,
    ) -> Result<Transaction, AppError> {
        // 1. Validate the transaction
        let account = self
            .account_repository
            .find_by_id(transaction.account_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;

        if !account.is_active {
            return Err(AppError::Validation("Account is not active".to_string()));
        }

        // 2. Update the account balance based on transaction type
        let mut account_to_update = account.clone();

        let amount_adjustment = match transaction.transaction_type {
            TransactionType::Income => transaction.amount.clone(),
            TransactionType::Expense => -transaction.amount.clone(),
            TransactionType::Transfer => transaction.amount.clone(), // This would be more complex in reality
            _ => transaction.amount.clone(),
        };

        account_to_update.update_balance(amount_adjustment)?;

        // 3. Save the updated account
        let _updated_account = self.account_repository.update(&account_to_update).await?;

        // 4. Complete and save the transaction
        transaction.complete()?;
        let saved_transaction = self.transaction_repository.update(transaction).await?;

        Ok(saved_transaction)
    }

    // Generate financial reports based on transactions
    pub async fn generate_income_statement(
        &self,
        company_id: Uuid,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<IncomeStatement, AppError> {
        // Implementation would fetch and aggregate transactions
        // This is a simplified placeholder

        Ok(IncomeStatement {
            company_id,
            period_start: start_date,
            period_end: end_date,
            revenue: Money::new(0, Currency::IDR),
            expenses: Money::new(0, Currency::IDR),
            net_income: Money::new(0, Currency::IDR),
            categories: HashMap::new(),
        })
    }
}

// ----------------
// Report Types
// ----------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomeStatement {
    pub company_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub revenue: Money,
    pub expenses: Money,
    pub net_income: Money,
    pub categories: HashMap<String, Money>,
}
