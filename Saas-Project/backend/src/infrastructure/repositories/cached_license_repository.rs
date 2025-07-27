#![allow(dead_code)]

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tracing::{debug, error, instrument};
use uuid::Uuid;

use crate::domain::licenses::{
    ApplicationStatus, ApplicationStatusHistory, License, LicenseDocument, LicenseType,
};

use crate::infrastructure::repositories::license_repository::LicenseStatistics;

use crate::infrastructure::cache::CacheService;

#[async_trait]
pub trait Cache: Send + Sync {
    async fn set<T: serde::Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        expiry_secs: Option<u64>,
    ) -> Result<(), redis::RedisError>;

    async fn get<T: serde::de::DeserializeOwned + Send + Sync>(
        &self,
        key: &str,
    ) -> Result<Option<T>, redis::RedisError>;

    async fn delete(&self, key: &str) -> Result<(), redis::RedisError>;
    async fn delete_by_pattern(&self, pattern: &str) -> Result<(), redis::RedisError>;
}

#[async_trait]
impl Cache for CacheService {
    async fn set<T: serde::Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        expiry_secs: Option<u64>,
    ) -> Result<(), redis::RedisError> {
        self.set(key, value, expiry_secs).await
    }

    async fn get<T: serde::de::DeserializeOwned + Send + Sync>(
        &self,
        key: &str,
    ) -> Result<Option<T>, redis::RedisError> {
        self.get(key).await
    }

    async fn delete(&self, key: &str) -> Result<(), redis::RedisError> {
        self.delete(key).await
    }

    async fn delete_by_pattern(&self, pattern: &str) -> Result<(), redis::RedisError> {
        self.delete_by_pattern(pattern).await
    }
}

#[async_trait]
pub trait LicenseRepository: Send + Sync {
    // License CRUD operations
    async fn create_license(&self, license: &License) -> Result<License, sqlx::Error>;
    async fn get_license_by_id(&self, id: Uuid) -> Result<Option<License>, sqlx::Error>;
    async fn get_licenses_by_user(&self, user_id: Uuid) -> Result<Vec<License>, sqlx::Error>;
    async fn get_licenses_by_company(&self, company_id: Uuid) -> Result<Vec<License>, sqlx::Error>;
    async fn update_license(&self, license: &License) -> Result<License, sqlx::Error>;
    async fn delete_license(&self, id: Uuid) -> Result<bool, sqlx::Error>;

    // License queries
    async fn get_licenses_by_status(
        &self,
        status: ApplicationStatus,
    ) -> Result<Vec<License>, sqlx::Error>;
    async fn get_licenses_by_type(
        &self,
        license_type: LicenseType,
    ) -> Result<Vec<License>, sqlx::Error>;
    async fn get_expiring_licenses(&self, days_ahead: i32) -> Result<Vec<License>, sqlx::Error>;
    async fn search_licenses(
        &self,
        query: &str,
        user_id: Option<Uuid>,
    ) -> Result<Vec<License>, sqlx::Error>;

    // Document operations
    async fn create_document(
        &self,
        document: &LicenseDocument,
    ) -> Result<LicenseDocument, sqlx::Error>;
    async fn get_documents_by_license(
        &self,
        license_id: Uuid,
    ) -> Result<Vec<LicenseDocument>, sqlx::Error>;
    async fn get_document_by_id(&self, id: Uuid) -> Result<Option<LicenseDocument>, sqlx::Error>;
    async fn update_document(
        &self,
        document: &LicenseDocument,
    ) -> Result<LicenseDocument, sqlx::Error>;
    async fn delete_document(&self, id: Uuid) -> Result<bool, sqlx::Error>;

    // Status history operations
    async fn create_status_history(
        &self,
        history: &ApplicationStatusHistory,
    ) -> Result<ApplicationStatusHistory, sqlx::Error>;
    async fn get_status_history_by_license(
        &self,
        license_id: Uuid,
    ) -> Result<Vec<ApplicationStatusHistory>, sqlx::Error>;

