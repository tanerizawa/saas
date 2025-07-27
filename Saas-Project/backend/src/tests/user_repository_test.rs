#[cfg(test)]
mod user_repository_tests {
    use chrono::Utc;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::repositories::UserRepository;
    use crate::domain::value_objects::{Email, UserId};
    use crate::infrastructure::repositories::PostgresUserRepository;
    use crate::shared::errors::AppResult;

    // Setup test helpers
    async fn setup_test_db() -> (PgPool, String) {
        // Use a unique schema for each test to isolate them
        let schema_name = format!("test_{}", Uuid::new_v4().as_simple());
        
        // Get database URL from environment or use a default test database
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string());

        // Connect to the test database
        let pool = PgPool::connect(&database_url).await.unwrap();

        // Create a new schema for this test
        sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
            .execute(&pool)
            .await
            .unwrap();

        // Set the search path to our new schema
        sqlx::query(&format!("SET search_path TO {}", schema_name))
            .execute(&pool)
            .await
            .unwrap();

        // Create test tables
        sqlx::query(
            r#"
            CREATE TABLE users (
                id UUID PRIMARY KEY,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                full_name TEXT NOT NULL,
                role TEXT NOT NULL,
                status TEXT NOT NULL,
                email_verified BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TIMESTAMPTZ NOT NULL,
                updated_at TIMESTAMPTZ NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        (pool, schema_name)
    }

    async fn create_test_users(repo: &dyn UserRepository, count: usize) -> Vec<User> {
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
            
            repo.save(&user).await.unwrap();
            users.push(user);
        }
        
        users
    }

    // Teardown function to clean up test resources
    #[allow(dead_code)] // This is used in real tests but may not be called in all test scenarios
    async fn teardown(pool: &PgPool, schema_name: &str) {
        // Drop the schema to clean up
        sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
            .execute(pool)
            .await
            .unwrap();
        
        // Close the connection pool to prevent potential memory leaks
        pool.close().await;
    }

    // Tests for pagination
    #[tokio::test]
    async fn test_list_all_with_pagination() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create 10 test users
        let _users = create_test_users(&repo, 10).await;
        
        // Test list with limit
        let result = repo.list_all(Some(5), None).await?;
        assert_eq!(result.len(), 5);
        
        // Test list with limit and offset
        let result = repo.list_all(Some(3), Some(5)).await?;
        assert_eq!(result.len(), 3);
        
        // Test count
        let count = repo.count_all().await?;
        assert_eq!(count, 10);
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
    
    // Tests for search
    #[tokio::test]
    async fn test_search_users() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create test users with specific emails for search
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
        
        repo.save(&user1).await?;
        repo.save(&user2).await?;
        repo.save(&user3).await?;
        
        // Test search by email
        let results = repo.search("john", None, None).await?;
        assert_eq!(results.len(), 2);  // Should match johndoe@example.com and John Smith
        
        // Test search by name
        let results = repo.search("Smith", None, None).await?;
        assert_eq!(results.len(), 1);  // Should match John Smith
        
        // Test search with limit
        let results = repo.search("doe", Some(1), None).await?;
        assert_eq!(results.len(), 1);  // Should only return 1 result even though there are 2 matches
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
    
    // Test for specific edge cases
    #[tokio::test]
    async fn test_edge_cases() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create a single test user
        create_test_users(&repo, 1).await;
        
        // Test empty search results
        let results = repo.search("nonexistent", None, None).await?;
        assert_eq!(results.len(), 0);
        
        // Test list with large offset (beyond available data)
        let results = repo.list_all(Some(10), Some(100)).await?;
        assert_eq!(results.len(), 0);
        
        // Test list with zero limit
        let results = repo.list_all(Some(0), None).await?;
        assert_eq!(results.len(), 0);
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
    
    // Test for basic CRUD operations
    #[tokio::test]
    async fn test_crud_operations() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create a user
        let user_id = UserId::new();
        let email = Email::new("crud_test@example.com").unwrap();
        
        let user = User {
            id: user_id.clone(),
            email: email.clone(),
            password_hash: "secure_hashed_password".to_string(),
            full_name: "CRUD Test User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Test Create
        repo.save(&user).await?;
        
        // Test Read
        let found_by_id = repo.find_by_id(&user_id).await?;
        assert!(found_by_id.is_some());
        assert_eq!(found_by_id.unwrap().full_name, "CRUD Test User");
        
        let found_by_email = repo.find_by_email(&email).await?;
        assert!(found_by_email.is_some());
        assert_eq!(found_by_email.unwrap().id, user_id);
        
        // Test Update
        let mut updated_user = user.clone();
        updated_user.full_name = "Updated CRUD User".to_string();
        updated_user.status = UserStatus::PendingVerification;
        
        repo.save(&updated_user).await?;
        
        let found_after_update = repo.find_by_id(&user_id).await?;
        assert!(found_after_update.is_some());
        let found_user = found_after_update.unwrap();
        assert_eq!(found_user.full_name, "Updated CRUD User");
        assert_eq!(found_user.status, UserStatus::PendingVerification);
        
        // Test Delete
        repo.delete(&user_id).await?;
        
        let found_after_delete = repo.find_by_id(&user_id).await?;
        assert!(found_after_delete.is_none());
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
    
    // Test for error handling
    #[tokio::test]
    async fn test_error_handling() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create two users with the same email to test duplicate constraint
        let user1 = User {
            id: UserId::new(),
            email: Email::new("duplicate@example.com").unwrap(),
            password_hash: "hashed_password".to_string(),
            full_name: "Original User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Save the first user successfully
        repo.save(&user1).await?;
        
        // Try to create another user with same email
        let user2 = User {
            id: UserId::new(), // Different ID
            email: Email::new("duplicate@example.com").unwrap(), // Same email
            password_hash: "different_password".to_string(),
            full_name: "Duplicate Email User".to_string(),
            phone: None,
            role: UserRole::AdminStaff,
            status: UserStatus::PendingVerification,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // This should fail with a unique constraint violation
        let result = repo.save(&user2).await;
        assert!(result.is_err());
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
}
