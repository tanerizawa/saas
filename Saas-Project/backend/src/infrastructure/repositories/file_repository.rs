use async_trait::async_trait;
use sqlx::{PgPool, Postgres, QueryBuilder, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::web::handlers::files::StoredFile;
use crate::shared::errors::AppError;

#[async_trait]
pub trait FileRepository: Send + Sync {
    /// Save file metadata to the database
    async fn save_file(&self, file: &StoredFile) -> Result<StoredFile, AppError>;

    /// Find file by ID
    async fn find_by_id(&self, file_id: &Uuid) -> Result<Option<StoredFile>, AppError>;

    /// Find files by user ID with optional pagination and filtering
    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
        page: u32,
        limit: u32,
        category: Option<&str>,
    ) -> Result<Vec<StoredFile>, AppError>;

    /// Delete a file by ID
    async fn delete_by_id(&self, file_id: &Uuid) -> Result<bool, AppError>;

    /// Count files for a user
    async fn count_by_user_id(
        &self,
        user_id: &Uuid,
        category: Option<&str>,
    ) -> Result<i64, AppError>;
}

/// PostgreSQL implementation of FileRepository
pub struct PgFileRepository {
    pool: Arc<PgPool>,
}

impl PgFileRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileRepository for PgFileRepository {
    async fn save_file(&self, file: &StoredFile) -> Result<StoredFile, AppError> {
        let result = sqlx::query_as::<_, StoredFile>(
            r#"
            INSERT INTO files (
                id, filename, original_filename, content_type, size_bytes,
                path, user_id, uploaded_at, is_public,
                storage_filename, file_path, uploaded_by, category
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING 
                id, filename, original_filename, content_type, size_bytes,
                path, user_id, uploaded_at, last_accessed_at, is_public,
                storage_filename, file_path, uploaded_by, category
            "#,
        )
        .bind(&file.id)
        .bind(&file.filename)
        .bind(&file.original_filename)
        .bind(&file.content_type)
        .bind(&file.size_bytes)
        .bind(&file.path)
        .bind(&file.user_id)
        .bind(&file.uploaded_at)
        .bind(&file.is_public)
        .bind(&file.storage_filename)
        .bind(&file.file_path)
        .bind(&file.uploaded_by)
        .bind(&file.category)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_id(&self, file_id: &Uuid) -> Result<Option<StoredFile>, AppError> {
        let result = sqlx::query_as::<_, StoredFile>(
            r#"
            SELECT 
                id, filename, original_filename, content_type, size_bytes,
                path, user_id, uploaded_at, last_accessed_at, is_public,
                storage_filename, file_path, uploaded_by, category
            FROM files
            WHERE id = $1
            "#,
        )
        .bind(file_id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
        page: u32,
        limit: u32,
        category: Option<&str>,
    ) -> Result<Vec<StoredFile>, AppError> {
        let offset = (page - 1) * limit;

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "SELECT id, filename, original_filename, content_type, size_bytes, path, user_id, uploaded_at, last_accessed_at, is_public, storage_filename, file_path, uploaded_by, category FROM files WHERE user_id = "
        );

        query_builder.push_bind(user_id);

        if let Some(cat) = category {
            query_builder.push(" AND category = ");
            query_builder.push_bind(cat);
        }

        query_builder.push(" ORDER BY uploaded_at DESC LIMIT ");
        query_builder.push_bind(limit as i64);
        query_builder.push(" OFFSET ");
        query_builder.push_bind(offset as i64);

        let result = query_builder
            .build_query_as::<StoredFile>()
            .fetch_all(&*self.pool)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(result)
    }

    async fn delete_by_id(&self, file_id: &Uuid) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM files
            WHERE id = $1
            "#,
        )
        .bind(file_id)
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    async fn count_by_user_id(
        &self,
        user_id: &Uuid,
        category: Option<&str>,
    ) -> Result<i64, AppError> {
        if let Some(cat) = category {
            let result = sqlx::query(
                r#"
                SELECT COUNT(*) as count
                FROM files
                WHERE user_id = $1 AND category = $2
                "#,
            )
            .bind(user_id)
            .bind(cat)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

            let count: i64 = result.get("count");
            Ok(count)
        } else {
            let result = sqlx::query(
                r#"
                SELECT COUNT(*) as count
                FROM files
                WHERE user_id = $1
                "#,
            )
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| AppError::InternalError(e.to_string()))?;

            let count: i64 = result.get("count");
            Ok(count)
        }
    }
}

// Mock implementation for testing
pub struct MockFileRepository {
    files: Vec<StoredFile>,
}

impl MockFileRepository {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }
}

#[async_trait]
impl FileRepository for MockFileRepository {
    async fn save_file(&self, file: &StoredFile) -> Result<StoredFile, AppError> {
        Ok(file.clone())
    }

    async fn find_by_id(&self, file_id: &Uuid) -> Result<Option<StoredFile>, AppError> {
        let file = self.files.iter().find(|f| f.id == *file_id).cloned();
        Ok(file)
    }

    async fn find_by_user_id(
        &self,
        user_id: &Uuid,
        page: u32,
        limit: u32,
        category: Option<&str>,
    ) -> Result<Vec<StoredFile>, AppError> {
        let offset = (page - 1) * limit;
        let mut files: Vec<StoredFile> = self
            .files
            .iter()
            .filter(|f| f.user_id == *user_id)
            .filter(|f| {
                if let Some(cat) = category {
                    if let Some(file_cat) = &f.category {
                        file_cat == cat
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        files.sort_by(|a, b| b.uploaded_at.cmp(&a.uploaded_at));

        let end = (offset as usize + limit as usize).min(files.len());
        let start = offset as usize;

        if start >= files.len() {
            return Ok(Vec::new());
        }

        Ok(files[start..end].to_vec())
    }

    async fn delete_by_id(&self, _file_id: &Uuid) -> Result<bool, AppError> {
        Ok(true)
    }

    async fn count_by_user_id(
        &self,
        user_id: &Uuid,
        category: Option<&str>,
    ) -> Result<i64, AppError> {
        let count = self
            .files
            .iter()
            .filter(|f| f.user_id == *user_id)
            .filter(|f| {
                if let Some(cat) = category {
                    if let Some(file_cat) = &f.category {
                        file_cat == cat
                    } else {
                        false
                    }
                } else {
                    true
                }
            })
            .count();

        Ok(count as i64)
    }
}
