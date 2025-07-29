use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;
use crate::shared::errors::AppError;
use crate::domain::value_objects::{UserId, LicenseId};
use crate::services::license_processing::{
    LicenseApplicationRequest, LicenseProcessingStatus, LicenseReviewRequest,
    ReviewAction, ProcessingStep, PriorityLevel, WorkflowStage, ProcessingStatistics,
};

#[async_trait]
pub trait LicenseProcessingRepository: Send + Sync {
    // Workflow management
    async fn create_workflow(&self, workflow: &LicenseApplicationRequest) -> Result<String, AppError>;
    async fn get_workflow(&self, workflow_id: &str) -> Result<Option<LicenseApplicationRequest>, AppError>;
    async fn update_workflow_stage(&self, workflow: &LicenseApplicationRequest) -> Result<(), AppError>;
    async fn get_workflows_by_status(&self, status: &str) -> Result<Vec<LicenseApplicationRequest>, AppError>;
    async fn get_workflows_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<LicenseApplicationRequest>, AppError>;
    
    // Stage management
    async fn create_stage(&self, step: &ProcessingStep) -> Result<String, AppError>;
    async fn get_stages_by_workflow(&self, workflow_id: &str) -> Result<Vec<ProcessingStep>, AppError>;
    async fn update_stage(&self, step: &ProcessingStep) -> Result<(), AppError>;
    async fn complete_stage(&self, workflow_id: &str, stage_number: i32) -> Result<(), AppError>;
    
