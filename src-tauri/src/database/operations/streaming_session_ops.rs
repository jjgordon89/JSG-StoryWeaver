//! Streaming Session database operations
//! Provides functions to interact with the streaming_sessions table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingSession {
    pub id: Option<i32>,
    pub session_id: String,
    pub project_id: i32,
    pub model_used: String,
    pub prompt: String,
    pub status: String, // "active", "completed", "error", "cancelled"
    pub generated_content: Option<String>,
    pub tokens_generated: Option<i32>,
    pub credits_consumed: Option<f64>,
    pub error_message: Option<String>,
    pub metadata: Option<String>, // JSON
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Streaming Session database operations
impl super::StreamingSessionOps {
    /// Create a new streaming session
    pub async fn create(pool: &Pool<Sqlite>, session: &StreamingSession) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO streaming_sessions (
                session_id, project_id, model_used, prompt, status, generated_content,
                tokens_generated, credits_consumed, error_message, metadata, started_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            session.session_id,
            session.project_id,
            session.model_used,
            session.prompt,
            session.status,
            session.generated_content,
            session.tokens_generated,
            session.credits_consumed,
            session.error_message,
            session.metadata,
            session.started_at
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create streaming session: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get a streaming session by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<StreamingSession>> {
        let row = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get streaming session: {}", e)))?;

        Ok(row.map(|r| StreamingSession {
            id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get a streaming session by session ID
    pub async fn get_by_session_id(pool: &Pool<Sqlite>, session_id: &str) -> Result<Option<StreamingSession>> {
        let row = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions WHERE session_id = ?
            "#,
            session_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get streaming session by session ID: {}", e)))?;

        Ok(row.map(|r| StreamingSession {
            id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get streaming sessions by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<Vec<StreamingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions WHERE project_id = ? ORDER BY created_at DESC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get streaming sessions by project: {}", e)))?;

        Ok(rows.into_iter().map(|r| StreamingSession {
            id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get streaming sessions by status
    pub async fn get_by_status(pool: &Pool<Sqlite>, status: &str) -> Result<Vec<StreamingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions WHERE status = ? ORDER BY created_at DESC
            "#,
            status
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get streaming sessions by status: {}", e)))?;

        Ok(rows.into_iter().map(|r| StreamingSession {
             id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List all streaming sessions
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<StreamingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list streaming sessions: {}", e)))?;

        Ok(rows.into_iter().map(|r| StreamingSession {
            id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Update streaming session content
    pub async fn update_content(pool: &Pool<Sqlite>, session_id: &str, generated_content: &str, tokens_generated: i32) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE streaming_sessions 
            SET generated_content = ?, tokens_generated = ?, updated_at = CURRENT_TIMESTAMP
            WHERE session_id = ?
            "#,
            generated_content,
            tokens_generated,
            session_id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update streaming session content: {}", e)))?;

        Ok(())
    }

    /// Update streaming session status
    pub async fn update_status(pool: &Pool<Sqlite>, session_id: &str, status: &str) -> Result<()> {
        let completed_at = if status == "completed" || status == "error" || status == "cancelled" {
            Some("CURRENT_TIMESTAMP")
        } else {
            None
        };

        if let Some(_) = completed_at {
            sqlx::query!(
                r#"
                UPDATE streaming_sessions 
                SET status = ?, completed_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
                WHERE session_id = ?
                "#,
                status,
                session_id
            )
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to update streaming session status: {}", e)))?;
        } else {
            sqlx::query!(
                r#"
                UPDATE streaming_sessions 
                SET status = ?, updated_at = CURRENT_TIMESTAMP
                WHERE session_id = ?
                "#,
                status,
                session_id
            )
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to update streaming session status: {}", e)))?;
        }

        Ok(())
    }

    /// Update streaming session with error
    pub async fn update_error(pool: &Pool<Sqlite>, session_id: &str, error_message: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE streaming_sessions 
            SET status = 'error', error_message = ?, completed_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE session_id = ?
            "#,
            error_message,
            session_id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update streaming session error: {}", e)))?;

        Ok(())
    }

    /// Update streaming session credits
    pub async fn update_credits(pool: &Pool<Sqlite>, session_id: &str, credits_consumed: f64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE streaming_sessions 
            SET credits_consumed = ?, updated_at = CURRENT_TIMESTAMP
            WHERE session_id = ?
            "#,
            credits_consumed,
            session_id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update streaming session credits: {}", e)))?;

        Ok(())
    }

    /// Update streaming session metadata
    pub async fn update_metadata(pool: &Pool<Sqlite>, session_id: &str, metadata: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE streaming_sessions 
            SET metadata = ?, updated_at = CURRENT_TIMESTAMP
            WHERE session_id = ?
            "#,
            metadata,
            session_id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update streaming session metadata: {}", e)))?;

        Ok(())
    }

    /// Delete a streaming session
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM streaming_sessions WHERE id = ?", id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete streaming session: {}", e)))?;

        Ok(())
    }

    /// Delete streaming session by session ID
    pub async fn delete_by_session_id(pool: &Pool<Sqlite>, session_id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM streaming_sessions WHERE session_id = ?", session_id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete streaming session by session ID: {}", e)))?;

        Ok(())
    }

    /// Get active streaming sessions
    pub async fn get_active_sessions(pool: &Pool<Sqlite>) -> Result<Vec<StreamingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions WHERE status = 'active' ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get active streaming sessions: {}", e)))?;

        Ok(rows.into_iter().map(|r| StreamingSession {
            id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get recent streaming sessions (last N)
    pub async fn get_recent(pool: &Pool<Sqlite>, limit: i32) -> Result<Vec<StreamingSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, session_id, project_id, model_used, prompt, status, generated_content,
                   tokens_generated, credits_consumed, error_message, metadata, started_at,
                   completed_at, created_at, updated_at
            FROM streaming_sessions ORDER BY created_at DESC LIMIT ?
            "#,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get recent streaming sessions: {}", e)))?;

        Ok(rows.into_iter().map(|r| StreamingSession {
             id: Some(r.id as i32),
            session_id: r.session_id,
            project_id: r.project_id as i32,
            model_used: r.model_used,
            prompt: r.prompt,
            status: r.status,
            generated_content: r.generated_content,
            tokens_generated: r.tokens_generated.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            error_message: r.error_message,
            metadata: r.metadata,
            started_at: r.started_at.map(|dt| dt.to_string()),
            completed_at: r.completed_at.map(|dt| dt.to_string()),
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get total credits consumed by streaming sessions for a project
    pub async fn get_total_credits_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<f64> {
        let row = sqlx::query!(
            "SELECT COALESCE(SUM(credits_consumed), 0.0) as total_credits FROM streaming_sessions WHERE project_id = ?",
            project_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total streaming credits for project: {}", e)))?;

        Ok(row.total_credits)
    }

    /// Get streaming session statistics
    pub async fn get_session_stats(pool: &Pool<Sqlite>) -> Result<Vec<(String, i32, f64)>> {
        let rows = sqlx::query!(
            r#"
            SELECT status, 
                   COUNT(*) as session_count,
                   COALESCE(SUM(credits_consumed), 0.0) as total_credits
            FROM streaming_sessions 
            GROUP BY status 
            ORDER BY session_count DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get streaming session stats: {}", e)))?;

        Ok(rows.into_iter().map(|r| (r.status, r.session_count as i32, r.total_credits)).collect())
    }

    /// Clean up old completed sessions (older than specified days)
    pub async fn cleanup_old_sessions(pool: &Pool<Sqlite>, days_old: i32) -> Result<i32> {
        let result = sqlx::query!(
            r#"
            DELETE FROM streaming_sessions 
            WHERE status IN ('completed', 'error', 'cancelled') 
            AND DATE(completed_at) < DATE('now', '-' || ? || ' days')
            "#,
            days_old
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to cleanup old streaming sessions: {}", e)))?;

        Ok(result.rows_affected() as i32)
    }
}