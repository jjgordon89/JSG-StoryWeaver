//! Migration 014: Create document_links table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

pub async fn up(pool: &Pool<Sqlite>) -> Result<()> {
    // Create table SQL
    let create_sql = r#"
        CREATE TABLE IF NOT EXISTS document_links (
            id TEXT PRIMARY KEY,
            from_document_id TEXT NOT NULL,
            to_document_id TEXT NOT NULL,
            link_order INTEGER NOT NULL DEFAULT 1,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (from_document_id) REFERENCES documents(id) ON DELETE CASCADE,
            FOREIGN KEY (to_document_id) REFERENCES documents(id) ON DELETE CASCADE
        )
    "#;

    println!("Applying migration 014 SQL (create table):\n{}", create_sql);

    sqlx::query(create_sql)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create document_links table: {}", e)))?;

    // Indexes to support lookups
    let idx_from = "CREATE INDEX IF NOT EXISTS idx_document_links_from ON document_links(from_document_id);";
    println!("Applying migration 014 SQL (index from):\n{}", idx_from);
    sqlx::query(idx_from)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create idx_document_links_from: {}", e)))?;

    let idx_to = "CREATE INDEX IF NOT EXISTS idx_document_links_to ON document_links(to_document_id);";
    println!("Applying migration 014 SQL (index to):\n{}", idx_to);
    sqlx::query(idx_to)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create idx_document_links_to: {}", e)))?;

    Ok(())
}
