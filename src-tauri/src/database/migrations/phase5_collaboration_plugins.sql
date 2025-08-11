-- Phase 5: Collaboration & Plugins Migration
-- Adds tables for document sharing, collaboration, plugins, and canvas features

-- Collaboration Tables

-- Shared documents for collaboration
CREATE TABLE IF NOT EXISTS shared_documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    share_token TEXT NOT NULL UNIQUE,
    share_type TEXT NOT NULL DEFAULT 'view', -- 'view', 'comment', 'edit'
    password_hash TEXT,
    expires_at DATETIME,
    max_uses INTEGER,
    current_uses INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Collaboration sessions for real-time editing
CREATE TABLE IF NOT EXISTS collaboration_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    max_participants INTEGER NOT NULL DEFAULT 5,
    current_participants INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Document comments for collaboration
CREATE TABLE IF NOT EXISTS document_comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    parent_comment_id INTEGER,
    author_name TEXT NOT NULL,
    author_identifier TEXT NOT NULL, -- Anonymous identifier
    content TEXT NOT NULL,
    position_start INTEGER,
    position_end INTEGER,
    selected_text TEXT,
    comment_type TEXT NOT NULL DEFAULT 'general', -- 'general', 'suggestion', 'question', 'issue'
    status TEXT NOT NULL DEFAULT 'open', -- 'open', 'resolved', 'dismissed'
    is_resolved BOOLEAN NOT NULL DEFAULT 0,
    resolved_by TEXT,
    resolved_at DATETIME,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id) ON DELETE CASCADE
);

-- Plugin System Tables

-- Core plugins table
CREATE TABLE IF NOT EXISTS plugins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    prompt_template TEXT NOT NULL,
    variables TEXT NOT NULL DEFAULT '[]', -- JSON array of PluginVariable
    ai_model TEXT NOT NULL DEFAULT 'gpt-4',
    temperature REAL NOT NULL DEFAULT 0.7,
    max_tokens INTEGER,
    stop_sequences TEXT, -- JSON array of strings
    category TEXT NOT NULL DEFAULT 'other',
    tags TEXT NOT NULL DEFAULT '[]', -- JSON array of strings
    is_multi_stage BOOLEAN NOT NULL DEFAULT 0,
    stage_count INTEGER NOT NULL DEFAULT 1,
    creator_id TEXT,
    is_public BOOLEAN NOT NULL DEFAULT 0,
    version TEXT NOT NULL DEFAULT '1.0.0',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Plugin marketplace entries
CREATE TABLE IF NOT EXISTS plugin_marketplace_entries (
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
    review TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE,
    UNIQUE(plugin_id, user_identifier)
);

-- Plugin daily statistics (aggregated by date)
CREATE TABLE IF NOT EXISTS plugin_daily_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    date DATE NOT NULL,
    total_executions INTEGER NOT NULL DEFAULT 0,
    successful_executions INTEGER NOT NULL DEFAULT 0,
    failed_executions INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(plugin_id, date)
);

