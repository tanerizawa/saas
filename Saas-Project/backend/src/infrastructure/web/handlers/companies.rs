use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{
        companies::{BusinessScale, BusinessType, Company, CompanyStatus},
        entities::UserRole,
    },
    infrastructure::web::middleware::auth::AuthenticatedUser,
    shared::errors::{AppError, AppResult},
};

// Import the AppState from handlers module
use super::AppState;

// Request/Response DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCompanyRequest {
    pub company_name: String,
    pub business_type: String,
    pub industry: String,
    pub description: Option<String>,
    pub address_street: String,
    pub address_city: String,
    pub address_province: String,
    pub address_postal_code: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub nib: Option<String>,
    pub siup: Option<String>,
    pub tdp: Option<String>,
    pub npwp: Option<String>,
    pub employee_count: Option<i32>,
    pub annual_revenue: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCompanyRequest {
    pub company_name: Option<String>,
    pub business_type: Option<String>,
    pub industry: Option<String>,
    pub description: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_province: Option<String>,
    pub address_postal_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub nib: Option<String>,
    pub siup: Option<String>,
    pub tdp: Option<String>,
    pub npwp: Option<String>,
    pub employee_count: Option<i32>,
    pub annual_revenue: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub company_name: String,
    pub business_type: String,
    pub business_scale: String,
    pub industry: String,
    pub description: Option<String>,
    pub address: CompanyAddressResponse,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub nib: Option<String>,
    pub siup: Option<String>,
    pub tdp: Option<String>,
    pub npwp: Option<String>,
    pub employee_count: Option<i32>,
    pub annual_revenue: Option<i64>,
    pub status: String,
    pub verification_status: String,
    pub verification_notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyAddressResponse {
    pub street: String,
    pub city: String,
    pub province: String,
    pub postal_code: String,
}

#[derive(Debug, Deserialize)]
pub struct ListCompaniesQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListCompaniesResponse {
    pub companies: Vec<CompanyResponse>,
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
}

// Helper function to convert Company domain entity to response DTO
fn company_to_response(company: &Company) -> CompanyResponse {
    CompanyResponse {
        id: company.id,
        owner_id: company.owner_id,
        company_name: company.company_name.clone(),
        business_type: company.business_type.clone(),
        business_scale: company.business_scale.clone(),
        industry: company.industry_sector.clone(),
        description: company.description.clone(),
        address: CompanyAddressResponse {
            street: company.address_street.clone(),
            city: company.address_city.clone(),
            province: company.address_province.clone(),
            postal_code: company.address_postal_code.clone(),
        },
        phone: company.phone.clone(),
        email: company.email.clone(),
        website: company.website.clone(),
        nib: company.nib.clone(),
        siup: company.siup_number.clone(),
        tdp: company.tdp_number.clone(),
        npwp: company.npwp_company.clone(),
        employee_count: Some(company.employee_count),
        annual_revenue: company.annual_revenue,
        status: company.status.clone(),
        verification_status: if company.is_verified {
            "verified".to_string()
        } else {
            "pending".to_string()
        },
        verification_notes: company.verification_notes.clone(),
        created_at: company.created_at,
        updated_at: company.updated_at,
    }
}

// Helper function to validate business type
fn validate_business_type(business_type: &str) -> Result<BusinessType, String> {
    match business_type.to_lowercase().as_str() {
        "pt" => Ok(BusinessType::PT),
        "cv" => Ok(BusinessType::CV),
        "ud" => Ok(BusinessType::UD),
        "koperasi" => Ok(BusinessType::Koperasi),
        "perorangan" => Ok(BusinessType::Perorangan),
        _ => Err(format!("Invalid business type: {}", business_type)),
    }
}

// Helper function to determine business scale from revenue
fn determine_business_scale(
    annual_revenue: Option<i64>,
    employee_count: Option<i32>,
) -> BusinessScale {
    if let Some(revenue) = annual_revenue {
        if revenue <= 300_000_000 {
            BusinessScale::Mikro
        } else if revenue <= 2_500_000_000 {
            BusinessScale::Kecil
        } else if revenue <= 50_000_000_000 {
            BusinessScale::Menengah
        } else {
            // For large businesses above 50 billion, we'll classify as Menengah for now
            // since our enum doesn't have Besar
            BusinessScale::Menengah
        }
    } else if let Some(employees) = employee_count {
        if employees <= 4 {
            BusinessScale::Mikro
        } else if employees <= 19 {
            BusinessScale::Kecil
        } else {
            BusinessScale::Menengah
        }
    } else {
        BusinessScale::Mikro // Default
    }
}

// API Handlers
pub async fn create_company(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateCompanyRequest>,
) -> AppResult<(StatusCode, Json<CompanyResponse>)> {
    let company_repo = state.company_repository();

    // Validate business type
    let business_type =
        validate_business_type(&payload.business_type).map_err(|e| AppError::Validation(e))?;

    // Determine business scale
    let business_scale = determine_business_scale(payload.annual_revenue, payload.employee_count);

    // Create company entity
    let company = Company {
        id: Uuid::new_v4(),
        owner_id: user.user_id.as_uuid().clone(),
        company_name: payload.company_name,
        business_type: business_type.to_string(),
        industry_sector: payload.industry,
        description: payload.description,
        establishment_date: None,
        employee_count: payload.employee_count.unwrap_or(0),
        nib: payload.nib,
        siup_number: payload.siup,
        tdp_number: payload.tdp,
        npwp_company: payload.npwp,
        email: payload.email,
        phone: payload.phone,
        website: payload.website,
        address_street: payload.address_street,
        address_city: payload.address_city,
        address_province: payload.address_province,
        address_postal_code: payload.address_postal_code,
        address_country: "Indonesia".to_string(),
        business_scale: business_scale.to_string(),
        annual_revenue: payload.annual_revenue,
        annual_revenue_year: None,
        is_verified: false,
        verification_date: None,
        verification_notes: None,
        bank_name: None,
        bank_account_number: None,
        bank_account_holder: None,
        logo_url: None,
        documents: serde_json::json!({}),
        status: CompanyStatus::Active.to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Save to repository
    company_repo.save(&company).await?;

    let response = company_to_response(&company);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn get_company(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> AppResult<Json<CompanyResponse>> {
    let company_repo = state.company_repository();

    let company = company_repo
        .find_by_id(&company_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    // Check if user owns the company or is admin
    if company.owner_id != *user.user_id.as_uuid() && user.role != UserRole::SuperAdmin {
        return Err(AppError::Forbidden(
            "You don't have permission to access this company".to_string(),
        ));
    }

    let response = company_to_response(&company);
    Ok(Json(response))
}

pub async fn update_company(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
    Json(payload): Json<UpdateCompanyRequest>,
) -> AppResult<Json<CompanyResponse>> {
    let company_repo = state.company_repository();

    let mut company = company_repo
        .find_by_id(&company_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    // Check if user owns the company or is admin
    if company.owner_id != *user.user_id.as_uuid() && user.role != UserRole::SuperAdmin {
        return Err(AppError::Forbidden(
            "You don't have permission to update this company".to_string(),
        ));
    }

    // Update fields if provided
    if let Some(name) = payload.company_name {
        company.company_name = name;
    }
    if let Some(business_type) = payload.business_type {
        let validated_type =
            validate_business_type(&business_type).map_err(|e| AppError::Validation(e))?;
        company.business_type = validated_type.to_string();
    }
    if let Some(industry) = payload.industry {
        company.industry_sector = industry;
    }
    if let Some(description) = payload.description {
        company.description = Some(description);
    }
    if let Some(street) = payload.address_street {
        company.address_street = street;
    }
    if let Some(city) = payload.address_city {
        company.address_city = city;
    }
    if let Some(province) = payload.address_province {
        company.address_province = province;
    }
    if let Some(postal_code) = payload.address_postal_code {
        company.address_postal_code = postal_code;
    }
    if let Some(phone) = payload.phone {
        company.phone = Some(phone);
    }
    if let Some(email) = payload.email {
        company.email = Some(email);
    }
    if let Some(website) = payload.website {
        company.website = Some(website);
    }
    if let Some(nib) = payload.nib {
        company.nib = Some(nib);
    }
    if let Some(siup) = payload.siup {
        company.siup_number = Some(siup);
    }
    if let Some(tdp) = payload.tdp {
        company.tdp_number = Some(tdp);
    }
    if let Some(npwp) = payload.npwp {
        company.npwp_company = Some(npwp);
    }
    if let Some(employee_count) = payload.employee_count {
        company.employee_count = employee_count;
    }
    if let Some(annual_revenue) = payload.annual_revenue {
        company.annual_revenue = Some(annual_revenue);
        // Update business scale based on new revenue
        let new_scale =
            determine_business_scale(Some(annual_revenue), Some(company.employee_count));
        company.business_scale = new_scale.to_string();
    }

    company.updated_at = chrono::Utc::now();

    // Save updated company
    company_repo.update(&company).await?;

    let response = company_to_response(&company);
    Ok(Json(response))
}

pub async fn delete_company(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(company_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let company_repo = state.company_repository();

    let company = company_repo
        .find_by_id(&company_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Company not found".to_string()))?;

    // Check if user owns the company or is admin
    if company.owner_id != *user.user_id.as_uuid() && user.role != UserRole::SuperAdmin {
        return Err(AppError::Forbidden(
            "You don't have permission to delete this company".to_string(),
        ));
    }

    company_repo.delete(&company_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_companies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(query): Query<ListCompaniesQuery>,
) -> AppResult<Json<ListCompaniesResponse>> {
    let company_repo = state.company_repository();

    let limit = query.limit.unwrap_or(20).min(100); // Max 100 items per page
    let offset = query.offset.unwrap_or(0);

    let companies = if user.role == UserRole::SuperAdmin {
        // Admin can see all companies
        if let Some(search_query) = query.search {
            company_repo
                .search(&search_query, Some(limit), Some(offset))
                .await?
        } else {
            company_repo.list_all(Some(limit), Some(offset)).await?
        }
    } else {
        // Regular users can only see their own companies
        company_repo
            .find_by_owner_id(user.user_id.as_uuid())
            .await?
    };

    let total = if user.role == UserRole::SuperAdmin {
        // For admin, we need to count all companies
        // For now, we'll use the companies count as approximation
        companies.len() as i64
    } else {
        company_repo.count_by_owner(user.user_id.as_uuid()).await?
    };

    let response_companies: Vec<CompanyResponse> =
        companies.iter().map(company_to_response).collect();

    let response = ListCompaniesResponse {
        companies: response_companies,
        total,
        limit,
        offset,
    };

    Ok(Json(response))
}

pub async fn get_my_companies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<Json<Vec<CompanyResponse>>> {
    let company_repo = state.company_repository();

    let companies = company_repo
        .find_by_owner_id(user.user_id.as_uuid())
        .await?;

    let response_companies: Vec<CompanyResponse> =
        companies.iter().map(company_to_response).collect();

    Ok(Json(response_companies))
}

// Routes configuration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_company))
        .route("/", get(list_companies))
        .route("/my", get(get_my_companies))
        .route("/:id", get(get_company))
        .route("/:id", put(update_company))
        .route("/:id", delete(delete_company))
}
