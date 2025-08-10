use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use tokio::sync::Mutex;

use crate::ai::{
    AdvancedAIManager, AdvancedGenerationRequest, AdvancedGenerationResult,
    VisualizeRequest, GeneratedImage, BrainstormRequest, BrainstormSession,
    StyleExample, ProseMode, SaliencyContext, BrainstormIdea,
    saliency_engine::StoryBibleElements,
};
use crate::error::StoryWeaverError;

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
    pub story_bible: Option<StoryBibleElements>,
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
) -> Result<AdvancedGenerationResult, StoryWeaverError> {
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
    
    ai_manager
        .generate_with_advanced_features(advanced_request, request.story_bible)
        .await
        .map_err(StoryWeaverError::ai)
}

// Image Generation with Visualize Engine
#[tauri::command]
pub async fn generate_image(
    request: ImageGenerationRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<GeneratedImage, StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    let visualize_request = VisualizeRequest {
        project_id: request.project_id,
        source_text: request.text_content,
        style_preference: Some(request.style_preference),
        resolution: match request.resolution.as_str() {
            "1024x1024" => crate::ai::ImageResolution::Square1024,
            "1792x1024" => crate::ai::ImageResolution::Landscape1792,
            "1024x1792" => crate::ai::ImageResolution::Portrait1024,
            _ => crate::ai::ImageResolution::Square1024,
        },
        enhance_prompt: request.enhance_prompt,
    };
    
    ai_manager
        .generate_image(visualize_request)
        .await
        .map_err(StoryWeaverError::ai)
}

// Advanced Brainstorming
#[tauri::command]
pub async fn create_brainstorm_session(
    request: BrainstormSessionRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<String, StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    let brainstorm_request = BrainstormRequest {
        project_id: request.project_id,
        category: crate::ai::BrainstormCategory::Custom(request.category),
        seed_prompt: Some(request.context),
        context: None,
        num_ideas: request.num_ideas as usize,
        creativity_level: request.creativity_level as i32,
        focus_areas: vec![request.focus_area],
    };
    
    ai_manager
        .create_brainstorm_session(brainstorm_request)
        .await
        .map_err(StoryWeaverError::ai)
}

#[tauri::command]
pub async fn get_brainstorm_session(
    session_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Option<BrainstormSession>, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    Ok(ai_manager.get_brainstorm_session(&session_id).cloned())
}

#[tauri::command]
pub async fn rate_brainstorm_idea(
    session_id: String,
    idea_id: String,
    rating: u32,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<(), StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    // This would need to be implemented in the brainstorm engine
    // For now, return success
    Ok(())
}

#[tauri::command]
pub async fn mark_idea_as_keeper(
    session_id: String,
    idea_id: String,
    is_keeper: bool,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<(), StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    // This would need to be implemented in the brainstorm engine
    // For now, return success
    Ok(())
}

// Style Examples Management
#[tauri::command]
pub async fn add_style_example(
    request: StyleExampleRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<StyleExample, StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    let style_example = StyleExample {
        id: uuid::Uuid::new_v4().to_string(),
        project_id: request.project_id,
        name: request.name,
        content: request.content.clone(),
        word_count: request.content.split_whitespace().count() as i32,
        analysis_result: Some(ai_manager.analyze_style(&request.content)),
        is_active: request.is_active,
    };
    
    ai_manager.add_style_example(style_example.clone());
    
    Ok(style_example)
}

#[tauri::command]
pub async fn analyze_text_style(
    content: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<crate::ai::advanced_ai_manager::StyleAnalysis, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    Ok(ai_manager.analyze_style(&content))
}

// Prose Modes Management
#[tauri::command]
pub async fn get_available_prose_modes(
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Vec<ProseMode>, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    Ok(ai_manager.get_prose_modes().into_iter().cloned().collect())
}

#[tauri::command]
pub async fn get_prose_mode_details(
    mode_name: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Option<ProseMode>, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    Ok(ai_manager.get_prose_modes()
        .into_iter()
        .find(|mode| mode.name == mode_name)
        .cloned())
}

// Credit Management
#[tauri::command]
pub async fn get_credit_usage(
    project_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<CreditUsageResponse, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    let project_usage = ai_manager.get_credit_usage(&project_id);
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    Ok(CreditUsageResponse {
        project_usage,
        daily_usage: 0, // Would need to implement daily tracking
        monthly_limit: None, // Would need to implement limit management
        remaining_credits: None, // Would need to implement remaining calculation
    })
}

// Image Management
#[tauri::command]
pub async fn get_project_images(
    project_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<Vec<GeneratedImage>, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    Ok(ai_manager.get_generated_images(&project_id)
        .into_iter()
        .cloned()
        .collect())
}

#[tauri::command]
pub async fn delete_generated_image(
    image_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<(), StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    // This would need to be implemented in the visualize engine
    // For now, return success
    Ok(())
}

// Saliency Engine
#[tauri::command]
pub async fn build_saliency_context(
    project_id: String,
    text_context: String,
    story_bible: StoryBibleElements,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<SaliencyContext, StoryWeaverError> {
    let mut ai_manager = ai_state.lock().await;
    
    // This would access the saliency engine directly
    // For now, return a placeholder
    Ok(SaliencyContext {
        id: "placeholder_id".to_string(),
        project_id,
        context_hash: "placeholder".to_string(),
        selected_elements: crate::ai::SelectedElements {
            characters: Vec::new(),
            locations: Vec::new(),
            plot_threads: Vec::new(),
            worldbuilding: Vec::new(),
        },
        relevance_scores: HashMap::new(),
        total_tokens: 0,
        expires_at: Some(chrono::Utc::now()),
    })
}

// Smart Import with Novel Analysis
#[tauri::command]
pub async fn smart_import_content(
    project_id: String,
    content: String,
    content_type: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<HashMap<String, serde_json::Value>, StoryWeaverError> {
    let ai_manager = ai_state.lock().await;
    
    // Analyze the content using AI to extract story elements
    let analysis_result = ai_manager.analyze_content_for_import(&project_id, &content, &content_type).await
        .map_err(StoryWeaverError::ai)?;
    
    let mut result = HashMap::new();
    result.insert("status".to_string(), serde_json::Value::String("analyzed".to_string()));
    result.insert("suggestions".to_string(), serde_json::to_value(&analysis_result.suggestions)
        .map_err(StoryWeaverError::serialization)?);
    result.insert("extracted_elements".to_string(), serde_json::to_value(&analysis_result.extracted_elements)
        .map_err(StoryWeaverError::serialization)?);
    
    Ok(result)
}

// Streaming Generation (placeholder for future implementation)
#[tauri::command]
pub async fn start_streaming_generation(
    request: ProseGenerationRequest,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<String, StoryWeaverError> {
    // This would start a streaming generation and return a stream ID
    Ok(uuid::Uuid::new_v4().to_string())
}

#[tauri::command]
pub async fn get_stream_status(
    stream_id: String,
    ai_state: State<'_, AdvancedAIState>,
) -> Result<HashMap<String, serde_json::Value>, StoryWeaverError> {
    // This would return the current status of a streaming generation
    let mut result = HashMap::new();
    result.insert("status".to_string(), serde_json::Value::String("completed".to_string()));
    result.insert("progress".to_string(), serde_json::Value::Number(serde_json::Number::from(100)));
    
    Ok(result)
}