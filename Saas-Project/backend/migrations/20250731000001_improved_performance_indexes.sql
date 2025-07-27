-- Add performance-enhancing indexes to existing tables
-- This migration helps with query optimization for high traffic scenarios

-- User table indexes
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_role ON users(role);
CREATE INDEX IF NOT EXISTS idx_users_status ON users(status);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
CREATE INDEX IF NOT EXISTS idx_users_full_name_gin ON users USING gin(to_tsvector('english', full_name));

-- Company table indexes
CREATE INDEX IF NOT EXISTS idx_companies_name ON companies(name);
CREATE INDEX IF NOT EXISTS idx_companies_created_at ON companies(created_at);
CREATE INDEX IF NOT EXISTS idx_companies_name_gin ON companies USING gin(to_tsvector('english', name));

-- Financial tables indexes
CREATE INDEX IF NOT EXISTS idx_financial_transactions_reference_id ON financial_transactions(reference_id);
CREATE INDEX IF NOT EXISTS idx_financial_transactions_amount ON financial_transactions(amount);
CREATE INDEX IF NOT EXISTS idx_financial_transactions_search ON financial_transactions USING gin(to_tsvector('english', description));

-- Audit logs indexes
CREATE INDEX IF NOT EXISTS idx_audit_logs_entity_id ON audit_logs(entity_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX IF NOT EXISTS idx_audit_logs_action ON audit_logs(action);

-- Foreign key indexes (automatically creating indexes on foreign keys improves join performance)
CREATE INDEX IF NOT EXISTS idx_company_users_user_id ON company_users(user_id);
CREATE INDEX IF NOT EXISTS idx_company_users_company_id ON company_users(company_id);
