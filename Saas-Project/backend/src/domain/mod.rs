// Domain module - Core business logic (Domain-Driven Design)
// This module contains the core business entities, value objects, and domain services
// following the principles outlined in the architecture document

pub mod business;
pub mod companies;
pub mod dto;
pub mod entities;
pub mod errors;
pub mod events;
pub mod finance;
pub mod licenses;
pub mod licensing;
pub mod repositories;
pub mod users;
pub mod value_objects;

// Re-export commonly used items
// Re-exports from entities module
// pub use entities::{User as EntityUser}; // Commented out - not used yet
