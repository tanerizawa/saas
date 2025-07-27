-- Update existing files table to add missing columns
DO $$
BEGIN
    -- Add storage_filename if not exists
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'storage_filename') THEN
        ALTER TABLE files ADD COLUMN storage_filename VARCHAR(255) DEFAULT '';
    END IF;
    
    -- Add file_path if not exists (different from existing path column)
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'file_path') THEN
        ALTER TABLE files ADD COLUMN file_path VARCHAR(1024) DEFAULT '';
    END IF;
    
    -- Add category if not exists
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'category') THEN
        ALTER TABLE files ADD COLUMN category VARCHAR(100) DEFAULT '';
    END IF;
    
    -- Rename size to size_bytes if needed
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'size') AND 
       NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'size_bytes') THEN
        ALTER TABLE files RENAME COLUMN size TO size_bytes;
    END IF;
    
    -- Add alias for uploaded_by to existing user_id
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'uploaded_by') AND
       EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'user_id') THEN
        ALTER TABLE files ADD COLUMN uploaded_by UUID;
        -- Copy data from user_id to uploaded_by
        UPDATE files SET uploaded_by = user_id WHERE uploaded_by IS NULL;
        -- Add foreign key constraint
        ALTER TABLE files ADD CONSTRAINT files_uploaded_by_fkey FOREIGN KEY (uploaded_by) REFERENCES users(id);
    END IF;
END
$$;

-- Add indexes for common queries with existence checks
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'uploaded_by') THEN
        CREATE INDEX IF NOT EXISTS idx_files_uploaded_by ON files(uploaded_by);
    END IF;
    
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'category') THEN
        CREATE INDEX IF NOT EXISTS idx_files_category ON files(category);
    END IF;
    
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'uploaded_at') THEN
        CREATE INDEX IF NOT EXISTS idx_files_uploaded_at ON files(uploaded_at);
    END IF;
    
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'files' AND column_name = 'content_type') THEN
        CREATE INDEX IF NOT EXISTS idx_files_content_type ON files(content_type);
    END IF;
END
$$;

-- Rollback functionality
-- DROP TABLE IF EXISTS files;
