-- Fix missing columns in database schema
-- This script adds missing columns that are causing compilation errors

-- Add missing columns to plugins table
ALTER TABLE plugins ADD COLUMN is_active BOOLEAN DEFAULT 1;

-- Add missing columns to plugin_templates table  
ALTER TABLE plugin_templates ADD COLUMN template_data TEXT;
ALTER TABLE plugin_templates ADD COLUMN is_official BOOLEAN DEFAULT 0;
ALTER TABLE plugin_templates ADD COLUMN variables_schema TEXT;
ALTER TABLE plugin_templates ADD COLUMN example_usage TEXT;
ALTER TABLE plugin_templates ADD COLUMN is_builtin BOOLEAN DEFAULT 0;

-- Add missing columns to canvas table
ALTER TABLE canvas ADD COLUMN template_type TEXT;

-- Update migrations table
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('fix_missing_columns', CURRENT_TIMESTAMP);