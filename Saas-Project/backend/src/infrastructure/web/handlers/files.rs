use axum::{
    body::StreamBody,
    extract::{Multipart, State, Path},
    response::{Json, Response, IntoResponse},
    http::{StatusCode, header, HeaderMap},
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::{PathBuf, Path as StdPath};
use tokio::{fs, io::BufReader};
use tokio_util::io::ReaderStream;
use uuid::Uuid;
use tracing::{info, error, warn};
use bytes::Bytes;
use mime::Mime;
use mime_guess::from_path;
use std::sync::Arc;
use std::collections::HashSet;

use crate::infrastructure::web::middleware::auth::AuthenticatedUser;
use crate::infrastructure::repositories::FileRepository;
use super::AppState;

/// Represents a file stored in the system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredFile {
    /// Unique identifier for the file
    pub id: Uuid,
    /// Original filename provided during upload
    pub original_filename: String,
    /// Actual filename in the storage system
    pub storage_filename: String,
    /// File mime type
    pub content_type: String,
    /// File size in bytes
    pub size_bytes: i64,
    /// Path where the file is stored
    pub file_path: String,
    /// The user who uploaded this file
    pub uploaded_by: Uuid,
    /// When the file was uploaded
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    /// Optional category for organizing files
    pub category: Option<String>,
}

impl StoredFile {
    /// Creates a new StoredFile instance
    pub fn new(
        original_filename: String,
        storage_filename: String,
        content_type: String,
        size_bytes: i64,
        file_path: String,
        uploaded_by: Uuid,
        category: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            original_filename,
            storage_filename,
            content_type,
            size_bytes,
            file_path,
            uploaded_by,
            uploaded_at: chrono::Utc::now(),
            category,
        }
    }
}

/// Response for file upload
#[derive(Debug, Serialize)]
pub struct FileUploadResponse {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub url: String,
}

/// List of allowed file types for security purposes
const ALLOWED_MIME_TYPES: [&str; 11] = [
    "image/jpeg", "image/png", "image/gif", "image/webp", 
    "application/pdf", "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "text/plain", "application/zip"
];

/// File upload handler with multipart form data
pub async fn upload_file(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<Json<FileUploadResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Create uploads directory if it doesn't exist
    let base_dir = state.config().upload_dir.clone();
    let user_dir = format!("{}/users/{}", base_dir, user.user_id.as_uuid());
    
    if let Err(e) = fs::create_dir_all(&user_dir).await {
        error!("Failed to create upload directory: {}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(json!({
                "error": "Upload failed",
                "message": "Could not prepare storage directory",
                "code": "STORAGE_ERROR"
            }))
        ));
    }
    
    // Process file upload from multipart form
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Error processing multipart form: {}", e);
        (
            StatusCode::BAD_REQUEST, 
            Json(json!({
                "error": "Invalid form data",
                "message": "Could not process upload form",
                "code": "INVALID_FORM"
            }))
        )
    })? {
        let name = field.name().unwrap_or("file").to_string();
        let file_name = match field.file_name() {
            Some(name) => name.to_string(),
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Invalid upload",
                        "message": "Missing filename",
                        "code": "MISSING_FILENAME"
                    }))
                ));
            }
        };
        
        // Get content type or guess from file extension
        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| {
                from_path(&file_name)
                    .first_or_octet_stream()
                    .to_string()
            });
        
        // Validate content type for security
        let allowed_types: HashSet<&str> = ALLOWED_MIME_TYPES.iter().cloned().collect();
        if !allowed_types.contains(content_type.as_str()) {
            return Err((
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Json(json!({
                    "error": "Unsupported file type",
                    "message": "The uploaded file type is not supported",
                    "code": "INVALID_FILE_TYPE",
                    "allowed_types": ALLOWED_MIME_TYPES
                }))
            ));
        }
        
        // Get file data
        let data = field.bytes().await.map_err(|e| {
            error!("Error reading file data: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid file data",
                    "message": "Failed to read file data",
                    "code": "FILE_READ_ERROR"
                }))
            )
        })?;
        
        // Check file size
        if data.len() as u64 > state.config().max_file_size {
            return Err((
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(json!({
                    "error": "File too large",
                    "message": format!("Maximum file size is {} bytes", state.config().max_file_size),
                    "code": "FILE_TOO_LARGE"
                }))
            ));
        }
        
        // Sanitize filename for storage
        let sanitized_name = file_name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
        
        // Create a unique filename to prevent collisions
        let storage_filename = format!("{}-{}", Uuid::new_v4(), sanitized_name);
        let file_path = format!("{}/{}", user_dir, storage_filename);
        
        // Save file to disk
        if let Err(e) = fs::write(&file_path, &data).await {
            error!("Failed to save file to disk: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Storage error",
                    "message": "Failed to save file",
                    "code": "FILE_WRITE_ERROR"
                }))
            ));
        }
        
        // Parse optional category parameter
        let category = field.name()
            .filter(|n| n.starts_with("category_"))
            .map(|n| n.trim_start_matches("category_").to_string());
        
        // Create file record
        let stored_file = StoredFile::new(
            file_name,
            storage_filename.clone(),
            content_type.clone(),
            data.len() as i64,
            file_path,
            *user.user_id.as_uuid(),
            category,
        );
        
        // In a full implementation, save the file metadata to database
        // This would use your FileRepository - but we'll leave as comment for now since
        // we haven't added the repository to AppState yet
        
        /* 
        let saved_file = match state.file_repository().save_file(&stored_file).await {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to save file metadata: {}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Database error",
                        "message": "Failed to save file metadata",
                        "code": "DB_ERROR"
                    }))
                ));
            }
        };
        */
        
        // Generate the URL for the file
        let url = format!("/api/v1/files/{}", stored_file.id);
        
        let response = FileUploadResponse {
            id: stored_file.id,
            filename: stored_file.original_filename,
            content_type: stored_file.content_type,
            size_bytes: stored_file.size_bytes,
            uploaded_at: stored_file.uploaded_at,
            url,
        };
        
        info!("File uploaded successfully: {} ({} bytes)", storage_filename, data.len());
        
        // Return after processing the first file
        // In a real implementation, you might want to handle multiple files
        return Ok(Json(response));
    }
    
    // If we got here, no files were found in the form
    Err((
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "No file uploaded",
            "message": "No file was found in the request",
            "code": "NO_FILE"
        }))
    ))
}

