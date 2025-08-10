-- Fix plugin_templates table schema to match code expectations
-- The code expects template_data and is_official columns, not template_code and example_variables

BEGIN TRANSACTION;

-- Create new plugin_templates table with correct schema
CREATE TABLE plugin_templates_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    template_data TEXT NOT NULL,  -- Changed back from template_code
    is_official BOOLEAN NOT NULL DEFAULT 0,  -- Changed back from example_variables
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Copy existing data from old table (mapping columns appropriately)
INSERT INTO plugin_templates_new (
    id, name, description, category, template_data, is_official, created_at, updated_at
)
SELECT 
    id, 
    name, 
    description, 
    category, 
    template_code,  -- Map template_code back to template_data
    0,  -- Set is_official to false since example_variables was text
    created_at,
    updated_at
FROM plugin_templates;

-- Drop old table
DROP TABLE plugin_templates;

-- Rename new table
ALTER TABLE plugin_templates_new RENAME TO plugin_templates;

-- Recreate index
CREATE INDEX idx_plugin_templates_category ON plugin_templates(category);

COMMIT;