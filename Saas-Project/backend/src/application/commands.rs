#![allow(dead_code)]

// use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{Email, UserId, PhoneNumber};
use crate::domain::entities::UserRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserCommand {
    pub email: Email,
    pub password: String,
    pub full_name: String,
    pub role: Option<UserRole>,
}

impl CreateUserCommand {
    pub fn new(email: Email, password: String, full_name: String) -> Self {
        Self {
            email,
            password,
            full_name,
            role: None, // Default to UmkmOwner
        }
    }

    pub fn with_role(mut self, role: UserRole) -> Self {
        self.role = Some(role);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub user_id: UserId,
    pub full_name: Option<String>,
    pub phone: Option<PhoneNumber>,
}

impl UpdateUserCommand {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            full_name: None,
            phone: None,
        }
    }

    pub fn with_full_name(mut self, full_name: String) -> Self {
        self.full_name = Some(full_name);
        self
    }

    pub fn with_phone(mut self, phone: PhoneNumber) -> Self {
        self.phone = Some(phone);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordCommand {
    pub user_id: UserId,
    pub current_password: String,
    pub new_password: String,
}

impl ChangePasswordCommand {
    pub fn new(user_id: UserId, current_password: String, new_password: String) -> Self {
        Self {
            user_id,
            current_password,
            new_password,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserCommand {
    pub user_id: UserId,
    pub reason: Option<String>,
}

impl DeleteUserCommand {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: String) -> Self {
        self.reason = Some(reason);
        self
    }
}

// License Management Commands (placeholder for future implementation)
#[allow(dead_code)]
pub struct CreateLicenseCommand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_command() {
        let email = Email::new("test@example.com").unwrap();
        let command = CreateUserCommand::new(
            email,
            "password123".to_string(),
            "Test User".to_string(),
        );

        assert_eq!(command.full_name, "Test User");
        assert!(command.role.is_none());
    }

    #[test]
    fn test_create_user_command_with_role() {
        let email = Email::new("admin@example.com").unwrap();
        let command = CreateUserCommand::new(
            email,
            "password123".to_string(),
            "Admin User".to_string(),
        ).with_role(UserRole::AdminStaff);

        assert_eq!(command.role, Some(UserRole::AdminStaff));
    }

    #[test]
    fn test_update_user_command() {
        let user_id = UserId::new();
        let command = UpdateUserCommand::new(user_id)
            .with_full_name("Updated Name".to_string());

        assert_eq!(command.full_name, Some("Updated Name".to_string()));
        assert!(command.phone.is_none());
    }

    #[test]
    fn test_change_password_command() {
        let user_id = UserId::new();
        let command = ChangePasswordCommand::new(
            user_id,
            "old_password".to_string(),
            "new_password".to_string(),
        );

        assert_eq!(command.current_password, "old_password");
        assert_eq!(command.new_password, "new_password");
    }
}