    // Statistics and reporting
    async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, AppError>;
    async fn get_reviewer_workload(&self, reviewer_id: &UserId) -> Result<i64, AppError>;
}

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
    async fn create_workflow(&self, workflow: &LicenseApplicationRequest) -> Result<String, AppError> {
        let workflow_id = Uuid::new_v4().to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO license_processing_workflows (
                id, license_id, current_stage, total_stages, priority, 
                assigned_reviewer_id, processing_notes, escalated, 
                escalation_reason, sla_deadline, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
            Uuid::parse_str(&workflow_id).map_err(|e| AppError::Validation(format!("Invalid workflow ID: {}", e)))?,
            workflow.license_id.as_uuid().clone().map_err(|e| AppError::Validation(format!("Invalid license ID: {}", e)))?,
            1i32, // current_stage
            8i32, // total_stages 
            workflow.priority.to_string(),
            workflow.reviewer_id.as_ref().map(|id| Uuid::parse_str(&id.clone().into_inner()).ok()).flatten(),
            workflow.additional_info.as_ref(),
            false, // escalated
            None as Option<String>, // escalation_reason
            workflow.submission_deadline,
            workflow.created_at,
            workflow.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(workflow_id)
    }

    async fn get_workflow(&self, workflow_id: &str) -> Result<Option<LicenseApplicationRequest>, AppError> {
        let workflow = sqlx::query!(
            r#"
            SELECT 
                id, license_id, current_stage, total_stages, priority, 
                assigned_reviewer_id, processing_notes, escalated, 
                escalation_reason, sla_deadline, created_at, updated_at
            FROM license_processing_workflows 
            WHERE id = $1
            "#,
            Uuid::parse_str(workflow_id).map_err(|e| AppError::Validation(format!("Invalid workflow ID: {}", e)))?
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        if let Some(w) = workflow {
            let license_id = LicenseId::from_uuid(w.license_id);
            let reviewer_id = w.assigned_reviewer_id.map(|id| UserId::from_uuid(id));
            
            Ok(Some(LicenseApplicationRequest {
                license_id,
                reviewer_id,
                priority: match w.priority.as_str() { "low" => PriorityLevel::Low, "high" => PriorityLevel::High, "urgent" => PriorityLevel::Urgent, _ => PriorityLevel::Normal },
                submission_deadline: w.sla_deadline,
                additional_info: w.processing_notes,
                created_at: w.created_at,
                updated_at: w.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_workflow_stage(&self, workflow: &LicenseApplicationRequest) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET priority = $2,
                assigned_reviewer_id = $3,
                processing_notes = $4,
                sla_deadline = $5,
                updated_at = $6
            WHERE license_id = $1
            "#,
            workflow.license_id.as_uuid().clone().map_err(|e| AppError::Validation(format!("Invalid license ID: {}", e)))?,
            workflow.priority.to_string(),
            workflow.reviewer_id.as_ref().map(|id| Uuid::parse_str(&id.clone().into_inner()).ok()).flatten(),
            workflow.additional_info.as_ref(),
            workflow.submission_deadline,
            workflow.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn get_workflows_by_status(&self, _status: &str) -> Result<Vec<LicenseApplicationRequest>, AppError> {
        // Note: The actual table doesn't have a status field, using current_stage instead
        let workflows = sqlx::query!(
            r#"
            SELECT 
                id, license_id, current_stage, total_stages, priority, 
                assigned_reviewer_id, processing_notes, escalated, 
                escalation_reason, sla_deadline, created_at, updated_at
            FROM license_processing_workflows 
            WHERE current_stage < total_stages
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let mut results = Vec::new();
        for w in workflows {
            let license_id = LicenseId::from_uuid(w.license_id);
            let reviewer_id = w.assigned_reviewer_id.map(|id| UserId::from_uuid(id));
            
            results.push(LicenseApplicationRequest {
                license_id,
                reviewer_id,
                priority: match w.priority.as_str() { "low" => PriorityLevel::Low, "high" => PriorityLevel::High, "urgent" => PriorityLevel::Urgent, _ => PriorityLevel::Normal },
                submission_deadline: w.sla_deadline,
                additional_info: w.processing_notes,
                created_at: w.created_at,
                updated_at: w.updated_at,
            });
        }

        Ok(results)
    }

    async fn get_workflows_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<LicenseApplicationRequest>, AppError> {
        let workflows = sqlx::query!(
            r#"
            SELECT 
                id, license_id, current_stage, total_stages, priority, 
                assigned_reviewer_id, processing_notes, escalated, 
                escalation_reason, sla_deadline, created_at, updated_at
            FROM license_processing_workflows 
            WHERE assigned_reviewer_id = $1
            "#,
            reviewer_id.as_uuid().clone().map_err(|e| AppError::Validation(format!("Invalid reviewer ID: {}", e)))?
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let mut results = Vec::new();
        for w in workflows {
            let license_id = LicenseId::from_uuid(w.license_id);
            let reviewer_id = w.assigned_reviewer_id.map(|id| UserId::from_uuid(id));
            
            results.push(LicenseApplicationRequest {
                license_id,
                reviewer_id,
                priority: match w.priority.as_str() { "low" => PriorityLevel::Low, "high" => PriorityLevel::High, "urgent" => PriorityLevel::Urgent, _ => PriorityLevel::Normal },
                submission_deadline: w.sla_deadline,
                additional_info: w.processing_notes,
                created_at: w.created_at,
                updated_at: w.updated_at,
            });
        }

        Ok(results)
    }

    async fn create_stage(&self, step: &ProcessingStep) -> Result<String, AppError> {
        let step_id = Uuid::new_v4().to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO license_processing_stages (
                id, workflow_id, stage_number, stage_name, status,
                reviewer_id, review_action, review_comments, started_at, 
                completed_at, data, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            Uuid::parse_str(&step_id).map_err(|e| AppError::Validation(format!("Invalid step ID: {}", e)))?,
            Uuid::parse_str(&step.workflow_id).map_err(|e| AppError::Validation(format!("Invalid workflow ID: {}", e)))?,
            step.step_number,
            step.step_name,
            step.status.to_string(),
            step.reviewer_id.as_ref().map(|id| Uuid::parse_str(&id.clone().into_inner()).ok()).flatten(),
            step.review_action.as_ref().map(|a| a.to_string()),
            step.review_comments.as_ref(),
            step.started_at,
            step.completed_at,
            step.data,
            step.created_at,
            step.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(step_id)
    }

    async fn get_stages_by_workflow(&self, workflow_id: &str) -> Result<Vec<ProcessingStep>, AppError> {
        let steps = sqlx::query!(
            r#"
            SELECT 
                id, workflow_id, stage_number, stage_name, status,
                reviewer_id, review_action, review_comments, started_at, 
                completed_at, data, created_at, updated_at
            FROM license_processing_stages 
            WHERE workflow_id = $1 
            ORDER BY stage_number
            "#,
            Uuid::parse_str(workflow_id).map_err(|e| AppError::Validation(format!("Invalid workflow ID: {}", e)))?
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let mut results = Vec::new();
        for s in steps {
            let reviewer_id = s.reviewer_id.map(|id| UserId::from_uuid(id));
            let review_action = s.review_action.and_then(|a| a.parse().ok());
            
            results.push(ProcessingStep {
                workflow_id: s.workflow_id.to_string(),
                step_number: s.stage_number,
                step_name: s.stage_name,
                status: match s.status.as_str() { "completed" => ProcessingStep::Completed, "failed" => ProcessingStep::ApplicationReceived, _ => ProcessingStep::ApplicationReceived },
                reviewer_id,
                review_action,
                review_comments: s.review_comments,
                started_at: s.started_at,
                completed_at: s.completed_at,
                data: s.data,
                created_at: s.created_at,
                updated_at: s.updated_at,
            });
        }

        Ok(results)
    }

    async fn update_stage(&self, step: &ProcessingStep) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE license_processing_stages 
            SET stage_name = $2, 
                status = $3,
                reviewer_id = $4,
                review_action = $5,
                review_comments = $6,
                started_at = $7,
                completed_at = $8,
                data = $9,
                updated_at = $10
            WHERE workflow_id = $1 AND stage_number = $11
            "#,
            Uuid::parse_str(&step.workflow_id).map_err(|e| AppError::Validation(format!("Invalid workflow ID: {}", e)))?,
            step.step_name,
            step.status.to_string(),
            step.reviewer_id.as_ref().map(|id| Uuid::parse_str(&id.clone().into_inner()).ok()).flatten(),
            step.review_action.as_ref().map(|a| a.to_string()),
            step.review_comments.as_ref(),
            step.started_at,
            step.completed_at,
            step.data,
            step.updated_at,
            step.step_number
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn complete_stage(&self, workflow_id: &str, stage_number: i32) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE license_processing_stages 
            SET status = 'completed', 
                completed_at = NOW(),
                updated_at = NOW()
            WHERE workflow_id = $1 AND stage_number = $2
            "#,
            Uuid::parse_str(workflow_id).map_err(|e| AppError::Validation(format!("Invalid workflow ID: {}", e)))?,
            stage_number
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, AppError> {
        Ok(ProcessingStatistics {
            sla_compliance_rate: 95.0, // Placeholder - would need SLA tracking
            applications_by_type: std::collections::HashMap::new(),
            applications_by_priority: std::collections::HashMap::new(),
            applications_by_stage: std::collections::HashMap::new(),
        })
    }

    async fn get_reviewer_workload(&self, reviewer_id: &UserId) -> Result<i64, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as workload
            FROM license_processing_workflows 
            WHERE assigned_reviewer_id = $1 AND current_stage < total_stages
            "#,
            reviewer_id.as_uuid().clone().map_err(|e| AppError::Validation(format!("Invalid reviewer ID: {}", e)))?
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result.workload.unwrap_or(0))
    }
}
