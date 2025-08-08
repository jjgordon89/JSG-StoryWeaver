//! AI Generation Commands for Story Bible Elements

use crate::commands::CommandResponse;
use crate::error::{StoryWeaverError, Result};
use crate::ai::{AIProviderManager, AIContext};
use crate::database::{get_pool};
use crate::database::operations::{StoryBibleOps, CharacterTraitOps, WorldElementOps, StyleExampleOps};
use crate::database::models::{StoryBible, CharacterTrait, WorldElement, StyleExample};
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
            project_id: Some(request.project_id.clone()),
            story_context: Some(request.braindump.clone()),
            genre: request.genre.clone(),
            writing_style: request.style.clone(),
            creativity_level: request.creativity,
            feature_type: Some("synopsis_generation".to_string()),
            ..Default::default()
        };
        
        // Build prompt based on length preference
        let length_instruction = match request.length.as_deref() {
            Some("short") => "Write a concise synopsis in 1-2 paragraphs.",
            Some("long") => "Write a detailed synopsis in 4-6 paragraphs with rich detail.",
            _ => "Write a comprehensive synopsis in 2-4 paragraphs.", // medium/default
        };
        
        let genre_context = request.genre
            .map(|g| format!(" This is a {} story.", g))
            .unwrap_or_default();
        
        let style_context = request.style
            .map(|s| format!(" Write in a {} style.", s))
            .unwrap_or_default();
        
        let base_prompt = format!(
            "Based on the following braindump, create a compelling synopsis that establishes the characters, their goals, the central conflict, how the story begins and how it ends. Also convey the story's tone, themes, and unique elements.{}{}

{}

Braindump:
{}

Synopsis:",
            genre_context,
            style_context,
            length_instruction,
            request.braindump
        );
        
        let prompt = request.custom_prompt.unwrap_or(base_prompt);
        
        // Generate synopsis
        let response = ai_manager.get_default_provider()?.generate_text(&prompt, &context).await
            .map_err(|e| StoryWeaverError::AIProvider { provider: "AI".to_string(), message: e.to_string() })?;
        
        Ok(AIGenerationResponse {
            generated_content: response.text,
            tokens_used: response.tokens_used,
            cost_estimate: response.cost_estimate,
            provider: response.provider,
            model: response.model,
        })
    }
    
    generate(request, ai_manager.inner().clone()).await
}

/// Generate character traits for a character
#[tauri::command]
pub async fn generate_character_traits(
    request: GenerateCharacterTraitsRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> CommandResponse<Vec<String>> {
    async fn generate(request: GenerateCharacterTraitsRequest, ai_manager: Arc<AIProviderManager>) -> Result<Vec<String>> {
        // Build AI context
        let context = AIContext {
            story_context: Some(request.story_context.clone()),
            creativity_level: request.creativity,
            feature_type: Some("character_trait_generation".to_string()),
            ..Default::default()
        };
        
        let trait_count = request.trait_count.unwrap_or(5);
        let existing_traits_text = if request.existing_traits.is_empty() {
            "No existing traits.".to_string()
        } else {
            format!("Existing traits: {}", request.existing_traits.join(", "))
        };
        
        let base_prompt = format!(
            "Generate {} unique and compelling character traits for '{}' based on the story context. Make each trait specific, memorable, and relevant to the character's role in the story. Avoid duplicating existing traits.

Story Context:
{}

{}

Generate {} new traits (one per line):",
            trait_count,
            request.character_name,
            request.story_context,
            existing_traits_text,
            trait_count
        );
        
        let prompt = request.custom_prompt.unwrap_or(base_prompt);
        
        // Generate traits
        let response = ai_manager.get_default_provider()?.generate_text(&prompt, &context).await
            .map_err(|e| StoryWeaverError::AIProvider { provider: "AI".to_string(), message: e.to_string() })?;
        
        // Parse response into individual traits
        let traits: Vec<String> = response.text
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                // Remove numbering or bullet points
                line.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c == '-' || c == '*')
                    .trim()
                    .to_string()
            })
            .take(trait_count as usize)
            .collect();
        
        Ok(traits)
    }
    
    generate(request, ai_manager.inner().clone()).await.into()
}

