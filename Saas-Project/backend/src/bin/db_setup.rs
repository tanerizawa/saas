//! Database setup utility
//! This utility checks if the database is available and sets up the required tables

use sqlx::postgres::PgPoolOptions;
use std::env;
use std::process::exit;
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    println!("Database setup utility");
    println!("=====================");

    // Get database URL from environment or use a default
    let database_url = env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        println!("Using default test database connection: postgres://saas_user:saas_password@localhost:5432/saas_test_db");
        "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string()
    });

    println!("Connecting to database...");
    
    // Try to connect to the database
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Successfully connected to the database!");
            pool
        },
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            eprintln!("Please make sure the database is running and accessible.");
            eprintln!("If using Docker, ensure the PostgreSQL container is running.");
            eprintln!("You may need to run: docker-compose up -d db");
            exit(1);
        }
    };

    // Create a unique test schema
    let schema_name = format!("test_{}", Uuid::new_v4().as_simple());
    println!("Creating test schema: {}", schema_name);

    match sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
        .execute(&pool)
        .await
    {
        Ok(_) => println!("Schema created successfully!"),
        Err(e) => {
            eprintln!("Failed to create schema: {}", e);
            exit(1);
        }
    }

    // Set the search path to our new schema
    match sqlx::query(&format!("SET search_path TO {}", schema_name))
        .execute(&pool)
        .await
    {
        Ok(_) => println!("Search path set to {}", schema_name),
        Err(e) => {
            eprintln!("Failed to set search path: {}", e);
            exit(1);
        }
    }

    // Create test tables
    println!("Creating test tables...");
    
    match sqlx::query(
        r#"
        CREATE TABLE users (
            id UUID PRIMARY KEY,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            full_name TEXT NOT NULL,
            phone TEXT,
            role TEXT NOT NULL,
            status TEXT NOT NULL,
            email_verified_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    {
        Ok(_) => println!("Created users table successfully!"),
        Err(e) => {
            eprintln!("Failed to create users table: {}", e);
            exit(1);
        }
    }

    println!("\nDatabase setup completed successfully!");
    println!("Connection string: {}", database_url);
    println!("Test schema: {}", schema_name);

    println!("\nYou can now run your tests with:");
    println!("export TEST_DATABASE_URL=\"{}\"", database_url);
    println!("export TEST_SCHEMA=\"{}\"", schema_name);
    println!("cargo test");

    // Close the pool
    pool.close().await;
}
