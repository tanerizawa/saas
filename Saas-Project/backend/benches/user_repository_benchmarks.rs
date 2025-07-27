use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use sqlx::PgPool;
use tokio::runtime::Runtime;
use uuid::Uuid;
use chrono::Utc;

use saas_umkm_backend::domain::entities::{User, UserRole, UserStatus};
use saas_umkm_backend::domain::repositories::UserRepository;
use saas_umkm_backend::domain::value_objects::{Email, UserId};
use saas_umkm_backend::infrastructure::repositories::PostgresUserRepository;

// Setup test database for benchmarking
async fn setup_benchmark_db() -> (PgPool, String) {
    let schema_name = format!("bench_{}", Uuid::new_v4().as_simple());
    
    // Get database URL from environment or use a default test database
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string());

    // Connect to the test database
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Create a new schema for this benchmark
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

// Teardown function to clean up benchmark resources
async fn teardown(pool: &PgPool, schema_name: &str) {
    // Drop the schema to clean up
    sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
        .execute(pool)
        .await
        .unwrap();
    
    // Close the connection pool
    pool.close().await;
}

// Create a function to generate test users
fn create_test_user(index: usize) -> User {
    let user_id = UserId::new();
    let email = Email::new(&format!("bench{}@example.com", index)).unwrap();
    
    User {
        id: user_id,
        email,
        password_hash: "benchmark_password_hash".to_string(),
        full_name: format!("Benchmark User {}", index),
        phone: None,
        role: if index % 3 == 0 { 
            UserRole::UmkmOwner 
        } else if index % 3 == 1 { 
            UserRole::AdminStaff 
        } else { 
            UserRole::SuperAdmin 
        },
        status: if index % 2 == 0 { 
            UserStatus::Active 
        } else { 
            UserStatus::PendingVerification 
        },
        email_verified_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

// Benchmark for user creation
fn benchmark_user_save(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    // Create a new benchmark group
    let mut group = c.benchmark_group("user_repository_save");
    
    // Run the async setup in the runtime
    let (pool, schema_name) = rt.block_on(setup_benchmark_db());
    
    // Create repository
    let repo = PostgresUserRepository::new(pool.clone());
    
    // Benchmark user creation
    group.bench_function("save_new_user", |b| {
        let mut index = 0;
        
        b.iter(|| {
            // Create a new user for each iteration
            let user = create_test_user(index);
            index += 1;
            
            // Execute the save operation
            rt.block_on(async {
                let _ = repo.save(&user).await.unwrap();
            });
        });
    });
    
    // Benchmark user update (save existing)
    group.bench_function("update_existing_user", |b| {
        // First create a user that we'll update repeatedly
        let user_id = UserId::new();
        let email = Email::new("update_benchmark@example.com").unwrap();
        
        let mut user = User {
            id: user_id.clone(),
            email,
            password_hash: "update_benchmark_hash".to_string(),
            full_name: "Update Benchmark User".to_string(),
            phone: None,
            role: UserRole::UmkmOwner,
            status: UserStatus::Active,
            email_verified_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Create the user initially
        rt.block_on(async {
            let _ = repo.save(&user).await.unwrap();
        });
        
        // Now benchmark updates to this user
        let mut counter = 0;
        b.iter(|| {
            counter += 1;
            user.full_name = format!("Updated User {}", counter);
            user.updated_at = Utc::now();
            
            rt.block_on(async {
                let _ = repo.save(&user).await.unwrap();
            });
        });
    });
    
    group.finish();
    
    // Benchmark for search operations
    let mut search_group = c.benchmark_group("user_repository_search");
    
    // First, create a dataset for search
    rt.block_on(async {
        for i in 0..100 {
            let user = create_test_user(i);
            let _ = repo.save(&user).await.unwrap();
        }
    });
    
    // Benchmark different search patterns
    for term in ["bench", "User", "Admin", "Owner"].iter() {
        search_group.bench_with_input(
            BenchmarkId::new("search_by_term", term), 
            term,
            |b, term| {
                b.iter(|| {
                    rt.block_on(async {
                        let _ = repo.search(term, Some(10), None).await.unwrap();
                    });
                });
            }
        );
    }
    
    search_group.finish();
    
    // Clean up after benchmarks
    rt.block_on(teardown(&pool, &schema_name));
}

criterion_group!(benches, benchmark_user_save);
criterion_main!(benches);