    // Business logic operations
    async fn submit_license_application(
        &self,
        license_id: Uuid,
        user_id: Uuid,
    ) -> Result<License, sqlx::Error>;
    async fn approve_license(
        &self,
        license_id: Uuid,
        admin_user_id: Uuid,
        license_number: String,
        issue_date: DateTime<Utc>,
        expiry_date: Option<DateTime<Utc>>,
        issuing_authority: String,
        admin_notes: Option<String>,
    ) -> Result<License, sqlx::Error>;
    async fn reject_license(
        &self,
        license_id: Uuid,
        admin_user_id: Uuid,
        reason: String,
        admin_notes: Option<String>,
    ) -> Result<License, sqlx::Error>;

    // Analytics operations
    async fn get_license_count_by_type(&self) -> Result<Vec<(LicenseType, i64)>, sqlx::Error>;
    async fn get_license_count_by_status(
        &self,
    ) -> Result<Vec<(ApplicationStatus, i64)>, sqlx::Error>;
    async fn get_processing_times(&self) -> Result<Vec<(LicenseType, f64)>, sqlx::Error>;
    async fn get_license_statistics(
        &self,
        user_id: Option<Uuid>,
    ) -> Result<LicenseStatistics, sqlx::Error>;
}

pub struct CachedLicenseRepository<C: Cache> {
    pool: PgPool,
    cache: Option<Arc<C>>,
}

impl<C: Cache> CachedLicenseRepository<C> {
    pub fn new(pool: PgPool) -> Self {
        Self { pool, cache: None }
    }

    pub fn new_with_cache(pool: PgPool, cache: Arc<C>) -> Self {
        Self {
            pool,
            cache: Some(cache),
        }
    }

    // Cache key generation helpers
    fn license_cache_key(id: Uuid) -> String {
        format!("license:{}", id)
    }

    fn user_licenses_cache_key(user_id: Uuid) -> String {
        format!("user:{}:licenses", user_id)
    }

    fn company_licenses_cache_key(company_id: Uuid) -> String {
        format!("company:{}:licenses", company_id)
    }

    fn document_cache_key(id: Uuid) -> String {
        format!("document:{}", id)
    }

    fn license_documents_cache_key(license_id: Uuid) -> String {
        format!("license:{}:documents", license_id)
    }

    fn license_status_history_cache_key(license_id: Uuid) -> String {
        format!("license:{}:status_history", license_id)
    }

    fn license_type_cache_key(license_type: LicenseType) -> String {
        format!("licenses:type:{}", license_type.to_string())
    }

    fn license_status_cache_key(status: ApplicationStatus) -> String {
        format!("licenses:status:{}", status.to_string())
    }

    // Helper to invalidate license caches when license is updated
    async fn invalidate_license_cache(
        &self,
        license_id: Uuid,
        user_id: Option<Uuid>,
        company_id: Option<Uuid>,
    ) {
        if let Some(cache) = &self.cache {
            debug!("Invalidating cache for license {}", license_id);

            // Delete specific license cache
            let _ = cache.delete(&Self::license_cache_key(license_id)).await;

            // Delete user licenses cache if user_id is provided
            if let Some(uid) = user_id {
                let _ = cache.delete(&Self::user_licenses_cache_key(uid)).await;
            }

            // Delete company licenses cache if company_id is provided
            if let Some(cid) = company_id {
                let _ = cache.delete(&Self::company_licenses_cache_key(cid)).await;
            }

            // Delete any aggregate caches that might include this license
            let _ = cache.delete_by_pattern("licenses:type:*").await;
            let _ = cache.delete_by_pattern("licenses:status:*").await;
            let _ = cache.delete_by_pattern("analytics:licenses:*").await;
        }
    }
}

#[async_trait]
impl<C: Cache> LicenseRepository for CachedLicenseRepository<C> {
    // Implement repository methods with caching
    // We'll implement a few key methods as examples

    #[instrument(skip(self, license))]
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

