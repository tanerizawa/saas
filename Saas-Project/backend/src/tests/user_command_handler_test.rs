// Tests for the user command handler
#[cfg(test)]
mod user_command_handler_tests {
    use std::sync::Arc;
    use chrono::Utc;

    use crate::application::command_handlers::UserCommandHandler;
    use crate::application::commands::CreateUserCommand;
    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::repositories::UserRepository;
    use crate::domain::value_objects::{Email, UserId, PhoneNumber};
    use crate::shared::errors::{AppError, AppResult};
    use crate::services::auth::AuthService;
    use async_trait::async_trait;
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock implementation of UserRepository that tracks operations for testing
    struct MockUserRepository {
        users: Mutex<HashMap<String, User>>,
    }

    // Helper function to create a mock auth service for tests
    fn create_mock_auth_service() -> AuthService {
        // Use the public constructor that exists in the auth service
        AuthService::new("test_secret_key".to_string())
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self { 
                users: Mutex::new(HashMap::new())
            }
        }

        fn with_users(users: Vec<User>) -> Self {
            let mut map = HashMap::new();
            for user in users {
                map.insert(user.email.as_str().to_string(), user);
            }
            Self { users: Mutex::new(map) }
        }

        // Helper to inspect the state of users for test assertions
        fn get_all_users(&self) -> Vec<User> {
            let users = self.users.lock().unwrap();
            users.values().cloned().collect()
        }

        // Helper to check if a specific user exists
        fn contains_user_with_email(&self, email: &str) -> bool {
            let users = self.users.lock().unwrap();
            users.contains_key(email)
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
            let users = self.users.lock().unwrap();
            let user = users.values()
                .find(|u| u.id.as_uuid() == id.as_uuid())
                .cloned();
            Ok(user)
        }

        async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
            let users = self.users.lock().unwrap();
            let user = users.get(email.as_str()).cloned();
            Ok(user)
        }

        async fn save(&self, user: &User) -> AppResult<()> {
            let mut users = self.users.lock().unwrap();
            users.insert(user.email.as_str().to_string(), user.clone());
            Ok(())
        }

        async fn delete(&self, id: &UserId) -> AppResult<()> {
            let mut users = self.users.lock().unwrap();
            users.retain(|_, user| user.id.as_uuid() != id.as_uuid());
            Ok(())
        }

        async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
            let users = self.users.lock().unwrap();
            let mut all_users: Vec<User> = users.values().cloned().collect();
            
            let offset = offset.unwrap_or(0) as usize;
            if offset > all_users.len() {
                return Ok(Vec::new());
            }
            
            match limit {
                Some(limit) if limit > 0 => {
                    let limit = limit as usize;
                    Ok(all_users.drain(offset..).take(limit).collect())
                },
                _ => Ok(all_users.drain(offset..).collect()),
            }
        }

        async fn count_all(&self) -> AppResult<i64> {
            let users = self.users.lock().unwrap();
            Ok(users.len() as i64)
        }

        async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
            let users = self.users.lock().unwrap();
            let query = query.to_lowercase();
            
            let matching_users: Vec<User> = users.values()
                .filter(|user| {
                    user.email.as_str().to_lowercase().contains(&query) ||
                    user.full_name.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
            
            let offset = offset.unwrap_or(0) as usize;
            if offset > matching_users.len() {
                return Ok(Vec::new());
            }
            
            match limit {
                Some(limit) if limit > 0 => {
                    let limit = limit as usize;
                    Ok(matching_users.into_iter().skip(offset).take(limit).collect())
                },
                _ => Ok(matching_users.into_iter().skip(offset).collect()),
            }
        }
    }

    #[tokio::test]
    async fn test_create_user_command() -> AppResult<()> {
        // Create mock repository
        let repo = Arc::new(MockUserRepository::new());
        let auth_service = Arc::new(create_mock_auth_service());
        
        // Create command handler with mock repository
        let handler = UserCommandHandler::new(repo.clone(), auth_service);
        
        // Create a test command
        let email = Email::new("new_user@example.com").unwrap();
        let cmd = CreateUserCommand {
            email,
            password: "SecurePassword123!".to_string(),
            full_name: "New Test User".to_string(),
            role: Some(UserRole::UmkmOwner),
        };
        
        // Execute the command
        let user_id = handler.handle_create_user(cmd).await?;
        
        // Verify the user was created
        assert!(repo.contains_user_with_email("new_user@example.com"));
        
        // Get the user and validate fields
        let all_users = repo.get_all_users();
        assert_eq!(all_users.len(), 1);
        
        let user = &all_users[0];
        assert_eq!(user.id, user_id);
        assert_eq!(user.email.as_str(), "new_user@example.com");
        assert_eq!(user.full_name, "New Test User");
        assert_eq!(user.role, UserRole::UmkmOwner);
        
        // Password should be hashed, not stored as plaintext
        assert_ne!(user.password_hash, "SecurePassword123!");
        assert!(user.password_hash.starts_with("hashed_"));
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_create_user_with_duplicate_email() -> AppResult<()> {
        // Create a user to start with
        let email = Email::new("existing@example.com").unwrap();
        let existing_user = User {
            id: UserId::new(),
            email,
            password_hash: "already_hashed".to_string(),
            full_name: "Existing User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let repo = Arc::new(MockUserRepository::with_users(vec![existing_user]));
        let auth_service = Arc::new(create_mock_auth_service());
        let handler = UserCommandHandler::new(repo.clone(), auth_service);
        
        // Try to create a user with the same email
        let email = Email::new("existing@example.com").unwrap();
        let cmd = CreateUserCommand {
            email,
            password: "AnotherPassword456!".to_string(),
            full_name: "Duplicate Email User".to_string(),
            role: None,
        };
        
        // This should fail with an appropriate error
        let result = handler.handle_create_user(cmd).await;
        
        assert!(result.is_err());
        // Check if error is the right type (email already exists)
        match result {
            Err(AppError::Conflict(msg)) => {
                assert!(msg.contains("User with this email already exists"));
            },
            _ => panic!("Expected a conflict error for duplicate email"),
        }
        
        // Verify no new user was added
        let all_users = repo.get_all_users();
        assert_eq!(all_users.len(), 1);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_update_user() -> AppResult<()> {
        // Create a user to update
        let user_id = UserId::new();
        let email = Email::new("update_test@example.com").unwrap();
        let existing_user = User {
            id: user_id.clone(),
            email,
            password_hash: "already_hashed".to_string(),
            full_name: "Original Name".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let repo = Arc::new(MockUserRepository::with_users(vec![existing_user]));
        let auth_service = Arc::new(create_mock_auth_service());
        let handler = UserCommandHandler::new(repo.clone(), auth_service);
        
        // Create update command
        let phone = PhoneNumber("555-123-4567".to_string());
        let cmd = crate::application::commands::UpdateUserCommand {
            user_id,
            full_name: Some("Updated Name".to_string()),
            phone: Some(phone),
        };
        
        // Execute the update
        handler.handle_update_user(cmd).await?;
        
        // Verify the update worked
        let all_users = repo.get_all_users();
        assert_eq!(all_users.len(), 1);
        
        let updated_user = &all_users[0];
        assert_eq!(updated_user.full_name, "Updated Name");
        assert_eq!(updated_user.phone.as_ref().unwrap().0, "555-123-4567");
        
        Ok(())
    }
}
