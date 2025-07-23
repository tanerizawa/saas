// Domain Entities - Objects with identity and lifecycle
// Following DDD principles with clear aggregate boundaries

use crate::domain::value_objects::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

// User Aggregate Root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: String,
    pub full_name: String,
    pub phone: Option<PhoneNumber>,
    pub role: UserRole,
    pub status: UserStatus,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    UmkmOwner,      // UMKM business owner
    AdminStaff,     // Internal admin staff
    SuperAdmin,     // System administrator
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::UmkmOwner => write!(f, "umkm_owner"),
            UserRole::AdminStaff => write!(f, "admin_staff"),
            UserRole::SuperAdmin => write!(f, "super_admin"),
        }
    }
}

impl FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "umkm_owner" => Ok(UserRole::UmkmOwner),
            "admin_staff" => Ok(UserRole::AdminStaff),
            "super_admin" => Ok(UserRole::SuperAdmin),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

impl UserRole {
    /// Check if this role has permission to access resources of the required role
    pub fn has_permission(&self, required_role: &UserRole) -> bool {
        match (self, required_role) {
            // SuperAdmin has access to everything
            (UserRole::SuperAdmin, _) => true,
            // AdminStaff has access to UmkmOwner resources
            (UserRole::AdminStaff, UserRole::UmkmOwner) => true,
            (UserRole::AdminStaff, UserRole::AdminStaff) => true,
            // UmkmOwner only has access to their own resources
            (UserRole::UmkmOwner, UserRole::UmkmOwner) => true,
            // All other combinations are denied
            _ => false,
        }
    }

    /// Get list of roles this role can access
    pub fn accessible_roles(&self) -> Vec<UserRole> {
        match self {
            UserRole::SuperAdmin => vec![UserRole::UmkmOwner, UserRole::AdminStaff, UserRole::SuperAdmin],
            UserRole::AdminStaff => vec![UserRole::UmkmOwner, UserRole::AdminStaff],
            UserRole::UmkmOwner => vec![UserRole::UmkmOwner],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    PendingVerification,
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserStatus::Active => write!(f, "active"),
            UserStatus::Inactive => write!(f, "inactive"),
            UserStatus::Suspended => write!(f, "suspended"),
            UserStatus::PendingVerification => write!(f, "pending_verification"),
        }
    }
}

impl User {
    pub fn new(
        email: Email,
        password_hash: String,
        full_name: String,
        role: UserRole,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: UserId::new(),
            email,
            password_hash,
            full_name,
            phone: None,
            role,
            status: UserStatus::PendingVerification,
            email_verified_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn verify_email(&mut self) {
        self.email_verified_at = Some(Utc::now());
        self.status = UserStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, UserStatus::Active)
    }

    pub fn can_access_admin(&self) -> bool {
        matches!(self.role, UserRole::AdminStaff | UserRole::SuperAdmin)
    }

    pub fn can_login(&self) -> bool {
        matches!(self.status, UserStatus::Active)
    }

    pub fn update_last_login(&mut self) {
        // In this simplified version, we'll just update the updated_at timestamp
        // In a full implementation, you might have a separate last_login field
        self.updated_at = Utc::now();
    }

    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.updated_at = Utc::now();
    }
}

// License Aggregate Root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub id: LicenseId,
    pub user_id: UserId,
    pub business_id: BusinessId,
    pub license_type: LicenseType,
    pub status: LicenseStatus,
    pub application_data: LicenseApplicationData,
    pub documents: Vec<DocumentReference>,
    pub submitted_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseType {
    NIB,        // Nomor Induk Berusaha
    SIUP,       // Surat Izin Usaha Perdagangan
    TDP,        // Tanda Daftar Perusahaan
    NPWP,       // Nomor Pokok Wajib Pajak
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    Draft,
    Submitted,
    UnderReview,
    RequiresDocuments,
    Approved,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseApplicationData {
    pub business_name: BusinessName,
    pub business_address: String,
    pub business_type: String,
    pub ktp_number: Option<KTP>,
    pub npwp_number: Option<NPWP>,
    pub capital_amount: Option<Money>,
    pub additional_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReference {
    pub file_id: FileId,
    pub document_type: DocumentType,
    pub file_name: FileName,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    KTPPhoto,
    NPWPPhoto,
    BusinessPhoto,
    BusinessLetter,
    Other(String),
}

impl License {
    #[allow(dead_code)] // Will be used when licensing feature is implemented
    pub fn new(
        user_id: UserId,
        business_id: BusinessId,
        license_type: LicenseType,
        application_data: LicenseApplicationData,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: LicenseId::new(),
            user_id,
            business_id,
            license_type,
            status: LicenseStatus::Draft,
            application_data,
            documents: Vec::new(),
            submitted_at: now,
            processed_at: None,
            approved_at: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    #[allow(dead_code)] // Will be used when licensing workflow is implemented
    pub fn submit(&mut self) -> Result<(), String> {
        if self.status != LicenseStatus::Draft {
            return Err("License can only be submitted from draft status".to_string());
        }
        
        self.status = LicenseStatus::Submitted;
        self.submitted_at = Utc::now();
        self.updated_at = Utc::now();
        Ok(())
    }

    #[allow(dead_code)] // Will be used when licensing approval process is implemented
    pub fn approve(&mut self, notes: Option<String>) {
        self.status = LicenseStatus::Approved;
        self.approved_at = Some(Utc::now());
        self.processed_at = Some(Utc::now());
        self.notes = notes;
        self.updated_at = Utc::now();
    }

    #[allow(dead_code)] // Will be used when licensing rejection process is implemented
    pub fn reject(&mut self, notes: String) {
        self.status = LicenseStatus::Rejected;
        self.processed_at = Some(Utc::now());
        self.notes = Some(notes);
        self.updated_at = Utc::now();
    }
}

// Business Aggregate Root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Business {
    pub id: BusinessId,
    pub owner_id: UserId,
    pub name: BusinessName,
    pub description: Option<String>,
    pub business_type: BusinessType,
    pub address: BusinessAddress,
    pub contact_info: BusinessContactInfo,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BusinessType {
    MikroUsaha,     // Micro business
    KecilUsaha,     // Small business
    MenengahUsaha,  // Medium business
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAddress {
    pub street: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessContactInfo {
    pub phone: Option<PhoneNumber>,
    pub email: Option<Email>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Unverified,
    PendingVerification,
    Verified,
    Rejected,
}

impl Business {
    #[allow(dead_code)] // Will be used when business registration is implemented
    pub fn new(
        owner_id: UserId,
        name: BusinessName,
        business_type: BusinessType,
        address: BusinessAddress,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: BusinessId::new(),
            owner_id,
            name,
            description: None,
            business_type,
            address,
            contact_info: BusinessContactInfo {
                phone: None,
                email: None,
                website: None,
            },
            verification_status: VerificationStatus::Unverified,
            created_at: now,
            updated_at: now,
        }
    }

    #[allow(dead_code)] // Will be used when business verification is implemented
    pub fn request_verification(&mut self) {
        self.verification_status = VerificationStatus::PendingVerification;
        self.updated_at = Utc::now();
    }

    #[allow(dead_code)] // Will be used when business verification process is implemented
    pub fn verify(&mut self) {
        self.verification_status = VerificationStatus::Verified;
        self.updated_at = Utc::now();
    }
}
