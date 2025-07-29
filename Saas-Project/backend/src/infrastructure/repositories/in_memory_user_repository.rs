// In-memory user repository for testing
// This implementation doesn't require a database connection

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::domain::entities::User;
#[cfg(test)]
use crate::domain::entities::{UserRole, UserStatus};
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::{Email, UserId};
use crate::shared::errors::{AppError, AppResult};

/// A simple in-memory implementation of the UserRepository trait for testing purposes
pub struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<UserId, User>>>,
}

impl InMemoryUserRepository {
    /// Create a new empty in-memory user repository
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new in-memory user repository with some initial data
    #[allow(dead_code)]
    pub fn with_users(users: Vec<User>) -> Self {
        let mut map = HashMap::new();
        for user in users {
            map.insert(user.id.clone(), user);
        }
        Self {
            users: Arc::new(Mutex::new(map)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> AppResult<()> {
        let mut users = self.users.lock().unwrap();
        
        // Check for duplicate email
        if users.values().any(|u| u.email == user.email && u.id != user.id) {
            return Err(AppError::Validation("Email already exists".to_string()));
        }
        
        users.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn delete(&self, id: &UserId) -> AppResult<()> {
        let mut users = self.users.lock().unwrap();
        users.remove(id);
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.get(id).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        let users = self.users.lock().unwrap();
        let user = users.values().find(|u| u.email == *email).cloned();
        Ok(user)
    }

    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
        let users = self.users.lock().unwrap();
        
        // Convert to vec and sort by created_at in descending order (newest first)
        let mut all_users: Vec<User> = users.values().cloned().collect();
        all_users.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply offset and limit
        let offset = offset.unwrap_or(0) as usize;
        
        if offset >= all_users.len() {
            return Ok(vec![]);
        }
        
        let users_after_offset = &all_users[offset..];
        
        if let Some(limit) = limit {
            let limit = limit as usize;
            if limit == 0 {
                return Ok(vec![]);
            }
            Ok(users_after_offset.iter().take(limit).cloned().collect())
        } else {
            Ok(users_after_offset.to_vec())
        }
    }

    async fn count_all(&self) -> AppResult<i64> {
        let users = self.users.lock().unwrap();
        Ok(users.len() as i64)
    }

    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
        let users = self.users.lock().unwrap();
        let query = query.to_lowercase();
        
        // Filter users where email or full_name contains the query string (case insensitive)
        let mut matching_users: Vec<User> = users
            .values()
            .filter(|u| {
                u.email.as_str().to_lowercase().contains(&query) || 
                u.full_name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();
        
        // Sort by created_at (newest first)
        matching_users.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply offset
        let offset = offset.unwrap_or(0) as usize;
        
        if offset >= matching_users.len() {
            return Ok(vec![]);
        }
        
        let users_after_offset = &matching_users[offset..];
        
        // Apply limit
        if let Some(limit) = limit {
            let limit = limit as usize;
            if limit == 0 {
                return Ok(vec![]);
            }
            Ok(users_after_offset.iter().take(limit).cloned().collect())
        } else {
            Ok(users_after_offset.to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_basic_operations() -> AppResult<()> {
        let repo = InMemoryUserRepository::new();
        
        // Create a user
        let user_id = UserId::new();
        let email = Email::new("test@example.com").unwrap();
        let user = User {
            id: user_id.clone(),
            email: email.clone(),
            password_hash: "hashed_password".to_string(),
            full_name: "Test User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Test save
        repo.save(&user).await?;
        
        // Test find_by_id
        let found_by_id = repo.find_by_id(&user_id).await?;
        assert!(found_by_id.is_some());
        assert_eq!(found_by_id.unwrap().full_name, "Test User");
        
        // Test find_by_email
        let found_by_email = repo.find_by_email(&email).await?;
        assert!(found_by_email.is_some());
        assert_eq!(found_by_email.unwrap().id, user_id);
        
        // Test update
        let mut updated_user = user.clone();
        updated_user.full_name = "Updated User".to_string();
        repo.save(&updated_user).await?;
        
        let found_after_update = repo.find_by_id(&user_id).await?;
        assert!(found_after_update.is_some());
        assert_eq!(found_after_update.unwrap().full_name, "Updated User");
        
        // Test delete
        repo.delete(&user_id).await?;
        let found_after_delete = repo.find_by_id(&user_id).await?;
        assert!(found_after_delete.is_none());
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_pagination_and_search() -> AppResult<()> {
        // Create multiple users for testing
        let mut users = Vec::new();
        for i in 0..10 {
            let user = User {
                id: UserId::new(),
                email: Email::new(&format!("test{}@example.com", i)).unwrap(),
                password_hash: "hashed_password".to_string(),
                full_name: format!("Test User {}", i),
                phone: None,
                role: UserRole::UmkmOwner,
                status: UserStatus::Active,
                email_verified_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            users.push(user);
        }
        
        let repo = InMemoryUserRepository::with_users(users);
        
        // Test count_all
        let count = repo.count_all().await?;
        assert_eq!(count, 10);
        
        // Test list_all with limit
        let list_limited = repo.list_all(Some(5), None).await?;
        assert_eq!(list_limited.len(), 5);
        
        // Test list_all with limit and offset
        let list_offset = repo.list_all(Some(3), Some(5)).await?;
        assert_eq!(list_offset.len(), 3);
        
        // Test search 
        let search_results = repo.search("test3", None, None).await?;
        assert_eq!(search_results.len(), 1);
        assert!(search_results[0].email.as_str().contains("test3"));
        
        // Test search with limit
        let search_limited = repo.search("test", Some(2), None).await?;
        assert_eq!(search_limited.len(), 2);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_duplicate_email() -> AppResult<()> {
        let repo = InMemoryUserRepository::new();
        
        // Create first user
        let user1 = User {
            id: UserId::new(),
            email: Email::new("same@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "First User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        repo.save(&user1).await?;
        
        // Try to create a second user with the same email but different ID
        let user2 = User {
            id: UserId::new(),  // Different ID
            email: Email::new("same@example.com").unwrap(), // Same email
            password_hash: "hashed_password".to_string(),
            full_name: "Second User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // This should fail
        let result = repo.save(&user2).await;
        assert!(result.is_err());
        
        Ok(())
    }
}
