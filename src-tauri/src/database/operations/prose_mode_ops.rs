//! Prose Mode database operations
//! Provides functions to interact with the prose_modes table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProseMode {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub model_configuration_id: i32,
    pub creativity_level: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub special_instructions: Option<String>,
    pub is_experimental: bool,
    pub max_context_words: i32,
    pub max_generation_words: i32,
    pub supports_streaming: bool,
    pub supports_unfiltered: bool,
    pub is_active: bool,
    pub created_at: Option<String>,
}

/// Prose Mode database operations
impl super::ProseModeOps {
    /// Create a new prose mode
    pub async fn create(pool: &Pool<Sqlite>, prose_mode: &ProseMode) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO prose_modes (
                name, description, model_configuration_id, creativity_level, temperature, top_p,
                frequency_penalty, presence_penalty, special_instructions, is_experimental,
                max_context_words, max_generation_words, supports_streaming, supports_unfiltered, is_active
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            prose_mode.name,
            prose_mode.description,
            prose_mode.model_configuration_id,
            prose_mode.creativity_level,
            prose_mode.temperature,
            prose_mode.top_p,
            prose_mode.frequency_penalty,
            prose_mode.presence_penalty,
            prose_mode.special_instructions,
            prose_mode.is_experimental,
            prose_mode.max_context_words,
            prose_mode.max_generation_words,
            prose_mode.supports_streaming,
            prose_mode.supports_unfiltered,
            prose_mode.is_active
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create prose mode: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get a prose mode by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<ProseMode>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, model_configuration_id, creativity_level, temperature, top_p,
                   frequency_penalty, presence_penalty, special_instructions, is_experimental,
                   max_context_words, max_generation_words, supports_streaming, supports_unfiltered,
                   is_active, created_at
            FROM prose_modes WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get prose mode: {}", e)))?;

        Ok(row.map(|r| ProseMode {
            id: Some(r.id as i32),
            name: r.name,
            description: r.description,
            model_configuration_id: r.model_configuration_id as i32,
            creativity_level: r.creativity_level.unwrap_or(5) as i32,
            temperature: r.temperature.unwrap_or(0.7) as f32,
            top_p: r.top_p.unwrap_or(0.9) as f32,
            frequency_penalty: r.frequency_penalty.unwrap_or(0.0) as f32,
            presence_penalty: r.presence_penalty.unwrap_or(0.0) as f32,
            special_instructions: r.special_instructions,
            is_experimental: r.is_experimental.unwrap_or(false),
            max_context_words: r.max_context_words.unwrap_or(4000) as i32,
            max_generation_words: r.max_generation_words.unwrap_or(2000) as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_unfiltered: r.supports_unfiltered.unwrap_or(false),
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get a prose mode by name
    pub async fn get_by_name(pool: &Pool<Sqlite>, name: &str) -> Result<Option<ProseMode>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, model_configuration_id, creativity_level, temperature, top_p,
                   frequency_penalty, presence_penalty, special_instructions, is_experimental,
                   max_context_words, max_generation_words, supports_streaming, supports_unfiltered,
                   is_active, created_at
            FROM prose_modes WHERE name = ?
            "#,
            name
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get prose mode by name: {}", e)))?;

        Ok(row.map(|r| ProseMode {
            id: r.id.map(|id| id as i32),
            name: r.name,
            description: r.description,
            model_configuration_id: r.model_configuration_id as i32,
            creativity_level: r.creativity_level.unwrap_or(5) as i32,
            temperature: r.temperature.unwrap_or(0.7) as f32,
            top_p: r.top_p.unwrap_or(0.9) as f32,
            frequency_penalty: r.frequency_penalty.unwrap_or(0.0) as f32,
            presence_penalty: r.presence_penalty.unwrap_or(0.0) as f32,
            special_instructions: r.special_instructions,
            is_experimental: r.is_experimental.unwrap_or(false),
            max_context_words: r.max_context_words.unwrap_or(4000) as i32,
            max_generation_words: r.max_generation_words.unwrap_or(2000) as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_unfiltered: r.supports_unfiltered.unwrap_or(false),
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// List all prose modes
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<ProseMode>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, model_configuration_id, creativity_level, temperature, top_p,
                   frequency_penalty, presence_penalty, special_instructions, is_experimental,
                   max_context_words, max_generation_words, supports_streaming, supports_unfiltered,
                   is_active, created_at
            FROM prose_modes ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list prose modes: {}", e)))?;

        Ok(rows.into_iter().map(|r| ProseMode {
            id: r.id.map(|id| id as i32),
            name: r.name,
            description: r.description,
            model_configuration_id: r.model_configuration_id as i32,
            creativity_level: r.creativity_level.unwrap_or(5) as i32,
            temperature: r.temperature.unwrap_or(0.7) as f32,
            top_p: r.top_p.unwrap_or(0.9) as f32,
            frequency_penalty: r.frequency_penalty.unwrap_or(0.0) as f32,
            presence_penalty: r.presence_penalty.unwrap_or(0.0) as f32,
            special_instructions: r.special_instructions,
            is_experimental: r.is_experimental.unwrap_or(false),
                max_context_words: r.max_context_words.unwrap_or(4000) as i32,
                max_generation_words: r.max_generation_words.unwrap_or(2000) as i32,
                supports_streaming: r.supports_streaming.unwrap_or(true),
                supports_unfiltered: r.supports_unfiltered.unwrap_or(false),
                is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List active prose modes
    pub async fn list_active(pool: &Pool<Sqlite>) -> Result<Vec<ProseMode>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, model_configuration_id, creativity_level, temperature, top_p,
                   frequency_penalty, presence_penalty, special_instructions, is_experimental,
                   max_context_words, max_generation_words, supports_streaming, supports_unfiltered,
                   is_active, created_at
            FROM prose_modes WHERE is_active = 1 ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list active prose modes: {}", e)))?;

        Ok(rows.into_iter().map(|r| ProseMode {
            id: r.id.map(|id| id as i32),
            name: r.name,
            description: r.description,
            model_configuration_id: r.model_configuration_id as i32,
            creativity_level: r.creativity_level.unwrap_or(5) as i32,
            temperature: r.temperature.unwrap_or(0.7) as f32,
            top_p: r.top_p.unwrap_or(0.9) as f32,
            frequency_penalty: r.frequency_penalty.unwrap_or(0.0) as f32,
            presence_penalty: r.presence_penalty.unwrap_or(0.0) as f32,
            special_instructions: r.special_instructions,
            is_experimental: r.is_experimental.unwrap_or(false),
                max_context_words: r.max_context_words.unwrap_or(4000) as i32,
                max_generation_words: r.max_generation_words.unwrap_or(2000) as i32,
                supports_streaming: r.supports_streaming.unwrap_or(true),
                supports_unfiltered: r.supports_unfiltered.unwrap_or(false),
                is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Update a prose mode
    pub async fn update(pool: &Pool<Sqlite>, id: i32, prose_mode: &ProseMode) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE prose_modes 
            SET name = ?, description = ?, model_configuration_id = ?, creativity_level = ?, temperature = ?, top_p = ?,
                frequency_penalty = ?, presence_penalty = ?, special_instructions = ?, is_experimental = ?,
                max_context_words = ?, max_generation_words = ?, supports_streaming = ?, supports_unfiltered = ?, is_active = ?
            WHERE id = ?
            "#,
            prose_mode.name,
            prose_mode.description,
            prose_mode.model_configuration_id,
            prose_mode.creativity_level,
            prose_mode.temperature,
            prose_mode.top_p,
            prose_mode.frequency_penalty,
            prose_mode.presence_penalty,
            prose_mode.special_instructions,
            prose_mode.is_experimental,
            prose_mode.max_context_words,
            prose_mode.max_generation_words,
            prose_mode.supports_streaming,
            prose_mode.supports_unfiltered,
            prose_mode.is_active,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update prose mode: {}", e)))?;

        Ok(())
    }

    /// Delete a prose mode
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM prose_modes WHERE id = ?", id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete prose mode: {}", e)))?;

        Ok(())
    }

    /// Toggle prose mode active status
    pub async fn toggle_active(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE prose_modes SET is_active = NOT is_active WHERE id = ?",
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to toggle prose mode status: {}", e)))?;

        Ok(())
    }

    /// Get prose modes by model configuration
    pub async fn get_by_model_configuration(pool: &Pool<Sqlite>, model_config_id: i32) -> Result<Vec<ProseMode>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, model_configuration_id, creativity_level, temperature, top_p,
                   frequency_penalty, presence_penalty, special_instructions, is_experimental,
                   max_context_words, max_generation_words, supports_streaming, supports_unfiltered,
                   is_active, created_at
            FROM prose_modes WHERE model_configuration_id = ? ORDER BY name
            "#,
            model_config_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get prose modes by model configuration: {}", e)))?;

        Ok(rows.into_iter().map(|r| ProseMode {
            id: r.id.map(|id| id as i32),
            name: r.name,
            description: r.description,
            model_configuration_id: r.model_configuration_id as i32,
            creativity_level: r.creativity_level.unwrap_or(5) as i32,
            temperature: r.temperature.unwrap_or(0.7) as f32,
            top_p: r.top_p.unwrap_or(0.9) as f32,
            frequency_penalty: r.frequency_penalty.unwrap_or(0.0) as f32,
            presence_penalty: r.presence_penalty.unwrap_or(0.0) as f32,
            special_instructions: r.special_instructions,
            is_experimental: r.is_experimental.unwrap_or(false),
            max_context_words: r.max_context_words.unwrap_or(4000) as i32,
            max_generation_words: r.max_generation_words.unwrap_or(2000) as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_unfiltered: r.supports_unfiltered.unwrap_or(false),
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }
}