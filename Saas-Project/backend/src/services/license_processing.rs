use crate::domain::licenses::{ApplicationStatus, License};
use crate::shared::errors::{AppError, AppResult};

/// Service responsible for handling license workflows
#[derive(Debug, Default)]
pub struct LicenseProcessingService;

impl LicenseProcessingService {
    /// Create a new instance of the service
    pub fn new() -> Self {
        Self
    }

    /// Submit a license application. This is a stub implementation that simply
    /// transitions the license into the `Submitted` state if possible.
    pub fn submit(&self, mut license: License) -> AppResult<License> {
        license.submit().map_err(AppError::Validation)?;
        Ok(license)
    }

    /// Return the current status of a license
    pub fn status(&self, license: &License) -> ApplicationStatus {
        license.application_status.clone()
    }
}