/// Generate world element description
#[tauri::command]
pub async fn generate_world_element(
    request: GenerateWorldElementRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    async fn generate(request: GenerateWorldElementRequest, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get existing world elements for context
        let existing_elements = WorldElementOps::get_by_project(&pool, &request.project_id).await?;
        
        // Build AI context
        let context = AIContext {
            project_id: Some(request.project_id.clone()),
            story_context: Some(request.story_context.clone()),
            creativity_level: request.creativity,
            feature_type: Some("world_element_generation".to_string()),
            ..Default::default()
        };
        
        let element_type_instruction = match request.element_type.as_str() {
            "location" => "Describe this location with vivid sensory details, its significance to the story, and how characters interact with it.",
            "culture" => "Detail this culture's customs, beliefs, social structure, and how it influences the story's characters and conflicts.",
            "magic_system" => "Explain how this magic system works, its rules and limitations, and its impact on the world and story.",
            "technology" => "Describe this technology, how it functions, its societal impact, and role in the story.",
            "organization" => "Detail this organization's structure, goals, methods, and influence on the story world.",
            "religion" => "Explain this religion's beliefs, practices, hierarchy, and significance to characters and plot.",
            _ => "Provide a detailed description of this world element and its significance to the story.",
        };
        
        let existing_context = if existing_elements.is_empty() {
            "No existing world elements.".to_string()
        } else {
            let element_names: Vec<String> = existing_elements.iter()
                .map(|e| format!("{} ({})", e.name, e.element_type))
                .collect();
            format!("Existing world elements: {}", element_names.join(", "))
        };
        
        let base_prompt = format!(
            "Create a detailed description for the {} named '{}' in this story world. {}

Story Context:
{}

{}

Ensure this element fits naturally into the established world and enhances the story. Be creative but consistent.

Description:",
            request.element_type,
            request.name,
            element_type_instruction,
            request.story_context,
            existing_context
        );
        
        let prompt = request.custom_prompt.unwrap_or(base_prompt);
        
        // Generate world element description
        let response = ai_manager.get_default_provider()?.generate_text(&prompt, &context).await
            .map_err(|e| StoryWeaverError::AIProvider { provider: "AI".to_string(), message: e.to_string() })?;
        
        Ok(AIGenerationResponse {
            generated_content: response.text,
            tokens_used: response.tokens_used,
            cost_estimate: response.cost_estimate,
            provider: response.provider,
            model: response.model,
        })
    }
    
    generate(request, ai_manager.inner().clone()).await.into()
}

/// Generate outline from story bible context
#[tauri::command]
pub async fn generate_outline_from_story_bible(
    project_id: String,
    custom_prompt: Option<String>,
    creativity: Option<f32>,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<AIGenerationResponse> {
    async fn generate(project_id: String, custom_prompt: Option<String>, creativity: Option<f32>, ai_manager: Arc<AIProviderManager>) -> Result<AIGenerationResponse> {
        let pool = get_pool()?;
        
        // Get story bible context
        let story_bible = StoryBibleOps::get_by_project(&pool, &project_id).await?;
        let characters = CharacterTraitOps::get_by_project(&pool, &project_id).await?;
        let world_elements = WorldElementOps::get_by_project(&pool, &project_id).await?;
        
        // Build comprehensive context
        let synopsis = story_bible.synopsis.as_ref()
            .or_else(|| story_bible.braindump.as_ref())
            .cloned()
            .unwrap_or_else(|| "No synopsis or braindump available.".to_string());
        
        let genre = story_bible.genre.as_ref()
            .cloned();
        
        let character_context = if characters.is_empty() {
            "No characters defined.".to_string()
        } else {
            let char_list: Vec<String> = characters.iter()
                .map(|c| format!("{}: {}", c.trait_name, c.trait_value))
                .collect();
            format!("Characters: {}", char_list.join("; "))
        };
        
        let world_context = if world_elements.is_empty() {
            "No world elements defined.".to_string()
        } else {
            let world_list: Vec<String> = world_elements.iter()
                .map(|w| format!("{} ({})", w.name, w.element_type))
                .collect();
            format!("World Elements: {}", world_list.join(", "))
        };
        
        // Build AI context
        let context = AIContext {
            project_id: Some(project_id),
            story_context: Some(synopsis.clone()),
            genre,
            creativity_level: creativity,
            feature_type: Some("outline_generation".to_string()),
            ..Default::default()
        };
        
        let base_prompt = format!(
            "Create a detailed story outline based on the following story bible information. Structure it with clear chapters or sections, each with a brief summary of key events, character development, and plot progression.

Synopsis:
{}

{}

{}

Generate a comprehensive outline with 10-20 chapters/sections. For each chapter, provide:
1. Chapter title
2. Key events
3. Character development
4. Plot progression
5. Conflicts/tensions

Outline:",
            synopsis,
            character_context,
            world_context
        );
        
        let prompt = custom_prompt.unwrap_or(base_prompt);
        
        // Generate outline
        let response = ai_manager.get_default_provider()?.generate_text(&prompt, &context).await
            .map_err(|e| StoryWeaverError::AIProvider { provider: "AI".to_string(), message: e.to_string() })?;
        
        Ok(AIGenerationResponse {
            generated_content: response.text,
            tokens_used: response.tokens_used,
            cost_estimate: response.cost_estimate,
            provider: response.provider,
            model: response.model,
        })
    }
    
    generate(project_id, custom_prompt, creativity, ai_manager.inner().clone()).await
}

/// Generate scene content from outline context
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
        // Build AI context
        let context = AIContext {
            story_context: Some(scene_summary.clone()),
            creativity_level: creativity,
            feature_type: Some("scene_generation".to_string()),
            ..Default::default()
        };
        
        let base_prompt = format!(
            "Generate detailed scene content for the following scene. Include dialogue, action, setting details, and character emotions. Make it engaging and vivid.

Scene Title: {}

Scene Summary: {}

Write the full scene content:",
            scene_title,
            scene_summary
        );
        
        let prompt = custom_prompt.unwrap_or(base_prompt);
        
        // Generate scene content
        let response = ai_manager.get_default_provider()?.generate_text(&prompt, &context).await
            .map_err(|e| StoryWeaverError::AIProvider { provider: "AI".to_string(), message: e.to_string() })?;
        
        Ok(AIGenerationResponse {
            generated_content: response.text,
            tokens_used: response.tokens_used,
            cost_estimate: response.cost_estimate,
            provider: response.provider,
            model: response.model,
        })
    }
    
    generate(outline_id, scene_title, scene_summary, custom_prompt, creativity, ai_manager.inner().clone()).await
}

