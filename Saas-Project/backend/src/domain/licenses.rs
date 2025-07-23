use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// License types supported by the Indonesian UMKM platform
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "license_type", rename_all = "lowercase")]
pub enum LicenseType {
    /// Nomor Induk Berusaha - Primary business registration number
    Nib,
    /// Surat Izin Usaha Perdagangan - Trading business license
    Siup,
    /// Tanda Daftar Perusahaan - Company registration certificate
    Tdp,
    /// Nomor Pokok Wajib Pajak - Tax identification number
    Npwp,
    /// Halal certification for food/beverage businesses
    Halal,
    /// Environmental permit for certain industries
    Environmental,
    /// Export-import license for trading businesses
    ExportImport,
}

impl std::fmt::Display for LicenseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LicenseType::Nib => write!(f, "nib"),
            LicenseType::Siup => write!(f, "siup"),
            LicenseType::Tdp => write!(f, "tdp"),
            LicenseType::Npwp => write!(f, "npwp"),
            LicenseType::Halal => write!(f, "halal"),
            LicenseType::Environmental => write!(f, "environmental"),
            LicenseType::ExportImport => write!(f, "exportimport"),
        }
    }
}

/// Application status workflow for license processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "application_status", rename_all = "lowercase")]
pub enum ApplicationStatus {
    /// Initial state - application created but not submitted
    Draft,
    /// Application submitted for review
    Submitted,
    /// Under review by admin/government
    Processing,
    /// Additional documents or information required
    PendingDocuments,
    /// Application approved and license issued
    Approved,
    /// Application rejected with reason
    Rejected,
    /// License expired and needs renewal
    Expired,
    /// License suspended due to violations
    Suspended,
}

impl std::fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationStatus::Draft => write!(f, "draft"),
            ApplicationStatus::Submitted => write!(f, "submitted"),
            ApplicationStatus::Processing => write!(f, "processing"),
            ApplicationStatus::PendingDocuments => write!(f, "pending_documents"),
            ApplicationStatus::Approved => write!(f, "approved"),
            ApplicationStatus::Rejected => write!(f, "rejected"),
            ApplicationStatus::Expired => write!(f, "expired"),
            ApplicationStatus::Suspended => write!(f, "suspended"),
        }
    }
}

/// Priority level for license applications
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "priority_level", rename_all = "lowercase")]
pub enum PriorityLevel {
    Low,
    Normal,
    High,
    Urgent,
}

/// Main License entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct License {
    pub id: Uuid,
    pub license_number: Option<String>, // Generated after approval
    pub license_type: LicenseType,
    pub company_id: Uuid, // Foreign key to companies table
    pub user_id: Uuid,    // Foreign key to users table

    // License details
    pub title: String,
    pub description: Option<String>,
    pub issue_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub issuing_authority: Option<String>,

    // Application details
    pub application_status: ApplicationStatus,
    pub priority: PriorityLevel,
    pub estimated_processing_days: Option<i32>,
    pub actual_processing_days: Option<i32>,

    // Government integration
    pub external_reference_id: Option<String>, // OSS integration reference
    pub government_fee: Option<i64>,           // Fee in rupiah
    pub service_fee: Option<i64>,              // Our platform fee in rupiah

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,

    // Admin notes
    pub admin_notes: Option<String>,
    pub rejection_reason: Option<String>,
}

/// License application form data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseApplication {
    pub license_type: LicenseType,
    pub company_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<PriorityLevel>,
    pub estimated_processing_days: Option<i32>,
    pub additional_data: Option<serde_json::Value>, // Flexible data for different license types
}

/// Document types for license applications
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "document_type", rename_all = "lowercase")]
pub enum DocumentType {
    /// Identity documents
    Ktp,
    /// Company deed (Akta Pendirian)
    CompanyDeed,
    /// Tax registration
    TaxCertificate,
    /// Bank statement
    BankStatement,
    /// Business plan
    BusinessPlan,
    /// Location permit
    LocationPermit,
    /// Other supporting documents
    Other,
}

/// Document entity for license applications
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LicenseDocument {
    pub id: Uuid,
    pub license_id: Uuid,
    pub document_type: DocumentType,
    pub file_name: String,
    pub original_file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub upload_date: DateTime<Utc>,
    pub is_verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<Uuid>, // Admin user ID
    pub notes: Option<String>,
}

/// Status history tracking for audit trail
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApplicationStatusHistory {
    pub id: Uuid,
    pub license_id: Uuid,
    pub from_status: Option<ApplicationStatus>,
    pub to_status: ApplicationStatus,
    pub changed_by: Uuid, // User ID who made the change
    pub changed_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub is_system_generated: bool,
}

