use sqlx::postgres::{PgPoolOptions, PgPool};
use tracing::{instrument, info, error};
use std::time::Duration;

#[derive(Clone)]
pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    #[instrument(level = "debug", name = "database.new", skip_all)]
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url)
            .await?;

        info!("Database connection pool established with {} max connections", max_connections);
        
        Ok(Self { pool })
    }
    
    #[instrument(level = "debug", name = "database.pool", skip_all)]
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    
    #[instrument(level = "debug", name = "database.check_health", skip_all)]
    pub async fn check_health(&self) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await;
            
        match result {
            Ok(_) => {
                info!("Database health check successful");
                Ok(true)
            },
            Err(err) => {
                tracing::error!("Database health check failed: {:?}", err);
                Err(err)
            }
        }
    }
    
    #[instrument(level = "info", name = "database.run_migrations", skip_all)]
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        info!("Running database migrations");
        
        match sqlx::migrate!("./migrations").run(&self.pool).await {
            Ok(_) => {
                info!("Database migrations completed successfully");
                Ok(())
            },
            Err(err) => {
                error!("Failed to run migrations: {}", err);
                // Convert MigrateError to sqlx::Error with Box
                Err(sqlx::Error::Migrate(Box::new(err)))
            }
        }
    }
    
    pub async fn close(self) {
        info!("Closing database connection pool");
        self.pool.close().await;
    }
}
