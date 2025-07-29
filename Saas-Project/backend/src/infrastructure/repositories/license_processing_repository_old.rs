use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;

use crate::domain::value_objects::{LicenseId, UserId, CompanyId};
use crate::services::license_processing::{
    LicenseApplicationRequest, LicenseProcessingStatus, LicenseReviewRequest,
    ReviewAction, ProcessingStep, PriorityLevel, WorkflowStage, ProcessingStatistics
};

// Repository trait for license processing operations
#[async_trait]
pub trait LicenseProcessingRepository: Send + Sync {
    // Workflow management
    async fn create_workflow(&self, workflow: &LicenseProcessingWorkflow) -> Result<(), sqlx::Error>;
    async fn get_workflow_by_license_id(&self, license_id: &LicenseId) -> Result<Option<LicenseProcessingWorkflow>, sqlx::Error>;
    async fn update_workflow(&self, workflow: &LicenseProcessingWorkflow) -> Result<(), sqlx::Error>;
    async fn get_workflows_by_status(&self, status: &str) -> Result<Vec<LicenseProcessingWorkflow>, sqlx::Error>;
    async fn get_workflows_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<LicenseProcessingWorkflow>, sqlx::Error>;
    
    // Processing steps management
    async fn create_step(&self, step: &LicenseProcessingStepData) -> Result<(), sqlx::Error>;
    async fn get_steps_by_workflow(&self, workflow_id: &Uuid) -> Result<Vec<LicenseProcessingStepData>, sqlx::Error>;
    async fn update_step(&self, step: &LicenseProcessingStepData) -> Result<(), sqlx::Error>;
    async fn complete_step(&self, workflow_id: &Uuid, step_number: i32) -> Result<(), sqlx::Error>;
    
    // Statistics and reporting
    async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, sqlx::Error>;
    async fn get_workload_by_reviewer(&self, reviewer_id: &UserId) -> Result<i64, sqlx::Error>;
}

