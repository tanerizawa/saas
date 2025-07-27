-- Rollback for file management tables (Phase 5)

-- Drop trigger
DROP TRIGGER IF EXISTS update_file_access ON files;

-- Drop function
DROP FUNCTION IF EXISTS update_file_access_timestamp();

-- Drop indexes
DROP INDEX IF EXISTS idx_files_uploaded_at;
DROP INDEX IF EXISTS idx_files_user_id;

-- Drop files table
DROP TABLE IF EXISTS files;
