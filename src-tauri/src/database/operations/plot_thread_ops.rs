use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite, Row};
use uuid::Uuid;
use serde_json;

/// Plot thread operations
impl super::PlotThreadOps {
    /// Create a new plot thread
    pub async fn create(pool: &Pool<Sqlite>, mut plot_thread: PlotThread) -> Result<PlotThread> {
        plot_thread.id = Uuid::new_v4().to_string();
        plot_thread.created_at = Utc::now();
        plot_thread.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO plot_threads (id, project_id, name, description, status, priority,
                                    characters_involved, documents_involved, visibility,
                                    created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&plot_thread.id)
        .bind(&plot_thread.project_id)
        .bind(&plot_thread.name)
        .bind(&plot_thread.description)
        .bind(&plot_thread.status)
        .bind(&plot_thread.priority)
        .bind(serde_json::to_string(&plot_thread.characters_involved).unwrap_or_default())
        .bind(serde_json::to_string(&plot_thread.documents_involved).unwrap_or_default())
        .bind(&plot_thread.visibility)
        .bind(plot_thread.created_at)
        .bind(plot_thread.updated_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create plot thread: {}", e)))?;
        
        Ok(plot_thread)
    }
    
    /// Get plot threads by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<PlotThread>> {
        let rows = sqlx::query(
            "SELECT * FROM plot_threads WHERE project_id = ? ORDER BY priority DESC, name"
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plot threads: {}", e)))?;
        
        let mut plot_threads = Vec::new();
        for row in rows {
            plot_threads.push(PlotThread {
                id: row.get("id"),
                project_id: row.get("project_id"),
                name: row.get("name"),
                description: row.get("description"),
                status: row.get("status"),
                priority: row.get("priority"),
                characters_involved: row.get("characters_involved"),
                documents_involved: row.get("documents_involved"),
                visibility: row.get("visibility"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(plot_threads)
    }
    
    /// Get plot thread by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<PlotThread> {
        let row = sqlx::query(
            "SELECT * FROM plot_threads WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plot thread: {}", e)))?;
        
        Ok(PlotThread {
            id: row.get("id"),
            project_id: row.get("project_id"),
            name: row.get("name"),
            description: row.get("description"),
            status: row.get("status"),
            priority: row.get("priority"),
            characters_involved: row.get("characters_involved"),
            documents_involved: row.get("documents_involved"),
            visibility: row.get("visibility"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    /// Update a plot thread
    pub async fn update(pool: &Pool<Sqlite>, plot_thread: &PlotThread) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE plot_threads SET 
                name = ?, description = ?, status = ?, priority = ?,
                characters_involved = ?, documents_involved = ?, visibility = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&plot_thread.name)
        .bind(&plot_thread.description)
        .bind(&plot_thread.status)
        .bind(&plot_thread.priority)
        .bind(serde_json::to_string(&plot_thread.characters_involved).unwrap_or_default())
        .bind(serde_json::to_string(&plot_thread.documents_involved).unwrap_or_default())
        .bind(&plot_thread.visibility)
        .bind(Utc::now())
        .bind(&plot_thread.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update plot thread: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a plot thread
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM plot_threads WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete plot thread: {}", e)))?;
        
        Ok(())
    }
    
    /// Get plot threads by character involvement
    pub async fn get_by_character(pool: &Pool<Sqlite>, project_id: &str, character_id: &str) -> Result<Vec<PlotThread>> {
        let plot_threads = Self::get_by_project(pool, project_id).await?;
        
        Ok(plot_threads.into_iter()
            .filter(|thread| thread.characters_involved.contains(&character_id.to_string()))
            .collect())
    }
    
    /// Get plot threads by document involvement
    pub async fn get_by_document(pool: &Pool<Sqlite>, project_id: &str, document_id: &str) -> Result<Vec<PlotThread>> {
        let plot_threads = Self::get_by_project(pool, project_id).await?;
        
        Ok(plot_threads.into_iter()
            .filter(|thread| thread.documents_involved.contains(&document_id.to_string()))
            .collect())
    }
    
    /// Get plot threads by status
    pub async fn get_by_status(pool: &Pool<Sqlite>, project_id: &str, status: &PlotThreadStatus) -> Result<Vec<PlotThread>> {
        let status_str = match status {
            PlotThreadStatus::Planned => "planned",
            PlotThreadStatus::Active => "active",
            PlotThreadStatus::Resolved => "resolved",
            PlotThreadStatus::Abandoned => "abandoned",
        };
        
        let rows = sqlx::query(
            "SELECT * FROM plot_threads WHERE project_id = ? AND status = ? ORDER BY priority DESC, name"
        )
        .bind(project_id)
        .bind(status_str)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plot threads by status: {}", e)))?;
        
        let mut plot_threads = Vec::new();
        for row in rows {
            plot_threads.push(PlotThread {
                id: row.get("id"),
                project_id: row.get("project_id"),
                name: row.get("name"),
                description: row.get("description"),
                status: row.get("status"),
                priority: row.get("priority"),
                characters_involved: row.get("characters_involved"),
                documents_involved: row.get("documents_involved"),
                visibility: row.get("visibility"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(plot_threads)
    }
    
    /// Get plot threads by priority
    pub async fn get_by_priority(pool: &Pool<Sqlite>, project_id: &str, priority: &ThreadPriority) -> Result<Vec<PlotThread>> {
        let priority_str = match priority {
            ThreadPriority::Main => "main",
            ThreadPriority::Subplot => "subplot",
            ThreadPriority::Background => "background",
        };
        
        let rows = sqlx::query(
            "SELECT * FROM plot_threads WHERE project_id = ? AND priority = ? ORDER BY name"
        )
        .bind(project_id)
        .bind(priority_str)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plot threads by priority: {}", e)))?;
        
        let mut plot_threads = Vec::new();
        for row in rows {
            plot_threads.push(PlotThread {
                id: row.get("id"),
                project_id: row.get("project_id"),
                name: row.get("name"),
                description: row.get("description"),
                status: row.get("status"),
                priority: row.get("priority"),
                characters_involved: row.get("characters_involved"),
                documents_involved: row.get("documents_involved"),
                visibility: row.get("visibility"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(plot_threads)
    }
    
    /// Update plot thread status
    pub async fn update_status(pool: &Pool<Sqlite>, id: &str, status: PlotThreadStatus) -> Result<()> {
        let status_str = match status {
            PlotThreadStatus::Planned => "planned",
            PlotThreadStatus::Active => "active",
            PlotThreadStatus::Resolved => "resolved",
            PlotThreadStatus::Abandoned => "abandoned",
        };
        
        sqlx::query(
            "UPDATE plot_threads SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(status_str)
        .bind(Utc::now())
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update plot thread status: {}", e)))?;
        
        Ok(())
    }
    
    /// Add character to plot thread
    pub async fn add_character(pool: &Pool<Sqlite>, plot_thread_id: &str, character_id: &str) -> Result<()> {
        let mut plot_thread = Self::get_by_id(pool, plot_thread_id).await?;
        
        if !plot_thread.characters_involved.contains(&character_id.to_string()) {
            plot_thread.characters_involved.push(character_id.to_string());
            Self::update(pool, &plot_thread).await?;
        }
        
        Ok(())
    }
    
    /// Remove character from plot thread
    pub async fn remove_character(pool: &Pool<Sqlite>, plot_thread_id: &str, character_id: &str) -> Result<()> {
        let mut plot_thread = Self::get_by_id(pool, plot_thread_id).await?;
        
        plot_thread.characters_involved.retain(|id| id != character_id);
        Self::update(pool, &plot_thread).await?;
        
        Ok(())
    }
    
    /// Add document to plot thread
    pub async fn add_document(pool: &Pool<Sqlite>, plot_thread_id: &str, document_id: &str) -> Result<()> {
        let mut plot_thread = Self::get_by_id(pool, plot_thread_id).await?;
        
        if !plot_thread.documents_involved.contains(&document_id.to_string()) {
            plot_thread.documents_involved.push(document_id.to_string());
            Self::update(pool, &plot_thread).await?;
        }
        
        Ok(())
    }
    
    /// Remove document from plot thread
    pub async fn remove_document(pool: &Pool<Sqlite>, plot_thread_id: &str, document_id: &str) -> Result<()> {
        let mut plot_thread = Self::get_by_id(pool, plot_thread_id).await?;
        
        plot_thread.documents_involved.retain(|id| id != document_id);
        Self::update(pool, &plot_thread).await?;
        
        Ok(())
    }
}
