//! AI Provider database operations
//! Provides functions to interact with the ai_providers table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub id: Option<i32>,
    pub name: String,
    pub display_name: String,
    pub api_endpoint: Option<String>,
    pub is_active: bool,
    pub created_at: Option<String>,
}

/// AI Provider database operations
impl super::AIProviderOps {
    /// Create a new AI provider
    pub async fn create(pool: &Pool<Sqlite>, provider: &AIProvider) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO ai_providers (name, display_name, api_endpoint, is_active)
            VALUES (?, ?, ?, ?)
            "#,
            provider.name,
            provider.display_name,
            provider.api_endpoint,
            provider.is_active
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create AI provider: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get an AI provider by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<AIProvider>> {
        let row = sqlx::query!(
            "SELECT id, name, display_name, api_endpoint, is_active, created_at FROM ai_providers WHERE id = ?",
            id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI provider: {}", e)))?;

        Ok(row.map(|r| AIProvider {
            id: Some(r.id as i32),
            name: r.name,
            display_name: r.display_name,
            api_endpoint: r.api_endpoint,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get an AI provider by name
    pub async fn get_by_name(pool: &Pool<Sqlite>, name: &str) -> Result<Option<AIProvider>> {
        let row = sqlx::query!(
            "SELECT id, name, display_name, api_endpoint, is_active, created_at FROM ai_providers WHERE name = ?",
            name
        )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI provider by name: {}", e)))?;

        Ok(row.map(|r| AIProvider {
            id: r.id.map(|id| id as i32),
            name: r.name,
            display_name: r.display_name,
            api_endpoint: r.api_endpoint,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// List all AI providers
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<AIProvider>> {
        let rows = sqlx::query!(
            "SELECT id, name, display_name, api_endpoint, is_active, created_at FROM ai_providers ORDER BY name"
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list AI providers: {}", e)))?;

        Ok(rows.into_iter().map(|r| AIProvider {
            id: r.id.map(|id| id as i32),
            name: r.name,
            display_name: r.display_name,
            api_endpoint: r.api_endpoint,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List active AI providers
    pub async fn list_active(pool: &Pool<Sqlite>) -> Result<Vec<AIProvider>> {
        let rows = sqlx::query!(
            "SELECT id, name, display_name, api_endpoint, is_active, created_at FROM ai_providers WHERE is_active = 1 ORDER BY name"
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list active AI providers: {}", e)))?;

        Ok(rows.into_iter().map(|r| AIProvider {
            id: r.id.map(|id| id as i32),
            name: r.name,
            display_name: r.display_name,
            api_endpoint: r.api_endpoint,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Update an AI provider
    pub async fn update(pool: &Pool<Sqlite>, id: i32, provider: &AIProvider) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE ai_providers 
            SET name = ?, display_name = ?, api_endpoint = ?, is_active = ?
            WHERE id = ?
            "#,
            provider.name,
            provider.display_name,
            provider.api_endpoint,
            provider.is_active,
            id
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update AI provider: {}", e)))?;

        Ok(())
    }

    /// Delete an AI provider
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM ai_providers WHERE id = ?", id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete AI provider: {}", e)))?;

        Ok(())
    }

    /// Toggle AI provider active status
    pub async fn toggle_active(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE ai_providers SET is_active = NOT is_active WHERE id = ?",
            id
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to toggle AI provider status: {}", e)))?;

        Ok(())
    }
}
