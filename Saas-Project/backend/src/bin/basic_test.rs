//! Simple standalone test to check if basic project functionality works

use chrono::Utc;
use uuid::Uuid;

fn main() {
    println!("Running standalone test...");
    
    // Test UUID generation
    let id = Uuid::new_v4();
    println!("Generated UUID: {}", id);
    
    // Test chrono
    let now = Utc::now();
    println!("Current time: {}", now);
    
    // Test basic functionality
    let value = "Hello, world!".to_string();
    println!("String value: {}", value);
    
    println!("All basic functionality works!");
}
