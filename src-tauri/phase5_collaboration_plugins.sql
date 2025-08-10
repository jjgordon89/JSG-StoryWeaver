-- Phase 5: Collaboration & Plugin System Migration
-- StoryWeaver Database Schema Extension

-- =====================================================
-- COLLABORATION FEATURES
-- =====================================================

-- Shared Documents Table
CREATE TABLE IF NOT EXISTS shared_documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id INTEGER NOT NULL,
    share_token TEXT UNIQUE NOT NULL,
    share_settings TEXT NOT NULL, -- JSON: {"allow_comments": true, "allow_editing": false, "expires_at": "2024-12-31T23:59:59Z"}
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    created_by TEXT,
    is_active BOOLEAN DEFAULT 1,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Collaboration Sessions Table
CREATE TABLE IF NOT EXISTS collaboration_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT UNIQUE NOT NULL,
    document_id INTEGER NOT NULL,
    host_user TEXT NOT NULL,
    participants TEXT, -- JSON array of participant info
    session_data TEXT, -- JSON: cursor positions, selections, etc.
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_activity DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT 1,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Document Comments Table
CREATE TABLE IF NOT EXISTS document_comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id INTEGER NOT NULL,
    thread_id TEXT, -- For grouping related comments
    parent_comment_id INTEGER, -- For replies
    author_name TEXT NOT NULL,
    author_email TEXT,
    content TEXT NOT NULL,
    position_data TEXT, -- JSON: {"start": 100, "end": 150, "line": 5}
    status TEXT DEFAULT 'open', -- 'open', 'resolved', 'deleted'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    resolved_at DATETIME,
    resolved_by TEXT,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id) ON DELETE CASCADE
);

-- =====================================================
-- PLUGIN SYSTEM
-- =====================================================

-- Plugins Table
CREATE TABLE IF NOT EXISTS plugins (
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
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name, version)
);

-- Plugin Marketplace Table
CREATE TABLE IF NOT EXISTS plugin_marketplace (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    short_description TEXT,
    long_description TEXT,
    screenshots TEXT, -- JSON array of screenshot URLs/data
    download_url TEXT,
    homepage_url TEXT,
    documentation_url TEXT,
    license TEXT,
    price DECIMAL(10,2) DEFAULT 0.00,
    is_featured BOOLEAN DEFAULT 0,
    is_verified BOOLEAN DEFAULT 0,
    visibility TEXT DEFAULT 'public', -- 'public', 'private', 'unlisted'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

-- Plugin Ratings Table
CREATE TABLE IF NOT EXISTS plugin_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    user_id TEXT NOT NULL,
    rating INTEGER CHECK (rating >= 1 AND rating <= 5),
    review TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE,
    UNIQUE(plugin_id, user_id)
);

-- Plugin Usage Statistics Table
CREATE TABLE IF NOT EXISTS plugin_usage_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    execution_count INTEGER DEFAULT 0,
    total_execution_time INTEGER DEFAULT 0, -- in milliseconds
    last_used DATETIME,
    error_count INTEGER DEFAULT 0,
    success_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

-- Plugin Execution History Table
CREATE TABLE IF NOT EXISTS plugin_execution_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    project_id INTEGER,
    document_id INTEGER,
    input_data TEXT, -- JSON: input parameters
    output_data TEXT, -- JSON: execution results
    execution_time INTEGER, -- in milliseconds
    status TEXT, -- 'success', 'error', 'timeout'
    error_message TEXT,
    executed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
);

-- Plugin Templates Table
CREATE TABLE IF NOT EXISTS plugin_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    template_code TEXT NOT NULL,
    variables_schema TEXT, -- JSON schema for variables
    example_usage TEXT,
    is_builtin BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- =====================================================
-- VISUAL STORY PLANNING (CANVAS)
-- =====================================================

-- Canvas Table
CREATE TABLE IF NOT EXISTS canvas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    canvas_data TEXT, -- JSON: canvas configuration, zoom, pan, etc.
    background_color TEXT DEFAULT '#ffffff',
    grid_enabled BOOLEAN DEFAULT 1,
    grid_size INTEGER DEFAULT 20,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Canvas Elements Table
CREATE TABLE IF NOT EXISTS canvas_elements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    element_type TEXT NOT NULL, -- 'character', 'location', 'plot_point', 'note', 'connection'
    element_data TEXT NOT NULL, -- JSON: element-specific data
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    width REAL DEFAULT 100,
    height REAL DEFAULT 100,
    z_index INTEGER DEFAULT 0,
    style_data TEXT, -- JSON: colors, fonts, borders, etc.
    is_locked BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- Outline Templates Table
