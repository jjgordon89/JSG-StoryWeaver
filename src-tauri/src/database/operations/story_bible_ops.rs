use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite, Row};
use uuid::Uuid;
use serde_json;

/// Story Bible operations
impl super::StoryBibleOps {
    /// Create or update story bible for a project
    pub async fn create_or_update(pool: &Pool<Sqlite>, story_bible: StoryBible) -> Result<StoryBible> {
        let mut story_bible = story_bible;
        
        // Check if story bible already exists for this project
        let existing = Self::get_by_project(&*pool, &story_bible.project_id).await;
        
        if existing.is_ok() {
            // Update existing
            story_bible.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                UPDATE story_bible SET 
                    braindump = ?, synopsis = ?, genre = ?, style = ?, style_examples = ?,
                    pov_mode = ?, global_pov = ?, global_tense = ?, global_character_pov_ids = ?,
                    updated_at = ?
                WHERE project_id = ?
                "#,
            )
            .bind(&story_bible.braindump)
            .bind(&story_bible.synopsis)
            .bind(&story_bible.genre)
            .bind(&story_bible.style)
            .bind(&story_bible.style_examples)
            .bind(&story_bible.pov_mode)
            .bind(&story_bible.global_pov)
            .bind(&story_bible.global_tense)
            .bind(serde_json::to_string(&story_bible.global_character_pov_ids).unwrap_or_default())
            .bind(story_bible.updated_at)
            .bind(&story_bible.project_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to update story bible: {}", e)))?;
        } else {
            // Create new
            story_bible.id = Uuid::new_v4().to_string();
            story_bible.created_at = Utc::now();
            story_bible.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                INSERT INTO story_bible (id, project_id, braindump, synopsis, genre, style, style_examples,
                                       pov_mode, global_pov, global_tense, global_character_pov_ids,
                                       created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&story_bible.id)
            .bind(&story_bible.project_id)
            .bind(&story_bible.braindump)
            .bind(&story_bible.synopsis)
            .bind(&story_bible.genre)
            .bind(&story_bible.style)
            .bind(&story_bible.style_examples)
            .bind(&story_bible.pov_mode)
            .bind(&story_bible.global_pov)
            .bind(&story_bible.global_tense)
            .bind(serde_json::to_string(&story_bible.global_character_pov_ids).unwrap_or_default())
            .bind(story_bible.created_at)
            .bind(story_bible.updated_at)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create story bible: {}", e)))?;
        }
        
        Ok(story_bible)
    }
    
    /// Get story bible by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<StoryBible> {
        let row = sqlx::query(
            "SELECT * FROM story_bible WHERE project_id = ?"
        )
        .bind(project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get story bible: {}", e)))?;
        
        let global_character_pov_ids: String = row.get("global_character_pov_ids");
        
        Ok(StoryBible {
            id: row.get("id"),
            project_id: row.get("project_id"),
            braindump: row.get("braindump"),
            synopsis: row.get("synopsis"),
            genre: row.get("genre"),
            style: row.get("style"),
            style_examples: row.get("style_examples"),
            pov_mode: row.get("pov_mode"),
            global_pov: row.get("global_pov"),
            global_tense: row.get("global_tense"),
            global_character_pov_ids,
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    /// Update braindump
    pub async fn update_braindump(pool: &Pool<Sqlite>, project_id: &str, braindump: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE story_bible SET braindump = ?, updated_at = ? WHERE project_id = ?"
        )
        .bind(braindump)
        .bind(Utc::now())
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update braindump: {}", e)))?;
        
        Ok(())
    }
    
    /// Update synopsis
    pub async fn update_synopsis(pool: &Pool<Sqlite>, project_id: &str, synopsis: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE story_bible SET synopsis = ?, updated_at = ? WHERE project_id = ?"
        )
        .bind(synopsis)
        .bind(Utc::now())
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update synopsis: {}", e)))?;
        
        Ok(())
    }
    
    /// Update genre
    pub async fn update_genre(pool: &Pool<Sqlite>, project_id: &str, genre: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE story_bible SET genre = ?, updated_at = ? WHERE project_id = ?"
        )
        .bind(genre)
        .bind(Utc::now())
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update genre: {}", e)))?;
        
        Ok(())
    }
    
    /// Update style and style examples
    pub async fn update_style(pool: &Pool<Sqlite>, project_id: &str, style: Option<String>, style_examples: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE story_bible SET style = ?, style_examples = ?, updated_at = ? WHERE project_id = ?"
        )
        .bind(style)
        .bind(style_examples)
        .bind(Utc::now())
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update style: {}", e)))?;
        
        Ok(())
    }
    
    /// Update POV settings
    pub async fn update_pov_settings(
        pool: &Pool<Sqlite>, 
        project_id: &str, 
        pov_mode: Option<String>,
        global_pov: Option<String>,
        global_tense: Option<String>,
        global_character_pov_ids: Option<Vec<String>>
    ) -> Result<()> {
        let character_pov_ids_json = global_character_pov_ids
            .map(|ids| serde_json::to_string(&ids).unwrap_or_default());
            
        sqlx::query(
            r#"
            UPDATE story_bible SET 
                pov_mode = ?, global_pov = ?, global_tense = ?, 
                global_character_pov_ids = ?, updated_at = ?
            WHERE project_id = ?
            "#
        )
        .bind(pov_mode)
        .bind(global_pov)
        .bind(global_tense)
        .bind(character_pov_ids_json)
        .bind(Utc::now())
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update POV settings: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete story bible
    pub async fn delete(pool: &Pool<Sqlite>, project_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM story_bible WHERE project_id = ?")
            .bind(project_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete story bible: {}", e)))?;
        
        Ok(())
    }
}
