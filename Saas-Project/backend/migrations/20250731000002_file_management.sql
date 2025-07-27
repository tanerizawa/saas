-- Migration to create files table for storing file metadata
CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY,
    original_filename VARCHAR(255) NOT NULL,
    storage_filename VARCHAR(255) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    size_bytes BIGINT NOT NULL,
    file_path VARCHAR(1024) NOT NULL,
    uploaded_by UUID NOT NULL REFERENCES users(id),
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    category VARCHAR(100),
    
    CONSTRAINT file_path_unique UNIQUE (file_path)
);

-- Add indexes for common queries
CREATE INDEX idx_files_uploaded_by ON files(uploaded_by);
CREATE INDEX idx_files_category ON files(category);
CREATE INDEX idx_files_uploaded_at ON files(uploaded_at);

-- Add index for content type to help with filtering by file type
CREATE INDEX idx_files_content_type ON files(content_type);

-- Rollback functionality
-- DROP TABLE IF EXISTS files;
