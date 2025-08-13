use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;
use std::str::FromStr;
use crate::database::{get_pool, models::{Document, DocumentType}, operations::DocumentOps};
use crate::security::rate_limit::{rl_create, rl_update, rl_delete, rl_list, rl_search, validate_request_body_size};

use crate::ai::{
    advanced_ai_manager::StyleAnalysis, AdvancedAIManager, AdvancedGenerationRequest,
    AdvancedGenerationResult, BrainstormRequest, BrainstormSession, GeneratedImage,
    ProseMode, SaliencyContext, StyleExample, VisualizeRequest,
};
use crate::commands::CommandResponse;
use crate::error::{Result, StoryWeaverError};
use crate::ai::saliency_engine::StoryBibleElements as SaliencyStoryBible;
use crate::ai::visualize::ImageResolution;
use crate::ai::brainstorm::BrainstormCategory;

// State wrapper for the Advanced AI Manager
pub type AdvancedAIState = Mutex<AdvancedAIManager>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProseGenerationRequest {
    pub project_id: String,
    pub document_id: Option<String>,
    pub prose_mode: String,
    pub text_context: String,
    pub generation_type: String,
    pub max_words: Option<i32>,
    pub ultra_creative: bool,
    pub use_saliency_engine: bool,
    pub style_examples: Vec<String>,
    pub special_instructions: Option<String>,
    pub story_bible: Option<SaliencyStoryBible>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageGenerationRequest {
    pub project_id: String,
    pub document_id: Option<String>,
    pub text_content: String,
    pub style_preference: String,
    pub resolution: String, // "1024x1024", "1792x1024", etc.
    pub enhance_prompt: bool,
    pub custom_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BrainstormSessionRequest {
    pub project_id: String,
    pub category: String,
    pub focus_area: String,
    pub num_ideas: u32,
    pub creativity_level: u32,
    pub context: String,
    pub constraints: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleExampleRequest {
    pub project_id: String,
    pub name: String,
    pub content: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreditUsageResponse {
    pub project_usage: i32,
    pub daily_usage: i32,
    pub monthly_limit: Option<i32>,
    pub remaining_credits: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportSuggestion {
    pub suggestion_type: String, // "character", "location", "plot_thread", "worldbuilding"
    pub name: String,
    pub description: String,
    pub confidence: f32,
    pub auto_apply: bool,
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedElements {
    pub characters: Vec<ExtractedCharacter>,
    pub locations: Vec<ExtractedLocation>,
    pub plot_points: Vec<String>,
    pub themes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedCharacter {
    pub name: String,
    pub description: String,
    pub traits: Vec<String>,
    pub relationships: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedLocation {
    pub name: String,
    pub description: String,
    pub atmosphere: String,
    pub significance: String,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartImportAnalysisResult {
    pub suggestions: Vec<ImportSuggestion>,
    pub extracted_elements: ExtractedElements,
}

// Advanced Text Generation with Prose Modes
#[tauri::command]
pub async fn generate_with_prose_mode(
    request: ProseGenerationRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<AdvancedGenerationResult> {
    // Input validation
    crate::security::validation::validate_security_input(&request.project_id)?;
    crate::security::validation::validate_security_input(&request.prose_mode)?;
    crate::security::validation::validate_content_length(&request.text_context, 50000)?;
    crate::security::validation::validate_security_input(&request.text_context)?;
    crate::security::validation::validate_security_input(&request.generation_type)?;
    
    if let Some(ref doc_id) = request.document_id {
        crate::security::validation::validate_security_input(doc_id)?;
    }
    
    if let Some(max_words) = request.max_words {
        if max_words < 0 {
            return Err(StoryWeaverError::InvalidInput { message: "max_words cannot be negative".to_string() });
        }
    }
    
    for style_example in &request.style_examples {
        crate::security::validation::validate_content_length(style_example, 10000)?;
        crate::security::validation::validate_security_input(style_example)?;
    }
    
    if let Some(ref instructions) = request.special_instructions {
        crate::security::validation::validate_content_length(instructions, 5000)?;
        crate::security::validation::validate_security_input(instructions)?;
    }
    
    let mut ai_manager = ai_state.lock().await;

    let advanced_request = AdvancedGenerationRequest {
        project_id: request.project_id,
        document_id: request.document_id,
        prose_mode: request.prose_mode,
        text_context: request.text_context,
        generation_type: request.generation_type,
        max_words: request.max_words,
        ultra_creative: request.ultra_creative,
        use_saliency_engine: request.use_saliency_engine,
        style_examples: request.style_examples,
        special_instructions: request.special_instructions,
    };

    let story_bible_saliency = request.story_bible.map(|sb| SaliencyStoryBible::from(sb));

    ai_manager
        .generate_with_advanced_features(advanced_request, story_bible_saliency)
        .await
}

// Image Generation with Visualize Engine
#[tauri::command]
pub async fn generate_image(
    request: ImageGenerationRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<GeneratedImage> {
    // Input validation
    crate::security::validation::validate_security_input(&request.project_id)?;
    crate::security::validation::validate_content_length(&request.text_content, 10000)?;
    crate::security::validation::validate_security_input(&request.text_content)?;
    crate::security::validation::validate_content_length(&request.style_preference, 500)?;
    crate::security::validation::validate_security_input(&request.style_preference)?;
    crate::security::validation::validate_security_input(&request.resolution)?;
    
    if let Some(ref doc_id) = request.document_id {
        crate::security::validation::validate_security_input(doc_id)?;
    }
    
    if let Some(ref custom_prompt) = request.custom_prompt {
        crate::security::validation::validate_content_length(custom_prompt, 5000)?;
        crate::security::validation::validate_security_input(custom_prompt)?;
    }
    
    let mut ai_manager = ai_state.lock().await;

    let visualize_request = VisualizeRequest {
        project_id: request.project_id,
        source_text: request.text_content,
        style_preference: Some(request.style_preference),
        resolution: ImageResolution::from_str(&request.resolution).unwrap_or(ImageResolution::Square1024),
        enhance_prompt: request.enhance_prompt,
    };

    ai_manager.generate_image(visualize_request).await
}

// Advanced Brainstorming
#[tauri::command]
pub async fn create_brainstorm_session(
    request: BrainstormSessionRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<String> {
    // Input validation
    crate::security::validation::validate_security_input(&request.project_id)?;
    crate::security::validation::validate_security_input(&request.category)?;
    crate::security::validation::validate_content_length(&request.focus_area, 500)?;
    crate::security::validation::validate_security_input(&request.focus_area)?;
    crate::security::validation::validate_content_length(&request.context, 10000)?;
    crate::security::validation::validate_security_input(&request.context)?;
    
    if request.num_ideas == 0 || request.num_ideas > 100 {
        return Err(StoryWeaverError::InvalidInput { message: "num_ideas must be between 1 and 100".to_string() });
    }
    
    if request.creativity_level > 10 {
        return Err(StoryWeaverError::InvalidInput { message: "creativity_level must be between 0 and 10".to_string() });
    }
    
    for constraint in &request.constraints {
        crate::security::validation::validate_content_length(constraint, 1000)?;
        crate::security::validation::validate_security_input(constraint)?;
    }
    
    let mut ai_manager = ai_state.lock().await;

    let brainstorm_request = BrainstormRequest {
        project_id: request.project_id,
        category: BrainstormCategory::from_str(&request.category).unwrap_or_default(),
        seed_prompt: Some(request.context),
        context: None,
        num_ideas: request.num_ideas as usize,
        creativity_level: request.creativity_level as i32,
        focus_areas: vec![request.focus_area],
    };

    ai_manager
        .create_brainstorm_session(brainstorm_request)
        .await
}

#[tauri::command]
pub async fn get_brainstorm_session(
    session_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Option<BrainstormSession>> {
    // Input validation
    crate::security::validation::validate_security_input(&session_id)?;
    
    let manager = ai_state.lock().await;
    Ok(manager.get_brainstorm_session(&session_id).cloned())
}

#[tauri::command]
pub async fn rate_brainstorm_idea(
    session_id: String,
    idea_id: String,
    rating: u32,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<()> {
    // Input validation
    crate::security::validation::validate_security_input(&session_id)?;
    crate::security::validation::validate_security_input(&idea_id)?;
    
    if rating > 10 {
        return Err(StoryWeaverError::InvalidInput { message: "rating must be between 0 and 10".to_string() });
    }
    
    let mut manager = ai_state.lock().await;
    manager.rate_brainstorm_idea(&session_id, &idea_id, rating)
}

#[tauri::command]
pub async fn mark_idea_as_keeper(
    session_id: String,
    idea_id: String,
    is_keeper: bool,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<()> {
    // Input validation
    crate::security::validation::validate_security_input(&session_id)?;
    crate::security::validation::validate_security_input(&idea_id)?;
    
    let mut manager = ai_state.lock().await;
    manager.mark_idea_as_keeper(&session_id, &idea_id, is_keeper)
}

// Style Examples Management
#[tauri::command]
pub async fn add_style_example(
    request: StyleExampleRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<StyleExample> {
    // Input validation
    crate::security::validation::validate_security_input(&request.project_id)?;
    crate::security::validation::validate_content_length(&request.name, 200)?;
    crate::security::validation::validate_security_input(&request.name)?;
    crate::security::validation::validate_content_length(&request.content, 10000)?;
    crate::security::validation::validate_security_input(&request.content)?;
    
    if request.name.trim().is_empty() {
        return Err(StoryWeaverError::InvalidInput { message: "Style example name cannot be empty".to_string() });
    }
    
    if request.content.trim().is_empty() {
        return Err(StoryWeaverError::InvalidInput { message: "Style example content cannot be empty".to_string() });
    }
    
    let mut ai_manager = ai_state.lock().await;

    let analysis_result = ai_manager.analyze_style(&request.content).await?;

    let style_example = StyleExample {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: request.project_id,
        name: request.name,
        content: request.content.clone(),
        word_count: request.content.split_whitespace().count() as i32,
        analysis_result: Some(analysis_result),
        is_active: request.is_active,
    };

    ai_manager.add_style_example(style_example.clone()).await;

    Ok(style_example)
}

#[tauri::command]
pub async fn analyze_text_style(
    content: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<StyleAnalysis> {
    // Input validation
    crate::security::validation::validate_content_length(&content, 50000)?;
    crate::security::validation::validate_security_input(&content)?;
    
    if content.trim().is_empty() {
        return Err(StoryWeaverError::InvalidInput { message: "Content cannot be empty".to_string() });
    }
    
    let ai_manager = ai_state.lock().await;
    ai_manager.analyze_style(&content).await
}

// Prose Modes Management
#[tauri::command]
pub async fn get_available_prose_modes(
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Vec<ProseMode>> {
    let ai_manager = ai_state.lock().await;
    Ok(ai_manager.get_prose_modes().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_prose_mode_details(
    mode_name: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Option<ProseMode>> {
    // Input validation
    crate::security::validation::validate_security_input(&mode_name)?;
    
    let ai_manager = ai_state.lock().await;
    Ok(ai_manager
        .get_prose_modes()
        .into_iter()
        .find(|mode| mode.name == mode_name)
        .cloned())
}

// Credit Management
#[tauri::command]
pub async fn get_credit_usage(
    project_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<CreditUsageResponse> {
    // Input validation
    crate::security::validation::validate_security_input(&project_id)?;
    
    let ai_manager = ai_state.lock().await;

    let project_usage = ai_manager.get_credit_usage(&project_id).await?;
    let daily_usage = ai_manager.get_daily_usage().await?;
    let (monthly_limit, remaining_credits) = ai_manager.get_credit_status().await?;

    Ok(CreditUsageResponse {
        project_usage: project_usage,
        daily_usage: daily_usage,
        monthly_limit: monthly_limit,
        remaining_credits: remaining_credits,
    })
}

// Image Management
#[tauri::command]
pub async fn get_project_images(
    project_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Vec<GeneratedImage>> {
    // Input validation
    crate::security::validation::validate_security_input(&project_id)?;
    
    let ai_manager = ai_state.lock().await;
    ai_manager.get_generated_images(&project_id).await
}

#[tauri::command]
pub async fn delete_generated_image(
    image_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<()> {
    // Input validation
    crate::security::validation::validate_security_input(&image_id)?;
    
    let mut ai_manager = ai_state.lock().await;
    ai_manager.delete_generated_image(&image_id).await
}

// Saliency Engine
#[tauri::command]
pub async fn build_saliency_context(
    project_id: String,
    text_context: String,
    story_bible: SaliencyStoryBible,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<SaliencyContext> {
    // Input validation
    crate::security::validation::validate_security_input(&project_id)?;
    crate::security::validation::validate_content_length(&text_context, 50000)?;
    crate::security::validation::validate_security_input(&text_context)?;
    
    let mut ai_manager = ai_state.lock().await;
    let story_bible_saliency = SaliencyStoryBible::from(story_bible);

    ai_manager
        .build_saliency_context(&project_id, &text_context, &story_bible_saliency)
        .await
}

// Smart Import with Novel Analysis
#[tauri::command]
pub async fn smart_import_content(
    project_id: String,
    content: String,
    content_type: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<SmartImportAnalysisResult> {
    // Input validation
    crate::security::validation::validate_security_input(&project_id)?;
    crate::security::validation::validate_content_length(&content, 100000)?;
    crate::security::validation::validate_security_input(&content)?;
    crate::security::validation::validate_security_input(&content_type)?;
    
    let ai_manager = ai_state.lock().await;
    ai_manager
        .analyze_content_for_import(&project_id, &content, &content_type)
        .await
}

// Streaming Generation (placeholder for future implementation)
#[tauri::command]
pub async fn start_streaming_generation(
    request: ProseGenerationRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<String> {
    // Input validation (reuse the same validation as generate_with_prose_mode)
    crate::security::validation::validate_security_input(&request.project_id)?;
    crate::security::validation::validate_security_input(&request.prose_mode)?;
    crate::security::validation::validate_content_length(&request.text_context, 50000)?;
    crate::security::validation::validate_security_input(&request.text_context)?;
    crate::security::validation::validate_security_input(&request.generation_type)?;
    
    if let Some(ref doc_id) = request.document_id {
        crate::security::validation::validate_security_input(doc_id)?;
    }
    
    if let Some(max_words) = request.max_words {
        if max_words < 0 {
            return Err(StoryWeaverError::InvalidInput { message: "max_words cannot be negative".to_string() });
        }
    }
    
    for style_example in &request.style_examples {
        crate::security::validation::validate_content_length(style_example, 10000)?;
        crate::security::validation::validate_security_input(style_example)?;
    }
    
    if let Some(ref instructions) = request.special_instructions {
        crate::security::validation::validate_content_length(instructions, 5000)?;
        crate::security::validation::validate_security_input(instructions)?;
    }
    
    let mut ai_manager = ai_state.lock().await;
    let advanced_request = AdvancedGenerationRequest {
        project_id: request.project_id,
        document_id: request.document_id,
        prose_mode: request.prose_mode,
        text_context: request.text_context,
        generation_type: request.generation_type,
        max_words: request.max_words,
        ultra_creative: request.ultra_creative,
        use_saliency_engine: request.use_saliency_engine,
        style_examples: request.style_examples,
        special_instructions: request.special_instructions,
    };
    let story_bible_saliency = request.story_bible.map(SaliencyStoryBible::from);
    ai_manager
        .start_streaming_generation(advanced_request, story_bible_saliency)
        .await
}

#[tauri::command]
pub async fn get_stream_status(
    streamId: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<HashMap<String, serde_json::Value>> {
    // Input validation
    crate::security::validation::validate_security_input(&streamId)?;
    
    let ai_manager = ai_state.lock().await;
    ai_manager.get_stream_status(&streamId).await
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveGeneratedContentRequest {
    pub content: String,
    pub location: String, // "document" | "snippet" | "note"
    pub title: Option<String>,
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "projectId")]
    pub project_id: Option<String>,
    #[serde(rename = "documentId")]
    pub document_id: Option<String>,
}

#[tauri::command]
pub async fn save_generated_content(data: SaveGeneratedContentRequest) -> Result<()> {
    // Basic validation
    crate::security::validation::validate_content_length(&data.content, 1_000_000)?;
    crate::security::validation::validate_security_input(&data.location)?;
    if let Some(ref t) = data.title {
        crate::security::validation::validate_content_length(t, 500)?;
        crate::security::validation::validate_security_input(t)?;
    }
    if let Some(ref pid) = data.project_id {
        crate::security::validation::validate_security_input(pid)?;
    }
    if let Some(ref did) = data.document_id {
        crate::security::validation::validate_security_input(did)?;
    }

    let pool = get_pool()?;

    match data.location.to_lowercase().as_str() {
        "document" => {
            let doc_id = data.document_id.clone()
                .ok_or_else(|| StoryWeaverError::InvalidInput { message: "documentId required for location=document".to_string() })?;
            let mut doc = DocumentOps::get_by_id(&pool, &doc_id)
                .await?
                .ok_or_else(|| StoryWeaverError::DocumentNotFound { id: doc_id.clone() })?;
            if doc.content.is_empty() {
                doc.content = data.content.clone();
            } else {
                doc.content.push_str("\n\n");
                doc.content.push_str(&data.content);
            }
            // Optionally merge metadata if provided
            if let Some(meta) = data.metadata.as_ref() {
                let existing: serde_json::Value = serde_json::from_str(&doc.metadata).unwrap_or(serde_json::json!({}));
                let mut merged = existing.as_object().cloned().unwrap_or_default();
                if let Some(obj) = meta.as_object() {
                    for (k, v) in obj {
                        merged.insert(k.clone(), v.clone());
                    }
                } else {
                    merged.insert("aiSave".to_string(), meta.clone());
                }
                doc.metadata = serde_json::Value::Object(merged).to_string();
            }
            DocumentOps::update(&pool, &doc).await?;
            Ok(())
        }
        "snippet" | "note" => {
            let project_id = data.project_id.clone()
                .ok_or_else(|| StoryWeaverError::InvalidInput { message: "projectId required for location=snippet/note".to_string() })?;
            let title = data.title.clone().unwrap_or_else(|| {
                let ts = chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string();
                format!("AI Snippet {}", ts)
            });
            // Validate title text
            crate::security::validation::validate_content_length(&title, 200)?;
            crate::security::validation::validate_security_input(&title)?;
            let mut new_doc = Document::new(project_id, title, DocumentType::Notes);
            new_doc.content = data.content.clone();
            if let Some(meta) = data.metadata {
                new_doc.metadata = meta.to_string();
            }
            DocumentOps::create(&pool, new_doc).await?;
            Ok(())
        }
        other => Err(StoryWeaverError::InvalidInput { message: format!("Unsupported save location: {}", other) }),
    }
}

#[tauri::command]
pub async fn cancel_streaming_generation(streamId: String, _ai_state: State<'_, AdvancedAIState>) -> Result<()> {
    // Validate input and accept cancel in current placeholder implementation
    crate::security::validation::validate_security_input(&streamId)?;
    // In a future implementation, signal the running task to cancel.
    Ok(())
}
