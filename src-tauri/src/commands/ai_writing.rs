//! AI Writing Commands for StoryWeaver

use crate::commands::CommandResponse;
use crate::error::StoryWeaverError;
use crate::ai::{AIProviderManager, AIContext, RewriteStyle};
use serde::{Deserialize, Serialize};
use tauri::{State, Manager};
use std::sync::Arc;
use std::collections::HashMap;

// Placeholder structs for WriteProcessor and its dependencies
pub struct ContextBuilder;
impl ContextBuilder {
    pub async fn build_write_context(&self, _document_id: i32, _cursor_position: usize, _word_count: usize) -> crate::error::Result<WriteContext> {
        // Placeholder implementation
        Ok(WriteContext {
            preceding_text: "This is some preceding text.".to_string(),
            story_summary: "A brief story summary.".to_string(),
            ai_context: AIContext {
                project_id: Some("placeholder_project_id".to_string()),
                document_id: Some(_document_id.to_string()),
                story_context: Some("A brief story context.".to_string()),
                user_preferences: Some(HashMap::new()),
            },
        })
    }
}

pub struct WriteContext {
    pub preceding_text: String,
    pub story_summary: String,
    pub ai_context: AIContext,
}

pub struct WriteProcessor {
    ai_provider_manager: Arc<AIProviderManager>,
    context_builder: ContextBuilder,
}

impl WriteProcessor {
    pub fn new(ai_provider_manager: Arc<AIProviderManager>, context_builder: ContextBuilder) -> Self {
        Self {
            ai_provider_manager,
            context_builder,
        }
    }

    pub async fn auto_write(&self, document_id: i32, cursor_position: usize, settings: WriteSettings) -> crate::error::Result<WriteResult> { // Changed return type to Result
        let context = self.context_builder.build_write_context(document_id, cursor_position, 1000).await?;
        
        let prompt = format!(
            "Continue this story naturally. Context: {}\n\nContinue from here:",
            context.preceding_text
        );
        
        let provider = self.ai_provider_manager.get_default_provider()
            .ok_or_else(|| crate::error::StoryWeaverError::ai_provider("default".to_string(), "No default AI provider found".to_string()))?;

        let generated_text = provider.generate_text(&prompt, &context.ai_context).await
            .map_err(|e| crate::error::StoryWeaverError::ai_provider("default".to_string(), e.to_string()))?; // Map error to StoryWeaverError
        
        Ok(WriteResult {
            generated_text,
            credits_used: 10, // Placeholder
            word_count: 100, // Placeholder
        })
    }
    
    pub async fn guided_write(&self, document_id: i32, user_prompt: &str, settings: WriteSettings) -> crate::error::Result<WriteResult> { // Changed return type to Result
        let context = self.context_builder.build_write_context(document_id, 0, 1000).await?;
        
        let prompt = format!(
            "Write the next part of this story based on this direction: '{}'\n\nStory context: {}",
            user_prompt, context.story_summary
        );
        
        let provider = self.ai_provider_manager.get_default_provider()
            .ok_or_else(|| crate::error::StoryWeaverError::ai_provider("default".to_string(), "No default AI provider found".to_string()))?;

        let generated_text = provider.generate_text(&prompt, &context.ai_context).await
            .map_err(|e| crate::error::StoryWeaverError::ai_provider("default".to_string(), e.to_string()))?; // Map error to StoryWeaverError
        
        Ok(WriteResult {
            generated_text,
            credits_used: 15, // Placeholder
            word_count: 150, // Placeholder
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteSettings {
    pub creativity_level: u8,
    pub tone: String,
    pub key_details: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteResult {
    pub generated_text: String,
    pub credits_used: u32,
    pub word_count: usize,
}

// Tauri Commands for AI Writing
#[tauri::command]
pub async fn auto_write(
    state: State<'_, Arc<AIProviderManager>>,
    document_id: i32,
    cursor_position: usize,
    settings: WriteSettings,
) -> CommandResponse<WriteResult> {
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    
    match processor.auto_write(document_id, cursor_position, settings).await {
        Ok(result) => CommandResponse::success(result),
        Err(e) => CommandResponse::error(format!("Auto write failed: {}", e)), // More specific error message
    }
}

#[tauri::command]
pub async fn guided_write(
    state: State<'_, Arc<AIProviderManager>>,
    document_id: i32,
    user_prompt: String,
    settings: WriteSettings,
) -> CommandResponse<WriteResult> {
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    
    match processor.guided_write(document_id, &user_prompt, settings).await {
        Ok(result) => CommandResponse::success(result),
        Err(e) => CommandResponse::error(format!("Guided write failed: {}", e)), // More specific error message
    }
}
