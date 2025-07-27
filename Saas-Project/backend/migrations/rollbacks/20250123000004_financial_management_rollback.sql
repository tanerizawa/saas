-- Rollback script for migration 20250123000004_financial_management.sql

-- Drop tables in reverse order to avoid constraint violations
DROP TABLE IF EXISTS payment_attempts;
DROP TABLE IF EXISTS payments;
DROP TABLE IF EXISTS invoice_items;
DROP TABLE IF EXISTS invoices;
DROP TABLE IF EXISTS financial_reports;
