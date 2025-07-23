// Value Objects - Immutable objects that represent domain concepts
// Using newtype pattern for type safety as recommended in document

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{AddAssign, Neg};
use uuid::Uuid;

// User-related value objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(pub String);

impl Email {
    pub fn new(email: &str) -> Result<Self, String> {
        if email.contains('@') && email.len() > 3 {
            Ok(Self(email.to_string()))
        } else {
            Err("Invalid email format".to_string())
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    pub fn new(phone: &str) -> Result<Self, String> {
        // Indonesian phone number validation
        if phone.starts_with("08") || phone.starts_with("+62") {
            Ok(Self(phone.to_string()))
        } else {
            Err("Invalid Indonesian phone number format".to_string())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Licensing-related value objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LicenseId(pub Uuid);

impl LicenseId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NIB(pub String);

impl NIB {
    pub fn new(nib: String) -> Result<Self, String> {
        // NIB validation - 13 digits
        if nib.len() == 13 && nib.chars().all(char::is_numeric) {
            Ok(Self(nib))
        } else {
            Err("NIB must be 13 digits".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NPWP(pub String);

impl NPWP {
    pub fn new(npwp: String) -> Result<Self, String> {
        // NPWP validation - 15 digits with dots and dashes
        if npwp.len() == 20 {
            Ok(Self(npwp))
        } else {
            Err("NPWP format invalid".to_string())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KTP(pub String);

impl KTP {
    pub fn new(ktp: String) -> Result<Self, String> {
        // KTP validation - 16 digits
        if ktp.len() == 16 && ktp.chars().all(char::is_numeric) {
            Ok(Self(ktp))
        } else {
            Err("KTP must be 16 digits".to_string())
        }
    }
}

// Business-related value objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusinessId(pub Uuid);

impl BusinessId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessName(pub String);

impl BusinessName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            Err("Business name cannot be empty".to_string())
        } else if name.len() > 255 {
            Err("Business name too long".to_string())
        } else {
            Ok(Self(name.trim().to_string()))
        }
    }
}

// Financial-related value objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64, // Amount in smallest currency unit (cents/rupiah)
    pub currency: Currency,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Currency {
    IDR, // Indonesian Rupiah
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Currency::IDR => write!(f, "IDR"),
        }
    }
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn idr(amount: i64) -> Self {
        Self::new(amount, Currency::IDR)
    }

    pub fn from_f64(amount_f64: f64, currency: Currency) -> Self {
        // Convert from f64 to i64 cents/smallest currency unit
        let amount = (amount_f64 * 100.0).round() as i64;
        Self::new(amount, currency)
    }

    pub fn to_f64(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

// Implement negation for Money
impl Neg for Money {
    type Output = Money;

    fn neg(self) -> Self::Output {
        Money {
            amount: -self.amount,
            currency: self.currency,
        }
    }
}

// Implement AddAssign for Money
impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        // In a real application, you'd want to check currencies match
        // and handle currency conversion if needed
        assert_eq!(
            self.currency, other.currency,
            "Cannot add money with different currencies"
        );
        self.amount += other.amount;
    }
}

// File-related value objects
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub Uuid);

impl FileId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileName(pub String);

impl FileName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.trim().is_empty() {
            Err("File name cannot be empty".to_string())
        } else {
            Ok(Self(name.trim().to_string()))
        }
    }
}
