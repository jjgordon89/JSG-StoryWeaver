use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Character operations
impl super::CharacterOps {
    /// Create a new character
    pub async fn create(pool: &Pool<Sqlite>, mut character: Character) -> Result<Character> {
        character.id = Uuid::new_v4().to_string();
        character.created_at = Utc::now();
        character.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO characters (id, project_id, name, description, role, age, appearance,
                                  personality, background, goals, relationships, visibility,
                                  created_at, updated_at, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&character.id)
        .bind(&character.project_id)
        .bind(&character.name)
        .bind(&character.description)
        .bind(&character.role)
        .bind(character.age)
        .bind(&character.appearance)
        .bind(&character.personality)
        .bind(&character.background)
        .bind(&character.goals)
        .bind(&character.relationships)
        .bind(&character.visibility)
        .bind(character.created_at)
        .bind(character.updated_at)
        .bind(&character.metadata)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create character: {}", e)))?;
        
        Ok(character)
    }
    
    /// Get characters by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Character>> {
        let characters = sqlx::query_as::<_, Character>(
            "SELECT * FROM characters WHERE project_id = ? ORDER BY name"
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get characters: {}", e)))?;
        
        Ok(characters)
    }
    
    /// Update a character
    pub async fn update(pool: &Pool<Sqlite>, character: &Character) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE characters SET name = ?, description = ?, role = ?, age = ?, appearance = ?,
                                personality = ?, background = ?, goals = ?, relationships = ?,
                                visibility = ?, updated_at = ?, metadata = ?
            WHERE id = ?
            "#,
        )
        .bind(&character.name)
        .bind(&character.description)
        .bind(&character.role)
        .bind(character.age)
        .bind(&character.appearance)
        .bind(&character.personality)
        .bind(&character.background)
        .bind(&character.goals)
        .bind(&character.relationships)
        .bind(&character.visibility)
        .bind(Utc::now())
        .bind(&character.metadata)
        .bind(&character.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update character: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a character
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM characters WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete character: {}", e)))?;
        
        Ok(())
    }
}
