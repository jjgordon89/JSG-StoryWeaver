//! Phase 5 Collaboration and Plugins Migration
//! Adds tables for document sharing, collaboration sessions, comments, plugins, and canvas functionality

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

/// Apply Phase 5 collaboration and plugins migration
pub async fn up(pool: &Pool<Sqlite>) -> Result<()> {
    // Create shared_documents table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shared_documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id TEXT NOT NULL,
            project_id TEXT NOT NULL,
            share_token TEXT UNIQUE NOT NULL,
            share_type TEXT NOT NULL DEFAULT 'read_only',
            password_hash TEXT,
            expires_at DATETIME,
            max_uses INTEGER,
            current_uses INTEGER DEFAULT 0,
            is_active BOOLEAN DEFAULT 1,
            created_by TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create shared_documents table: {}", e)))?;

    // Create collaboration_sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS collaboration_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_token TEXT UNIQUE NOT NULL,
            document_id TEXT NOT NULL,
            project_id TEXT NOT NULL,
            host_user TEXT NOT NULL,
            participants TEXT DEFAULT '[]',
            session_type TEXT NOT NULL DEFAULT 'document_edit',
            is_active BOOLEAN DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create collaboration_sessions table: {}", e)))?;

    // Create document_comments table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS document_comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id TEXT NOT NULL,
            project_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            comment_text TEXT NOT NULL,
            position_data TEXT,
            parent_comment_id INTEGER,
            is_resolved BOOLEAN DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create document_comments table: {}", e)))?;

    // Create plugins table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plugins (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            version TEXT NOT NULL,
            description TEXT,
            author TEXT,
            plugin_type TEXT NOT NULL,
            entry_point TEXT NOT NULL,
            config_schema TEXT,
            permissions TEXT DEFAULT '[]',
            is_enabled BOOLEAN DEFAULT 1,
            is_system BOOLEAN DEFAULT 0,
            installed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugins table: {}", e)))?;

    // Create plugin_marketplace table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plugin_marketplace (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plugin_id TEXT UNIQUE NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            author TEXT,
            version TEXT NOT NULL,
            download_url TEXT NOT NULL,
            homepage_url TEXT,
            repository_url TEXT,
            license TEXT,
            tags TEXT DEFAULT '[]',
            category TEXT,
            download_count INTEGER DEFAULT 0,
            rating REAL DEFAULT 0.0,
            is_verified BOOLEAN DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin_marketplace table: {}", e)))?;

    // Create plugin_ratings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plugin_ratings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plugin_id TEXT NOT NULL,
            user_id TEXT NOT NULL,
            rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
            review_text TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(plugin_id, user_id)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin_ratings table: {}", e)))?;

    // Create plugin_usage_stats table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plugin_usage_stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plugin_name TEXT NOT NULL,
            user_id TEXT NOT NULL,
            usage_count INTEGER DEFAULT 1,
            last_used DATETIME DEFAULT CURRENT_TIMESTAMP,
            total_execution_time INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(plugin_name, user_id)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin_usage_stats table: {}", e)))?;

    // Create plugin_execution_history table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plugin_execution_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plugin_name TEXT NOT NULL,
            user_id TEXT NOT NULL,
            execution_context TEXT,
            input_data TEXT,
            output_data TEXT,
            execution_time INTEGER,
            status TEXT NOT NULL DEFAULT 'success',
            error_message TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin_execution_history table: {}", e)))?;

    // Create plugin_templates table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plugin_templates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            template_type TEXT NOT NULL,
            template_data TEXT NOT NULL,
            category TEXT,
            tags TEXT DEFAULT '[]',
            is_system BOOLEAN DEFAULT 0,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin_templates table: {}", e)))?;

    // Create canvas table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS canvas (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            canvas_data TEXT NOT NULL DEFAULT '{}',
            canvas_type TEXT NOT NULL DEFAULT 'story_map',
            is_active BOOLEAN DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas table: {}", e)))?;

    // Create canvas_elements table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS canvas_elements (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            canvas_id INTEGER NOT NULL,
            element_type TEXT NOT NULL,
            position_x REAL NOT NULL DEFAULT 0,
            position_y REAL NOT NULL DEFAULT 0,
            width REAL DEFAULT 100,
            height REAL DEFAULT 100,
            content TEXT NOT NULL DEFAULT '{}',
            style TEXT,
            z_index INTEGER DEFAULT 0,
            is_locked BOOLEAN DEFAULT 0,
            is_visible BOOLEAN DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas_elements table: {}", e)))?;

    // Create outline_templates table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS outline_templates (
            id TEXT PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            template_type TEXT NOT NULL,
            structure TEXT NOT NULL,
            is_public BOOLEAN DEFAULT 0,
            usage_count INTEGER DEFAULT 0,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create outline_templates table: {}", e)))?;

    // Create canvas_snapshots table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS canvas_snapshots (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            canvas_id INTEGER NOT NULL,
            snapshot_name TEXT NOT NULL,
            snapshot_data TEXT NOT NULL,
            description TEXT,
            created_by TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas_snapshots table: {}", e)))?;

    // Create canvas_collaboration_sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS canvas_collaboration_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            canvas_id INTEGER NOT NULL,
            session_token TEXT UNIQUE NOT NULL,
            host_user TEXT NOT NULL,
            participants TEXT DEFAULT '[]',
            is_active BOOLEAN DEFAULT 1,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME,
            FOREIGN KEY (canvas_id) REFERENCES canvas(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas_collaboration_sessions table: {}", e)))?;

    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_shared_documents_token ON shared_documents(share_token)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create shared_documents index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_token ON collaboration_sessions(session_token)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create collaboration_sessions index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_canvas_elements_canvas_id ON canvas_elements(canvas_id)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas_elements index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_canvas_collaboration_sessions_token ON canvas_collaboration_sessions(session_token)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas_collaboration_sessions index: {}", e)))?;

    // Insert default plugin templates
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO plugin_templates (name, description, template_type, template_data, category, is_system, created_by)
        VALUES 
        ('Basic Story Plugin', 'A basic template for story enhancement plugins', 'story_enhancement', 
         '{"hooks": ["before_save", "after_load"], "permissions": ["read_document", "modify_document"]}', 
         'story', 1, 'system'),
        ('Character Analysis Plugin', 'Template for character analysis and development plugins', 'character_analysis', 
         '{"hooks": ["character_created", "character_updated"], "permissions": ["read_characters", "analyze_text"]}', 
         'character', 1, 'system'),
        ('Plot Structure Plugin', 'Template for plot analysis and structure plugins', 'plot_analysis', 
         '{"hooks": ["document_analyzed"], "permissions": ["read_document", "analyze_structure"]}', 
         'plot', 1, 'system')
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to insert plugin templates: {}", e)))?;

    // Insert default outline templates
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO outline_templates (name, description, template_data, category, is_system, created_by)
        VALUES 
        ('Three-Act Structure', 'Classic three-act story structure template', 
         '{"acts": [{"name": "Act 1: Setup", "scenes": ["Opening", "Inciting Incident", "Plot Point 1"]}, {"name": "Act 2: Confrontation", "scenes": ["Rising Action", "Midpoint", "Plot Point 2"]}, {"name": "Act 3: Resolution", "scenes": ["Climax", "Falling Action", "Resolution"]}]}', 
         'structure', 1, 'system'),
        ('Hero\'s Journey', 'Joseph Campbell\'s monomyth structure template', 
         '{"stages": ["Ordinary World", "Call to Adventure", "Refusal of the Call", "Meeting the Mentor", "Crossing the Threshold", "Tests and Allies", "Approach to the Inmost Cave", "Ordeal", "Reward", "The Road Back", "Resurrection", "Return with the Elixir"]}', 
         'structure', 1, 'system'),
        ('Character Arc Template', 'Template for tracking character development', 
         '{"phases": ["Initial State", "Inciting Incident", "Internal Conflict", "Moment of Truth", "Final State"], "elements": ["Want vs Need", "Ghost/Backstory", "Arc Type"]}', 
         'character', 1, 'system')
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to insert outline templates: {}", e)))?;

    Ok(())
}

/// Rollback Phase 5 collaboration and plugins migration
pub async fn down(pool: &Pool<Sqlite>) -> Result<()> {
    // Drop tables in reverse order to handle foreign key constraints
    let tables = vec![
        "canvas_collaboration_sessions",
        "canvas_snapshots", 
        "outline_templates",
        "canvas_elements",
        "canvas",
        "plugin_templates",
        "plugin_execution_history",
        "plugin_usage_stats",
        "plugin_ratings",
        "plugin_marketplace",
        "plugins",
        "document_comments",
        "collaboration_sessions",
        "shared_documents"
    ];

    for table in tables {
        sqlx::query(&format!("DROP TABLE IF EXISTS {}", table))
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to drop table {}: {}", table, e)))?;
    }

    Ok(())
}