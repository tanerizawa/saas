#![allow(dead_code)]

use std::sync::Arc;

use super::commands::{ChangePasswordCommand, CreateUserCommand, UpdateUserCommand};
use crate::domain::entities::{User, UserRole};
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::UserId;
use crate::services::auth::AuthService;
use crate::shared::errors::{AppError, AppResult};

pub struct UserCommandHandler {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    auth_service: Arc<AuthService>,
}

impl UserCommandHandler {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        auth_service: Arc<AuthService>,
    ) -> Self {
        Self {
            user_repository,
            auth_service,
        }
    }

    pub async fn handle_create_user(&self, command: CreateUserCommand) -> AppResult<UserId> {
        // Check if user already exists
        let existing_user = self.user_repository.find_by_email(&command.email).await?;
        if existing_user.is_some() {
            return Err(AppError::Conflict(
                "User with this email already exists".to_string(),
            ));
        }

        // Hash password
        let password_hash = self
            .auth_service
            .hash_password(&command.password)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?;

        // Create user entity
        let user = User::new(
            command.email,
            password_hash,
            command.full_name,
            command.role.unwrap_or(UserRole::UmkmOwner),
        );

        // Save user
        self.user_repository.save(&user).await?;

        Ok(user.id)
    }

    pub async fn handle_update_user(&self, command: UpdateUserCommand) -> AppResult<()> {
        // Get existing user
        let mut user = self
            .user_repository
            .find_by_id(&command.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Update fields if provided
        if let Some(full_name) = command.full_name {
            user.full_name = full_name;
        }

        if let Some(phone) = command.phone {
            user.phone = Some(phone);
        }

        // Save updated user
        self.user_repository.save(&user).await?;

        Ok(())
    }

    pub async fn handle_change_password(&self, command: ChangePasswordCommand) -> AppResult<()> {
        // Get user
        let mut user = self
            .user_repository
            .find_by_id(&command.user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Verify current password
        let is_valid = self
            .auth_service
            .verify_password(&command.current_password, &user.password_hash)
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            return Err(AppError::Unauthorized(
                "Current password is incorrect".to_string(),
            ));
        }

        // Hash new password
        let new_password_hash = self
            .auth_service
            .hash_password(&command.new_password)
            .map_err(|e| AppError::InternalError(format!("Failed to hash new password: {}", e)))?;

        // Update password
        user.update_password(new_password_hash);

        // Save user
        self.user_repository.save(&user).await?;

        Ok(())
    }

    pub async fn handle_verify_email(&self, user_id: UserId) -> AppResult<()> {
        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.verify_email();
        self.user_repository.save(&user).await?;

        Ok(())
    }

    pub async fn handle_deactivate_user(&self, user_id: UserId) -> AppResult<()> {
        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Set user status to inactive
        user.status = crate::domain::entities::UserStatus::Inactive;
        self.user_repository.save(&user).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::domain::value_objects::PhoneNumber;

    // Mock implementations would go here for testing
    // For now, we'll add basic construction tests

    #[test]
    fn test_command_handler_construction() {
        // This test will be completed when we have mock repositories
        // let handler = UserCommandHandler::new(mock_repo, mock_auth);
        // assert!(true);
    }
}
