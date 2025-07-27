use crate::shared::errors::AppError;
use std::fmt::{Display, Formatter};

/// Domain-specific errors for user operations
#[derive(Debug)]
pub enum UserError {
    NotFound(String),
    ValidationFailed(String),
    DuplicateEmail(String),
    AuthenticationFailed,
    Unauthorized(String),
    InactiveAccount,
    ServiceUnavailable(String),
}

impl Display for UserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::NotFound(msg) => write!(f, "User not found: {}", msg),
            UserError::ValidationFailed(msg) => write!(f, "User validation failed: {}", msg),
            UserError::DuplicateEmail(email) => write!(f, "Email already in use: {}", email),
            UserError::AuthenticationFailed => write!(f, "Authentication failed"),
            UserError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            UserError::InactiveAccount => write!(f, "Account is inactive"),
            UserError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
        }
    }
}

/// Domain-specific errors for company operations
#[derive(Debug)]
pub enum CompanyError {
    NotFound(String),
    ValidationFailed(String),
    DuplicateName(String),
    InsufficientPermissions(String),
    InvalidOperation(String),
}

impl Display for CompanyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanyError::NotFound(msg) => write!(f, "Company not found: {}", msg),
            CompanyError::ValidationFailed(msg) => write!(f, "Company validation failed: {}", msg),
            CompanyError::DuplicateName(name) => write!(f, "Company name already in use: {}", name),
            CompanyError::InsufficientPermissions(msg) => write!(f, "Insufficient permissions: {}", msg),
            CompanyError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
        }
    }
}

/// Domain-specific errors for financial operations
#[derive(Debug)]
pub enum FinancialError {
    InsufficientFunds(String),
    InvalidAmount(String),
    TransactionFailed(String),
    NotFound(String),
    ValidationFailed(String),
    AccessDenied(String),
}

impl Display for FinancialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FinancialError::InsufficientFunds(msg) => write!(f, "Insufficient funds: {}", msg),
            FinancialError::InvalidAmount(msg) => write!(f, "Invalid amount: {}", msg),
            FinancialError::TransactionFailed(msg) => write!(f, "Transaction failed: {}", msg),
            FinancialError::NotFound(msg) => write!(f, "Financial record not found: {}", msg),
            FinancialError::ValidationFailed(msg) => write!(f, "Financial validation failed: {}", msg),
            FinancialError::AccessDenied(msg) => write!(f, "Access denied: {}", msg),
        }
    }
}

// Convert domain errors to AppError
impl From<UserError> for AppError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::NotFound(_) => AppError::NotFound(error.to_string()),
            UserError::ValidationFailed(_) => AppError::Validation(error.to_string()),
            UserError::DuplicateEmail(_) => AppError::Conflict(error.to_string()),
            UserError::AuthenticationFailed => AppError::Authentication(error.to_string()),
            UserError::Unauthorized(_) => AppError::Authorization(error.to_string()),
            UserError::InactiveAccount => AppError::Authorization(error.to_string()),
            UserError::ServiceUnavailable(_) => AppError::Internal(error.to_string()),
        }
    }
}

impl From<CompanyError> for AppError {
    fn from(error: CompanyError) -> Self {
        match error {
            CompanyError::NotFound(_) => AppError::NotFound(error.to_string()),
            CompanyError::ValidationFailed(_) => AppError::Validation(error.to_string()),
            CompanyError::DuplicateName(_) => AppError::Conflict(error.to_string()),
            CompanyError::InsufficientPermissions(_) => AppError::Authorization(error.to_string()),
            CompanyError::InvalidOperation(_) => AppError::BadRequest(error.to_string()),
        }
    }
}

impl From<FinancialError> for AppError {
    fn from(error: FinancialError) -> Self {
        match error {
            FinancialError::InsufficientFunds(_) => AppError::BadRequest(error.to_string()),
            FinancialError::InvalidAmount(_) => AppError::Validation(error.to_string()),
            FinancialError::TransactionFailed(_) => AppError::Internal(error.to_string()),
            FinancialError::NotFound(_) => AppError::NotFound(error.to_string()),
            FinancialError::ValidationFailed(_) => AppError::Validation(error.to_string()),
            FinancialError::AccessDenied(_) => AppError::Authorization(error.to_string()),
        }
    }
}
