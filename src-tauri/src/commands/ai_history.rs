//! AI generation history command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::AIHistoryOps};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use crate::security::rate_limit::{rl_create, rl_list, rl_delete};

/// Create AI history record request
#[derive(Debug, Deserialize)]
pub struct CreateAIHistoryRequest {
    pub project_id: String,
    pub document_id: Option<String>,
    pub generation_type: String,
    pub provider: String,
    pub model: String,
    pub prompt: String,
    pub response: String,
    pub token_count: Option<i32>,
    pub cost_estimate: Option<f64>,
    pub context_used: Option<String>,
}

/// AI usage statistics
#[derive(Debug, Serialize)]
pub struct AIUsageStats {
    pub total_generations: i32,
    pub total_tokens: i32,
    pub total_cost: f64,
    pub by_provider: std::collections::HashMap<String, ProviderStats>,
    pub by_type: std::collections::HashMap<String, TypeStats>,
    pub recent_generations: Vec<AIGenerationSummary>,
}

/// Provider-specific statistics
#[derive(Debug, Serialize)]
pub struct ProviderStats {
    pub count: i32,
    pub tokens: i32,
    pub cost: f64,
    pub models: std::collections::HashMap<String, i32>,
}

/// Generation type statistics
#[derive(Debug, Serialize)]
pub struct TypeStats {
    pub count: i32,
    pub tokens: i32,
    pub cost: f64,
}

/// AI generation summary for display
#[derive(Debug, Serialize)]
pub struct AIGenerationSummary {
    pub id: String,
    pub generation_type: String,
    pub provider: String,
    pub model: String,
    pub token_count: Option<i32>,
    pub cost_estimate: Option<f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub prompt_preview: String,
    pub response_preview: String,
}

/// Create a new AI generation history record
#[tauri::command]
pub async fn create_ai_history(request: CreateAIHistoryRequest) -> Result<AIGenerationHistory> {
    async fn create(request: CreateAIHistoryRequest) -> Result<AIGenerationHistory> {
        // Rate limiting
        rl_create("ai_history", Some(&request.project_id))?;
        
        // Validate input
        if request.project_id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Project ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&request.project_id)?;
        crate::security::validation::validate_security_input(&request.generation_type)?;
        crate::security::validation::validate_content_length(&request.provider, 100)?;
        crate::security::validation::validate_security_input(&request.provider)?;
        crate::security::validation::validate_content_length(&request.model, 100)?;
        crate::security::validation::validate_security_input(&request.model)?;
        crate::security::validation::validate_content_length(&request.prompt, 50000)?;
        crate::security::validation::validate_security_input(&request.prompt)?;
        crate::security::validation::validate_content_length(&request.response, 100000)?;
        crate::security::validation::validate_security_input(&request.response)?;
        
        if let Some(ref doc_id) = request.document_id {
            crate::security::validation::validate_security_input(doc_id)?;
        }
        
        if let Some(ref context) = request.context_used {
            crate::security::validation::validate_content_length(context, 10000)?;
            crate::security::validation::validate_security_input(context)?;
        }
        
        if let Some(token_count) = request.token_count {
            if token_count < 0 {
                return Err(crate::error::StoryWeaverError::InvalidInput { message: "token_count cannot be negative".to_string() });
            }
        }
        
        if let Some(cost) = request.cost_estimate {
            if cost < 0.0 {
                return Err(crate::error::StoryWeaverError::InvalidInput { message: "cost_estimate cannot be negative".to_string() });
            }
        }
        
        let pool = get_pool()?;
        
        let record = AIGenerationHistory {
            id: String::new(), // Will be set by AIHistoryOps::create
            project_id: request.project_id,
            document_id: request.document_id,
            generation_type: match request.generation_type.as_str() {
                "auto_write" => AIGenerationType::AutoWrite,
                "expand" => AIGenerationType::Expand,
                "rewrite" => AIGenerationType::Rewrite,
                "describe" => AIGenerationType::Describe,
                "brainstorm" => AIGenerationType::Brainstorm,
                "outline" => AIGenerationType::Outline,
                "character_development" => AIGenerationType::CharacterDevelopment,
                "world_building" => AIGenerationType::WorldBuilding,
                _ => return Err(crate::error::StoryWeaverError::InvalidInput { message: format!("Invalid generation type: {}", request.generation_type) }),
            },
            provider: request.provider,
            model: request.model,
            prompt: request.prompt,
            response: request.response,
            token_count: request.token_count.unwrap_or(0),
            cost_estimate: request.cost_estimate,
            context_used: request.context_used.unwrap_or_else(|| "{}".to_string()),
            created_at: chrono::Utc::now(),
        };
        
        AIHistoryOps::create(&pool, record).await
    }
    
    create(request).await
}

