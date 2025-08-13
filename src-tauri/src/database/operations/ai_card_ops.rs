//! AI Response Card database operations

use sqlx::{Pool, Sqlite};
use crate::error::Result;
use crate::models::ai_card::{AIResponseCard, CreateAICardRequest, UpdateAICardRequest, AICardFilter};

pub struct AICardOps;

impl AICardOps {
    /// Create a new AI response card
    pub async fn create(pool: &Pool<Sqlite>, request: CreateAICardRequest) -> Result<AIResponseCard> {
        AIResponseCard::create(&*pool, request).await
    }
    
    /// Get AI card by ID
    pub async fn get(pool: &Pool<Sqlite>, id: &str) -> Result<AIResponseCard> {
        AIResponseCard::get_by_id(&*pool, id).await
    }
    
    /// Get AI cards by project
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: None,
            model_used: None,
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Get AI cards by document
    pub async fn get_by_document(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: None,
            document_id: Some(document_id.to_string()),
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: None,
            model_used: None,
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Get AI cards by type (feature_type)
    pub async fn get_by_type(pool: &Pool<Sqlite>, project_id: &str, card_type: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: Some(card_type.to_string()),
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: None,
            model_used: None,
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
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
            date_start: None,
            date_end: None,
            provider: None,
            model_used: None,
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Get AI cards by date range
    pub async fn get_by_date_range(pool: &Pool<Sqlite>, project_id: &str, start_date: &str, end_date: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            date_start: Some(start_date.to_string()),
            date_end: Some(end_date.to_string()),
            provider: None,
            model_used: None,
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Get AI cards by provider
    pub async fn get_by_provider(pool: &Pool<Sqlite>, project_id: &str, provider: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: Some(provider.to_string()),
            model_used: None,
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Get AI cards by model
    pub async fn get_by_model(pool: &Pool<Sqlite>, project_id: &str, model: &str) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: None,
            model_used: Some(model.to_string()),
            cost_min: None,
            cost_max: None,
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Get AI cards by cost range
    pub async fn get_by_cost_range(pool: &Pool<Sqlite>, project_id: &str, min_cost: f64, max_cost: f64) -> Result<Vec<AIResponseCard>> {
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: None,
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: None,
            model_used: None,
            cost_min: Some(min_cost),
            cost_max: Some(max_cost),
            limit: None,
            offset: None,
        };
        AIResponseCard::get_filtered(&*pool, filter).await
    }
    
    /// Update AI card
    pub async fn update(pool: &Pool<Sqlite>, id: &str, request: UpdateAICardRequest) -> Result<AIResponseCard> {
        AIResponseCard::update(&*pool, id, request).await
    }
    
    /// Delete AI card
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        AIResponseCard::delete(&*pool, id).await
    }
}
