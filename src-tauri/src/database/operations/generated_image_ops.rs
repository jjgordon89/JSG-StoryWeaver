//! Generated Image database operations
//! Provides functions to interact with the generated_images table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub id: Option<i32>,
    pub project_id: i32,
    pub prompt: String,
    pub negative_prompt: Option<String>,
    pub model_used: String,
    pub image_url: String,
    pub local_path: Option<String>,
    pub width: i32,
    pub height: i32,
    pub seed: Option<i64>,
    pub steps: Option<i32>,
    pub cfg_scale: Option<f64>,
    pub style: Option<String>,
    pub generation_time: Option<f64>,
    pub cost_credits: Option<f64>,
    pub metadata: Option<String>, // JSON
    pub created_at: Option<String>,
}

/// Generated Image database operations
impl super::GeneratedImageOps {
    /// Create a new generated image record
    pub async fn create(pool: &Pool<Sqlite>, image: &GeneratedImage) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO generated_images (
                project_id, prompt, negative_prompt, model_used, image_url, local_path,
                width, height, seed, steps, cfg_scale, style, generation_time, cost_credits, metadata
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            image.project_id,
            image.prompt,
            image.negative_prompt,
            image.model_used,
            image.image_url,
            image.local_path,
            image.width,
            image.height,
            image.seed,
            image.steps,
            image.cfg_scale,
            image.style,
            image.generation_time,
            image.cost_credits,
            image.metadata
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create generated image: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get a generated image by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<GeneratedImage>> {
        let row = sqlx::query!(
            r#"
            SELECT id, project_id, prompt, negative_prompt, model_used, image_url, local_path,
                   width, height, seed, steps, cfg_scale, style, generation_time, cost_credits, metadata, created_at
            FROM generated_images WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get generated image: {}", e)))?;

        Ok(row.map(|r| GeneratedImage {
            id: Some(r.id.unwrap_or_else(|| String::new()).parse().unwrap_or(0)),
            project_id: r.project_id.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            prompt: r.prompt.unwrap_or_else(|| String::new()),
            negative_prompt: r.negative_prompt,
            model_used: r.model_used.unwrap_or_else(|| String::new()),
            image_url: r.image_url.unwrap_or_else(|| String::new()),
            local_path: r.local_path,
            width: r.width.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            height: r.height.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            seed: r.seed,
            steps: r.steps.map(|s| s as i32),
            cfg_scale: r.cfg_scale,
            style: r.style,
            generation_time: r.generation_time,
            cost_credits: r.cost_credits.map(|c| c as f64),
            metadata: r.metadata,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get generated images by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<Vec<GeneratedImage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, prompt, negative_prompt, model_used, image_url, local_path,
                   width, height, seed, steps, cfg_scale, style, generation_time, cost_credits, metadata, created_at
            FROM generated_images WHERE project_id = ? ORDER BY created_at DESC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get generated images by project: {}", e)))?;

        Ok(rows.into_iter().map(|r| GeneratedImage {
            id: Some(r.id.unwrap_or_else(|| String::new()).parse().unwrap_or(0)),
            project_id: r.project_id.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            prompt: r.prompt.unwrap_or_else(|| String::new()),
            negative_prompt: r.negative_prompt,
            model_used: r.model_used.unwrap_or_else(|| String::new()),
            image_url: r.image_url.unwrap_or_else(|| String::new()),
            local_path: r.local_path,
            width: r.width.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            height: r.height.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            seed: r.seed,
            steps: r.steps.map(|s| s as i32),
            cfg_scale: r.cfg_scale,
            style: r.style,
            generation_time: r.generation_time,
            cost_credits: r.cost_credits.map(|c| c as f64),
            metadata: r.metadata,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List all generated images
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<GeneratedImage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, prompt, negative_prompt, model_used, image_url, local_path,
                   width, height, seed, steps, cfg_scale, style, generation_time, cost_credits, metadata, created_at
            FROM generated_images ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list generated images: {}", e)))?;

        Ok(rows.into_iter().map(|r| GeneratedImage {
            id: Some(r.id.unwrap_or_else(|| String::new()).parse().unwrap_or(0)),
            project_id: r.project_id.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            prompt: r.prompt.unwrap_or_else(|| String::new()),
            negative_prompt: r.negative_prompt,
            model_used: r.model_used.unwrap_or_else(|| String::new()),
            image_url: r.image_url.unwrap_or_else(|| String::new()),
            local_path: r.local_path,
            width: r.width.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            height: r.height.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            seed: r.seed,
            steps: r.steps.map(|s| s as i32),
            cfg_scale: r.cfg_scale,
            style: r.style,
            generation_time: r.generation_time,
            cost_credits: r.cost_credits.map(|c| c as f64),
            metadata: r.metadata,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Update a generated image record
    pub async fn update(pool: &Pool<Sqlite>, id: i32, image: &GeneratedImage) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE generated_images 
            SET project_id = ?, prompt = ?, negative_prompt = ?, model_used = ?, image_url = ?, local_path = ?,
                width = ?, height = ?, seed = ?, steps = ?, cfg_scale = ?, style = ?, generation_time = ?, 
                cost_credits = ?, metadata = ?
            WHERE id = ?
            "#,
            image.project_id,
            image.prompt,
            image.negative_prompt,
            image.model_used,
            image.image_url,
            image.local_path,
            image.width,
            image.height,
            image.seed,
            image.steps,
            image.cfg_scale,
            image.style,
            image.generation_time,
            image.cost_credits,
            image.metadata,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update generated image: {}", e)))?;

        Ok(())
    }

    /// Delete a generated image record
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM generated_images WHERE id = ?", id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete generated image: {}", e)))?;

        Ok(())
    }

    /// Update local path for a generated image
    pub async fn update_local_path(pool: &Pool<Sqlite>, id: i32, local_path: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE generated_images SET local_path = ? WHERE id = ?",
            local_path,
            id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update image local path: {}", e)))?;

        Ok(())
    }

    /// Get generated images by model
    pub async fn get_by_model(pool: &Pool<Sqlite>, model_used: &str) -> Result<Vec<GeneratedImage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, prompt, negative_prompt, model_used, image_url, local_path,
                   width, height, seed, steps, cfg_scale, style, generation_time, cost_credits, metadata, created_at
            FROM generated_images WHERE model_used = ? ORDER BY created_at DESC
            "#,
            model_used
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get generated images by model: {}", e)))?;

        Ok(rows.into_iter().map(|r| GeneratedImage {
            id: Some(r.id.unwrap_or_else(|| String::new()).parse().unwrap_or(0)),
            project_id: r.project_id.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            prompt: r.prompt.unwrap_or_else(|| String::new()),
            negative_prompt: r.negative_prompt,
            model_used: r.model_used.unwrap_or_else(|| String::new()),
            image_url: r.image_url.unwrap_or_else(|| String::new()),
            local_path: r.local_path,
            width: r.width.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            height: r.height.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            seed: r.seed,
            steps: r.steps.map(|s| s as i32),
            cfg_scale: r.cfg_scale,
            style: r.style,
            generation_time: r.generation_time,
            cost_credits: r.cost_credits.map(|c| c as f64),
            metadata: r.metadata,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get recent generated images (last N)
    pub async fn get_recent(pool: &Pool<Sqlite>, limit: i32) -> Result<Vec<GeneratedImage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id, prompt, negative_prompt, model_used, image_url, local_path,
                   width, height, seed, steps, cfg_scale, style, generation_time, cost_credits, metadata, created_at
            FROM generated_images ORDER BY created_at DESC LIMIT ?
            "#,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get recent generated images: {}", e)))?;

        Ok(rows.into_iter().map(|r| GeneratedImage {
            id: Some(r.id.unwrap_or_else(|| String::new()).parse().unwrap_or(0)),
            project_id: r.project_id.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            prompt: r.prompt.unwrap_or_else(|| String::new()),
            negative_prompt: r.negative_prompt,
            model_used: r.model_used.unwrap_or_else(|| String::new()),
            image_url: r.image_url.unwrap_or_else(|| String::new()),
            local_path: r.local_path,
            width: r.width.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            height: r.height.unwrap_or_else(|| String::new()).parse().unwrap_or(0),
            seed: r.seed,
            steps: r.steps.map(|s| s as i32),
            cfg_scale: r.cfg_scale,
            style: r.style,
            generation_time: r.generation_time,
            cost_credits: r.cost_credits.map(|c| c as f64),
            metadata: r.metadata,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get total cost for generated images by project
    pub async fn get_total_cost_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<f64> {
        let row = sqlx::query!(
            "SELECT COALESCE(SUM(cost_credits), 0.0) as total_cost FROM generated_images WHERE project_id = ?",
            project_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total cost for project: {}", e)))?;

        Ok(row.total_cost)
    }
}
