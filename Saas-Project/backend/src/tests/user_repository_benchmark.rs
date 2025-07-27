#[cfg(test)]
mod user_repository_benchmarks {
    use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
    use tokio::runtime::Runtime;
    use sqlx::PgPool;
    use uuid::Uuid;
    
    use crate::domain::entities::{User, UserRole, UserStatus};
    use crate::domain::repositories::UserRepository;
    use crate::domain::value_objects::{Email, UserId};
    use crate::infrastructure::repositories::PostgresUserRepository;
    
    // Setup benchmark database
    async fn setup_benchmark_db() -> (PgPool, String) {
        // Use a unique schema for benchmark isolation
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
        
        // Close the connection pool to prevent potential memory leaks
        pool.close().await;
    }

    // Prepare test data
    async fn prepare_test_data(repo: &dyn UserRepository, count: usize) {
        for i in 0..count {
            let user_id = UserId::new();
            let email = Email::new(&format!("bench_user{}@example.com", i)).unwrap();
            
            let user = User {
                id: user_id,
                email,
                password_hash: "benchmark_password".to_string(),
                full_name: format!("Benchmark User {}", i),
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
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            repo.save(&user).await.unwrap();
        }
    }

    // Benchmark for finding users by ID
    fn bench_find_by_id(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        
        let (pool, schema_name) = rt.block_on(async {
            let (pool, schema) = setup_benchmark_db().await;
            let repo = PostgresUserRepository::new(pool.clone());
            
            // Create a test user
            let user_id = UserId::new();
            let email = Email::new("bench_find_id@example.com").unwrap();
            
            let user = User {
                id: user_id.clone(),
                email,
                password_hash: "benchmark_password".to_string(),
                full_name: "Find By ID Benchmark".to_string(),
                phone: None,
                role: UserRole::UmkmOwner,
                status: UserStatus::Active,
                email_verified_at: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            
            repo.save(&user).await.unwrap();
            
            (pool, schema)
        });
        
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Generate 20 random IDs that don't exist plus our one that does
        let mut user_ids = Vec::new();
        for _ in 0..20 {
            user_ids.push(UserId::new());
        }
        
        // Add our real ID
        let real_id = rt.block_on(async {
            let users = repo.list_all(Some(1), None).await.unwrap();
            users[0].id.clone()
        });
        user_ids.push(real_id.clone());
        
        // Benchmark finding users by ID
        let mut group = c.benchmark_group("user_repository_find_by_id");
        
        for id in user_ids {
            group.bench_with_input(BenchmarkId::from_parameter(id.to_string()), &id, |b, id| {
                b.iter(|| {
                    rt.block_on(async {
                        let _ = repo.find_by_id(id).await;
                    })
                });
            });
        }
        
        group.finish();
        
        // Clean up
        rt.block_on(async {
            teardown(&pool, &schema_name).await;
        });
    }

    // Benchmark for listing users with pagination
    fn bench_list_all(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        
        let (pool, schema_name) = rt.block_on(async {
            let (pool, schema) = setup_benchmark_db().await;
            let repo = PostgresUserRepository::new(pool.clone());
            
            // Create 100 test users for pagination benchmarks
            prepare_test_data(&repo, 100).await;
            
            (pool, schema)
        });
        
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Define pagination parameters to test
        let pagination_cases = vec![
            (Some(10), None),    // First 10 users
            (Some(20), None),    // First 20 users
            (Some(50), None),    // First 50 users
            (Some(10), Some(0)), // First 10 users explicitly
            (Some(10), Some(10)), // Second 10 users
            (Some(10), Some(50)), // Middle 10 users
            (Some(100), None),   // All users
        ];
        
        // Benchmark listing users with different pagination
        let mut group = c.benchmark_group("user_repository_list_all");
        
        for (i, (limit, offset)) in pagination_cases.iter().enumerate() {
            let case_name = match (limit, offset) {
                (Some(l), None) => format!("first_{}", l),
                (Some(l), Some(o)) => format!("limit_{}_offset_{}", l, o),
                _ => format!("case_{}", i),
            };
            
            group.bench_with_input(BenchmarkId::from_parameter(case_name), &(limit, offset), |b, (limit, offset)| {
                b.iter(|| {
                    rt.block_on(async {
                        let _ = repo.list_all(*limit, *offset).await;
                    })
                });
            });
        }
        
        group.finish();
        
        // Clean up
        rt.block_on(async {
            teardown(&pool, &schema_name).await;
        });
    }

    // Benchmark for searching users
    fn bench_search_users(c: &mut Criterion) {
        let rt = Runtime::new().unwrap();
        
        let (pool, schema_name) = rt.block_on(async {
            let (pool, schema) = setup_benchmark_db().await;
            let repo = PostgresUserRepository::new(pool.clone());
            
            // Create 100 test users
            prepare_test_data(&repo, 100).await;
            
            // Create specific test users for search benchmarks
            let special_users = vec![
                (UserId::new(), "john.smith@example.com", "John Smith"),
                (UserId::new(), "jane.smith@example.com", "Jane Smith"),
                (UserId::new(), "john.doe@example.com", "John Doe"),
                (UserId::new(), "jane.doe@example.com", "Jane Doe"),
                (UserId::new(), "unique.person@example.com", "Unique Person"),
            ];
            
            for (id, email, name) in special_users {
                let user = User {
                    id,
                    email: Email::new(email).unwrap(),
                    password_hash: "benchmark_password".to_string(),
                    full_name: name.to_string(),
                    phone: None,
                    role: UserRole::UmkmOwner,
                    status: UserStatus::Active,
                    email_verified_at: None,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                
                repo.save(&user).await.unwrap();
            }
            
            (pool, schema)
        });
        
        let repo = PostgresUserRepository::new(pool.clone());
        
        // Define search cases to benchmark
        let search_cases = vec![
            ("john", None, None),       // Common name
            ("smith", None, None),      // Common surname
            ("unique", None, None),     // Rare term
            ("nonexistent", None, None), // No matches
            ("user", Some(10), None),   // Many matches with limit
            ("doe", Some(1), None),     // Few matches with small limit
        ];
        
        // Benchmark searching users
        let mut group = c.benchmark_group("user_repository_search");
        
        for (i, (query, limit, offset)) in search_cases.iter().enumerate() {
            let case_name = format!("search_{}", query);
            
            group.bench_with_input(BenchmarkId::from_parameter(case_name), &(query, limit, offset), |b, (query, limit, offset)| {
                b.iter(|| {
                    rt.block_on(async {
                        let _ = repo.search(query, *limit, *offset).await;
                    })
                });
            });
        }
        
        group.finish();
        
        // Clean up
        rt.block_on(async {
            teardown(&pool, &schema_name).await;
        });
    }

    criterion_group!(
        benches,
        bench_find_by_id,
        bench_list_all,
        bench_search_users
    );
    criterion_main!(benches);
}