/// Analyze writing style from example text and generate style prompt
#[tauri::command]
pub async fn analyze_style_example(
    request: GenerateStyleAnalysisRequest,
    ai_manager: State<'_, Arc<AIProviderManager>>,
) -> Result<StyleAnalysisResponse> {
    async fn analyze(request: GenerateStyleAnalysisRequest, ai_manager: Arc<AIProviderManager>) -> Result<StyleAnalysisResponse> {
        let pool = get_pool()?;
        
        // Build AI context for style analysis
        let context = AIContext {
            story_context: Some(format!("Analyzing writing style for project: {}", request.project_id)),
            creativity_level: request.creativity,
            feature_type: Some("style_analysis".to_string()),
            ..Default::default()
        };
        
        let base_prompt = format!(
            "Analyze the following writing sample and provide a detailed style analysis. Then generate a concise style prompt that could be used to replicate this writing style.

Writing Sample:
{}

Please provide:
1. ANALYSIS: A detailed analysis of the writing style including:
   - Sentence structure and length patterns
   - Vocabulary choices and tone
   - Narrative voice and perspective
   - Pacing and rhythm
   - Literary devices used
   - Overall mood and atmosphere

2. STYLE_PROMPT: A concise, actionable prompt (2-3 sentences) that captures the essence of this style for AI writing generation.

Format your response as:
ANALYSIS:
[Your detailed analysis here]

STYLE_PROMPT:
[Your concise style prompt here]",
            request.example_text
        );
        
        let prompt = request.custom_prompt.unwrap_or(base_prompt);
        
        // Generate style analysis
        let response = ai_manager.get_default_provider()?.generate_text(&prompt, &context).await
            .map_err(|e| StoryWeaverError::AIProvider { provider: "AI".to_string(), message: e.to_string() })?;
        
        // Parse the response to extract analysis and style prompt
        let content = response.text;
        let (analysis_result, generated_style_prompt) = if let Some(analysis_start) = content.find("ANALYSIS:") {
            if let Some(prompt_start) = content.find("STYLE_PROMPT:") {
                let analysis = content[analysis_start + 9..prompt_start].trim().to_string();
                let style_prompt = content[prompt_start + 13..].trim().to_string();
                (analysis, style_prompt)
            } else {
                // Fallback if format is not followed
                let parts: Vec<&str> = content.splitn(2, "\n\n").collect();
                if parts.len() >= 2 {
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (content.clone(), "Write in a similar style to the analyzed example.".to_string())
                }
            }
        } else {
            // Fallback if format is not followed
            let parts: Vec<&str> = content.splitn(2, "\n\n").collect();
            if parts.len() >= 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                (content.clone(), "Write in a similar style to the analyzed example.".to_string())
            }
        };
        
        // Update the style example in the database with the analysis results
        StyleExampleOps::update_analysis(
            &pool,
            &request.style_example_id,
            Some(analysis_result.clone()),
            Some(generated_style_prompt.clone())
        ).await?;
        
        Ok(StyleAnalysisResponse {
            analysis_result,
            generated_style_prompt,
            tokens_used: response.tokens_used,
            cost_estimate: response.cost_estimate,
            provider: response.provider,
            model: response.model,
        })
    }
    
    analyze(request, ai_manager.inner().clone()).await
}
