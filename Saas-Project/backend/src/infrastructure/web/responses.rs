// Standard HTTP responses
#![allow(dead_code)]

use serde_json::{json, Value};

pub fn success_response(data: Value) -> Value {
    json!({
        "success": true,
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339()
    })
}

pub fn error_response(message: &str, code: &str) -> Value {
    json!({
        "success": false,
        "error": {
            "code": code,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }
    })
}
