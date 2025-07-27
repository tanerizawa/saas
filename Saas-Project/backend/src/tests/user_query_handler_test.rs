// Tests for the user query handler
#[cfg(test)]
mod user_query_handler_tests {
    use std::sync::Arc;
    use chrono::Utc;

    use crate::application::query_handlers::UserQueryHandler;
    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::repositories::UserRepository;
    use crate::domain::value_objects::{Email, UserId};
    use crate::shared::errors::AppResult;
    use async_trait::async_trait;

    // Mock implementation of UserRepository for testing
    struct MockUserRepository {
        users: Vec<User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self { users: Vec::new() }
        }

        fn with_users(users: Vec<User>) -> Self {
            Self { users }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
            let user = self.users.iter()
                .find(|u| u.id.as_uuid() == id.as_uuid())
                .cloned();
            Ok(user)
        }

        async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
            let user = self.users.iter()
                .find(|u| u.email.as_str() == email.as_str())
                .cloned();
            Ok(user)
        }

        async fn save(&self, _user: &User) -> AppResult<()> {
            // In a real implementation, we'd add the user to self.users
            // but since self is immutable, we're just returning Ok
            Ok(())
        }

        async fn delete(&self, _id: &UserId) -> AppResult<()> {
            Ok(())
        }

        async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
            let offset = offset.unwrap_or(0) as usize;
            let users = match limit {
                Some(limit) if limit > 0 => {
                    let limit = limit as usize;
                    self.users.iter()
                        .skip(offset)
                        .take(limit)
                        .cloned()
                        .collect()
                },
                _ => self.users.iter().skip(offset).cloned().collect()
            };
            
            Ok(users)
        }

        async fn count_all(&self) -> AppResult<i64> {
            Ok(self.users.len() as i64)
        }

        async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
            let query = query.to_lowercase();
            let offset = offset.unwrap_or(0) as usize;
            
            let filtered_users = self.users.iter()
                .filter(|u| {
                    u.email.as_str().to_lowercase().contains(&query) ||
                    u.full_name.to_lowercase().contains(&query)
                })
                .cloned();
            
            let users = match limit {
                Some(limit) if limit > 0 => {
                    filtered_users.skip(offset).take(limit as usize).collect()
                },
                _ => filtered_users.skip(offset).collect()
            };
            
            Ok(users)
        }
    }

    // Helper to create test users
    fn create_test_users(count: usize) -> Vec<User> {
        let mut users = Vec::new();
        
        for i in 0..count {
            let user_id = UserId::new();
            let email = Email::new(&format!("test{}@example.com", i)).unwrap();
            
            let user = User {
                id: user_id,
                email,
                password_hash: "hashed_password".to_string(),
                full_name: format!("Test User {}", i),
                phone: None,
                role: if i % 3 == 0 { 
                    UserRole::UmkmOwner 
                } else if i % 3 == 1 { 
                    UserRole::AdminStaff 
                } else { 
                    UserRole::SuperAdmin 
                },
                status: if i % 2 == 0 { 
                    UserStatus::Active 
                } else { 
                    UserStatus::PendingVerification 
                },
                email_verified_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            
            users.push(user);
        }
        
        users
    }

    #[tokio::test]
    async fn test_handle_list_users_pagination() -> AppResult<()> {
        // Create 10 test users
        let test_users = create_test_users(10);
        
        // Create mock repository with test users
        let repo = Arc::new(MockUserRepository::with_users(test_users));
        
        // Create query handler
        let handler = UserQueryHandler::new(repo);
        
        // Test first page with limit 5
        let page1 = handler.handle_list_users(1, 5).await?;
        assert_eq!(page1.data.len(), 5);
        assert_eq!(page1.total, 10);
        assert_eq!(page1.page, 1);
        assert_eq!(page1.limit, 5);
        
        // Test second page with limit 5
        let page2 = handler.handle_list_users(2, 5).await?;
        assert_eq!(page2.data.len(), 5);
        assert_eq!(page2.total, 10);
        assert_eq!(page2.page, 2);
        
        // Test page beyond available data
        let empty_page = handler.handle_list_users(3, 5).await?;
        assert_eq!(empty_page.data.len(), 0);
        assert_eq!(empty_page.total, 10);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_handle_search_users() -> AppResult<()> {
        // Create mock users with specific names and emails for search testing
        let mut users = Vec::new();
        
        let user1 = User {
            id: UserId::new(),
            email: Email::new("johndoe@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "John Doe".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let user2 = User {
            id: UserId::new(),
            email: Email::new("janedoe@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "Jane Doe".to_string(),
            phone: None,
            role: UserRole::AdminStaff,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let user3 = User {
            id: UserId::new(),
            email: Email::new("smith@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "John Smith".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        users.push(user1);
        users.push(user2);
        users.push(user3);
        
        // Create mock repository with test users
        let repo = Arc::new(MockUserRepository::with_users(users));
        
        // Create query handler
        let handler = UserQueryHandler::new(repo);
        
        // Test search by name
        let results = handler.handle_search_users("john").await?;
        assert_eq!(results.len(), 2);  // Should match John Doe and John Smith
        
        // Test search by email
        let results = handler.handle_search_users("doe@example").await?;
        assert_eq!(results.len(), 2);  // Should match johndoe@example.com and janedoe@example.com
        
        // Test search with no matches
        let results = handler.handle_search_users("nonexistent").await?;
        assert_eq!(results.len(), 0);
        
        Ok(())
    }
}
