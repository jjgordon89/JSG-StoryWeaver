//! Database migrations for StoryWeaver
//! Handles schema creation and updates

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

/// Run all database migrations
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to enable foreign keys: {}", e)))?;
    
    // Create migrations table
    create_migrations_table(pool).await?;
    
    // Run migrations in order
    let migrations: &[(&str, fn(&Pool<Sqlite>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + '_>>)] = &[
        ("001_initial_schema", |pool| Box::pin(migration_001_initial_schema(pool))),
        ("002_story_bible_tables", |pool| Box::pin(migration_002_story_bible_tables(pool))),
        ("003_ai_history_table", |pool| Box::pin(migration_003_ai_history_table(pool))),
        ("004_user_preferences", |pool| Box::pin(migration_004_user_preferences(pool))),
        ("005_full_text_search", |pool| Box::pin(migration_005_full_text_search(pool))),
        ("006_indexes", |pool| Box::pin(migration_006_indexes(pool))),
    ];
    
    for (name, migration_fn) in migrations {
        if !is_migration_applied(pool, name).await? {
            let future = migration_fn(pool);
            future.await?;
            mark_migration_applied(pool, name).await?;
            println!("Applied migration: {}", name);
        }
    }
    
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
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create migrations table: {}", e)))?;
    
    Ok(())
}

/// Check if a migration has been applied
async fn is_migration_applied(pool: &Pool<Sqlite>, name: &str) -> Result<bool> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM migrations WHERE name = ?")
        .bind(name)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check migration status: {}", e)))?;
    
    Ok(count > 0)
}

/// Mark a migration as applied
async fn mark_migration_applied(pool: &Pool<Sqlite>, name: &str) -> Result<()> {
    sqlx::query("INSERT INTO migrations (name) VALUES (?)")
        .bind(name)
        .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create user_preferences table: {}", e)))?;
    
    // Insert default preferences
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO user_preferences (id) VALUES ('default')
        "#,
    )
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
    .execute(pool)
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
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    }
    
    Ok(())
}