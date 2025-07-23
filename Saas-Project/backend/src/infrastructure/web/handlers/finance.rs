// Financial management handlers - Enhanced for Phase 6
#![allow(dead_code)]

use axum::{
    extract::{Json, Path, State},
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    domain::{
        finance::{
            AccountType, FinancialAccount, FinancialAccountRepository, FinancialService,
            Transaction, TransactionId, TransactionRepository, TransactionStatus, TransactionType,
        },
        value_objects::{Currency, Money},
    },
    infrastructure::{cache::CacheService, web::middleware::auth::AuthenticatedUser},
    shared::errors::AppError,
};

// --------------------
// Request/Response DTOs
// --------------------

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub transaction_date: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub currency: String, // We parse this into Currency in the handler
    pub description: String,
    pub reference_number: Option<String>,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub transaction_date: DateTime<Utc>,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub currency: String, // We keep this as String for the API response
    pub description: String,
    pub reference_number: Option<String>,
    pub status: TransactionStatus,
    pub account_id: Uuid,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        Self {
            id: tx.id.value(),
            transaction_date: tx.transaction_date,
            transaction_type: tx.transaction_type,
            amount: tx.amount.to_f64(),
            currency: tx.amount.currency.to_string(),
            description: tx.description,
            reference_number: tx.reference_number,
            status: tx.status,
            account_id: tx.account_id,
            category_id: tx.category_id,
            tags: tx.tags,
            created_at: tx.created_at,
            updated_at: tx.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub account_type: AccountType,
    pub currency: String,
    pub initial_balance: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub name: String,
    pub account_type: String,
    pub currency: String,
    pub balance: f64,
    pub is_active: bool,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<FinancialAccount> for AccountResponse {
    fn from(account: FinancialAccount) -> Self {
        Self {
            id: account.id,
            name: account.name,
            account_type: account.account_type,
            currency: account.currency.to_string(),
            balance: account.balance.to_f64(),
            is_active: account.is_active,
            description: account.description,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<String>,
    pub transaction_type: Option<String>,
    pub account_id: Option<String>,
    pub min_amount: Option<String>,
    pub max_amount: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<TransactionResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

// --------------------
// Handler Functions
// --------------------

pub async fn create_transaction<T, A>(
    State(state): State<AppState<T, A>>,
    auth_user: AuthenticatedUser,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    // Convert request to domain entity
    // Convert currency string to Currency enum
    let currency = match req.currency.to_uppercase().as_str() {
        "IDR" => Currency::IDR,
        _ => return Err(AppError::Validation("Unsupported currency".to_string())),
    };
    let money = Money::from_f64(req.amount, currency);

    let mut transaction = Transaction::new(
        auth_user.company_id,
        req.transaction_date,
        req.transaction_type,
        money,
        req.description,
        req.account_id,
        auth_user.user_id.0,
    );

    // Set optional fields
    transaction.reference_number = req.reference_number;
    transaction.category_id = req.category_id;
    transaction.tags = req.tags.unwrap_or_default();

    // Execute the transaction
    let service = FinancialService::new(
        state.transaction_repository.clone(),
        state.account_repository.clone(),
    );

    let result = service.execute_transaction(&mut transaction).await?;

    // Cache the result
    let cache_key = format!("transaction:{}", result.id.value());
    if let Some(ref cache) = state.cache {
        let _ = cache.set(&cache_key, &result, Some(300)).await;
    }

    Ok(Json(result.into()))
}

pub async fn get_transaction<T, A>(
    State(state): State<AppState<T, A>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    let transaction_id = TransactionId(id);

    // Check cache first
    let cache_key = format!("transaction:{}", id);
    if let Some(ref cache) = state.cache {
        if let Ok(Some(transaction)) = cache.get::<Transaction>(&cache_key).await {
            return Ok(Json(transaction.into()));
        }
    }

    // If not in cache, get from database
    let result = state
        .transaction_repository
        .find_by_id(&transaction_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Transaction not found".to_string()))?;

    // Cache the result
    if let Some(ref cache) = state.cache {
        let _ = cache.set(&cache_key, &result, Some(300)).await;
    }

    Ok(Json(result.into()))
}

pub async fn create_account<T, A>(
    State(state): State<AppState<T, A>>,
    auth_user: AuthenticatedUser,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    // Convert currency string to Currency enum
    let currency = match req.currency.to_uppercase().as_str() {
        "IDR" => Currency::IDR,
        _ => return Err(AppError::Validation("Unsupported currency".to_string())),
    };
    let money = Money::from_f64(req.initial_balance, currency.clone());

    let mut account = FinancialAccount::new(
        auth_user.company_id,
        req.name,
        req.account_type.to_string(),
        currency,
        money,
    );

    account.description = req.description;

    let result = state.account_repository.create(&account).await?;

    Ok(Json(result.into()))
}

pub async fn list_accounts<T, A>(
    State(state): State<AppState<T, A>>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<AccountResponse>>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    // Cache key based on company
    let cache_key = format!("accounts:company:{}", auth_user.company_id);

    // Try to get from cache
    if let Some(ref cache) = state.cache {
        if let Ok(Some(accounts)) = cache.get::<Vec<FinancialAccount>>(&cache_key).await {
            return Ok(Json(accounts.into_iter().map(|a| a.into()).collect()));
        }
    }

    // Get from database
    let accounts = state
        .account_repository
        .list_by_company(auth_user.company_id)
        .await?;

    // Cache the result
    if let Some(ref cache) = state.cache {
        let _ = cache.set(&cache_key, &accounts, Some(300)).await;
    }

    Ok(Json(accounts.into_iter().map(|a| a.into()).collect()))
}

// --------------------
// Router & App State
// --------------------

#[derive(Clone)]
pub struct AppState<T, A>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    transaction_repository: Arc<T>,
    account_repository: Arc<A>,
    cache: Option<Arc<CacheService>>,
}

pub fn routes() -> Router {
    // Placeholder until we set up the repositories
    Router::new()
        .route("/transactions", post(placeholder_transaction))
        .route("/transactions/:id", get(placeholder_transaction_detail))
        .route("/accounts", post(placeholder_account))
        .route("/accounts", get(placeholder_accounts))
}

// Temporary placeholder handlers until we set up the repositories
async fn placeholder_transaction() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Create transaction endpoint - coming soon in Phase 6",
        "status": "in_progress"
    }))
}

