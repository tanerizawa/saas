-- Rollback for financial module v2 tables (Phase 6)

-- Drop triggers
DROP TRIGGER IF EXISTS update_account_balance_on_transaction ON transactions;

-- Drop functions
DROP FUNCTION IF EXISTS update_account_balance();

-- Drop indexes
DROP INDEX IF EXISTS idx_transactions_user_id;
DROP INDEX IF EXISTS idx_transactions_account_id;
DROP INDEX IF EXISTS idx_transactions_created_at;
DROP INDEX IF EXISTS idx_transactions_status;
DROP INDEX IF EXISTS idx_accounts_user_id;

-- Drop tables
DROP TABLE IF EXISTS transactions;
DROP TABLE IF EXISTS accounts;
