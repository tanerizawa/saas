#[cfg(test)]
mod user_repository_property_tests {
    use chrono::Utc;
    use proptest::prelude::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::repositories::UserRepository;
    use crate::domain::value_objects::{Email, UserId};
    use crate::infrastructure::repositories::PostgresUserRepository;
    use crate::shared::errors::AppResult;

    // Setup test helpers (reused from user_repository_test.rs)
    async fn setup_test_db() -> (PgPool, String) {
        // Use a unique schema for each test to isolate them
        let schema_name = format!("proptest_{}", Uuid::new_v4().as_simple());
        
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

    // Teardown function to clean up test resources
    async fn teardown(pool: &PgPool, schema_name: &str) {
        // Drop the schema to clean up
        sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
            .execute(pool)
            .await
            .unwrap();
        
        // Close the connection pool to prevent potential memory leaks
        pool.close().await;
    }

    // Proptest strategy for generating valid emails
    fn valid_email_strategy() -> impl Strategy<Value = String> {
        // Generate realistic but random email addresses
        // Username: 3-20 alphanumeric characters
        // Domain: 2-10 alphanumeric characters
        // TLD: com, org, net, io
        let username = prop::string::string_regex("[a-z0-9]{3,20}").unwrap();
        let domain = prop::string::string_regex("[a-z0-9]{2,10}").unwrap();
        let tld = prop::sample::select(vec!["com", "org", "net", "io"]);
        
        (username, domain, tld).prop_map(|(u, d, t)| format!("{}@{}.{}", u, d, t))
    }

    // Strategy for generating valid full names
    fn valid_name_strategy() -> impl Strategy<Value = String> {
        // First name: 2-15 alphabetic characters
        // Last name: 2-15 alphabetic characters
        // May include middle name or initials
        let first_name = prop::string::string_regex("[A-Z][a-z]{1,14}").unwrap();
        let last_name = prop::string::string_regex("[A-Z][a-z]{1,14}").unwrap();
        let with_middle = prop::bool::ANY;
        
        (first_name, last_name, with_middle).prop_map(|(f, l, with_middle)| {
            if with_middle {
                // Add a middle initial or name
                let middle = prop::string::string_regex("[A-Z][a-z]{0,10}").unwrap().new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current();
                format!("{} {} {}", f, middle, l)
            } else {
                format!("{} {}", f, l)
            }
        })
    }

    // Strategy for generating valid password hashes
    fn password_hash_strategy() -> impl Strategy<Value = String> {
        // Simulated bcrypt hash (doesn't need to be real for tests)
        prop::string::string_regex("\\$2b\\$[0-9]{2}\\$[A-Za-z0-9./]{53}").unwrap()
    }

    // Strategy for generating valid users
    fn valid_user_strategy() -> impl Strategy<Value = User> {
        let email_strategy = valid_email_strategy();
        let name_strategy = valid_name_strategy();
        let password_strategy = password_hash_strategy();
        
        // Generate a random user with valid data
        (email_strategy, name_strategy, password_strategy).prop_map(|(email_str, name, password_hash)| {
            let id = UserId::new();
            let email = Email::new(&email_str).unwrap(); // Safe because our strategy generates valid emails
            
            // Randomly assign role and status
            let role = match proptest::sample::select(&[0, 1, 2]).new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current() {
                0 => UserRole::UmkmOwner,
                1 => UserRole::AdminStaff,
                _ => UserRole::SuperAdmin,
            };
            
            let status = if proptest::bool::ANY.new_tree(&mut proptest::test_runner::TestRunner::default()).unwrap().current() {
                UserStatus::Active
            } else {
                UserStatus::PendingVerification
            };
            
            User {
                id,
                email,
                password_hash,
                full_name: name,
                phone: None, // We could generate random phone numbers too
                role,
                status,
                email_verified_at: None, // Could randomly set some as verified
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        })
    }

    // Property test: Creating and retrieving users by ID
    #[tokio::test]
    async fn prop_test_save_find_by_id() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        let user_strategy = valid_user_strategy();
        let mut runner = TestRunner::default();
        
        // Run property test multiple times with different users
        for _ in 0..10 {
            // Generate a random valid user
            let user = user_strategy.new_tree(&mut runner).unwrap().current();
            
            // Save the user
            repo.save(&user).await?;
            
            // Verify we can find it by ID
            let found = repo.find_by_id(&user.id).await?;
            assert!(found.is_some());
            
            let found_user = found.unwrap();
            assert_eq!(found_user.id, user.id);
            assert_eq!(found_user.email, user.email);
            assert_eq!(found_user.full_name, user.full_name);
            assert_eq!(found_user.role, user.role);
            assert_eq!(found_user.status, user.status);
        }
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }

    // Property test: Email uniqueness constraint
    #[tokio::test]
    async fn prop_test_email_uniqueness() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        let mut runner = TestRunner::default();
        
        // Create some random users with unique emails
        for _ in 0..5 {
            let user = valid_user_strategy().new_tree(&mut runner).unwrap().current();
            repo.save(&user).await?;
            
            // Try to create another user with the same email but different ID
            let mut duplicate_user = user.clone();
            duplicate_user.id = UserId::new(); // New ID
            
            // This should fail due to unique constraint
            let result = repo.save(&duplicate_user).await;
            assert!(result.is_err());
        }
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
    
    // Property test: Search functionality returns correct results
    #[tokio::test]
    async fn prop_test_search_functionality() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create a set of users with known name components for search testing
        let test_users = vec![
            User {
                id: UserId::new(),
                email: Email::new("john.smith@example.com").unwrap(),
                password_hash: "hashed_password".to_string(),
                full_name: "John Smith".to_string(),
                phone: None,
                role: UserRole::UmkmOwner,
                status: UserStatus::Active,
                email_verified_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            User {
                id: UserId::new(),
                email: Email::new("sarah.johnson@example.com").unwrap(),
                password_hash: "hashed_password".to_string(),
                full_name: "Sarah Johnson".to_string(),
                phone: None,
                role: UserRole::AdminStaff,
                status: UserStatus::Active,
                email_verified_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            User {
                id: UserId::new(),
                email: Email::new("michael.smith@example.com").unwrap(),
                password_hash: "hashed_password".to_string(),
                full_name: "Michael Smith".to_string(),
                phone: None,
                role: UserRole::SuperAdmin,
                status: UserStatus::PendingVerification,
                email_verified_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            User {
                id: UserId::new(),
                email: Email::new("john.doe@otherexample.com").unwrap(),
                password_hash: "hashed_password".to_string(),
                full_name: "John Doe".to_string(),
                phone: None,
                role: UserRole::UmkmOwner,
                status: UserStatus::Active,
                email_verified_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        ];
        
        // Save all test users
        for user in &test_users {
            repo.save(user).await?;
        }
        
        // Property: Searching for "John" should return exactly 2 users
        let john_results = repo.search("John", None, None).await?;
        assert_eq!(john_results.len(), 2);
        
        // Property: Searching for "Smith" should return exactly 2 users
        let smith_results = repo.search("Smith", None, None).await?;
        assert_eq!(smith_results.len(), 2);
        
        // Property: Searching for "example.com" should return exactly 3 users
        let example_results = repo.search("example.com", None, None).await?;
        assert_eq!(example_results.len(), 3);
        
        // Property: Searching for "michael" should return exactly 1 user
        let michael_results = repo.search("michael", None, None).await?;
        assert_eq!(michael_results.len(), 1);
        assert_eq!(michael_results[0].full_name, "Michael Smith");
        
        // Property: Searching for nonexistent term should return 0 results
        let nonexistent_results = repo.search("nonexistentxyz123", None, None).await?;
        assert_eq!(nonexistent_results.len(), 0);
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
    
    // Property test: Pagination with different page sizes
    #[tokio::test]
    async fn prop_test_pagination() -> AppResult<()> {
        // Setup
        let (pool, schema_name) = setup_test_db().await;
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Create 20 test users
        let mut runner = TestRunner::default();
        let mut all_users = Vec::new();
        
        for _ in 0..20 {
            let user = valid_user_strategy().new_tree(&mut runner).unwrap().current();
            repo.save(&user).await?;
            all_users.push(user);
        }
        
        // Property: Count should match the number of users we created
        let count = repo.count_all().await?;
        assert_eq!(count, 20);
        
        // Test different page sizes from 1 to 25
        for page_size in 1..=25 {
            let results = repo.list_all(Some(page_size), None).await?;
            
            // Property: Results should not exceed page size
            assert!(results.len() <= page_size);
            
            // Property: If page size > total users, all users should be returned
            if page_size > 20 {
                assert_eq!(results.len(), 20);
            } else {
                assert_eq!(results.len(), page_size);
            }
        }
        
        // Test offsets with fixed page size
        for offset in 0..22 {
            let page_size = 5;
            let results = repo.list_all(Some(page_size), Some(offset)).await?;
            
            // Property: Results should be empty if offset >= total users
            if offset >= 20 {
                assert_eq!(results.len(), 0);
            } 
            // Property: Results should be less than page size if near the end
            else if offset + page_size > 20 {
                assert_eq!(results.len(), 20 - offset);
            } 
            // Property: Results should be exactly page size otherwise
            else {
                assert_eq!(results.len(), page_size);
            }
        }
        
        // Clean up
        teardown(&pool, &schema_name).await;
        
        Ok(())
    }
}
