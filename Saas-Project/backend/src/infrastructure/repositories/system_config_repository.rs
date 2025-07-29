use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// System configuration entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfigGroup {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfigSetting {
    pub id: Uuid,
    pub group_id: Uuid,
    pub key: String,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub data_type: String,
    pub description: Option<String>,
    pub validation_rule: Option<String>,
    pub is_sensitive: bool,
    pub is_required: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Repository trait
#[async_trait]
pub trait SystemConfigRepository: Send + Sync {
    async fn get_all_groups(&self) -> Result<Vec<SystemConfigGroup>, sqlx::Error>;
    async fn get_group_by_name(&self, name: &str) -> Result<Option<SystemConfigGroup>, sqlx::Error>;
    async fn get_group_settings(&self, group_id: &Uuid) -> Result<Vec<SystemConfigSetting>, sqlx::Error>;
    async fn get_setting_by_key(&self, key: &str) -> Result<Option<SystemConfigSetting>, sqlx::Error>;
    async fn set_setting_value(&self, key: &str, value: &str) -> Result<(), sqlx::Error>;
    async fn reset_setting_to_default(&self, key: &str) -> Result<(), sqlx::Error>;
    async fn create_setting(&self, setting: &SystemConfigSetting) -> Result<(), sqlx::Error>;
    async fn update_setting(&self, setting: &SystemConfigSetting) -> Result<(), sqlx::Error>;
    async fn export_configuration(&self) -> Result<serde_json::Value, sqlx::Error>;
    async fn import_configuration(&self, config: &serde_json::Value) -> Result<(), sqlx::Error>;
}

// PostgreSQL implementation
pub struct PostgresSystemConfigRepository {
    pool: PgPool,
}