impl License {
    /// Create a new license application in draft status
    pub fn new(
        license_type: LicenseType,
        company_id: Uuid,
        user_id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            license_number: None,
            license_type,
            company_id,
            user_id,
            title,
            description,
            issue_date: None,
            expiry_date: None,
            issuing_authority: None,
            application_status: ApplicationStatus::Draft,
            priority: PriorityLevel::Normal,
            estimated_processing_days: Self::default_processing_days(&license_type),
            actual_processing_days: None,
            external_reference_id: None,
            government_fee: None,
            service_fee: None,
            created_at: now,
            updated_at: now,
            submitted_at: None,
            approved_at: None,
            rejected_at: None,
            admin_notes: None,
            rejection_reason: None,
        }
    }

    /// Get default processing days for different license types
    pub fn default_processing_days(license_type: &LicenseType) -> Option<i32> {
        match license_type {
            LicenseType::Nib => Some(7),            // NIB typically 7 days
            LicenseType::Siup => Some(14),          // SIUP typically 14 days
            LicenseType::Tdp => Some(10),           // TDP typically 10 days
            LicenseType::Npwp => Some(3),           // NPWP typically 3 days
            LicenseType::Halal => Some(30),         // Halal certification longer process
            LicenseType::Environmental => Some(45), // Environmental permits complex
            LicenseType::ExportImport => Some(21),  // Export-import licenses
        }
    }

    /// Submit the application (change status from Draft to Submitted)
    pub fn submit(&mut self) -> Result<(), String> {
        if self.application_status != ApplicationStatus::Draft {
            return Err("Can only submit applications in Draft status".to_string());
        }

        let now = Utc::now();
        self.application_status = ApplicationStatus::Submitted;
        self.submitted_at = Some(now);
        self.updated_at = now;

        Ok(())
    }

    /// Approve the license application
    pub fn approve(
        &mut self,
        license_number: String,
        issue_date: DateTime<Utc>,
        expiry_date: Option<DateTime<Utc>>,
        issuing_authority: String,
        admin_notes: Option<String>,
    ) -> Result<(), String> {
        if !matches!(
            self.application_status,
            ApplicationStatus::Processing | ApplicationStatus::PendingDocuments
        ) {
            return Err(
                "Can only approve applications in Processing or PendingDocuments status"
                    .to_string(),
            );
        }

        let now = Utc::now();
        self.application_status = ApplicationStatus::Approved;
        self.license_number = Some(license_number);
        self.issue_date = Some(issue_date);
        self.expiry_date = expiry_date;
        self.issuing_authority = Some(issuing_authority);
        self.approved_at = Some(now);
        self.updated_at = now;
        self.admin_notes = admin_notes;

        // Calculate actual processing days
        if let Some(submitted_at) = self.submitted_at {
            let processing_duration = now.signed_duration_since(submitted_at);
            self.actual_processing_days = Some(processing_duration.num_days() as i32);
        }

        Ok(())
    }

    /// Reject the license application
    pub fn reject(
        &mut self,
        rejection_reason: String,
        admin_notes: Option<String>,
    ) -> Result<(), String> {
        if !matches!(
            self.application_status,
            ApplicationStatus::Processing | ApplicationStatus::PendingDocuments
        ) {
            return Err(
                "Can only reject applications in Processing or PendingDocuments status".to_string(),
            );
        }

        let now = Utc::now();
        self.application_status = ApplicationStatus::Rejected;
        self.rejection_reason = Some(rejection_reason);
        self.rejected_at = Some(now);
        self.updated_at = now;
        self.admin_notes = admin_notes;

        Ok(())
    }

    /// Check if license is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiry_date) = self.expiry_date {
            expiry_date < Utc::now()
        } else {
            false
        }
    }

    /// Check if license is active (approved and not expired)
    pub fn is_active(&self) -> bool {
        self.application_status == ApplicationStatus::Approved && !self.is_expired()
    }

    /// Get days until expiry (negative if expired)
    pub fn days_until_expiry(&self) -> Option<i64> {
        self.expiry_date
            .map(|expiry| expiry.signed_duration_since(Utc::now()).num_days())
    }

    /// Check if license needs renewal (within 30 days of expiry)
    pub fn needs_renewal(&self) -> bool {
        if let Some(days_until_expiry) = self.days_until_expiry() {
            days_until_expiry <= 30 && days_until_expiry > 0
        } else {
            false
        }
    }
}

impl LicenseDocument {
    /// Create a new license document
    pub fn new(
        license_id: Uuid,
        document_type: DocumentType,
        file_name: String,
        original_file_name: String,
        file_path: String,
        file_size: i64,
        mime_type: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            license_id,
            document_type,
            file_name,
            original_file_name,
            file_path,
            file_size,
            mime_type,
            upload_date: Utc::now(),
            is_verified: false,
            verified_at: None,
            verified_by: None,
            notes: None,
        }
    }

    /// Verify the document
    pub fn verify(&mut self, verified_by: Uuid, notes: Option<String>) {
        self.is_verified = true;
        self.verified_at = Some(Utc::now());
        self.verified_by = Some(verified_by);
        self.notes = notes;
    }
}

impl ApplicationStatusHistory {
    /// Create a new status history entry
    pub fn new(
        license_id: Uuid,
        from_status: Option<ApplicationStatus>,
        to_status: ApplicationStatus,
        changed_by: Uuid,
        notes: Option<String>,
        is_system_generated: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            license_id,
            from_status,
            to_status,
            changed_by,
            changed_at: Utc::now(),
            notes,
            is_system_generated,
        }
    }
}
