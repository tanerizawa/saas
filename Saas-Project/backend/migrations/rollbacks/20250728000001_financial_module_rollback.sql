-- Rollback script for migration 20250728000001_financial_module.sql

-- Drop tables in reverse order to avoid constraint violations
DROP TABLE IF EXISTS budget_items;
DROP TABLE IF EXISTS budgets;
DROP TABLE IF EXISTS financial_transactions;
DROP TABLE IF EXISTS transaction_categories;
DROP TABLE IF EXISTS financial_accounts;
