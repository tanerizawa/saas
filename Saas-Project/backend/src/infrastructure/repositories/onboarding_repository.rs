use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::value_objects::{UserId, CompanyId};

// Onboarding workflow entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingWorkflow {
    pub id: Uuid,
    pub user_id: UserId,
    pub company_id: Option<CompanyId>,
    pub current_step: i32,
    pub total_steps: i32,
    pub completion_percentage: f64,
    pub status: String,
    pub estimated_completion_time: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: i32,
    pub step_name: String,
    pub description: Option<String>,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Repository trait
#[async_trait]
pub trait OnboardingRepository: Send + Sync {
    async fn create_workflow(&self, user_id: UserId, company_id: Option<CompanyId>) -> Result<OnboardingWorkflow, sqlx::Error>;
    async fn get_workflow_by_user_id(&self, user_id: &UserId) -> Result<Option<OnboardingWorkflow>, sqlx::Error>;
    async fn get_workflow_by_id(&self, id: &Uuid) -> Result<Option<OnboardingWorkflow>, sqlx::Error>;
    async fn update_workflow(&self, workflow: &OnboardingWorkflow) -> Result<(), sqlx::Error>;
    async fn complete_step(&self, workflow_id: &Uuid, step_number: i32) -> Result<(), sqlx::Error>;
    async fn get_workflow_steps(&self, workflow_id: &Uuid) -> Result<Vec<OnboardingStep>, sqlx::Error>;
    async fn create_step(&self, step: &OnboardingStep) -> Result<(), sqlx::Error>;
    async fn update_step(&self, step: &OnboardingStep) -> Result<(), sqlx::Error>;
}

// PostgreSQL implementation
pub struct PostgresOnboardingRepository {
    pool: PgPool,
}

