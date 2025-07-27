// Database migration binary
// Separate binary for running database migrations

use sqlx::postgres::PgPoolOptions;
use sqlx::Row; // Added for row.get method
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("up");

    match command {
        "up" => run_migrations(&database_url).await?,
        "down" => {
            let version = args.get(2);
            rollback_migrations(&database_url, version).await?
        },
        "status" => show_status(&database_url).await?,
        _ => {
            println!("Unknown command: {}", command);
            println!("Available commands: up, down, status");
            return Ok(());
        }
    }

    println!("✅ Migration command completed successfully!");
    
    Ok(())
}

async fn run_migrations(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("🔄 Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("✅ Database migrations completed successfully!");
    Ok(())
}

async fn rollback_migrations(database_url: &str, version: Option<&String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    let migrator = sqlx::migrate!("./migrations");
    
    if let Some(target_version) = version {
        println!("🔄 Rolling back migrations to version {}...", target_version);
        // TODO: Implement targeted rollback when sqlx supports it better
        // For now we'll roll back one step at a time
        println!("⚠️  Rolling back to specific version not yet supported. Rolling back one step.");
        migrator.undo(&pool, 1).await?;
    } else {
        println!("🔄 Rolling back last migration...");
        migrator.undo(&pool, 1).await?;
    }

    println!("✅ Rollback completed successfully!");
    Ok(())
}

async fn show_status(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Connecting to database...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    println!("📊 Checking migration status...");
    
    // Read migration files
    let migrations_dir = std::path::Path::new("./migrations");
    let migration_files: Vec<_> = std::fs::read_dir(migrations_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if let Ok(file_type) = entry.file_type() {
                file_type.is_file() && entry.file_name().to_string_lossy().ends_with(".sql")
            } else {
                false
            }
        })
        .collect();
    
    // Get applied migrations from database
    let query = "SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY installed_on";
    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await?;
    
    println!("\nMigration Status:");
    println!("=================");
    println!("{:<20} {:<40} {:<20} {:<10}", "Version", "Description", "Applied At", "Status");
    println!("{:<20} {:<40} {:<20} {:<10}", "-------", "-----------", "----------", "------");
    
    // Display migration status
    for file in &migration_files {
        let file_name = file.file_name().to_string_lossy();
        
        if !file_name.contains("_") {
            continue;
        }
        
        let version = file_name.split('_').next().unwrap_or("unknown");
        let description = file_name
            .trim_start_matches(&format!("{}_", version))
            .trim_end_matches(".sql");
        
        // Check if migration is applied by searching rows
        let is_applied = rows.iter().any(|row| {
            let db_version: &str = row.get("version");
            db_version == version
        });
        
        println!("{:<20} {:<40} {:<20} {:<10}", 
            version, 
            description, 
            if is_applied { "Yes" } else { "No" },
            if is_applied { "✅" } else { "❌" }
        );
    }
    
    println!("\nTotal: {} migrations found in directory", migration_files.len());
    
    Ok(())
}
