use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Email entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub html_body: String,
    pub text_body: Option<String>,
    pub variables: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLog {
    pub id: Uuid,
    pub template_id: Option<Uuid>,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub subject: String,
    pub status: String,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

// Repository trait
#[async_trait]
pub trait EmailRepository: Send + Sync {
    async fn get_template_by_name(&self, name: &str) -> Result<Option<EmailTemplate>, sqlx::Error>;
    async fn get_all_templates(&self) -> Result<Vec<EmailTemplate>, sqlx::Error>;
    async fn create_template(&self, template: &EmailTemplate) -> Result<(), sqlx::Error>;
    async fn update_template(&self, template: &EmailTemplate) -> Result<(), sqlx::Error>;
    async fn log_email(&self, log: &EmailLog) -> Result<(), sqlx::Error>;
    async fn update_email_status(&self, log_id: &Uuid, status: &str, error_message: Option<&str>) -> Result<(), sqlx::Error>;
    async fn get_email_logs(&self, recipient_email: Option<&str>, limit: i64, offset: i64) -> Result<Vec<EmailLog>, sqlx::Error>;
}

// PostgreSQL implementation
pub struct PostgresEmailRepository {
    pool: PgPool,
}

impl PostgresEmailRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmailRepository for PostgresEmailRepository {
    async fn get_template_by_name(&self, name: &str) -> Result<Option<EmailTemplate>, sqlx::Error> {
        let template = sqlx::query!(
            r#"
            SELECT 
                id, 
                name, 
                subject, 
                html_body, 
                text_body, 
                variables, 
                is_active, 
                created_at, 
                updated_at
            FROM email_templates 
            WHERE name = $1 AND is_active = true
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(template.map(|t| EmailTemplate {
            id: t.id,
            name: t.name,
            subject: t.subject,
            html_body: t.html_body,
            text_body: t.text_body,
            variables: t.variables.unwrap_or_default(),
            is_active: t.is_active,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }))
    }

    async fn get_all_templates(&self) -> Result<Vec<EmailTemplate>, sqlx::Error> {
        let templates = sqlx::query!(
            r#"
            SELECT 
                id, 
                name, 
                subject, 
                html_body, 
                text_body, 
                variables, 
                is_active, 
                created_at, 
                updated_at
            FROM email_templates 
            WHERE is_active = true 
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(templates.into_iter().map(|t| EmailTemplate {
            id: t.id,
            name: t.name,
            subject: t.subject,
            html_body: t.html_body,
            text_body: t.text_body,
            variables: t.variables.unwrap_or_default(),
            is_active: t.is_active,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }).collect())
    }

    async fn create_template(&self, template: &EmailTemplate) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO email_templates 
            (id, name, subject, html_body, text_body, variables, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            template.id,
            template.name,
            template.subject,
            template.html_body,
            template.text_body,
            template.variables,
            template.is_active,
            template.created_at,
            template.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn update_template(&self, template: &EmailTemplate) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE email_templates 
            SET subject = $2, 
                html_body = $3, 
                text_body = $4, 
                variables = $5, 
                is_active = $6,
                updated_at = NOW()
            WHERE id = $1
            "#,
            template.id,
            template.subject,
            template.html_body,
            template.text_body,
            template.variables,
            template.is_active
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn log_email(&self, log: &EmailLog) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO email_logs 
            (id, template_id, recipient_email, recipient_name, subject, status, error_message, sent_at, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            log.id,
            log.template_id,
            log.recipient_email,
            log.recipient_name,
            log.subject,
            log.status,
            log.error_message,
            log.sent_at,
            log.metadata,
            log.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn update_email_status(&self, log_id: &Uuid, status: &str, error_message: Option<&str>) -> Result<(), sqlx::Error> {
        let sent_at = if status == "sent" { Some(Utc::now()) } else { None };
        
        sqlx::query!(
            r#"
            UPDATE email_logs 
            SET status = $2, 
                error_message = $3, 
                sent_at = COALESCE($4, sent_at)
            WHERE id = $1
            "#,
            log_id,
            status,
            error_message,
            sent_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_email_logs(&self, recipient_email: Option<&str>, limit: i64, offset: i64) -> Result<Vec<EmailLog>, sqlx::Error> {
        let logs = sqlx::query!(
            r#"
            SELECT 
                id, 
                template_id, 
                recipient_email, 
                recipient_name, 
                subject, 
                status, 
                error_message, 
                sent_at, 
                metadata, 
                created_at
            FROM email_logs 
            WHERE ($1::text IS NULL OR recipient_email = $1)
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
            "#,
            recipient_email,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(logs.into_iter().map(|l| EmailLog {
            id: l.id,
            template_id: l.template_id,
            recipient_email: l.recipient_email,
            recipient_name: l.recipient_name,
            subject: l.subject,
            status: l.status,
            error_message: l.error_message,
            sent_at: l.sent_at,
            metadata: l.metadata.unwrap_or_default(),
            created_at: l.created_at,
        }).collect())
    }
}
