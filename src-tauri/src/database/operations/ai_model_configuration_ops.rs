//! AI Model Configuration database operations
//! Provides functions to interact with the ai_model_configurations table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelConfiguration {
    pub id: Option<i32>,
    pub provider_id: i32,
    pub model_name: String,
    pub display_name: String,
    pub context_window: i32,
    pub max_output_tokens: i32,
    pub supports_streaming: bool,
    pub supports_images: bool,
    pub cost_per_input_token: Option<f64>,
    pub cost_per_output_token: Option<f64>,
    pub cost_per_image: Option<f64>,
    pub quality_tier: String,
    pub specializations: Option<String>, // JSON
    pub is_active: bool,
    pub created_at: Option<String>,
}

/// AI Model Configuration database operations
impl super::AIModelConfigurationOps {
    /// Create a new AI model configuration
    pub async fn create(pool: &Pool<Sqlite>, config: &AIModelConfiguration) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO ai_model_configurations (
                provider_id, model_name, display_name, context_window, max_output_tokens,
                supports_streaming, supports_images, cost_per_input_token, cost_per_output_token,
                cost_per_image, quality_tier, specializations, is_active
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            config.provider_id,
            config.model_name,
            config.display_name,
            config.context_window,
            config.max_output_tokens,
            config.supports_streaming,
            config.supports_images,
            config.cost_per_input_token,
            config.cost_per_output_token,
            config.cost_per_image,
            config.quality_tier,
            config.specializations,
            config.is_active
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create AI model configuration: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get an AI model configuration by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<AIModelConfiguration>> {
        let row = sqlx::query!(
            r#"
            SELECT id, provider_id, model_name, display_name, context_window, max_output_tokens,
                   supports_streaming, supports_images, cost_per_input_token, cost_per_output_token,
                   cost_per_image, quality_tier, specializations, is_active, created_at
            FROM ai_model_configurations WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI model configuration: {}", e)))?;

        Ok(row.map(|r| AIModelConfiguration {
            id: Some(r.id as i32),
            provider_id: r.provider_id as i32,
            model_name: r.model_name,
            display_name: r.display_name,
            context_window: r.context_window as i32,
            max_output_tokens: r.max_output_tokens as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_images: r.supports_images.unwrap_or(false),
            cost_per_input_token: r.cost_per_input_token,
            cost_per_output_token: r.cost_per_output_token,
            cost_per_image: r.cost_per_image,
            quality_tier: r.quality_tier.unwrap_or("standard".to_string()),
            specializations: r.specializations,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get AI model configurations by provider ID
    pub async fn get_by_provider(pool: &Pool<Sqlite>, provider_id: i32) -> Result<Vec<AIModelConfiguration>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, provider_id, model_name, display_name, context_window, max_output_tokens,
                   supports_streaming, supports_images, cost_per_input_token, cost_per_output_token,
                   cost_per_image, quality_tier, specializations, is_active, created_at
            FROM ai_model_configurations WHERE provider_id = ? ORDER BY display_name
            "#,
            provider_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI model configurations by provider: {}", e)))?;

        Ok(rows.into_iter().map(|r| AIModelConfiguration {
            id: Some(r.id as i32),
            provider_id: r.provider_id as i32,
            model_name: r.model_name,
            display_name: r.display_name,
            context_window: r.context_window as i32,
            max_output_tokens: r.max_output_tokens as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_images: r.supports_images.unwrap_or(false),
            cost_per_input_token: r.cost_per_input_token,
            cost_per_output_token: r.cost_per_output_token,
            cost_per_image: r.cost_per_image,
            quality_tier: r.quality_tier.unwrap_or("standard".to_string()),
            specializations: r.specializations,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List all AI model configurations
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<AIModelConfiguration>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, provider_id, model_name, display_name, context_window, max_output_tokens,
                   supports_streaming, supports_images, cost_per_input_token, cost_per_output_token,
                   cost_per_image, quality_tier, specializations, is_active, created_at
            FROM ai_model_configurations ORDER BY display_name
            "#
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list AI model configurations: {}", e)))?;

        Ok(rows.into_iter().map(|r| AIModelConfiguration {
            id: Some(r.id as i32),
            provider_id: r.provider_id as i32,
            model_name: r.model_name,
            display_name: r.display_name,
            context_window: r.context_window as i32,
            max_output_tokens: r.max_output_tokens as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_images: r.supports_images.unwrap_or(false),
            cost_per_input_token: r.cost_per_input_token,
            cost_per_output_token: r.cost_per_output_token,
            cost_per_image: r.cost_per_image,
            quality_tier: r.quality_tier.unwrap_or_default(),
            specializations: r.specializations,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List active AI model configurations
    pub async fn list_active(pool: &Pool<Sqlite>) -> Result<Vec<AIModelConfiguration>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, provider_id, model_name, display_name, context_window, max_output_tokens,
                   supports_streaming, supports_images, cost_per_input_token, cost_per_output_token,
                   cost_per_image, quality_tier, specializations, is_active, created_at
            FROM ai_model_configurations WHERE is_active = 1 ORDER BY display_name
            "#
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list active AI model configurations: {}", e)))?;

        Ok(rows.into_iter().map(|r| AIModelConfiguration {
            id: Some(r.id as i32),
            provider_id: r.provider_id as i32,
            model_name: r.model_name,
            display_name: r.display_name,
            context_window: r.context_window as i32,
            max_output_tokens: r.max_output_tokens as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_images: r.supports_images.unwrap_or(false),
            cost_per_input_token: r.cost_per_input_token,
            cost_per_output_token: r.cost_per_output_token,
            cost_per_image: r.cost_per_image,
            quality_tier: r.quality_tier.unwrap_or_default(),
            specializations: r.specializations,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Update an AI model configuration
    pub async fn update(pool: &Pool<Sqlite>, id: i32, config: &AIModelConfiguration) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE ai_model_configurations 
            SET provider_id = ?, model_name = ?, display_name = ?, context_window = ?, max_output_tokens = ?,
                supports_streaming = ?, supports_images = ?, cost_per_input_token = ?, cost_per_output_token = ?,
                cost_per_image = ?, quality_tier = ?, specializations = ?, is_active = ?
            WHERE id = ?
            "#,
            config.provider_id,
            config.model_name,
            config.display_name,
            config.context_window,
            config.max_output_tokens,
            config.supports_streaming,
            config.supports_images,
            config.cost_per_input_token,
            config.cost_per_output_token,
            config.cost_per_image,
            config.quality_tier,
            config.specializations,
            config.is_active,
            id
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update AI model configuration: {}", e)))?;

        Ok(())
    }

    /// Delete an AI model configuration
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM ai_model_configurations WHERE id = ?", id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete AI model configuration: {}", e)))?;

        Ok(())
    }

    /// Toggle AI model configuration active status
    pub async fn toggle_active(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!(
            "UPDATE ai_model_configurations SET is_active = NOT is_active WHERE id = ?",
            id
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to toggle AI model configuration status: {}", e)))?;

        Ok(())
    }

    /// Get model configuration by model name and provider
    pub async fn get_by_model_and_provider(pool: &Pool<Sqlite>, model_name: &str, provider_id: i32) -> Result<Option<AIModelConfiguration>> {
        let row = sqlx::query!(
            r#"
            SELECT id, provider_id, model_name, display_name, context_window, max_output_tokens,
                   supports_streaming, supports_images, cost_per_input_token, cost_per_output_token,
                   cost_per_image, quality_tier, specializations, is_active, created_at
            FROM ai_model_configurations WHERE model_name = ? AND provider_id = ?
            "#,
            model_name,
            provider_id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI model configuration by model and provider: {}", e)))?;

        Ok(row.map(|r| AIModelConfiguration {
            id: Some(r.id as i32),
            provider_id: r.provider_id as i32,
            model_name: r.model_name,
            display_name: r.display_name,
            context_window: r.context_window as i32,
            max_output_tokens: r.max_output_tokens as i32,
            supports_streaming: r.supports_streaming.unwrap_or(true),
            supports_images: r.supports_images.unwrap_or(false),
            cost_per_input_token: r.cost_per_input_token,
            cost_per_output_token: r.cost_per_output_token,
            cost_per_image: r.cost_per_image,
            quality_tier: r.quality_tier.unwrap_or_default(),
            specializations: r.specializations,
            is_active: r.is_active.unwrap_or(true),
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }
}