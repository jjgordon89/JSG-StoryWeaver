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
        _scene_summary: String,
        custom_prompt: Option<String>,
        creativity: Option<f32>,
        ai_manager: Arc<AIProviderManager>
    ) -> Result<AIGenerationResponse> {
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
