//! Simplified user repository test
//! This will test the basic functionality of the user repository without other dependencies

use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;

// Simple error type for our test
#[derive(Debug)]
enum TestError {
    Database(String),
    NotFound(String),
}

type TestResult<T> = Result<T, TestError>;

/// User ID value object
#[derive(Debug, Clone, PartialEq)]
struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

/// Email value object with validation
#[derive(Debug, Clone, PartialEq)]
struct Email(String);

impl Email {
    fn new(email: &str) -> Result<Self, &'static str> {
        // Simple validation
        if email.contains('@') {
            Ok(Self(email.to_string()))
        } else {
            Err("Invalid email format")
        }
    }
    
    fn as_str(&self) -> &str {
        &self.0
    }
}

/// User roles
#[derive(Debug, Clone, PartialEq)]
enum UserRole {
    UmkmOwner,
    AdminStaff,
    SuperAdmin,
}

impl UserRole {
    fn as_str(&self) -> &'static str {
        match self {
            UserRole::UmkmOwner => "umkm_owner",
            UserRole::AdminStaff => "admin_staff",
            UserRole::SuperAdmin => "super_admin",
        }
    }
    
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "umkm_owner" => Ok(UserRole::UmkmOwner),
            "admin_staff" => Ok(UserRole::AdminStaff),
            "super_admin" => Ok(UserRole::SuperAdmin),
            _ => Err("Invalid user role"),
        }
    }
}

/// User status
#[derive(Debug, Clone, PartialEq)]
enum UserStatus {
    Active,
    Inactive,
    PendingVerification,
    Suspended,
}

impl UserStatus {
    fn as_str(&self) -> &'static str {
        match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::PendingVerification => "pending_verification",
            UserStatus::Suspended => "suspended",
        }
    }
    
    fn from_str(s: &str) -> Result<Self, &'static str> {
        match s {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            "pending_verification" => Ok(UserStatus::PendingVerification),
            "suspended" => Ok(UserStatus::Suspended),
            _ => Err("Invalid user status"),
        }
    }
}

/// User entity
#[derive(Debug, Clone)]
struct User {
    id: UserId,
    email: Email,
    password_hash: String,
    full_name: String,
    phone: Option<String>,
    role: UserRole,
    status: UserStatus,
    email_verified_at: Option<chrono::DateTime<Utc>>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

/// User repository trait
#[async_trait]
trait UserRepository {
    async fn find_by_id(&self, id: &UserId) -> TestResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> TestResult<Option<User>>;
    async fn save(&self, user: &User) -> TestResult<()>;
    async fn delete(&self, id: &UserId) -> TestResult<()>;
    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> TestResult<Vec<User>>;
    async fn count_all(&self) -> TestResult<i64>;
    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> TestResult<Vec<User>>;
}

/// PostgreSQL implementation of the user repository
struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn row_to_user(&self, row: &sqlx::postgres::PgRow) -> TestResult<User> {
        use sqlx::Row;
        
        let id_uuid: Uuid = row.try_get("id")
            .map_err(|e| TestError::Database(format!("Invalid id: {}", e)))?;
        let user_id = UserId(id_uuid);
        
        let email_str: String = row.try_get("email")
            .map_err(|e| TestError::Database(format!("Invalid email: {}", e)))?;
        let email = Email::new(&email_str)
            .map_err(|e| TestError::Database(format!("{}", e)))?;
        
        let role_str: String = row.try_get("role")
            .map_err(|e| TestError::Database(format!("Invalid role: {}", e)))?;
        let role = UserRole::from_str(&role_str)
            .map_err(|e| TestError::Database(format!("{}", e)))?;
        
        let status_str: String = row.try_get("status")
            .map_err(|e| TestError::Database(format!("Invalid status: {}", e)))?;
        let status = UserStatus::from_str(&status_str)
            .map_err(|e| TestError::Database(format!("{}", e)))?;

        let email_verified: bool = row.try_get("email_verified")
            .map_err(|e| TestError::Database(format!("Invalid email_verified: {}", e)))?;
        let email_verified_at = if email_verified {
            Some(
                row.try_get("created_at")
                    .map_err(|e| TestError::Database(format!("Invalid created_at: {}", e)))?,
            )
        } else {
            None
        };

        Ok(User {
            id: user_id,
            email,
            password_hash: row.try_get("password_hash")
                .map_err(|e| TestError::Database(format!("Invalid password_hash: {}", e)))?,
            full_name: row.try_get("full_name")
                .map_err(|e| TestError::Database(format!("Invalid full_name: {}", e)))?,
            phone: None, // Database doesn't have phone column yet
            role,
            status,
            email_verified_at,
            created_at: row.try_get("created_at")
                .map_err(|e| TestError::Database(format!("Invalid created_at: {}", e)))?,
            updated_at: row.try_get("updated_at")
                .map_err(|e| TestError::Database(format!("Invalid updated_at: {}", e)))?,
        })
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: &UserId) -> TestResult<Option<User>> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, full_name, role, status, 
                   email_verified, created_at, updated_at
            FROM users 
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        match result {
            Some(row) => {
                let user = self.row_to_user(&row).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &Email) -> TestResult<Option<User>> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, full_name, role, status, 
                   email_verified, created_at, updated_at
            FROM users 
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        match result {
            Some(row) => {
                let user = self.row_to_user(&row).await?;
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> TestResult<()> {
        let role_str = user.role.as_str();
        let status_str = user.status.as_str();
        let email_verified = user.email_verified_at.is_some();

        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, full_name, role, status, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) 
            DO UPDATE SET 
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                full_name = EXCLUDED.full_name,
                role = EXCLUDED.role,
                status = EXCLUDED.status,
                email_verified = EXCLUDED.email_verified,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(user.id.as_uuid())
        .bind(user.email.as_str())
        .bind(&user.password_hash)
        .bind(&user.full_name)
        .bind(role_str)
        .bind(status_str)
        .bind(email_verified)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, id: &UserId) -> TestResult<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(TestError::NotFound("User not found".to_string()));
        }

        Ok(())
    }
    
    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> TestResult<Vec<User>> {
        let query = match (limit, offset) {
            (Some(limit), Some(offset)) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#
                )
                .bind(limit)
                .bind(offset)
            },
            (Some(limit), None) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    LIMIT $1
                    "#
                )
                .bind(limit)
            },
            _ => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    "#
                )
            }
        };

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        let mut users = Vec::new();
        for row in rows {
            match self.row_to_user(&row).await {
                Ok(user) => users.push(user),
                Err(e) => eprintln!("Error converting row to user: {:?}", e),
            }
        }

        Ok(users)
    }

    async fn count_all(&self) -> TestResult<i64> {
        use sqlx::Row;
        
        let row = sqlx::query("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        let count: i64 = row.try_get("count").map_err(|e| {
            TestError::Database(format!("Error getting count from row: {}", e))
        })?;

        Ok(count)
    }

    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> TestResult<Vec<User>> {
        let search_term = format!("%{}%", query);
        let sql_query = match (limit, offset) {
            (Some(limit), Some(offset)) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    WHERE email ILIKE $1 OR full_name ILIKE $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#
                )
                .bind(&search_term)
                .bind(limit)
                .bind(offset)
            },
            (Some(limit), None) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    WHERE email ILIKE $1 OR full_name ILIKE $1
                    ORDER BY created_at DESC
                    LIMIT $2
                    "#
                )
                .bind(&search_term)
                .bind(limit)
            },
            _ => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    WHERE email ILIKE $1 OR full_name ILIKE $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(&search_term)
            }
        };

        let rows = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| TestError::Database(format!("Database error: {}", e)))?;

        let mut users = Vec::new();
        for row in rows {
            match self.row_to_user(&row).await {
                Ok(user) => users.push(user),
                Err(e) => eprintln!("Error converting row to user: {:?}", e),
            }
        }

        Ok(users)
    }
}

