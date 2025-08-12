//! AI Response Card management commands

use crate::database::{get_pool, operations::ai_card_ops::AICardOps};
use crate::error::Result;
use crate::commands::CommandResponse;
use crate::models::ai_card::{AIResponseCard, CreateAICardRequest, UpdateAICardRequest};

/// Create a new AI response card
#[tauri::command]
pub async fn create_ai_response_card(
    request: CreateAICardRequest,
) -> CommandResponse<AIResponseCard> {
    async fn create(request: CreateAICardRequest) -> Result<AIResponseCard> {
        // Input validation
        crate::security::validation::validate_security_input(&request.project_id)?;
        crate::security::validation::validate_security_input(&request.card_type)?;
        crate::security::validation::validate_content_length(&request.content, 50000)?;
        crate::security::validation::validate_security_input(&request.content)?;
        
        if let Some(ref doc_id) = request.document_id {
            crate::security::validation::validate_security_input(doc_id)?;
        }
        
        if let Some(ref provider) = request.provider {
            crate::security::validation::validate_content_length(provider, 100)?;
            crate::security::validation::validate_security_input(provider)?;
        }
        
        if let Some(ref model) = request.model {
            crate::security::validation::validate_content_length(model, 100)?;
            crate::security::validation::validate_security_input(model)?;
        }
        
        let pool = get_pool()?;
        AICardOps::create(&pool, request).await
    }
    
    create(request).await.into()
}

/// Get an AI response card by ID
#[tauri::command]
pub async fn get_ai_response_card(id: String) -> CommandResponse<AIResponseCard> {
    async fn get(id: String) -> Result<AIResponseCard> {
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        let pool = get_pool()?;
        AICardOps::get(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Get all AI response cards for a project
#[tauri::command]
pub async fn get_ai_response_cards_by_project(project_id: String) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_project(project_id: String) -> Result<Vec<AIResponseCard>> {
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        AICardOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Update an AI response card
#[tauri::command]
pub async fn update_ai_response_card(
    id: String,
    request: UpdateAICardRequest,
) -> CommandResponse<AIResponseCard> {
    async fn update(id: String, request: UpdateAICardRequest) -> Result<AIResponseCard> {
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        if let Some(ref content) = request.content {
            crate::security::validation::validate_content_length(content, 50000)?;
            crate::security::validation::validate_security_input(content)?;
        }
        
        if let Some(ref card_type) = request.card_type {
            crate::security::validation::validate_security_input(card_type)?;
        }
        
        if let Some(ref provider) = request.provider {
            crate::security::validation::validate_content_length(provider, 100)?;
            crate::security::validation::validate_security_input(provider)?;
        }
        
        if let Some(ref model) = request.model {
            crate::security::validation::validate_content_length(model, 100)?;
            crate::security::validation::validate_security_input(model)?;
        }
        
        let pool = get_pool()?;
        AICardOps::update(&pool, &id, request).await
    }
    
    update(id, request).await.into()
}

/// Delete an AI response card
#[tauri::command]
pub async fn delete_ai_response_card(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        let pool = get_pool()?;
        AICardOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Get AI response cards by document
#[tauri::command]
pub async fn get_ai_response_cards_by_document(document_id: String) -> CommandResponse<Vec<AIResponseCard>> {
    async fn get_by_document(document_id: String) -> Result<Vec<AIResponseCard>> {
        // Input validation
        crate::security::validation::validate_security_input(&document_id)?;
        
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
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_security_input(&card_type)?;
        
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
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_security_input(&status)?;
        
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
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_security_input(&start_date)?;
        crate::security::validation::validate_security_input(&end_date)?;
        
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
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_content_length(&provider, 100)?;
        crate::security::validation::validate_security_input(&provider)?;
        
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
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        crate::security::validation::validate_content_length(&model, 100)?;
        crate::security::validation::validate_security_input(&model)?;
        
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
    
    get_by_cost_range(project_id, min_cost, max_cost).await.into()
}
