//! Database migrations for StoryWeaver
//! Handles schema creation and updates

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

// Import migrations
mod background_tasks;
mod performance_metrics;
mod phase4_advanced_ai;
mod fix_credit_usage_schema;
mod phase5_collaboration_plugins;
mod add_folder_support;
mod _015_phase6_optimization;

/// Run all database migrations
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to enable foreign keys: {}", e)))?;
    
    // Create migrations table
    create_migrations_table(&*pool).await?;
    
    // Run migrations in order
    let migrations: &[(&str, fn(&Pool<Sqlite>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + '_>>)] = &[
        ("001_initial_schema", |pool| Box::pin(migration_001_initial_schema(&*pool))),
        ("002_story_bible_tables", |pool| Box::pin(migration_002_story_bible_tables(&*pool))),
        ("003_ai_history_table", |pool| Box::pin(migration_003_ai_history_table(&*pool))),
        ("004_user_preferences", |pool| Box::pin(migration_004_user_preferences(&*pool))),
        ("005_full_text_search", |pool| Box::pin(migration_005_full_text_search(&*pool))),
        ("006_indexes", |pool| Box::pin(migration_006_indexes(&*pool))),
        ("007_backup_recovery_versioning", |pool| Box::pin(migration_007_backup_recovery_versioning(&*pool))),
        ("008_background_tasks", |pool| Box::pin(migration_008_background_tasks(&*pool))),
        ("009_performance_metrics", |pool| Box::pin(migration_009_performance_metrics(&*pool))),
        ("010_ai_response_cards", |pool| Box::pin(migration_010_ai_response_cards(&*pool))),
        ("011_story_bible_core", |pool| Box::pin(migration_011_story_bible_core(&*pool))),
        ("012_style_examples", |pool| Box::pin(migration_012_style_examples(&*pool))),
        ("013_character_series_support", |pool| Box::pin(migration_013_character_series_support(&*pool))),
        ("015_phase4_advanced_ai", |pool| Box::pin(phase4_advanced_ai::up(&*pool))),
        ("016_fix_credit_usage_schema", |pool| Box::pin(fix_credit_usage_schema::up(&*pool))),
        ("017_phase5_collaboration_plugins", |pool| Box::pin(phase5_collaboration_plugins::up(&*pool))),
        ("018_add_folder_support", |pool| Box::pin(add_folder_support::up(&*pool))),
        ("019_phase6_optimization", |pool| Box::pin(_015_phase6_optimization::up(&*pool))),
    ];
    
    for (name, migration_fn) in migrations {
        if !is_migration_applied(&*pool, name).await? {
            let future = migration_fn(&*pool);
            future.await?;
            mark_migration_applied(&*pool, name).await?;
            println!("Applied migration: {}", name);
        }
    }
    
    Ok(())
}

async fn migration_012_style_examples(pool: &Pool<Sqlite>) -> Result<()> {
    // Create style_examples table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS style_examples (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            user_id TEXT,
            example_text TEXT NOT NULL,
            analysis_result TEXT,
            generated_style_prompt TEXT,
            word_count INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create style_examples table: {}", e)))?;

    // Create index for style_examples
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_style_examples_project_id ON style_examples(project_id)")
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create style_examples index: {}", e)))?;

    Ok(())
}

/// Migration 013: Add series support to characters
async fn migration_013_character_series_support(pool: &Pool<Sqlite>) -> Result<()> {
    // Add series_id and original_project_id columns to characters table
    sqlx::query(
        r#"
        ALTER TABLE characters ADD COLUMN series_id TEXT
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to add series_id to characters: {}", e)))?;

    sqlx::query(
        r#"
        ALTER TABLE characters ADD COLUMN original_project_id TEXT
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to add original_project_id to characters: {}", e)))?;

    // Add foreign key constraints (SQLite doesn't support adding foreign keys to existing tables,
    // but we can add them as indexes for performance)
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_characters_series_id ON characters(series_id)"
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create characters series_id index: {}", e)))?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_characters_original_project_id ON characters(original_project_id)"
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create characters original_project_id index: {}", e)))?;

    Ok(())
}


/// Migration 010: Create AI response cards table
async fn migration_010_ai_response_cards(pool: &Pool<Sqlite>) -> Result<()> {
    // Create ai_response_cards table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create ai_response_cards table: {}", e)))?;

    // Create indexes for the ai_response_cards table
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_project_id ON ai_response_cards(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_document_id ON ai_response_cards(document_id)",
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_feature_type ON ai_response_cards(feature_type)",
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_is_stacked ON ai_response_cards(is_stacked)",
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_is_starred ON ai_response_cards(is_starred)",
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_created_at ON ai_response_cards(created_at)",
        "CREATE INDEX IF NOT EXISTS idx_ai_cards_stack_position ON ai_response_cards(stack_position)",
    ];

    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create ai_response_cards index: {}", e)))?;
    }

    Ok(())
}

/// Migration 008: Add background tasks table
async fn migration_008_background_tasks(pool: &Pool<Sqlite>) -> Result<()> {
    background_tasks::create_background_tasks_table(&*pool).await?;
    
    // Add default settings for background processing
    let default_settings = [
        ("max_concurrent_tasks", "3"),
        ("max_task_history", "100"),
        ("task_cleanup_days", "7"),
    ];
    
    for (key, value) in default_settings {
        sqlx::query(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?, ?)"
        )
        .bind(key)
        .bind(value)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to insert background task setting: {}", e)))?;
    }
    
    Ok(())
}

/// Migration 009: Add performance metrics tables
async fn migration_009_performance_metrics(pool: &Pool<Sqlite>) -> Result<()> {
    performance_metrics::create_performance_metrics_table(&*pool).await?;
    Ok(())
}

/// Create the migrations tracking table
async fn create_migrations_table(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create migrations table: {}", e)))?;
    
    Ok(())
}

/// Check if a migration has been applied
async fn is_migration_applied(pool: &Pool<Sqlite>, name: &str) -> Result<bool> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM migrations WHERE name = ?")
        .bind(name)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check migration status: {}", e)))?;
    
    Ok(count > 0)
}

