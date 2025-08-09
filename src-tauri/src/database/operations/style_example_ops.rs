use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};

/// Style example operations
impl super::StyleExampleOps {
    /// Create a new style example
    pub async fn create(pool: &Pool<Sqlite>, mut style_example: StyleExample) -> Result<StyleExample> {
        style_example.created_at = Utc::now();
        
        let result = sqlx::query(
            r#"
            INSERT INTO style_examples (project_id, user_id, example_text, analysis_result, generated_style_prompt, word_count, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&style_example.project_id)
        .bind(&style_example.user_id)
        .bind(&style_example.example_text)
        .bind(&style_example.analysis_result)
        .bind(&style_example.generated_style_prompt)
        .bind(style_example.word_count)
        .bind(style_example.created_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create style example: {}", e)))?;
        
        style_example.id = result.last_insert_rowid();
        Ok(style_example)
    }
    
    /// Get all style examples for a project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<StyleExample>> {
        let examples = sqlx::query_as::<_, StyleExample>(
            r#"
            SELECT id, project_id, user_id, example_text, analysis_result, generated_style_prompt, word_count, created_at
            FROM style_examples
            WHERE project_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get style examples: {}", e)))?;
        
        Ok(examples)
    }
    
    /// Get a style example by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<StyleExample> {
        let style_example = sqlx::query_as::<_, StyleExample>(
            r#"
            SELECT id, project_id, user_id, example_text, analysis_result, generated_style_prompt, word_count, created_at
            FROM style_examples
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get style example: {}", e)))?;
        
        Ok(style_example)
    }
    
    /// Update a style example
    pub async fn update(pool: &Pool<Sqlite>, style_example: &StyleExample) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE style_examples
            SET project_id = ?, user_id = ?, example_text = ?, analysis_result = ?, generated_style_prompt = ?, word_count = ?
            WHERE id = ?
            "#,
        )
        .bind(&style_example.project_id)
        .bind(&style_example.user_id)
        .bind(&style_example.example_text)
        .bind(&style_example.analysis_result)
        .bind(&style_example.generated_style_prompt)
        .bind(style_example.word_count)
        .bind(style_example.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update style example: {}", e)))?;
        
        Ok(())
    }
    
    /// Update analysis result for a style example
    pub async fn update_analysis(pool: &Pool<Sqlite>, id: i64, analysis_result: &str, generated_style_prompt: Option<&str>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE style_examples
            SET analysis_result = ?, generated_style_prompt = ?
            WHERE id = ?
            "#,
        )
        .bind(analysis_result)
        .bind(generated_style_prompt)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update style example analysis: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a style example
    pub async fn delete(pool: &Pool<Sqlite>, id: i64) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM style_examples WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete style example: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete all style examples for a project
    pub async fn delete_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM style_examples WHERE project_id = ?
            "#,
        )
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete style examples for project: {}", e)))?;
        
        Ok(())
    }
    
    /// Get style examples with analysis results for a project
    pub async fn get_analyzed_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<StyleExample>> {
        let examples = sqlx::query_as::<_, StyleExample>(
            r#"
            SELECT id, project_id, user_id, example_text, analysis_result, generated_style_prompt, word_count, created_at
            FROM style_examples
            WHERE project_id = ? AND analysis_result IS NOT NULL
            ORDER BY created_at DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get analyzed style examples: {}", e)))?;
        
        Ok(examples)
    }
}
