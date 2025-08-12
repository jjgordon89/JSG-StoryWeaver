use crate::database::{get_pool};
use crate::database::operations::StyleExampleOps;
use crate::database::models::StyleExample;
use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStyleExampleRequest {
    pub project_id: String,
    pub user_id: String,
    pub example_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStyleExampleRequest {
    pub id: i64,
    pub example_text: Option<String>,
    pub analysis_result: Option<String>,
    pub generated_style_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleExampleResponse {
    pub id: i64,
    pub project_id: String,
    pub user_id: Option<String>,
    pub example_text: String,
    pub analysis_result: Option<String>,
    pub generated_style_prompt: Option<String>,
    pub word_count: Option<i32>,
    pub created_at: String,
}

impl From<StyleExample> for StyleExampleResponse {
    fn from(style_example: StyleExample) -> Self {
        Self {
            id: style_example.id,
            project_id: style_example.project_id,
            user_id: style_example.user_id,
            example_text: style_example.example_text,
            analysis_result: style_example.analysis_result,
            generated_style_prompt: style_example.generated_style_prompt,
            word_count: style_example.word_count,
            created_at: style_example.created_at.to_rfc3339(),
        }
    }
}

/// Create a new style example
#[command]
pub async fn create_style_example(
    request: CreateStyleExampleRequest,
) -> Result<StyleExampleResponse> {
    // Input validation
    crate::security::validate_security_input(&request.project_id)?;
    crate::security::validate_security_input(&request.user_id)?;
    if request.example_text.trim().is_empty() {
        return Err(StoryWeaverError::validation("Example text cannot be empty".to_string()));
    }
    if request.example_text.len() > 20000 {
        return Err(StoryWeaverError::validation("Example text too long (max 20000 characters)".to_string()));
    }
    crate::security::validate_security_input(&request.example_text)?;

    let pool = get_pool()?;
    
    let style_example = StyleExample::new(
        request.project_id,
        Some(request.user_id),
        request.example_text,
    );
    
    let created = StyleExampleOps::create(&pool, style_example).await?;
    Ok(created.into())
}

/// Get all style examples for a project
#[command]
pub async fn get_style_examples_by_project(
    project_id: String,
) -> Result<Vec<StyleExampleResponse>> {
    // Input validation
    crate::security::validate_security_input(&project_id)?;

    let pool = get_pool()?;
    
    let style_examples = StyleExampleOps::get_by_project(&pool, &project_id).await?;
    Ok(style_examples.into_iter().map(|se| se.into()).collect())
}

/// Get analyzed style examples for a project
#[command]
pub async fn get_analyzed_style_examples(
    project_id: String,
) -> Result<Vec<StyleExampleResponse>> {
    // Input validation
    crate::security::validate_security_input(&project_id)?;

    let pool = get_pool()?;
    
    let style_examples = StyleExampleOps::get_analyzed_by_project(&pool, &project_id).await?;
    Ok(style_examples.into_iter().map(|se| se.into()).collect())
}

/// Get a specific style example by ID
#[command]
pub async fn get_style_example_by_id(
    id: i64,
) -> Result<StyleExampleResponse> {
    // Input validation
    if id <= 0 {
        return Err(StoryWeaverError::validation("Invalid id".to_string()));
    }

    let pool = get_pool()?;
    
    let style_example = StyleExampleOps::get_by_id(&pool, id).await
        .map_err(|_| StoryWeaverError::Internal { 
            message: format!("StyleExample not found: {}", id) 
        })?;
    
    Ok(style_example.into())
}

/// Update a style example
#[command]
pub async fn update_style_example(
    request: UpdateStyleExampleRequest,
) -> Result<StyleExampleResponse> {
    // Input validation
    if request.id <= 0 {
        return Err(StoryWeaverError::validation("Invalid id".to_string()));
    }
    if let Some(ref example_text) = request.example_text {
        if example_text.trim().is_empty() {
            return Err(StoryWeaverError::validation("Example text cannot be empty".to_string()));
        }
        if example_text.len() > 20000 {
            return Err(StoryWeaverError::validation("Example text too long (max 20000 characters)".to_string()));
        }
        crate::security::validate_security_input(example_text)?;
    }
    if let Some(ref analysis_result) = request.analysis_result {
        if analysis_result.len() > 10000 {
            return Err(StoryWeaverError::validation("Analysis result too long (max 10000 characters)".to_string()));
        }
        crate::security::validate_security_input(analysis_result)?;
    }
    if let Some(ref generated_style_prompt) = request.generated_style_prompt {
        if generated_style_prompt.len() > 5000 {
            return Err(StoryWeaverError::validation("Generated style prompt too long (max 5000 characters)".to_string()));
        }
        crate::security::validate_security_input(generated_style_prompt)?;
    }

    let pool = get_pool()?;
    
    // Get the existing style example
    let mut style_example = StyleExampleOps::get_by_id(&pool, request.id).await?;
    
    // Update fields if provided
    if let Some(example_text) = request.example_text {
        style_example.example_text = example_text;
        style_example.word_count = Some(style_example.example_text.split_whitespace().count() as i32);
    }
    if let Some(analysis_result) = request.analysis_result {
        style_example.analysis_result = Some(analysis_result);
    }
    if let Some(generated_style_prompt) = request.generated_style_prompt {
        style_example.generated_style_prompt = Some(generated_style_prompt);
    }
    
    StyleExampleOps::update(&pool, &style_example).await?;
    Ok(style_example.into())
}

/// Delete a style example
#[command]
pub async fn delete_style_example(
    id: i64,
) -> Result<()> {
    // Input validation
    if id <= 0 {
        return Err(StoryWeaverError::validation("Invalid id".to_string()));
    }

    let pool = get_pool()?;
    
    StyleExampleOps::delete(&pool, id).await?;
    Ok(())
}

/// Delete all style examples for a project
#[command]
pub async fn delete_style_examples_by_project(
    project_id: String,
) -> Result<()> {
    // Input validation
    if !crate::security::is_safe_input(&project_id) {
        return Err(StoryWeaverError::validation("Invalid project_id".to_string()));
    }

    let pool = get_pool()?;
    
    StyleExampleOps::delete_by_project(&pool, &project_id).await?;
    Ok(())
}
