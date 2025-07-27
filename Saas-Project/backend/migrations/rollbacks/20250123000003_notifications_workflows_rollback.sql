-- Rollback script for migration 20250123000003_notifications_workflows.sql

-- Drop tables in reverse order to avoid constraint violations
DROP TABLE IF EXISTS workflow_logs;
DROP TABLE IF EXISTS workflow_executions;
DROP TABLE IF EXISTS workflow_triggers;
DROP TABLE IF EXISTS workflows;
DROP TABLE IF EXISTS notification_templates;
DROP TABLE IF EXISTS notification_settings;
DROP TABLE IF EXISTS notifications;
