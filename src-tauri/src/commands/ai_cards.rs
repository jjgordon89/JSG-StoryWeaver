//! AI Response Card management commands

use crate::models::ai_card::{AIResponseCard, CreateAICardRequest, UpdateAICardRequest, AICardFilter};
use crate::database::get_pool;
use crate::error::Result;

#[tauri::command]
pub async fn create_ai_card(
    request: CreateAICardRequest,
) -> Result<AIResponseCard, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    AIResponseCard::create(&pool, request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_card(
    id: String,
) -> Result<AIResponseCard, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    AIResponseCard::get_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_cards(
    filter: AICardFilter,
) -> Result<Vec<AIResponseCard>, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    AIResponseCard::get_filtered(&pool, filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_ai_card(
    id: String,
    request: UpdateAICardRequest,
) -> Result<AIResponseCard, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    AIResponseCard::update(&pool, &id, request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_ai_card(
    id: String,
) -> Result<(), String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    AIResponseCard::delete(&pool, &id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_cards_by_project(
    project_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<AIResponseCard>, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    let filter = AICardFilter {
        project_id: Some(project_id),
        document_id: None,
        feature_type: None,
        is_stacked: None,
        is_starred: None,
        limit,
        offset,
    };
    
    AIResponseCard::get_filtered(&pool, filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_cards_by_document(
    document_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<AIResponseCard>, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    let filter = AICardFilter {
        project_id: None,
        document_id: Some(document_id),
        feature_type: None,
        is_stacked: None,
        is_starred: None,
        limit,
        offset,
    };
    
    AIResponseCard::get_filtered(&pool, filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_stacked_ai_cards(
    project_id: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<AIResponseCard>, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    let filter = AICardFilter {
        project_id,
        document_id: None,
        feature_type: None,
        is_stacked: Some(true),
        is_starred: None,
        limit,
        offset,
    };
    
    AIResponseCard::get_filtered(&pool, filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_starred_ai_cards(
    project_id: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<AIResponseCard>, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    let filter = AICardFilter {
        project_id,
        document_id: None,
        feature_type: None,
        is_stacked: None,
        is_starred: Some(true),
        limit,
        offset,
    };
    
    AIResponseCard::get_filtered(&pool, filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_ai_card_stack(
    id: String,
) -> Result<AIResponseCard, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    // Get current card to toggle its stacked state
    let current_card = AIResponseCard::get_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    
    let request = UpdateAICardRequest {
        is_stacked: Some(!current_card.is_stacked),
        is_starred: None,
        is_collapsed: None,
        stack_position: None,
        tags: None,
    };
    
    AIResponseCard::update(&pool, &id, request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_ai_card_star(
    id: String,
) -> Result<AIResponseCard, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    // Get current card to toggle its starred state
    let current_card = AIResponseCard::get_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    
    let request = UpdateAICardRequest {
        is_stacked: None,
        is_starred: Some(!current_card.is_starred),
        is_collapsed: None,
        stack_position: None,
        tags: None,
    };
    
    AIResponseCard::update(&pool, &id, request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_ai_card_collapse(
    id: String,
) -> Result<AIResponseCard, String> {
    let pool = get_pool().map_err(|e| e.to_string())?;
    
    // Get current card to toggle its collapsed state
    let current_card = AIResponseCard::get_by_id(&pool, &id)
        .await
        .map_err(|e| e.to_string())?;
    
    let request = UpdateAICardRequest {
        is_stacked: None,
        is_starred: None,
        is_collapsed: Some(!current_card.is_collapsed),
        stack_position: None,
        tags: None,
    };
    
    AIResponseCard::update(&pool, &id, request)
        .await
        .map_err(|e| e.to_string())
}
