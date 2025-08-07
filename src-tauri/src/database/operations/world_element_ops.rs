use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// World element operations
pub struct WorldElementOps;

impl WorldElementOps {
    /// Create a new world element
    pub async fn create(pool: &Pool<Sqlite>, world_element: WorldElement) -> Result<WorldElement> {
        let mut world_element = world_element;
        world_element.id = Uuid::new_v4().to_string();
        world_element.created_at = Utc::now();
        world_element.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO worldbuilding (id, project_id, series_id, name, description, element_type, 
                                     properties, is_visible, original_project_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&world_element.id)
        .bind(&world_element.project_id)
        .bind(&world_element.series_id)
        .bind(&world_element.name)
        .bind(&world_element.description)
        .bind(&world_element.element_type)
        .bind(&world_element.properties)
        .bind(world_element.is_visible)
        .bind(&world_element.original_project_id)
        .bind(world_element.created_at)
        .bind(world_element.updated_at)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create world element: {}", e)))?;
        
        Ok(world_element)
    }
    
    /// Get all world elements for a project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<WorldElement>> {
        let elements = sqlx::query_as::<_, WorldElement>(
            r#"
            SELECT id, project_id, series_id, name, description, element_type, 
                   properties, is_visible, original_project_id, created_at, updated_at
            FROM worldbuilding
            WHERE project_id = ?
            ORDER BY name
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get world elements: {}", e)))?;
        
        Ok(elements)
    }
    
    /// Get all world elements for a series
    pub async fn get_by_series(pool: &Pool<Sqlite>, series_id: &str) -> Result<Vec<WorldElement>> {
        let elements = sqlx::query_as::<_, WorldElement>(
            r#"
            SELECT id, project_id, series_id, name, description, element_type, 
                   properties, is_visible, original_project_id, created_at, updated_at
            FROM worldbuilding
            WHERE series_id = ?
            ORDER BY name
            "#,
        )
        .bind(series_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get series world elements: {}", e)))?;
        
        Ok(elements)
    }
    
    /// Get visible world elements for a project
    pub async fn get_visible_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<WorldElement>> {
        let elements = sqlx::query_as::<_, WorldElement>(
            r#"
            SELECT id, project_id, series_id, name, description, element_type, 
                   properties, is_visible, original_project_id, created_at, updated_at
            FROM worldbuilding
            WHERE project_id = ? AND is_visible = true
            ORDER BY name
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get visible world elements: {}", e)))?;
        
        Ok(elements)
    }
    
    /// Get world elements by type
    pub async fn get_by_type(pool: &Pool<Sqlite>, project_id: &str, element_type: &str) -> Result<Vec<WorldElement>> {
        let elements = sqlx::query_as::<_, WorldElement>(
            r#"
            SELECT id, project_id, series_id, name, description, element_type, 
                   properties, is_visible, original_project_id, created_at, updated_at
            FROM worldbuilding
            WHERE project_id = ? AND element_type = ?
            ORDER BY name
            "#,
        )
        .bind(project_id)
        .bind(element_type)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get world elements by type: {}", e)))?;
        
        Ok(elements)
    }
    
    /// Get a world element by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<WorldElement> {
        let element = sqlx::query_as::<_, WorldElement>(
            r#"
            SELECT id, project_id, series_id, name, description, element_type, 
                   properties, is_visible, original_project_id, created_at, updated_at
            FROM worldbuilding
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get world element: {}", e)))?;
        
        Ok(element)
    }
    
    /// Update a world element
    pub async fn update(pool: &Pool<Sqlite>, world_element: WorldElement) -> Result<WorldElement> {
        let mut world_element = world_element;
        world_element.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE worldbuilding SET
                name = ?, description = ?, element_type = ?, properties = ?, 
                is_visible = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&world_element.name)
        .bind(&world_element.description)
        .bind(&world_element.element_type)
        .bind(&world_element.properties)
        .bind(world_element.is_visible)
        .bind(world_element.updated_at)
        .bind(&world_element.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update world element: {}", e)))?;
        
        Ok(world_element)
    }
    
    /// Update element visibility
    pub async fn update_visibility(pool: &Pool<Sqlite>, id: &str, is_visible: bool) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE worldbuilding SET is_visible = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(is_visible)
        .bind(Utc::now())
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update world element visibility: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a world element
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM worldbuilding WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete world element: {}", e)))?;
        
        Ok(())
    }
    
    /// Share world element to series
    pub async fn share_to_series(pool: &Pool<Sqlite>, id: &str, series_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE worldbuilding SET series_id = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(series_id)
        .bind(Utc::now())
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to share world element to series: {}", e)))?;
        
        Ok(())
    }
    
    /// Unshare world element from series
    pub async fn unshare_from_series(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE worldbuilding SET series_id = NULL, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(Utc::now())
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to unshare world element from series: {}", e)))?;
        
        Ok(())
    }
    
    /// Search world elements by name or description
    pub async fn search(pool: &Pool<Sqlite>, project_id: &str, query: &str) -> Result<Vec<WorldElement>> {
        let search_query = format!("%{}%", query);
        
        let elements = sqlx::query_as::<_, WorldElement>(
            r#"
            SELECT id, project_id, series_id, name, description, element_type, 
                   properties, is_visible, original_project_id, created_at, updated_at
            FROM worldbuilding
            WHERE project_id = ? AND (name LIKE ? OR description LIKE ?)
            ORDER BY name
            "#,
        )
        .bind(project_id)
        .bind(&search_query)
        .bind(&search_query)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to search world elements: {}", e)))?;
        
        Ok(elements)
    }
    
    /// Bulk create world elements
    pub async fn bulk_create(pool: &Pool<Sqlite>, elements: Vec<WorldElement>) -> Result<Vec<WorldElement>> {
        let mut created_elements = Vec::new();
        
        for mut element in elements {
            element.id = Uuid::new_v4().to_string();
            element.created_at = Utc::now();
            element.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                INSERT INTO worldbuilding (id, project_id, series_id, name, description, element_type, 
                                         properties, is_visible, original_project_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&element.id)
            .bind(&element.project_id)
            .bind(&element.series_id)
            .bind(&element.name)
            .bind(&element.description)
            .bind(&element.element_type)
            .bind(&element.properties)
            .bind(element.is_visible)
            .bind(&element.original_project_id)
            .bind(element.created_at)
            .bind(element.updated_at)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create world element: {}", e)))?;
            
            created_elements.push(element);
        }
        
        Ok(created_elements)
    }
}