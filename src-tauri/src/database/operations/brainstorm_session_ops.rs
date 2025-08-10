//! Brainstorm Session database operations
//! Provides functions to interact with the brainstorm_sessions table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormSession {
    pub id: Option<i32>,
    pub project_id: i32,
    pub session_name: String,
    pub session_type: String, // "character", "plot", "worldbuilding", "dialogue", "general"
    pub initial_prompt: String,
    pub context_data: Option<String>, // JSON
    pub generated_ideas: Option<String>, // JSON array of ideas
    pub selected_ideas: Option<String>, // JSON array of selected idea IDs
    pub session_notes: Option<String>,
    pub model_used: String,
    pub total_tokens: Option<i32>,
    pub cost_credits: Option<f64>,
    pub status: String, // "active", "completed", "archived"
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Brainstorm Session database operations
impl super::BrainstormSessionOps {
    /// Create a new brainstorm session
    pub async fn create(pool: &Pool<Sqlite>, session: &BrainstormSession) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO brainstorm_sessions (
                project_id, session_name, session_type, initial_prompt, context_data,
                generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                cost_credits, status
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            session.project_id,
            session.session_name,
            session.session_type,
            session.initial_prompt,
            session.context_data,
            session.generated_ideas,
            session.selected_ideas,
            session.session_notes,
            session.model_used,
            session.total_tokens,
            session.cost_credits,
            session.status
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create brainstorm session: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get a brainstorm session by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<BrainstormSession>> {
        let row = sqlx::query!(
            r#"
            SELECT id, project_id, session_name, session_type, initial_prompt, context_data,
                   generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                   cost_credits, status, created_at, updated_at
            FROM brainstorm_sessions WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get brainstorm session: {}", e)))?;

        Ok(row.map(|r| BrainstormSession {
            id: r.id.map(|id| id.parse().unwrap_or(0)),
            project_id: r.project_id.parse().unwrap_or(0),
            session_name: r.session_name,
            session_type: r.session_type,
            initial_prompt: r.initial_prompt,
            context_data: r.context_data,
            generated_ideas: r.generated_ideas,
            selected_ideas: r.selected_ideas,
            session_notes: r.session_notes,
            model_used: r.model_used,
            total_tokens: r.total_tokens.map(|t| t as i32),
            cost_credits: r.cost_credits.map(|c| c as f64),
            status: r.status,
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get brainstorm sessions by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<Vec<BrainstormSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, session_name, session_type, initial_prompt, context_data,
                   generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                   cost_credits, status, created_at, updated_at
            FROM brainstorm_sessions WHERE project_id = ? ORDER BY updated_at DESC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get brainstorm sessions by project: {}", e)))?;

        Ok(rows.into_iter().map(|r| BrainstormSession {
            id: r.id.map(|id| id.parse().unwrap_or(0)),
            project_id: r.project_id.parse().unwrap_or(0),
            session_name: r.session_name,
            session_type: r.session_type,
            initial_prompt: r.initial_prompt,
            context_data: r.context_data,
            generated_ideas: r.generated_ideas,
            selected_ideas: r.selected_ideas,
            session_notes: r.session_notes,
            model_used: r.model_used,
            total_tokens: r.total_tokens.map(|t| t as i32),
            cost_credits: r.cost_credits.map(|c| c as f64),
            status: r.status,
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get brainstorm sessions by type
    pub async fn get_by_type(pool: &Pool<Sqlite>, session_type: &str) -> Result<Vec<BrainstormSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, session_name, session_type, initial_prompt, context_data,
                   generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                   cost_credits, status, created_at, updated_at
            FROM brainstorm_sessions WHERE session_type = ? ORDER BY updated_at DESC
            "#,
            session_type
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get brainstorm sessions by type: {}", e)))?;

        Ok(rows.into_iter().map(|r| BrainstormSession {
            id: r.id.map(|id| id.parse().unwrap_or(0)),
            project_id: r.project_id.parse().unwrap_or(0),
            session_name: r.session_name,
            session_type: r.session_type,
            initial_prompt: r.initial_prompt,
            context_data: r.context_data,
            generated_ideas: r.generated_ideas,
            selected_ideas: r.selected_ideas,
            session_notes: r.session_notes,
            model_used: r.model_used,
            total_tokens: r.total_tokens.map(|t| t as i32),
            cost_credits: r.cost_credits.map(|c| c as f64),
            status: r.status,
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get brainstorm sessions by status
    pub async fn get_by_status(pool: &Pool<Sqlite>, status: &str) -> Result<Vec<BrainstormSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, session_name, session_type, initial_prompt, context_data,
                   generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                   cost_credits, status, created_at, updated_at
            FROM brainstorm_sessions WHERE status = ? ORDER BY updated_at DESC
            "#,
            status
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get brainstorm sessions by status: {}", e)))?;

        Ok(rows.into_iter().map(|r| BrainstormSession {
            id: r.id.map(|id| id.parse().unwrap_or(0)),
            project_id: r.project_id.parse().unwrap_or(0),
            session_name: r.session_name,
            session_type: r.session_type,
            initial_prompt: r.initial_prompt,
            context_data: r.context_data,
            generated_ideas: r.generated_ideas,
            selected_ideas: r.selected_ideas,
            session_notes: r.session_notes,
            model_used: r.model_used,
            total_tokens: r.total_tokens.map(|t| t as i32),
            cost_credits: r.cost_credits.map(|c| c as f64),
            status: r.status,
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List all brainstorm sessions
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<BrainstormSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, session_name, session_type, initial_prompt, context_data,
                   generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                   cost_credits, status, created_at, updated_at
            FROM brainstorm_sessions ORDER BY updated_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list brainstorm sessions: {}", e)))?;

        Ok(rows.into_iter().map(|r| BrainstormSession {
            id: r.id.map(|id| id.parse().unwrap_or(0)),
            project_id: r.project_id.parse().unwrap_or(0),
            session_name: r.session_name,
            session_type: r.session_type,
            initial_prompt: r.initial_prompt,
            context_data: r.context_data,
            generated_ideas: r.generated_ideas,
            selected_ideas: r.selected_ideas,
            session_notes: r.session_notes,
            model_used: r.model_used,
            total_tokens: r.total_tokens.map(|t| t as i32),
            cost_credits: r.cost_credits.map(|c| c as f64),
            status: r.status,
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Update a brainstorm session
    pub async fn update(pool: &Pool<Sqlite>, id: i32, session: &BrainstormSession) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE brainstorm_sessions 
            SET project_id = ?, session_name = ?, session_type = ?, initial_prompt = ?, context_data = ?,
                generated_ideas = ?, selected_ideas = ?, session_notes = ?, model_used = ?, total_tokens = ?,
                cost_credits = ?, status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            session.project_id,
            session.session_name,
            session.session_type,
            session.initial_prompt,
            session.context_data,
            session.generated_ideas,
            session.selected_ideas,
            session.session_notes,
            session.model_used,
            session.total_tokens,
            session.cost_credits,
            session.status,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update brainstorm session: {}", e)))?;

        Ok(())
    }

    /// Update session ideas
    pub async fn update_ideas(pool: &Pool<Sqlite>, id: i32, generated_ideas: &str, selected_ideas: Option<&str>) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE brainstorm_sessions 
            SET generated_ideas = ?, selected_ideas = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            generated_ideas,
            selected_ideas,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update session ideas: {}", e)))?;

        Ok(())
    }

    /// Update session status
    pub async fn update_status(pool: &Pool<Sqlite>, id: i32, status: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE brainstorm_sessions 
            SET status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            status,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update session status: {}", e)))?;

        Ok(())
    }

    /// Update session notes
    pub async fn update_notes(pool: &Pool<Sqlite>, id: i32, notes: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE brainstorm_sessions 
            SET session_notes = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            notes,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update session notes: {}", e)))?;

        Ok(())
    }

    /// Delete a brainstorm session
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM brainstorm_sessions WHERE id = ?", id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete brainstorm session: {}", e)))?;

        Ok(())
    }

    /// Get recent brainstorm sessions (last N)
    pub async fn get_recent(pool: &Pool<Sqlite>, limit: i32) -> Result<Vec<BrainstormSession>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, session_name, session_type, initial_prompt, context_data,
                   generated_ideas, selected_ideas, session_notes, model_used, total_tokens,
                   cost_credits, status, created_at, updated_at
            FROM brainstorm_sessions ORDER BY updated_at DESC LIMIT ?
            "#,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get recent brainstorm sessions: {}", e)))?;

        Ok(rows.into_iter().map(|r| BrainstormSession {
            id: r.id.map(|id| id.parse().unwrap_or(0)),
            project_id: r.project_id.parse().unwrap_or(0),
            session_name: r.session_name,
            session_type: r.session_type,
            initial_prompt: r.initial_prompt,
            context_data: r.context_data,
            generated_ideas: r.generated_ideas,
            selected_ideas: r.selected_ideas,
            session_notes: r.session_notes,
            model_used: r.model_used,
            total_tokens: r.total_tokens.map(|t| t as i32),
            cost_credits: r.cost_credits.map(|c| c as f64),
            status: r.status,
            created_at: r.created_at.map(|dt| dt.to_string()),
            updated_at: r.updated_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get total cost for brainstorm sessions by project
    pub async fn get_total_cost_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<f64> {
        let row = sqlx::query!(
            "SELECT COALESCE(SUM(cost_credits), 0.0) as total_cost FROM brainstorm_sessions WHERE project_id = ?",
            project_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total brainstorm cost for project: {}", e)))?;

        Ok(row.total_cost)
    }
}
