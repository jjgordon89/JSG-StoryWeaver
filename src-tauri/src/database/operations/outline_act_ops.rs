use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Outline act operations
impl super::OutlineActOps {
    /// Create a new outline act
    pub async fn create(pool: &Pool<Sqlite>, outline_act: OutlineAct) -> Result<OutlineAct> {
        let mut outline_act = outline_act;
        outline_act.id = Uuid::new_v4().to_string();
        outline_act.created_at = Utc::now();
        outline_act.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO outline_acts (id, outline_id, act_type, act_number, title, position, 
                                    created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&outline_act.id)
        .bind(&outline_act.outline_id)
        .bind(&outline_act.act_type)
        .bind(outline_act.act_number)
        .bind(&outline_act.title)
        .bind(outline_act.position)
        .bind(outline_act.created_at)
        .bind(outline_act.updated_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create outline act: {}", e)))?;
        
        Ok(outline_act)
    }
    
    /// Get all acts for an outline
    pub async fn get_by_outline(pool: &Pool<Sqlite>, outline_id: &str) -> Result<Vec<OutlineAct>> {
        let acts = sqlx::query_as::<_, OutlineAct>(
            r#"
            SELECT id, outline_id, act_type, act_number, title, position, created_at, updated_at
            FROM outline_acts
            WHERE outline_id = ?
            ORDER BY position, act_number
            "#,
        )
        .bind(outline_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outline acts: {}", e)))?;
        
        Ok(acts)
    }
    
    /// Get an outline act by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<OutlineAct> {
        let act = sqlx::query_as::<_, OutlineAct>(
            r#"
            SELECT id, outline_id, act_type, act_number, title, position, created_at, updated_at
            FROM outline_acts
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outline act: {}", e)))?;
        
        Ok(act)
    }
    
    /// Get acts by type
    pub async fn get_by_type(pool: &Pool<Sqlite>, outline_id: &str, act_type: &str) -> Result<Vec<OutlineAct>> {
        let acts = sqlx::query_as::<_, OutlineAct>(
            r#"
            SELECT id, outline_id, act_type, act_number, title, position, created_at, updated_at
            FROM outline_acts
            WHERE outline_id = ? AND act_type = ?
            ORDER BY position, act_number
            "#,
        )
        .bind(outline_id)
        .bind(act_type)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outline acts by type: {}", e)))?;
        
        Ok(acts)
    }
    
    /// Update an outline act
    pub async fn update(pool: &Pool<Sqlite>, outline_act: OutlineAct) -> Result<OutlineAct> {
        let mut outline_act = outline_act;
        outline_act.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE outline_acts SET
                act_type = ?, act_number = ?, title = ?, position = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&outline_act.act_type)
        .bind(outline_act.act_number)
        .bind(&outline_act.title)
        .bind(outline_act.position)
        .bind(outline_act.updated_at)
        .bind(&outline_act.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update outline act: {}", e)))?;
        
        Ok(outline_act)
    }
    
    /// Update act position
    pub async fn update_position(pool: &Pool<Sqlite>, id: &str, position: i32) -> Result<()> {
        sqlx::query(
            "UPDATE outline_acts SET position = ?, updated_at = ? WHERE id = ?"
        )
        .bind(position)
        .bind(Utc::now())
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update act position: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete an outline act
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        // First delete associated scenes
        sqlx::query("DELETE FROM scenes WHERE outline_id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete act scenes: {}", e)))?;
        
        // Then delete the act
        sqlx::query("DELETE FROM outline_acts WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete outline act: {}", e)))?;
        
        Ok(())
    }
    
    /// Get next available position for an outline
    pub async fn get_next_position(pool: &Pool<Sqlite>, outline_id: &str) -> Result<i32> {
        let max_position: (Option<i32>,) = sqlx::query_as(
            "SELECT MAX(position) FROM outline_acts WHERE outline_id = ?"
        )
        .bind(outline_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get max position: {}", e)))?;
        
        Ok(max_position.0.unwrap_or(0) + 1)
    }
    
    /// Get next available act number for a type
    pub async fn get_next_act_number(pool: &Pool<Sqlite>, outline_id: &str, act_type: &str) -> Result<i32> {
        let max_act_number: (Option<i32>,) = sqlx::query_as(
            "SELECT MAX(act_number) FROM outline_acts WHERE outline_id = ? AND act_type = ?"
        )
        .bind(outline_id)
        .bind(act_type)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get max act number: {}", e)))?;
        
        Ok(max_act_number.0.unwrap_or(0) + 1)
    }
    
    /// Reorder acts
    pub async fn reorder_acts(pool: &Pool<Sqlite>, outline_id: &str, act_orders: Vec<(String, i32)>) -> Result<()> {
        for (act_id, new_position) in act_orders {
            sqlx::query(
                "UPDATE outline_acts SET position = ?, updated_at = ? WHERE id = ? AND outline_id = ?"
            )
            .bind(new_position)
            .bind(Utc::now())
            .bind(&act_id)
            .bind(outline_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to reorder act: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Search acts by title
    pub async fn search(pool: &Pool<Sqlite>, outline_id: &str, query: &str) -> Result<Vec<OutlineAct>> {
        let search_query = format!("%{}%", query);
        
        let acts = sqlx::query_as::<_, OutlineAct>(
            r#"
            SELECT id, outline_id, act_type, act_number, title, position, created_at, updated_at
            FROM outline_acts
            WHERE outline_id = ? AND title LIKE ?
            ORDER BY position, act_number
            "#,
        )
        .bind(outline_id)
        .bind(&search_query)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to search outline acts: {}", e)))?;
        
        Ok(acts)
    }
    
    /// Get act count by type
    pub async fn get_count_by_type(pool: &Pool<Sqlite>, outline_id: &str, act_type: &str) -> Result<i32> {
        let count: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM outline_acts WHERE outline_id = ? AND act_type = ?"
        )
        .bind(outline_id)
        .bind(act_type)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get act count by type: {}", e)))?;
        
        Ok(count.0)
    }
    
    /// Get total act count for outline
    pub async fn get_total_count(pool: &Pool<Sqlite>, outline_id: &str) -> Result<i32> {
        let count: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM outline_acts WHERE outline_id = ?"
        )
        .bind(outline_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total act count: {}", e)))?;
        
        Ok(count.0)
    }
    
    /// Bulk create outline acts
    pub async fn bulk_create(pool: &Pool<Sqlite>, acts: Vec<OutlineAct>) -> Result<Vec<OutlineAct>> {
        let mut created_acts = Vec::new();
        
        for mut act in acts {
            act.id = Uuid::new_v4().to_string();
            act.created_at = Utc::now();
            act.updated_at = Utc::now();
            
            sqlx::query(
                r#"
                INSERT INTO outline_acts (id, outline_id, act_type, act_number, title, position, 
                                        created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&act.id)
            .bind(&act.outline_id)
            .bind(&act.act_type)
            .bind(act.act_number)
            .bind(&act.title)
            .bind(act.position)
            .bind(act.created_at)
            .bind(act.updated_at)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create outline act: {}", e)))?;
            
            created_acts.push(act);
        }
        
        Ok(created_acts)
    }
    
    /// Delete all acts for an outline
    pub async fn delete_by_outline(pool: &Pool<Sqlite>, outline_id: &str) -> Result<()> {
        // First delete all scenes for these acts
        sqlx::query(
            "DELETE FROM scenes WHERE outline_id IN (SELECT id FROM outline_acts WHERE outline_id = ?)"
        )
        .bind(outline_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete act scenes: {}", e)))?;
        
        // Then delete all acts
        sqlx::query("DELETE FROM outline_acts WHERE outline_id = ?")
            .bind(outline_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete outline acts: {}", e)))?;
        
        Ok(())
    }
}
