use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::infrastructure::web::AppState;
use crate::services::system_config::{SystemConfigService, ConfigValue, ConfigGroup};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConfigUpdateRequest {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GroupUpdateRequest {
    pub updates: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ConfigImportRequest {
    pub config_data: String,
}

/// Get all configuration groups
/// 
/// Returns all system configuration groups with their settings
#[utoipa::path(
    get,
    path = "/api/v1/system/config/groups",
    responses(
        (status = 200, description = "Configuration groups retrieved"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn get_all_config_groups(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    match config_service.get_all_groups().await {
        Ok(groups) => {
            let response = serde_json::json!({
                "groups": groups,
                "total": groups.len(),
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get configuration groups: {}", e);
            let error_response = serde_json::json!({
                "error": "Configuration Retrieval Failed",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

/// Get configuration group by name
/// 
/// Returns a specific configuration group with all its settings
#[utoipa::path(
    get,
    path = "/api/v1/system/config/groups/{group_name}",
    params(
        ("group_name" = String, Path, description = "Configuration group name")
    ),
    responses(
        (status = 200, description = "Configuration group retrieved"),
        (status = 404, description = "Group not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn get_config_group(
    State(_state): State<AppState>,
    Path(group_name): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    match config_service.get_group(&group_name).await {
        Ok(group) => {
            let response = serde_json::json!({
                "group": group,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get configuration group {}: {}", group_name, e);
            let error_response = serde_json::json!({
                "error": "Group Not Found",
                "message": e.to_string(),
                "status": 404,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/// Get configuration value
/// 
/// Returns the current value of a specific configuration setting
#[utoipa::path(
    get,
    path = "/api/v1/system/config/get/{key}",
    params(
        ("key" = String, Path, description = "Configuration key (e.g., 'general:platform_name')")
    ),
    responses(
        (status = 200, description = "Configuration value retrieved"),
        (status = 404, description = "Configuration key not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn get_config_value(
    State(_state): State<AppState>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    match config_service.get_string(&key).await {
        Ok(value) => {
            let response = serde_json::json!({
                "key": key,
                "value": value,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to get configuration value {}: {}", key, e);
            let error_response = serde_json::json!({
                "error": "Configuration Key Not Found",
                "message": e.to_string(),
                "status": 404,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/// Update configuration value
/// 
/// Updates a specific configuration setting
#[utoipa::path(
    put,
    path = "/api/v1/system/config/set",
    request_body = ConfigUpdateRequest,
    responses(
        (status = 200, description = "Configuration updated successfully"),
        (status = 400, description = "Invalid configuration value"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn update_config_value(
    State(_state): State<AppState>,
    Json(payload): Json<ConfigUpdateRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    // Convert JSON value to ConfigValue
    let config_value = match payload.value {
        serde_json::Value::String(s) => ConfigValue::String(s),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                ConfigValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                ConfigValue::Float(f)
            } else {
                let error_response = serde_json::json!({
                    "error": "Invalid Number",
                    "message": "Number value is not valid",
                    "status": 400,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                });
                return Err((StatusCode::BAD_REQUEST, Json(error_response)));
            }
        }
        serde_json::Value::Bool(b) => ConfigValue::Boolean(b),
        serde_json::Value::Array(arr) => {
            let string_array: Result<Vec<String>, _> = arr
                .into_iter()
                .map(|v| match v {
                    serde_json::Value::String(s) => Ok(s),
                    _ => Err("Array must contain only strings"),
                })
                .collect();
            
            match string_array {
                Ok(arr) => ConfigValue::Array(arr),
                Err(e) => {
                    let error_response = serde_json::json!({
                        "error": "Invalid Array",
                        "message": e,
                        "status": 400,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(error_response)));
                }
            }
        }
        _ => {
            let error_response = serde_json::json!({
                "error": "Unsupported Value Type",
                "message": "Only string, number, boolean, and string array values are supported",
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    match config_service.set(&payload.key, config_value).await {
        Ok(()) => {
            let response = serde_json::json!({
                "message": "Configuration updated successfully",
                "key": payload.key,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to update configuration {}: {}", payload.key, e);
            let error_response = serde_json::json!({
                "error": "Configuration Update Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

/// Update configuration group
/// 
/// Updates multiple configuration settings within a specific group
#[utoipa::path(
    put,
    path = "/api/v1/system/config/groups/{group_name}",
    params(
        ("group_name" = String, Path, description = "Configuration group name")
    ),
    request_body = GroupUpdateRequest,
    responses(
        (status = 200, description = "Configuration group updated successfully"),
        (status = 400, description = "Invalid configuration values"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn update_config_group(
    State(_state): State<AppState>,
    Path(group_name): Path<String>,
    Json(payload): Json<GroupUpdateRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    // Convert updates to ConfigValue format
    let mut config_updates = HashMap::new();
    for (key, value) in payload.updates {
        let config_value = match value {
            serde_json::Value::String(s) => ConfigValue::String(s),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    ConfigValue::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    ConfigValue::Float(f)
                } else {
                    let error_response = serde_json::json!({
                        "error": "Invalid Number",
                        "message": format!("Number value for key {} is not valid", key),
                        "status": 400,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    });
                    return Err((StatusCode::BAD_REQUEST, Json(error_response)));
                }
            }
            serde_json::Value::Bool(b) => ConfigValue::Boolean(b),
            _ => {
                let error_response = serde_json::json!({
                    "error": "Unsupported Value Type",
                    "message": format!("Value type for key {} is not supported", key),
                    "status": 400,
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                });
                return Err((StatusCode::BAD_REQUEST, Json(error_response)));
            }
        };
        config_updates.insert(key, config_value);
    }

    match config_service.update_group(&group_name, config_updates).await {
        Ok(()) => {
            let response = serde_json::json!({
                "message": "Configuration group updated successfully",
                "group": group_name,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to update configuration group {}: {}", group_name, e);
            let error_response = serde_json::json!({
                "error": "Group Update Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}

/// Reset configuration to default
/// 
/// Resets a configuration setting to its default value
#[utoipa::path(
    post,
    path = "/api/v1/system/config/reset/{key}",
    params(
        ("key" = String, Path, description = "Configuration key to reset")
    ),
    responses(
        (status = 200, description = "Configuration reset successfully"),
        (status = 404, description = "Configuration key not found"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn reset_config_to_default(
    State(_state): State<AppState>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    match config_service.reset_to_default(&key).await {
        Ok(()) => {
            let response = serde_json::json!({
                "message": "Configuration reset to default successfully",
                "key": key,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to reset configuration {}: {}", key, e);
            let error_response = serde_json::json!({
                "error": "Reset Failed",
                "message": e.to_string(),
                "status": 404,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

/// Export configuration
/// 
/// Exports the entire system configuration as JSON
#[utoipa::path(
    get,
    path = "/api/v1/system/config/export",
    responses(
        (status = 200, description = "Configuration exported successfully"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn export_config(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    match config_service.export_config().await {
        Ok(config_json) => {
            let response = serde_json::json!({
                "message": "Configuration exported successfully",
                "config": serde_json::from_str::<serde_json::Value>(&config_json).unwrap_or_default(),
                "exported_at": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to export configuration: {}", e);
            let error_response = serde_json::json!({
                "error": "Export Failed",
                "message": e.to_string(),
                "status": 500,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

/// Import configuration
/// 
/// Imports system configuration from JSON data
#[utoipa::path(
    post,
    path = "/api/v1/system/config/import",
    request_body = ConfigImportRequest,
    responses(
        (status = 200, description = "Configuration imported successfully"),
        (status = 400, description = "Invalid configuration data"),
        (status = 403, description = "Insufficient permissions")
    ),
    tag = "System Configuration"
)]
pub async fn import_config(
    State(_state): State<AppState>,
    Json(payload): Json<ConfigImportRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let config_service = SystemConfigService::new().await;

    match config_service.import_config(&payload.config_data).await {
        Ok(()) => {
            let response = serde_json::json!({
                "message": "Configuration imported successfully",
                "imported_at": chrono::Utc::now().to_rfc3339(),
            });

            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Failed to import configuration: {}", e);
            let error_response = serde_json::json!({
                "error": "Import Failed",
                "message": e.to_string(),
                "status": 400,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            Err((StatusCode::BAD_REQUEST, Json(error_response)))
        }
    }
}
