-- Recreate brainstorm_sessions table with correct schema
-- This will fix the corrupted table structure

PRAGMA foreign_keys = OFF;

-- Backup existing data
CREATE TABLE brainstorm_sessions_backup AS 
SELECT id, project_id, category, seed_prompt, session_data, created_at, updated_at
FROM brainstorm_sessions;

-- Drop the corrupted table
DROP TABLE brainstorm_sessions;

-- Recreate with proper schema
CREATE TABLE brainstorm_sessions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    session_name TEXT NOT NULL,
    session_type TEXT NOT NULL, -- "character", "plot", "worldbuilding", "dialogue", "general"
    initial_prompt TEXT NOT NULL,
    context_data TEXT, -- JSON
    generated_ideas TEXT, -- JSON array of ideas
    selected_ideas TEXT, -- JSON array of selected idea IDs
    session_notes TEXT,
    model_used TEXT NOT NULL,
    total_tokens INTEGER,
    cost_credits REAL,
    status TEXT NOT NULL, -- "active", "completed", "archived"
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Restore data with proper mapping
INSERT INTO brainstorm_sessions (
    id, project_id, session_name, session_type, initial_prompt, context_data,
    generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
    cost_credits, status, created_at, updated_at
)
SELECT 
    id,
    project_id,
    COALESCE(category, 'Unnamed Session') as session_name,
    COALESCE(category, 'general') as session_type,
    COALESCE(seed_prompt, '') as initial_prompt,
    '{}' as context_data,
    COALESCE(session_data, '[]') as generated_ideas,
    '[]' as selected_ideas,
    '' as session_notes,
    'unknown' as model_used,
    0 as total_tokens,
    0.0 as cost_credits,
    'completed' as status,
    created_at,
    updated_at
FROM brainstorm_sessions_backup;

-- Drop backup table
DROP TABLE brainstorm_sessions_backup;

PRAGMA foreign_keys = ON;
PRAGMA foreign_key_check;