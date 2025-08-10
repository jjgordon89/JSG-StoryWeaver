-- Fix plugins table by adding missing columns
BEGIN TRANSACTION;

-- Add missing columns to plugins table
ALTER TABLE plugins ADD COLUMN prompt_template TEXT;
ALTER TABLE plugins ADD COLUMN ai_model TEXT DEFAULT 'gpt-3.5-turbo';
ALTER TABLE plugins ADD COLUMN temperature REAL DEFAULT 0.7;
ALTER TABLE plugins ADD COLUMN max_tokens INTEGER;
ALTER TABLE plugins ADD COLUMN stop_sequences TEXT; -- JSON array
ALTER TABLE plugins ADD COLUMN is_multi_stage BOOLEAN DEFAULT 0;
ALTER TABLE plugins ADD COLUMN stage_count INTEGER DEFAULT 1;
ALTER TABLE plugins ADD COLUMN creator_id TEXT;
ALTER TABLE plugins ADD COLUMN is_public BOOLEAN DEFAULT 0;
ALTER TABLE plugins ADD COLUMN code TEXT; -- JavaScript/Lua code (rename from existing)

-- Update existing records with default values
UPDATE plugins SET 
    prompt_template = COALESCE(prompt_template, ''),
    ai_model = COALESCE(ai_model, 'gpt-3.5-turbo'),
    temperature = COALESCE(temperature, 0.7),
    is_multi_stage = COALESCE(is_multi_stage, 0),
    stage_count = COALESCE(stage_count, 1),
    is_public = COALESCE(is_public, 0)
WHERE prompt_template IS NULL OR ai_model IS NULL OR temperature IS NULL;

-- Create missing plugin-related tables

-- Plugin marketplace entries
CREATE TABLE IF NOT EXISTS plugin_marketplace (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    creator_name TEXT NOT NULL,
    visibility TEXT NOT NULL DEFAULT 'private', -- 'published', 'unlisted', 'private'
    download_count INTEGER NOT NULL DEFAULT 0,
    rating_average REAL NOT NULL DEFAULT 0.0,
    rating_count INTEGER NOT NULL DEFAULT 0,
    featured BOOLEAN NOT NULL DEFAULT 0,
    published_at DATETIME,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

-- Plugin ratings and reviews
CREATE TABLE IF NOT EXISTS plugin_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL, -- Anonymous identifier
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE,
    UNIQUE(plugin_id, user_identifier)
);

-- Plugin usage statistics
CREATE TABLE IF NOT EXISTS plugin_usage_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL,
    execution_count INTEGER NOT NULL DEFAULT 0,
    total_credits_used INTEGER NOT NULL DEFAULT 0,
    last_used DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE,
    UNIQUE(plugin_id, user_identifier)
);

-- Plugin execution history
CREATE TABLE IF NOT EXISTS plugin_execution_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL,
    execution_request TEXT NOT NULL, -- JSON string of request
    execution_result TEXT NOT NULL, -- JSON string of result
    credits_used INTEGER NOT NULL DEFAULT 0,
    execution_time_ms INTEGER NOT NULL DEFAULT 0,
    success BOOLEAN NOT NULL DEFAULT 1,
    error_message TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

-- Plugin templates
CREATE TABLE IF NOT EXISTS plugin_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    template_data TEXT NOT NULL, -- JSON string of plugin configuration
    is_official BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_plugin_marketplace_plugin_id ON plugin_marketplace(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_ratings_plugin_id ON plugin_ratings(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_usage_stats_plugin_id ON plugin_usage_stats(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_execution_history_plugin_id ON plugin_execution_history(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_templates_category ON plugin_templates(category);

COMMIT;