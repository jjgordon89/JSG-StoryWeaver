//! Migration for background tasks table
//! Adds support for task queue and history

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

/// Create background tasks table
pub async fn create_background_tasks_table(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS background_tasks (
            id TEXT PRIMARY KEY,
            task_type TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL,
            priority INTEGER NOT NULL DEFAULT 1,
            progress REAL NOT NULL DEFAULT 0.0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            started_at DATETIME,
            completed_at DATETIME,
            error_message TEXT,
            user_initiated BOOLEAN NOT NULL DEFAULT 0,
            project_id TEXT,
            document_id TEXT,
            metadata TEXT NOT NULL DEFAULT '{}',
            FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
            FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create background_tasks table: {}", e)))?;
    
    // Create indexes
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_background_tasks_status ON background_tasks(status)",
        "CREATE INDEX IF NOT EXISTS idx_background_tasks_priority ON background_tasks(priority)",
        "CREATE INDEX IF NOT EXISTS idx_background_tasks_project_id ON background_tasks(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_background_tasks_document_id ON background_tasks(document_id)",
        "CREATE INDEX IF NOT EXISTS idx_background_tasks_created_at ON background_tasks(created_at)",
        "CREATE INDEX IF NOT EXISTS idx_background_tasks_completed_at ON background_tasks(completed_at)",
    ];
    
    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    }
    
    Ok(())
}
