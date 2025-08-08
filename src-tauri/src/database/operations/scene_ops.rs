use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Scene operations
pub struct SceneOps;

impl SceneOps {
    /// Create a new scene
    pub async fn create(pool: &Pool<Sqlite>, scene: Scene) -> Result<Scene> {
        let mut scene = scene;
        scene.id = Uuid::new_v4().to_string();
        scene.created_at = Utc::now();
        scene.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO scenes (id, outline_id, scene_number, title, summary, extra_instructions, 
                              pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                              is_validated, validation_issues, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&scene.id)
        .bind(&scene.outline_id)
        .bind(scene.scene_number)
        .bind(&scene.title)
        .bind(&scene.summary)
        .bind(&scene.extra_instructions)
        .bind(&scene.pov)
        .bind(&scene.tense)
        .bind(&scene.character_pov_ids)
        .bind(scene.word_count_estimate)
        .bind(scene.credit_estimate)
        .bind(scene.is_validated)
        .bind(&scene.validation_issues)
        .bind(scene.created_at)
        .bind(scene.updated_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create scene: {}", e)))?;
        
        Ok(scene)
    }
    
    /// Get all scenes for an outline
    pub async fn get_by_outline(pool: &Pool<Sqlite>, outline_id: &str) -> Result<Vec<Scene>> {
        let scenes = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE outline_id = ?
            ORDER BY scene_number
            "#,
        )
        .bind(outline_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get scenes: {}", e)))?;
        
        Ok(scenes)
    }
    
    /// Get a scene by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Scene> {
        let scene = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get scene: {}", e)))?;
        
        Ok(scene)
    }
    
    /// Get scene by scene number
    pub async fn get_by_scene_number(pool: &Pool<Sqlite>, outline_id: &str, scene_number: i32) -> Result<Option<Scene>> {
        let scene = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE outline_id = ? AND scene_number = ?
            "#,
        )
        .bind(outline_id)
        .bind(scene_number)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get scene by number: {}", e)))?;
        
