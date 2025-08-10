-- Fix plugins table by recreating it with proper schema
BEGIN TRANSACTION;

-- Create new plugins table with correct schema
CREATE TABLE plugins_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT,
    author TEXT,
    category TEXT, -- 'writing_assistant', 'formatting', 'export', 'analysis', 'integration'
    tags TEXT, -- JSON array of tags
    code TEXT NOT NULL, -- JavaScript/Lua code
    variables TEXT, -- JSON: plugin configuration variables
    permissions TEXT, -- JSON: required permissions
    icon_data TEXT, -- Base64 encoded icon or SVG
    is_enabled BOOLEAN DEFAULT 1,
    is_marketplace BOOLEAN DEFAULT 0,
    marketplace_id TEXT UNIQUE,
    install_count INTEGER DEFAULT 0,
    download_count INTEGER DEFAULT 0,
    rating_average REAL DEFAULT 0.0,
    rating_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    prompt_template TEXT,
    ai_model TEXT DEFAULT 'gpt-3.5-turbo',
    temperature REAL DEFAULT 0.7,
    max_tokens INTEGER,
    stop_sequences TEXT,
    is_multi_stage BOOLEAN DEFAULT 0,
    stage_count INTEGER DEFAULT 1,
    creator_id TEXT,
    is_public BOOLEAN DEFAULT 0,
    UNIQUE(name, version)
);

-- Copy data from old table, handling the corrupted schema
INSERT INTO plugins_new (
    id, name, version, description, author, category, tags, code, variables, 
    permissions, icon_data, is_enabled, is_marketplace, marketplace_id, 
    install_count, created_at, updated_at
)
SELECT 
    id, name, version, description, author, category, tags, code, variables,
    permissions, icon_data, is_enabled, is_marketplace, marketplace_id,
    install_count, created_at, updated_at
FROM plugins;

-- Drop old table and rename new one
DROP TABLE plugins;
ALTER TABLE plugins_new RENAME TO plugins;

-- Recreate indexes
CREATE INDEX idx_plugins_category ON plugins(category);
CREATE INDEX idx_plugins_marketplace ON plugins(marketplace_id);
CREATE INDEX idx_plugins_rating ON plugins(rating_average);
CREATE INDEX idx_plugins_downloads ON plugins(download_count);

COMMIT;