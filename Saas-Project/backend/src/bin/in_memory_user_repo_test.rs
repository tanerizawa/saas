//! In-memory user repository test
//! This test will simulate a database in memory

use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use std::error::Error;

// User entity
#[derive(Clone, Debug, PartialEq)]
struct UserId(String);

impl UserId {
    fn new() -> Self {
        Self(format!("user-{}", uuid::Uuid::new_v4()))
    }
}

// User roles
#[derive(Clone, Debug, PartialEq)]
enum UserRole {
    AdminStaff,
    SuperAdmin,
    UmkmOwner,
}

// User status
#[derive(Clone, Debug, PartialEq)]
enum UserStatus {
    Inactive,
    Active,
    PendingVerification,
    Suspended,
}

// Email value object
#[derive(Clone, Debug, PartialEq)]
struct Email(String);

impl Email {
    fn new(email: &str) -> Result<Self, Box<dyn Error>> {
        // Very basic validation
        if !email.contains('@') || !email.contains('.') {
            return Err("Invalid email format".into());
        }
        Ok(Self(email.to_string()))
    }
    
    fn as_str(&self) -> &str {
        &self.0
    }
}

// User entity
#[derive(Clone, Debug)]
struct User {
    id: UserId,
    email: Email,
    password_hash: String,
    full_name: String,
    role: UserRole,
    status: UserStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

// Using chrono without making it too complex
use chrono::DateTime;

// Generic result type for this test
type TestResult<T> = Result<T, Box<dyn Error>>;

// User Repository trait
#[async_trait]
trait UserRepository {
    async fn save(&self, user: &User) -> TestResult<()>;
    async fn delete(&self, id: &UserId) -> TestResult<()>;
    async fn find_by_id(&self, id: &UserId) -> TestResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> TestResult<Option<User>>;
    async fn list_all(&self, limit: Option<i64>, offset: Option<i64>) -> TestResult<Vec<User>>;
    async fn count_all(&self) -> TestResult<i64>;
    async fn search(&self, query: &str, limit: Option<i64>, offset: Option<i64>) -> TestResult<Vec<User>>;
}

// In-memory implementation
struct InMemoryUserRepository {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    fn with_users(users: Vec<User>) -> Self {
        let mut map = HashMap::new();
        for user in users {
            map.insert(user.id.0.clone(), user);
        }
        Self {
            users: Arc::new(Mutex::new(map)),
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: &User) -> TestResult<()> {
        let mut users = self.users.lock().unwrap();
        
        // Check for duplicate email (except for the same user id)
        if users.values().any(|u| u.email.0 == user.email.0 && u.id.0 != user.id.0) {
            return Err("Email already exists".into());
        }
        
        users.insert(user.id.0.clone(), user.clone());
        Ok(())
    }
    
    async fn delete(&self, id: &UserId) -> TestResult<()> {
        let mut users = self.users.lock().unwrap();
        users.remove(&id.0);
        Ok(())
    }
    
    async fn find_by_id(&self, id: &UserId) -> TestResult<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.get(&id.0).cloned())
    }
    
    async fn find_by_email(&self, email: &Email) -> TestResult<Option<User>> {
        let users = self.users.lock().unwrap();
        let user = users.values().find(|u| u.email.0 == email.0).cloned();
        Ok(user)
    }
    
