// Database migration binary
// Separate binary for running database migrations

use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("🔄 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("🔄 Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("✅ Database migrations completed successfully!");
    
    Ok(())
}
