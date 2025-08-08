use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Outline operations
pub struct OutlineOps;

impl OutlineOps {
    /// Create a new outline
    pub async fn create(pool: &Pool<Sqlite>, outline: Outline) -> Result<Outline> {
        let mut outline = outline;
        outline.id = Uuid::new_v4().to_string();
        outline.created_at = Utc::now();
        outline.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO outlines (id, project_id, chapter_number, title, summary, pov, tense, 
                                character_pov_ids, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&outline.id)
        .bind(&outline.project_id)
        .bind(outline.chapter_number)
        .bind(&outline.title)
        .bind(&outline.summary)
        .bind(&outline.pov)
        .bind(&outline.tense)
        .bind(&outline.character_pov_ids)
        .bind(outline.created_at)
        .bind(outline.updated_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create outline: {}", e)))?;
        
        Ok(outline)
    }
    
    /// Get all outlines for a project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Outline>> {
        let outlines = sqlx::query_as::<_, Outline>(
            r#"
            SELECT id, project_id, chapter_number, title, summary, pov, tense, 
                   character_pov_ids, created_at, updated_at
            FROM outlines
            WHERE project_id = ?
            ORDER BY chapter_number
            "#,
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outlines: {}", e)))?;
        
        Ok(outlines)
    }
    
    /// Get an outline by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Outline> {
        let outline = sqlx::query_as::<_, Outline>(
            r#"
            SELECT id, project_id, chapter_number, title, summary, pov, tense, 
                   character_pov_ids, created_at, updated_at
            FROM outlines
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outline: {}", e)))?;
        
        Ok(outline)
    }
    
    /// Get outline by chapter number
    pub async fn get_by_chapter(pool: &Pool<Sqlite>, project_id: &str, chapter_number: i32) -> Result<Option<Outline>> {
        let outline = sqlx::query_as::<_, Outline>(
            r#"
            SELECT id, project_id, chapter_number, title, summary, pov, tense, 
                   character_pov_ids, created_at, updated_at
            FROM outlines
            WHERE project_id = ? AND chapter_number = ?
            "#,
        )
        .bind(project_id)
        .bind(chapter_number)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outline by chapter: {}", e)))?;
        
        Ok(outline)
    }
    
    /// Update an outline
    pub async fn update(pool: &Pool<Sqlite>, outline: Outline) -> Result<Outline> {
        let mut outline = outline;
        outline.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE outlines SET
                chapter_number = ?, title = ?, summary = ?, pov = ?, tense = ?, 
                character_pov_ids = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(outline.chapter_number)
        .bind(&outline.title)
        .bind(&outline.summary)
        .bind(&outline.pov)
        .bind(&outline.tense)
        .bind(&outline.character_pov_ids)
        .bind(outline.updated_at)
        .bind(&outline.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update outline: {}", e)))?;
        
        Ok(outline)
    }
    
    /// Update outline POV settings
    pub async fn update_pov_settings(pool: &Pool<Sqlite>, id: &str, pov: Option<String>, tense: Option<String>, character_pov_ids: Option<String>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE outlines SET pov = ?, tense = ?, character_pov_ids = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(pov)
        .bind(tense)
        .bind(character_pov_ids)
        .bind(Utc::now())
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update outline POV settings: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete an outline
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        // First delete associated acts and scenes
        sqlx::query("DELETE FROM scenes WHERE outline_id IN (SELECT id FROM outline_acts WHERE outline_id = ?)")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete outline scenes: {}", e)))?;
        
        sqlx::query("DELETE FROM outline_acts WHERE outline_id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete outline acts: {}", e)))?;
        
        // Then delete the outline
        sqlx::query("DELETE FROM outlines WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete outline: {}", e)))?;
        
        Ok(())
    }
    
    /// Get outlines by character POV
    pub async fn get_by_character_pov(pool: &Pool<Sqlite>, project_id: &str, character_id: &str) -> Result<Vec<Outline>> {
        let outlines = sqlx::query_as::<_, Outline>(
            r#"
            SELECT id, project_id, chapter_number, title, summary, pov, tense, 
                   character_pov_ids, created_at, updated_at
            FROM outlines
            WHERE project_id = ? AND character_pov_ids LIKE ?
            ORDER BY chapter_number
            "#,
        )
        .bind(project_id)
        .bind(format!("%{}%", character_id))
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outlines by character POV: {}", e)))?;
        
        Ok(outlines)
    }
    
    /// Search outlines by title or summary
    pub async fn search(pool: &Pool<Sqlite>, project_id: &str, query: &str) -> Result<Vec<Outline>> {
        let search_query = format!("%{}%", query);
        
        let outlines = sqlx::query_as::<_, Outline>(
            r#"
            SELECT id, project_id, chapter_number, title, summary, pov, tense, 
                   character_pov_ids, created_at, updated_at
            FROM outlines
            WHERE project_id = ? AND (title LIKE ? OR summary LIKE ?)
            ORDER BY chapter_number
            "#,
        )
        .bind(project_id)
        .bind(&search_query)
        .bind(&search_query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to search outlines: {}", e)))?;
        
        Ok(outlines)
    }
    
    /// Get chapter count for project
    pub async fn get_chapter_count(pool: &Pool<Sqlite>, project_id: &str) -> Result<i32> {
        let count: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM outlines WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get chapter count: {}", e)))?;
        
        Ok(count.0)
    }
    
    /// Get next available chapter number
    pub async fn get_next_chapter_number(pool: &Pool<Sqlite>, project_id: &str) -> Result<i32> {
        let max_chapter: (Option<i32>,) = sqlx::query_as(
            "SELECT MAX(chapter_number) FROM outlines WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get max chapter number: {}", e)))?;
        
        Ok(max_chapter.0.unwrap_or(0) + 1)
    }
    
    /// Reorder chapters
    pub async fn reorder_chapters(pool: &Pool<Sqlite>, project_id: &str, chapter_orders: Vec<(String, i32)>) -> Result<()> {
        for (outline_id, new_chapter_number) in chapter_orders {
            sqlx::query(
                "UPDATE outlines SET chapter_number = ?, updated_at = ? WHERE id = ? AND project_id = ?"
            )
            .bind(new_chapter_number)
            .bind(Utc::now())
            .bind(&outline_id)
            .bind(project_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to reorder chapter: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Bulk create outlines
    pub async fn bulk_create(pool: &Pool<Sqlite>, outlines: Vec<Outline>) -> Result<Vec<Outline>> {
        let mut created_outlines = Vec::new();
        
        for mut outline in outlines {
            outline.id = Uuid::new_v4().to_string();
            outline.created_at = Utc::now();
            outline.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                INSERT INTO outlines (id, project_id, chapter_number, title, summary, pov, tense, 
                                    character_pov_ids, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&outline.id)
            .bind(&outline.project_id)
            .bind(outline.chapter_number)
            .bind(&outline.title)
            .bind(&outline.summary)
            .bind(&outline.pov)
            .bind(&outline.tense)
            .bind(&outline.character_pov_ids)
            .bind(outline.created_at)
            .bind(outline.updated_at)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create outline: {}", e)))?;
            
            created_outlines.push(outline);
        }
        
        Ok(created_outlines)
    }
}
