use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::domain::entities::{User, UserRole};
use crate::domain::value_objects::UserId;

#[derive(Debug, Clone)]
pub struct AuthService {
    jwt_secret: String,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub role: String, // User role
    pub exp: i64,     // Expiration time
    pub iat: i64,     // Issued at
    pub jti: String,  // JWT ID
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub role: UserRole,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UserAlreadyExists,
    TokenExpired,
    InvalidToken,
    PasswordHashError,
    DatabaseError(String),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid email or password"),
            AuthError::UserAlreadyExists => write!(f, "User with this email already exists"),
            AuthError::TokenExpired => write!(f, "Token has expired"),
            AuthError::InvalidToken => write!(f, "Invalid token"),
            AuthError::PasswordHashError => write!(f, "Failed to hash password"),
            AuthError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for AuthError {}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self {
            jwt_secret,
            access_token_duration: Duration::minutes(15), // 15 minutes
            refresh_token_duration: Duration::days(7),    // 7 days
        }
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        // Use lighter Argon2 configuration for development
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(
                8192, // m_cost (memory cost in KB) - reduced from default ~19MB to 8MB
                3,    // t_cost (time cost) - reduced from default
                1,    // p_cost (parallelism) - single thread
                None, // output length
            )
            .map_err(|_| AuthError::PasswordHashError)?,
        );

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| AuthError::PasswordHashError)
    }

    /// Verify a password against its hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::InvalidCredentials)?;

        let argon2 = Argon2::default();
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate JWT access token
    pub fn generate_access_token(&self, user: &User) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = now + self.access_token_duration;

        let claims = Claims {
            sub: user.id.to_string(),
            role: user.role.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AuthError::InvalidToken)
    }

    /// Generate JWT refresh token
    pub fn generate_refresh_token(&self, user: &User) -> Result<String, AuthError> {
        let now = Utc::now();
        let exp = now + self.refresh_token_duration;

        let claims = Claims {
            sub: user.id.to_string(),
            role: user.role.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|_| AuthError::InvalidToken)
    }

    /// Generate both access and refresh tokens
    pub fn generate_tokens(&self, user: &User) -> Result<AuthTokens, AuthError> {
        let access_token = self.generate_access_token(user)?;
        let refresh_token = self.generate_refresh_token(user)?;
        let expires_at = Utc::now() + self.access_token_duration;

        Ok(AuthTokens {
            access_token,
            refresh_token,
            expires_at,
        })
    }

    /// Validate and decode JWT token
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::default();

        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })
    }

    /// Extract user ID from token
    pub fn extract_user_id(&self, token: &str) -> Result<UserId, AuthError> {
        let claims = self.validate_token(token)?;
        let uuid = Uuid::parse_str(&claims.sub).map_err(|_| AuthError::InvalidToken)?;
        Ok(UserId(uuid))
    }

    /// Extract user role from token
    pub fn extract_user_role(&self, token: &str) -> Result<UserRole, AuthError> {
        let claims = self.validate_token(token)?;
        claims
            .role
            .parse::<UserRole>()
            .map_err(|_| AuthError::InvalidToken)
    }

    /// Extract company ID from token (placeholder implementation)
    pub fn extract_company_id(&self, _token: &str) -> Result<Uuid, AuthError> {
        // For now, return a default company ID
        // In a real implementation, this would be part of the token claims
        Ok(Uuid::new_v4())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::Email;

    fn create_test_user() -> User {
        let email = Email::new("test@example.com").unwrap();
        User::new(
            email,
            "hashed_password".to_string(),
            "Test User".to_string(),
            UserRole::UmkmOwner,
        )
    }

    #[test]
    fn test_password_hashing() {
        let auth_service = AuthService::new("test_secret".to_string());
        let password = "test_password_123";

        let hash = auth_service.hash_password(password).unwrap();
        assert!(auth_service.verify_password(password, &hash).unwrap());
        assert!(!auth_service
            .verify_password("wrong_password", &hash)
            .unwrap());
    }

    #[test]
    fn test_token_generation_and_validation() {
        let auth_service = AuthService::new("test_secret".to_string());
        let user = create_test_user();

        let tokens = auth_service.generate_tokens(&user).unwrap();
        let claims = auth_service.validate_token(&tokens.access_token).unwrap();

        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.role, user.role.to_string());
    }

    #[test]
    fn test_user_id_extraction() {
        let auth_service = AuthService::new("test_secret".to_string());
        let user = create_test_user();

        let token = auth_service.generate_access_token(&user).unwrap();
        let extracted_id = auth_service.extract_user_id(&token).unwrap();

        assert_eq!(extracted_id, user.id);
    }
}
