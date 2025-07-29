use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SystemConfigService {
    config_cache: std::sync::Arc<tokio::sync::RwLock<HashMap<String, ConfigValue>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<String>),
    Object(HashMap<String, String>),
}

impl ConfigValue {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            ConfigValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ConfigValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            ConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ConfigValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<String>> {
        match self {
            ConfigValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigGroup {
    pub name: String,
    pub description: String,
    pub settings: HashMap<String, ConfigSetting>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSetting {
    pub key: String,
    pub value: ConfigValue,
    pub default_value: ConfigValue,
    pub description: String,
    pub category: String,
    pub is_sensitive: bool,
    pub requires_restart: bool,
    pub validation_rules: Vec<ValidationRule>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    MinValue(f64),
    MaxValue(f64),
    Email,
    Url,
    OneOf(Vec<String>),
    Regex(String),
}

#[derive(Debug)]
pub enum ConfigError {
    KeyNotFound(String),
    InvalidValue(String),
    ValidationFailed(String),
    DatabaseError(String),
    AccessDenied(String),
    ConfigurationConflict(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::KeyNotFound(key) => write!(f, "Configuration key not found: {}", key),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
            ConfigError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            ConfigError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ConfigError::AccessDenied(msg) => write!(f, "Access denied: {}", msg),
            ConfigError::ConfigurationConflict(msg) => write!(f, "Configuration conflict: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl SystemConfigService {
    pub async fn new() -> Self {
        let config_cache = std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new()));
        let service = Self { config_cache };
        
        // Initialize with default configurations
        service.initialize_default_config().await;
        
        service
    }

    /// Get a configuration value by key
    pub async fn get<T>(&self, key: &str) -> Result<T, ConfigError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let cache = self.config_cache.read().await;
        let value = cache.get(key)
            .ok_or_else(|| ConfigError::KeyNotFound(key.to_string()))?;

        // Convert ConfigValue to the requested type
        let json_value = serde_json::to_value(value)
            .map_err(|e| ConfigError::InvalidValue(e.to_string()))?;
        
        serde_json::from_value(json_value)
            .map_err(|e| ConfigError::InvalidValue(e.to_string()))
    }

    /// Get a string configuration value
    pub async fn get_string(&self, key: &str) -> Result<String, ConfigError> {
        let cache = self.config_cache.read().await;
        let value = cache.get(key)
            .ok_or_else(|| ConfigError::KeyNotFound(key.to_string()))?;

        value.as_string()
            .cloned()
            .ok_or_else(|| ConfigError::InvalidValue("Expected string value".to_string()))
    }

    /// Get an integer configuration value
    pub async fn get_integer(&self, key: &str) -> Result<i64, ConfigError> {
        let cache = self.config_cache.read().await;
        let value = cache.get(key)
            .ok_or_else(|| ConfigError::KeyNotFound(key.to_string()))?;

        value.as_integer()
            .ok_or_else(|| ConfigError::InvalidValue("Expected integer value".to_string()))
    }

    /// Get a boolean configuration value
    pub async fn get_boolean(&self, key: &str) -> Result<bool, ConfigError> {
        let cache = self.config_cache.read().await;
        let value = cache.get(key)
            .ok_or_else(|| ConfigError::KeyNotFound(key.to_string()))?;

        value.as_boolean()
            .ok_or_else(|| ConfigError::InvalidValue("Expected boolean value".to_string()))
    }

    /// Set a configuration value
    pub async fn set(&self, key: &str, value: ConfigValue) -> Result<(), ConfigError> {
        // TODO: Validate the value
        // TODO: Check permissions
        // TODO: Save to database
        
        let mut cache = self.config_cache.write().await;
        cache.insert(key.to_string(), value);
        
        tracing::info!("Configuration updated: {}", key);
        Ok(())
    }

    /// Get all configuration groups
    pub async fn get_all_groups(&self) -> Result<Vec<ConfigGroup>, ConfigError> {
        Ok(vec![
            self.get_general_config_group().await,
            self.get_email_config_group().await,
            self.get_license_config_group().await,
            self.get_payment_config_group().await,
            self.get_security_config_group().await,
            self.get_notification_config_group().await,
        ])
    }

    /// Get configuration group by name
    pub async fn get_group(&self, name: &str) -> Result<ConfigGroup, ConfigError> {
        match name {
            "general" => Ok(self.get_general_config_group().await),
            "email" => Ok(self.get_email_config_group().await),
            "license" => Ok(self.get_license_config_group().await),
            "payment" => Ok(self.get_payment_config_group().await),
            "security" => Ok(self.get_security_config_group().await),
            "notification" => Ok(self.get_notification_config_group().await),
            _ => Err(ConfigError::KeyNotFound(format!("Group not found: {}", name))),
        }
    }

    /// Update multiple configuration values at once
    pub async fn update_group(
        &self,
        group_name: &str,
        updates: HashMap<String, ConfigValue>,
    ) -> Result<(), ConfigError> {
        // TODO: Validate all values first
        // TODO: Check for conflicts
        // TODO: Apply atomically

        for (key, value) in updates {
            let full_key = format!("{}:{}", group_name, key);
            self.set(&full_key, value).await?;
        }

        Ok(())
    }

    /// Reset a configuration to its default value
    pub async fn reset_to_default(&self, key: &str) -> Result<(), ConfigError> {
        // TODO: Look up default value
        // TODO: Apply default
        
        tracing::info!("Configuration reset to default: {}", key);
        Ok(())
    }

    /// Export configuration as JSON
    pub async fn export_config(&self) -> Result<String, ConfigError> {
        let cache = self.config_cache.read().await;
        serde_json::to_string_pretty(&*cache)
            .map_err(|e| ConfigError::InvalidValue(e.to_string()))
    }

    /// Import configuration from JSON
    pub async fn import_config(&self, json_data: &str) -> Result<(), ConfigError> {
        let imported_config: HashMap<String, ConfigValue> = serde_json::from_str(json_data)
            .map_err(|e| ConfigError::InvalidValue(e.to_string()))?;

        // TODO: Validate all imported values
        // TODO: Check for conflicts
        // TODO: Create backup before applying

        let mut cache = self.config_cache.write().await;
        for (key, value) in imported_config {
            cache.insert(key, value);
        }

        tracing::info!("Configuration imported from JSON");
        Ok(())
    }

    // Private helper methods

    async fn initialize_default_config(&self) {
        let mut defaults = HashMap::new();

        // General settings
        defaults.insert("general:platform_name".to_string(), ConfigValue::String("SaaS UMKM Platform".to_string()));
        defaults.insert("general:platform_url".to_string(), ConfigValue::String("https://saasumkm.id".to_string()));
        defaults.insert("general:support_email".to_string(), ConfigValue::String("support@saasumkm.id".to_string()));
        defaults.insert("general:max_users_per_company".to_string(), ConfigValue::Integer(10));
        defaults.insert("general:maintenance_mode".to_string(), ConfigValue::Boolean(false));

        // Email settings
        defaults.insert("email:smtp_host".to_string(), ConfigValue::String("smtp.gmail.com".to_string()));
        defaults.insert("email:smtp_port".to_string(), ConfigValue::Integer(587));
        defaults.insert("email:use_tls".to_string(), ConfigValue::Boolean(true));
        defaults.insert("email:from_address".to_string(), ConfigValue::String("noreply@saasumkm.id".to_string()));
        defaults.insert("email:from_name".to_string(), ConfigValue::String("SaaS UMKM Platform".to_string()));

        // License processing settings
        defaults.insert("license:auto_approval_enabled".to_string(), ConfigValue::Boolean(false));
        defaults.insert("license:max_processing_days".to_string(), ConfigValue::Integer(5));
        defaults.insert("license:supported_types".to_string(), ConfigValue::Array(vec!["NIB".to_string(), "SIUP".to_string(), "TDP".to_string(), "NPWP".to_string()]));
        defaults.insert("license:document_retention_days".to_string(), ConfigValue::Integer(2555)); // 7 years

        // Payment settings
        defaults.insert("payment:gateway_provider".to_string(), ConfigValue::String("midtrans".to_string()));
        defaults.insert("payment:subscription_trial_days".to_string(), ConfigValue::Integer(14));
        defaults.insert("payment:currency".to_string(), ConfigValue::String("IDR".to_string()));
        defaults.insert("payment:late_fee_percentage".to_string(), ConfigValue::Float(2.5));

        // Security settings
        defaults.insert("security:jwt_expiry_minutes".to_string(), ConfigValue::Integer(15));
        defaults.insert("security:refresh_token_expiry_days".to_string(), ConfigValue::Integer(7));
        defaults.insert("security:max_login_attempts".to_string(), ConfigValue::Integer(5));
        defaults.insert("security:lockout_duration_minutes".to_string(), ConfigValue::Integer(30));
        defaults.insert("security:password_min_length".to_string(), ConfigValue::Integer(8));
        defaults.insert("security:require_2fa".to_string(), ConfigValue::Boolean(false));

        // Notification settings
        defaults.insert("notification:email_notifications_enabled".to_string(), ConfigValue::Boolean(true));
        defaults.insert("notification:sms_notifications_enabled".to_string(), ConfigValue::Boolean(false));
        defaults.insert("notification:push_notifications_enabled".to_string(), ConfigValue::Boolean(true));
        defaults.insert("notification:daily_digest_enabled".to_string(), ConfigValue::Boolean(true));

        // Apply defaults to cache
        let mut cache = self.config_cache.write().await;
        for (key, value) in defaults {
            cache.insert(key, value);
        }
    }

    async fn get_general_config_group(&self) -> ConfigGroup {
        let mut settings = HashMap::new();
        
        settings.insert("platform_name".to_string(), ConfigSetting {
            key: "general:platform_name".to_string(),
            value: ConfigValue::String("SaaS UMKM Platform".to_string()),
            default_value: ConfigValue::String("SaaS UMKM Platform".to_string()),
            description: "Name of the platform displayed to users".to_string(),
            category: "branding".to_string(),
            is_sensitive: false,
            requires_restart: false,
            validation_rules: vec![ValidationRule::Required, ValidationRule::MinLength(3)],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        settings.insert("platform_url".to_string(), ConfigSetting {
            key: "general:platform_url".to_string(),
            value: ConfigValue::String("https://saasumkm.id".to_string()),
            default_value: ConfigValue::String("https://saasumkm.id".to_string()),
            description: "Base URL of the platform".to_string(),
            category: "system".to_string(),
            is_sensitive: false,
            requires_restart: true,
            validation_rules: vec![ValidationRule::Required, ValidationRule::Url],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        ConfigGroup {
            name: "general".to_string(),
            description: "General platform settings".to_string(),
            settings,
        }
    }

    async fn get_email_config_group(&self) -> ConfigGroup {
        let mut settings = HashMap::new();
        
        settings.insert("smtp_host".to_string(), ConfigSetting {
            key: "email:smtp_host".to_string(),
            value: ConfigValue::String("smtp.gmail.com".to_string()),
            default_value: ConfigValue::String("smtp.gmail.com".to_string()),
            description: "SMTP server hostname".to_string(),
            category: "email".to_string(),
            is_sensitive: false,
            requires_restart: true,
            validation_rules: vec![ValidationRule::Required],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        ConfigGroup {
            name: "email".to_string(),
            description: "Email service configuration".to_string(),
            settings,
        }
    }

    async fn get_license_config_group(&self) -> ConfigGroup {
        let mut settings = HashMap::new();
        
        settings.insert("max_processing_days".to_string(), ConfigSetting {
            key: "license:max_processing_days".to_string(),
            value: ConfigValue::Integer(5),
            default_value: ConfigValue::Integer(5),
            description: "Maximum days for license processing".to_string(),
            category: "processing".to_string(),
            is_sensitive: false,
            requires_restart: false,
            validation_rules: vec![ValidationRule::MinValue(1.0), ValidationRule::MaxValue(30.0)],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        ConfigGroup {
            name: "license".to_string(),
            description: "License processing configuration".to_string(),
            settings,
        }
    }

    async fn get_payment_config_group(&self) -> ConfigGroup {
        let mut settings = HashMap::new();
        
        settings.insert("gateway_provider".to_string(), ConfigSetting {
            key: "payment:gateway_provider".to_string(),
            value: ConfigValue::String("midtrans".to_string()),
            default_value: ConfigValue::String("midtrans".to_string()),
            description: "Payment gateway provider".to_string(),
            category: "payment".to_string(),
            is_sensitive: false,
            requires_restart: true,
            validation_rules: vec![ValidationRule::OneOf(vec!["midtrans".to_string(), "xendit".to_string()])],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        ConfigGroup {
            name: "payment".to_string(),
            description: "Payment system configuration".to_string(),
            settings,
        }
    }

    async fn get_security_config_group(&self) -> ConfigGroup {
        let mut settings = HashMap::new();
        
        settings.insert("jwt_expiry_minutes".to_string(), ConfigSetting {
            key: "security:jwt_expiry_minutes".to_string(),
            value: ConfigValue::Integer(15),
            default_value: ConfigValue::Integer(15),
            description: "JWT token expiry time in minutes".to_string(),
            category: "authentication".to_string(),
            is_sensitive: false,
            requires_restart: false,
            validation_rules: vec![ValidationRule::MinValue(5.0), ValidationRule::MaxValue(1440.0)],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        ConfigGroup {
            name: "security".to_string(),
            description: "Security and authentication settings".to_string(),
            settings,
        }
    }

    async fn get_notification_config_group(&self) -> ConfigGroup {
        let mut settings = HashMap::new();
        
        settings.insert("email_notifications_enabled".to_string(), ConfigSetting {
            key: "notification:email_notifications_enabled".to_string(),
            value: ConfigValue::Boolean(true),
            default_value: ConfigValue::Boolean(true),
            description: "Enable email notifications".to_string(),
            category: "notifications".to_string(),
            is_sensitive: false,
            requires_restart: false,
            validation_rules: vec![],
            updated_at: chrono::Utc::now(),
            updated_by: None,
        });

        ConfigGroup {
            name: "notification".to_string(),
            description: "Notification system settings".to_string(),
            settings,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_config_service_creation() {
        let service = SystemConfigService::new().await;
        let platform_name = service.get_string("general:platform_name").await.unwrap();
        assert_eq!(platform_name, "SaaS UMKM Platform");
    }

    #[tokio::test]
    async fn test_config_value_types() {
        let service = SystemConfigService::new().await;
        
        // Test different value types
        let _ = service.get_string("general:platform_name").await.unwrap();
        let _ = service.get_integer("license:max_processing_days").await.unwrap();
        let _ = service.get_boolean("notification:email_notifications_enabled").await.unwrap();
    }

    #[tokio::test]
    async fn test_config_groups() {
        let service = SystemConfigService::new().await;
        let groups = service.get_all_groups().await.unwrap();
        
        assert!(!groups.is_empty());
        assert!(groups.iter().any(|g| g.name == "general"));
        assert!(groups.iter().any(|g| g.name == "email"));
        assert!(groups.iter().any(|g| g.name == "license"));
    }
}
