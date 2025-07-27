-- Rollback script for migration 20250730000001_consolidated_financial.sql

-- Drop tables in reverse order to avoid constraint violations
DROP TABLE IF EXISTS budget_items;
DROP TABLE IF EXISTS budgets;
DROP TABLE IF EXISTS payment_attempts;
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS invoice_items;
DROP TABLE IF EXISTS invoices;
DROP TABLE IF EXISTS financial_reports;
DROP TABLE IF EXISTS financial_transactions;
DROP TABLE IF EXISTS transaction_categories;
DROP TABLE IF EXISTS financial_accounts;
