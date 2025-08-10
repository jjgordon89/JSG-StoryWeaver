-- Migration 012: Database optimizations and improvements
-- Adds NOT NULL constraints where appropriate, creates missing indexes,
-- implements upsert patterns, and enhances error handling

-- Add NOT NULL constraints to critical columns
PRAGMA foreign_keys = ON;

-- For projects table
ALTER TABLE projects 
ADD COLUMN word_count INTEGER NOT NULL DEFAULT 0;

-- For documents table
ALTER TABLE documents 
ADD COLUMN content TEXT NOT NULL DEFAULT '';

-- Create indexes for frequently queried columns
CREATE INDEX IF NOT EXISTS idx_projects_status ON projects (status);
CREATE INDEX IF NOT EXISTS idx_documents_project_id ON documents (project_id);
CREATE INDEX IF NOT EXISTS idx_characters_project_id ON characters (project_id);
CREATE INDEX IF NOT EXISTS idx_ai_generation_history_project_id ON ai_generation_history (project_id);
CREATE INDEX IF NOT EXISTS idx_ai_response_cards_project_id ON ai_response_cards (project_id);

-- Add unique constraints for ai_providers and ai_model_configurations
ALTER TABLE ai_providers
ADD CONSTRAINT unique_provider_name UNIQUE (name);

ALTER TABLE ai_model_configurations
ADD CONSTRAINT unique_model_name UNIQUE (model_name);

-- Implement upsert pattern for settings
CREATE TABLE IF NOT EXISTS settings_upsert (
    key TEXT PRIMARY KEY,
    value TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create trigger to handle upsert for settings
CREATE TRIGGER IF NOT EXISTS upsert_settings 
INSTEAD OF INSERT ON settings_upsert
BEGIN
    INSERT OR REPLACE INTO settings (key, value, updated_at)
    SELECT key, value, datetime('now') 
    FROM settings_upsert;
END;

-- Add check constraints for valid ranges
ALTER TABLE prose_modes
ADD CONSTRAINT valid_creativity_level CHECK (creativity_level BETWEEN 1 AND 10);

ALTER TABLE prose_modes
ADD CONSTRAINT valid_temperature CHECK (temperature BETWEEN 0.0 AND 1.0);

-- Add error handling for constraint violations
CREATE TABLE IF NOT EXISTS constraint_violations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    column_name TEXT,
    violation_type TEXT NOT NULL,
    error_message TEXT,
    occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Create triggers to catch NOT NULL violations
CREATE TRIGGER IF NOT EXISTS catch_null_project_name
BEFORE INSERT ON projects
FOR EACH ROW
WHEN (NEW.name IS NULL)
BEGIN
    INSERT INTO constraint_violations (table_name, column_name, violation_type, error_message)
    VALUES ('projects', 'name', 'NOT NULL', 'Project name cannot be null');
    SELECT RAISE(ABORT, 'Project name cannot be null');
END;

-- Similar triggers can be added for other critical columns...

-- Record this migration
INSERT OR IGNORE INTO migrations (name) VALUES ('012_database_improvements');
