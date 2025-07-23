// Company domain entity for business profile management
// Implements company/business information with Indonesian UMKM compliance

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessType {
    CV,         // Commanditaire Vennootschap
    PT,         // Perseroan Terbatas
    UD,         // Usaha Dagang
    Koperasi,   // Cooperative
    Perorangan, // Individual/Sole Proprietorship
}

impl std::fmt::Display for BusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusinessType::CV => write!(f, "CV"),
            BusinessType::PT => write!(f, "PT"),
            BusinessType::UD => write!(f, "UD"),
            BusinessType::Koperasi => write!(f, "Koperasi"),
            BusinessType::Perorangan => write!(f, "Perorangan"),
        }
    }
}

impl std::str::FromStr for BusinessType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "CV" => Ok(BusinessType::CV),
            "PT" => Ok(BusinessType::PT),
            "UD" => Ok(BusinessType::UD),
            "KOPERASI" => Ok(BusinessType::Koperasi),
            "PERORANGAN" => Ok(BusinessType::Perorangan),
            _ => Err(format!("Invalid business type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessScale {
    Mikro,    // Micro: Asset <= 50 juta OR Revenue <= 300 juta
    Kecil,    // Small: 50 juta < Asset <= 500 juta OR 300 juta < Revenue <= 2.5 miliar
    Menengah, // Medium: 500 juta < Asset <= 10 miliar OR 2.5 miliar < Revenue <= 50 miliar
}

impl std::fmt::Display for BusinessScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusinessScale::Mikro => write!(f, "mikro"),
            BusinessScale::Kecil => write!(f, "kecil"),
            BusinessScale::Menengah => write!(f, "menengah"),
        }
    }
}

impl std::str::FromStr for BusinessScale {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mikro" => Ok(BusinessScale::Mikro),
            "kecil" => Ok(BusinessScale::Kecil),
            "menengah" => Ok(BusinessScale::Menengah),
            _ => Err(format!("Invalid business scale: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyStatus {
    Active,
    Inactive,
    Suspended,
    PendingVerification,
}

impl std::fmt::Display for CompanyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanyStatus::Active => write!(f, "active"),
            CompanyStatus::Inactive => write!(f, "inactive"),
            CompanyStatus::Suspended => write!(f, "suspended"),
            CompanyStatus::PendingVerification => write!(f, "pending_verification"),
        }
    }
}

