//! AI Generation Commands for Story Bible Elements

use crate::commands::CommandResponse;
use crate::error::{Result};
use crate::ai::{AIProviderManager, AIContext, WritingFeature, AIProvider};
use crate::database::{get_pool};
use crate::database::operations::{StoryBibleOps, CharacterTraitOps, StyleExampleOps};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::collections::HashMap;
use std::sync::Arc;

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
) -> CommandResponse<AIGenerationResponse> {
    async fn generate(request: GenerateSynopsisRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get existing story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: request.project_id.clone(),
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::SynopsisGeneration,
        };
        
        // Generate synopsis
        let result = ai_manager.generate_text(&context, &request.braindump).await?;
        
        Ok(AIGenerationResponse {
            generated_content: result.content,
            tokens_used: result.tokens_used,
            cost_estimate: result.cost_estimate,
            provider: result.provider,
            model: result.model,
        })
    }
    
    generate(request, ai_manager.inner().clone()).await.into()
}

/// Generate character traits
#[tauri::command]
pub async fn generate_character_traits(
    request: GenerateCharacterTraitsRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> CommandResponse<Vec<String>> {
    async fn generate(request: GenerateCharacterTraitsRequest, ai_manager: Arc<AIProviderManager>) -> Result<Vec<String>> {
        let pool = get_pool()?;
        
        // Get character and story bible for context
        let character = CharacterTraitOps::get_character(&pool, &request.character_id).await?;
        let story_bible = StoryBibleOps::get_by_project(&pool, &character.project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: character.project_id,
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::CharacterDevelopment,
        };
        
        // Generate traits
        let result = ai_manager.generate_text(&context, &format!("Generate character traits for {}", request.character_name)).await?;
        
        // Parse traits from response
        let traits = result.content
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
    
    generate(request, ai_manager.inner().clone()).await.into()
}

/// Generate world element
#[tauri::command]
pub async fn generate_world_element(
    request: GenerateWorldElementRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> CommandResponse<AIGenerationResponse> {
    async fn generate(request: GenerateWorldElementRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: request.project_id.clone(),
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::WorldBuilding,
        };
        
        // Generate world element
        let prompt = format!("Generate {}: {}", request.element_type, request.name);
        let result = ai_manager.generate_text(&context, &prompt).await?;
        
        Ok(AIGenerationResponse {
            generated_content: result.content,
            tokens_used: result.tokens_used,
            cost_estimate: result.cost_estimate,
            provider: result.provider,
            model: result.model,
        })
    }
    
    generate(request, ai_manager.inner().clone()).await.into()
}

/// Generate outline from story bible
#[tauri::command]
pub async fn generate_outline_from_story_bible(
    project_id: String,
    custom_prompt: Option<String>,
    creativity: Option<f32>,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> CommandResponse<AIGenerationResponse> {
    async fn generate(project_id: String, custom_prompt: Option<String>, creativity: Option<f32>, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: project_id.clone(),
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::OutlineGeneration,
        };
        
        // Generate outline
        let prompt = custom_prompt.unwrap_or_else(|| "Generate a detailed story outline".to_string());
        let result = ai_manager.generate_text(&context, &prompt).await?;
        
        Ok(AIGenerationResponse {
            generated_content: result.content,
            tokens_used: result.tokens_used,
            cost_estimate: result.cost_estimate,
            provider: result.provider,
            model: result.model,
        })
    }
    
    generate(project_id, custom_prompt, creativity, ai_manager.inner().clone()).await.into()
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
) -> CommandResponse<AIGenerationResponse> {
    async fn generate(
        outline_id: String,
        scene_title: String,
        scene_summary: String,
        custom_prompt: Option<String>,
        creativity: Option<f32>,
        ai_manager: Arc<AIProviderManager>
    ) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get outline and story bible for context
        let outline = StyleExampleOps::get_outline(&pool, &outline_id).await?;
        let story_bible = StoryBibleOps::get_by_project(&pool, &outline.project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: outline.project_id,
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::SceneGeneration,
        };
        
        // Generate scene content
        let prompt = custom_prompt.unwrap_or_else(|| format!("Generate scene content for: {}", scene_title));
        let result = ai_manager.generate_text(&context, &prompt).await?;
        
        Ok(AIGenerationResponse {
            generated_content: result.content,
            tokens_used: result.tokens_used,
            cost_estimate: result.cost_estimate,
            provider: result.provider,
            model: result.model,
        })
    }
    
    generate(outline_id, scene_title, scene_summary, custom_prompt, creativity, ai_manager.inner().clone()).await.into()
}

/// Analyze style example
#[tauri::command]
pub async fn analyze_style_example(
    request: GenerateStyleAnalysisRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> CommandResponse<StyleAnalysisResponse> {
    async fn analyze(request: GenerateStyleAnalysisRequest, ai_manager: Arc<AIProviderManager>) -> Result<StyleAnalysisResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: request.project_id.clone(),
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::StyleAnalysis,
        };
        
        // Analyze style
        let prompt = request.custom_prompt.unwrap_or_else(|| "Analyze the writing style and generate a style prompt".to_string());
        let result = ai_manager.generate_text(&context, &prompt).await?;
        
        Ok(StyleAnalysisResponse {
            analysis_result: result.content.clone(),
            generated_style_prompt: result.content,
            tokens_used: result.tokens_used,
            cost_estimate: result.cost_estimate,
            provider: result.provider,
            model: result.model,
        })
    }
    
    analyze(request, ai_manager.inner().clone()).await.into()
}

/// Generate outline from text
#[tauri::command]
pub async fn generate_outline_from_text(
    request: GenerateOutlineFromTextRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> CommandResponse<AIGenerationResponse> {
    async fn generate(request: GenerateOutlineFromTextRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible for context
        let story_bible = StoryBibleOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let mut context = AIContext {
            project_id: request.project_id.clone(),
            story_bible: Some(story_bible),
            current_document: None,
            user_preferences: HashMap::new(),
            writing_feature: WritingFeature::OutlineGeneration,
        };
        
        // Generate outline
        let prompt = request.custom_prompt.unwrap_or_else(|| "Generate an outline from the provided text".to_string());
        let result = ai_manager.generate_text(&context, &prompt).await?;
        
        Ok(AIGenerationResponse {
            generated_content: result.content,
            tokens_used: result.tokens_used,
            cost_estimate: result.cost_estimate,
            provider: result.provider,
            model: result.model,
        })
    }
    
    generate(request, ai_manager.inner().clone()).await.into()
}
