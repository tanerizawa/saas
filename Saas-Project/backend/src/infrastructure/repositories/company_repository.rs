// Company repository implementation using PostgreSQL
// Implements CompanyRepository trait with SQLx for database operations

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::companies::Company;
use crate::domain::repositories::CompanyRepository;
use crate::shared::errors::{AppError, AppResult};

pub struct PostgresCompanyRepository {
    pool: PgPool,
}

impl PostgresCompanyRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CompanyRepository for PostgresCompanyRepository {
    async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Company>> {
        let query = r#"
            SELECT 
                id, owner_id, company_name, business_type, industry_sector,
                description, establishment_date, employee_count,
                nib, siup_number, tdp_number, npwp_company,
                email, phone, website,
                address_street, address_city, address_province, 
                address_postal_code, address_country,
                business_scale, annual_revenue, annual_revenue_year,
                is_verified, verification_date, verification_notes,
                bank_name, bank_account_number, bank_account_holder,
                logo_url, documents, status, created_at, updated_at
            FROM companies 
            WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        match row {
            Some(row) => {
                let company = Company {
                    id: row.get("id"),
                    owner_id: row.get("owner_id"),
                    company_name: row.get("company_name"),
                    business_type: row.get("business_type"),
                    industry_sector: row.get("industry_sector"),
                    description: row.get("description"),
                    establishment_date: row.get("establishment_date"),
                    employee_count: row.get("employee_count"),
                    nib: row.get("nib"),
                    siup_number: row.get("siup_number"),
                    tdp_number: row.get("tdp_number"),
                    npwp_company: row.get("npwp_company"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    website: row.get("website"),
                    address_street: row.get("address_street"),
                    address_city: row.get("address_city"),
                    address_province: row.get("address_province"),
                    address_postal_code: row.get("address_postal_code"),
                    address_country: row.get("address_country"),
                    business_scale: row.get("business_scale"),
                    annual_revenue: row.get("annual_revenue"),
                    annual_revenue_year: row.get("annual_revenue_year"),
                    is_verified: row.get("is_verified"),
                    verification_date: row.get("verification_date"),
                    verification_notes: row.get("verification_notes"),
                    bank_name: row.get("bank_name"),
                    bank_account_number: row.get("bank_account_number"),
                    bank_account_holder: row.get("bank_account_holder"),
                    logo_url: row.get("logo_url"),
                    documents: row.get("documents"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(company))
            }
            None => Ok(None),
        }
    }

    async fn find_by_owner_id(&self, owner_id: &Uuid) -> AppResult<Vec<Company>> {
        let query = r#"
            SELECT 
                id, owner_id, company_name, business_type, industry_sector,
                description, establishment_date, employee_count,
                nib, siup_number, tdp_number, npwp_company,
                email, phone, website,
                address_street, address_city, address_province, 
                address_postal_code, address_country,
                business_scale, annual_revenue, annual_revenue_year,
                is_verified, verification_date, verification_notes,
                bank_name, bank_account_number, bank_account_holder,
                logo_url, documents, status, created_at, updated_at
            FROM companies 
            WHERE owner_id = $1
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .bind(owner_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        let companies: Vec<Company> = rows
            .iter()
            .map(|row| Company {
                id: row.get("id"),
                owner_id: row.get("owner_id"),
                company_name: row.get("company_name"),
                business_type: row.get("business_type"),
                industry_sector: row.get("industry_sector"),
                description: row.get("description"),
                establishment_date: row.get("establishment_date"),
                employee_count: row.get("employee_count"),
                nib: row.get("nib"),
                siup_number: row.get("siup_number"),
                tdp_number: row.get("tdp_number"),
                npwp_company: row.get("npwp_company"),
                email: row.get("email"),
                phone: row.get("phone"),
                website: row.get("website"),
                address_street: row.get("address_street"),
                address_city: row.get("address_city"),
                address_province: row.get("address_province"),
                address_postal_code: row.get("address_postal_code"),
                address_country: row.get("address_country"),
                business_scale: row.get("business_scale"),
                annual_revenue: row.get("annual_revenue"),
                annual_revenue_year: row.get("annual_revenue_year"),
                is_verified: row.get("is_verified"),
                verification_date: row.get("verification_date"),
                verification_notes: row.get("verification_notes"),
                bank_name: row.get("bank_name"),
                bank_account_number: row.get("bank_account_number"),
                bank_account_holder: row.get("bank_account_holder"),
                logo_url: row.get("logo_url"),
                documents: row.get("documents"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(companies)
    }

    async fn find_by_nib(&self, nib: &str) -> AppResult<Option<Company>> {
        let query = r#"
            SELECT 
                id, owner_id, company_name, business_type, industry_sector,
                description, establishment_date, employee_count,
                nib, siup_number, tdp_number, npwp_company,
                email, phone, website,
                address_street, address_city, address_province, 
                address_postal_code, address_country,
                business_scale, annual_revenue, annual_revenue_year,
                is_verified, verification_date, verification_notes,
                bank_name, bank_account_number, bank_account_holder,
                logo_url, documents, status, created_at, updated_at
            FROM companies 
            WHERE nib = $1
        "#;

        let row = sqlx::query(query)
            .bind(nib)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        match row {
            Some(row) => {
                let company = Company {
                    id: row.get("id"),
                    owner_id: row.get("owner_id"),
                    company_name: row.get("company_name"),
                    business_type: row.get("business_type"),
                    industry_sector: row.get("industry_sector"),
                    description: row.get("description"),
                    establishment_date: row.get("establishment_date"),
                    employee_count: row.get("employee_count"),
                    nib: row.get("nib"),
                    siup_number: row.get("siup_number"),
                    tdp_number: row.get("tdp_number"),
                    npwp_company: row.get("npwp_company"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    website: row.get("website"),
                    address_street: row.get("address_street"),
                    address_city: row.get("address_city"),
                    address_province: row.get("address_province"),
                    address_postal_code: row.get("address_postal_code"),
                    address_country: row.get("address_country"),
                    business_scale: row.get("business_scale"),
                    annual_revenue: row.get("annual_revenue"),
                    annual_revenue_year: row.get("annual_revenue_year"),
                    is_verified: row.get("is_verified"),
                    verification_date: row.get("verification_date"),
                    verification_notes: row.get("verification_notes"),
                    bank_name: row.get("bank_name"),
                    bank_account_number: row.get("bank_account_number"),
                    bank_account_holder: row.get("bank_account_holder"),
                    logo_url: row.get("logo_url"),
                    documents: row.get("documents"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(company))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, company: &Company) -> AppResult<()> {
        let query = r#"
            INSERT INTO companies (
                id, owner_id, company_name, business_type, industry_sector,
                description, establishment_date, employee_count,
                nib, siup_number, tdp_number, npwp_company,
                email, phone, website,
                address_street, address_city, address_province, 
                address_postal_code, address_country,
                business_scale, annual_revenue, annual_revenue_year,
                is_verified, verification_date, verification_notes,
                bank_name, bank_account_number, bank_account_holder,
                logo_url, documents, status, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29,
                $30, $31, $32, $33, $34
            )
        "#;

        sqlx::query(query)
            .bind(&company.id)
            .bind(&company.owner_id)
            .bind(&company.company_name)
            .bind(&company.business_type)
            .bind(&company.industry_sector)
            .bind(&company.description)
            .bind(&company.establishment_date)
            .bind(&company.employee_count)
            .bind(&company.nib)
            .bind(&company.siup_number)
            .bind(&company.tdp_number)
            .bind(&company.npwp_company)
            .bind(&company.email)
            .bind(&company.phone)
            .bind(&company.website)
            .bind(&company.address_street)
            .bind(&company.address_city)
            .bind(&company.address_province)
            .bind(&company.address_postal_code)
            .bind(&company.address_country)
            .bind(&company.business_scale)
            .bind(&company.annual_revenue)
            .bind(&company.annual_revenue_year)
            .bind(&company.is_verified)
            .bind(&company.verification_date)
            .bind(&company.verification_notes)
            .bind(&company.bank_name)
            .bind(&company.bank_account_number)
            .bind(&company.bank_account_holder)
            .bind(&company.logo_url)
            .bind(&company.documents)
            .bind(&company.status)
            .bind(&company.created_at)
            .bind(&company.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(())
    }

    async fn update(&self, company: &Company) -> AppResult<()> {
        let query = r#"
            UPDATE companies SET
                company_name = $2, business_type = $3, industry_sector = $4,
                description = $5, establishment_date = $6, employee_count = $7,
                nib = $8, siup_number = $9, tdp_number = $10, npwp_company = $11,
                email = $12, phone = $13, website = $14,
                address_street = $15, address_city = $16, address_province = $17, 
                address_postal_code = $18, address_country = $19,
                business_scale = $20, annual_revenue = $21, annual_revenue_year = $22,
                is_verified = $23, verification_date = $24, verification_notes = $25,
                bank_name = $26, bank_account_number = $27, bank_account_holder = $28,
                logo_url = $29, documents = $30, status = $31, updated_at = $32
            WHERE id = $1
        "#;

        let result = sqlx::query(query)
            .bind(&company.id)
            .bind(&company.company_name)
            .bind(&company.business_type)
            .bind(&company.industry_sector)
            .bind(&company.description)
            .bind(&company.establishment_date)
            .bind(&company.employee_count)
            .bind(&company.nib)
            .bind(&company.siup_number)
            .bind(&company.tdp_number)
            .bind(&company.npwp_company)
            .bind(&company.email)
            .bind(&company.phone)
            .bind(&company.website)
            .bind(&company.address_street)
            .bind(&company.address_city)
            .bind(&company.address_province)
            .bind(&company.address_postal_code)
            .bind(&company.address_country)
            .bind(&company.business_scale)
            .bind(&company.annual_revenue)
            .bind(&company.annual_revenue_year)
            .bind(&company.is_verified)
            .bind(&company.verification_date)
            .bind(&company.verification_notes)
            .bind(&company.bank_name)
            .bind(&company.bank_account_number)
            .bind(&company.bank_account_holder)
            .bind(&company.logo_url)
            .bind(&company.documents)
            .bind(&company.status)
            .bind(&company.updated_at)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Company not found".to_string()));
        }

        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> AppResult<()> {
        let query = "DELETE FROM companies WHERE id = $1";

        let result = sqlx::query(query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Company not found".to_string()));
        }

        Ok(())
    }

    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<Company>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let query = r#"
            SELECT 
                id, owner_id, company_name, business_type, industry_sector,
                description, establishment_date, employee_count,
                nib, siup_number, tdp_number, npwp_company,
                email, phone, website,
                address_street, address_city, address_province, 
                address_postal_code, address_country,
                business_scale, annual_revenue, annual_revenue_year,
                is_verified, verification_date, verification_notes,
                bank_name, bank_account_number, bank_account_holder,
                logo_url, documents, status, created_at, updated_at
            FROM companies 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        "#;

        let rows = sqlx::query(query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        let companies: Vec<Company> = rows
            .iter()
            .map(|row| Company {
                id: row.get("id"),
                owner_id: row.get("owner_id"),
                company_name: row.get("company_name"),
                business_type: row.get("business_type"),
                industry_sector: row.get("industry_sector"),
                description: row.get("description"),
                establishment_date: row.get("establishment_date"),
                employee_count: row.get("employee_count"),
                nib: row.get("nib"),
                siup_number: row.get("siup_number"),
                tdp_number: row.get("tdp_number"),
                npwp_company: row.get("npwp_company"),
                email: row.get("email"),
                phone: row.get("phone"),
                website: row.get("website"),
                address_street: row.get("address_street"),
                address_city: row.get("address_city"),
                address_province: row.get("address_province"),
                address_postal_code: row.get("address_postal_code"),
                address_country: row.get("address_country"),
                business_scale: row.get("business_scale"),
                annual_revenue: row.get("annual_revenue"),
                annual_revenue_year: row.get("annual_revenue_year"),
                is_verified: row.get("is_verified"),
                verification_date: row.get("verification_date"),
                verification_notes: row.get("verification_notes"),
                bank_name: row.get("bank_name"),
                bank_account_number: row.get("bank_account_number"),
                bank_account_holder: row.get("bank_account_holder"),
                logo_url: row.get("logo_url"),
                documents: row.get("documents"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(companies)
    }

    async fn count_by_owner(&self, owner_id: &Uuid) -> AppResult<i64> {
        let query = "SELECT COUNT(*) as count FROM companies WHERE owner_id = $1";

        let row = sqlx::query(query)
            .bind(owner_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        Ok(row.get::<i64, _>("count"))
    }

    async fn search(
        &self,
        query: &str,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> AppResult<Vec<Company>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let search_query = format!("%{}%", query);

        let query = r#"
            SELECT 
                id, owner_id, company_name, business_type, industry_sector,
                description, establishment_date, employee_count,
                nib, siup_number, tdp_number, npwp_company,
                email, phone, website,
                address_street, address_city, address_province, 
                address_postal_code, address_country,
                business_scale, annual_revenue, annual_revenue_year,
                is_verified, verification_date, verification_notes,
                bank_name, bank_account_number, bank_account_holder,
                logo_url, documents, status, created_at, updated_at
            FROM companies 
            WHERE 
                company_name ILIKE $1 OR 
                industry_sector ILIKE $1 OR 
                address_city ILIKE $1 OR
                nib ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        "#;

        let rows = sqlx::query(query)
            .bind(&search_query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e))?;

        let companies: Vec<Company> = rows
            .iter()
            .map(|row| Company {
                id: row.get("id"),
                owner_id: row.get("owner_id"),
                company_name: row.get("company_name"),
                business_type: row.get("business_type"),
                industry_sector: row.get("industry_sector"),
                description: row.get("description"),
                establishment_date: row.get("establishment_date"),
                employee_count: row.get("employee_count"),
                nib: row.get("nib"),
                siup_number: row.get("siup_number"),
                tdp_number: row.get("tdp_number"),
                npwp_company: row.get("npwp_company"),
                email: row.get("email"),
                phone: row.get("phone"),
                website: row.get("website"),
                address_street: row.get("address_street"),
                address_city: row.get("address_city"),
                address_province: row.get("address_province"),
                address_postal_code: row.get("address_postal_code"),
                address_country: row.get("address_country"),
                business_scale: row.get("business_scale"),
                annual_revenue: row.get("annual_revenue"),
                annual_revenue_year: row.get("annual_revenue_year"),
                is_verified: row.get("is_verified"),
                verification_date: row.get("verification_date"),
                verification_notes: row.get("verification_notes"),
                bank_name: row.get("bank_name"),
                bank_account_number: row.get("bank_account_number"),
                bank_account_holder: row.get("bank_account_holder"),
                logo_url: row.get("logo_url"),
                documents: row.get("documents"),
                status: row.get("status"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        Ok(companies)
    }
}
