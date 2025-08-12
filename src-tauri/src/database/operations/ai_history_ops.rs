//! AI Generation History Operations

use crate::database::models::AIGenerationHistory;
use crate::error::{StoryWeaverError, Result};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

impl super::AIHistoryOps {
    /// Create a new AI generation history record
    pub async fn create(pool: &Pool<Sqlite>, mut record: AIGenerationHistory) -> Result<AIGenerationHistory> {
        // Generate ID if not provided
        if record.id.is_empty() {
            record.id = Uuid::new_v4().to_string();
        }

        sqlx::query!(
            r#"
            INSERT INTO ai_generation_history (
                id, project_id, document_id, generation_type, provider, model,
                prompt, response, token_count, cost_estimate, context_used, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            record.id,
            record.project_id,
            record.document_id,
            record.generation_type,
            record.provider,
            record.model,
            record.prompt,
            record.response,
            record.token_count,
            record.cost_estimate,
            record.context_used,
            record.created_at
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create AI history record: {}", e)))?;

        Ok(record)
    }

    /// Get AI generation history by project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str, limit: Option<i32>) -> Result<Vec<AIGenerationHistory>> {
        let limit = limit.unwrap_or(100);
        
        let records = sqlx::query_as::<_, AIGenerationHistory>(
            "SELECT * FROM ai_generation_history WHERE project_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(project_id)
        .bind(limit)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI history: {}", e)))?;

        Ok(records)
    }

    /// Get AI generation history by document
    pub async fn get_by_document(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<AIGenerationHistory>> {
        let records = sqlx::query_as::<_, AIGenerationHistory>(
            "SELECT * FROM ai_generation_history WHERE document_id = ? ORDER BY created_at DESC"
        )
        .bind(document_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI history by document: {}", e)))?;

        Ok(records)
    }

    /// Get AI generation history by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<AIGenerationHistory>> {
        let record = sqlx::query_as::<_, AIGenerationHistory>(
            "SELECT * FROM ai_generation_history WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI history by ID: {}", e)))?;

        Ok(record)
    }

    /// Delete AI generation history record
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM ai_generation_history WHERE id = ?", id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete AI history: {}", e)))?;

        Ok(())
    }

    /// Clear all AI generation history for a project
    pub async fn clear_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<()> {
        sqlx::query!("DELETE FROM ai_generation_history WHERE project_id = ?", project_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to clear AI history: {}", e)))?;

        Ok(())
    }

    /// Update AI generation history record
    pub async fn update(pool: &Pool<Sqlite>, record: &AIGenerationHistory) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE ai_generation_history SET
                project_id = ?, document_id = ?, generation_type = ?, provider = ?,
                model = ?, prompt = ?, response = ?, token_count = ?, cost_estimate = ?,
                context_used = ?
            WHERE id = ?
            "#,
            record.project_id,
            record.document_id,
            record.generation_type,
            record.provider,
            record.model,
            record.prompt,
            record.response,
            record.token_count,
            record.cost_estimate,
            record.context_used,
            record.id
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update AI history: {}", e)))?;

        Ok(())
    }
}