/// Get file metadata by ID
pub async fn get_file_metadata(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(file_id): Path<Uuid>,
) -> Result<Json<StoredFile>, (StatusCode, Json<serde_json::Value>)> {
    // In a full implementation, you would retrieve the file metadata from the database
    // This would be where your repository pattern comes in
    
    // Simulate file lookup for demonstration
    // In a real implementation, this would query the database
    
    // Placeholder implementation
    if file_id.is_nil() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "File not found",
                "message": "The requested file does not exist",
                "code": "FILE_NOT_FOUND"
            }))
        ));
    }
    
    // For now, return a mock file record for testing
    let mock_file = StoredFile {
        id: file_id,
        original_filename: "example.pdf".to_string(),
        storage_filename: format!("{}-example.pdf", file_id),
        content_type: "application/pdf".to_string(),
        size_bytes: 1024,
        file_path: format!("{}/users/{}/{}-example.pdf", 
                           state.config().upload_dir,
                           user.user_id.as_uuid(),
                           file_id),
        uploaded_by: *user.user_id.as_uuid(),
        uploaded_at: chrono::Utc::now(),
        category: Some("document".to_string()),
    };
    
    Ok(Json(mock_file))
}

/// Delete a file by ID
pub async fn delete_file(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(file_id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // In a full implementation, you would:
    // 1. Look up file metadata in database to verify ownership
    // 2. Delete file from storage
    // 3. Remove metadata from database
    
    // Mock implementation for demonstration
    // This would integrate with your file repository in a real implementation
    
    let mock_file_path = format!("{}/users/{}/{}-example.pdf", 
                               state.config().upload_dir,
                               user.user_id.as_uuid(),
                               file_id);
    
    // In a real implementation, you'd check if the file exists in the database first
    // and verify that the current user owns it
    
    // Check if file exists on disk (simplified)
    if !StdPath::new(&mock_file_path).exists() {
        // We'll pretend the file exists for this mock implementation
        warn!("File not found on disk, but proceeding with mock deletion");
    } else {
        // If file exists, try to delete it
        if let Err(e) = fs::remove_file(&mock_file_path).await {
            error!("Failed to delete file {}: {}", mock_file_path, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Delete failed",
                    "message": "Failed to delete file from storage",
                    "code": "DELETE_ERROR"
                }))
            ));
        }
    }
    
    // In a real implementation, you'd also delete the metadata from the database here
    
    // Return success
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category: Option<String>,
}

/// Response for file listing
#[derive(Debug, Serialize)]
pub struct FileListResponse {
    pub files: Vec<StoredFile>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}