impl PostgresOnboardingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OnboardingRepository for PostgresOnboardingRepository {
    async fn create_workflow(&self, user_id: UserId, company_id: Option<CompanyId>) -> Result<OnboardingWorkflow, sqlx::Error> {
        let workflow_id = Uuid::new_v4();
        let now = Utc::now();
        
        let company_uuid = company_id.map(|id| id.into_inner());
        
        let workflow = sqlx::query!(
            r#"
            INSERT INTO onboarding_workflows 
            (id, user_id, company_id, current_step, total_steps, completion_percentage, status, started_at, created_at, updated_at, metadata)
            VALUES ($1, $2, $3, 1, 7, 0.0, 'in_progress', $4, $4, $4, '{}')
            RETURNING 
                id, 
                user_id, 
                company_id, 
                current_step, 
                total_steps, 
                completion_percentage, 
                status, 
                estimated_completion_time,
                started_at, 
                completed_at, 
                metadata, 
                created_at, 
                updated_at
            "#,
            workflow_id,
            user_id.into_inner(),
            company_uuid,
            now
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(OnboardingWorkflow {
            id: workflow.id,
            user_id: UserId::from_uuid(workflow.user_id),
            company_id: workflow.company_id.map(CompanyId::from_uuid),
            current_step: workflow.current_step,
            total_steps: workflow.total_steps,
            completion_percentage: workflow.completion_percentage.to_string().parse().unwrap_or(0.0),
            status: workflow.status,
            estimated_completion_time: workflow.estimated_completion_time,
            started_at: workflow.started_at,
            completed_at: workflow.completed_at,
            metadata: workflow.metadata.unwrap_or_else(|| serde_json::json!({})),
            created_at: workflow.created_at,
            updated_at: workflow.updated_at,
        })
    }

    async fn get_workflow_by_user_id(&self, user_id: &UserId) -> Result<Option<OnboardingWorkflow>, sqlx::Error> {
        let workflow = sqlx::query!(
            r#"
            SELECT 
                id, 
                user_id, 
                company_id, 
                current_step, 
                total_steps, 
                completion_percentage, 
                status, 
                estimated_completion_time,
                started_at, 
                completed_at, 
                metadata, 
                created_at, 
                updated_at
            FROM onboarding_workflows 
            WHERE user_id = $1 
            ORDER BY created_at DESC 
            LIMIT 1
            "#,
            user_id.clone().into_inner()
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(workflow.map(|w| OnboardingWorkflow {
            id: w.id,
            user_id: UserId::from_uuid(w.user_id),
            company_id: w.company_id.map(CompanyId::from_uuid),
            current_step: w.current_step,
            total_steps: w.total_steps,
            completion_percentage: w.completion_percentage.to_string().parse().unwrap_or(0.0),
            status: w.status,
            estimated_completion_time: w.estimated_completion_time,
            started_at: w.started_at,
            completed_at: w.completed_at,
            metadata: w.metadata.unwrap_or_else(|| serde_json::json!({})),
            created_at: w.created_at,
            updated_at: w.updated_at,
        }))
    }

    async fn get_workflow_by_id(&self, id: &Uuid) -> Result<Option<OnboardingWorkflow>, sqlx::Error> {
        let workflow = sqlx::query!(
            r#"
            SELECT 
                id, 
                user_id, 
                company_id, 
                current_step, 
                total_steps, 
                completion_percentage, 
                status, 
                estimated_completion_time,
                started_at, 
                completed_at, 
                metadata, 
                created_at, 
                updated_at
            FROM onboarding_workflows 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(workflow.map(|w| OnboardingWorkflow {
            id: w.id,
            user_id: UserId::from_uuid(w.user_id),
            company_id: w.company_id.map(CompanyId::from_uuid),
            current_step: w.current_step,
            total_steps: w.total_steps,
            completion_percentage: w.completion_percentage.to_string().parse().unwrap_or(0.0),
            status: w.status,
            estimated_completion_time: w.estimated_completion_time,
            started_at: w.started_at,
            completed_at: w.completed_at,
            metadata: w.metadata.unwrap_or_else(|| serde_json::json!({})),
            created_at: w.created_at,
            updated_at: w.updated_at,
        }))
    }

    async fn update_workflow(&self, workflow: &OnboardingWorkflow) -> Result<(), sqlx::Error> {
        let completion_percentage = sqlx::types::BigDecimal::from(workflow.completion_percentage as i64);
        
        sqlx::query!(
            r#"
            UPDATE onboarding_workflows 
            SET current_step = $2, 
                completion_percentage = $3, 
                status = $4, 
                estimated_completion_time = $5,
                completed_at = $6,
                metadata = $7,
                updated_at = NOW()
            WHERE id = $1
            "#,
            workflow.id,
            workflow.current_step,
            completion_percentage,
            workflow.status,
            workflow.estimated_completion_time,
            workflow.completed_at,
            workflow.metadata
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn complete_step(&self, workflow_id: &Uuid, step_number: i32) -> Result<(), sqlx::Error> {
        let now = Utc::now();
        
        // Update the step status
        sqlx::query!(
            r#"
            UPDATE onboarding_steps 
            SET status = 'completed', 
                completed_at = $3,
                updated_at = NOW()
            WHERE workflow_id = $1 AND step_number = $2
            "#,
            workflow_id,
            step_number,
            now
        )
        .execute(&self.pool)
        .await?;
        
        // Update workflow progress
        let completion_percentage = sqlx::types::BigDecimal::from((step_number as f64 / 7.0 * 100.0) as i64);
        let next_step = if step_number >= 7 { 7 } else { step_number + 1 };
        let status = if step_number >= 7 { "completed" } else { "in_progress" };
        let completed_at = if step_number >= 7 { Some(now) } else { None };
        
        sqlx::query!(
            r#"
            UPDATE onboarding_workflows 
            SET current_step = $2, 
                completion_percentage = $3, 
                status = $4,
                completed_at = $5,
                updated_at = NOW()
            WHERE id = $1
            "#,
            workflow_id,
            next_step,
            completion_percentage,
            status,
            completed_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_workflow_steps(&self, workflow_id: &Uuid) -> Result<Vec<OnboardingStep>, sqlx::Error> {
        let steps = sqlx::query!(
            r#"
            SELECT 
                id, 
                workflow_id, 
                step_number, 
                step_name, 
                description, 
                status, 
                started_at, 
                completed_at, 
                data, 
                created_at, 
                updated_at
            FROM onboarding_steps 
            WHERE workflow_id = $1 
            ORDER BY step_number
            "#,
            workflow_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(steps.into_iter().map(|s| OnboardingStep {
            id: s.id,
            workflow_id: s.workflow_id,
            step_number: s.step_number,
            step_name: s.step_name,
            description: s.description,
            status: s.status,
            started_at: s.started_at,
            completed_at: s.completed_at,
            data: s.data.unwrap_or_else(|| serde_json::json!({})),
            created_at: s.created_at,
            updated_at: s.updated_at,
        }).collect())
    }

    async fn create_step(&self, step: &OnboardingStep) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO onboarding_steps 
            (id, workflow_id, step_number, step_name, description, status, started_at, completed_at, data, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
            step.id,
            step.workflow_id,
            step.step_number,
            step.step_name,
            step.description,
            step.status,
            step.started_at,
            step.completed_at,
            step.data,
            step.created_at,
            step.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn update_step(&self, step: &OnboardingStep) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE onboarding_steps 
            SET step_name = $2, 
                description = $3, 
                status = $4, 
                started_at = $5,
                completed_at = $6,
                data = $7,
                updated_at = NOW()
            WHERE id = $1
            "#,
            step.id,
            step.step_name,
            step.description,
            step.status,
            step.started_at,
            step.completed_at,
            step.data
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
