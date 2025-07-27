use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

/// Status for financial entities
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Active,
    Pending,
    Completed,
    Failed,
    Cancelled,
    Suspended,
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Status::Active),
            "pending" => Ok(Status::Pending),
            "completed" => Ok(Status::Completed),
            "failed" => Ok(Status::Failed),
            "cancelled" => Ok(Status::Cancelled),
            "suspended" => Ok(Status::Suspended),
            _ => Err(()),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Active => "active".to_string(),
            Status::Pending => "pending".to_string(),
            Status::Completed => "completed".to_string(),
            Status::Failed => "failed".to_string(),
            Status::Cancelled => "cancelled".to_string(),
            Status::Suspended => "suspended".to_string(),
        }
    }
}

/// Types of transactions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Deposit
    }
}

impl TransactionType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "withdrawal" => TransactionType::Withdrawal,
            "transfer" => TransactionType::Transfer,
            _ => TransactionType::Deposit,
        }
    }
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Deposit => "deposit".to_string(),
            TransactionType::Withdrawal => "withdrawal".to_string(),
            TransactionType::Transfer => "transfer".to_string(),
        }
    }
}

/// Types of accounts
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AccountType {
    Checking,
    Savings,
    Investment,
    Credit,
    Cash,
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Checking
    }
}

impl FromStr for AccountType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "checking" => Ok(AccountType::Checking),
            "savings" => Ok(AccountType::Savings),
            "investment" => Ok(AccountType::Investment),
            "credit" => Ok(AccountType::Credit),
            "cash" => Ok(AccountType::Cash),
            _ => Err(()),
        }
    }
}

impl ToString for AccountType {
    fn to_string(&self) -> String {
        match self {
            AccountType::Checking => "checking".to_string(),
            AccountType::Savings => "savings".to_string(),
            AccountType::Investment => "investment".to_string(),
            AccountType::Credit => "credit".to_string(),
            AccountType::Cash => "cash".to_string(),
        }
    }
}

/// Financial transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Unique identifier
    pub id: Uuid,
    /// User who owns this transaction
    pub user_id: Uuid,
    /// Account associated with this transaction
    pub account_id: Uuid,
    /// Transaction amount
    pub amount: f64,
    /// Type of transaction
    pub transaction_type: TransactionType,
    /// Transaction description
    pub description: String,
    /// Optional category for organizing transactions
    pub category: Option<String>,
    /// When the transaction was created
    pub created_at: DateTime<Utc>,
    /// Current status of the transaction
    pub status: Status,
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(
        user_id: Uuid,
        account_id: Uuid,
        amount: f64,
        transaction_type: TransactionType,
        description: String,
        category: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            account_id,
            amount,
            transaction_type,
            description,
            category,
            created_at: Utc::now(),
            status: Status::Pending,
        }
    }
}

/// Financial account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Unique identifier
    pub id: Uuid,
    /// User who owns this account
    pub user_id: Uuid,
    /// Account name
    pub name: String,
    /// Type of account
    pub account_type: AccountType,
    /// Current balance
    pub balance: f64,
    /// Currency code (ISO 4217)
    pub currency: String,
    /// When the account was created
    pub created_at: DateTime<Utc>,
    /// When the account was last updated
    pub updated_at: DateTime<Utc>,
    /// Current status of the account
    pub status: Status,
}

impl Account {
    /// Creates a new account
    pub fn new(
        user_id: Uuid,
        name: String,
        account_type: AccountType,
        initial_balance: f64,
        currency: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            account_type,
            balance: initial_balance,
            currency,
            created_at: now,
            updated_at: now,
            status: Status::Active,
        }
    }
}