impl std::str::FromStr for CompanyStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(CompanyStatus::Active),
            "inactive" => Ok(CompanyStatus::Inactive),
            "suspended" => Ok(CompanyStatus::Suspended),
            "pending_verification" => Ok(CompanyStatus::PendingVerification),
            _ => Err(format!("Invalid company status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAddress {
    pub street: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,
    pub country: String,
}

impl CompanyAddress {
    pub fn new(street: String, city: String, province: String, postal_code: String) -> Self {
        Self {
            street,
            city,
            province,
            postal_code,
            country: "Indonesia".to_string(),
        }
    }

    pub fn full_address(&self) -> String {
        format!(
            "{}, {}, {} {}, {}",
            self.street, self.city, self.province, self.postal_code, self.country
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub bank_name: String,
    pub account_number: String,
    pub account_holder: String,
}

impl BankAccount {
    pub fn new(bank_name: String, account_number: String, account_holder: String) -> Self {
        Self {
            bank_name,
            account_number,
            account_holder,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub owner_id: Uuid,

    // Basic Information
    pub company_name: String,
    pub business_type: String, // Will be converted to/from BusinessType
    pub industry_sector: String,
    pub description: Option<String>,
    pub establishment_date: Option<NaiveDate>,
    pub employee_count: i32,

    // License Information
    pub nib: Option<String>,
    pub siup_number: Option<String>,
    pub tdp_number: Option<String>,
    pub npwp_company: Option<String>,

    // Contact Information
    pub email: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,

    // Address Information
    pub address_street: String,
    pub address_city: String,
    pub address_province: String,
    pub address_postal_code: String,
    pub address_country: String,

    // Business Information
    pub business_scale: String, // Will be converted to/from BusinessScale
    pub annual_revenue: Option<i64>,
    pub annual_revenue_year: Option<i32>,

    // Verification Status
    pub is_verified: bool,
    pub verification_date: Option<DateTime<Utc>>,
    pub verification_notes: Option<String>,

    // Bank Information
    pub bank_name: Option<String>,
    pub bank_account_number: Option<String>,
    pub bank_account_holder: Option<String>,

    // Documents & Files
    pub logo_url: Option<String>,
    pub documents: serde_json::Value, // JSON storage for documents

    // Status & Metadata
    pub status: String, // Will be converted to/from CompanyStatus
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Company {
    pub fn new(
        owner_id: Uuid,
        company_name: String,
        business_type: BusinessType,
        industry_sector: String,
        address: CompanyAddress,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            owner_id,
            company_name,
            business_type: business_type.to_string(),
            industry_sector,
            description: None,
            establishment_date: None,
            employee_count: 0,
            nib: None,
            siup_number: None,
            tdp_number: None,
            npwp_company: None,
            email: None,
            phone: None,
            website: None,
            address_street: address.street,
            address_city: address.city,
            address_province: address.province,
            address_postal_code: address.postal_code,
            address_country: address.country,
            business_scale: BusinessScale::Mikro.to_string(),
            annual_revenue: None,
            annual_revenue_year: None,
            is_verified: false,
            verification_date: None,
            verification_notes: None,
            bank_name: None,
            bank_account_number: None,
            bank_account_holder: None,
            logo_url: None,
            documents: serde_json::json!({}),
            status: CompanyStatus::PendingVerification.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    // Type-safe getters
    pub fn get_business_type(&self) -> Result<BusinessType, String> {
        self.business_type.parse()
    }

    pub fn get_business_scale(&self) -> Result<BusinessScale, String> {
        self.business_scale.parse()
    }

    pub fn get_status(&self) -> Result<CompanyStatus, String> {
        self.status.parse()
    }

    pub fn get_address(&self) -> CompanyAddress {
        CompanyAddress {
            street: self.address_street.clone(),
            city: self.address_city.clone(),
            province: self.address_province.clone(),
            postal_code: self.address_postal_code.clone(),
            country: self.address_country.clone(),
        }
    }

    pub fn get_bank_account(&self) -> Option<BankAccount> {
        if let (Some(bank_name), Some(account_number), Some(account_holder)) = (
            &self.bank_name,
            &self.bank_account_number,
            &self.bank_account_holder,
        ) {
            Some(BankAccount::new(
                bank_name.clone(),
                account_number.clone(),
                account_holder.clone(),
            ))
        } else {
            None
        }
    }

    // Business logic methods
    pub fn can_be_verified(&self) -> bool {
        // Company must have basic required information
        !self.company_name.trim().is_empty()
            && !self.address_street.trim().is_empty()
            && !self.address_city.trim().is_empty()
            && !self.address_province.trim().is_empty()
            && self.get_status().unwrap_or(CompanyStatus::Inactive)
                == CompanyStatus::PendingVerification
    }

    pub fn verify(&mut self, verification_notes: Option<String>) {
        self.is_verified = true;
        self.verification_date = Some(Utc::now());
        self.verification_notes = verification_notes;
        self.status = CompanyStatus::Active.to_string();
        self.updated_at = Utc::now();
    }

    pub fn set_business_scale_from_revenue(&mut self, annual_revenue: i64) {
        self.annual_revenue = Some(annual_revenue);
        self.annual_revenue_year = Some(Utc::now().year());

        // Determine business scale based on revenue (UU No. 20 Tahun 2008)
        let scale = if annual_revenue <= 300_000_000 {
            BusinessScale::Mikro
        } else if annual_revenue <= 2_500_000_000 {
            BusinessScale::Kecil
        } else if annual_revenue <= 50_000_000_000 {
            BusinessScale::Menengah
        } else {
            BusinessScale::Menengah // Cap at Menengah for UMKM platform
        };

        self.business_scale = scale.to_string();
        self.updated_at = Utc::now();
    }

    pub fn update_contact_info(
        &mut self,
        email: Option<String>,
        phone: Option<String>,
        website: Option<String>,
    ) {
        self.email = email;
        self.phone = phone;
        self.website = website;
        self.updated_at = Utc::now();
    }

    pub fn set_bank_account(&mut self, bank_account: BankAccount) {
        self.bank_name = Some(bank_account.bank_name);
        self.bank_account_number = Some(bank_account.account_number);
        self.bank_account_holder = Some(bank_account.account_holder);
        self.updated_at = Utc::now();
    }

    pub fn set_nib(&mut self, nib: String) -> Result<(), String> {
        // Validate NIB format (13 digits)
        if !nib.chars().all(|c| c.is_ascii_digit()) || nib.len() != 13 {
            return Err("NIB must be exactly 13 digits".to_string());
        }

        self.nib = Some(nib);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_complete_profile(&self) -> bool {
        // Check if company has minimum required information
        !self.company_name.trim().is_empty()
            && !self.address_street.trim().is_empty()
            && !self.address_city.trim().is_empty()
            && !self.address_province.trim().is_empty()
            && !self.address_postal_code.trim().is_empty()
            && self.email.is_some()
            && self.phone.is_some()
    }

    pub fn calculate_completeness_percentage(&self) -> u8 {
        let mut score = 0u8;
        let total_fields = 15u8;

        // Required fields (5 points each)
        if !self.company_name.trim().is_empty() {
            score += 1;
        }
        if !self.address_street.trim().is_empty() {
            score += 1;
        }
        if !self.address_city.trim().is_empty() {
            score += 1;
        }
        if !self.address_province.trim().is_empty() {
            score += 1;
        }
        if !self.address_postal_code.trim().is_empty() {
            score += 1;
        }

        // Important fields (1 point each)
        if self.email.is_some() {
            score += 1;
        }
        if self.phone.is_some() {
            score += 1;
        }
        if self.description.is_some() {
            score += 1;
        }
        if self.establishment_date.is_some() {
            score += 1;
        }
        if self.nib.is_some() {
            score += 1;
        }
        if self.npwp_company.is_some() {
            score += 1;
        }
        if self.bank_name.is_some() && self.bank_account_number.is_some() {
            score += 1;
        }
        if self.annual_revenue.is_some() {
            score += 1;
        }
        if self.logo_url.is_some() {
            score += 1;
        }
        if self.website.is_some() {
            score += 1;
        }

        (score * 100) / total_fields
    }
}
