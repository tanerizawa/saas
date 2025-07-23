//! Business metrics collector for the SaaS UMKM platform
//! This module implements custom metrics for business-related activities

use lazy_static::lazy_static;
use prometheus::{register_counter, register_gauge, register_histogram, Counter, Gauge, Histogram};

lazy_static! {
    // License application metrics
    pub static ref LICENSE_APPLICATIONS_TOTAL: Counter = register_counter!(
        "saas_umkm_license_applications_total",
        "Total number of license applications submitted",
        &["license_type", "region"]
    )
    .unwrap();

    pub static ref LICENSE_PROCESSING_TIME: Histogram = register_histogram!(
        "saas_umkm_license_processing_time_seconds",
        "Time taken to process license applications",
        &["license_type", "status"],
        vec![0.1, 0.5, 1.0, 2.0, 5.0, 10.0, 30.0, 60.0, 300.0, 600.0, 1800.0, 3600.0]
    )
    .unwrap();

    pub static ref LICENSES_PENDING: Gauge = register_gauge!(
        "saas_umkm_licenses_pending",
        "Number of license applications currently pending",
        &["license_type"]
    )
    .unwrap();

    // User activity metrics
    pub static ref USER_REGISTRATIONS_TOTAL: Counter = register_counter!(
        "saas_umkm_user_registrations_total",
        "Total number of user registrations",
        &["user_type", "region"]
    )
    .unwrap();

    pub static ref ACTIVE_USERS: Gauge = register_gauge!(
        "saas_umkm_active_users",
        "Number of active users in the last 30 days",
        &["user_type"]
    )
    .unwrap();

    // Document processing metrics
    pub static ref DOCUMENT_UPLOADS_TOTAL: Counter = register_counter!(
        "saas_umkm_document_uploads_total",
        "Total number of documents uploaded",
        &["document_type"]
    )
    .unwrap();

    pub static ref DOCUMENT_VERIFICATION_TIME: Histogram = register_histogram!(
        "saas_umkm_document_verification_time_seconds",
        "Time taken to verify documents",
        &["document_type", "result"],
        vec![0.5, 1.0, 2.0, 5.0, 10.0, 30.0, 60.0, 300.0]
    )
    .unwrap();

    // Business performance metrics
    pub static ref COMPANIES_REGISTERED: Counter = register_counter!(
        "saas_umkm_companies_registered_total",
        "Total number of companies registered",
        &["company_size", "industry", "region"]
    )
    .unwrap();

    pub static ref COMPLIANCE_SCORE: Gauge = register_gauge!(
        "saas_umkm_compliance_score",
        "Average compliance score of registered companies",
        &["company_size", "industry"]
    )
    .unwrap();
}

/// Initialize all metrics with default values
pub fn init_metrics() {
    // Set initial values for gauges
    LICENSES_PENDING.with_label_values(&["NIB"]).set(0.0);
    LICENSES_PENDING.with_label_values(&["SIUP"]).set(0.0);
    LICENSES_PENDING.with_label_values(&["TDP"]).set(0.0);

    ACTIVE_USERS.with_label_values(&["individual"]).set(0.0);
    ACTIVE_USERS.with_label_values(&["business"]).set(0.0);

    COMPLIANCE_SCORE
        .with_label_values(&["small", "retail"])
        .set(0.0);
    COMPLIANCE_SCORE
        .with_label_values(&["small", "service"])
        .set(0.0);
    COMPLIANCE_SCORE
        .with_label_values(&["medium", "retail"])
        .set(0.0);
    COMPLIANCE_SCORE
        .with_label_values(&["medium", "service"])
        .set(0.0);
}

/// Record a license application submission
pub fn record_license_application(license_type: &str, region: &str) {
    LICENSE_APPLICATIONS_TOTAL
        .with_label_values(&[license_type, region])
        .inc();
    LICENSES_PENDING.with_label_values(&[license_type]).inc();
}

/// Record license processing completion
pub fn record_license_processed(license_type: &str, status: &str, duration_secs: f64) {
    LICENSE_PROCESSING_TIME
        .with_label_values(&[license_type, status])
        .observe(duration_secs);
    LICENSES_PENDING.with_label_values(&[license_type]).dec();
}

/// Record a new user registration
pub fn record_user_registration(user_type: &str, region: &str) {
    USER_REGISTRATIONS_TOTAL
        .with_label_values(&[user_type, region])
        .inc();
    ACTIVE_USERS.with_label_values(&[user_type]).inc();
}

/// Record document upload and processing
pub fn record_document_upload(document_type: &str) {
    DOCUMENT_UPLOADS_TOTAL
        .with_label_values(&[document_type])
        .inc();
}

/// Record document verification completion
pub fn record_document_verified(document_type: &str, result: &str, duration_secs: f64) {
    DOCUMENT_VERIFICATION_TIME
        .with_label_values(&[document_type, result])
        .observe(duration_secs);
}

/// Record a new company registration
pub fn record_company_registered(company_size: &str, industry: &str, region: &str) {
    COMPANIES_REGISTERED
        .with_label_values(&[company_size, industry, region])
        .inc();
}

/// Update compliance score for a company category
pub fn update_compliance_score(company_size: &str, industry: &str, score: f64) {
    COMPLIANCE_SCORE
        .with_label_values(&[company_size, industry])
        .set(score);
}
