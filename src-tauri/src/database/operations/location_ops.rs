use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Location operations
impl super::LocationOps {
    /// Create a new location
    pub async fn create(pool: &Pool<Sqlite>, mut location: Location) -> Result<Location> {
        location.id = Uuid::new_v4().to_string();
        location.created_at = Utc::now();
        location.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO locations (id, project_id, name, description, location_type, geography,
                                 climate, culture, history, significance, visibility,
                                 created_at, updated_at, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&location.id)
        .bind(&location.project_id)
        .bind(&location.name)
        .bind(&location.description)
        .bind(&location.location_type)
        .bind(&location.geography)
        .bind(&location.climate)
        .bind(&location.culture)
        .bind(&location.history)
        .bind(&location.significance)
        .bind(&location.visibility)
        .bind(location.created_at)
        .bind(location.updated_at)
        .bind(&location.metadata)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create location: {}", e)))?;
        
        Ok(location)
    }
    
    /// Get locations by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Location>> {
        let locations = sqlx::query_as::<_, Location>(
            "SELECT * FROM locations WHERE project_id = ? ORDER BY name"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get locations: {}", e)))?;
        
        Ok(locations)
    }
    
    /// Update a location
    pub async fn update(pool: &Pool<Sqlite>, location: &Location) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE locations SET name = ?, description = ?, location_type = ?, geography = ?,
                               climate = ?, culture = ?, history = ?, significance = ?,
                               visibility = ?, updated_at = ?, metadata = ?
            WHERE id = ?
            "#,
        )
        .bind(&location.name)
        .bind(&location.description)
        .bind(&location.location_type)
        .bind(&location.geography)
        .bind(&location.climate)
        .bind(&location.culture)
        .bind(&location.history)
        .bind(&location.significance)
        .bind(&location.visibility)
        .bind(Utc::now())
        .bind(&location.metadata)
        .bind(&location.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update location: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a location
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM locations WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete location: {}", e)))?;
        
        Ok(())
    }
}
