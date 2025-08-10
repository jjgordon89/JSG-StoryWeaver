-- Fix brainstorm_sessions table schema to match code expectations
-- Add missing columns that the Rust code is trying to access

PRAGMA foreign_keys = ON;

-- Add missing columns to brainstorm_sessions table
ALTER TABLE brainstorm_sessions ADD COLUMN session_name TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN session_type TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN initial_prompt TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN context_data TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN generated_ideas TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN selected_ideas TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN session_notes TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN model_used TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN total_tokens INTEGER;
ALTER TABLE brainstorm_sessions ADD COLUMN cost_credits REAL;
ALTER TABLE brainstorm_sessions ADD COLUMN status TEXT;
-- updated_at column already exists, skipping

-- Update existing records to have default values for new columns
UPDATE brainstorm_sessions SET 
    session_name = COALESCE(category, 'Unnamed Session'),
    session_type = COALESCE(category, 'general'),
    initial_prompt = COALESCE(seed_prompt, ''),
    context_data = '{}',
    generated_ideas = COALESCE(session_data, '[]'),
    selected_ideas = '[]',
    session_notes = '',
    model_used = 'unknown',
    total_tokens = 0,
    cost_credits = 0.0,
    status = 'completed',
    updated_at = created_at
WHERE session_name IS NULL;

PRAGMA foreign_key_check;