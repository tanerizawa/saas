// Financial management handlers - Enhanced for Phase 6

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::finance_simplified::{Transaction, Account, TransactionType, AccountType, Status};
use crate::infrastructure::web::middleware::auth::AuthenticatedUser;
use crate::infrastructure::web::router::AppState;
use crate::shared::errors::AppError;

// Routes for financial management
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        // Transactions
        .route("/transactions", post(create_transaction))
        .route("/transactions/:id", get(get_transaction))
        .route("/transactions", get(list_transactions))
        // Accounts
        .route("/accounts", post(create_account))
        .route("/accounts", get(list_accounts))
        .route("/accounts/:id", get(get_account))
        // Reports
        .route("/reports/summary", get(get_financial_summary))
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    /// Account ID for the transaction
    pub account_id: Uuid,
    /// Transaction amount
    pub amount: f64,
    /// Type of transaction (deposit, withdrawal, transfer)
    pub transaction_type: String,
    /// Transaction description
    pub description: String,
    /// Optional category
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    /// Transaction ID
    pub id: Uuid,
    /// Account ID
    pub account_id: Uuid,
    /// Transaction amount
    pub amount: f64,
    /// Transaction type
    pub transaction_type: String,
    /// Transaction description
    pub description: String,
    /// Optional category
    pub category: Option<String>,
    /// Transaction creation date
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Transaction status
    pub status: String,
}

impl From<Transaction> for TransactionResponse {
    fn from(transaction: Transaction) -> Self {
        Self {
            id: transaction.id,
            account_id: transaction.account_id,
            amount: transaction.amount,
            transaction_type: transaction.transaction_type.to_string(),
            description: transaction.description,
            category: transaction.category,
            created_at: transaction.created_at,
            status: transaction.status.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<TransactionResponse>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

/// Create a transaction
pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    if req.amount <= 0.0 {
        return Err(AppError::Validation("Amount must be positive".to_string()));
    }
    
    // Convert request to domain model
    let transaction = Transaction::new(
        *auth.user_id.as_uuid(),
        req.account_id,
        req.amount,
        match req.transaction_type.to_lowercase().as_str() {
            "withdrawal" => TransactionType::Withdrawal,
            "transfer" => TransactionType::Transfer,
            _ => TransactionType::Deposit,
        },
        req.description,
        req.category,
    );
    
    // Save transaction
    let transaction_id = state.financial_repository().create_transaction(transaction).await
        .map_err(|e| AppError::Database(format!("Failed to create transaction: {}", e)))?;
    
    // Get the saved transaction
    let transaction = state.financial_repository().get_transaction_by_id(transaction_id).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve transaction: {}", e)))?
        .ok_or_else(|| AppError::Internal("Transaction not found after creation".to_string()))?;
    
    // Return response
    Ok((StatusCode::CREATED, Json(TransactionResponse::from(transaction))))
}

/// Get transaction by ID
pub async fn get_transaction(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Get transaction
    let transaction = state.financial_repository().get_transaction_by_id(id).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve transaction: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))?;
    
    // Check permission
    if transaction.user_id != *auth.user_id.as_uuid() {
        return Err(AppError::Forbidden("You don't have permission to view this transaction".to_string()));
    }
    
    // Return response
    Ok(Json(TransactionResponse::from(transaction)))
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// List transactions with pagination
pub async fn list_transactions(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Query(params): Query<PaginationParams>,
) -> Result<impl IntoResponse, AppError> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10).min(100);
    
    if page == 0 {
        return Err(AppError::Validation("Page must be greater than 0".to_string()));
    }
    
    // Get transactions
    let (transactions, total) = state.financial_repository()
        .get_transactions_by_user(*auth.user_id.as_uuid(), page, limit).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve transactions: {}", e)))?;
    
    // Calculate total pages
    let total_pages = (total as f64 / limit as f64).ceil() as u32;
    
    // Convert to response
    let transaction_responses = transactions.into_iter()
        .map(TransactionResponse::from)
        .collect();
    
    // Return response
    let response = TransactionListResponse {
        transactions: transaction_responses,
        total,
        page,
        limit,
        total_pages,
    };
    
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    /// Account name
    pub name: String,
    /// Type of account
    pub account_type: String,
    /// Initial balance
    pub initial_balance: f64,
    /// Currency code (ISO 4217)
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    /// Account ID
    pub id: Uuid,
    /// Account name
    pub name: String,
    /// Account type
    pub account_type: String,
    /// Current balance
    pub balance: f64,
    /// Currency code
    pub currency: String,
    /// Account creation date
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        Self {
            id: account.id,
            name: account.name,
            account_type: account.account_type.to_string(),
            balance: account.balance,
            currency: account.currency,
            created_at: account.created_at,
        }
    }
}

