-- Add missing columns to outline_templates and canvas_snapshots tables
-- These columns are expected by the Rust code but missing from the database schema

-- Add missing columns to outline_templates table
ALTER TABLE outline_templates ADD COLUMN structure TEXT;
ALTER TABLE outline_templates ADD COLUMN is_public BOOLEAN DEFAULT 0;
ALTER TABLE outline_templates ADD COLUMN usage_count INTEGER DEFAULT 0;

-- Add missing columns to canvas_snapshots table
ALTER TABLE canvas_snapshots ADD COLUMN canvas_data TEXT;

-- Update migrations table
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('fix_remaining_columns', CURRENT_TIMESTAMP);