-- File management tables for Phase 5

-- Create files table
CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    filename TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size BIGINT NOT NULL,
    path TEXT NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_accessed_at TIMESTAMPTZ,
    is_public BOOLEAN NOT NULL DEFAULT FALSE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_files_user_id ON files(user_id);
CREATE INDEX IF NOT EXISTS idx_files_uploaded_at ON files(uploaded_at);

-- Add function to update last_accessed_at
CREATE OR REPLACE FUNCTION update_file_access_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_accessed_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to update last_accessed_at when file is accessed
CREATE TRIGGER update_file_access
BEFORE UPDATE ON files
FOR EACH ROW
WHEN (OLD.* IS DISTINCT FROM NEW.*)
EXECUTE FUNCTION update_file_access_timestamp();

-- Add comments for documentation
COMMENT ON TABLE files IS 'Stores metadata about uploaded files';
COMMENT ON COLUMN files.id IS 'Unique identifier for the file';
COMMENT ON COLUMN files.filename IS 'System filename (UUID-based)';
COMMENT ON COLUMN files.original_filename IS 'Original filename as uploaded by the user';
COMMENT ON COLUMN files.content_type IS 'MIME type of the file';
COMMENT ON COLUMN files.size IS 'File size in bytes';
COMMENT ON COLUMN files.path IS 'Storage path relative to the storage root';
COMMENT ON COLUMN files.user_id IS 'User who uploaded the file';
COMMENT ON COLUMN files.uploaded_at IS 'Timestamp when file was uploaded';
COMMENT ON COLUMN files.last_accessed_at IS 'Timestamp when file was last accessed';
COMMENT ON COLUMN files.is_public IS 'Whether the file is publicly accessible without authentication';
