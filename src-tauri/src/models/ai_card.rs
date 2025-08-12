//! AI Response Card models and database operations

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite, Row};
use uuid::Uuid;
use crate::error::{Result, StoryWeaverError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponseCard {
    pub id: String,
    pub project_id: String,
    pub document_id: Option<String>,
    pub feature_type: String,
    pub prompt_context: String,
    pub response_text: String,
    pub model_used: Option<String>,
    pub token_count: Option<i32>,
    pub cost_estimate: Option<f64>,
    pub is_stacked: bool,
    pub is_starred: bool,
    pub is_collapsed: bool,
    pub stack_position: Option<i32>,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAICardRequest {
    pub project_id: String,
    pub document_id: Option<String>,
    pub feature_type: String,
    pub prompt_context: String,
    pub response_text: String,
    pub model_used: Option<String>,
    pub token_count: Option<i32>,
    pub cost_estimate: Option<f64>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAICardRequest {
    pub is_stacked: Option<bool>,
    pub is_starred: Option<bool>,
    pub is_collapsed: Option<bool>,
    pub stack_position: Option<i32>,
    pub tags: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AICardFilter {
    pub project_id: Option<String>,
    pub document_id: Option<String>,
    pub feature_type: Option<String>,
    pub is_stacked: Option<bool>,
    pub is_starred: Option<bool>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl AIResponseCard {
    /// Create a new AI response card
    pub async fn create(pool: &Pool<Sqlite>, request: CreateAICardRequest) -> Result<AIResponseCard> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        sqlx::query(
            r#"
            INSERT INTO ai_response_cards (
                id, project_id, document_id, feature_type, prompt_context, 
                response_text, model_used, token_count, cost_estimate, tags,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&id)
        .bind(&request.project_id)
        .bind(&request.document_id)
        .bind(&request.feature_type)
        .bind(&request.prompt_context)
        .bind(&request.response_text)
        .bind(&request.model_used)
        .bind(request.token_count)
        .bind(request.cost_estimate)
        .bind(&request.tags)
        .bind(&now)
        .bind(&now)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create AI card: {}", e)))?;
        
        Self::get_by_id(&*pool, &id).await
    }
    
    /// Get AI card by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<AIResponseCard> {
        let row = sqlx::query(
            "SELECT * FROM ai_response_cards WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI card: {}", e)))?;
        
        Ok(Self::from_row(&row)?)
    }
    
    /// Get AI cards with filters
    pub async fn get_filtered(pool: &Pool<Sqlite>, filter: AICardFilter) -> Result<Vec<AIResponseCard>> {
        let mut query = "SELECT * FROM ai_response_cards WHERE 1=1".to_string();
        let mut bindings = Vec::new();
        
        if let Some(project_id) = &filter.project_id {
            query.push_str(" AND project_id = ?");
            bindings.push(project_id.as_str());
        }
        
        if let Some(document_id) = &filter.document_id {
            query.push_str(" AND document_id = ?");
            bindings.push(document_id.as_str());
        }
        
        if let Some(feature_type) = &filter.feature_type {
            query.push_str(" AND feature_type = ?");
            bindings.push(feature_type.as_str());
        }
        
        if let Some(is_stacked) = filter.is_stacked {
            query.push_str(" AND is_stacked = ?");
            bindings.push(if is_stacked { "1" } else { "0" });
        }
        
        if let Some(is_starred) = filter.is_starred {
            query.push_str(" AND is_starred = ?");
            bindings.push(if is_starred { "1" } else { "0" });
        }
        
        query.push_str(" ORDER BY created_at DESC");
        
        if let Some(limit) = filter.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = filter.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }
        
        let mut sql_query = sqlx::query(&query);
        for binding in bindings {
            sql_query = sql_query.bind(binding);
        }
        
        let rows = sql_query
            .fetch_all(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get AI cards: {}", e)))?;
        
        let mut cards = Vec::new();
        for row in rows {
            cards.push(Self::from_row(&row)?);
        }
        
        Ok(cards)
    }
    
    /// Update AI card
    pub async fn update(pool: &Pool<Sqlite>, id: &str, request: UpdateAICardRequest) -> Result<AIResponseCard> {
        let now = chrono::Utc::now().to_rfc3339();
        
        let mut updates = Vec::new();
        let mut binding_values = Vec::new();
        
        if let Some(is_stacked) = request.is_stacked {
            updates.push("is_stacked = ?");
            binding_values.push(if is_stacked { "1".to_string() } else { "0".to_string() });
        }
        
        if let Some(is_starred) = request.is_starred {
            updates.push("is_starred = ?");
            binding_values.push(if is_starred { "1".to_string() } else { "0".to_string() });
        }
        
        if let Some(is_collapsed) = request.is_collapsed {
            updates.push("is_collapsed = ?");
            binding_values.push(if is_collapsed { "1".to_string() } else { "0".to_string() });
        }
        
        if let Some(stack_position) = request.stack_position {
            updates.push("stack_position = ?");
            binding_values.push(stack_position.to_string());
        }
        
        if let Some(tags) = &request.tags {
            updates.push("tags = ?");
            binding_values.push(tags.clone());
        }
        
        let mut bindings: Vec<&str> = binding_values.iter().map(|s| s.as_str()).collect();
        
        if updates.is_empty() {
            return Self::get_by_id(&*pool, id).await;
        }
        
        updates.push("updated_at = ?");
        bindings.push(&now);
        bindings.push(id);
        
        let query = format!(
            "UPDATE ai_response_cards SET {} WHERE id = ?",
            updates.join(", ")
        );
        
        let mut sql_query = sqlx::query(&query);
        for binding in bindings {
            sql_query = sql_query.bind(binding);
        }
        
        sql_query
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to update AI card: {}", e)))?;
        
        Self::get_by_id(&*pool, id).await
    }
    
    /// Delete AI card
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM ai_response_cards WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete AI card: {}", e)))?;
        
        Ok(())
    }
    
    /// Convert database row to AIResponseCard
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<AIResponseCard> {
        Ok(AIResponseCard {
            id: row.try_get("id").map_err(|e| StoryWeaverError::database(format!("Failed to get id: {}", e)))?,
            project_id: row.try_get("project_id").map_err(|e| StoryWeaverError::database(format!("Failed to get project_id: {}", e)))?,
            document_id: row.try_get("document_id").map_err(|e| StoryWeaverError::database(format!("Failed to get document_id: {}", e)))?,
            feature_type: row.try_get("feature_type").map_err(|e| StoryWeaverError::database(format!("Failed to get feature_type: {}", e)))?,
            prompt_context: row.try_get("prompt_context").map_err(|e| StoryWeaverError::database(format!("Failed to get prompt_context: {}", e)))?,
            response_text: row.try_get("response_text").map_err(|e| StoryWeaverError::database(format!("Failed to get response_text: {}", e)))?,
            model_used: row.try_get("model_used").map_err(|e| StoryWeaverError::database(format!("Failed to get model_used: {}", e)))?,
            token_count: row.try_get("token_count").map_err(|e| StoryWeaverError::database(format!("Failed to get token_count: {}", e)))?,
            cost_estimate: row.try_get("cost_estimate").map_err(|e| StoryWeaverError::database(format!("Failed to get cost_estimate: {}", e)))?,
            is_stacked: row.try_get::<i32, _>("is_stacked").map_err(|e| StoryWeaverError::database(format!("Failed to get is_stacked: {}", e)))? != 0,
            is_starred: row.try_get::<i32, _>("is_starred").map_err(|e| StoryWeaverError::database(format!("Failed to get is_starred: {}", e)))? != 0,
            is_collapsed: row.try_get::<i32, _>("is_collapsed").map_err(|e| StoryWeaverError::database(format!("Failed to get is_collapsed: {}", e)))? != 0,
            stack_position: row.try_get("stack_position").map_err(|e| StoryWeaverError::database(format!("Failed to get stack_position: {}", e)))?,
            tags: row.try_get("tags").map_err(|e| StoryWeaverError::database(format!("Failed to get tags: {}", e)))?,
            created_at: row.try_get("created_at").map_err(|e| StoryWeaverError::database(format!("Failed to get created_at: {}", e)))?,
            updated_at: row.try_get("updated_at").map_err(|e| StoryWeaverError::database(format!("Failed to get updated_at: {}", e)))?,
        })
    }
}