CREATE TABLE IF NOT EXISTS outline_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    template_type TEXT, -- 'three_act', 'heros_journey', 'save_the_cat', 'custom'
    structure_data TEXT NOT NULL, -- JSON: template structure
    is_builtin BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Canvas Snapshots Table
CREATE TABLE IF NOT EXISTS canvas_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    snapshot_data TEXT NOT NULL, -- JSON: complete canvas state
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- Canvas Collaboration Sessions Table
CREATE TABLE IF NOT EXISTS canvas_collaboration_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    canvas_id INTEGER NOT NULL,
    session_id TEXT UNIQUE NOT NULL,
    host_user TEXT NOT NULL,
    participants TEXT, -- JSON array of participants
    session_data TEXT, -- JSON: real-time collaboration data
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_activity DATETIME DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT 1,
    FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
);

-- =====================================================
-- INDEXES FOR PERFORMANCE
-- =====================================================

-- Collaboration indexes
CREATE INDEX IF NOT EXISTS idx_shared_documents_token ON shared_documents(share_token);
CREATE INDEX IF NOT EXISTS idx_shared_documents_document ON shared_documents(document_id);
CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_document ON collaboration_sessions(document_id);
CREATE INDEX IF NOT EXISTS idx_document_comments_document ON document_comments(document_id);
CREATE INDEX IF NOT EXISTS idx_document_comments_thread ON document_comments(thread_id);

-- Plugin indexes
CREATE INDEX IF NOT EXISTS idx_plugins_category ON plugins(category);
CREATE INDEX IF NOT EXISTS idx_plugins_marketplace ON plugins(marketplace_id);
CREATE INDEX IF NOT EXISTS idx_plugin_ratings_plugin ON plugin_ratings(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_usage_plugin ON plugin_usage_stats(plugin_id);
CREATE INDEX IF NOT EXISTS idx_plugin_execution_plugin ON plugin_execution_history(plugin_id);

-- Canvas indexes
CREATE INDEX IF NOT EXISTS idx_canvas_project ON canvas(project_id);
CREATE INDEX IF NOT EXISTS idx_canvas_elements_canvas ON canvas_elements(canvas_id);
CREATE INDEX IF NOT EXISTS idx_canvas_snapshots_canvas ON canvas_snapshots(canvas_id);
CREATE INDEX IF NOT EXISTS idx_canvas_collaboration_canvas ON canvas_collaboration_sessions(canvas_id);

-- =====================================================
-- DEFAULT DATA
-- =====================================================

-- Insert default plugin templates
INSERT OR IGNORE INTO plugin_templates (name, description, category, template_code, variables_schema, example_usage, is_builtin) VALUES
('Basic Text Processor', 'A simple template for text processing plugins', 'writing_assistant', 
'function execute(input, variables) {
    // Your plugin logic here
    const text = input.selectedText || input.documentText;
    const processedText = text.toUpperCase(); // Example transformation
    return {
        success: true,
        result: processedText,
        message: "Text processed successfully"
    };
}', 
'{"type": "object", "properties": {"option1": {"type": "string", "default": "value1"}}}',
'Use this template to create plugins that process selected text or entire documents.',
1),

('Document Analyzer', 'Template for analyzing document content', 'analysis',
'function execute(input, variables) {
    const text = input.documentText;
    const wordCount = text.split(/\s+/).length;
    const charCount = text.length;
    const paragraphs = text.split(/\n\s*\n/).length;
    
    return {
        success: true,
        result: {
            wordCount,
            charCount,
            paragraphs,
            readingTime: Math.ceil(wordCount / 200) // minutes
        },
        message: "Document analyzed successfully"
    };
}',
'{"type": "object", "properties": {"includeReadingTime": {"type": "boolean", "default": true}}}',
'Analyze document statistics like word count, character count, and reading time.',
1),

('Export Formatter', 'Template for custom export formats', 'export',
'function execute(input, variables) {
    const document = input.document;
    const format = variables.format || "txt";
    
    let output = "";
    switch(format) {
        case "markdown":
            output = `# ${document.title}\n\n${document.content}`;
            break;
        case "html":
            output = `<h1>${document.title}</h1>\n<p>${document.content.replace(/\n/g, "</p>\n<p>")}</p>`;
            break;
        default:
            output = `${document.title}\n\n${document.content}`;
    }
    
    return {
        success: true,
        result: output,
        message: `Document exported as ${format}`
    };
}',
'{"type": "object", "properties": {"format": {"type": "string", "enum": ["txt", "markdown", "html"], "default": "txt"}}}',
'Create custom export formats for your documents.',
1);