/// Mark a migration as applied
async fn mark_migration_applied(pool: &Pool<Sqlite>, name: &str) -> Result<()> {
    sqlx::query("INSERT INTO migrations (name) VALUES (?)")
        .bind(name)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to mark migration as applied: {}", e)))?;
    
    Ok(())
}

/// Migration 001: Initial schema with projects and documents
async fn migration_001_initial_schema(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create projects table: {}", e)))?;
    
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create documents table: {}", e)))?;
    
    Ok(())
}

/// Migration 002: Story bible tables (characters, locations, timeline, plot threads)
async fn migration_002_story_bible_tables(pool: &Pool<Sqlite>) -> Result<()> {
    // Characters table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create characters table: {}", e)))?;
    
    // Locations table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create locations table: {}", e)))?;
    
    // Timeline events table
    sqlx::query(
        r#"
        CREATE TABLE timeline_events (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            event_date TEXT,
            real_date DATETIME,
            importance TEXT NOT NULL DEFAULT 'minor',
            characters_involved TEXT NOT NULL DEFAULT '[]',
            locations_involved TEXT NOT NULL DEFAULT '[]',
            visibility TEXT NOT NULL DEFAULT 'relevant',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create timeline_events table: {}", e)))?;
    
    // Plot threads table
    sqlx::query(
        r#"
        CREATE TABLE plot_threads (
            id TEXT PRIMARY KEY,
            project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'planned',
            priority TEXT NOT NULL DEFAULT 'subplot',
            characters_involved TEXT NOT NULL DEFAULT '[]',
            documents_involved TEXT NOT NULL DEFAULT '[]',
            visibility TEXT NOT NULL DEFAULT 'relevant',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plot_threads table: {}", e)))?;
    
    Ok(())
}

/// Migration 003: AI generation history table
async fn migration_003_ai_history_table(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create ai_generation_history table: {}", e)))?;
    
    Ok(())
}

/// Migration 004: User preferences table
async fn migration_004_user_preferences(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE user_preferences (
            id TEXT PRIMARY KEY DEFAULT 'default',
            ai_provider_preferences TEXT NOT NULL DEFAULT '{}',
            writing_preferences TEXT NOT NULL DEFAULT '{}',
            ui_preferences TEXT NOT NULL DEFAULT '{}',
            plugin_preferences TEXT NOT NULL DEFAULT '{}',
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create user_preferences table: {}", e)))?;
    
    // Insert default preferences
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO user_preferences (id) VALUES ('default')
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to insert default preferences: {}", e)))?;
    
    Ok(())
}

/// Migration 005: Full-text search setup
async fn migration_005_full_text_search(pool: &Pool<Sqlite>) -> Result<()> {
    // Create FTS5 virtual table for documents
    sqlx::query(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(
            title,
            content,
            content='documents',
            content_rowid='rowid'
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create documents FTS table: {}", e)))?;
    
    // Create FTS5 virtual table for story bible elements
    sqlx::query(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS story_bible_fts USING fts5(
            name,
            description,
            content_type,
            content='',
            content_rowid=''
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create story bible FTS table: {}", e)))?;
    
    // Create triggers to keep FTS tables in sync
    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS documents_fts_insert AFTER INSERT ON documents
        BEGIN
            INSERT INTO documents_fts(rowid, title, content) VALUES (new.rowid, new.title, new.content);
        END
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create FTS insert trigger: {}", e)))?;
    
    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS documents_fts_update AFTER UPDATE ON documents
        BEGIN
            UPDATE documents_fts SET title = new.title, content = new.content WHERE rowid = new.rowid;
        END
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create FTS update trigger: {}", e)))?;
    
    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS documents_fts_delete AFTER DELETE ON documents
        BEGIN
            DELETE FROM documents_fts WHERE rowid = old.rowid;
        END
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create FTS delete trigger: {}", e)))?;
    
    Ok(())
}

/// Migration 006: Create indexes for performance
async fn migration_006_indexes(pool: &Pool<Sqlite>) -> Result<()> {
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_documents_project_id ON documents(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_documents_type ON documents(document_type)",
        "CREATE INDEX IF NOT EXISTS idx_documents_parent_id ON documents(parent_id)",
        "CREATE INDEX IF NOT EXISTS idx_characters_project_id ON characters(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_characters_role ON characters(role)",
        "CREATE INDEX IF NOT EXISTS idx_locations_project_id ON locations(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_locations_type ON locations(location_type)",
        "CREATE INDEX IF NOT EXISTS idx_timeline_events_project_id ON timeline_events(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_plot_threads_project_id ON plot_threads(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_ai_history_project_id ON ai_generation_history(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_ai_history_document_id ON ai_generation_history(document_id)",
        "CREATE INDEX IF NOT EXISTS idx_ai_history_type ON ai_generation_history(generation_type)",
        "CREATE INDEX IF NOT EXISTS idx_projects_status ON projects(status)",
        "CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at)",
        "CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at)",
    ];
    
    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    }
    
    Ok(())
}

/// Migration 011: Create story bible core tables
async fn migration_011_story_bible_core(pool: &Pool<Sqlite>) -> Result<()> {
    // Create story_bible table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create story_bible table: {}", e)))?;

    // Create character_traits table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS character_traits (
            id TEXT PRIMARY KEY,
            character_id TEXT NOT NULL,
            trait_name TEXT NOT NULL,
            trait_value TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create character_traits table: {}", e)))?;

    // Create folders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS folders (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_folder_id TEXT,
            is_series BOOLEAN NOT NULL DEFAULT 0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (parent_folder_id) REFERENCES folders(id) ON DELETE SET NULL
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create folders table: {}", e)))?;

    // Create series table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS series (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT,
            folder_id TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE SET NULL
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create series table: {}", e)))?;

    // Create worldbuilding table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create worldbuilding table: {}", e)))?;

    // Create outlines table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create outlines table: {}", e)))?;

    // Create outline_acts table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS outline_acts (
            id TEXT PRIMARY KEY,
            outline_id TEXT NOT NULL,
            act_type TEXT NOT NULL,
            act_number INTEGER NOT NULL,
            title TEXT NOT NULL,
            position INTEGER NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (outline_id) REFERENCES outlines(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create outline_acts table: {}", e)))?;

    // Create scenes table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create scenes table: {}", e)))?;

    // Create indexes for story bible tables
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_story_bible_project_id ON story_bible(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_character_traits_character_id ON character_traits(character_id)",
        "CREATE INDEX IF NOT EXISTS idx_worldbuilding_project_id ON worldbuilding(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_worldbuilding_series_id ON worldbuilding(series_id)",
        "CREATE INDEX IF NOT EXISTS idx_outlines_project_id ON outlines(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_outline_acts_outline_id ON outline_acts(outline_id)",
        "CREATE INDEX IF NOT EXISTS idx_scenes_outline_id ON scenes(outline_id)",
    ];

    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create story bible index: {}", e)))?;
    }

    Ok(())
}

/// Migration 007: Add backup, recovery, and versioning tables
async fn migration_007_backup_recovery_versioning(pool: &Pool<Sqlite>) -> Result<()> {
    // Create backups table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS backups (
            id TEXT PRIMARY KEY,
            filename TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            is_auto BOOLEAN NOT NULL DEFAULT 0,
            comment TEXT
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create backups table: {}", e)))?;
    
    // Create document versions table
    sqlx::query(
        r#"
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
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create document_versions table: {}", e)))?;
    
    // Create deleted items table (trash)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS deleted_items (
            id TEXT PRIMARY KEY,
            item_type TEXT NOT NULL,
            item_id TEXT NOT NULL,
            item_data TEXT NOT NULL,
            parent_id TEXT,
            deletion_reason TEXT,
            deleted_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            can_restore BOOLEAN NOT NULL DEFAULT 1
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create deleted_items table: {}", e)))?;
    
    // Create settings table for backup and recovery settings
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT UNIQUE NOT NULL,
            value TEXT,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create settings table: {}", e)))?;
    
    // Insert default settings
    let default_settings = [
        ("auto_backup_interval", "daily"),
        ("max_auto_backups", "10"),
        ("max_document_versions", "20"),
        ("auto_version_on_save", "true"),
        ("trash_retention_days", "30"),
    ];
    
    for (key, value) in default_settings {
        sqlx::query(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?, ?)"
        )
        .bind(key)
        .bind(value)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to insert default setting: {}", e)))?;
    }
    
    // Create indexes for the new tables
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_document_versions_document_id ON document_versions(document_id)",
        "CREATE INDEX IF NOT EXISTS idx_document_versions_version_number ON document_versions(version_number)",
        "CREATE INDEX IF NOT EXISTS idx_deleted_items_item_type ON deleted_items(item_type)",
        "CREATE INDEX IF NOT EXISTS idx_deleted_items_parent_id ON deleted_items(parent_id)",
        "CREATE INDEX IF NOT EXISTS idx_deleted_items_deleted_at ON deleted_items(deleted_at)",
        "CREATE INDEX IF NOT EXISTS idx_backups_is_auto ON backups(is_auto)",
        "CREATE INDEX IF NOT EXISTS idx_backups_created_at ON backups(created_at)",
    ];
    
    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    }
    
    Ok(())
}
