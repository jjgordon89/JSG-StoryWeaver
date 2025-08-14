use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use sqlx::Row;
use uuid::Uuid;
use serde_json;

/// Timeline event operations
impl super::TimelineOps {
    /// Create a new timeline event
    pub async fn create(pool: &Pool<Sqlite>, mut event: TimelineEvent) -> Result<TimelineEvent> {
        event.id = Uuid::new_v4().to_string();
        event.created_at = Utc::now();
        event.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO timeline_events (id, project_id, title, description, event_date, real_date,
                                       importance, characters_involved, locations_involved, visibility,
                                       created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&event.id)
        .bind(&event.project_id)
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.event_date)
        .bind(event.real_date)
        .bind(&event.importance)
        .bind(serde_json::to_string(&event.characters_involved).unwrap_or_default())
        .bind(serde_json::to_string(&event.locations_involved).unwrap_or_default())
        .bind(&event.visibility)
        .bind(event.created_at)
        .bind(event.updated_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create timeline event: {}", e)))?;
        
        Ok(event)
    }
    
    /// Get timeline events by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<TimelineEvent>> {
        let rows = sqlx::query(
            "SELECT * FROM timeline_events WHERE project_id = ? ORDER BY event_date, created_at"
        )
        .bind(project_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get timeline events: {}", e)))?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(TimelineEvent {
                id: row.get("id"),
                project_id: row.get("project_id"),
                title: row.get("title"),
                description: row.get("description"),
                event_date: row.get("event_date"),
                real_date: row.get("real_date"),
                importance: row.get("importance"),
                characters_involved: row.get("characters_involved"),
                locations_involved: row.get("locations_involved"),
                visibility: row.get("visibility"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(events)
    }
    
    /// Get timeline event by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<TimelineEvent> {
        let row = sqlx::query(
            "SELECT * FROM timeline_events WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get timeline event: {}", e)))?;
        
        Ok(TimelineEvent {
            id: row.get("id"),
            project_id: row.get("project_id"),
            title: row.get("title"),
            description: row.get("description"),
            event_date: row.get("event_date"),
            real_date: row.get("real_date"),
            importance: row.get("importance"),
            characters_involved: row.get("characters_involved"),
            locations_involved: row.get("locations_involved"),
            visibility: row.get("visibility"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
    
    /// Update a timeline event
    pub async fn update(pool: &Pool<Sqlite>, event: &TimelineEvent) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE timeline_events SET 
                title = ?, description = ?, event_date = ?, real_date = ?,
                importance = ?, characters_involved = ?, locations_involved = ?,
                visibility = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&event.title)
        .bind(&event.description)
        .bind(&event.event_date)
        .bind(event.real_date)
        .bind(&event.importance)
        .bind(serde_json::to_string(&event.characters_involved).unwrap_or_default())
        .bind(serde_json::to_string(&event.locations_involved).unwrap_or_default())
        .bind(&event.visibility)
        .bind(Utc::now())
        .bind(&event.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update timeline event: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a timeline event
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM timeline_events WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete timeline event: {}", e)))?;
        
        Ok(())
    }
    
    /// Get timeline events by character involvement
    pub async fn get_by_character(pool: &Pool<Sqlite>, project_id: &str, character_id: &str) -> Result<Vec<TimelineEvent>> {
        let events = Self::get_by_project(&*pool, project_id).await?;
        
        Ok(events.into_iter()
            .filter(|event| event.characters_involved.contains(&character_id.to_string()))
            .collect())
    }
    
    /// Get timeline events by location involvement
    pub async fn get_by_location(pool: &Pool<Sqlite>, project_id: &str, location_id: &str) -> Result<Vec<TimelineEvent>> {
        let events = Self::get_by_project(&*pool, project_id).await?;
        
        Ok(events.into_iter()
            .filter(|event| event.locations_involved.contains(&location_id.to_string()))
            .collect())
    }
    
    /// Get timeline events by importance level
    pub async fn get_by_importance(pool: &Pool<Sqlite>, project_id: &str, importance: &EventImportance) -> Result<Vec<TimelineEvent>> {
        let importance_str = match importance {
            EventImportance::Critical => "critical",
            EventImportance::Major => "major",
            EventImportance::Minor => "minor",
            EventImportance::Background => "background",
        };
        
        let rows = sqlx::query(
            "SELECT * FROM timeline_events WHERE project_id = ? AND importance = ? ORDER BY event_date, created_at"
        )
        .bind(project_id)
        .bind(importance_str)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get timeline events by importance: {}", e)))?;
        
        let mut events = Vec::new();
        for row in rows {
            events.push(TimelineEvent {
                id: row.get("id"),
                project_id: row.get("project_id"),
                title: row.get("title"),
                description: row.get("description"),
                event_date: row.get("event_date"),
                real_date: row.get("real_date"),
                importance: row.get("importance"),
                characters_involved: row.get("characters_involved"),
                locations_involved: row.get("locations_involved"),
                visibility: row.get("visibility"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            });
        }
        
        Ok(events)
    }
}