/// Get AI generation history for a project
#[tauri::command]
pub async fn get_ai_history(project_id: String, limit: Option<i32>) -> Result<Vec<AIGenerationHistory>> {
    async fn get_history(project_id: String, limit: Option<i32>) -> Result<Vec<AIGenerationHistory>> {
        // Rate limiting
        rl_list("ai_history", Some(&project_id))?;
        
        // Validate input
        if project_id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Project ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        
        if let Some(limit_val) = limit {
            if limit_val < 0 {
                return Err(crate::error::StoryWeaverError::InvalidInput { message: "limit cannot be negative".to_string() });
            }
        }
        
        let pool = get_pool()?;
        AIHistoryOps::get_by_project(&pool, &project_id, limit).await
    }
    
    get_history(project_id, limit).await
}

/// Get AI usage statistics for a project
#[tauri::command]
pub async fn get_ai_usage_stats(project_id: String) -> Result<AIUsageStats> {
    async fn get_stats(project_id: String) -> Result<AIUsageStats> {
        // Rate limiting
        rl_list("ai_stats", Some(&project_id))?;
        
        // Validate input
        if project_id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Project ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        let history = AIHistoryOps::get_by_project(&pool, &project_id, None).await?;
        
        let total_generations = history.len() as i32;
        let total_tokens = history.iter().map(|h| h.token_count).sum();
        let total_cost = history.iter().map(|h| h.cost_estimate.unwrap_or(0.0)).sum();
        
        // Group by provider
        let mut by_provider: std::collections::HashMap<String, ProviderStats> = std::collections::HashMap::new();
        for record in &history {
            let provider_stats = by_provider.entry(record.provider.clone()).or_insert(ProviderStats {
                count: 0,
                tokens: 0,
                cost: 0.0,
                models: std::collections::HashMap::new(),
            });
            
            provider_stats.count += 1;
            provider_stats.tokens += record.token_count;
            provider_stats.cost += record.cost_estimate.unwrap_or(0.0);
            *provider_stats.models.entry(record.model.clone()).or_insert(0) += 1;
        }
        
        // Group by generation type
        let mut by_type: std::collections::HashMap<String, TypeStats> = std::collections::HashMap::new();
        for record in &history {
            let type_key = format!("{:?}", record.generation_type);
            let type_stats = by_type.entry(type_key).or_insert(TypeStats {
                count: 0,
                tokens: 0,
                cost: 0.0,
            });
            
            type_stats.count += 1;
            type_stats.tokens += record.token_count;
            type_stats.cost += record.cost_estimate.unwrap_or(0.0);
        }
        
        // Get recent generations (last 10)
        let mut recent_history = history.clone();
        recent_history.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        recent_history.truncate(10);
        
        let recent_generations = recent_history
            .into_iter()
            .map(|record| {
                let prompt_preview = if record.prompt.len() > 100 {
                    format!("{}...", &record.prompt[..97])
                } else {
                    record.prompt.clone()
                };
                
                let response_preview = if record.response.len() > 100 {
                    format!("{}...", &record.response[..97])
                } else {
                    record.response.clone()
                };
                
                AIGenerationSummary {
                    id: record.id,
                    generation_type: format!("{:?}", record.generation_type),
                    provider: record.provider,
                    model: record.model,
                    token_count: Some(record.token_count),
                    cost_estimate: record.cost_estimate,
                    created_at: record.created_at,
                    prompt_preview,
                    response_preview,
                }
            })
            .collect();
        
        Ok(AIUsageStats {
            total_generations,
            total_tokens,
            total_cost,
            by_provider,
            by_type,
            recent_generations,
        })
    }
    
    get_stats(project_id).await
}

/// Get AI generation history by document
#[tauri::command]
pub async fn get_ai_history_by_document(document_id: String) -> Result<Vec<AIGenerationHistory>> {
    async fn get_by_document(document_id: String) -> Result<Vec<AIGenerationHistory>> {
        // Rate limiting
        rl_list("ai_history_by_document", Some(&document_id))?;
        
        // Validate input
        if document_id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Document ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&document_id)?;
        
        let pool = get_pool()?;
        
        let history = sqlx::query_as::<_, AIGenerationHistory>(
            "SELECT * FROM ai_generation_history WHERE document_id = ? ORDER BY created_at DESC"
        )
        .bind(&document_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get AI history: {}", e)))?;
        
        Ok(history)
    }
    
    get_by_document(document_id).await
}

/// Delete AI generation history record
#[tauri::command]
pub async fn delete_ai_history(id: String) -> Result<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("ai_history", Some(&id))?;
        
        // Validate input
        if id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&id)?;
        
        let pool = get_pool()?;
        
        sqlx::query("DELETE FROM ai_generation_history WHERE id = ?")
            .bind(&id)
            .execute(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to delete AI history: {}", e)))?;
        
        Ok(())
    }
    
    delete(id).await
}

