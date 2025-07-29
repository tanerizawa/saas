// This is a simple demo of the InMemoryUserRepository
// It doesn't require a real database connection

use chrono::Utc;
use std::error::Error;

// Import from the main crate
use saas_umkm_backend::domain::entities::{User, UserRole, UserStatus};
use saas_umkm_backend::domain::repositories::UserRepository;
use saas_umkm_backend::domain::value_objects::{Email, UserId};
use saas_umkm_backend::infrastructure::repositories::in_memory_user_repository::InMemoryUserRepository;

type DemoResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> DemoResult<()> {
    println!("Starting in-memory user repository demo...\n");

    // Create a new in-memory repository
    let repo = InMemoryUserRepository::new();
    
    // Test basic operations
    println!("Testing basic operations:");
    test_basic_operations(&repo).await?;
    
    // Test pagination and search
    println!("\nTesting pagination and search:");
    test_pagination_and_search().await?;
    
    println!("\nAll tests passed successfully!");
    
    Ok(())
}

async fn test_basic_operations(repo: &InMemoryUserRepository) -> DemoResult<()> {
    // Create a user
    let user_id = UserId::new();
    let email = Email::new("test@example.com")?;
    
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
    println!("✓ Save user successful");
    
    // Find by ID
    let found_by_id = repo.find_by_id(&user_id).await?;
    assert!(found_by_id.is_some());
    assert_eq!(found_by_id.unwrap().full_name, "Test User");
    println!("✓ Find by ID successful");
    
    // Find by email
    let found_by_email = repo.find_by_email(&email).await?;
    assert!(found_by_email.is_some());
    assert_eq!(found_by_email.unwrap().id, user_id);
    println!("✓ Find by email successful");
    
    // Update user
    let mut updated_user = user.clone();
    updated_user.full_name = "Updated User".to_string();
    repo.save(&updated_user).await?;
    
    let found_after_update = repo.find_by_id(&user_id).await?;
    assert!(found_after_update.is_some());
    assert_eq!(found_after_update.unwrap().full_name, "Updated User");
    println!("✓ Update user successful");
    
    // Delete user
    repo.delete(&user_id).await?;
    let found_after_delete = repo.find_by_id(&user_id).await?;
    assert!(found_after_delete.is_none());
    println!("✓ Delete user successful");
    
    Ok(())
}

async fn test_pagination_and_search() -> DemoResult<()> {
    // Create multiple users
    let mut users = Vec::new();
    for i in 0..10 {
        let user = User {
            id: UserId::new(),
            email: Email::new(&format!("test{}@example.com", i))?,
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
    println!("✓ Created 10 test users");
    
    // Test list with limit
    let list_limited = repo.list_all(Some(5), None).await?;
    assert_eq!(list_limited.len(), 5);
    println!("✓ List with limit: Expected 5, got {}", list_limited.len());
    
    // Test list with limit and offset
    let list_offset = repo.list_all(Some(3), Some(5)).await?;
    assert_eq!(list_offset.len(), 3);
    println!("✓ List with limit and offset: Expected 3, got {}", list_offset.len());
    
    // Test count
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
