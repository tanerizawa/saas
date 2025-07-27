// Database infrastructure - PostgreSQL with SQLx
// Optimized configuration as per document recommendations

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::{info, instrument};

pub type DatabasePool = Pool<Postgres>;

#[derive(Clone)]
pub struct DatabaseConnection {
    pool: DatabasePool,
}

impl DatabaseConnection {
    #[instrument(skip(database_url))]
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        info!("🔗 Connecting to PostgreSQL database");

        // Connection pool optimized for PostgreSQL performance
        // Enhanced for Phase 4 with more optimal settings
        let pool = PgPoolOptions::new()
            .max_connections(25) // Increased for better throughput
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(300)) // Reduced to 5 minutes for better resource usage
            .max_lifetime(Duration::from_secs(1800)) // 30 minutes
            .test_before_acquire(true) // Ensure connections are valid before use
            .connect_lazy_with(Self::parse_pool_options(database_url)?);

        tracing::info!("✅ Database connection pool established with enhanced settings");

        Ok(Self { pool })
    }

    // Parse connection options to optimize PostgreSQL performance
    fn parse_pool_options(url: &str) -> Result<sqlx::postgres::PgConnectOptions, sqlx::Error> {
        use sqlx::postgres::PgConnectOptions;
        use sqlx::ConnectOptions;
        use std::str::FromStr;
        use tracing::log::LevelFilter;

        let options = PgConnectOptions::from_str(url)?;

        // Configure options that are available in current sqlx version
        let options = options
            .statement_cache_capacity(512)
            .application_name("saas-umkm-backend")
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Warn, Duration::from_secs(1));

        // Note: The following options were removed from newer sqlx versions:
        // - tcp_keepalives_idle
        // - tcp_keepalives_interval
        // - tcp_keepalives_retries
        // - connect_timeout
        // These are now handled by the underlying connection libraries

        Ok(options)
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        info!("🔄 Running database migrations");
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        info!("✅ Database migrations completed");
        Ok(())
    }

    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    // Health check for database connection
    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").fetch_one(&self.pool).await?;
        Ok(())
    }
}
