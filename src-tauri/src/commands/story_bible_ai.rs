//! AI Generation Commands for Story Bible Elements

use crate::error::{Result};
use crate::database::get_pool;

use crate::database::operations::StoryBibleOps;
use crate::database::operations::OutlineOps;
use crate::ai::{AIProviderManager, AIProvider, AIContext, WritingFeature, TokenCounter};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;

// Global token counter instance
static TOKEN_COUNTER: Lazy<TokenCounter> = Lazy::new(|| TokenCounter::new());

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSynopsisRequest {
    pub project_id: String,
    pub braindump: String,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub custom_prompt: Option<String>,
    pub creativity: Option<f32>,
    pub length: Option<String>, // "short", "medium", "long"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCharacterTraitsRequest {
    pub character_id: String,
    pub character_name: String,
    pub story_context: String,
    pub existing_traits: Vec<String>,
    pub trait_count: Option<u32>,
    pub custom_prompt: Option<String>,
    pub creativity: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateWorldElementRequest {
    pub project_id: String,
    pub element_type: String, // "location", "culture", "magic_system", "technology", etc.
    pub name: String,
    pub story_context: String,
    pub existing_elements: Vec<String>,
    pub custom_prompt: Option<String>,
    pub creativity: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateStyleAnalysisRequest {
    pub project_id: String,
    pub style_example_id: String,
    pub example_text: String,
    pub custom_prompt: Option<String>,
    pub creativity: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateOutlineFromTextRequest {
    pub project_id: String,
    pub input_text: String,
    pub outline_type: Option<String>, // "chapter", "scene", "full_story", etc.
    pub target_length: Option<String>, // "short", "medium", "detailed"
    pub include_character_arcs: Option<bool>,
    pub include_subplots: Option<bool>,
    pub custom_prompt: Option<String>,
    pub creativity: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleAnalysisResponse {
    pub analysis_result: String,
    pub generated_style_prompt: String,
    pub tokens_used: u32,
    pub cost_estimate: f64,
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIGenerationResponse {
    pub generated_content: String,
    pub tokens_used: u32,
    pub cost_estimate: f64,
    pub provider: String,
    pub model: String,
}

/// Generate synopsis from braindump
#[tauri::command]
pub async fn generate_synopsis(
    request: GenerateSynopsisRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    async fn generate(request: GenerateSynopsisRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        // Input validation
        crate::security::validation::validate_security_input(&request.project_id)?;
        
        if request.braindump.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::validation("Braindump cannot be empty"));
        }
        crate::security::validation::validate_content_length(&request.braindump, 50000)?;
        crate::security::validation::validate_security_input(&request.braindump)?;
        
        if let Some(ref genre) = request.genre {
            crate::security::validation::validate_content_length(genre, 100)?;
            crate::security::validation::validate_security_input(genre)?;
        }
        
        if let Some(ref style) = request.style {
            crate::security::validation::validate_content_length(style, 500)?;
            crate::security::validation::validate_security_input(style)?;
        }
        
        if let Some(ref prompt) = request.custom_prompt {
            crate::security::validation::validate_content_length(prompt, 2000)?;
            crate::security::validation::validate_security_input(prompt)?;
        }
        
        if let Some(creativity_val) = request.creativity {
            if creativity_val < 0.0 || creativity_val > 1.0 {
                return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0"));
            }
        }
        
        if let Some(ref length) = request.length {
            let valid_lengths = ["short", "medium", "long"];
            if !valid_lengths.contains(&length.as_str()) {
                return Err(crate::error::StoryWeaverError::validation("Length must be 'short', 'medium', or 'long'"));
            }
        }
        
        let pool = get_pool()?;
        
        // Get existing story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(request.project_id.clone()),
            document_id: None,
            preceding_text: Some(request.braindump.clone()),
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: request.style.clone(),
            tone: None,
            creativity_level: request.creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: request.genre.clone(),
            key_details: None,
        };
        
        // Generate synopsis
        let result = ai_manager.generate_text(&request.braindump, &context).await?;
        
        // Count tokens and estimate cost
        let input_tokens = TOKEN_COUNTER.count_tokens(&request.braindump);
        let output_tokens = TOKEN_COUNTER.count_tokens(&result);
        let total_tokens = input_tokens + output_tokens;
        
        let cost_estimate = TOKEN_COUNTER.estimate_cost(
            ai_manager.get_provider_name(),
            ai_manager.get_model_name(),
            input_tokens,
            output_tokens,
        );
        
        Ok(AIGenerationResponse {
            generated_content: result,
            tokens_used: total_tokens,
            cost_estimate,
            provider: ai_manager.get_provider_name().to_string(),
            model: ai_manager.get_model_name().to_string(),
        })
    }
    
    generate(request, ai_manager.inner().clone()).await
}

/// Generate character traits
#[tauri::command]
pub async fn generate_character_traits(
    request: GenerateCharacterTraitsRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<Vec<String>> {
    // Input validation
    if !crate::security::is_safe_input(&request.character_id) {
        return Err(crate::error::StoryWeaverError::validation("Invalid character_id".to_string()));
    }
    if request.character_name.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Character name cannot be empty".to_string()));
    }
    if request.character_name.len() > 200 {
        return Err(crate::error::StoryWeaverError::validation("Character name too long (max 200 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.character_name) {
        return Err(crate::error::StoryWeaverError::validation("Invalid character_name".to_string()));
    }
    if request.story_context.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Story context cannot be empty".to_string()));
    }
    if request.story_context.len() > 10000 {
        return Err(crate::error::StoryWeaverError::validation("Story context too long (max 10000 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.story_context) {
        return Err(crate::error::StoryWeaverError::validation("Invalid story_context".to_string()));
    }
    if let Some(ref custom_prompt) = request.custom_prompt {
        if custom_prompt.len() > 2000 {
            return Err(crate::error::StoryWeaverError::validation("Custom prompt too long (max 2000 characters)".to_string()));
        }
        if !crate::security::is_safe_input(custom_prompt) {
            return Err(crate::error::StoryWeaverError::validation("Invalid custom_prompt".to_string()));
        }
    }
    if let Some(creativity) = request.creativity {
        if creativity < 0.0 || creativity > 1.0 {
            return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0".to_string()));
        }
    }
    if let Some(trait_count) = request.trait_count {
        if trait_count == 0 || trait_count > 20 {
            return Err(crate::error::StoryWeaverError::validation("Trait count must be between 1 and 20".to_string()));
        }
    }

    async fn generate(request: GenerateCharacterTraitsRequest, ai_manager: Arc<AIProviderManager>) -> Result<Vec<String>> {
        let pool = get_pool()?;
        
        // Get character and story bible for context
        let character = sqlx::query_as::<_, crate::database::models::Character>("SELECT * FROM characters WHERE id = ?")
            .bind(&request.character_id)
            .fetch_one(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get character: {}", e)))?;
        let story_bible = StoryBibleOps::get_by_project(&pool, &character.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(character.project_id),
            document_id: None,
            preceding_text: Some(request.story_context.clone()),
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: request.creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Generate traits
        let result = ai_manager.generate_text(&format!("Generate character traits for {}", request.character_name), &context).await?;
        
        // Parse traits from response
        let traits = result
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() || line.starts_with('-') || line.starts_with('*') {
                    None
                } else {
                    Some(line.to_string())
                }
            })
            .take(request.trait_count.unwrap_or(5) as usize)
            .collect();
        
        Ok(traits)
    }
    
    generate(request, ai_manager.inner().clone()).await
}

/// Generate world element
#[tauri::command]
pub async fn generate_world_element(
    request: GenerateWorldElementRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    // Input validation
    if !crate::security::is_safe_input(&request.project_id) {
        return Err(crate::error::StoryWeaverError::validation("Invalid project_id".to_string()));
    }
    if request.element_type.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Element type cannot be empty".to_string()));
    }
    if request.element_type.len() > 100 {
        return Err(crate::error::StoryWeaverError::validation("Element type too long (max 100 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.element_type) {
        return Err(crate::error::StoryWeaverError::validation("Invalid element_type".to_string()));
    }
    if request.name.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Name cannot be empty".to_string()));
    }
    if request.name.len() > 200 {
        return Err(crate::error::StoryWeaverError::validation("Name too long (max 200 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.name) {
        return Err(crate::error::StoryWeaverError::validation("Invalid name".to_string()));
    }
    if request.story_context.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Story context cannot be empty".to_string()));
    }
    if request.story_context.len() > 10000 {
        return Err(crate::error::StoryWeaverError::validation("Story context too long (max 10000 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.story_context) {
        return Err(crate::error::StoryWeaverError::validation("Invalid story_context".to_string()));
    }
    if let Some(ref custom_prompt) = request.custom_prompt {
        if custom_prompt.len() > 2000 {
            return Err(crate::error::StoryWeaverError::validation("Custom prompt too long (max 2000 characters)".to_string()));
        }
        if !crate::security::is_safe_input(custom_prompt) {
            return Err(crate::error::StoryWeaverError::validation("Invalid custom_prompt".to_string()));
        }
    }
    if let Some(creativity) = request.creativity {
        if creativity < 0.0 || creativity > 1.0 {
            return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0".to_string()));
        }
    }

    async fn generate(request: GenerateWorldElementRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(request.project_id.clone()),
            document_id: None,
            preceding_text: Some(request.story_context.clone()),
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: request.creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Generate world element
        let prompt = format!("Generate {}: {}", request.element_type, request.name);
        let result = ai_manager.generate_text(&prompt, &context).await?;
        
        // Count tokens and estimate cost
        let input_tokens = TOKEN_COUNTER.count_tokens(&prompt);
        let output_tokens = TOKEN_COUNTER.count_tokens(&result);
        let total_tokens = input_tokens + output_tokens;
        
        let cost_estimate = TOKEN_COUNTER.estimate_cost(
            ai_manager.get_provider_name(),
            ai_manager.get_model_name(),
            input_tokens,
            output_tokens,
        );
        
        Ok(AIGenerationResponse {
            generated_content: result,
            tokens_used: total_tokens,
            cost_estimate,
            provider: ai_manager.get_provider_name().to_string(),
            model: ai_manager.get_model_name().to_string(),
        })
    }
    
    generate(request, ai_manager.inner().clone()).await
}

/// Generate outline from story bible
#[tauri::command]
pub async fn generate_outline_from_story_bible(
    project_id: String,
    custom_prompt: Option<String>,
    creativity: Option<f32>,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    // Input validation
    if !crate::security::is_safe_input(&project_id) {
        return Err(crate::error::StoryWeaverError::validation("Invalid project_id".to_string()));
    }
    if let Some(ref custom_prompt) = custom_prompt {
        if custom_prompt.len() > 2000 {
            return Err(crate::error::StoryWeaverError::validation("Custom prompt too long (max 2000 characters)".to_string()));
        }
        if !crate::security::is_safe_input(custom_prompt) {
            return Err(crate::error::StoryWeaverError::validation("Invalid custom_prompt".to_string()));
        }
    }
    if let Some(creativity) = creativity {
        if creativity < 0.0 || creativity > 1.0 {
            return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0".to_string()));
        }
    }

    async fn generate(project_id: String, custom_prompt: Option<String>, creativity: Option<f32>, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(project_id.clone()),
            document_id: None,
            preceding_text: None,
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Generate outline
        let prompt = custom_prompt.unwrap_or_else(|| "Generate a detailed story outline".to_string());
        let result = ai_manager.generate_text(&prompt, &context).await?;
        
        // Count tokens and estimate cost
        let input_tokens = TOKEN_COUNTER.count_tokens(&prompt);
        let output_tokens = TOKEN_COUNTER.count_tokens(&result);
        let total_tokens = input_tokens + output_tokens;
        
        let cost_estimate = TOKEN_COUNTER.estimate_cost(
            ai_manager.get_provider_name(),
            ai_manager.get_model_name(),
            input_tokens,
            output_tokens,
        );
        
        Ok(AIGenerationResponse {
            generated_content: result,
            tokens_used: total_tokens,
            cost_estimate,
            provider: ai_manager.get_provider_name().to_string(),
            model: ai_manager.get_model_name().to_string(),
        })
    }
    
    generate(project_id, custom_prompt, creativity, ai_manager.inner().clone()).await
}

/// Generate scene content
#[tauri::command]
pub async fn generate_scene_content(
    outline_id: String,
    scene_title: String,
    scene_summary: String,
    custom_prompt: Option<String>,
    creativity: Option<f32>,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    async fn generate(
        outline_id: String,
        scene_title: String,
        scene_summary: String,
        custom_prompt: Option<String>,
        creativity: Option<f32>,
        ai_manager: Arc<AIProviderManager>
    ) -> Result<AIGenerationResponse> {
        // Input validation
        crate::security::validation::validate_security_input(&outline_id)?;
        
        if scene_title.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::validation("Scene title cannot be empty"));
        }
        crate::security::validation::validate_content_length(&scene_title, 500)?;
        crate::security::validation::validate_security_input(&scene_title)?;
        
        if scene_summary.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::validation("Scene summary cannot be empty"));
        }
        crate::security::validation::validate_content_length(&scene_summary, 5000)?;
        crate::security::validation::validate_security_input(&scene_summary)?;
        
        if let Some(ref prompt) = custom_prompt {
            crate::security::validation::validate_content_length(prompt, 2000)?;
            crate::security::validation::validate_security_input(prompt)?;
        }
        
        if let Some(creativity_val) = creativity {
            if creativity_val < 0.0 || creativity_val > 1.0 {
                return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0"));
            }
        }
        
        let pool = get_pool()?;
        
        // Get outline and story bible for context
        let outline = OutlineOps::get_by_id(&pool, &outline_id).await?;
        let story_bible = StoryBibleOps::get_by_project(&pool, &outline.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(outline.project_id),
            document_id: None,
            preceding_text: None,
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Generate scene content
        let prompt = custom_prompt.unwrap_or_else(|| format!("Generate scene content for: {}", scene_title));
        let result = ai_manager.generate_text(&prompt, &context).await?;
        
        // Count tokens and estimate cost
        let input_tokens = TOKEN_COUNTER.count_tokens(&prompt);
        let output_tokens = TOKEN_COUNTER.count_tokens(&result);
        let total_tokens = input_tokens + output_tokens;
        
        let cost_estimate = TOKEN_COUNTER.estimate_cost(
            ai_manager.get_provider_name(),
            ai_manager.get_model_name(),
            input_tokens,
            output_tokens,
        );
        
        Ok(AIGenerationResponse {
            generated_content: result,
            tokens_used: total_tokens,
            cost_estimate,
            provider: ai_manager.get_provider_name().to_string(),
            model: ai_manager.get_model_name().to_string(),
        })
    }
    
    generate(outline_id, scene_title, scene_summary, custom_prompt, creativity, ai_manager.inner().clone()).await
}

/// Analyze style example
#[tauri::command]
pub async fn analyze_style_example(
    request: GenerateStyleAnalysisRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<StyleAnalysisResponse> {
    // Input validation
    if !crate::security::is_safe_input(&request.project_id) {
        return Err(crate::error::StoryWeaverError::validation("Invalid project_id".to_string()));
    }
    if !crate::security::is_safe_input(&request.style_example_id) {
        return Err(crate::error::StoryWeaverError::validation("Invalid style_example_id".to_string()));
    }
    if request.example_text.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Example text cannot be empty".to_string()));
    }
    if request.example_text.len() > 20000 {
        return Err(crate::error::StoryWeaverError::validation("Example text too long (max 20000 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.example_text) {
        return Err(crate::error::StoryWeaverError::validation("Invalid example_text".to_string()));
    }
    if let Some(ref custom_prompt) = request.custom_prompt {
        if custom_prompt.len() > 2000 {
            return Err(crate::error::StoryWeaverError::validation("Custom prompt too long (max 2000 characters)".to_string()));
        }
        if !crate::security::is_safe_input(custom_prompt) {
            return Err(crate::error::StoryWeaverError::validation("Invalid custom_prompt".to_string()));
        }
    }
    if let Some(creativity) = request.creativity {
        if creativity < 0.0 || creativity > 1.0 {
            return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0".to_string()));
        }
    }

    async fn analyze(request: GenerateStyleAnalysisRequest, ai_manager: Arc<AIProviderManager>) -> Result<StyleAnalysisResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(request.project_id.clone()),
            document_id: None,
            preceding_text: Some(request.example_text.clone()),
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: request.creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Analyze style
        let prompt = request.custom_prompt.unwrap_or_else(|| "Analyze the writing style and generate a style prompt".to_string());
        let result = ai_manager.generate_text(&prompt, &context).await?;
        
        // Count tokens and estimate cost
        let input_tokens = TOKEN_COUNTER.count_tokens(&prompt);
        let output_tokens = TOKEN_COUNTER.count_tokens(&result);
        let total_tokens = input_tokens + output_tokens;
        
        let cost_estimate = TOKEN_COUNTER.estimate_cost(
            ai_manager.get_provider_name(),
            ai_manager.get_model_name(),
            input_tokens,
            output_tokens,
        );
        
        Ok(StyleAnalysisResponse {
            analysis_result: result.clone(),
            generated_style_prompt: result,
            tokens_used: total_tokens,
            cost_estimate,
            provider: ai_manager.get_provider_name().to_string(),
            model: ai_manager.get_model_name().to_string(),
        })
    }
    
    analyze(request, ai_manager.inner().clone()).await
}

/// Generate outline from text
#[tauri::command]
pub async fn generate_outline_from_text(
    request: GenerateOutlineFromTextRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    // Input validation
    if !crate::security::is_safe_input(&request.project_id) {
        return Err(crate::error::StoryWeaverError::validation("Invalid project_id".to_string()));
    }
    if request.input_text.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::validation("Input text cannot be empty".to_string()));
    }
    if request.input_text.len() > 50000 {
        return Err(crate::error::StoryWeaverError::validation("Input text too long (max 50000 characters)".to_string()));
    }
    if !crate::security::is_safe_input(&request.input_text) {
        return Err(crate::error::StoryWeaverError::validation("Invalid input_text".to_string()));
    }
    if let Some(ref outline_type) = request.outline_type {
        if outline_type.len() > 50 {
            return Err(crate::error::StoryWeaverError::validation("Outline type too long (max 50 characters)".to_string()));
        }
        if !crate::security::is_safe_input(outline_type) {
            return Err(crate::error::StoryWeaverError::validation("Invalid outline_type".to_string()));
        }
    }
    if let Some(ref target_length) = request.target_length {
        if !matches!(target_length.as_str(), "short" | "medium" | "detailed") {
            return Err(crate::error::StoryWeaverError::validation("Target length must be 'short', 'medium', or 'detailed'".to_string()));
        }
    }
    if let Some(ref custom_prompt) = request.custom_prompt {
        if custom_prompt.len() > 2000 {
            return Err(crate::error::StoryWeaverError::validation("Custom prompt too long (max 2000 characters)".to_string()));
        }
        if !crate::security::is_safe_input(custom_prompt) {
            return Err(crate::error::StoryWeaverError::validation("Invalid custom_prompt".to_string()));
        }
    }
    if let Some(creativity) = request.creativity {
        if creativity < 0.0 || creativity > 1.0 {
            return Err(crate::error::StoryWeaverError::validation("Creativity must be between 0.0 and 1.0".to_string()));
        }
    }

    async fn generate(request: GenerateOutlineFromTextRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(request.project_id.clone()),
            document_id: None,
            preceding_text: Some(request.input_text.clone()),
            following_text: None,
            selected_text: None,
            story_context: story_bible.synopsis.clone(),
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: request.creativity.map(|c| (c * 10.0) as u8),
            feature_type: Some(WritingFeature::Write),
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Generate outline
        let prompt = request.custom_prompt.unwrap_or_else(|| "Generate an outline from the provided text".to_string());
        let result = ai_manager.generate_text(&prompt, &context).await?;
        
        // Count tokens and estimate cost
        let input_tokens = TOKEN_COUNTER.count_tokens(&prompt);
        let output_tokens = TOKEN_COUNTER.count_tokens(&result);
        let total_tokens = input_tokens + output_tokens;
        
        let cost_estimate = TOKEN_COUNTER.estimate_cost(
            ai_manager.get_provider_name(),
            ai_manager.get_model_name(),
            input_tokens,
            output_tokens,
        );
        
        Ok(AIGenerationResponse {
            generated_content: result,
            tokens_used: total_tokens,
            cost_estimate,
            provider: ai_manager.get_provider_name().to_string(),
            model: ai_manager.get_model_name().to_string(),
        })
    }
    
    generate(request, ai_manager.inner().clone()).await
}
