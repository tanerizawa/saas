#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

// Value Objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if Self::is_valid(&email) {
            Ok(Self(email.to_lowercase()))
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_valid(email: &str) -> bool {
        // Basic email validation
        email.contains('@') && email.contains('.') && email.len() > 5
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    UmkmOwner,  // UMKM business owner
    AdminStaff, // Administrative staff
    SuperAdmin, // System administrator
}

impl UserRole {
    /// Check if this role has permission to access resources requiring the specified role
    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        match (self, required_role) {
            // SuperAdmin can access everything
            (UserRole::SuperAdmin, _) => true,

            // AdminStaff can access UmkmOwner resources and their own
            (UserRole::AdminStaff, UserRole::UmkmOwner) => true,
            (UserRole::AdminStaff, UserRole::AdminStaff) => true,

            // UmkmOwner can only access their own resources
            (UserRole::UmkmOwner, UserRole::UmkmOwner) => true,

            // All other combinations are denied
            _ => false,
        }
    }

    /// Get all roles that this role can access
    pub fn accessible_roles(&self) -> Vec<UserRole> {
        match self {
            UserRole::SuperAdmin => vec![
                UserRole::UmkmOwner,
                UserRole::AdminStaff,
                UserRole::SuperAdmin,
            ],
            UserRole::AdminStaff => vec![UserRole::UmkmOwner, UserRole::AdminStaff],
            UserRole::UmkmOwner => vec![UserRole::UmkmOwner],
        }
    }
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::UmkmOwner => write!(f, "umkm_owner"),
            UserRole::AdminStaff => write!(f, "admin_staff"),
            UserRole::SuperAdmin => write!(f, "super_admin"),
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "umkm_owner" => Ok(UserRole::UmkmOwner),
            "admin_staff" => Ok(UserRole::AdminStaff),
            "super_admin" => Ok(UserRole::SuperAdmin),
            _ => Err(format!("Invalid user role: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    PendingVerification,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::PendingVerification => write!(f, "pending_verification"),
        }
    }
}

impl std::str::FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "suspended" => Ok(UserStatus::Suspended),
            "pending_verification" => Ok(UserStatus::PendingVerification),
            _ => Err(format!("Invalid user status: {}", s)),
        }
    }
}

// User Entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: String,
    pub full_name: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub email_verified: bool,
    pub email_verification_token: Option<String>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<DateTime<Utc>>,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, password_hash: String, full_name: String, role: UserRole) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            password_hash,
            full_name,
            role,
            status: UserStatus::PendingVerification,
            email_verified: false,
            email_verification_token: Some(Uuid::new_v4().to_string()),
            password_reset_token: None,
            password_reset_expires: None,
            last_login: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if user can perform action on target user
    pub fn can_access_user(&self, target_user_role: &UserRole) -> bool {
        self.role.has_permission(target_user_role)
    }

    /// Check if user is active and can log in
    pub fn can_login(&self) -> bool {
        matches!(self.status, UserStatus::Active) && self.email_verified
    }

    /// Mark user as verified
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.email_verification_token = None;
        if matches!(self.status, UserStatus::PendingVerification) {
            self.status = UserStatus::Active;
        }
        self.updated_at = Utc::now();
    }

    /// Set password reset token
    pub fn set_password_reset_token(&mut self, token: String, expires_in_hours: i64) {
        self.password_reset_token = Some(token);
        self.password_reset_expires = Some(Utc::now() + chrono::Duration::hours(expires_in_hours));
        self.updated_at = Utc::now();
    }

    /// Clear password reset token
    pub fn clear_password_reset_token(&mut self) {
        self.password_reset_token = None;
        self.password_reset_expires = None;
        self.updated_at = Utc::now();
    }

    /// Update password
    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.clear_password_reset_token();
        self.updated_at = Utc::now();
    }

    /// Update last login time
    pub fn update_last_login(&mut self) {
        self.last_login = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}

// Domain Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserEvent {
    UserRegistered {
        user_id: UserId,
        email: Email,
        role: UserRole,
        timestamp: DateTime<Utc>,
    },
    UserLoggedIn {
        user_id: UserId,
        timestamp: DateTime<Utc>,
    },
    UserEmailVerified {
        user_id: UserId,
        timestamp: DateTime<Utc>,
    },
    UserPasswordChanged {
        user_id: UserId,
        timestamp: DateTime<Utc>,
    },
    UserStatusChanged {
        user_id: UserId,
        old_status: UserStatus,
        new_status: UserStatus,
        timestamp: DateTime<Utc>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(Email::new("valid@example.com".to_string()).is_ok());
        assert!(Email::new("invalid".to_string()).is_err());
        assert!(Email::new("@invalid.com".to_string()).is_err());
    }

    #[test]
    fn test_user_role_permissions() {
        assert!(UserRole::SuperAdmin.has_permission(&UserRole::UmkmOwner));
        assert!(UserRole::SuperAdmin.has_permission(&UserRole::AdminStaff));
        assert!(UserRole::AdminStaff.has_permission(&UserRole::UmkmOwner));
        assert!(!UserRole::UmkmOwner.has_permission(&UserRole::AdminStaff));
    }

    #[test]
    fn test_user_creation() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let user = User::new(
            email,
            "hashed_password".to_string(),
            "Test User".to_string(),
            UserRole::UmkmOwner,
        );

        assert_eq!(user.status, UserStatus::PendingVerification);
        assert!(!user.email_verified);
        assert!(user.email_verification_token.is_some());
    }

    #[test]
    fn test_user_email_verification() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let mut user = User::new(
            email,
            "hashed_password".to_string(),
            "Test User".to_string(),
            UserRole::UmkmOwner,
        );

        user.verify_email();
        assert!(user.email_verified);
        assert_eq!(user.status, UserStatus::Active);
        assert!(user.email_verification_token.is_none());
    }
}
