// Data Transfer Objects for mapping between database and domain models
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::licenses::{ApplicationStatus, License, LicenseType, PriorityLevel};

// DTO for License entity that matches database schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LicenseDto {
    pub id: Uuid,
    pub license_number: Option<String>,
    pub license_type: LicenseType,
    pub company_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub issue_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub issuing_authority: Option<String>,
    pub application_status: ApplicationStatus,
    pub priority: PriorityLevel,
    pub estimated_processing_days: Option<i32>,
    pub actual_processing_days: Option<i32>,
    pub external_reference_id: Option<String>,
    pub government_fee: Option<i64>,
    pub service_fee: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub admin_notes: Option<String>,
    pub rejection_reason: Option<String>,
}

// Conversion from DTO to domain entity
impl From<LicenseDto> for License {
    fn from(dto: LicenseDto) -> Self {
        Self {
            id: dto.id,
            license_number: dto.license_number,
            license_type: dto.license_type,
            company_id: dto.company_id,
            user_id: dto.user_id,
            title: dto.title,
            description: dto.description,
            issue_date: dto.issue_date,
            expiry_date: dto.expiry_date,
            issuing_authority: dto.issuing_authority,
            application_status: dto.application_status,
            priority: dto.priority,
            estimated_processing_days: dto.estimated_processing_days,
            actual_processing_days: dto.actual_processing_days,
            external_reference_id: dto.external_reference_id,
            government_fee: dto.government_fee,
            service_fee: dto.service_fee,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            submitted_at: dto.submitted_at,
            approved_at: dto.approved_at,
            rejected_at: dto.rejected_at,
            admin_notes: dto.admin_notes,
            rejection_reason: dto.rejection_reason,
        }
    }
}

// Conversion from domain entity to DTO
impl From<License> for LicenseDto {
    fn from(entity: License) -> Self {
        Self {
            id: entity.id,
            license_number: entity.license_number,
            license_type: entity.license_type,
            company_id: entity.company_id,
            user_id: entity.user_id,
            title: entity.title,
            description: entity.description,
            issue_date: entity.issue_date,
            expiry_date: entity.expiry_date,
            issuing_authority: entity.issuing_authority,
            application_status: entity.application_status,
            priority: entity.priority,
            estimated_processing_days: entity.estimated_processing_days,
            actual_processing_days: entity.actual_processing_days,
            external_reference_id: entity.external_reference_id,
            government_fee: entity.government_fee,
            service_fee: entity.service_fee,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            submitted_at: entity.submitted_at,
            approved_at: entity.approved_at,
            rejected_at: entity.rejected_at,
            admin_notes: entity.admin_notes,
            rejection_reason: entity.rejection_reason,
        }
    }
}
