// Utility functions
#![allow(dead_code)]

use chrono::{DateTime, Utc};

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

pub fn format_timestamp(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

// Password validation
pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password must be at least 8 characters long".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password must contain at least one uppercase letter".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err("Password must contain at least one lowercase letter".to_string());
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password must contain at least one digit".to_string());
    }

    Ok(())
}

// Email validation
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.len() > 3 && email.len() < 255
}