-- Insert default outline templates
INSERT OR IGNORE INTO outline_templates (name, description, template_type, structure_data, is_builtin) VALUES
('Three-Act Structure', 'Classic three-act story structure', 'three_act',
'{
  "acts": [
    {
      "name": "Act I - Setup",
      "description": "Introduce characters, world, and inciting incident",
      "percentage": 25,
      "beats": [
        {"name": "Opening Image", "description": "First impression of the story"},
        {"name": "Inciting Incident", "description": "Event that starts the story"},
        {"name": "Plot Point 1", "description": "End of beginning, commitment to journey"}
      ]
    },
    {
      "name": "Act II - Confrontation",
      "description": "Rising action, obstacles, and character development",
      "percentage": 50,
      "beats": [
        {"name": "First Pinch Point", "description": "Reminder of antagonistic force"},
        {"name": "Midpoint", "description": "Major revelation or shift"},
        {"name": "Second Pinch Point", "description": "Antagonist strikes back"},
        {"name": "Plot Point 2", "description": "End of middle, final push"}
      ]
    },
    {
      "name": "Act III - Resolution",
      "description": "Climax and resolution of conflicts",
      "percentage": 25,
      "beats": [
        {"name": "Climax", "description": "Final confrontation"},
        {"name": "Falling Action", "description": "Immediate aftermath"},
        {"name": "Resolution", "description": "New normal, final image"}
      ]
    }
  ]
}',
1),

('Hero''s Journey', 'Joseph Campbell''s monomyth structure', 'heros_journey',
'{
  "stages": [
    {"name": "Ordinary World", "description": "Hero''s normal life before transformation"},
    {"name": "Call to Adventure", "description": "Hero faces a problem or challenge"},
    {"name": "Refusal of the Call", "description": "Hero hesitates or refuses the adventure"},
    {"name": "Meeting the Mentor", "description": "Hero encounters wise figure"},
    {"name": "Crossing the Threshold", "description": "Hero commits to the adventure"},
    {"name": "Tests, Allies, Enemies", "description": "Hero faces challenges and makes allies"},
    {"name": "Approach to the Inmost Cave", "description": "Hero prepares for major challenge"},
    {"name": "Ordeal", "description": "Hero faces greatest fear or danger"},
    {"name": "Reward", "description": "Hero survives and gains something"},
    {"name": "The Road Back", "description": "Hero begins journey back to ordinary world"},
    {"name": "Resurrection", "description": "Final test, hero is transformed"},
    {"name": "Return with the Elixir", "description": "Hero returns home changed"}
  ]
}',
1),

('Save the Cat Beat Sheet', 'Blake Snyder''s story structure', 'save_the_cat',
'{
  "beats": [
    {"name": "Opening Image", "description": "Snapshot of hero''s life before change", "page": 1},
    {"name": "Theme Stated", "description": "What the story is about", "page": 5},
    {"name": "Set-Up", "description": "Introduce hero, stakes, and goal", "page": "1-10"},
    {"name": "Catalyst", "description": "Life-changing event", "page": 12},
    {"name": "Debate", "description": "Should hero go on this journey?", "page": "12-25"},
    {"name": "Break into Two", "description": "Hero decides to act", "page": 25},
    {"name": "B Story", "description": "Subplot, usually love story", "page": 30},
    {"name": "Fun and Games", "description": "Promise of the premise", "page": "30-55"},
    {"name": "Midpoint", "description": "False victory or defeat", "page": 55},
    {"name": "Bad Guys Close In", "description": "Complications arise", "page": "55-75"},
    {"name": "All Is Lost", "description": "Hero''s lowest point", "page": 75},
    {"name": "Dark Night of the Soul", "description": "Hero wallows in defeat", "page": "75-85"},
    {"name": "Break into Three", "description": "Hero finds solution", "page": 85},
    {"name": "Finale", "description": "Climax and resolution", "page": "85-110"},
    {"name": "Final Image", "description": "Opposite of opening image", "page": 110}
  ]
}',
1);

-- Update migrations table
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('phase5_collaboration_plugins', CURRENT_TIMESTAMP);