-- Plugin execution history
CREATE TABLE IF NOT EXISTS plugin_execution_history (
    id TEXT PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL,
    execution_request TEXT NOT NULL, -- JSON string
    execution_result TEXT NOT NULL, -- JSON string
    credits_used INTEGER NOT NULL DEFAULT 0,
    execution_time_ms INTEGER NOT NULL DEFAULT 0,
    success BOOLEAN NOT NULL DEFAULT 1,
    error_message TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

-- Plugin templates for common tasks
CREATE TABLE IF NOT EXISTS plugin_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    template_data TEXT NOT NULL, -- JSON string
    is_official BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Canvas System Tables

-- Canvas for visual story planning
CREATE TABLE IF NOT EXISTS canvas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    canvas_data TEXT NOT NULL DEFAULT '{}', -- JSON string
    template_type TEXT,
    width INTEGER NOT NULL DEFAULT 1920,
    height INTEGER NOT NULL DEFAULT 1080,
    zoom_level REAL NOT NULL DEFAULT 1.0,
    viewport_x REAL NOT NULL DEFAULT 0.0,
    viewport_y REAL NOT NULL DEFAULT 0.0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Canvas elements for story planning components
CREATE TABLE IF NOT EXISTS canvas_elements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    element_type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL DEFAULT '',
    position_x REAL NOT NULL DEFAULT 0.0,
    position_y REAL NOT NULL DEFAULT 0.0,
    width REAL NOT NULL DEFAULT 200.0,
    height REAL NOT NULL DEFAULT 150.0,
    color TEXT NOT NULL DEFAULT '#ffffff',
    metadata TEXT NOT NULL DEFAULT '{}', -- JSON string
    connections TEXT NOT NULL DEFAULT '[]', -- JSON array
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- Outline templates for story structures
CREATE TABLE IF NOT EXISTS outline_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    template_type TEXT NOT NULL,
    template_data TEXT NOT NULL, -- JSON string
    is_official BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Canvas snapshots for version control
CREATE TABLE IF NOT EXISTS canvas_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    snapshot_name TEXT NOT NULL,
    canvas_data TEXT NOT NULL, -- JSON string
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- Canvas collaboration sessions
CREATE TABLE IF NOT EXISTS canvas_collaboration_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    max_participants INTEGER NOT NULL DEFAULT 5,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_shared_documents_token ON shared_documents(share_token);
CREATE INDEX IF NOT EXISTS idx_shared_documents_document ON shared_documents(document_id);
CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_token ON collaboration_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_document_comments_document ON document_comments(document_id);
CREATE INDEX IF NOT EXISTS idx_document_comments_parent ON document_comments(parent_comment_id);
CREATE INDEX IF NOT EXISTS idx_plugins_category ON plugins(category);
CREATE INDEX IF NOT EXISTS idx_plugins_public ON plugins(is_public);
CREATE INDEX IF NOT EXISTS idx_plugin_marketplace_visibility ON plugin_marketplace_entries(visibility);
CREATE INDEX IF NOT EXISTS idx_plugin_marketplace_featured ON plugin_marketplace_entries(featured);
CREATE INDEX IF NOT EXISTS idx_plugin_ratings_plugin ON plugin_ratings(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_usage_stats_plugin ON plugin_usage_stats(plugin_id);
CREATE INDEX IF NOT EXISTS idx_canvas_project ON canvas(project_id);
CREATE INDEX IF NOT EXISTS idx_canvas_elements_canvas ON canvas_elements(canvas_id);
CREATE INDEX IF NOT EXISTS idx_canvas_snapshots_canvas ON canvas_snapshots(canvas_id);
CREATE INDEX IF NOT EXISTS idx_canvas_collaboration_token ON canvas_collaboration_sessions(session_token);

-- Insert some default plugin templates
INSERT OR IGNORE INTO plugin_templates (name, description, category, template_data, is_official) VALUES
('Character Description Generator', 'Generate detailed character descriptions based on basic traits', 'writing', '{"prompt_template": "Create a detailed character description for {{character_name}}, a {{age}}-year-old {{role}} who is {{personality_traits}}. Include physical appearance, mannerisms, and background.", "variables": [{"name": "character_name", "display_name": "Character Name", "description": "The name of the character", "variable_type": "Text", "required": true}, {"name": "age", "display_name": "Age", "description": "Character age", "variable_type": "Number", "required": false}, {"name": "role", "display_name": "Role", "description": "Character role in story", "variable_type": "Select", "options": ["protagonist", "antagonist", "supporting character", "minor character"], "required": true}, {"name": "personality_traits", "display_name": "Personality Traits", "description": "Key personality traits", "variable_type": "TextArea", "required": true}]}', 1),
('Scene Expansion', 'Expand a scene outline into full prose', 'writing', '{"prompt_template": "Expand this scene outline into a full scene: {{scene_outline}}\n\nWrite in {{tense}} tense from {{pov}} perspective. Target length: {{target_length}} words.", "variables": [{"name": "scene_outline", "display_name": "Scene Outline", "description": "Brief outline of the scene", "variable_type": "TextArea", "required": true}, {"name": "tense", "display_name": "Tense", "description": "Narrative tense", "variable_type": "Select", "options": ["past", "present", "future"], "required": true}, {"name": "pov", "display_name": "Point of View", "description": "Narrative perspective", "variable_type": "Select", "options": ["first person", "second person", "third person limited", "third person omniscient"], "required": true}, {"name": "target_length", "display_name": "Target Length", "description": "Approximate word count", "variable_type": "Number", "required": false, "default_value": "500"}]}', 1),
('Dialogue Polish', 'Improve dialogue to sound more natural and character-specific', 'editing', '{"prompt_template": "Polish this dialogue to make it more natural and character-specific:\n\n{{dialogue_text}}\n\nCharacter context: {{character_context}}", "variables": [{"name": "dialogue_text", "display_name": "Dialogue Text", "description": "The dialogue to improve", "variable_type": "TextArea", "required": true}, {"name": "character_context", "display_name": "Character Context", "description": "Information about the characters speaking", "variable_type": "TextArea", "required": false}]}', 1);

-- Insert some default outline templates
INSERT OR IGNORE INTO outline_templates (name, description, template_type, template_data, is_official) VALUES
('Hero\'s Journey', 'Classic monomyth structure with 17 stages', 'heros_journey', '{"stages": [{"name": "Ordinary World", "description": "Hero\'s normal life before transformation"}, {"name": "Call to Adventure", "description": "Hero is presented with a problem or challenge"}, {"name": "Refusal of the Call", "description": "Hero hesitates or refuses the adventure"}, {"name": "Meeting the Mentor", "description": "Hero encounters wise figure who gives advice"}, {"name": "Crossing the Threshold", "description": "Hero commits to the adventure"}, {"name": "Tests, Allies, and Enemies", "description": "Hero faces challenges and makes allies"}, {"name": "Approach to the Inmost Cave", "description": "Hero prepares for major challenge"}, {"name": "The Ordeal", "description": "Hero faces greatest fear or most difficult challenge"}, {"name": "Reward", "description": "Hero survives and gains something"}, {"name": "The Road Back", "description": "Hero begins journey back to ordinary world"}, {"name": "Resurrection", "description": "Final test where hero is purified or transformed"}, {"name": "Return with the Elixir", "description": "Hero returns home with wisdom or power to help others"}]}', 1),
('Three-Act Structure', 'Classic three-act dramatic structure', 'three_act', '{"acts": [{"name": "Act I - Setup", "description": "Introduce characters, world, and inciting incident", "percentage": 25}, {"name": "Act II - Confrontation", "description": "Rising action, obstacles, and character development", "percentage": 50}, {"name": "Act III - Resolution", "description": "Climax and resolution of conflicts", "percentage": 25}]}', 1),
('Romance Arc', 'Standard romance story structure', 'romance_outline', '{"beats": [{"name": "Meet Cute", "description": "Initial meeting between romantic leads"}, {"name": "Attraction", "description": "Characters are drawn to each other"}, {"name": "Barrier", "description": "Obstacle preventing relationship"}, {"name": "Getting to Know You", "description": "Characters learn about each other"}, {"name": "Relationship Deepens", "description": "Emotional and physical intimacy grows"}, {"name": "Crisis", "description": "Major conflict threatens relationship"}, {"name": "Dark Moment", "description": "All seems lost"}, {"name": "Resolution", "description": "Conflict resolved, love declared"}, {"name": "Happily Ever After", "description": "Future happiness implied or shown"}]}', 1);