        Ok(scene)
    }
    
    /// Update a scene
    pub async fn update(pool: &Pool<Sqlite>, scene: Scene) -> Result<Scene> {
        let mut scene = scene;
        scene.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE scenes SET
                scene_number = ?, title = ?, summary = ?, extra_instructions = ?, 
                pov = ?, tense = ?, character_pov_ids = ?, word_count_estimate = ?, 
                credit_estimate = ?, is_validated = ?, validation_issues = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(scene.scene_number)
        .bind(&scene.title)
        .bind(&scene.summary)
        .bind(&scene.extra_instructions)
        .bind(&scene.pov)
        .bind(&scene.tense)
        .bind(&scene.character_pov_ids)
        .bind(scene.word_count_estimate)
        .bind(scene.credit_estimate)
        .bind(scene.is_validated)
        .bind(&scene.validation_issues)
        .bind(scene.updated_at)
        .bind(&scene.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update scene: {}", e)))?;
        
        Ok(scene)
    }
    
    /// Update scene POV settings
    pub async fn update_pov_settings(pool: &Pool<Sqlite>, id: &str, pov: Option<String>, tense: Option<String>, character_pov_ids: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE scenes SET pov = ?, tense = ?, character_pov_ids = ?, updated_at = ? WHERE id = ?"
        )
        .bind(pov)
        .bind(tense)
        .bind(character_pov_ids)
        .bind(Utc::now())
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update scene POV settings: {}", e)))?;
        
        Ok(())
    }
    
    /// Update scene validation status
    pub async fn update_validation(pool: &Pool<Sqlite>, id: &str, is_validated: bool, validation_issues: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE scenes SET is_validated = ?, validation_issues = ?, updated_at = ? WHERE id = ?"
        )
        .bind(is_validated)
        .bind(validation_issues)
        .bind(Utc::now())
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update scene validation: {}", e)))?;
        
        Ok(())
    }
    
    /// Update scene estimates
    pub async fn update_estimates(pool: &Pool<Sqlite>, id: &str, word_count_estimate: Option<i32>, credit_estimate: Option<f64>) -> Result<()> {
        sqlx::query(
            "UPDATE scenes SET word_count_estimate = ?, credit_estimate = ?, updated_at = ? WHERE id = ?"
        )
        .bind(word_count_estimate)
        .bind(credit_estimate)
        .bind(Utc::now())
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update scene estimates: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a scene
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM scenes WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete scene: {}", e)))?;
        
        Ok(())
    }
    
    /// Get scenes by character POV
    pub async fn get_by_character_pov(pool: &Pool<Sqlite>, outline_id: &str, character_id: &str) -> Result<Vec<Scene>> {
        let scenes = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE outline_id = ? AND character_pov_ids LIKE ?
            ORDER BY scene_number
            "#,
        )
        .bind(outline_id)
        .bind(format!("%{}%", character_id))
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get scenes by character POV: {}", e)))?;
        
        Ok(scenes)
    }
    
    /// Get validated scenes
    pub async fn get_validated(pool: &Pool<Sqlite>, outline_id: &str) -> Result<Vec<Scene>> {
        let scenes = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE outline_id = ? AND is_validated = true
            ORDER BY scene_number
            "#,
        )
        .bind(outline_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get validated scenes: {}", e)))?;
        
        Ok(scenes)
    }
    
    /// Get unvalidated scenes
    pub async fn get_unvalidated(pool: &Pool<Sqlite>, outline_id: &str) -> Result<Vec<Scene>> {
        let scenes = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE outline_id = ? AND is_validated = false
            ORDER BY scene_number
            "#,
        )
        .bind(outline_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get unvalidated scenes: {}", e)))?;
        
        Ok(scenes)
    }
    
    /// Search scenes by title or summary
    pub async fn search(pool: &Pool<Sqlite>, outline_id: &str, query: &str) -> Result<Vec<Scene>> {
        let search_query = format!("%{}%", query);
        
        let scenes = sqlx::query_as::<_, Scene>(
            r#"
            SELECT id, outline_id, scene_number, title, summary, extra_instructions, 
                   pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                   is_validated, validation_issues, created_at, updated_at
            FROM scenes
            WHERE outline_id = ? AND (title LIKE ? OR summary LIKE ?)
            ORDER BY scene_number
            "#,
        )
        .bind(outline_id)
        .bind(&search_query)
        .bind(&search_query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to search scenes: {}", e)))?;
        
        Ok(scenes)
    }
    
    /// Get scene count for outline
    pub async fn get_scene_count(pool: &Pool<Sqlite>, outline_id: &str) -> Result<i32> {
        let count: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM scenes WHERE outline_id = ?"
        )
        .bind(outline_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get scene count: {}", e)))?;
        
        Ok(count.0)
    }
    
    /// Get next available scene number
    pub async fn get_next_scene_number(pool: &Pool<Sqlite>, outline_id: &str) -> Result<i32> {
        let max_scene: (Option<i32>,) = sqlx::query_as(
            "SELECT MAX(scene_number) FROM scenes WHERE outline_id = ?"
        )
        .bind(outline_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get max scene number: {}", e)))?;
        
        Ok(max_scene.0.unwrap_or(0) + 1)
    }
    
    /// Reorder scenes
    pub async fn reorder_scenes(pool: &Pool<Sqlite>, outline_id: &str, scene_orders: Vec<(String, i32)>) -> Result<()> {
        for (scene_id, new_scene_number) in scene_orders {
            sqlx::query(
                "UPDATE scenes SET scene_number = ?, updated_at = ? WHERE id = ? AND outline_id = ?"
            )
            .bind(new_scene_number)
            .bind(Utc::now())
            .bind(&scene_id)
            .bind(outline_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to reorder scene: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Get total word count estimate for outline
    pub async fn get_total_word_count_estimate(pool: &Pool<Sqlite>, outline_id: &str) -> Result<i32> {
        let total: (Option<i64>,) = sqlx::query_as(
            "SELECT SUM(word_count_estimate) FROM scenes WHERE outline_id = ?"
        )
        .bind(outline_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total word count estimate: {}", e)))?;
        
        Ok(total.0.unwrap_or(0) as i32)
    }
    
    /// Get total credit estimate for outline
    pub async fn get_total_credit_estimate(pool: &Pool<Sqlite>, outline_id: &str) -> Result<f64> {
        let total: (Option<f64>,) = sqlx::query_as(
            "SELECT SUM(credit_estimate) FROM scenes WHERE outline_id = ?"
        )
        .bind(outline_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total credit estimate: {}", e)))?;
        
        Ok(total.0.unwrap_or(0.0))
    }
    
    /// Bulk create scenes
    pub async fn bulk_create(pool: &Pool<Sqlite>, scenes: Vec<Scene>) -> Result<Vec<Scene>> {
        let mut created_scenes = Vec::new();
        
        for mut scene in scenes {
            scene.id = Uuid::new_v4().to_string();
            scene.created_at = Utc::now();
            scene.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                INSERT INTO scenes (id, outline_id, scene_number, title, summary, extra_instructions, 
                                  pov, tense, character_pov_ids, word_count_estimate, credit_estimate, 
                                  is_validated, validation_issues, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&scene.id)
            .bind(&scene.outline_id)
            .bind(scene.scene_number)
            .bind(&scene.title)
            .bind(&scene.summary)
            .bind(&scene.extra_instructions)
            .bind(&scene.pov)
            .bind(&scene.tense)
            .bind(&scene.character_pov_ids)
            .bind(scene.word_count_estimate)
            .bind(scene.credit_estimate)
            .bind(scene.is_validated)
            .bind(&scene.validation_issues)
            .bind(scene.created_at)
            .bind(scene.updated_at)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create scene: {}", e)))?;
            
            created_scenes.push(scene);
        }
        
        Ok(created_scenes)
    }
    
    /// Delete all scenes for an outline
    pub async fn delete_by_outline(pool: &Pool<Sqlite>, outline_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM scenes WHERE outline_id = ?")
            .bind(outline_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete scenes by outline: {}", e)))?;
        
        Ok(())
    }
}
