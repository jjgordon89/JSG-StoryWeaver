//! AI Response Card management commands

use crate::database::{get_pool, operations::ai_card_ops::AICardOps};
use crate::error::Result;
use crate::commands::CommandResponse;
use crate::models::ai_card::{AIResponseCard, CreateAICardRequest, UpdateAICardRequest};
use crate::security::rate_limit::{rl_create, rl_update, rl_delete, rl_list, rl_search};

/// Create a new AI response card
#[tauri::command]
pub async fn create_ai_response_card(
    request: CreateAICardRequest,
) -> CommandResponse<AIResponseCard> {
    async fn inner_create(request: CreateAICardRequest) -> Result<AIResponseCard> {
        // Rate limiting
        rl_create("ai_response_card", Some(&request.project_id))?;
        // Input validation
        crate::security::validation::validate_security_input(&request.project_id)?;
        crate::security::validation::validate_security_input(&request.feature_type)?;
        crate::security::validation::validate_content_length(&request.response_text, 50000)?;
        crate::security::validation::validate_security_input(&request.response_text)?;
        
        if let Some(ref doc_id) = request.document_id {
            crate::security::validation::validate_security_input(doc_id)?;
        }
        
        if let Some(ref model_used) = request.model_used {
            crate::security::validation::validate_content_length(model_used, 100)?;
            crate::security::validation::validate_security_input(model_used)?;
        }
        
        let pool = get_pool()?;
        AICardOps::create(&pool, request).await
    }
    
    match inner_create(request).await {
        Ok(card) => CommandResponse::success(card),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get an AI response card by ID
#[tauri::command]
pub async fn get_ai_response_card(id: String) -> CommandResponse<AIResponseCard> {
    async fn inner_get(id: String) -> Result<AIResponseCard> {
        // Rate limiting
        rl_list("ai_response_card", Some(&id))?;
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        let pool = get_pool()?;
        AICardOps::get(&pool, &id).await
    }
    
    match inner_get(id).await {
        Ok(card) => CommandResponse::success(card),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Update an AI response card
#[tauri::command]
pub async fn update_ai_response_card(
    id: String,
    request: UpdateAICardRequest,
) -> CommandResponse<AIResponseCard> {
    async fn inner_update(id: String, request: UpdateAICardRequest) -> Result<AIResponseCard> {
        // Rate limiting
        rl_update("ai_response_card", Some(&id))?;
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        // Update request only supports a small set of mutable fields.
        // Validate those present.
        if let Some(is_stacked) = request.is_stacked {
            let _ = is_stacked;
        }
        if let Some(is_starred) = request.is_starred {
            let _ = is_starred;
        }
        if let Some(is_collapsed) = request.is_collapsed {
            let _ = is_collapsed;
        }
        if let Some(stack_position) = request.stack_position {
            let _ = stack_position;
        }
        if let Some(ref tags) = request.tags {
            crate::security::validation::validate_content_length(tags, 200)?;
            crate::security::validation::validate_security_input(tags)?;
        }
        
        
        let pool = get_pool()?;
        AICardOps::update(&pool, &id, request).await
    }
    
    match inner_update(id, request).await {
        Ok(card) => CommandResponse::success(card),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Delete an AI response card
#[tauri::command]
pub async fn delete_ai_response_card(id: String) -> CommandResponse<()> {
    async fn inner_delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("ai_response_card", Some(&id))?;
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        let pool = get_pool()?;
        AICardOps::delete(&pool, &id).await
    }
    
    match inner_delete(id).await {
        Ok(_) => CommandResponse::success(()),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by document
#[tauri::command]
pub async fn get_ai_response_cards_by_document(document_id: String) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_document(document_id: String) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_list("ai_response_cards_by_document", Some(&document_id))?;
        // Input validation
        crate::security::validation::validate_security_input(&document_id)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_document(&pool, &document_id).await
    }
    
    match inner_get_by_document(document_id).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by type
#[tauri::command]
pub async fn get_ai_response_cards_by_type(
    project_id: String,
    card_type: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_type(project_id: String, card_type: String) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_search("ai_response_cards_by_type", Some(&format!("{}:{}", &project_id, &card_type)))?;
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_security_input(&card_type)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_type(&pool, &project_id, &card_type).await
    }
    
    match inner_get_by_type(project_id, card_type).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by status
#[tauri::command]
pub async fn get_ai_response_cards_by_status(
    project_id: String,
    status: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_status(project_id: String, status: String) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_search("ai_response_cards_by_status", Some(&format!("{}:{}", &project_id, &status)))?;
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_security_input(&status)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_status(&pool, &project_id, &status).await
    }
    
    match inner_get_by_status(project_id, status).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by date range
#[tauri::command]
pub async fn get_ai_response_cards_by_date_range(
    project_id: String,
    start_date: String,
    end_date: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_date_range(project_id: String, start_date: String, end_date: String) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_search("ai_response_cards_by_date_range", Some(&format!("{}:{}-{}", &project_id, &start_date, &end_date)))?;
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_security_input(&start_date)?;
        crate::security::validation::validate_security_input(&end_date)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_date_range(&pool, &project_id, &start_date, &end_date).await
    }
    
    match inner_get_by_date_range(project_id, start_date, end_date).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by provider
#[tauri::command]
pub async fn get_ai_response_cards_by_provider(
    project_id: String,
    provider: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_provider(project_id: String, provider: String) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_search("ai_response_cards_by_provider", Some(&format!("{}:{}", &project_id, &provider)))?;
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_content_length(&provider, 100)?;
        crate::security::validation::validate_security_input(&provider)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_provider(&pool, &project_id, &provider).await
    }
    
    match inner_get_by_provider(project_id, provider).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by model
#[tauri::command]
pub async fn get_ai_response_cards_by_model(
    project_id: String,
    model: String,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_model(project_id: String, model: String) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_search("ai_response_cards_by_model", Some(&format!("{}:{}", &project_id, &model)))?;
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_content_length(&model, 100)?;
        crate::security::validation::validate_security_input(&model)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_model(&pool, &project_id, &model).await
    }
    
    match inner_get_by_model(project_id, model).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}

/// Get AI response cards by cost range
#[tauri::command]
pub async fn get_ai_response_cards_by_cost_range(
    project_id: String,
    min_cost: f64,
    max_cost: f64,
) -> CommandResponse<Vec<AIResponseCard>> {
    async fn inner_get_by_cost_range(project_id: String, min_cost: f64, max_cost: f64) -> Result<Vec<AIResponseCard>> {
        // Rate limiting
        rl_search("ai_response_cards_by_cost_range", Some(&format!("{}:{}-{}", &project_id, min_cost, max_cost)))?;
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        
        if min_cost < 0.0 {
            return Err(crate::error::StoryWeaverError::validation("min_cost cannot be negative"));
        }
        
        if max_cost < 0.0 {
            return Err(crate::error::StoryWeaverError::validation("max_cost cannot be negative"));
        }
        
        if min_cost > max_cost {
            return Err(crate::error::StoryWeaverError::validation("min_cost cannot be greater than max_cost"));
        }
        
        let pool = get_pool()?;
        AICardOps::get_by_cost_range(&pool, &project_id, min_cost, max_cost).await
    }
    
    match inner_get_by_cost_range(project_id, min_cost, max_cost).await {
        Ok(cards) => CommandResponse::success(cards),
        Err(e) => CommandResponse::error(e.to_string()),
    }
}
