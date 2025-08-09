//! Migration to fix credit_usage table schema to match the code expectations

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

/// Fix credit_usage table schema to match code expectations
pub async fn up(pool: &Pool<Sqlite>) -> Result<()> {
    // First, check if the table exists and what columns it has
    let table_info = sqlx::query!("PRAGMA table_info(credit_usage)")
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get table info: {}", e)))?;

    // Check if we need to migrate from old schema
    let has_feature_name = table_info.iter().any(|row| row.name == "feature_name");
    let has_operation_type = table_info.iter().any(|row| row.name == "operation_type");
    
    if has_feature_name && !has_operation_type {
        // We have the old schema, need to migrate
        
        // Create a temporary table with the new schema
        sqlx::query(
            r#"
            CREATE TABLE credit_usage_new (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                operation_type TEXT NOT NULL,
                model_used TEXT,
                tokens_used INTEGER,
                credits_consumed REAL,
                operation_details TEXT,
                session_id TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create new credit_usage table: {}", e)))?;

        // Copy data from old table to new table, mapping columns
        sqlx::query(
            r#"
            INSERT INTO credit_usage_new (
                id, project_id, operation_type, model_used, tokens_used, 
                credits_consumed, created_at
            )
            SELECT 
                id, 
                project_id, 
                COALESCE(feature_name, 'unknown') as operation_type,
                model_used,
                COALESCE(tokens_input + tokens_output, 0) as tokens_used,
                COALESCE(credits_used, 0.0) as credits_consumed,
                created_at
            FROM credit_usage
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to copy data to new table: {}", e)))?;

        // Drop the old table
        sqlx::query("DROP TABLE credit_usage")
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to drop old table: {}", e)))?;

        // Rename the new table
        sqlx::query("ALTER TABLE credit_usage_new RENAME TO credit_usage")
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to rename new table: {}", e)))?;
            
    } else if !has_operation_type {
        // Table doesn't exist or has completely different schema, create new one
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS credit_usage (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                operation_type TEXT NOT NULL,
                model_used TEXT,
                tokens_used INTEGER,
                credits_consumed REAL,
                operation_details TEXT,
                session_id TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            )
            "#,
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create credit_usage table: {}", e)))?;
    }
    
    Ok(())
}