use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Project operations
impl super::ProjectOps {
    /// Create a new project
    pub async fn create(pool: &Pool<Sqlite>, mut project: Project) -> Result<Project> {
        project.id = Uuid::new_v4().to_string();
        project.created_at = Utc::now();
        project.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, description, genre, target_word_count, 
                                current_word_count, status, created_at, updated_at, settings)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.genre)
        .bind(project.target_word_count)
        .bind(project.current_word_count)
        .bind(&project.status)
        .bind(project.created_at)
        .bind(project.updated_at)
        .bind(&project.settings)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create project: {}", e)))?;
        
        Ok(project)
    }
    
    /// Get a project by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<Project>> {
        let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get project: {}", e)))?;
        
        Ok(project)
    }
    
    /// Get all projects
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Project>> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects ORDER BY updated_at DESC"
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get projects: {}", e)))?;
        
        Ok(projects)
    }
    
    /// Update a project
    pub async fn update(pool: &Pool<Sqlite>, project: &Project) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE projects SET name = ?, description = ?, genre = ?, target_word_count = ?,
                              current_word_count = ?, status = ?, updated_at = ?, settings = ?
            WHERE id = ?
            "#,
        )
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.genre)
        .bind(project.target_word_count)
        .bind(project.current_word_count)
        .bind(&project.status)
        .bind(Utc::now())
        .bind(&project.settings)
        .bind(&project.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update project: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a project
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete project: {}", e)))?;
        
        Ok(())
    }
    
    /// Update word count for a project
    pub async fn update_word_count(pool: &Pool<Sqlite>, project_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE projects SET current_word_count = (
                SELECT COALESCE(SUM(word_count), 0) FROM documents WHERE project_id = ?
            ), updated_at = ? WHERE id = ?
            "#,
        )
        .bind(project_id)
        .bind(Utc::now())
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update word count: {}", e)))?;
        
        Ok(())
    }
}
