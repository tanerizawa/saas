use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::licenses::{ApplicationStatus, License, LicenseType, PriorityLevel};
use crate::infrastructure::repositories::{LicenseRepository, PostgresLicenseRepositoryImpl};

async fn setup_test_db() -> (PgPool, String) {
    let schema_name = format!("license_test_{}", Uuid::new_v4().as_simple());
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://saas_user:saas_password@localhost:5432/saas_test_db".to_string());
    let pool = PgPool::connect(&database_url).await.unwrap();
    sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(&format!("SET search_path TO {}", schema_name))
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE licenses (
            id UUID PRIMARY KEY,
            license_number TEXT,
            license_type TEXT NOT NULL,
            company_id UUID NOT NULL,
            user_id UUID NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            issue_date TIMESTAMPTZ,
            expiry_date TIMESTAMPTZ,
            issuing_authority TEXT,
            application_status TEXT NOT NULL,
            priority TEXT NOT NULL,
            estimated_processing_days INTEGER,
            actual_processing_days INTEGER,
            external_reference_id TEXT,
            government_fee BIGINT,
            service_fee BIGINT,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL,
            submitted_at TIMESTAMPTZ,
            approved_at TIMESTAMPTZ,
            rejected_at TIMESTAMPTZ,
            admin_notes TEXT,
            rejection_reason TEXT
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE license_documents (
            id UUID PRIMARY KEY,
            license_id UUID NOT NULL,
            document_type TEXT NOT NULL,
            file_name TEXT NOT NULL,
            original_file_name TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_size BIGINT NOT NULL,
            mime_type TEXT NOT NULL,
            upload_date TIMESTAMPTZ NOT NULL,
            is_verified BOOLEAN NOT NULL,
            verified_at TIMESTAMPTZ,
            verified_by UUID,
            notes TEXT
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE application_status_history (
            id UUID PRIMARY KEY,
            license_id UUID NOT NULL,
            from_status TEXT,
            to_status TEXT NOT NULL,
            changed_by UUID NOT NULL,
            changed_at TIMESTAMPTZ NOT NULL,
            notes TEXT,
            is_system_generated BOOLEAN NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();

    (pool, schema_name)
}

async fn teardown(pool: &PgPool, schema: &str) {
    sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema))
        .execute(pool)
        .await
        .unwrap();
    pool.close().await;
}

#[tokio::test]
async fn test_license_crud_and_status() -> Result<(), sqlx::Error> {
    let (pool, schema) = setup_test_db().await;
    let repo = PostgresLicenseRepositoryImpl::new(pool.clone());

    let mut license = License::new(
        LicenseType::Nib,
        Uuid::new_v4(),
        Uuid::new_v4(),
        "Test License".to_string(),
        None,
    );
    license.priority = PriorityLevel::Normal;

    let created = repo.create_license(&license).await?;
    assert_eq!(created.id, license.id);

    let fetched = repo.get_license_by_id(license.id).await?.unwrap();
    assert_eq!(fetched.title, "Test License");

    license.title = "Updated".into();
    license.updated_at = Utc::now();
    let updated = repo.update_license(&license).await?;
    assert_eq!(updated.title, "Updated");

    let submitted = repo
        .submit_license_application(license.id, license.user_id)
        .await?;
    assert_eq!(submitted.application_status, ApplicationStatus::Submitted);

    let approved = repo
        .approve_license(
            license.id,
            Uuid::new_v4(),
            "LIC-1".to_string(),
            Utc::now(),
            None,
            "Gov".to_string(),
            Some("ok".to_string()),
        )
        .await?;
    assert_eq!(approved.application_status, ApplicationStatus::Approved);

    let by_status = repo
        .get_licenses_by_status(ApplicationStatus::Approved)
        .await?;
    assert_eq!(by_status.len(), 1);

    let by_type = repo.get_licenses_by_type(LicenseType::Nib).await?;
    assert_eq!(by_type.len(), 1);

    assert!(repo.delete_license(license.id).await?);
    assert!(repo.get_license_by_id(license.id).await?.is_none());

    teardown(&pool, &schema).await;
    Ok(())
}
