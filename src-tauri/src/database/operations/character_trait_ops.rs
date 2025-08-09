use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Character trait operations
impl super::CharacterTraitOps {
    /// Create a new character trait
    pub async fn create(pool: &Pool<Sqlite>, character_trait: CharacterTrait) -> Result<CharacterTrait> {
        let mut character_trait = character_trait;
        character_trait.id = Uuid::new_v4().to_string();
        character_trait.created_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO character_traits (id, character_id, trait_name, trait_value, is_visible, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&character_trait.id)
        .bind(&character_trait.character_id)
        .bind(&character_trait.trait_name)
        .bind(&character_trait.trait_value)
        .bind(character_trait.is_visible)
        .bind(character_trait.created_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create character trait: {}", e)))?;
        
        Ok(character_trait)
    }
    
    /// Get all traits for a character
    pub async fn get_by_character(pool: &Pool<Sqlite>, character_id: &str) -> Result<Vec<CharacterTrait>> {
        let traits = sqlx::query_as::<_, CharacterTrait>(
            r#"
            SELECT id, character_id, trait_name, trait_value, is_visible, created_at
            FROM character_traits
            WHERE character_id = ?
            ORDER BY trait_name
            "#,
        )
        .bind(character_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get character traits: {}", e)))?;
        
        Ok(traits)
    }
    
    /// Get visible traits for a character
    pub async fn get_visible_by_character(pool: &Pool<Sqlite>, character_id: &str) -> Result<Vec<CharacterTrait>> {
        let traits = sqlx::query_as::<_, CharacterTrait>(
            r#"
            SELECT id, character_id, trait_name, trait_value, is_visible, created_at
            FROM character_traits
            WHERE character_id = ? AND is_visible = true
            ORDER BY trait_name
            "#,
        )
        .bind(character_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get visible character traits: {}", e)))?;
        
        Ok(traits)
    }
    
    /// Get all character traits for a project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<CharacterTrait>> {
        let traits = sqlx::query_as::<_, CharacterTrait>(
            r#"
            SELECT ct.id, ct.character_id, ct.trait_name, ct.trait_value, ct.is_visible, ct.created_at
            FROM character_traits ct
            JOIN characters c ON ct.character_id = c.id
            WHERE c.project_id = ?
            ORDER BY c.name, ct.trait_name
            "#,
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get character traits for project: {}", e)))?;
        
        Ok(traits)
    }
    
    /// Get a character trait by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<CharacterTrait> {
        let character_trait = sqlx::query_as::<_, CharacterTrait>(
            r#"
            SELECT id, character_id, trait_name, trait_value, is_visible, created_at
            FROM character_traits
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get character trait: {}", e)))?;
        
        Ok(character_trait)
    }
    
    /// Update a character trait
    pub async fn update(pool: &Pool<Sqlite>, character_trait: CharacterTrait) -> Result<CharacterTrait> {
        sqlx::query(
            r#"
            UPDATE character_traits SET
                trait_name = ?, trait_value = ?, is_visible = ?
            WHERE id = ?
            "#,
        )
        .bind(&character_trait.trait_name)
        .bind(&character_trait.trait_value)
        .bind(character_trait.is_visible)
        .bind(&character_trait.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update character trait: {}", e)))?;
        
        Ok(character_trait)
    }
    
    /// Update trait visibility
    pub async fn update_visibility(pool: &Pool<Sqlite>, id: &str, is_visible: bool) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE character_traits SET is_visible = ?
            WHERE id = ?
            "#,
        )
        .bind(is_visible)
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update character trait visibility: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a character trait
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM character_traits WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete character trait: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete all traits for a character
    pub async fn delete_by_character(pool: &Pool<Sqlite>, character_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM character_traits WHERE character_id = ?
            "#,
        )
        .bind(character_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete character traits: {}", e)))?;
        
        Ok(())
    }
    
    /// Bulk create character traits
    pub async fn bulk_create(pool: &Pool<Sqlite>, traits: Vec<CharacterTrait>) -> Result<Vec<CharacterTrait>> {
        let mut created_traits = Vec::new();
        
        for mut trait_item in traits {
            trait_item.id = Uuid::new_v4().to_string();
            trait_item.created_at = Utc::now();
            
            sqlx::query(
                r#"
                INSERT INTO character_traits (id, character_id, trait_name, trait_value, is_visible, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&trait_item.id)
            .bind(&trait_item.character_id)
            .bind(&trait_item.trait_name)
            .bind(&trait_item.trait_value)
            .bind(trait_item.is_visible)
            .bind(trait_item.created_at)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create character trait: {}", e)))?;
            
            created_traits.push(trait_item);
        }
        
        Ok(created_traits)
    }
}