/// List files for the current user with pagination
pub async fn list_files(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    query: Option<axum::extract::Query<FileListParams>>,
) -> Result<Json<FileListResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Extract query parameters with defaults
    let params = query.unwrap_or_default();
    let page = params.0.page.unwrap_or(1);
    let limit = params.0.limit.unwrap_or(10);
    let category = params.0.category;
    
    // Validate pagination parameters
    if page == 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid parameters",
                "message": "Page must be greater than 0",
                "code": "INVALID_PAGINATION"
            }))
        ));
    }
    
    if limit == 0 || limit > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Invalid parameters", 
                "message": "Limit must be between 1 and 100",
                "code": "INVALID_PAGINATION"
            }))
        ));
    }
    
    // In a full implementation, this would use FileRepository:
    /*
    // Get total count for pagination
    let total = match state.file_repository().count_by_user_id(
        user.user_id.as_uuid(),
        category.as_deref()
    ).await {
        Ok(count) => count,
        Err(e) => {
            error!("Failed to count files: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to retrieve file count",
                    "code": "DB_ERROR"
                }))
            ));
        }
    };
    
    // Get files for the current page
    let files = match state.file_repository().find_by_user_id(
        user.user_id.as_uuid(),
        page,
        limit,
        category.as_deref()
    ).await {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to retrieve files: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database error",
                    "message": "Failed to retrieve files",
                    "code": "DB_ERROR"
                }))
            ));
        }
    };
    */
    
    // Mock implementation for demonstration
    // Generate some mock files
    let mut mock_files = Vec::new();
    
    // Only add category-specific files if a category filter is provided
    let categories = match &category {
        Some(cat) => vec![cat.as_str()],
        None => vec!["document", "image", "spreadsheet"]
    };
    
    for i in 0..3 {
        let file_type = match i % 3 {
            0 => "application/pdf",
            1 => "image/jpeg",
            _ => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        };
        
        let file_category = categories[i % categories.len()];
        
        let mock_file = StoredFile {
            id: Uuid::new_v4(),
            original_filename: format!("example-{}.{}", i, if i % 3 == 0 { "pdf" } else if i % 3 == 1 { "jpg" } else { "xlsx" }),
            storage_filename: format!("{}-example-{}.{}", Uuid::new_v4(), i, if i % 3 == 0 { "pdf" } else if i % 3 == 1 { "jpg" } else { "xlsx" }),
            content_type: file_type.to_string(),
            size_bytes: 1024 * (i + 1),
            file_path: format!("{}/users/{}/example-{}.{}", 
                             state.config().upload_dir,
                             user.user_id.as_uuid(),
                             i,
                             if i % 3 == 0 { "pdf" } else if i % 3 == 1 { "jpg" } else { "xlsx" }),
            uploaded_by: *user.user_id.as_uuid(),
            uploaded_at: chrono::Utc::now() - chrono::Duration::days(i as i64),
            category: Some(file_category.to_string()),
        };
        
        mock_files.push(mock_file);
    }
    
    // Calculate total pages
    let total = 10; // Mock total count
    let total_pages = (total as f64 / limit as f64).ceil() as u32;
    
    // Create response
    let response = FileListResponse {
        files: mock_files,
        total,
        page,
        limit,
        total_pages,
    };
    
    // Return our mock response
    Ok(Json(response))
}

/// Get file content by ID - streams the actual file
pub async fn get_file_content(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(file_id): Path<Uuid>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // In a full implementation, you would:
    // 1. Look up file metadata in database
    // 2. Check permissions (if user owns file or has access)
    // 3. Verify file exists on disk
    // 4. Stream file to client
    
    // This implementation would integrate with your file repository
    
    // Placeholder for demonstration - creates a path where we'd expect the file
    let file_path = format!("{}/users/{}/{}-example.pdf", 
                          state.config().upload_dir,
                          user.user_id.as_uuid(),
                          file_id);
    
    // Check if file exists (would be from DB in real implementation)
    if !StdPath::new(&file_path).exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "File not found",
                "message": "The requested file does not exist on disk",
                "code": "FILE_NOT_FOUND"
            }))
        ));
    }
    
    // Open the file
    let file = match fs::File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => {
            error!("Failed to open file {}: {}", file_path, err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "File access error",
                    "message": "Could not access the requested file",
                    "code": "FILE_ACCESS_ERROR"
                }))
            ));
        }
    };
    
    // Get file size for content-length header
    let metadata = match file.metadata().await {
        Ok(metadata) => metadata,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "File metadata error",
                    "message": "Could not read file metadata",
                    "code": "FILE_METADATA_ERROR"
                }))
            ));
        }
    };
    
    // Create a buffer reader and a stream from the file
    let buf_reader = BufReader::new(file);
    let stream = ReaderStream::new(buf_reader);
    let body = StreamBody::new(stream);
    
    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(header::CONTENT_LENGTH, metadata.len().to_string().parse().unwrap());
    headers.insert(header::CONTENT_DISPOSITION, format!("attachment; filename=\"example.pdf\"").parse().unwrap());
    
    // Return the streamed response
    Ok((StatusCode::OK, headers, body).into_response())
}

/// Routes for file management
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(upload_file))
        .route("/", get(list_files))
        .route("/:id", get(get_file_metadata))
        .route("/:id/content", get(get_file_content))
        .route("/:id", delete(delete_file))
}
