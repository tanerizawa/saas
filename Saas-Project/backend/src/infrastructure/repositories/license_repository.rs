#![allow(dead_code)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::licenses::{
    ApplicationStatus, ApplicationStatusHistory, License, LicenseDocument, LicenseType,
};
use crate::domain::dto::LicenseDto;

// Import LicenseRepository trait from cached_license_repository.rs
use super::cached_license_repository::LicenseRepository;

pub struct PostgresLicenseRepositoryImpl {
    pool: PgPool,
}

impl PostgresLicenseRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LicenseRepository for PostgresLicenseRepositoryImpl {
    async fn create_license(&self, license: &License) -> Result<License, sqlx::Error> {
        let query = r#"
            INSERT INTO licenses (
                id, license_number, license_type, company_id, user_id,
                title, description, issue_date, expiry_date, issuing_authority,
                application_status, priority, estimated_processing_days, actual_processing_days,
                external_reference_id, government_fee, service_fee, created_at, updated_at,
                submitted_at, approved_at, rejected_at, admin_notes, rejection_reason
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24
            ) RETURNING *
        "#;

        let dto: LicenseDto = license.clone().into();

        let inserted: LicenseDto = sqlx::query_as(query)
            .bind(dto.id)
            .bind(&dto.license_number)
            .bind(dto.license_type)
            .bind(dto.company_id)
            .bind(dto.user_id)
            .bind(&dto.title)
            .bind(&dto.description)
            .bind(dto.issue_date)
            .bind(dto.expiry_date)
            .bind(&dto.issuing_authority)
            .bind(dto.application_status)
            .bind(dto.priority)
            .bind(dto.estimated_processing_days)
            .bind(dto.actual_processing_days)
            .bind(&dto.external_reference_id)
            .bind(dto.government_fee)
            .bind(dto.service_fee)
            .bind(dto.created_at)
            .bind(dto.updated_at)
            .bind(dto.submitted_at)
            .bind(dto.approved_at)
            .bind(dto.rejected_at)
            .bind(&dto.admin_notes)
            .bind(&dto.rejection_reason)
            .fetch_one(&self.pool)
            .await?;

        Ok(inserted.into())
    }

    async fn get_license_by_id(&self, id: Uuid) -> Result<Option<License>, sqlx::Error> {
        let query = "SELECT * FROM licenses WHERE id = $1";
        let dto = sqlx::query_as::<_, LicenseDto>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(dto.map(|d| d.into()))
    }

