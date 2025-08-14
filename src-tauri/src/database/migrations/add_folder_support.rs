//! Migration to add folder support to projects and documents tables
//! Adds folder_id columns to enable organizing projects and documents in folders

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

/// Apply folder support migration
pub async fn up(pool: &Pool<Sqlite>) -> Result<()> {
    // Add folder_id column to projects table
    sqlx::query(
        r#"
        ALTER TABLE projects ADD COLUMN folder_id TEXT
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to add folder_id to projects table: {}", e)))?;

    // Add folder_id column to documents table
    sqlx::query(
        r#"
        ALTER TABLE documents ADD COLUMN folder_id TEXT
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to add folder_id to documents table: {}", e)))?;

    // Create indexes for better performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_projects_folder_id ON projects(folder_id)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create projects folder_id index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_folder_id ON documents(folder_id)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create documents folder_id index: {}", e)))?;

    Ok(())
}

/// Rollback folder support migration
#[allow(dead_code)]
pub async fn down(pool: &Pool<Sqlite>) -> Result<()> {
    // SQLite doesn't support DROP COLUMN, so we would need to recreate tables
    // For now, we'll just drop the indexes
    sqlx::query("DROP INDEX IF EXISTS idx_projects_folder_id")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to drop projects folder_id index: {}", e)))?;

    sqlx::query("DROP INDEX IF EXISTS idx_documents_folder_id")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to drop documents folder_id index: {}", e)))?;

    Ok(())
}
