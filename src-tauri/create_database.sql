-- StoryWeaver Database Schema Creation Script
-- This script creates all the necessary tables for the StoryWeaver application

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- Create migrations table
CREATE TABLE IF NOT EXISTS migrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Migration 001: Initial schema with projects and documents
CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    genre TEXT,
    target_word_count INTEGER,
    current_word_count INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'planning',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    settings TEXT NOT NULL DEFAULT '{}'
);

CREATE TABLE documents (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    document_type TEXT NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0,
    word_count INTEGER NOT NULL DEFAULT 0,
    parent_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES documents(id) ON DELETE SET NULL
);

-- Migration 002: Story bible tables
CREATE TABLE characters (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    role TEXT NOT NULL,
    age INTEGER,
    appearance TEXT,
    personality TEXT,
    background TEXT,
    goals TEXT,
    relationships TEXT NOT NULL DEFAULT '{}',
    visibility TEXT NOT NULL DEFAULT 'relevant',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE locations (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    location_type TEXT NOT NULL,
    geography TEXT,
    climate TEXT,
    culture TEXT,
    history TEXT,
    significance TEXT,
    visibility TEXT NOT NULL DEFAULT 'relevant',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE timeline_events (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    event_date TEXT,
    event_type TEXT NOT NULL,
    importance INTEGER NOT NULL DEFAULT 1,
    visibility TEXT NOT NULL DEFAULT 'relevant',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Migration 003: AI generation history table
CREATE TABLE ai_generation_history (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    document_id TEXT,
    generation_type TEXT NOT NULL,
    provider TEXT NOT NULL,
    model TEXT NOT NULL,
    prompt TEXT NOT NULL,
    response TEXT NOT NULL,
    token_count INTEGER NOT NULL DEFAULT 0,
    cost_estimate REAL,
    context_used TEXT NOT NULL DEFAULT '{}',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
);

-- Migration 007: Backup, recovery, and versioning tables
CREATE TABLE IF NOT EXISTS backups (
    id TEXT PRIMARY KEY,
    filename TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_auto BOOLEAN NOT NULL DEFAULT 0,
    comment TEXT
);

CREATE TABLE IF NOT EXISTS document_versions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    content TEXT NOT NULL,
    word_count INTEGER NOT NULL DEFAULT 0,
    version_number INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    comment TEXT,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS deleted_items (
    id TEXT PRIMARY KEY,
    item_type TEXT NOT NULL,
    item_id TEXT NOT NULL,
    item_data TEXT NOT NULL,
    parent_id TEXT,
    deletion_reason TEXT,
    deleted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    can_restore BOOLEAN NOT NULL DEFAULT 1
);

-- Migration 008: Phase 4 Advanced AI Features
-- AI Providers table
CREATE TABLE IF NOT EXISTS ai_providers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    api_endpoint TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- AI Model Configurations
CREATE TABLE IF NOT EXISTS ai_model_configurations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    provider_id INTEGER NOT NULL,
    model_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    context_window INTEGER NOT NULL,
    max_output_tokens INTEGER NOT NULL,
    supports_streaming BOOLEAN DEFAULT TRUE,
    supports_images BOOLEAN DEFAULT FALSE,
    cost_per_input_token REAL,
    cost_per_output_token REAL,
    cost_per_image REAL,
    quality_tier TEXT DEFAULT 'standard',
    specializations TEXT, -- JSON
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (provider_id) REFERENCES ai_providers(id)
);

-- Prose Modes
CREATE TABLE IF NOT EXISTS prose_modes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    model_configuration_id INTEGER NOT NULL,
    creativity_level INTEGER DEFAULT 5,
    temperature REAL DEFAULT 0.7,
    top_p REAL DEFAULT 0.9,
    frequency_penalty REAL DEFAULT 0.0,
    presence_penalty REAL DEFAULT 0.0,
    special_instructions TEXT,
    is_experimental BOOLEAN DEFAULT FALSE,
    max_context_words INTEGER DEFAULT 4000,
    max_generation_words INTEGER DEFAULT 2000,
    supports_streaming BOOLEAN DEFAULT TRUE,
    supports_unfiltered BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (model_configuration_id) REFERENCES ai_model_configurations(id)
);

-- Style Examples
CREATE TABLE IF NOT EXISTS style_examples (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    content TEXT NOT NULL,
    word_count INTEGER NOT NULL,
    analysis_result TEXT, -- JSON with style analysis
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Brainstorm Sessions
CREATE TABLE IF NOT EXISTS brainstorm_sessions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    category TEXT NOT NULL,
    seed_prompt TEXT,
    session_data TEXT NOT NULL, -- JSON with ideas and keepers
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Generated Images (Visualize feature)
CREATE TABLE IF NOT EXISTS generated_images (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    source_text TEXT NOT NULL,
    image_prompt TEXT NOT NULL,
    image_data BLOB, -- Base64 encoded image or file path
    image_url TEXT, -- External URL if hosted
    resolution TEXT DEFAULT '1024x1024',
    credits_used INTEGER DEFAULT 2500,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Credit Usage Tracking
CREATE TABLE IF NOT EXISTS credit_usage (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    credits_used INTEGER NOT NULL,
    cost_estimate REAL,
    provider TEXT NOT NULL,
    model TEXT NOT NULL,
    details TEXT, -- JSON with operation details
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Saliency Engine Context Cache
CREATE TABLE IF NOT EXISTS saliency_context_cache (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    context_hash TEXT NOT NULL,
    selected_elements TEXT NOT NULL, -- JSON with selected Story Bible elements
    relevance_scores TEXT NOT NULL, -- JSON with relevance scores
    total_tokens INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Migration 010: AI response cards table
CREATE TABLE IF NOT EXISTS ai_response_cards (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    document_id TEXT,
    feature_type TEXT NOT NULL,
    prompt_context TEXT NOT NULL,
    response_text TEXT NOT NULL,
    model_used TEXT,
    token_count INTEGER,
    cost_estimate REAL,
    is_stacked BOOLEAN NOT NULL DEFAULT 0,
    is_starred BOOLEAN NOT NULL DEFAULT 0,
    is_collapsed BOOLEAN NOT NULL DEFAULT 0,
    stack_position INTEGER,
    tags TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Migration 011: Story bible core tables
CREATE TABLE IF NOT EXISTS story_bible (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL UNIQUE,
    braindump TEXT,
    synopsis TEXT,
    genre TEXT,
    style TEXT,
    pov_mode TEXT NOT NULL DEFAULT 'single',
    global_pov TEXT,
    global_tense TEXT NOT NULL DEFAULT 'past',
    global_character_pov_ids TEXT NOT NULL DEFAULT '[]',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS character_traits (
    id TEXT PRIMARY KEY,
    character_id TEXT NOT NULL,
    trait_name TEXT NOT NULL,
    trait_value TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS folders (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_folder_id TEXT,
    is_series BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_folder_id) REFERENCES folders(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS series (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    folder_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS worldbuilding (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    series_id TEXT,
    name TEXT NOT NULL,
    description TEXT,
    element_type TEXT NOT NULL,
    is_visible BOOLEAN NOT NULL DEFAULT 1,
    original_project_id TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE SET NULL,
    FOREIGN KEY (original_project_id) REFERENCES projects(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS outlines (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    chapter_number INTEGER,
    title TEXT NOT NULL,
    summary TEXT,
    pov TEXT,
    tense TEXT,
    character_pov_ids TEXT NOT NULL DEFAULT '[]',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS outline_acts (
    id TEXT PRIMARY KEY,
    outline_id TEXT NOT NULL,
    act_type TEXT NOT NULL,
    act_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    position INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outline_id) REFERENCES outlines(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS scenes (
    id TEXT PRIMARY KEY,
    outline_id TEXT NOT NULL,
    scene_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    summary TEXT,
    extra_instructions TEXT,
    pov TEXT,
    tense TEXT,
    character_pov_ids TEXT NOT NULL DEFAULT '[]',
    word_count_estimate INTEGER,
    credit_estimate REAL,
    is_validated BOOLEAN NOT NULL DEFAULT 0,
    validation_issues TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outline_id) REFERENCES outlines(id) ON DELETE CASCADE
);

-- Insert default settings
INSERT OR IGNORE INTO settings (key, value) VALUES ('auto_backup_interval', 'daily');
INSERT OR IGNORE INTO settings (key, value) VALUES ('max_auto_backups', '10');
INSERT OR IGNORE INTO settings (key, value) VALUES ('max_document_versions', '20');
INSERT OR IGNORE INTO settings (key, value) VALUES ('auto_version_on_save', 'true');
INSERT OR IGNORE INTO settings (key, value) VALUES ('trash_retention_days', '30');

-- Record applied migrations
INSERT OR IGNORE INTO migrations (name) VALUES ('001_initial_schema');
INSERT OR IGNORE INTO migrations (name) VALUES ('002_story_bible_tables');
INSERT OR IGNORE INTO migrations (name) VALUES ('003_ai_history_table');
INSERT OR IGNORE INTO migrations (name) VALUES ('007_backup_recovery_versioning');
INSERT OR IGNORE INTO migrations (name) VALUES ('010_ai_response_cards');
INSERT OR IGNORE INTO migrations (name) VALUES ('011_story_bible_core');