// Test helpers
async fn setup_test_db() -> (PgPool, String) {
    // Use a unique schema for each test to isolate them
    let schema_name = format!("test_{}", Uuid::new_v4().simple());
    
    // Get database URL from environment or use a default test database
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string());

    println!("Connecting to database: {}", database_url);
    println!("Using schema: {}", schema_name);

    // Connect to the test database
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    // Create a new schema for this test
    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
        .execute(&pool)
        .await
        .expect("Failed to create schema");

    // Set the search path to our new schema
    sqlx::query(&format!("SET search_path TO {}", schema_name))
        .execute(&pool)
        .await
        .expect("Failed to set search path");

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
    .expect("Failed to create users table");

    (pool, schema_name)
}

async fn teardown(pool: &PgPool, schema_name: &str) {
    // Drop the schema to clean up
    sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
        .execute(pool)
        .await
        .expect("Failed to drop schema");
    
    // Close the connection pool
    pool.close().await;
}

async fn create_test_users(repo: &PostgresUserRepository, count: usize) -> Vec<User> {
    let mut users = Vec::new();
    
    for i in 0..count {
        let user_id = UserId::new();
        let email = Email::new(&format!("test{}@example.com", i)).expect("Failed to create email");
        
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
        
        repo.save(&user).await.expect("Failed to save user");
        users.push(user);
    }
    
    users
}

