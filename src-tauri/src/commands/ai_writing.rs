//! AI Writing Commands for StoryWeaver

use crate::error::{StoryWeaverError, Result};
use crate::ai::{AIProviderManager, AIContext, TextStream};
use crate::security::rate_limit::{rl_create, rl_update, rl_list};
use crate::security::validators::{validate_non_empty_str, validate_body_limits, validate_optional_str};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, State, Window};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use chrono;

/// Validate WriteSettings input
fn validate_write_settings(settings: &WriteSettings) -> Result<()> {
    // Validate creativity level (1-10)
    if settings.creativity_level == 0 || settings.creativity_level > 10 {
        return Err(StoryWeaverError::ValidationError {
            message: "Creativity level must be between 1 and 10".to_string(),
        });
    }
    
    // Validate tone
    validate_non_empty_str("tone", &settings.tone, 100)?;
    
    // Validate key details
    validate_body_limits("key_details", &settings.key_details, 5_000, 5_000)?;
    
    Ok(())
}

/// Validate RewriteSettings input
fn validate_rewrite_settings(settings: &RewriteSettings) -> Result<()> {
    // Validate creativity level (1-10)
    if settings.creativity_level == 0 || settings.creativity_level > 10 {
        return Err(StoryWeaverError::ValidationError {
            message: "Creativity level must be between 1 and 10".to_string(),
        });
    }
    
    // Validate style
    let valid_styles = ["rephrase", "shorter", "longer", "more_formal", "more_casual", "more_descriptive", "simpler"];
    if !valid_styles.contains(&settings.style.as_str()) {
        return Err(StoryWeaverError::ValidationError {
            message: "Invalid rewrite style".to_string(),
        });
    }
    
    Ok(())
}

/// Validate ExpandSettings input
fn validate_expand_settings(settings: &ExpandSettings) -> Result<()> {
    // Validate creativity level (1-10)
    if settings.creativity_level == 0 || settings.creativity_level > 10 {
        return Err(StoryWeaverError::ValidationError {
            message: "Creativity level must be between 1 and 10".to_string(),
        });
    }
    
    // Validate focus
    let valid_focuses = ["sensory_details", "dialogue", "action", "emotion", "setting"];
    if !valid_focuses.contains(&settings.focus.as_str()) {
        return Err(StoryWeaverError::ValidationError {
            message: "Invalid expand focus".to_string(),
        });
    }
    
    // Validate length multiplier
    if settings.length_multiplier < 1.0 || settings.length_multiplier > 5.0 {
        return Err(StoryWeaverError::ValidationError {
            message: "Length multiplier must be between 1.0 and 5.0".to_string(),
        });
    }
    
    Ok(())
}