// Domain models for database operations
#[derive(Debug, Clone)]
pub struct LicenseProcessingWorkflow {
    pub id: Uuid,
    pub license_id: LicenseId,
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub license_type: String,
    pub current_status: String,
    pub current_step: i32,
    pub assigned_reviewer_id: Option<UserId>,
    pub priority_level: String,
    pub workflow_stage: String,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub completion_percentage: i32,
    pub business_description: String,
    pub additional_info: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LicenseProcessingStepData {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: i32,
    pub step_name: String,
    pub description: String,
    pub status: String,
    pub assigned_reviewer_id: Option<UserId>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub estimated_duration_hours: Option<i32>,
    pub data: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// PostgreSQL implementation
pub struct PostgresLicenseProcessingRepository {
    pool: PgPool,
}

impl PostgresLicenseProcessingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LicenseProcessingRepository for PostgresLicenseProcessingRepository {
    async fn create_workflow(&self, workflow: &LicenseProcessingWorkflow) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO license_processing_workflows (
                id, license_id, user_id, company_id, license_type, current_status,
                current_step, assigned_reviewer_id, priority_level, workflow_stage,
                estimated_completion, completion_percentage, business_description,
                additional_info, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            workflow.id,
            workflow.license_id.clone().into_inner(),
            workflow.user_id.clone().into_inner(),
            workflow.company_id.clone().into_inner(),
            workflow.license_type,
            workflow.current_status,
            workflow.current_step,
            workflow.assigned_reviewer_id.as_ref().map(|id| id.clone().into_inner()),
            workflow.priority_level,
            workflow.workflow_stage,
            workflow.estimated_completion,
            workflow.completion_percentage,
            workflow.business_description,
            workflow.additional_info,
            workflow.created_at,
            workflow.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_workflow_by_license_id(&self, license_id: &LicenseId) -> Result<Option<LicenseProcessingWorkflow>, sqlx::Error> {
        let workflow = sqlx::query!(
            r#"
            SELECT 
                id, license_id, user_id, company_id, license_type, current_status,
                current_step, assigned_reviewer_id, priority_level, workflow_stage,
                estimated_completion, completion_percentage, business_description,
                additional_info, created_at, updated_at
            FROM license_processing_workflows 
            WHERE license_id = $1 
            ORDER BY created_at DESC 
            LIMIT 1
            "#,
            license_id.clone().into_inner()
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(workflow.map(|w| LicenseProcessingWorkflow {
            id: w.id,
            license_id: LicenseId::from_uuid(w.license_id),
            user_id: UserId::from_uuid(w.user_id),
            company_id: CompanyId::from_uuid(w.company_id),
            license_type: w.license_type,
            current_status: w.current_status,
            current_step: w.current_step,
            assigned_reviewer_id: w.assigned_reviewer_id.map(UserId::from_uuid),
            priority_level: w.priority_level,
            workflow_stage: w.workflow_stage,
            estimated_completion: w.estimated_completion,
            completion_percentage: w.completion_percentage,
            business_description: w.business_description,
            additional_info: w.additional_info.unwrap_or_default(),
            created_at: w.created_at,
            updated_at: w.updated_at,
        }))
    }

    async fn update_workflow(&self, workflow: &LicenseProcessingWorkflow) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET current_status = $2, 
                current_step = $3, 
                assigned_reviewer_id = $4, 
                priority_level = $5, 
                workflow_stage = $6,
                estimated_completion = $7,
                completion_percentage = $8,
                additional_info = $9,
                updated_at = NOW()
            WHERE id = $1
            "#,
            workflow.id,
            workflow.current_status,
            workflow.current_step,
            workflow.assigned_reviewer_id.as_ref().map(|id| id.clone().into_inner()),
            workflow.priority_level,
            workflow.workflow_stage,
            workflow.estimated_completion,
            workflow.completion_percentage,
            workflow.additional_info
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_workflows_by_status(&self, status: &str) -> Result<Vec<LicenseProcessingWorkflow>, sqlx::Error> {
        let workflows = sqlx::query!(
            r#"
            SELECT 
                id, license_id, user_id, company_id, license_type, current_status,
                current_step, assigned_reviewer_id, priority_level, workflow_stage,
                estimated_completion, completion_percentage, business_description,
                additional_info, created_at, updated_at
            FROM license_processing_workflows 
            WHERE current_status = $1 
            ORDER BY created_at ASC
            "#,
            status
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(workflows.into_iter().map(|w| LicenseProcessingWorkflow {
            id: w.id,
            license_id: LicenseId::from_uuid(w.license_id),
            user_id: UserId::from_uuid(w.user_id),
            company_id: CompanyId::from_uuid(w.company_id),
            license_type: w.license_type,
            current_status: w.current_status,
            current_step: w.current_step,
            assigned_reviewer_id: w.assigned_reviewer_id.map(UserId::from_uuid),
            priority_level: w.priority_level,
            workflow_stage: w.workflow_stage,
            estimated_completion: w.estimated_completion,
            completion_percentage: w.completion_percentage,
            business_description: w.business_description,
            additional_info: w.additional_info.unwrap_or_default(),
            created_at: w.created_at,
            updated_at: w.updated_at,
        }).collect())
    }

    async fn get_workflows_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<LicenseProcessingWorkflow>, sqlx::Error> {
        let workflows = sqlx::query!(
            r#"
            SELECT 
                id, license_id, user_id, company_id, license_type, current_status,
                current_step, assigned_reviewer_id, priority_level, workflow_stage,
                estimated_completion, completion_percentage, business_description,
                additional_info, created_at, updated_at
            FROM license_processing_workflows 
            WHERE assigned_reviewer_id = $1 
            ORDER BY priority_level DESC, created_at ASC
            "#,
            reviewer_id.clone().into_inner()
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(workflows.into_iter().map(|w| LicenseProcessingWorkflow {
            id: w.id,
            license_id: LicenseId::from_uuid(w.license_id),
            user_id: UserId::from_uuid(w.user_id),
            company_id: CompanyId::from_uuid(w.company_id),
            license_type: w.license_type,
            current_status: w.current_status,
            current_step: w.current_step,
            assigned_reviewer_id: w.assigned_reviewer_id.map(UserId::from_uuid),
            priority_level: w.priority_level,
            workflow_stage: w.workflow_stage,
            estimated_completion: w.estimated_completion,
            completion_percentage: w.completion_percentage,
            business_description: w.business_description,
            additional_info: w.additional_info.unwrap_or_default(),
            created_at: w.created_at,
            updated_at: w.updated_at,
        }).collect())
    }

    async fn create_step(&self, step: &LicenseProcessingStepData) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO license_processing_steps (
                id, workflow_id, step_number, step_name, description, status,
                assigned_reviewer_id, started_at, completed_at, estimated_duration_hours,
                data, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            step.id,
            step.workflow_id,
            step.step_number,
            step.step_name,
            step.description,
            step.status,
            step.assigned_reviewer_id.as_ref().map(|id| id.clone().into_inner()),
            step.started_at,
            step.completed_at,
            step.estimated_duration_hours,
            step.data,
            step.created_at,
            step.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_steps_by_workflow(&self, workflow_id: &Uuid) -> Result<Vec<LicenseProcessingStepData>, sqlx::Error> {
        let steps = sqlx::query!(
            r#"
            SELECT 
                id, workflow_id, step_number, step_name, description, status,
                assigned_reviewer_id, started_at, completed_at, estimated_duration_hours,
                data, created_at, updated_at
            FROM license_processing_steps 
            WHERE workflow_id = $1 
            ORDER BY step_number ASC
            "#,
            workflow_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(steps.into_iter().map(|s| LicenseProcessingStepData {
            id: s.id,
            workflow_id: s.workflow_id,
            step_number: s.step_number,
            step_name: s.step_name,
            description: s.description,
            status: s.status,
            assigned_reviewer_id: s.assigned_reviewer_id.map(UserId::from_uuid),
            started_at: s.started_at,
            completed_at: s.completed_at,
            estimated_duration_hours: s.estimated_duration_hours,
            data: s.data.unwrap_or_default(),
            created_at: s.created_at,
            updated_at: s.updated_at,
        }).collect())
    }

    async fn update_step(&self, step: &LicenseProcessingStepData) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE license_processing_steps 
            SET step_name = $2, 
                description = $3, 
                status = $4, 
                assigned_reviewer_id = $5,
                started_at = $6,
                completed_at = $7,
                data = $8,
                updated_at = NOW()
            WHERE id = $1
            "#,
            step.id,
            step.step_name,
            step.description,
            step.status,
            step.assigned_reviewer_id.as_ref().map(|id| id.clone().into_inner()),
            step.started_at,
            step.completed_at,
            step.data
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn complete_step(&self, workflow_id: &Uuid, step_number: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE license_processing_steps 
            SET status = 'completed', 
                completed_at = NOW(),
                updated_at = NOW()
            WHERE workflow_id = $1 AND step_number = $2
            "#,
            workflow_id,
            step_number
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, sqlx::Error> {
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total_applications,
                COUNT(*) FILTER (WHERE current_status = 'pending') as pending_applications,
                COUNT(*) FILTER (WHERE current_status = 'in_review') as in_review_applications,
                COUNT(*) FILTER (WHERE current_status = 'approved') as approved_applications,
                COUNT(*) FILTER (WHERE current_status = 'rejected') as rejected_applications,
                AVG(EXTRACT(EPOCH FROM (updated_at - created_at))/3600) as avg_processing_time_hours
            FROM license_processing_workflows
            "#,
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(ProcessingStatistics {
            total_applications: stats.total_applications.unwrap_or(0) as u64,
            pending_applications: stats.pending_applications.unwrap_or(0) as u64,
            in_review_applications: stats.in_review_applications.unwrap_or(0) as u64,
            approved_applications: stats.approved_applications.unwrap_or(0) as u64,
            rejected_applications: stats.rejected_applications.unwrap_or(0) as u64,
            average_processing_time_hours: stats.avg_processing_time_hours.unwrap_or(0.0),
        })
    }

    async fn get_workload_by_reviewer(&self, reviewer_id: &UserId) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as workload
            FROM license_processing_workflows 
            WHERE assigned_reviewer_id = $1 
            AND current_status IN ('pending', 'in_review')
            "#,
            reviewer_id.clone().into_inner()
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result.workload.unwrap_or(0))
    }
}
