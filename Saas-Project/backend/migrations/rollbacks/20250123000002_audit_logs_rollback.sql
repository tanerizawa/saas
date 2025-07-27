-- Rollback script for migration 20250123000002_audit_logs.sql

-- Drop tables in reverse order to avoid constraint violations
DROP TABLE IF EXISTS audit_logs;
