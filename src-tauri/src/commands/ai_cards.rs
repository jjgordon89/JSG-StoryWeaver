//! AI Response Card management commands

use crate::database::{get_pool, operations::AICardOps};
use crate::error::{Result, StoryWeaverError};
use crate::commands::CommandResponse;
use crate::models::ai_card::{AIResponseCard, CreateAIResponseCardRequest, UpdateAIResponseCardRequest};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Create a new AI response card
#[tauri::command]
pub async fn create_ai_response_card(
    request: CreateAIResponseCardRequest,
) -> CommandResponse<AIResponseCard> {
    async fn create(request: CreateAIResponseCardRequest) -> Result<AIResponseCard> {
        let pool = get_pool()?;
        AICardOps::create(&pool, request).await
    }
    
    create(request).await.into()
}

/// Get an AI response card by ID
#[tauri::command]
pub async fn get_ai_response_card(id: String) -> CommandResponse<AIResponseCard> {
    async fn get(id: String) -> Result<AIResponseCard> {
        let pool = get_pool()?;
        let card = AICardOps::get(&pool, &id).await?;
        card.ok_or_else(|| StoryWeaverError::NotFound { resource: "AI Response Card".to_string(), id })
    }
    
    get(id).await.into()
}

/// Get all AI response cards for a project
#[tauri::command]
pub async fn get_ai_response_cards_by_project(project_id: String) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_project(project_id: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Update an AI response card
#[tauri::command]
pub async fn update_ai_response_card(
    id: String,
    request: UpdateAIResponseCardRequest,
) -> CommandResponse<AIResponseCard> {
    async fn update(id: String, request: UpdateAIResponseCardRequest) -> Result<AIResponseCard> {
        let pool = get_pool()?;
        AICardOps::update(&pool, &id, request).await
    }
    
    update(id, request).await.into()
}

/// Delete an AI response card
#[tauri::command]
pub async fn delete_ai_response_card(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        AICardOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Get AI response cards by document
#[tauri::command]
pub async fn get_ai_response_cards_by_document(document_id: String) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_document(document_id: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_document(&pool, &document_id).await
    }
    
    get_by_document(document_id).await.into()
}

/// Get AI response cards by type
#[tauri::command]
pub async fn get_ai_response_cards_by_type(
    project_id: String,
    card_type: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_type(project_id: String, card_type: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_type(&pool, &project_id, &card_type).await
    }
    
    get_by_type(project_id, card_type).await.into()
}

/// Get AI response cards by status
#[tauri::command]
pub async fn get_ai_response_cards_by_status(
    project_id: String,
    status: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_status(project_id: String, status: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_status(&pool, &project_id, &status).await
    }
    
    get_by_status(project_id, status).await.into()
}

/// Get AI response cards by date range
#[tauri::command]
pub async fn get_ai_response_cards_by_date_range(
    project_id: String,
    start_date: String,
    end_date: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_date_range(project_id: String, start_date: String, end_date: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_date_range(&pool, &project_id, &start_date, &end_date).await
    }
    
    get_by_date_range(project_id, start_date, end_date).await.into()
}

/// Get AI response cards by provider
#[tauri::command]
pub async fn get_ai_response_cards_by_provider(
    project_id: String,
    provider: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_provider(project_id: String, provider: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_provider(&pool, &project_id, &provider).await
    }
    
    get_by_provider(project_id, provider).await.into()
}

/// Get AI response cards by model
#[tauri::command]
pub async fn get_ai_response_cards_by_model(
    project_id: String,
    model: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_model(project_id: String, model: String) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_model(&pool, &project_id, &model).await
    }
    
    get_by_model(project_id, model).await.into()
}

/// Get AI response cards by cost range
#[tauri::command]
pub async fn get_ai_response_cards_by_cost_range(
    project_id: String,
    min_cost: f64,
    max_cost: f64,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_cost_range(project_id: String, min_cost: f64, max_cost: f64) -> Result<Vec<AIResponseCard>> {
        let pool = get_pool()?;
        AICardOps::get_by_cost_range(&pool, &project_id, min_cost, max_cost).await
    }
    
    get_by_cost_range(project_id, min_cost, max_cost).await.into()
}