/// Create a financial account
pub async fn create_account(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Json(req): Json<CreateAccountRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate request
    if req.name.trim().is_empty() {
        return Err(AppError::Validation("Account name cannot be empty".to_string()));
    }
    
    if req.initial_balance < 0.0 {
        return Err(AppError::Validation("Initial balance cannot be negative".to_string()));
    }
    
    if req.currency.trim().is_empty() {
        return Err(AppError::Validation("Currency code cannot be empty".to_string()));
    }
    
    // Convert request to domain model
    let account = Account::new(
        *auth.user_id.as_uuid(),
        req.name,
        match req.account_type.to_lowercase().as_str() {
            "savings" => AccountType::Savings,
            "investment" => AccountType::Investment,
            "credit" => AccountType::Credit,
            "cash" => AccountType::Cash,
            _ => AccountType::Checking,
        },
        req.initial_balance,
        req.currency,
    );
    
    // Save account
    let account_id = state.financial_repository().create_account(account).await
        .map_err(|e| AppError::Database(format!("Failed to create account: {}", e)))?;
    
    // Get the saved account
    let account = state.financial_repository().get_account_by_id(account_id).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve account: {}", e)))?
        .ok_or_else(|| AppError::Internal("Account not found after creation".to_string()))?;
    
    // Return response
    Ok((StatusCode::CREATED, Json(AccountResponse::from(account))))
}

/// Get account by ID
pub async fn get_account(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Get account
    let account = state.financial_repository().get_account_by_id(id).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve account: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Account not found".to_string()))?;
    
    // Check permission
    if account.user_id != *auth.user_id.as_uuid() {
        return Err(AppError::Forbidden("You don't have permission to view this account".to_string()));
    }
    
    // Return response
    Ok(Json(AccountResponse::from(account)))
}

/// List accounts for a user
pub async fn list_accounts(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Get accounts
    let accounts = state.financial_repository().get_accounts_by_user(*auth.user_id.as_uuid()).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve accounts: {}", e)))?;
    
    // Convert to response
    let account_responses = accounts.into_iter()
        .map(AccountResponse::from)
        .collect::<Vec<_>>();
    
    // Return response
    Ok(Json(account_responses))
}

#[derive(Debug, Serialize)]
pub struct FinancialSummaryResponse {
    /// Total balance across all accounts
    pub total_balance: f64,
    /// Total income in the current month
    pub income_this_month: f64,
    /// Total expenses in the current month
    pub expenses_this_month: f64,
    /// Net change for the current month
    pub net_change_this_month: f64,
    /// Top expense categories
    pub top_expense_categories: Vec<CategorySummary>,
    /// Account summary
    pub accounts: Vec<AccountSummary>,
}

#[derive(Debug, Serialize)]
pub struct CategorySummary {
    /// Category name
    pub category: String,
    /// Total amount for this category
    pub amount: f64,
    /// Percentage of total expenses
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct AccountSummary {
    /// Account ID
    pub id: Uuid,
    /// Account name
    pub name: String,
    /// Account type
    pub account_type: String,
    /// Current balance
    pub balance: f64,
    /// Currency code
    pub currency: String,
}

/// Get financial summary for a user
pub async fn get_financial_summary(
    State(state): State<Arc<AppState>>,
    auth: AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Get accounts
    let accounts = state.financial_repository().get_accounts_by_user(*auth.user_id.as_uuid()).await
        .map_err(|e| AppError::Database(format!("Failed to retrieve accounts: {}", e)))?;
    
    // Calculate total balance
    let total_balance = accounts.iter().map(|a| a.balance).sum();
    
    // For a real implementation, we would calculate actual income and expenses
    // from transactions within the current month
    
    // Mock values for demonstration
    let income_this_month = 5000.0;
    let expenses_this_month = 3000.0;
    let net_change_this_month = income_this_month - expenses_this_month;
    
    // Mock category data for demonstration
    let top_expense_categories = vec![
        CategorySummary {
            category: "Housing".to_string(),
            amount: 1200.0,
            percentage: 40.0,
        },
        CategorySummary {
            category: "Food".to_string(),
            amount: 800.0,
            percentage: 26.7,
        },
        CategorySummary {
            category: "Transportation".to_string(),
            amount: 600.0,
            percentage: 20.0,
        },
        CategorySummary {
            category: "Entertainment".to_string(),
            amount: 400.0,
            percentage: 13.3,
        },
    ];
    
    // Convert accounts to summary
    let account_summaries = accounts.into_iter()
        .map(|a| AccountSummary {
            id: a.id,
            name: a.name,
            account_type: a.account_type.to_string(),
            balance: a.balance,
            currency: a.currency,
        })
        .collect();
    
    // Return response
    let response = FinancialSummaryResponse {
        total_balance,
        income_this_month,
        expenses_this_month,
        net_change_this_month,
        top_expense_categories,
        accounts: account_summaries,
    };
    
    Ok(Json(response))
}
