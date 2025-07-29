use sqlx::PgPool;
use uuid::Uuid;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::shared::errors::AppError;
use crate::domain::value_objects::{UserId, LicenseId, CompanyId};

// Enhanced domain models for license processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseApplication {
    pub id: LicenseId,
    pub company_id: CompanyId,
    pub license_type: String,
    pub application_data: serde_json::Value,
    pub current_stage: i32,
    pub total_stages: i32,
    pub assigned_reviewer_id: Option<UserId>,
    pub status: ApplicationStatus,
    pub priority: PriorityLevel,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseReview {
    pub id: Uuid,
    pub application_id: LicenseId,
    pub reviewer_id: UserId,
    pub stage: i32,
    pub decision: ReviewDecision,
    pub comments: Option<String>,
    pub review_data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApplicationStatus {
    Submitted,
    UnderReview,
    RequiredDocuments,
    Processing,
    PendingDocuments,
    Approved,
    Rejected,
    OnHold,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReviewDecision {
    Approve,
    Reject,
    RequestRevision,
    Escalate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
}

// Create structs for new entities
#[derive(Debug, Clone)]
pub struct LicenseApplicationCreate {
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub license_type: String,
    pub business_description: String,
    pub required_documents: Vec<String>,
    pub additional_info: HashMap<String, String>,
    pub status: ApplicationStatus,
    pub priority: PriorityLevel,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LicenseReviewCreate {
    pub license_id: LicenseId,
    pub reviewer_id: UserId,
    pub decision: ReviewDecision,
    pub comments: String,
    pub review_date: DateTime<Utc>,
    pub recommendations: Option<Vec<String>>,
}

// Enhanced repository trait with complete CRUD operations
#[async_trait]
pub trait LicenseProcessingRepository: Send + Sync {
    // Application management
    async fn create_application(&self, application: LicenseApplicationCreate) -> Result<LicenseApplication, AppError>;
    async fn get_application(&self, application_id: LicenseId) -> Result<Option<LicenseApplication>, AppError>;
    async fn update_application(&self, application: &LicenseApplication) -> Result<(), AppError>;
    async fn get_applications_by_company(&self, company_id: &CompanyId) -> Result<Vec<LicenseApplication>, AppError>;
    async fn get_applications_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<LicenseApplication>, AppError>;
    async fn get_applications_by_status(&self, status: &ApplicationStatus) -> Result<Vec<LicenseApplication>, AppError>;
    
    // Review management
    async fn create_review(&self, review: LicenseReviewCreate) -> Result<LicenseReview, AppError>;
    async fn get_reviews_for_application(&self, application_id: &LicenseId) -> Result<Vec<LicenseReview>, AppError>;
    async fn get_latest_review(&self, application_id: &LicenseId) -> Result<Option<LicenseReview>, AppError>;
    
    // Workflow management
    async fn assign_reviewer(&self, application_id: LicenseId, reviewer_id: UserId) -> Result<LicenseApplication, AppError>;
    async fn advance_stage(&self, application_id: &LicenseId) -> Result<(), AppError>;
    async fn update_status(&self, application_id: LicenseId, status: ApplicationStatus) -> Result<LicenseApplication, AppError>;
    async fn set_priority(&self, application_id: &LicenseId, priority: &PriorityLevel) -> Result<(), AppError>;
    
    // Statistics and analytics
    async fn get_workflows_count(&self) -> Result<i64, AppError>;
    async fn get_reviewer_workload(&self, reviewer_id: &UserId) -> Result<i64, AppError>;
    async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, AppError>;
    async fn get_applications_by_priority(&self, priority: &PriorityLevel) -> Result<Vec<LicenseApplication>, AppError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingStatistics {
    pub total_applications: i64,
    pub pending_applications: i64,
    pub approved_applications: i64,
    pub rejected_applications: i64,
    pub average_processing_time_hours: f64,
    pub applications_by_priority: std::collections::HashMap<String, i64>,
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
    // Application management - adapted for current schema
    async fn create_application(&self, application: &LicenseApplication) -> Result<LicenseId, AppError> {
        // First create the license record
        let license_result = sqlx::query!(
            r#"
            INSERT INTO licenses 
            (id, company_id, license_type, status, metadata, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            application.id.as_uuid(),
            application.company_id.as_uuid(),
            application.license_type,
            serde_json::to_string(&application.status).unwrap(),
            application.application_data,
            application.created_at,
            application.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        // Then create the workflow record
        sqlx::query!(
            r#"
            INSERT INTO license_processing_workflows 
            (id, license_id, current_stage, total_stages, priority, assigned_reviewer_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            Uuid::new_v4(),
            license_result.id,
            application.current_stage,
            application.total_stages,
            serde_json::to_string(&application.priority).unwrap(),
            application.assigned_reviewer_id.as_ref().map(|id| id.as_uuid()),
            application.created_at,
            application.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(LicenseId::from_uuid(license_result.id))
    }

    async fn get_application(&self, application_id: &LicenseId) -> Result<Option<LicenseApplication>, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT l.id, l.company_id, l.license_type, l.status, l.metadata, l.created_at, l.updated_at,
                   w.current_stage, w.total_stages, w.priority, w.assigned_reviewer_id
            FROM licenses l
            JOIN license_processing_workflows w ON l.id = w.license_id
            WHERE l.id = $1
            "#,
            application_id.as_uuid()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::from)?;

        if let Some(row) = result {
            Ok(Some(LicenseApplication {
                id: LicenseId::from_uuid(row.id),
                company_id: CompanyId::from_uuid(row.company_id),
                license_type: row.license_type,
                application_data: row.metadata.unwrap_or_default(),
                current_stage: row.current_stage,
                total_stages: row.total_stages,
                assigned_reviewer_id: row.assigned_reviewer_id.map(UserId::from_uuid),
                status: serde_json::from_str(&row.status).unwrap_or(ApplicationStatus::Submitted),
                priority: serde_json::from_str(&row.priority).unwrap_or(PriorityLevel::Medium),
                estimated_completion: None,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_application(&self, application: &LicenseApplication) -> Result<(), AppError> {
        // Update license record
        sqlx::query!(
            r#"
            UPDATE licenses 
            SET license_type = $2, status = $3, metadata = $4, updated_at = $5
            WHERE id = $1
            "#,
            application.id.as_uuid(),
            application.license_type,
            serde_json::to_string(&application.status).unwrap(),
            application.application_data,
            application.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        // Update workflow record
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET current_stage = $2, total_stages = $3, priority = $4, assigned_reviewer_id = $5, updated_at = $6
            WHERE license_id = $1
            "#,
            application.id.as_uuid(),
            application.current_stage,
            application.total_stages,
            serde_json::to_string(&application.priority).unwrap(),
            application.assigned_reviewer_id.as_ref().map(|id| id.as_uuid()),
            application.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn get_applications_by_company(&self, company_id: &CompanyId) -> Result<Vec<LicenseApplication>, AppError> {
        let results = sqlx::query!(
            r#"
            SELECT l.id, l.company_id, l.license_type, l.status, l.metadata, l.created_at, l.updated_at,
                   w.current_stage, w.total_stages, w.priority, w.assigned_reviewer_id
            FROM licenses l
            JOIN license_processing_workflows w ON l.id = w.license_id
            WHERE l.company_id = $1
            ORDER BY l.created_at DESC
            "#,
            company_id.as_uuid()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let applications = results.into_iter().map(|row| {
            LicenseApplication {
                id: LicenseId::from_uuid(row.id),
                company_id: CompanyId::from_uuid(row.company_id),
                license_type: row.license_type,
                application_data: row.metadata.unwrap_or_default(),
                current_stage: row.current_stage,
                total_stages: row.total_stages,
                assigned_reviewer_id: row.assigned_reviewer_id.map(UserId::from_uuid),
                status: serde_json::from_str(&row.status).unwrap_or(ApplicationStatus::Submitted),
                priority: serde_json::from_str(&row.priority).unwrap_or(PriorityLevel::Medium),
                estimated_completion: None,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(applications)
    }

    async fn get_applications_by_reviewer(&self, reviewer_id: &UserId) -> Result<Vec<LicenseApplication>, AppError> {
        let results = sqlx::query!(
            r#"
            SELECT l.id, l.company_id, l.license_type, l.status, l.metadata, l.created_at, l.updated_at,
                   w.current_stage, w.total_stages, w.priority, w.assigned_reviewer_id
            FROM licenses l
            JOIN license_processing_workflows w ON l.id = w.license_id
            WHERE w.assigned_reviewer_id = $1
            ORDER BY w.priority DESC, l.created_at ASC
            "#,
            reviewer_id.as_uuid()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let applications = results.into_iter().map(|row| {
            LicenseApplication {
                id: LicenseId::from_uuid(row.id),
                company_id: CompanyId::from_uuid(row.company_id),
                license_type: row.license_type,
                application_data: row.metadata.unwrap_or_default(),
                current_stage: row.current_stage,
                total_stages: row.total_stages,
                assigned_reviewer_id: row.assigned_reviewer_id.map(UserId::from_uuid),
                status: serde_json::from_str(&row.status).unwrap_or(ApplicationStatus::Submitted),
                priority: serde_json::from_str(&row.priority).unwrap_or(PriorityLevel::Medium),
                estimated_completion: None,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(applications)
    }

    async fn get_applications_by_status(&self, status: &ApplicationStatus) -> Result<Vec<LicenseApplication>, AppError> {
        let status_str = serde_json::to_string(status).unwrap();
        let results = sqlx::query!(
            r#"
            SELECT l.id, l.company_id, l.license_type, l.status, l.metadata, l.created_at, l.updated_at,
                   w.current_stage, w.total_stages, w.priority, w.assigned_reviewer_id
            FROM licenses l
            JOIN license_processing_workflows w ON l.id = w.license_id
            WHERE l.status = $1
            ORDER BY w.priority DESC, l.created_at ASC
            "#,
            status_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let applications = results.into_iter().map(|row| {
            LicenseApplication {
                id: LicenseId::from_uuid(row.id),
                company_id: CompanyId::from_uuid(row.company_id),
                license_type: row.license_type,
                application_data: row.metadata.unwrap_or_default(),
                current_stage: row.current_stage,
                total_stages: row.total_stages,
                assigned_reviewer_id: row.assigned_reviewer_id.map(UserId::from_uuid),
                status: serde_json::from_str(&row.status).unwrap_or(ApplicationStatus::Submitted),
                priority: serde_json::from_str(&row.priority).unwrap_or(PriorityLevel::Medium),
                estimated_completion: None,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(applications)
    }

    // Review management - simplified for now without reviews table
    async fn create_review(&self, review: &LicenseReview) -> Result<Uuid, AppError> {
        // For now, just update the workflow with processing notes
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET processing_notes = $2, updated_at = NOW()
            WHERE license_id = $1
            "#,
            review.application_id.as_uuid(),
            review.comments
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(review.id)
    }

    async fn get_reviews_for_application(&self, application_id: &LicenseId) -> Result<Vec<LicenseReview>, AppError> {
        // For now, return empty as we don't have reviews table
        Ok(vec![])
    }

    async fn get_latest_review(&self, application_id: &LicenseId) -> Result<Option<LicenseReview>, AppError> {
        // For now, return None as we don't have reviews table
        Ok(None)
    }

    // Workflow management
    async fn assign_reviewer(&self, application_id: &LicenseId, reviewer_id: &UserId) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET assigned_reviewer_id = $2, updated_at = NOW()
            WHERE license_id = $1
            "#,
            application_id.as_uuid(),
            reviewer_id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn advance_stage(&self, application_id: &LicenseId) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET current_stage = current_stage + 1, updated_at = NOW()
            WHERE license_id = $1 AND current_stage < total_stages
            "#,
            application_id.as_uuid()
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn update_status(&self, application_id: &LicenseId, status: &ApplicationStatus) -> Result<(), AppError> {
        let status_str = serde_json::to_string(status).unwrap();
        sqlx::query!(
            r#"
            UPDATE licenses 
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            "#,
            application_id.as_uuid(),
            status_str
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    async fn set_priority(&self, application_id: &LicenseId, priority: &PriorityLevel) -> Result<(), AppError> {
        let priority_str = serde_json::to_string(priority).unwrap();
        sqlx::query!(
            r#"
            UPDATE license_processing_workflows 
            SET priority = $2, updated_at = NOW()
            WHERE license_id = $1
            "#,
            application_id.as_uuid(),
            priority_str
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }

    // Statistics and analytics
    async fn get_workflows_count(&self) -> Result<i64, AppError> {
        let result = sqlx::query!(
            r#"SELECT COUNT(*) as total FROM license_processing_workflows"#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result.total.unwrap_or(0))
    }

    async fn get_reviewer_workload(&self, reviewer_id: &UserId) -> Result<i64, AppError> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as workload
            FROM license_processing_workflows 
            WHERE assigned_reviewer_id = $1 AND current_stage < total_stages
            "#,
            reviewer_id.as_uuid()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        Ok(result.workload.unwrap_or(0))
    }

    async fn get_processing_statistics(&self) -> Result<ProcessingStatistics, AppError> {
        // Get total counts by status
        let total = sqlx::query!(r#"SELECT COUNT(*) as count FROM license_processing_workflows"#)
            .fetch_one(&self.pool).await.map_err(AppError::from)?.count.unwrap_or(0);

        let pending = sqlx::query!(r#"SELECT COUNT(*) as count FROM licenses WHERE status IN ('"Submitted"', '"UnderReview"', '"RequiredDocuments"')"#)
            .fetch_one(&self.pool).await.map_err(AppError::from)?.count.unwrap_or(0);

        let approved = sqlx::query!(r#"SELECT COUNT(*) as count FROM licenses WHERE status = '"Approved"'"#)
            .fetch_one(&self.pool).await.map_err(AppError::from)?.count.unwrap_or(0);

        let rejected = sqlx::query!(r#"SELECT COUNT(*) as count FROM licenses WHERE status = '"Rejected"'"#)
            .fetch_one(&self.pool).await.map_err(AppError::from)?.count.unwrap_or(0);

        // Calculate average processing time
        let avg_time_result = sqlx::query!(
            r#"
            SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at))/3600) as avg_hours
            FROM licenses 
            WHERE status IN ('"Approved"', '"Rejected"')
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::from)?;

        let avg_time = avg_time_result.avg_hours
            .and_then(|bd| bd.to_string().parse::<f64>().ok())
            .unwrap_or(0.0);

        // Get priority distribution
        let mut priority_map = HashMap::new();
        let priority_results = sqlx::query!(
            r#"SELECT priority, COUNT(*) as count FROM license_processing_workflows GROUP BY priority"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        for row in priority_results {
            priority_map.insert(row.priority, row.count.unwrap_or(0));
        }

        Ok(ProcessingStatistics {
            total_applications: total,
            pending_applications: pending,
            approved_applications: approved,
            rejected_applications: rejected,
            average_processing_time_hours: avg_time,
            applications_by_priority: priority_map,
        })
    }

    async fn get_applications_by_priority(&self, priority: &PriorityLevel) -> Result<Vec<LicenseApplication>, AppError> {
        let priority_str = serde_json::to_string(priority).unwrap();
        let results = sqlx::query!(
            r#"
            SELECT l.id, l.company_id, l.license_type, l.status, l.metadata, l.created_at, l.updated_at,
                   w.current_stage, w.total_stages, w.priority, w.assigned_reviewer_id
            FROM licenses l
            JOIN license_processing_workflows w ON l.id = w.license_id
            WHERE w.priority = $1
            ORDER BY l.created_at ASC
            "#,
            priority_str
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::from)?;

        let applications = results.into_iter().map(|row| {
            LicenseApplication {
                id: LicenseId::from_uuid(row.id),
                company_id: CompanyId::from_uuid(row.company_id),
                license_type: row.license_type,
                application_data: row.metadata.unwrap_or_default(),
                current_stage: row.current_stage,
                total_stages: row.total_stages,
                assigned_reviewer_id: row.assigned_reviewer_id.map(UserId::from_uuid),
                status: serde_json::from_str(&row.status).unwrap_or(ApplicationStatus::Submitted),
                priority: serde_json::from_str(&row.priority).unwrap_or(PriorityLevel::Medium),
                estimated_completion: None,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }
        }).collect();

        Ok(applications)
    }
}