async fn placeholder_transaction_detail(Path(_id): Path<Uuid>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Get transaction endpoint - coming soon in Phase 6",
        "status": "in_progress"
    }))
}

async fn placeholder_account() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Create account endpoint - coming soon in Phase 6",
        "status": "in_progress"
    }))
}

async fn placeholder_accounts() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "List accounts endpoint - coming soon in Phase 6",
        "status": "in_progress"
    }))
}

// Helper function to create the fully functional router with repositories
pub fn create_finance_router<T, A>(
    transaction_repository: T,
    account_repository: A,
    cache: Option<CacheService>,
) -> Router
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    let state = AppState {
        transaction_repository: Arc::new(transaction_repository),
        account_repository: Arc::new(account_repository),
        cache: cache.map(Arc::new),
    };

    Router::new()
        .route("/transactions", post(handler_create_transaction::<T, A>))
        .route("/transactions/:id", get(handler_get_transaction::<T, A>))
        .route("/accounts", post(handler_create_account::<T, A>))
        .route("/accounts", get(handler_list_accounts::<T, A>))
        .route("/transactions", get(|| async { "List transactions" }))
        .route("/reports", get(|| async { "Financial reports" }))
        .route("/tax", get(|| async { "Tax management" }))
        .route("/payments", post(|| async { "Process payment" }))
        .with_state(state)
}

pub async fn placeholder() -> &'static str {
    "Finance API endpoints - Under development"
}

// Wrapper handler functions for Axum routing
async fn handler_create_transaction<T, A>(
    State(state): State<AppState<T, A>>,
    auth_user: AuthenticatedUser,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    create_transaction(State(state), auth_user, Json(req)).await
}

async fn handler_get_transaction<T, A>(
    State(state): State<AppState<T, A>>,
    Path(id): Path<Uuid>,
) -> Result<Json<TransactionResponse>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    get_transaction(State(state), Path(id)).await
}

async fn handler_create_account<T, A>(
    State(state): State<AppState<T, A>>,
    auth_user: AuthenticatedUser,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    create_account(State(state), auth_user, Json(req)).await
}

async fn handler_list_accounts<T, A>(
    State(state): State<AppState<T, A>>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<AccountResponse>>, AppError>
where
    T: TransactionRepository + Send + Sync + Clone + 'static,
    A: FinancialAccountRepository + Send + Sync + Clone + 'static,
{
    list_accounts(State(state), auth_user).await
}
