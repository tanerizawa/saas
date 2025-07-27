// PostgreSQL implementation of UserRepository
// Follows Hexagonal Architecture pattern for data access

use async_trait::async_trait;
use sqlx::PgPool;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::domain::entities::{User, UserRole, UserStatus};
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::{Email, UserId};
use crate::shared::errors::{AppError, AppResult};

/// PostgreSQL implementation of UserRepository
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_user(&self, row: &sqlx::postgres::PgRow) -> AppResult<User> {
        use sqlx::Row;

        let id_uuid: Uuid = row
            .try_get("id")
            .map_err(|e| AppError::Validation(format!("Invalid user id: {}", e)))?;
        let user_id = UserId::from_uuid(id_uuid);

        let email_str: String = row
            .try_get("email")
            .map_err(|e| AppError::Validation(format!("Invalid email: {}", e)))?;
        let email = Email::new(&email_str)
            .map_err(|e| AppError::Validation(format!("Invalid email format: {}", e)))?;

        let role_str: String = row
            .try_get("role")
            .map_err(|e| AppError::Validation(format!("Invalid role: {}", e)))?;
        let role = match role_str.as_str() {
            "umkm_owner" => UserRole::UmkmOwner,
            "admin_staff" => UserRole::AdminStaff,
            "super_admin" => UserRole::SuperAdmin,
            _ => return Err(AppError::Validation(format!("Unknown role: {}", role_str))),
        };

        let status_str: String = row
            .try_get("status")
            .map_err(|e| AppError::Validation(format!("Invalid status: {}", e)))?;
        let status = match status_str.as_str() {
            "active" => UserStatus::Active,
            "inactive" => UserStatus::Inactive,
            "pending_verification" => UserStatus::PendingVerification,
            "suspended" => UserStatus::Suspended,
            _ => {
                return Err(AppError::Validation(format!(
                    "Unknown status: {}",
                    status_str
                )))
            }
        };

        let _phone_str: Option<String> = None; // Database doesn't have phone column yet
        let phone = None;

        let email_verified: bool = row
            .try_get("email_verified")
            .map_err(|e| AppError::Validation(format!("Invalid email_verified: {}", e)))?;
        let email_verified_at = if email_verified {
            Some(
                row.try_get("created_at")
                    .map_err(|e| AppError::Validation(format!("Invalid created_at: {}", e)))?,
            )
        } else {
            None
        };

        Ok(User {
            id: user_id,
            email,
            password_hash: row
                .try_get("password_hash")
                .map_err(|e| AppError::Validation(format!("Invalid password_hash: {}", e)))?,
            full_name: row
                .try_get("full_name")
                .map_err(|e| AppError::Validation(format!("Invalid full_name: {}", e)))?,
            phone,
            role,
            status,
            email_verified_at,
            created_at: row
                .try_get("created_at")
                .map_err(|e| AppError::Validation(format!("Invalid created_at: {}", e)))?,
            updated_at: row
                .try_get("updated_at")
                .map_err(|e| AppError::Validation(format!("Invalid updated_at: {}", e)))?,
        })
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    #[instrument(skip(self), fields(user_id = %id.as_uuid()))]
    async fn find_by_id(&self, id: &UserId) -> AppResult<Option<User>> {
        info!("Finding user by id: {}", id.as_uuid());

        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, full_name, role, status, 
                   email_verified, created_at, updated_at
            FROM users 
            WHERE id = $1
            "#,
        )
        .bind(id.as_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Database error finding user by id: {}", e);
            AppError::Database(e)
        })?;

        match result {
            Some(row) => {
                let user = self.row_to_user(&row)?;
                info!("Found user by id: {}", id.as_uuid());
                Ok(Some(user))
            }
            None => {
                info!("User not found for id: {}", id.as_uuid());
                Ok(None)
            }
        }
    }

    #[instrument(skip(self), fields(email = %email.as_str()))]
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        info!("Finding user by email: {}", email.as_str());

        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, full_name, role, status, 
                   email_verified, created_at, updated_at
            FROM users 
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            error!("Database error finding user by email: {}", e);
            AppError::Database(e)
        })?;

        match result {
            Some(row) => {
                let user = self.row_to_user(&row)?;
                info!("Found user by email: {}", email.as_str());
                Ok(Some(user))
            }
            None => {
                info!("User not found for email: {}", email.as_str());
                Ok(None)
            }
        }
    }

    #[instrument(skip(self, user), fields(user_id = %user.id.as_uuid(), email = %user.email.as_str()))]
    async fn save(&self, user: &User) -> AppResult<()> {
        info!("Saving user: {}", user.id.as_uuid());

        let role_str = match user.role {
            UserRole::UmkmOwner => "umkm_owner",
            UserRole::AdminStaff => "admin_staff",
            UserRole::SuperAdmin => "super_admin",
        };

        let status_str = match user.status {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
            UserStatus::PendingVerification => "pending_verification",
            UserStatus::Suspended => "suspended",
        };

        let _phone_str: Option<&str> = None; // Database doesn't have phone column yet
        let _email_verified = user.email_verified_at.is_some(); // Will be used when we implement email verification

        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, full_name, role, status, email_verified, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) 
            DO UPDATE SET 
                email = EXCLUDED.email,
                password_hash = EXCLUDED.password_hash,
                full_name = EXCLUDED.full_name,
                role = EXCLUDED.role,
                status = EXCLUDED.status,
                email_verified = EXCLUDED.email_verified,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(user.id.as_uuid())
        .bind(user.email.as_str())
        .bind(&user.password_hash)
        .bind(&user.full_name)
        .bind(role_str)
        .bind(status_str)
        .bind(user.email_verified_at.is_some())
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Database error saving user: {}", e);
            AppError::Database(e)
        })?;

        info!("Successfully saved user: {}", user.id.as_uuid());
        Ok(())
    }

    #[instrument(skip(self), fields(user_id = %id.as_uuid()))]
    async fn delete(&self, id: &UserId) -> AppResult<()> {
        info!("Deleting user: {}", id.as_uuid());

        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id.as_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Database error deleting user: {}", e);
                AppError::Database(e)
            })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        info!("Successfully deleted user: {}", id.as_uuid());
        Ok(())
    }
    
    #[instrument(skip(self), fields(limit, offset))]
    async fn list_all(&self, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
        info!("Listing users with limit: {:?}, offset: {:?}", limit, offset);

        let query = match (limit, offset) {
            (Some(limit), Some(offset)) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    LIMIT $1 OFFSET $2
                    "#
                )
                .bind(limit)
                .bind(offset)
            },
            (Some(limit), None) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    LIMIT $1
                    "#
                )
                .bind(limit)
            },
            _ => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    ORDER BY created_at DESC
                    "#
                )
            }
        };

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Database error listing users: {}", e);
                AppError::Database(e)
            })?;

        let mut users = Vec::new();
        for row in rows {
            match self.row_to_user(&row) {
                Ok(user) => users.push(user),
                Err(e) => {
                    error!("Error converting row to user: {}", e);
                    continue;
                }
            }
        }

        info!("Found {} users", users.len());
        Ok(users)
    }

    #[instrument(skip(self))]
    async fn count_all(&self) -> AppResult<i64> {
        use sqlx::Row;
        
        let row = sqlx::query("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                error!("Database error counting users: {}", e);
                AppError::Database(e)
            })?;

        let count: i64 = row.try_get("count").map_err(|e| {
            error!("Error getting count from row: {}", e);
            AppError::Database(e)
        })?;

        info!("Total user count: {}", count);
        Ok(count)
    }

    #[instrument(skip(self), fields(query))]
    async fn search(&self, query: &str, limit: Option<i32>, offset: Option<i32>) -> AppResult<Vec<User>> {
        info!("Searching users with query: {}", query);

        let search_term = format!("%{}%", query);
        let sql_query = match (limit, offset) {
            (Some(limit), Some(offset)) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    WHERE email ILIKE $1 OR full_name ILIKE $1
                    ORDER BY created_at DESC
                    LIMIT $2 OFFSET $3
                    "#
                )
                .bind(&search_term)
                .bind(limit)
                .bind(offset)
            },
            (Some(limit), None) => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    WHERE email ILIKE $1 OR full_name ILIKE $1
                    ORDER BY created_at DESC
                    LIMIT $2
                    "#
                )
                .bind(&search_term)
                .bind(limit)
            },
            _ => {
                sqlx::query(
                    r#"
                    SELECT id, email, password_hash, full_name, role, status, 
                           email_verified, created_at, updated_at
                    FROM users
                    WHERE email ILIKE $1 OR full_name ILIKE $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(&search_term)
            }
        };

        let rows = sql_query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Database error searching users: {}", e);
                AppError::Database(e)
            })?;

        let mut users = Vec::new();
        for row in rows {
            match self.row_to_user(&row) {
                Ok(user) => users.push(user),
                Err(e) => {
                    error!("Error converting row to user: {}", e);
                    continue;
                }
            }
        }

        info!("Found {} users matching query: {}", users.len(), query);
        Ok(users)
    }
}
