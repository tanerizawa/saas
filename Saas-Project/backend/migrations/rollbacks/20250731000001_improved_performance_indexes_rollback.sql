-- Rollback script for migration 20250731000001_improved_performance_indexes.sql

-- Drop all indexes created in the migration
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_role;
DROP INDEX IF EXISTS idx_users_status;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_full_name_gin;

DROP INDEX IF EXISTS idx_companies_name;
DROP INDEX IF EXISTS idx_companies_created_at;
DROP INDEX IF EXISTS idx_companies_name_gin;

DROP INDEX IF EXISTS idx_financial_transactions_reference_id;
DROP INDEX IF EXISTS idx_financial_transactions_amount;
DROP INDEX IF EXISTS idx_financial_transactions_search;

DROP INDEX IF EXISTS idx_audit_logs_entity_id;
DROP INDEX IF EXISTS idx_audit_logs_created_at;
DROP INDEX IF EXISTS idx_audit_logs_action;

DROP INDEX IF EXISTS idx_company_users_user_id;
DROP INDEX IF EXISTS idx_company_users_company_id;
