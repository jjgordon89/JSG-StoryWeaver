//! AI Writing Commands for StoryWeaver

use crate::commands::CommandResponse;
use crate::error::StoryWeaverError;
use crate::ai::{AIProviderManager, AIContext, RewriteStyle, TextStream};
use serde::{Deserialize, Serialize};
use tauri::{State, Manager, Window};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use chrono;

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
                preceding_text: None,
                following_text: None,
                selected_text: None,
                characters: None,
                locations: None,
                plot_threads: None,
                writing_style: None,
                tone: None,
                creativity_level: None,
                feature_type: None,
                feature_options: None,
                word_count_target: None,
                genre: None,
                key_details: None,
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
    
    pub async fn auto_write_stream(&self, document_id: i32, cursor_position: usize, settings: WriteSettings) -> crate::error::Result<TextStream> {
        let context = self.context_builder.build_write_context(document_id, cursor_position, 1000).await?;
        
        let prompt = format!(
            "Continue this story naturally. Context: {}\n\nContinue from here:",
            context.preceding_text
        );
        
        let provider = self.ai_provider_manager.get_default_provider()
            .ok_or_else(|| crate::error::StoryWeaverError::ai_provider("default".to_string(), "No default AI provider found".to_string()))?;

        let stream = provider.generate_text_stream(&prompt, &context.ai_context).await
            .map_err(|e| crate::error::StoryWeaverError::ai_provider("default".to_string(), e.to_string()))?;
        
        Ok(stream)
    }
    
    pub async fn guided_write_stream(&self, document_id: i32, user_prompt: &str, settings: WriteSettings) -> crate::error::Result<TextStream> {
        let context = self.context_builder.build_write_context(document_id, 0, 1000).await?;
        
        let prompt = format!(
            "Write the next part of this story based on this direction: '{}'\n\nStory context: {}",
            user_prompt, context.story_summary
        );
        
        let provider = self.ai_provider_manager.get_default_provider()
            .ok_or_else(|| crate::error::StoryWeaverError::ai_provider("default".to_string(), "No default AI provider found".to_string()))?;

        let stream = provider.generate_text_stream(&prompt, &context.ai_context).await
            .map_err(|e| crate::error::StoryWeaverError::ai_provider("default".to_string(), e.to_string()))?;
        
        Ok(stream)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamChunk {
    pub content: String,
    pub is_complete: bool,
    pub token_count: usize,
    pub stream_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamStartResponse {
    pub stream_id: String,
    pub success: bool,
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

// Streaming Tauri Commands
#[tauri::command]
pub async fn auto_write_stream(
    state: State<'_, Arc<AIProviderManager>>,
    window: Window,
    document_id: i32,
    cursor_position: usize,
    settings: WriteSettings,
) -> CommandResponse<StreamStartResponse> {
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    let stream_id = format!("auto_write_{}_{}", document_id, chrono::Utc::now().timestamp_millis());
    let stream_id_clone = stream_id.clone();
    
    // Start streaming in a background task
    tokio::spawn(async move {
        match processor.auto_write_stream(document_id, cursor_position, settings).await {
            Ok(mut stream) => {
                // Simulate streaming by sending chunks
                let words: Vec<&str> = stream.content.split_whitespace().collect();
                let mut current_content = String::new();
                
                for (i, word) in words.iter().enumerate() {
                    current_content.push_str(word);
                    if i < words.len() - 1 {
                        current_content.push(' ');
                    }
                    
                    let chunk = StreamChunk {
                        content: current_content.clone(),
                        is_complete: i == words.len() - 1,
                        token_count: current_content.len() / 4, // Rough estimate
                        stream_id: stream_id_clone.clone(),
                    };
                    
                    // Emit the chunk to the frontend
                    if let Err(e) = window.emit("ai_stream_chunk", &chunk) {
                        eprintln!("Failed to emit stream chunk: {}", e);
                        break;
                    }
                    
                    // Add delay to simulate real streaming
                    sleep(Duration::from_millis(50)).await;
                }
            }
            Err(e) => {
                // Emit error
                if let Err(emit_err) = window.emit("ai_stream_error", format!("Auto write stream failed: {}", e)) {
                    eprintln!("Failed to emit stream error: {}", emit_err);
                }
            }
        }
    });
    
    CommandResponse::success(StreamStartResponse {
        stream_id,
        success: true,
    })
}

#[tauri::command]
pub async fn guided_write_stream(
    state: State<'_, Arc<AIProviderManager>>,
    window: Window,
    document_id: i32,
    user_prompt: String,
    settings: WriteSettings,
) -> CommandResponse<StreamStartResponse> {
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    let stream_id = format!("guided_write_{}_{}", document_id, chrono::Utc::now().timestamp_millis());
    let stream_id_clone = stream_id.clone();
    
    // Start streaming in a background task
    tokio::spawn(async move {
        match processor.guided_write_stream(document_id, &user_prompt, settings).await {
            Ok(mut stream) => {
                // Simulate streaming by sending chunks
                let words: Vec<&str> = stream.content.split_whitespace().collect();
                let mut current_content = String::new();
                
                for (i, word) in words.iter().enumerate() {
                    current_content.push_str(word);
                    if i < words.len() - 1 {
                        current_content.push(' ');
                    }
                    
                    let chunk = StreamChunk {
                        content: current_content.clone(),
                        is_complete: i == words.len() - 1,
                        token_count: current_content.len() / 4, // Rough estimate
                        stream_id: stream_id_clone.clone(),
                    };
                    
                    // Emit the chunk to the frontend
                    if let Err(e) = window.emit("ai_stream_chunk", &chunk) {
                        eprintln!("Failed to emit stream chunk: {}", e);
                        break;
                    }
                    
                    // Add delay to simulate real streaming
                    sleep(Duration::from_millis(50)).await;
                }
            }
            Err(e) => {
                // Emit error
                if let Err(emit_err) = window.emit("ai_stream_error", format!("Guided write stream failed: {}", e)) {
                    eprintln!("Failed to emit stream error: {}", emit_err);
                }
            }
        }
    });
    
    CommandResponse::success(StreamStartResponse {
        stream_id,
        success: true,
    })
}

// Additional AI Writing Tools

#[derive(Debug, Deserialize, Serialize)]
pub struct RewriteSettings {
    pub style: String, // "rephrase", "shorter", "longer", "more_formal", "more_casual", "more_descriptive", "simpler"
    pub creativity_level: u8,
    pub preserve_meaning: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpandSettings {
    pub focus: String, // "sensory_details", "dialogue", "action", "emotion", "setting"
    pub length_multiplier: f32, // 1.5, 2.0, 3.0
    pub creativity_level: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BrainstormSettings {
    pub category: String, // "characters", "plot_points", "settings", "conflicts", "themes"
    pub count: u8,
    pub creativity_level: u8,
}

#[tauri::command]
pub async fn rewrite_text(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    settings: RewriteSettings,
) -> CommandResponse<String> {
    match state.get_default_provider() {
        Some(provider) => {
            let rewrite_style = match settings.style.as_str() {
                "rephrase" => crate::ai::RewriteStyle::Rephrase,
                "shorter" => crate::ai::RewriteStyle::Shorter,
                "longer" => crate::ai::RewriteStyle::Longer,
                "more_formal" => crate::ai::RewriteStyle::MoreFormal,
                "more_casual" => crate::ai::RewriteStyle::MoreCasual,
                "more_descriptive" => crate::ai::RewriteStyle::MoreDescriptive,
                "simpler" => crate::ai::RewriteStyle::MoreDirect,
                _ => crate::ai::RewriteStyle::Rephrase,
            };
            
            match provider.rewrite_text(&text, &rewrite_style).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Rewrite failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn expand_text(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    settings: ExpandSettings,
) -> CommandResponse<String> {
    match state.get_default_provider() {
        Some(provider) => {
            let mut context = crate::ai::AIContext::default();
            context.selected_text = Some(text.clone());
            context.creativity_level = Some(settings.creativity_level);
            context.feature_options = Some({
                let mut options = std::collections::HashMap::new();
                options.insert("focus".to_string(), settings.focus);
                options.insert("length_multiplier".to_string(), settings.length_multiplier.to_string());
                options
            });
            
            match provider.expand_text(&text, &context).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Expand failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn describe_scene(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    focus: Option<String>,
) -> CommandResponse<String> {
    match state.get_default_provider() {
        Some(provider) => {
            let mut context = crate::ai::AIContext::default();
            context.selected_text = Some(text.clone());
            if let Some(focus_val) = focus {
                context.feature_options = Some({
                    let mut options = std::collections::HashMap::new();
                    options.insert("focus".to_string(), focus_val);
                    options
                });
            }
            
            match provider.describe_scene(&text, &context).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Describe scene failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn brainstorm(
    state: State<'_, Arc<AIProviderManager>>,
    prompt: String,
    settings: BrainstormSettings,
) -> CommandResponse<Vec<String>> {
    match state.get_default_provider() {
        Some(provider) => {
            let mut context = crate::ai::AIContext::default();
            context.creativity_level = Some(settings.creativity_level);
            context.feature_options = Some({
                let mut options = std::collections::HashMap::new();
                options.insert("category".to_string(), settings.category);
                options.insert("count".to_string(), settings.count.to_string());
                options
            });
            
            match provider.brainstorm(&prompt, &context).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Brainstorm failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn visualize_scene(
    state: State<'_, Arc<AIProviderManager>>,
    description: String,
) -> CommandResponse<String> {
    match state.get_default_provider() {
        Some(provider) => {
            match provider.generate_image(&description).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Visualize scene failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn quick_edit(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    instruction: String,
) -> CommandResponse<String> {
    match state.get_default_provider() {
        Some(provider) => {
            match provider.quick_edit(&text, &instruction).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Quick edit failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn quick_chat(
    state: State<'_, Arc<AIProviderManager>>,
    message: String,
    context: Option<String>,
) -> CommandResponse<String> {
    match state.get_default_provider() {
        Some(provider) => {
            let mut ai_context = crate::ai::AIContext::default();
            if let Some(ctx) = context {
                ai_context.story_context = Some(ctx);
            }
            
            match provider.quick_chat(&message, &ai_context).await {
                Ok(result) => CommandResponse::success(result),
                Err(e) => CommandResponse::error(format!("Quick chat failed: {}", e)),
            }
        }
        None => CommandResponse::error("No AI provider available".to_string()),
    }
}

#[tauri::command]
pub async fn tone_shift_write(
    state: State<'_, Arc<AIProviderManager>>,
    document_id: i32,
    cursor_position: usize,
    tone: String,
    settings: WriteSettings,
) -> CommandResponse<WriteResult> {
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    
    // Create modified settings with the specified tone
    let mut tone_settings = settings;
    tone_settings.tone = tone;
    
    match processor.auto_write(document_id, cursor_position, tone_settings).await {
        Ok(result) => CommandResponse::success(result),
        Err(e) => CommandResponse::error(format!("Tone shift write failed: {}", e)),
    }
}