    async fn list_all(&self, limit: Option<i64>, offset: Option<i64>) -> TestResult<Vec<User>> {
        let users = self.users.lock().unwrap();
        
        // Convert to vec and sort by created_at desc
        let mut all_users: Vec<User> = users.values().cloned().collect();
        all_users.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Apply offset
        let offset = offset.unwrap_or(0) as usize;
        if offset >= all_users.len() {
            return Ok(vec![]);
        }
        
        let users_after_offset = &all_users[offset..];
        
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
    
    async fn count_all(&self) -> TestResult<i64> {
        let users = self.users.lock().unwrap();
        Ok(users.len() as i64)
    }
    
    async fn search(&self, query: &str, limit: Option<i64>, offset: Option<i64>) -> TestResult<Vec<User>> {
        let users = self.users.lock().unwrap();
        let query = query.to_lowercase();
        
        // Filter users where email or name contains the query
        let mut matching_users: Vec<User> = users
            .values()
            .filter(|u| {
                u.email.0.to_lowercase().contains(&query) || 
                u.full_name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();
        
        // Sort by created_at desc
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

// Main function to run tests
#[tokio::main]
async fn main() {
    println!("Starting in-memory user repository test...\n");
    
    // Test basic operations
    println!("Testing basic operations:");
    if let Err(e) = test_basic_operations().await {
        eprintln!("Error in basic operations test: {}", e);
        return;
    }
    
    // Test pagination and search
    println!("\nTesting pagination and search:");
    if let Err(e) = test_pagination_and_search().await {
        eprintln!("Error in pagination and search test: {}", e);
        return;
    }
    
    println!("\nAll tests passed successfully!");
}

// Test basic CRUD operations
async fn test_basic_operations() -> TestResult<()> {
    let repo = InMemoryUserRepository::new();
    
    // Create user
    let user_id = UserId::new();
    let email = Email::new("test@example.com")?;
    
    let user = User {
        id: user_id.clone(),
        email: email.clone(),
        password_hash: "hashed_password".to_string(),
        full_name: "Test User".to_string(),
        role: UserRole::UmkmOwner,
        status: UserStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Save
    repo.save(&user).await?;
    println!("✓ Save user successful");
    
    // Find by ID
    let found_by_id = repo.find_by_id(&user_id).await?;
    assert!(found_by_id.is_some());
    assert_eq!(found_by_id.unwrap().full_name, "Test User");
    println!("✓ Find by ID successful");
    
    // Find by email
    let found_by_email = repo.find_by_email(&email).await?;
    assert!(found_by_email.is_some());
    assert_eq!(found_by_email.unwrap().id.0, user_id.0);
    println!("✓ Find by email successful");
    
    // Update
    let mut updated_user = user.clone();
    updated_user.full_name = "Updated User".to_string();
    repo.save(&updated_user).await?;
    
    let found_after_update = repo.find_by_id(&user_id).await?;
    assert!(found_after_update.is_some());
    assert_eq!(found_after_update.unwrap().full_name, "Updated User");
    println!("✓ Update user successful");
    
    // Delete
    repo.delete(&user_id).await?;
    
    let found_after_delete = repo.find_by_id(&user_id).await?;
    assert!(found_after_delete.is_none());
    println!("✓ Delete user successful");
    
    Ok(())
}

// Test pagination and search
async fn test_pagination_and_search() -> TestResult<()> {
    // Create 10 test users
    let mut users = Vec::new();
    for i in 0..10 {
        let user = User {
            id: UserId::new(),
            email: Email::new(&format!("test{}@example.com", i))?,
            password_hash: "hashed_password".to_string(),
            full_name: format!("Test User {}", i),
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        users.push(user);
    }
    
    let repo = InMemoryUserRepository::with_users(users);
    println!("✓ Created 10 test users");
    
    // Test list with limit
    let list_limited = repo.list_all(Some(5), None).await?;
    assert_eq!(list_limited.len(), 5);
    println!("✓ List with limit: Expected 5, got {}", list_limited.len());
    
    // Test list with limit and offset
    let list_offset = repo.list_all(Some(3), Some(5)).await?;
    assert_eq!(list_offset.len(), 3);
    println!("✓ List with limit and offset: Expected 3, got {}", list_offset.len());
    
    // Test count all
    let count = repo.count_all().await?;
    assert_eq!(count, 10);
    println!("✓ Count all: Expected 10, got {}", count);
    
    // Test search
    let search_results = repo.search("test3", None, None).await?;
    assert_eq!(search_results.len(), 1);
    println!("✓ Search for 'test3': Expected 1, got {}", search_results.len());
    
    // Test search with limit
    let search_limited = repo.search("test", Some(2), None).await?;
    assert_eq!(search_limited.len(), 2);
    println!("✓ Search with limit: Expected 2, got {}", search_limited.len());
    
    Ok(())
}
