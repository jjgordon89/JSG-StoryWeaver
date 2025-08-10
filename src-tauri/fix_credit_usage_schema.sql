-- Fix credit_usage table schema to match the expected columns
-- The table has 'model' but code expects 'model_used'
-- Also missing several other columns

PRAGMA foreign_keys = OFF;

-- Backup existing data
CREATE TABLE credit_usage_backup AS 
SELECT id, project_id, operation_type, credits_used, cost_estimate, provider, model, details, created_at
FROM credit_usage;

-- Drop the existing table
DROP TABLE credit_usage;

-- Recreate with proper schema matching the code expectations
CREATE TABLE credit_usage (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    operation_type TEXT NOT NULL, -- "text_generation", "image_generation", "brainstorming", "style_analysis"
    model_used TEXT NOT NULL,
    tokens_used INTEGER,
    credits_consumed REAL NOT NULL,
    operation_details TEXT, -- JSON
    session_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Restore data with proper column mapping
INSERT INTO credit_usage (
    id, project_id, operation_type, model_used, tokens_used, credits_consumed,
    operation_details, session_id, created_at
)
SELECT 
    id,
    CAST(project_id AS INTEGER) as project_id,
    operation_type,
    model as model_used,
    NULL as tokens_used, -- This column didn't exist before
    COALESCE(credits_used, cost_estimate, 0.0) as credits_consumed,
    details as operation_details,
    NULL as session_id, -- This column didn't exist before
    created_at
FROM credit_usage_backup;

-- Drop backup table
DROP TABLE credit_usage_backup;

PRAGMA foreign_keys = ON;
PRAGMA foreign_key_check;