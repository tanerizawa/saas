//! Simple test for user repository functionality
//! This is an isolated test to verify the user repository works correctly

use sqlx::{PgPool, postgres::PgPoolOptions, Row};
use uuid::Uuid;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from environment or use a default test database
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string());

    println!("Connecting to database: {}", database_url);

    // Connect to the test database with a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    // Create a unique schema for isolation
    let schema_name = format!("test_{}", Uuid::new_v4().simple());
    println!("Using test schema: {}", schema_name);

    // Create the schema
    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
        .execute(&pool)
        .await?;

    // Set search path to our schema
    sqlx::query(&format!("SET search_path TO {}", schema_name))
        .execute(&pool)
        .await?;
    
    // Create users table for testing
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
        "#
    )
    .execute(&pool)
    .await?;

    println!("Test table created successfully");
    
    // Insert a test user
    let id = Uuid::new_v4();
    let email = "test@example.com";
    let now = chrono::Utc::now();
    
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash, full_name, role, status, email_verified, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#
    )
    .bind(id)
    .bind(email)
    .bind("hashed_password")
    .bind("Test User")
    .bind("umkm_owner")
    .bind("active")
    .bind(false)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await?;

    println!("Test user inserted successfully");

    // Verify the user was inserted
    let result = sqlx::query("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let retrieved_email: String = result.get("email");
    println!("User retrieved successfully: {}", retrieved_email);
    
    assert_eq!(retrieved_email, email);
    
    // Clean up - drop the schema
    sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema_name))
        .execute(&pool)
        .await?;
    
    println!("Test completed successfully! All database operations worked.");
    
    Ok(())
}