/// Clear AI generation history for a project
#[tauri::command]
pub async fn clear_ai_history(project_id: String) -> Result<()> {
    async fn clear(project_id: String) -> Result<()> {
        // Rate limiting
        rl_delete("ai_history_by_project", Some(&project_id))?;
        
        // Validate input
        if project_id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Project ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?
        
        let pool = get_pool()?;
        
        sqlx::query("DELETE FROM ai_generation_history WHERE project_id = ?")
            .bind(&project_id)
            .execute(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to clear AI history: {}", e)))?;
        
        Ok(())
    }
    
    clear(project_id).await
}

/// AI cost estimation for different providers and models
#[derive(Debug, Serialize)]
pub struct CostEstimate {
    pub provider: String,
    pub model: String,
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub estimated_cost: f64,
}

/// Calculate cost estimate for AI generation
#[tauri::command]
pub async fn calculate_cost_estimate(
    provider: String,
    model: String,
    input_tokens: i32,
    output_tokens: i32,
) -> Result<CostEstimate> {
    async fn calculate(provider: String, model: String, input_tokens: i32, output_tokens: i32) -> Result<CostEstimate> {
        // Rate limiting
        rl_list("ai_cost", None)?;
        
        // Input validation
        crate::security::validation::validate_security_input(&provider)?;
        crate::security::validation::validate_security_input(&model)?;
        
        if input_tokens < 0 {
            return Err(crate::error::StoryWeaverError::InvalidInput { message: "input_tokens cannot be negative".to_string() });
        }
        
        if output_tokens < 0 {
            return Err(crate::error::StoryWeaverError::InvalidInput { message: "output_tokens cannot be negative".to_string() });
        }
        // Cost calculation based on provider and model
        // These are approximate rates as of 2024 - should be updated regularly
        let estimated_cost = match provider.to_lowercase().as_str() {
            "openai" => {
                match model.as_str() {
                    "gpt-4" => {
                        (input_tokens as f64 * 0.00003) + (output_tokens as f64 * 0.00006)
                    }
                    "gpt-4-turbo" => {
                        (input_tokens as f64 * 0.00001) + (output_tokens as f64 * 0.00003)
                    }
                    "gpt-3.5-turbo" => {
                        (input_tokens as f64 * 0.0000015) + (output_tokens as f64 * 0.000002)
                    }
                    _ => {
                        // Default to GPT-3.5-turbo pricing
                        (input_tokens as f64 * 0.0000015) + (output_tokens as f64 * 0.000002)
                    }
                }
            }
            "anthropic" => {
                match model.as_str() {
                    "claude-3-opus" => {
                        (input_tokens as f64 * 0.000015) + (output_tokens as f64 * 0.000075)
                    }
                    "claude-3-sonnet" => {
                        (input_tokens as f64 * 0.000003) + (output_tokens as f64 * 0.000015)
                    }
                    "claude-3-haiku" => {
                        (input_tokens as f64 * 0.00000025) + (output_tokens as f64 * 0.00000125)
                    }
                    _ => {
                        // Default to Claude-3-haiku pricing
                        (input_tokens as f64 * 0.00000025) + (output_tokens as f64 * 0.00000125)
                    }
                }
            }
            "google" => {
                match model.as_str() {
                    "gemini-pro" => {
                        (input_tokens as f64 * 0.0000005) + (output_tokens as f64 * 0.0000015)
                    }
                    "gemini-pro-vision" => {
                        (input_tokens as f64 * 0.0000005) + (output_tokens as f64 * 0.0000015)
                    }
                    _ => {
                        // Default to Gemini Pro pricing
                        (input_tokens as f64 * 0.0000005) + (output_tokens as f64 * 0.0000015)
                    }
                }
            }
            "ollama" | "local" => {
                // Local models have no API cost
                0.0
            }
            _ => {
                // Unknown provider, use conservative estimate
                (input_tokens as f64 * 0.000001) + (output_tokens as f64 * 0.000002)
            }
        };
        
        Ok(CostEstimate {
            provider,
            model,
            input_tokens,
            output_tokens,
            estimated_cost,
        })
    }
    
    calculate(provider, model, input_tokens, output_tokens).await
}

/// Export AI generation history to CSV
#[tauri::command]
pub async fn export_ai_history(project_id: String) -> Result<String> {
    async fn export(project_id: String) -> Result<String> {
        // Rate limiting
        rl_list("ai_history_export", Some(&project_id))?;
        
        // Validate input
        if project_id.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Project ID cannot be empty".to_string(),
            });
        }
        
        // Input validation
        crate::security::validation::validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        let history = AIHistoryOps::get_by_project(&pool, &project_id, None).await?;
        
        let mut csv_content = String::new();
        csv_content.push_str("ID,Project ID,Document ID,Generation Type,Provider,Model,Token Count,Cost Estimate,Created At,Prompt Preview,Response Preview\n");
        
        for record in history {
            let prompt_preview = record.prompt.chars().take(50).collect::<String>().replace('"', "\"\"");
            let response_preview = record.response.chars().take(50).collect::<String>().replace('"', "\"\"");
            
            csv_content.push_str(&format!(
                "{},{},{},{:?},{},{},{},{},{},\"{}\",\"{}\"
",
                record.id,
                record.project_id,
                record.document_id.unwrap_or_else(|| "N/A".to_string()),
                record.generation_type,
                record.provider,
                record.model,
                record.token_count,
                record.cost_estimate.unwrap_or(0.0),
                record.created_at.format("%Y-%m-%d %H:%M:%S"),
                prompt_preview,
                response_preview
            ));
        }
        
        Ok(csv_content)
    }
    
    export(project_id).await
}
