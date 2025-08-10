-- Add missing columns to canvas table
-- These columns are expected by the Rust code but missing from the database schema

ALTER TABLE canvas ADD COLUMN width INTEGER DEFAULT 800;
ALTER TABLE canvas ADD COLUMN height INTEGER DEFAULT 600;
ALTER TABLE canvas ADD COLUMN zoom_level REAL DEFAULT 1.0;
ALTER TABLE canvas ADD COLUMN viewport_x REAL DEFAULT 0.0;
ALTER TABLE canvas ADD COLUMN viewport_y REAL DEFAULT 0.0;

-- Update migrations table
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('fix_canvas_columns', CURRENT_TIMESTAMP);