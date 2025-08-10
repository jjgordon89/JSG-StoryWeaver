//! AI Response Card database operations

use sqlx::{Pool, Sqlite};
use crate::error::Result;
use crate::models::ai_card::{AIResponseCard, CreateAICardRequest, UpdateAICardRequest, AICardFilter};

pub struct AICardOps;

impl AICardOps {
    /// Create a new AI response card
    pub async fn create(pool: &Pool<Sqlite>, request: CreateAICardRequest) -> Result<AIResponseCard> {
        AIResponseCard::create(pool, request).await
    }
    
    /// Get AI card by ID
    pub async fn get(pool: &Pool<Sqlite>, id: &str) -> Result<AIResponseCard> {
        AIResponseCard::get_by_id(pool, id).await
    }
    
    /// Get AI cards by project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(pool, filter).await
    }
    
    /// Get AI cards by document
    pub async fn get_by_document(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: None,
            document_id: Some(document_id.to_string()),
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(pool, filter).await
    }
    
    /// Get AI cards by type (feature_type)
    pub async fn get_by_type(pool: &Pool<Sqlite>, project_id: &str, card_type: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: Some(card_type.to_string()),
            is_stacked: None,
            is_starred: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(pool, filter).await
    }
    
    /// Get AI cards by status (stacked/starred)
    pub async fn get_by_status(pool: &Pool<Sqlite>, project_id: &str, status: &str) -> Result<Vec<AIResponseCard>> {
        let (is_stacked, is_starred) = match status {
            "stacked" => (Some(true), None),
            "starred" => (None, Some(true)),
            _ => (None, None),
        };
        
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked,
            is_starred,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(pool, filter).await
    }
    
    /// Get AI cards by date range
    pub async fn get_by_date_range(pool: &Pool<Sqlite>, project_id: &str, _start_date: &str, _end_date: &str) -> Result<Vec<AIResponseCard>> {
        // For now, just return all cards for the project
        // TODO: Implement actual date range filtering
        Self::get_by_project(pool, project_id).await
    }
    
    /// Get AI cards by provider
    pub async fn get_by_provider(pool: &Pool<Sqlite>, project_id: &str, _provider: &str) -> Result<Vec<AIResponseCard>> {
        // For now, just return all cards for the project
        // TODO: Implement actual provider filtering based on model_used field
        Self::get_by_project(pool, project_id).await
    }
    
    /// Get AI cards by model
    pub async fn get_by_model(pool: &Pool<Sqlite>, project_id: &str, model: &str) -> Result<Vec<AIResponseCard>> {
        // This would require a more complex filter, for now return all for project
        // TODO: Add model filtering to AICardFilter
        Self::get_by_project(pool, project_id).await
    }
    
    /// Get AI cards by cost range
    pub async fn get_by_cost_range(pool: &Pool<Sqlite>, project_id: &str, _min_cost: f64, _max_cost: f64) -> Result<Vec<AIResponseCard>> {
        // For now, just return all cards for the project
        // TODO: Implement actual cost range filtering
        Self::get_by_project(pool, project_id).await
    }
    
    /// Update AI card
    pub async fn update(pool: &Pool<Sqlite>, id: &str, request: UpdateAICardRequest) -> Result<AIResponseCard> {
        AIResponseCard::update(pool, id, request).await
    }
    
    /// Delete AI card
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        AIResponseCard::delete(pool, id).await
    }
}