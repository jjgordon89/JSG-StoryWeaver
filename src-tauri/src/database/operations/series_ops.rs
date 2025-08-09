use crate::database::models::Series;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Series operations
impl super::SeriesOps {
    /// Create a new series
    pub async fn create(pool: &Pool<Sqlite>, mut series: Series) -> Result<Series> {
        series.id = Uuid::new_v4().to_string();
        series.created_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO series (id, name, description, folder_id, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&series.id)
        .bind(&series.name)
        .bind(&series.description)
        .bind(&series.folder_id)
        .bind(series.created_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create series: {}", e)))?;
        
        Ok(series)
    }
    
    /// Get a series by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<Series>> {
        let series = sqlx::query_as::<_, Series>("SELECT * FROM series WHERE id = ?")
            .bind(id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get series: {}", e)))?;
        
        Ok(series)
    }
    
    /// Get all series
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Series>> {
        let series = sqlx::query_as::<_, Series>(
            "SELECT * FROM series ORDER BY name"
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get all series: {}", e)))?;
        
        Ok(series)
    }
    
    /// Update a series
    pub async fn update(pool: &Pool<Sqlite>, series: &Series) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE series SET name = ?, description = ?, folder_id = ?
            WHERE id = ?
            "#,
        )
        .bind(&series.name)
        .bind(&series.description)
        .bind(&series.folder_id)
        .bind(&series.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update series: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a series
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        // Check if there are any projects in this series
        let project_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM projects WHERE series_id = ?"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check series projects: {}", e)))?;
        
        if project_count > 0 {
            return Err(StoryWeaverError::SeriesNotEmpty { id: id.to_string() });
        }
        
        // If no projects, delete the series
        sqlx::query("DELETE FROM series WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete series: {}", e)))?;
        
        Ok(())
    }
    
    /// Get projects in a series
    pub async fn get_projects(pool: &Pool<Sqlite>, series_id: &str) -> Result<Vec<crate::database::models::Project>> {
        let projects = sqlx::query_as::<_, crate::database::models::Project>(
            "SELECT * FROM projects WHERE series_id = ? ORDER BY name"
        )
        .bind(series_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get series projects: {}", e)))?;
        
        Ok(projects)
    }
    
    /// Add project to series
    pub async fn add_project(pool: &Pool<Sqlite>, series_id: &str, project_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE projects SET series_id = ? WHERE id = ?"
        )
        .bind(series_id)
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to add project to series: {}", e)))?;
        
        Ok(())
    }
    
    /// Remove project from series
    pub async fn remove_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE projects SET series_id = NULL WHERE id = ?"
        )
        .bind(project_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to remove project from series: {}", e)))?;
        
        Ok(())
    }
    
    /// Get series with project counts
    pub async fn get_series_with_counts(pool: &Pool<Sqlite>) -> Result<Vec<SeriesWithCount>> {
        let series = sqlx::query_as!(
            SeriesWithCount,
            r#"
            SELECT 
                s.id,
                s.name,
                s.description,
                s.folder_id, 
                s.created_at as "created_at!: chrono::DateTime<chrono::Utc>",
                CAST(COUNT(p.id) AS INTEGER) as project_count
            FROM 
                series s
            LEFT JOIN 
                projects p ON s.id = p.series_id
            GROUP BY 
                s.id, s.name, s.description, s.folder_id, s.created_at
            ORDER BY 
                s.name
            "#
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get series with counts: {}", e)))?;
        
        Ok(series)
    }
}

/// Series with project count
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct SeriesWithCount {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub folder_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub project_count: i64,
}