    async fn get_licenses_by_user(&self, user_id: Uuid) -> Result<Vec<License>, sqlx::Error> {
        let query = r#"
            SELECT * FROM licenses 
            WHERE user_id = $1 
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query_as::<_, LicenseDto>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(|d| d.into()).collect())
    }
    async fn get_licenses_by_company(&self, company_id: Uuid) -> Result<Vec<License>, sqlx::Error> {
        let query = r#"
            SELECT * FROM licenses 
            WHERE company_id = $1 
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query_as::<_, LicenseDto>(query)
            .bind(company_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(|d| d.into()).collect())
    }

    // For the rest of the implementation, let's keep the original code...
    // Just implement enough for the compiler to be satisfied

    // Similar placeholders for the remaining methods
    async fn update_license(&self, license: &License) -> Result<License, sqlx::Error> {
        let query = r#"
            UPDATE licenses
            SET
                license_number = $1,
                license_type = $2,
                company_id = $3,
                user_id = $4,
                title = $5,
                description = $6,
                issue_date = $7,
                expiry_date = $8,
                issuing_authority = $9,
                application_status = $10,
                priority = $11,
                estimated_processing_days = $12,
                actual_processing_days = $13,
                external_reference_id = $14,
                government_fee = $15,
                service_fee = $16,
                updated_at = $17,
                submitted_at = $18,
                approved_at = $19,
                rejected_at = $20,
                admin_notes = $21,
                rejection_reason = $22
            WHERE id = $23
            RETURNING *
        "#;

        let dto: LicenseDto = license.clone().into();

        let updated: LicenseDto = sqlx::query_as(query)
            .bind(&dto.license_number)
            .bind(dto.license_type)
            .bind(dto.company_id)
            .bind(dto.user_id)
            .bind(&dto.title)
            .bind(&dto.description)
            .bind(dto.issue_date)
            .bind(dto.expiry_date)
            .bind(&dto.issuing_authority)
            .bind(dto.application_status)
            .bind(dto.priority)
            .bind(dto.estimated_processing_days)
            .bind(dto.actual_processing_days)
            .bind(&dto.external_reference_id)
            .bind(dto.government_fee)
            .bind(dto.service_fee)
            .bind(dto.updated_at)
            .bind(dto.submitted_at)
            .bind(dto.approved_at)
            .bind(dto.rejected_at)
            .bind(&dto.admin_notes)
            .bind(&dto.rejection_reason)
            .bind(dto.id)
            .fetch_one(&self.pool)
            .await?;

        Ok(updated.into())
    }

    async fn delete_license(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM licenses WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn get_licenses_by_status(
        &self,
        status: ApplicationStatus,
    ) -> Result<Vec<License>, sqlx::Error> {
        let rows = sqlx::query_as::<_, LicenseDto>(
            "SELECT * FROM licenses WHERE application_status = $1 ORDER BY created_at DESC",
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|d| d.into()).collect())
    }

    async fn get_licenses_by_type(
        &self,
        license_type: LicenseType,
    ) -> Result<Vec<License>, sqlx::Error> {
        let rows = sqlx::query_as::<_, LicenseDto>(
            "SELECT * FROM licenses WHERE license_type = $1 ORDER BY created_at DESC",
        )
        .bind(license_type)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|d| d.into()).collect())
    }

    async fn get_expiring_licenses(&self, days_ahead: i32) -> Result<Vec<License>, sqlx::Error> {
        let query = r#"
            SELECT * FROM licenses
            WHERE expiry_date IS NOT NULL
              AND expiry_date <= NOW() + ($1 || ' days')::INTERVAL
            ORDER BY expiry_date ASC
        "#;

        let licenses = sqlx::query_as::<_, License>(query)
            .bind(days_ahead)
            .fetch_all(&self.pool)
            .await?;

        Ok(licenses)
    }

    async fn search_licenses(
        &self,
        query: &str,
        user_id: Option<Uuid>,
    ) -> Result<Vec<License>, sqlx::Error> {
        let like_query = format!("%{}%", query);

        let sql = if user_id.is_some() {
            r#"
                SELECT * FROM licenses
                WHERE user_id = $1
                  AND (title ILIKE $2 OR license_number ILIKE $2)
                ORDER BY created_at DESC
            "#
        } else {
            r#"
                SELECT * FROM licenses
                WHERE title ILIKE $1 OR license_number ILIKE $1
                ORDER BY created_at DESC
            "#
        };

        let rows = if let Some(uid) = user_id {
            sqlx::query_as::<_, LicenseDto>(sql)
                .bind(uid)
                .bind(&like_query)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, LicenseDto>(sql)
                .bind(&like_query)
                .fetch_all(&self.pool)
                .await?
        };

        Ok(rows.into_iter().map(|d| d.into()).collect())
    }

    async fn create_document(
        &self,
        document: &LicenseDocument,
    ) -> Result<LicenseDocument, sqlx::Error> {
        let query = r#"
            INSERT INTO license_documents (
                id, license_id, document_type, file_name, original_file_name,
                file_path, file_size, mime_type, upload_date, is_verified,
                verified_at, verified_by, notes
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
            ) RETURNING *
        "#;

        let inserted = sqlx::query_as::<_, LicenseDocument>(query)
            .bind(document.id)
            .bind(document.license_id)
            .bind(&document.document_type)
            .bind(&document.file_name)
            .bind(&document.original_file_name)
            .bind(&document.file_path)
            .bind(document.file_size)
            .bind(&document.mime_type)
            .bind(document.upload_date)
            .bind(document.is_verified)
            .bind(document.verified_at)
            .bind(document.verified_by)
            .bind(&document.notes)
            .fetch_one(&self.pool)
            .await?;

        Ok(inserted)
    }

    async fn get_documents_by_license(
        &self,
        license_id: Uuid,
    ) -> Result<Vec<LicenseDocument>, sqlx::Error> {
        let rows = sqlx::query_as::<_, LicenseDocument>(
            "SELECT * FROM license_documents WHERE license_id = $1 ORDER BY upload_date ASC",
        )
        .bind(license_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_document_by_id(&self, id: Uuid) -> Result<Option<LicenseDocument>, sqlx::Error> {
        let row = sqlx::query_as::<_, LicenseDocument>(
            "SELECT * FROM license_documents WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }

    async fn update_document(
        &self,
        document: &LicenseDocument,
    ) -> Result<LicenseDocument, sqlx::Error> {
        let query = r#"
            UPDATE license_documents
            SET
                document_type = $1,
                file_name = $2,
                original_file_name = $3,
                file_path = $4,
                file_size = $5,
                mime_type = $6,
                upload_date = $7,
                is_verified = $8,
                verified_at = $9,
                verified_by = $10,
                notes = $11
            WHERE id = $12
            RETURNING *
        "#;

        let updated = sqlx::query_as::<_, LicenseDocument>(query)
            .bind(&document.document_type)
            .bind(&document.file_name)
            .bind(&document.original_file_name)
            .bind(&document.file_path)
            .bind(document.file_size)
            .bind(&document.mime_type)
            .bind(document.upload_date)
            .bind(document.is_verified)
            .bind(document.verified_at)
            .bind(document.verified_by)
            .bind(&document.notes)
            .bind(document.id)
            .fetch_one(&self.pool)
            .await?;

        Ok(updated)
    }

    async fn delete_document(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM license_documents WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn create_status_history(
        &self,
        history: &ApplicationStatusHistory,
    ) -> Result<ApplicationStatusHistory, sqlx::Error> {
        let query = r#"
            INSERT INTO application_status_history (
                id, license_id, from_status, to_status, changed_by,
                changed_at, notes, is_system_generated
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            ) RETURNING *
        "#;

        let inserted = sqlx::query_as::<_, ApplicationStatusHistory>(query)
            .bind(history.id)
            .bind(history.license_id)
            .bind(history.from_status)
            .bind(history.to_status)
            .bind(history.changed_by)
            .bind(history.changed_at)
            .bind(&history.notes)
            .bind(history.is_system_generated)
            .fetch_one(&self.pool)
            .await?;

        Ok(inserted)
    }

    async fn get_status_history_by_license(
        &self,
        license_id: Uuid,
    ) -> Result<Vec<ApplicationStatusHistory>, sqlx::Error> {
        let rows = sqlx::query_as::<_, ApplicationStatusHistory>(
            "SELECT * FROM application_status_history WHERE license_id = $1 ORDER BY changed_at ASC",
        )
        .bind(license_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn submit_license_application(
        &self,
        license_id: Uuid,
        user_id: Uuid,
    ) -> Result<License, sqlx::Error> {
        let query = r#"
            UPDATE licenses
            SET application_status = 'submitted',
                submitted_at = NOW(),
                updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING *
        "#;

        let dto: LicenseDto = sqlx::query_as(query)
            .bind(license_id)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(dto.into())
    }

    async fn approve_license(
        &self,
        license_id: Uuid,
        _admin_user_id: Uuid,
        license_number: String,
        issue_date: DateTime<Utc>,
        expiry_date: Option<DateTime<Utc>>,
        issuing_authority: String,
        admin_notes: Option<String>,
    ) -> Result<License, sqlx::Error> {
        let query = r#"
            UPDATE licenses
            SET application_status = 'approved',
                license_number = $2,
                issue_date = $3,
                expiry_date = $4,
                issuing_authority = $5,
                admin_notes = $6,
                approved_at = NOW(),
                updated_at = NOW(),
                actual_processing_days = CASE WHEN submitted_at IS NOT NULL THEN EXTRACT(DAY FROM (NOW() - submitted_at))::INT END
            WHERE id = $1
            RETURNING *
        "#;

        let dto: LicenseDto = sqlx::query_as(query)
            .bind(license_id)
            .bind(&license_number)
            .bind(issue_date)
            .bind(expiry_date)
            .bind(&issuing_authority)
            .bind(&admin_notes)
            .fetch_one(&self.pool)
            .await?;

        Ok(dto.into())
    }

    async fn reject_license(
        &self,
        license_id: Uuid,
        _admin_user_id: Uuid,
        reason: String,
        admin_notes: Option<String>,
    ) -> Result<License, sqlx::Error> {
        let query = r#"
            UPDATE licenses
            SET application_status = 'rejected',
                rejection_reason = $2,
                admin_notes = $3,
                rejected_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        let dto: LicenseDto = sqlx::query_as(query)
            .bind(license_id)
            .bind(&reason)
            .bind(&admin_notes)
            .fetch_one(&self.pool)
            .await?;

        Ok(dto.into())
    }

    async fn get_license_statistics(
        &self,
        user_id: Option<Uuid>,
    ) -> Result<LicenseStatistics, sqlx::Error> {
        let query = if let Some(_user_id) = user_id {
            r#"
                SELECT 
                    COUNT(*) as total_licenses,
                    COUNT(*) FILTER (WHERE application_status = 'draft') as draft_count,
                    COUNT(*) FILTER (WHERE application_status = 'submitted') as submitted_count,
                    COUNT(*) FILTER (WHERE application_status = 'processing') as processing_count,
                    COUNT(*) FILTER (WHERE application_status = 'approved') as approved_count,
                    COUNT(*) FILTER (WHERE application_status = 'rejected') as rejected_count,
                    AVG(actual_processing_days) FILTER (WHERE actual_processing_days IS NOT NULL) as avg_processing_days
                FROM licenses 
                WHERE user_id = $1
            "#
        } else {
            r#"
                SELECT 
                    COUNT(*) as total_licenses,
                    COUNT(*) FILTER (WHERE application_status = 'draft') as draft_count,
                    COUNT(*) FILTER (WHERE application_status = 'submitted') as submitted_count,
                    COUNT(*) FILTER (WHERE application_status = 'processing') as processing_count,
                    COUNT(*) FILTER (WHERE application_status = 'approved') as approved_count,
                    COUNT(*) FILTER (WHERE application_status = 'rejected') as rejected_count,
                    AVG(actual_processing_days) FILTER (WHERE actual_processing_days IS NOT NULL) as avg_processing_days
                FROM licenses
            "#
        };

        let row = if let Some(user_id) = user_id {
            sqlx::query(query)
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?
        } else {
            sqlx::query(query).fetch_one(&self.pool).await?
        };

        Ok(LicenseStatistics {
            total_licenses: row.get::<i64, _>("total_licenses"),
            draft_count: row.get::<i64, _>("draft_count"),
            submitted_count: row.get::<i64, _>("submitted_count"),
            processing_count: row.get::<i64, _>("processing_count"),
            approved_count: row.get::<i64, _>("approved_count"),
            rejected_count: row.get::<i64, _>("rejected_count"),
            avg_processing_days: row.get::<Option<f64>, _>("avg_processing_days"),
        })
    }

    async fn get_license_count_by_type(&self) -> Result<Vec<(LicenseType, i64)>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT license_type, COUNT(*) as count FROM licenses GROUP BY license_type",
        )
        .map(|row: sqlx::postgres::PgRow| (row.get("license_type"), row.get::<i64, _>("count")))
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_license_count_by_status(
        &self,
    ) -> Result<Vec<(ApplicationStatus, i64)>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT application_status, COUNT(*) as count FROM licenses GROUP BY application_status",
        )
        .map(|row: sqlx::postgres::PgRow| {
            (row.get("application_status"), row.get::<i64, _>("count"))
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    async fn get_processing_times(&self) -> Result<Vec<(LicenseType, f64)>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT license_type, AVG(actual_processing_days)::float AS avg_days FROM licenses WHERE actual_processing_days IS NOT NULL GROUP BY license_type",
        )
        .map(|row: sqlx::postgres::PgRow| {
            (row.get("license_type"), row.get::<f64, _>("avg_days"))
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}

// Supporting structs for analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseStatistics {
    pub total_licenses: i64,
    pub draft_count: i64,
    pub submitted_count: i64,
    pub processing_count: i64,
    pub approved_count: i64,
    pub rejected_count: i64,
    pub avg_processing_days: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingPerformance {
    pub license_type: LicenseType,
    pub total_processed: i64,
    pub avg_processing_days: Option<f64>,
    pub min_processing_days: Option<i32>,
    pub max_processing_days: Option<i32>,
    pub approved_count: i64,
    pub rejected_count: i64,
}