impl PostgresSystemConfigRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SystemConfigRepository for PostgresSystemConfigRepository {
    async fn get_all_groups(&self) -> Result<Vec<SystemConfigGroup>, sqlx::Error> {
        let groups = sqlx::query!(
            r#"
            SELECT 
                id, 
                name, 
                display_name, 
                description, 
                is_active, 
                sort_order, 
                created_at, 
                updated_at
            FROM system_config_groups 
            WHERE is_active = true 
            ORDER BY sort_order, display_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(groups.into_iter().map(|g| SystemConfigGroup {
            id: g.id,
            name: g.name,
            display_name: g.display_name,
            description: g.description,
            is_active: g.is_active,
            sort_order: g.sort_order,
            created_at: g.created_at,
            updated_at: g.updated_at,
        }).collect())
    }

    async fn get_group_by_name(&self, name: &str) -> Result<Option<SystemConfigGroup>, sqlx::Error> {
        let group = sqlx::query!(
            r#"
            SELECT 
                id, 
                name, 
                display_name, 
                description, 
                is_active, 
                sort_order, 
                created_at, 
                updated_at
            FROM system_config_groups 
            WHERE name = $1 AND is_active = true
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(group.map(|g| SystemConfigGroup {
            id: g.id,
            name: g.name,
            display_name: g.display_name,
            description: g.description,
            is_active: g.is_active,
            sort_order: g.sort_order,
            created_at: g.created_at,
            updated_at: g.updated_at,
        }))
    }

    async fn get_group_settings(&self, group_id: &Uuid) -> Result<Vec<SystemConfigSetting>, sqlx::Error> {
        let settings = sqlx::query!(
            r#"
            SELECT 
                id, 
                group_id, 
                key, 
                value, 
                default_value, 
                data_type, 
                description, 
                validation_rule, 
                is_sensitive, 
                is_required, 
                sort_order, 
                created_at, 
                updated_at
            FROM system_config_settings 
            WHERE group_id = $1 
            ORDER BY sort_order, key
            "#,
            group_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(settings.into_iter().map(|s| SystemConfigSetting {
            id: s.id,
            group_id: s.group_id,
            key: s.key,
            value: s.value,
            default_value: s.default_value,
            data_type: s.data_type,
            description: s.description,
            validation_rule: s.validation_rule,
            is_sensitive: s.is_sensitive,
            is_required: s.is_required,
            sort_order: s.sort_order,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }).collect())
    }

    async fn get_setting_by_key(&self, key: &str) -> Result<Option<SystemConfigSetting>, sqlx::Error> {
        let setting = sqlx::query!(
            r#"
            SELECT 
                s.id, 
                s.group_id, 
                s.key, 
                s.value, 
                s.default_value, 
                s.data_type, 
                s.description, 
                s.validation_rule, 
                s.is_sensitive, 
                s.is_required, 
                s.sort_order, 
                s.created_at, 
                s.updated_at
            FROM system_config_settings s
            JOIN system_config_groups g ON s.group_id = g.id
            WHERE s.key = $1 AND g.is_active = true
            "#,
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(setting.map(|s| SystemConfigSetting {
            id: s.id,
            group_id: s.group_id,
            key: s.key,
            value: s.value,
            default_value: s.default_value,
            data_type: s.data_type,
            description: s.description,
            validation_rule: s.validation_rule,
            is_sensitive: s.is_sensitive,
            is_required: s.is_required,
            sort_order: s.sort_order,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }))
    }

    async fn set_setting_value(&self, key: &str, value: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE system_config_settings 
            SET value = $2, updated_at = NOW()
            WHERE key = $1
            "#,
            key,
            value
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn reset_setting_to_default(&self, key: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE system_config_settings 
            SET value = default_value, updated_at = NOW()
            WHERE key = $1
            "#,
            key
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn create_setting(&self, setting: &SystemConfigSetting) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO system_config_settings 
            (id, group_id, key, value, default_value, data_type, description, validation_rule, is_sensitive, is_required, sort_order, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            setting.id,
            setting.group_id,
            setting.key,
            setting.value,
            setting.default_value,
            setting.data_type,
            setting.description,
            setting.validation_rule,
            setting.is_sensitive,
            setting.is_required,
            setting.sort_order,
            setting.created_at,
            setting.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn update_setting(&self, setting: &SystemConfigSetting) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE system_config_settings 
            SET value = $2, 
                default_value = $3, 
                data_type = $4, 
                description = $5, 
                validation_rule = $6, 
                is_sensitive = $7, 
                is_required = $8, 
                sort_order = $9,
                updated_at = NOW()
            WHERE id = $1
            "#,
            setting.id,
            setting.value,
            setting.default_value,
            setting.data_type,
            setting.description,
            setting.validation_rule,
            setting.is_sensitive,
            setting.is_required,
            setting.sort_order
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn export_configuration(&self) -> Result<serde_json::Value, sqlx::Error> {
        let groups_and_settings = sqlx::query!(
            r#"
            SELECT 
                g.name as group_name,
                g.display_name as group_display_name,
                s.key,
                s.value,
                s.default_value,
                s.data_type,
                s.description as setting_description
            FROM system_config_groups g
            LEFT JOIN system_config_settings s ON g.id = s.group_id
            WHERE g.is_active = true
            ORDER BY g.sort_order, s.sort_order
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut config = serde_json::Map::new();
        let mut current_group = String::new();
        let mut group_settings = serde_json::Map::new();

        for row in groups_and_settings {
            if current_group != row.group_name {
                if !current_group.is_empty() {
                    config.insert(current_group.clone(), serde_json::Value::Object(group_settings.clone()));
                    group_settings.clear();
                }
                current_group = row.group_name.clone();
            }

            if !row.key.is_empty() {
                let setting_info = serde_json::json!({
                    "value": row.value,
                    "default_value": row.default_value,
                    "data_type": row.data_type,
                    "description": row.setting_description
                });
                group_settings.insert(row.key, setting_info);
            }
        }

        if !current_group.is_empty() {
            config.insert(current_group, serde_json::Value::Object(group_settings));
        }

        Ok(serde_json::Value::Object(config))
    }

    async fn import_configuration(&self, config: &serde_json::Value) -> Result<(), sqlx::Error> {
        // Begin transaction
        let mut tx = self.pool.begin().await?;

        if let serde_json::Value::Object(groups) = config {
            for (group_name, group_config) in groups {
                // Get group ID
                let group = sqlx::query!(
                    "SELECT id FROM system_config_groups WHERE name = $1",
                    group_name
                )
                .fetch_optional(&mut *tx)
                .await?;

                if let Some(group_row) = group {
                    if let serde_json::Value::Object(settings) = group_config {
                        for (setting_key, setting_info) in settings {
                            if let serde_json::Value::Object(info) = setting_info {
                                if let Some(value) = info.get("value").and_then(|v| v.as_str()) {
                                    sqlx::query!(
                                        r#"
                                        INSERT INTO system_config_settings 
                                        (id, group_id, key, value, default_value, data_type, description, created_at, updated_at)
                                        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
                                        ON CONFLICT (group_id, key) 
                                        DO UPDATE SET value = EXCLUDED.value, updated_at = NOW()
                                        "#,
                                        Uuid::new_v4(),
                                        group_row.id,
                                        setting_key,
                                        value,
                                        info.get("default_value").and_then(|v| v.as_str()),
                                        info.get("data_type").and_then(|v| v.as_str()).unwrap_or("string"),
                                        info.get("description").and_then(|v| v.as_str())
                                    )
                                    .execute(&mut *tx)
                                    .await?;
                                }
                            }
                        }
                    }
                }
            }
        }

        tx.commit().await?;
        Ok(())
    }
}