        let inserted = sqlx::query_as::<_, License>(query)
            .bind(license.id)
            .bind(&license.license_number)
            .bind(&license.license_type)
            .bind(license.company_id)
            .bind(license.user_id)
            .bind(&license.title)
            .bind(&license.description)
            .bind(license.issue_date)
            .bind(license.expiry_date)
            .bind(&license.issuing_authority)
            .bind(&license.application_status)
            .bind(&license.priority)
            .bind(license.estimated_processing_days)
            .bind(license.actual_processing_days)
            .bind(&license.external_reference_id)
            .bind(license.government_fee)
            .bind(license.service_fee)
            .bind(license.created_at)
            .bind(license.updated_at)
            .bind(license.submitted_at)
            .bind(license.approved_at)
            .bind(license.rejected_at)
            .bind(&license.admin_notes)
            .bind(&license.rejection_reason)
            .fetch_one(&self.pool)
            .await?;

        self
            .invalidate_license_cache(
                inserted.id,
                Some(inserted.user_id),
                Some(inserted.company_id),
            )
            .await;

        if let Some(cache) = &self.cache {
            let cache_key = Self::license_cache_key(inserted.id);
            let _ = cache.set(&cache_key, &inserted, Some(300)).await;
        }

        Ok(inserted)
    }

    #[instrument(skip(self), fields(license_id = %id))]
    async fn get_license_by_id(&self, id: Uuid) -> Result<Option<License>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            let cache_key = Self::license_cache_key(id);
            debug!("Checking cache for license: {}", id);

            match cache.get::<License>(&cache_key).await {
                Ok(Some(license)) => {
                    debug!("Cache hit for license: {}", id);
                    return Ok(Some(license));
                }
                Ok(None) => debug!("Cache miss for license: {}", id),
                Err(e) => error!("Cache error: {}", e),
            }
        }

        // If not in cache or error, get from database
        let license = sqlx::query("SELECT * FROM licenses WHERE id = $1")
            .bind(id)
            .map(|row: sqlx::postgres::PgRow| License {
                id: row.get("id"),
                user_id: row.get("user_id"),
                company_id: row.get("company_id"),
                license_type: row.get("license_type"),
                application_status: row.get("application_status"),
                license_number: row.get("license_number"),
                title: row.get("title"),
                description: row.get("description"),
                issue_date: row.get("issue_date"),
                expiry_date: row.get("expiry_date"),
                issuing_authority: row.get("issuing_authority"),
                priority: row.get("priority"),
                estimated_processing_days: row.get("estimated_processing_days"),
                actual_processing_days: row.get("actual_processing_days"),
                external_reference_id: row.get("external_reference_id"),
                government_fee: row.get("government_fee"),
                service_fee: row.get("service_fee"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                submitted_at: row.get("submitted_at"),
                approved_at: row.get("approved_at"),
                rejected_at: row.get("rejected_at"),
                admin_notes: row.get("admin_notes"),
                rejection_reason: row.get("rejection_reason"),
            })
            .fetch_optional(&self.pool)
            .await?;

        // Cache the result if found
        if let Some(ref license) = license {
            if let Some(cache) = &self.cache {
                let cache_key = Self::license_cache_key(id);
                debug!("Caching license: {}", id);

                // Cache for 5 minutes
                let _ = cache.set(&cache_key, license, Some(300)).await;
            }
        }

        Ok(license)
    }

    #[instrument(skip(self), fields(user_id = %user_id))]
    async fn get_licenses_by_user(&self, user_id: Uuid) -> Result<Vec<License>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            let cache_key = Self::user_licenses_cache_key(user_id);
            debug!("Checking cache for user licenses: {}", user_id);

            match cache.get::<Vec<License>>(&cache_key).await {
                Ok(Some(licenses)) => {
                    debug!("Cache hit for user licenses: {}", user_id);
                    return Ok(licenses);
                }
                Ok(None) => debug!("Cache miss for user licenses: {}", user_id),
                Err(e) => error!("Cache error: {}", e),
            }
        }

        // If not in cache or error, get from database
        let licenses =
            sqlx::query("SELECT * FROM licenses WHERE user_id = $1 ORDER BY created_at DESC")
                .bind(user_id)
                .map(|row: sqlx::postgres::PgRow| License {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    company_id: row.get("company_id"),
                    license_type: row.get("license_type"),
                    application_status: row.get("application_status"),
                    license_number: row.get("license_number"),
                    title: row.get("title"),
                    description: row.get("description"),
                    issue_date: row.get("issue_date"),
                    expiry_date: row.get("expiry_date"),
                    issuing_authority: row.get("issuing_authority"),
                    priority: row.get("priority"),
                    estimated_processing_days: row.get("estimated_processing_days"),
                    actual_processing_days: row.get("actual_processing_days"),
                    external_reference_id: row.get("external_reference_id"),
                    government_fee: row.get("government_fee"),
                    service_fee: row.get("service_fee"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    submitted_at: row.get("submitted_at"),
                    approved_at: row.get("approved_at"),
                    rejected_at: row.get("rejected_at"),
                    admin_notes: row.get("admin_notes"),
                    rejection_reason: row.get("rejection_reason"),
                })
                .fetch_all(&self.pool)
                .await?;

        // Cache the result
        if let Some(cache) = &self.cache {
            let cache_key = Self::user_licenses_cache_key(user_id);
            debug!("Caching user licenses: {}", user_id);

            // Cache for 2 minutes
            let _ = cache.set(&cache_key, &licenses, Some(120)).await;
        }

        Ok(licenses)
    }

    // Analytics implementation
    async fn get_license_statistics(
        &self,
        user_id: Option<Uuid>,
    ) -> Result<LicenseStatistics, sqlx::Error> {
        // Try to get from cache first if available
        if let Some(cache) = &self.cache {
            let cache_key = match user_id {
                Some(uid) => format!("stats:user:{}", uid),
                None => "stats:global".to_string(),
            };

            debug!("Checking cache for license statistics");
            match cache.get::<LicenseStatistics>(&cache_key).await {
                Ok(Some(stats)) => {
                    debug!("Cache hit for license statistics");
                    return Ok(stats);
                }
                Ok(None) => debug!("Cache miss for license statistics"),
                Err(e) => error!("Cache error: {}", e),
            }
        }

        // If not in cache or error, get from database
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

        let stats = LicenseStatistics {
            total_licenses: row.get::<i64, _>("total_licenses"),
            draft_count: row.get::<i64, _>("draft_count"),
            submitted_count: row.get::<i64, _>("submitted_count"),
            processing_count: row.get::<i64, _>("processing_count"),
            approved_count: row.get::<i64, _>("approved_count"),
            rejected_count: row.get::<i64, _>("rejected_count"),
            avg_processing_days: row.get::<Option<f64>, _>("avg_processing_days"),
        };

        // Cache the result
        if let Some(cache) = &self.cache {
            let cache_key = match user_id {
                Some(uid) => format!("stats:user:{}", uid),
                None => "stats:global".to_string(),
            };
            debug!("Caching license statistics");

            // Cache for 2 minutes
            let _ = cache.set(&cache_key, &stats, Some(120)).await;
        }

        Ok(stats)
    }

    async fn get_license_count_by_type(&self) -> Result<Vec<(LicenseType, i64)>, sqlx::Error> {
        if let Some(cache) = &self.cache {
            if let Ok(Some(data)) = cache
                .get::<Vec<(LicenseType, i64)>>("analytics:licenses:type_counts")
                .await
            {
                return Ok(data);
            }
        }

        let rows = sqlx::query(
            "SELECT license_type, COUNT(*) as count FROM licenses GROUP BY license_type",
        )
        .map(|row: sqlx::postgres::PgRow| (row.get("license_type"), row.get::<i64, _>("count")))
        .fetch_all(&self.pool)
        .await?;

        if let Some(cache) = &self.cache {
            let _ = cache
                .set("analytics:licenses:type_counts", &rows, Some(300))
                .await;
        }

        Ok(rows)
    }

    async fn get_license_count_by_status(
        &self,
    ) -> Result<Vec<(ApplicationStatus, i64)>, sqlx::Error> {
        if let Some(cache) = &self.cache {
            if let Ok(Some(data)) = cache
                .get::<Vec<(ApplicationStatus, i64)>>("analytics:licenses:status_counts")
                .await
            {
                return Ok(data);
            }
        }

        let rows = sqlx::query(
            "SELECT application_status, COUNT(*) as count FROM licenses GROUP BY application_status",
        )
        .map(|row: sqlx::postgres::PgRow| {
            (row.get("application_status"), row.get::<i64, _>("count"))
        })
        .fetch_all(&self.pool)
        .await?;

        if let Some(cache) = &self.cache {
            let _ = cache
                .set("analytics:licenses:status_counts", &rows, Some(300))
                .await;
        }

        Ok(rows)
    }

    async fn get_processing_times(&self) -> Result<Vec<(LicenseType, f64)>, sqlx::Error> {
        if let Some(cache) = &self.cache {
            if let Ok(Some(data)) = cache
                .get::<Vec<(LicenseType, f64)>>("analytics:licenses:processing_times")
                .await
            {
                return Ok(data);
            }
        }

        let rows = sqlx::query(
            "SELECT license_type, AVG(actual_processing_days)::float AS avg_days FROM licenses WHERE actual_processing_days IS NOT NULL GROUP BY license_type",
        )
        .map(|row: sqlx::postgres::PgRow| {
            (row.get("license_type"), row.get::<f64, _>("avg_days"))
        })
        .fetch_all(&self.pool)
        .await?;

        if let Some(cache) = &self.cache {
            let _ = cache
                .set("analytics:licenses:processing_times", &rows, Some(300))
                .await;
        }

        Ok(rows)
    }

    // For all the other unimplemented methods, we would add implementations similar to
    // the ones we've already created, with caching logic.

    #[instrument(skip(self), fields(company_id = %company_id))]
    async fn get_licenses_by_company(&self, company_id: Uuid) -> Result<Vec<License>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            let cache_key = Self::company_licenses_cache_key(company_id);
            debug!("Checking cache for company licenses: {}", company_id);

            match cache.get::<Vec<License>>(&cache_key).await {
                Ok(Some(licenses)) => {
                    debug!("Cache hit for company licenses: {}", company_id);
                    return Ok(licenses);
                }
                Ok(None) => debug!("Cache miss for company licenses: {}", company_id),
                Err(e) => error!("Cache error: {}", e),
            }
        }

        // If not in cache or error, get from database
        let licenses =
            sqlx::query("SELECT * FROM licenses WHERE company_id = $1 ORDER BY created_at DESC")
                .bind(company_id)
                .map(|row: sqlx::postgres::PgRow| License {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    company_id: row.get("company_id"),
                    license_type: row.get("license_type"),
                    application_status: row.get("application_status"),
                    license_number: row.get("license_number"),
                    title: row.get("title"),
                    description: row.get("description"),
                    issue_date: row.get("issue_date"),
                    expiry_date: row.get("expiry_date"),
                    issuing_authority: row.get("issuing_authority"),
                    priority: row.get("priority"),
                    estimated_processing_days: row.get("estimated_processing_days"),
                    actual_processing_days: row.get("actual_processing_days"),
                    external_reference_id: row.get("external_reference_id"),
                    government_fee: row.get("government_fee"),
                    service_fee: row.get("service_fee"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    submitted_at: row.get("submitted_at"),
                    approved_at: row.get("approved_at"),
                    rejected_at: row.get("rejected_at"),
                    admin_notes: row.get("admin_notes"),
                    rejection_reason: row.get("rejection_reason"),
                })
                .fetch_all(&self.pool)
                .await?;

        // Cache the result
        if let Some(cache) = &self.cache {
            let cache_key = Self::company_licenses_cache_key(company_id);
            debug!("Caching company licenses: {}", company_id);

            // Cache for 2 minutes
            let _ = cache.set(&cache_key, &licenses, Some(120)).await;
        }

        Ok(licenses)
    }

    #[instrument(skip(self, license))]
    async fn update_license(&self, license: &License) -> Result<License, sqlx::Error> {
        // Update in database
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

        let updated = sqlx::query_as::<_, License>(query)
            .bind(&license.license_number)
            .bind(&license.license_type)
            .bind(license.company_id)
            .bind(license.user_id)
            .bind(&license.title)
            .bind(&license.description)
            .bind(license.issue_date)
            .bind(license.expiry_date)
            .bind(&license.issuing_authority)
            .bind(&license.application_status)
            .bind(&license.priority)
            .bind(license.estimated_processing_days)
            .bind(license.actual_processing_days)
            .bind(&license.external_reference_id)
            .bind(license.government_fee)
            .bind(license.service_fee)
            .bind(license.updated_at)
            .bind(license.submitted_at)
            .bind(license.approved_at)
            .bind(license.rejected_at)
            .bind(&license.admin_notes)
            .bind(&license.rejection_reason)
            .bind(license.id)
            .fetch_one(&self.pool)
            .await?;

        // Invalidate caches after update
        self.invalidate_license_cache(updated.id, Some(updated.user_id), Some(updated.company_id))
            .await;

        if let Some(cache) = &self.cache {
            let cache_key = Self::license_cache_key(updated.id);
            let _ = cache.set(&cache_key, &updated, Some(300)).await;
        }

        Ok(updated)
    }

    #[instrument(skip(self), fields(license_id = %id))]
    async fn delete_license(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        // First, get the license to know user_id and company_id for cache invalidation
        let license = self.get_license_by_id(id).await?;

        if let Some(license) = license {
            // Delete the license
            let result = sqlx::query("DELETE FROM licenses WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await?;

            // Invalidate caches
            self.invalidate_license_cache(id, Some(license.user_id), Some(license.company_id))
                .await;

            Ok(result.rows_affected() > 0)
        } else {
            // License not found
            Ok(false)
        }
    }

    #[instrument(skip(self))]
    async fn get_licenses_by_status(
        &self,
        status: ApplicationStatus,
    ) -> Result<Vec<License>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            let cache_key = Self::license_status_cache_key(status.clone());
            debug!("Checking cache for licenses by status: {}", status);

            match cache.get::<Vec<License>>(&cache_key).await {
                Ok(Some(licenses)) => {
                    debug!("Cache hit for licenses by status: {}", status);
                    return Ok(licenses);
                }
                Ok(None) => debug!("Cache miss for licenses by status: {}", status),
                Err(e) => error!("Cache error: {}", e),
            }
        }

        // If not in cache or error, get from database
        // Create a clone to avoid moving the original status
        let status_clone = status.clone();
        let licenses = sqlx::query(
            "SELECT * FROM licenses WHERE application_status = $1 ORDER BY created_at DESC",
        )
        .bind(status_clone)
        .map(|row: sqlx::postgres::PgRow| License {
            id: row.get("id"),
            user_id: row.get("user_id"),
            company_id: row.get("company_id"),
            license_type: row.get("license_type"),
            application_status: row.get("application_status"),
            license_number: row.get("license_number"),
            title: row.get("title"),
            description: row.get("description"),
            issue_date: row.get("issue_date"),
            expiry_date: row.get("expiry_date"),
            issuing_authority: row.get("issuing_authority"),
            priority: row.get("priority"),
            estimated_processing_days: row.get("estimated_processing_days"),
            actual_processing_days: row.get("actual_processing_days"),
            external_reference_id: row.get("external_reference_id"),
            government_fee: row.get("government_fee"),
            service_fee: row.get("service_fee"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            submitted_at: row.get("submitted_at"),
            approved_at: row.get("approved_at"),
            rejected_at: row.get("rejected_at"),
            admin_notes: row.get("admin_notes"),
            rejection_reason: row.get("rejection_reason"),
        })
        .fetch_all(&self.pool)
        .await?;

        // Cache the result
        if let Some(cache) = &self.cache {
            let cache_key = Self::license_status_cache_key(status.clone());
            debug!("Caching licenses by status");

            // Cache for 1 minute (shorter time for status-based queries which may change frequently)
            let _ = cache.set(&cache_key, &licenses, Some(60)).await;
        }

        Ok(licenses)
    }

    #[instrument(skip(self))]
    async fn get_licenses_by_type(
        &self,
        license_type: LicenseType,
    ) -> Result<Vec<License>, sqlx::Error> {
        // Try to get from cache first
        if let Some(cache) = &self.cache {
            let cache_key = Self::license_type_cache_key(license_type);
            debug!("Checking cache for licenses by type: {}", license_type);

            match cache.get::<Vec<License>>(&cache_key).await {
                Ok(Some(licenses)) => {
                    debug!("Cache hit for licenses by type: {}", license_type);
                    return Ok(licenses);
                }
                Ok(None) => debug!("Cache miss for licenses by type: {}", license_type),
                Err(e) => error!("Cache error: {}", e),
            }
        }

        // If not in cache or error, get from database
        let licenses =
            sqlx::query("SELECT * FROM licenses WHERE license_type = $1 ORDER BY created_at DESC")
                .bind(license_type)
                .map(|row: sqlx::postgres::PgRow| License {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    company_id: row.get("company_id"),
                    license_type: row.get("license_type"),
                    application_status: row.get("application_status"),
                    license_number: row.get("license_number"),
                    title: row.get("title"),
                    description: row.get("description"),
                    issue_date: row.get("issue_date"),
                    expiry_date: row.get("expiry_date"),
                    issuing_authority: row.get("issuing_authority"),
                    priority: row.get("priority"),
                    estimated_processing_days: row.get("estimated_processing_days"),
                    actual_processing_days: row.get("actual_processing_days"),
                    external_reference_id: row.get("external_reference_id"),
                    government_fee: row.get("government_fee"),
                    service_fee: row.get("service_fee"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    submitted_at: row.get("submitted_at"),
                    approved_at: row.get("approved_at"),
                    rejected_at: row.get("rejected_at"),
                    admin_notes: row.get("admin_notes"),
                    rejection_reason: row.get("rejection_reason"),
                })
                .fetch_all(&self.pool)
                .await?;

        // Cache the result
        if let Some(cache) = &self.cache {
            let cache_key = Self::license_type_cache_key(license_type);
            debug!("Caching licenses by type");

            // Cache for 5 minutes
            let _ = cache.set(&cache_key, &licenses, Some(300)).await;
        }

        Ok(licenses)
    }

    async fn get_expiring_licenses(&self, days_ahead: i32) -> Result<Vec<License>, sqlx::Error> {
        let cache_key = format!("licenses:expiring:{}", days_ahead);

        if let Some(cache) = &self.cache {
            if let Ok(Some(licenses)) = cache.get::<Vec<License>>(&cache_key).await {
                return Ok(licenses);
            }
        }

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

        if let Some(cache) = &self.cache {
            let _ = cache.set(&cache_key, &licenses, Some(300)).await;
        }

        Ok(licenses)
    }

    async fn search_licenses(
        &self,
        query: &str,
        user_id: Option<Uuid>,
    ) -> Result<Vec<License>, sqlx::Error> {
        let cache_key = format!(
            "licenses:search:{}:{}",
            user_id
                .map(|u| u.to_string())
                .unwrap_or_else(|| "all".into()),
            query
        );

        if let Some(cache) = &self.cache {
            if let Ok(Some(licenses)) = cache.get::<Vec<License>>(&cache_key).await {
                return Ok(licenses);
            }
        }

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

        let licenses = if let Some(uid) = user_id {
            sqlx::query_as::<_, License>(sql)
                .bind(uid)
                .bind(&like_query)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, License>(sql)
                .bind(&like_query)
                .fetch_all(&self.pool)
                .await?
        };

        if let Some(cache) = &self.cache {
            let _ = cache.set(&cache_key, &licenses, Some(60)).await;
        }

        Ok(licenses)
    }

    async fn create_document(
        &self,
        document: &LicenseDocument,
    ) -> Result<LicenseDocument, sqlx::Error> {
        Ok(document.clone())
    }

    async fn get_documents_by_license(
        &self,
        _license_id: Uuid,
    ) -> Result<Vec<LicenseDocument>, sqlx::Error> {
        Ok(vec![])
    }

    async fn get_document_by_id(&self, _id: Uuid) -> Result<Option<LicenseDocument>, sqlx::Error> {
        Ok(None)
    }

    async fn update_document(
        &self,
        document: &LicenseDocument,
    ) -> Result<LicenseDocument, sqlx::Error> {
        Ok(document.clone())
    }

    async fn delete_document(&self, _id: Uuid) -> Result<bool, sqlx::Error> {
        Ok(true)
    }

    async fn create_status_history(
        &self,
        history: &ApplicationStatusHistory,
    ) -> Result<ApplicationStatusHistory, sqlx::Error> {
        Ok(history.clone())
    }

    async fn get_status_history_by_license(
        &self,
        _license_id: Uuid,
    ) -> Result<Vec<ApplicationStatusHistory>, sqlx::Error> {
        Ok(vec![])
    }

    async fn submit_license_application(
        &self,
        _license_id: Uuid,
        _user_id: Uuid,
    ) -> Result<License, sqlx::Error> {
        // Placeholder implementation
        Err(sqlx::Error::RowNotFound)
    }

    async fn approve_license(
        &self,
        _license_id: Uuid,
        _admin_user_id: Uuid,
        _license_number: String,
        _issue_date: DateTime<Utc>,
        _expiry_date: Option<DateTime<Utc>>,
        _issuing_authority: String,
        _admin_notes: Option<String>,
    ) -> Result<License, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }

    async fn reject_license(
        &self,
        _license_id: Uuid,
        _admin_user_id: Uuid,
        _reason: String,
        _admin_notes: Option<String>,
    ) -> Result<License, sqlx::Error> {
        Err(sqlx::Error::RowNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Mutex;
    use std::collections::HashMap;
    use sqlx::postgres::PgPoolOptions;
    use crate::domain::licenses::{PriorityLevel};

    #[derive(Clone, Default)]
    struct InMemoryCache {
        store: Arc<Mutex<HashMap<String, String>>>,
    }

    #[async_trait]
    impl Cache for InMemoryCache {
        async fn set<T: serde::Serialize + Send + Sync>(
            &self,
            key: &str,
            value: &T,
            _expiry: Option<u64>,
        ) -> Result<(), redis::RedisError> {
            let mut map = self.store.lock().await;
            map.insert(key.to_string(), serde_json::to_string(value).unwrap());
            Ok(())
        }

        async fn get<T: serde::de::DeserializeOwned + Send + Sync>(
            &self,
            key: &str,
        ) -> Result<Option<T>, redis::RedisError> {
            let map = self.store.lock().await;
            Ok(match map.get(key) {
                Some(v) => Some(serde_json::from_str(v).unwrap()),
                None => None,
            })
        }

        async fn delete(&self, key: &str) -> Result<(), redis::RedisError> {
            let mut map = self.store.lock().await;
            map.remove(key);
            Ok(())
        }

        async fn delete_by_pattern(&self, pattern: &str) -> Result<(), redis::RedisError> {
            let mut map = self.store.lock().await;
            let prefix = pattern.trim_end_matches('*');
            let keys: Vec<String> = map
                .keys()
                .filter(|k| k.starts_with(prefix))
                .cloned()
                .collect();
            for k in keys { map.remove(&k); }
            Ok(())
        }
    }

    fn sample_license() -> License {
        License {
            id: Uuid::new_v4(),
            license_number: None,
            license_type: LicenseType::Nib,
            company_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            title: "Test".into(),
            description: None,
            issue_date: None,
            expiry_date: None,
            issuing_authority: None,
            application_status: ApplicationStatus::Draft,
            priority: PriorityLevel::Normal,
            estimated_processing_days: None,
            actual_processing_days: None,
            external_reference_id: None,
            government_fee: None,
            service_fee: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            submitted_at: None,
            approved_at: None,
            rejected_at: None,
            admin_notes: None,
            rejection_reason: None,
        }
    }

    #[tokio::test]
    async fn cache_hit_returns_value() {
        let cache = Arc::new(InMemoryCache::default());
        let pool = PgPoolOptions::new().connect_lazy("postgres://localhost").unwrap();
        let repo = CachedLicenseRepository::new_with_cache(pool, cache.clone());

        let license = sample_license();
        cache
            .set(&CachedLicenseRepository::<InMemoryCache>::license_cache_key(license.id), &license, None)
            .await
            .unwrap();

        let res = repo.get_license_by_id(license.id).await.unwrap().unwrap();
        assert_eq!(res.id, license.id);
    }

    #[tokio::test]
    async fn cache_miss_attempts_db() {
        let cache = Arc::new(InMemoryCache::default());
        let pool = PgPoolOptions::new().connect_lazy("postgres://localhost").unwrap();
        let repo = CachedLicenseRepository::new_with_cache(pool, cache.clone());

        let id = Uuid::new_v4();
        let result = repo.get_license_by_id(id).await;
        assert!(result.is_err());
    }
}