// Test for basic CRUD operations
async fn test_crud_operations(pool: &PgPool, _schema_name: &str) -> TestResult<()> {
    // Setup
    let repo = PostgresUserRepository::new(pool.clone());
    
    // Create a user
    let user_id = UserId::new();
    let email = Email::new("crud_test@example.com").map_err(|e| TestError::Database(e.to_string()))?;
    
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
    println!("✓ User created successfully");
    
    // Test Read
    let found_by_id = repo.find_by_id(&user_id).await?;
    assert!(found_by_id.is_some(), "User should be found by ID");
    assert_eq!(found_by_id.as_ref().unwrap().full_name, "CRUD Test User", "User full name should match");
    println!("✓ User found by ID");
    
    let found_by_email = repo.find_by_email(&email).await?;
    assert!(found_by_email.is_some(), "User should be found by email");
    assert_eq!(found_by_email.as_ref().unwrap().id.as_uuid(), user_id.as_uuid(), "User IDs should match");
    println!("✓ User found by email");
    
    // Test Update
    let mut updated_user = user.clone();
    updated_user.full_name = "Updated CRUD User".to_string();
    updated_user.status = UserStatus::PendingVerification;
    
    repo.save(&updated_user).await?;
    println!("✓ User updated successfully");
    
    let found_after_update = repo.find_by_id(&user_id).await?;
    assert!(found_after_update.is_some(), "User should be found after update");
    let found_user = found_after_update.unwrap();
    assert_eq!(found_user.full_name, "Updated CRUD User", "User name should be updated");
    assert_eq!(found_user.status, UserStatus::PendingVerification, "User status should be updated");
    println!("✓ User update verified");
    
    // Test Delete
    repo.delete(&user_id).await?;
    println!("✓ User deleted successfully");
    
    let found_after_delete = repo.find_by_id(&user_id).await?;
    assert!(found_after_delete.is_none(), "User should not be found after delete");
    println!("✓ User deletion verified");
    
    Ok(())
}

// Tests for pagination
async fn test_list_all_with_pagination(pool: &PgPool, _schema_name: &str) -> TestResult<()> {
    // Setup
    let repo = PostgresUserRepository::new(pool.clone());
    
    // Create 10 test users
    let _users = create_test_users(&repo, 10).await;
    println!("✓ Created 10 test users");
    
    // Test list with limit
    let result = repo.list_all(Some(5), None).await?;
    assert_eq!(result.len(), 5, "Should return 5 users when limit is 5");
    println!("✓ List with limit works");
    
    // Test list with limit and offset
    let result = repo.list_all(Some(3), Some(5)).await?;
    assert_eq!(result.len(), 3, "Should return 3 users when limit is 3 and offset is 5");
    println!("✓ List with limit and offset works");
    
    // Test count
    let count = repo.count_all().await?;
    assert_eq!(count, 10, "Should count 10 users");
    println!("✓ Count all works");
    
    Ok(())
}

// Tests for search
async fn test_search_users(pool: &PgPool, _schema_name: &str) -> TestResult<()> {
    // Setup
    let repo = PostgresUserRepository::new(pool.clone());
    
    // Create test users with specific emails for search
    let user1 = User {
        id: UserId::new(),
        email: Email::new("johndoe@example.com").map_err(|e| TestError::Database(e.to_string()))?,
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
        email: Email::new("janedoe@example.com").map_err(|e| TestError::Database(e.to_string()))?,
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
        email: Email::new("smith@example.com").map_err(|e| TestError::Database(e.to_string()))?,
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
    println!("✓ Created test users for search");
    
    // Test search by email or name containing "john"
    let results = repo.search("john", None, None).await?;
    assert_eq!(results.len(), 2, "Should find 2 users matching 'john'");
    println!("✓ Search by name/email works");
    
    // Test search by name
    let results = repo.search("Smith", None, None).await?;
    assert_eq!(results.len(), 1, "Should find 1 user matching 'Smith'");
    println!("✓ Search by specific name works");
    
    // Test search with limit
    let results = repo.search("doe", Some(1), None).await?;
    assert_eq!(results.len(), 1, "Should only return 1 result with limit=1");
    println!("✓ Search with limit works");
    
    Ok(())
}

// Test for error handling
async fn test_error_handling(pool: &PgPool, _schema_name: &str) -> TestResult<()> {
    // Setup
    let repo = PostgresUserRepository::new(pool.clone());
    
    // Create a user
    let user1 = User {
        id: UserId::new(),
        email: Email::new("duplicate@example.com").map_err(|e| TestError::Database(e.to_string()))?,
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
    println!("✓ Saved first user");
    
    // Try to create another user with same email but different ID
    let user2 = User {
        id: UserId::new(), // Different ID
        email: Email::new("duplicate@example.com").map_err(|e| TestError::Database(e.to_string()))?, // Same email
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
    assert!(result.is_err(), "Should fail to save user with duplicate email");
    println!("✓ Duplicate email constraint works");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running user repository tests...");

    // Setup database
    let (pool, schema_name) = setup_test_db().await;
    
    // Run tests
    println!("\n🔍 Testing CRUD operations...");
    test_crud_operations(&pool, &schema_name).await
        .map_err(|e| format!("CRUD test failed: {:?}", e))?;
    
    println!("\n🔍 Testing pagination...");
    test_list_all_with_pagination(&pool, &schema_name).await
        .map_err(|e| format!("Pagination test failed: {:?}", e))?;
    
    println!("\n🔍 Testing search...");
    test_search_users(&pool, &schema_name).await
        .map_err(|e| format!("Search test failed: {:?}", e))?;
    
    println!("\n🔍 Testing error handling...");
    test_error_handling(&pool, &schema_name).await
        .map_err(|e| format!("Error handling test failed: {:?}", e))?;
    
    // Clean up
    teardown(&pool, &schema_name).await;
    
    println!("\n✅ All tests passed successfully!");
    Ok(())
}
