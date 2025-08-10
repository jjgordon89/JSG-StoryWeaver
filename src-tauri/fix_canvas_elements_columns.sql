-- Add missing columns to canvas_elements table
-- These columns are expected by the Rust code but missing from the database schema

ALTER TABLE canvas_elements ADD COLUMN title TEXT;
ALTER TABLE canvas_elements ADD COLUMN content TEXT;
ALTER TABLE canvas_elements ADD COLUMN color TEXT;
ALTER TABLE canvas_elements ADD COLUMN metadata TEXT;
ALTER TABLE canvas_elements ADD COLUMN connections TEXT;
ALTER TABLE canvas_elements ADD COLUMN order_index INTEGER DEFAULT 0;

-- Update migrations table
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('fix_canvas_elements_columns', CURRENT_TIMESTAMP);