/// Validate BrainstormSettings input
fn validate_brainstorm_settings(settings: &BrainstormSettings) -> Result<()> {
    // Validate creativity level (1-10)
    if settings.creativity_level == 0 || settings.creativity_level > 10 {
        return Err(StoryWeaverError::ValidationError {
            message: "Creativity level must be between 1 and 10".to_string(),
        });
    }
    
    // Validate category
    let valid_categories = ["characters", "plot_points", "settings", "conflicts", "themes"];
    if !valid_categories.contains(&settings.category.as_str()) {
        return Err(StoryWeaverError::ValidationError {
            message: "Invalid brainstorm category".to_string(),
        });
    }
    
    // Validate count
    if settings.count == 0 || settings.count > 20 {
        return Err(StoryWeaverError::ValidationError {
            message: "Brainstorm count must be between 1 and 20".to_string(),
        });
    }
    
    Ok(())
}

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
    
    pub async fn auto_write_stream(&self, document_id: i32, cursor_position: usize, _settings: WriteSettings) -> crate::error::Result<TextStream> {
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
    
    pub async fn guided_write_stream(&self, document_id: i32, user_prompt: &str, _settings: WriteSettings) -> crate::error::Result<TextStream> {
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

    pub async fn auto_write(&self, document_id: i32, cursor_position: usize, _settings: WriteSettings) -> crate::error::Result<WriteResult> { // Changed return type to Result
        let context = self.context_builder.build_write_context(document_id, cursor_position, 1000).await?;
        
        let prompt = format!(
            "Continue this story naturally. Context: {}\n\nContinue from here:",
            context.preceding_text
        );
        
        let provider = self.ai_provider_manager.get_default_provider()
            .ok_or_else(|| crate::error::StoryWeaverError::ai_provider("default".to_string(), "No default AI provider found".to_string()))?;

        let generated_text = provider.generate_text(&prompt, &context.ai_context).await
            .map_err(|e| crate::error::StoryWeaverError::ai_provider("default".to_string(), e.to_string()))?; // Map error to StoryWeaverError
        
        // Calculate actual credits and word count
        let word_count = generated_text.split_whitespace().count();
        let token_count = generated_text.len() / 4; // Rough estimate: 4 chars per token
        let credits_used = (token_count as f32 * 0.002) as u32; // Rough cost estimate
        
        Ok(WriteResult {
            generated_text,
            credits_used,
            word_count,
        })
    }
    
    pub async fn guided_write(&self, document_id: i32, user_prompt: &str, _settings: WriteSettings) -> crate::error::Result<WriteResult> { // Changed return type to Result
        let context = self.context_builder.build_write_context(document_id, 0, 1000).await?;
        
        let prompt = format!(
            "Write the next part of this story based on this direction: '{}'\n\nStory context: {}",
            user_prompt, context.story_summary
        );
        
        let provider = self.ai_provider_manager.get_default_provider()
            .ok_or_else(|| crate::error::StoryWeaverError::ai_provider("default".to_string(), "No default AI provider found".to_string()))?;

        let generated_text = provider.generate_text(&prompt, &context.ai_context).await
            .map_err(|e| crate::error::StoryWeaverError::ai_provider("default".to_string(), e.to_string()))?; // Map error to StoryWeaverError
        
        // Calculate actual credits and word count
        let word_count = generated_text.split_whitespace().count();
        let token_count = generated_text.len() / 4; // Rough estimate: 4 chars per token
        let credits_used = (token_count as f32 * 0.002) as u32; // Rough cost estimate
        
        Ok(WriteResult {
            generated_text,
            credits_used,
            word_count,
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
) -> Result<WriteResult> {
    rl_create("ai_write", Some(&document_id.to_string()))?;
    
    // Input validation
    if document_id <= 0 {
        return Err(StoryWeaverError::ValidationError {
            message: "Document ID must be a positive integer".to_string(),
        });
    }
    
    // Validate WriteSettings
    validate_write_settings(&settings)?;
    
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    processor.auto_write(document_id, cursor_position, settings).await
}

#[tauri::command]
pub async fn guided_write(
    state: State<'_, Arc<AIProviderManager>>,
    document_id: i32,
    user_prompt: String,
    settings: WriteSettings,
) -> Result<WriteResult> {
    rl_create("ai_write", Some(&document_id.to_string()))?;
    
    // Input validation
    if document_id <= 0 {
        return Err(StoryWeaverError::ValidationError {
            message: "Document ID must be a positive integer".to_string(),
        });
    }
    
    // Validate user prompt
    validate_non_empty_str("user_prompt", &user_prompt, 10_000)?;
    
    // Validate WriteSettings
    validate_write_settings(&settings)?;
    
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    processor.guided_write(document_id, &user_prompt, settings).await
}

// Streaming Tauri Commands
#[tauri::command]
pub async fn auto_write_stream(
    state: State<'_, Arc<AIProviderManager>>,
    window: Window,
    document_id: i32,
    cursor_position: usize,
    settings: WriteSettings,
) -> Result<StreamStartResponse> {
    rl_create("ai_write_stream", Some(&document_id.to_string()))?;
    
    // Input validation
    if document_id <= 0 {
        return Err(StoryWeaverError::ValidationError {
            message: "Document ID must be a positive integer".to_string(),
        });
    }
    
    // Validate WriteSettings
    validate_write_settings(&settings)?;
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    let stream_id = format!("auto_write_{}_{}", document_id, chrono::Utc::now().timestamp_millis());
    let stream_id_clone = stream_id.clone();
    
    // Start streaming in a background task
    tokio::spawn(async move {
        match processor.auto_write_stream(document_id, cursor_position, settings).await {
            Ok(stream) => {
                // Real streaming implementation
                let mut accumulated_content = String::new();
                let mut token_count = 0;
                
                // Check if the provider supports real streaming
                if stream.is_complete {
                    // Provider returned complete text, simulate streaming
                    let words: Vec<&str> = stream.content.split_whitespace().collect();
                    
                    for (i, word) in words.iter().enumerate() {
                        accumulated_content.push_str(word);
                        if i < words.len() - 1 {
                            accumulated_content.push(' ');
                        }
                        
                        token_count = accumulated_content.len() / 4; // Rough estimate
                        
                        let chunk = StreamChunk {
                            content: accumulated_content.clone(),
                            is_complete: i == words.len() - 1,
                            token_count,
                            stream_id: stream_id_clone.clone(),
                        };
                        
                        // Emit the chunk to the frontend
                        if let Err(e) = window.emit("ai_stream_chunk", &chunk) {
                            eprintln!("Failed to emit stream chunk: {}", e);
                            break;
                        }
                        
                        // Add delay to simulate real streaming
                        sleep(Duration::from_millis(100)).await;
                    }
                } else {
                    // Real streaming - emit content as it arrives
                    // This would be implemented when providers support true streaming
                    let chunk = StreamChunk {
                        content: stream.content.clone(),
                        is_complete: true,
                        token_count: stream.token_count,
                        stream_id: stream_id_clone.clone(),
                    };
                    
                    if let Err(e) = window.emit("ai_stream_chunk", &chunk) {
                        eprintln!("Failed to emit stream chunk: {}", e);
                    }
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
    
    Ok(StreamStartResponse {
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
) -> Result<StreamStartResponse> {
    rl_create("ai_write_stream", Some(&document_id.to_string()))?;
    
    // Input validation
    if document_id <= 0 {
        return Err(StoryWeaverError::ValidationError {
            message: "Document ID must be a positive integer".to_string(),
        });
    }
    
    // Validate user prompt
    validate_non_empty_str("user_prompt", &user_prompt, 10_000)?;
    
    // Validate WriteSettings
    validate_write_settings(&settings)?;
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    let stream_id = format!("guided_write_{}_{}", document_id, chrono::Utc::now().timestamp_millis());
    let stream_id_clone = stream_id.clone();
    
    // Start streaming in a background task
    tokio::spawn(async move {
        match processor.guided_write_stream(document_id, &user_prompt, settings).await {
            Ok(stream) => {
                // Real streaming implementation
                let mut accumulated_content = String::new();
                let mut token_count = 0;
                
                // Check if the provider supports real streaming
                if stream.is_complete {
                    // Provider returned complete text, simulate streaming
                    let words: Vec<&str> = stream.content.split_whitespace().collect();
                    
                    for (i, word) in words.iter().enumerate() {
                        accumulated_content.push_str(word);
                        if i < words.len() - 1 {
                            accumulated_content.push(' ');
                        }
                        
                        token_count = accumulated_content.len() / 4; // Rough estimate
                        
                        let chunk = StreamChunk {
                            content: accumulated_content.clone(),
                            is_complete: i == words.len() - 1,
                            token_count,
                            stream_id: stream_id_clone.clone(),
                        };
                        
                        // Emit the chunk to the frontend
                        if let Err(e) = window.emit("ai_stream_chunk", &chunk) {
                            eprintln!("Failed to emit stream chunk: {}", e);
                            break;
                        }
                        
                        // Add delay to simulate real streaming
                        sleep(Duration::from_millis(100)).await;
                    }
                } else {
                    // Real streaming - emit content as it arrives
                    // This would be implemented when providers support true streaming
                    let chunk = StreamChunk {
                        content: stream.content.clone(),
                        is_complete: true,
                        token_count: stream.token_count,
                        stream_id: stream_id_clone.clone(),
                    };
                    
                    if let Err(e) = window.emit("ai_stream_chunk", &chunk) {
                        eprintln!("Failed to emit stream chunk: {}", e);
                    }
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
    
    Ok(StreamStartResponse {
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
) -> Result<String> {
    rl_update("ai_rewrite", None)?;
    
    // Input validation
    validate_non_empty_str("text", &text, 50_000)?;
    
    // Validate RewriteSettings
    validate_rewrite_settings(&settings)?;
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
            
            provider.rewrite_text(&text, &rewrite_style).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn expand_text(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    settings: ExpandSettings,
) -> Result<String> {
    rl_update("ai_expand", None)?;
    
    // Input validation
    validate_non_empty_str("text", &text, 50_000)?;
    
    // Validate ExpandSettings
    validate_expand_settings(&settings)?;
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
            
            provider.expand_text(&text, &context).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn describe_scene(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    focus: Option<String>,
) -> Result<String> {
    rl_create("ai_writing", None)?;
    
    // Input validation
    validate_non_empty_str("text", &text, 10_000)?;
    
    if let Some(ref _focus_val) = focus {
        validate_optional_str("focus", &focus, 100, false)?;
    }
    
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
            
            provider.describe_scene(&text, &context).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn brainstorm_ideas(
    state: State<'_, Arc<AIProviderManager>>,
    prompt: String,
    settings: BrainstormSettings,
) -> Result<Vec<String>> {
    rl_list("ai_words", None)?;
    
    // Input validation
    validate_non_empty_str("prompt", &prompt, 5_000)?;
    
    // Validate BrainstormSettings
    validate_brainstorm_settings(&settings)?;
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
            
            provider.brainstorm(&prompt, &context).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn visualize_scene(
    state: State<'_, Arc<AIProviderManager>>,
    description: String,
) -> Result<String> {
    rl_create("ai_visualize", None)?;
    
    // Input validation
    validate_non_empty_str("description", &description, 5_000)?;
    
    match state.get_default_provider() {
        Some(provider) => {
            provider.generate_image(&description).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn quick_edit(
    state: State<'_, Arc<AIProviderManager>>,
    text: String,
    instruction: String,
) -> Result<String> {
    rl_update("ai_edit", None)?;
    
    // Input validation
    validate_non_empty_str("text", &text, 10_000)?;
    validate_non_empty_str("instruction", &instruction, 1_000)?;
    
    match state.get_default_provider() {
        Some(provider) => {
            provider.quick_edit(&text, &instruction).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn quick_chat(
    state: State<'_, Arc<AIProviderManager>>,
    message: String,
    context: Option<String>,
) -> Result<String> {
    rl_create("ai_chat", None)?;
    
    // Input validation
    validate_non_empty_str("message", &message, 5_000)?;
    
    if let Some(ref _ctx) = context {
        validate_optional_str("context", &context, 10_000, true)?;
    }
    
    match state.get_default_provider() {
        Some(provider) => {
            let mut ai_context = crate::ai::AIContext::default();
            if let Some(ctx) = context {
                ai_context.story_context = Some(ctx);
            }
            
            provider.quick_chat(&message, &ai_context).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}

#[tauri::command]
pub async fn tone_shift_write(
    state: State<'_, Arc<AIProviderManager>>,
    document_id: i32,
    cursor_position: usize,
    tone: String,
    settings: WriteSettings,
) -> Result<WriteResult> {
    rl_create("ai_tone_shift", Some(&document_id.to_string()))?;
    
    // Input validation
    if document_id <= 0 {
        return Err(StoryWeaverError::ValidationError {
            message: "Document ID must be a positive integer".to_string(),
        });
    }
    
    validate_non_empty_str("tone", &tone, 100)?;
    
    // Validate WriteSettings
    validate_write_settings(&settings)?;
    
    let processor = WriteProcessor::new(state.inner().clone(), ContextBuilder);
    
    // Create modified settings with the specified tone
    let mut tone_settings = settings;
    tone_settings.tone = tone;
    
    processor.auto_write(document_id, cursor_position, tone_settings).await
}

#[tauri::command]
pub async fn get_related_words(
    state: State<'_, Arc<AIProviderManager>>,
    word: String,
    context: Option<String>,
) -> Result<Vec<String>> {
    rl_list("ai_words", None)?;
    
    // Input validation
    validate_non_empty_str("word", &word, 100)?;
    
    if let Some(ref _ctx) = context {
        validate_optional_str("context", &context, 5000, true)?;
    }
    
    match state.get_default_provider() {
        Some(provider) => {
            let mut ai_context = crate::ai::AIContext::default();
            if let Some(ctx) = context {
                ai_context.preceding_text = Some(ctx);
            }
            
            provider.related_words(&word, &ai_context).await.map_err(|e| StoryWeaverError::ai(e.to_string()))
        }
        None => Err(StoryWeaverError::ai("No AI provider available")),
    }
}
