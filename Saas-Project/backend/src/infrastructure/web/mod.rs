// Web infrastructure - HTTP handlers and middleware
// Axum-based REST API implementation

pub mod handlers;
pub mod middleware;
pub mod responses;
pub mod routes;

// Re-export commonly used types
pub use handlers::{AppState, AppStateType};
