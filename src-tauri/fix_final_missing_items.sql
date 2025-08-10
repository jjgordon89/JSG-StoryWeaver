-- Add final missing columns and tables
-- These are the last items needed to resolve compilation errors

-- Add missing column to outline_templates table
ALTER TABLE outline_templates ADD COLUMN created_by TEXT;

-- Create missing canvas_operations table
CREATE TABLE IF NOT EXISTS canvas_operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    element_id TEXT,
    data TEXT NOT NULL,
    user_token TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- Create index for canvas_operations
CREATE INDEX IF NOT EXISTS idx_canvas_operations_canvas_id ON canvas_operations(canvas_id);
CREATE INDEX IF NOT EXISTS idx_canvas_operations_timestamp ON canvas_operations(timestamp);

-- Update migrations table
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('fix_final_missing_items', CURRENT_TIMESTAMP);