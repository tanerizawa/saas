//! In-memory user repository test runner
//! This is a standalone binary to run tests for the in-memory user repository

use chrono::Utc;
use std::fmt;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use async_trait::async_trait;

// Define local versions of the domain entities for this standalone binary
#[derive(Debug, Clone)]
enum UserRole {
    UmkmOwner,
    AdminStaff,
    SuperAdmin,
}

#[derive(Debug, Clone)]
enum UserStatus {
    Active,
    Inactive,
    PendingVerification,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserId(Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    fn as_str(&self) -> String {
        self.0.to_string()
    }
    
    fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Email(String);

impl Email {
    fn new(email: &str) -> Result<Self, String> {
        // Simple validation
        if email.contains('@') {
            Ok(Self(email.to_string()))
        } else {
            Err("Invalid email format".to_string())
        }
    }
    
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct User {
    id: UserId,
    email: Email,
    password_hash: String,
    full_name: String,
    phone: Option<String>,
    role: UserRole,
    status: UserStatus,
    email_verified_at: Option<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

#[derive(Debug)]
enum AppError {
    NotFound,
    Database(String),
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl Error for AppError {}

// Define the UserRepository trait
#[async_trait]
trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), AppError>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError>;
    async fn delete(&self, id: &UserId) -> Result<(), AppError>;
    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>, AppError>;
    async fn count_all(&self) -> Result<i64, AppError>;
    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>, AppError>;
}

// Define the in-memory repository implementation
struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> Result<(), AppError> {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id.as_str(), user.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>, AppError> {
        let users = self.users.lock().unwrap();
        Ok(users.get(&id.as_str()).cloned())
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, AppError> {
        let users = self.users.lock().unwrap();
        let email_str = email.as_str();
        
        for user in users.values() {
            if user.email.as_str() == email_str {
                return Ok(Some(user.clone()));
            }
        }
        
        Ok(None)
    }

    async fn delete(&self, id: &UserId) -> Result<(), AppError> {
        let mut users = self.users.lock().unwrap();
        users.remove(&id.as_str());
        Ok(())
    }

    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>, AppError> {
        let users = self.users.lock().unwrap();
        let offset_val = offset.unwrap_or(0) as usize;
        
        let result: Vec<User> = users.values()
            .skip(offset_val)
            .take(limit.map(|l| l as usize).unwrap_or(users.len()))
            .cloned()
            .collect();
            
        Ok(result)
    }

    async fn count_all(&self) -> Result<i64, AppError> {
        let users = self.users.lock().unwrap();
        Ok(users.len() as i64)
    }

    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<User>, AppError> {
        let users = self.users.lock().unwrap();
        let offset_val = offset.unwrap_or(0) as usize;
        let query_lower = query.to_lowercase();
        
        let result: Vec<User> = users.values()
            .filter(|user| {
                user.full_name.to_lowercase().contains(&query_lower) || 
                user.email.as_str().to_lowercase().contains(&query_lower)
            })
            .skip(offset_val)
            .take(limit.map(|l| l as usize).unwrap_or(users.len()))
            .cloned()
            .collect();
            
        Ok(result)
    }
}

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn test_basic_operations(repo: &dyn UserRepository) -> TestResult<()> {
    println!("✓ Save user successful");
    
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
    
    // Save user
    repo.save(&user).await?;
    
    // Find by ID
    let found = repo.find_by_id(&user_id).await?;
    assert!(found.is_some());
    assert_eq!(found.unwrap().full_name, "Test User");
    println!("✓ Find by ID successful");
    
    // Find by email
    let found = repo.find_by_email(&email).await?;
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user_id);
    println!("✓ Find by email successful");
    
    // Update user
    let mut updated_user = user.clone();
    updated_user.full_name = "Updated User".to_string();
    repo.save(&updated_user).await?;
    
    let found = repo.find_by_id(&user_id).await?;
    assert_eq!(found.unwrap().full_name, "Updated User");
    println!("✓ Update user successful");
    
    // Delete user
    repo.delete(&user_id).await?;
    let found = repo.find_by_id(&user_id).await?;
    assert!(found.is_none());
    println!("✓ Delete user successful");
    
    Ok(())
}

async fn test_pagination_and_search(repo: &dyn UserRepository) -> TestResult<()> {
    // Create multiple test users
    for i in 0..10 {
        let user = User {
            id: UserId::new(),
            email: Email::new(&format!("test{}@example.com", i)).unwrap(),
            password_hash: format!("password{}", i),
            full_name: format!("Test User {}", i),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        repo.save(&user).await?;
    }
    println!("✓ Created 10 test users");
    
    // Test list with limit
    let users = repo.list_all(Some(5), None).await?;
    println!("✓ List with limit: Expected 5, got {}", users.len());
    assert_eq!(users.len(), 5);
    
    // Test list with limit and offset
    let users = repo.list_all(Some(3), Some(5)).await?;
    println!("✓ List with limit and offset: Expected 3, got {}", users.len());
    assert_eq!(users.len(), 3);
    
    // Test count
    let count = repo.count_all().await?;
    println!("✓ Count all: Expected 10, got {}", count);
    assert_eq!(count, 10);
    
    // Test search
    let users = repo.search("test3", None, None).await?;
    println!("✓ Search for 'test3': Expected 1, got {}", users.len());
    assert_eq!(users.len(), 1);
    
    // Test search with limit
    let users = repo.search("test", Some(2), None).await?;
    println!("✓ Search with limit: Expected 2, got {}", users.len());
    assert_eq!(users.len(), 2);
    
    Ok(())
}

#[tokio::main]
async fn main() -> TestResult<()> {
    println!("Starting in-memory user repository test...\n");
    
    let repo = InMemoryUserRepository::new();
    
    println!("Testing basic operations:");
    test_basic_operations(&repo).await?;
    
    println!("\nTesting pagination and search:");
    let repo = InMemoryUserRepository::new(); // Use a fresh repository
    test_pagination_and_search(&repo).await?;
    
    println!("\nAll tests passed successfully!");
    
    Ok(())
}
