// Database infrastructure - PostgreSQL with SQLx
pub mod manager;

// Re-export the manager for easier imports
pub use self::manager::DatabaseManager;
