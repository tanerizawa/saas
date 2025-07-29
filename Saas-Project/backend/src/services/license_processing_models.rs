use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::licenses::{ApplicationStatus, LicenseType, PriorityLevel};

/// Request payload for submitting a license application
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct LicenseApplicationRequest {
    pub company_id: Uuid,
    pub license_type: LicenseType,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<PriorityLevel>,
}

/// Simple response returned after creating a license application
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct LicenseApplicationResponse {
    pub id: Uuid,
    pub status: ApplicationStatus,
}
