//! Tauri commands for plugin system features

use crate::database::models::plugin::{
    Plugin, PluginCategory, PluginExecutionHistory, PluginExecutionRequest, PluginExecutionResult,
    PluginRating, PluginSearchResult, PluginSortOrder, PluginTemplate, PluginVisibility,
};
use crate::database::{get_pool, operations::plugin as plugin_ops};
use crate::error::{Result, StoryWeaverError};
use serde_json::Value;
use std::str::FromStr;

/// Create a new plugin
#[tauri::command]
pub async fn create_plugin(
    name: String,
    description: String,
    version: String,
    creator_id: String,
    category: String,
    visibility: String,
    prompt_template: String,
    variables: Option<Value>,
    ai_model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<i32>,
    tags: Option<Vec<String>>,
) -> Result<Plugin> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = PluginCategory::from_str(&category).map_err(|e| StoryWeaverError::invalid_input(e))?;
    let visibility_enum = PluginVisibility::from_str(&visibility).map_err(|e| StoryWeaverError::invalid_input(e))?;
    
    let plugin = Plugin {
        id: 0, // Will be set by database
        name,
        description,
        prompt_template,
        variables: variables
            .map(|v| serde_json::to_string(&v).unwrap_or_default())
            .unwrap_or_default(),
        ai_model: ai_model.unwrap_or_else(|| "gpt-3.5-turbo".to_string()),
        temperature: temperature.map(|t| t as f64),
        max_tokens: max_tokens.map(|mt| mt as i64),
        stop_sequences: None,
        category: category_enum,
        tags: tags.map(|t| t.join(",")),
        is_multi_stage: false,
        stage_count: Some(1),
        creator_id: Some(creator_id),
        is_public: visibility_enum == PluginVisibility::Published,
        version,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    plugin_ops::create_plugin_from_struct(&pool, plugin)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin: {}", e)))
}

/// Get plugin by ID
#[tauri::command]
pub async fn get_plugin(plugin_id: i32) -> Result<Option<Plugin>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    plugin_ops::get_plugin_by_id(&pool, &plugin_id.to_string())
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin: {}", e)))
}

/// Search plugins
#[tauri::command]
pub async fn search_plugins(
    query: Option<String>,
    category: Option<String>,
    sort_by: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<PluginSearchResult>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = category
        .map(|c| PluginCategory::from_str(&c))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?;
    
    let sort_order = sort_by
        .map(|s| PluginSortOrder::from_str(&s))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?
        .unwrap_or_default();

    plugin_ops::search_plugins(
        &pool,
        query.as_deref(),
        category_enum,
        sort_order,
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to search plugins: {}", e)))
}

/// Update plugin
#[tauri::command]
pub async fn update_plugin(
    plugin_id: i32,
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    category: Option<String>,
    visibility: Option<String>,
    code: Option<String>,
    metadata: Option<Value>,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    let category_enum = category
        .map(|c| PluginCategory::from_str(&c))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?;

    let visibility_enum = visibility
        .map(|v| PluginVisibility::from_str(&v))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?;

    plugin_ops::update_plugin(
        &pool,
        &plugin_id.to_string(),
        name.as_deref(),
        description.as_deref(),
        version.as_deref(),
        code.as_deref(),
        category_enum,
        visibility_enum,
        metadata,
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to update plugin: {}", e)))
}

/// Delete plugin
#[tauri::command]
pub async fn delete_plugin(plugin_id: i32) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    plugin_ops::delete_plugin(&pool, &plugin_id.to_string())
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete plugin: {}", e)))
}

/// Execute plugin
#[tauri::command]
pub async fn execute_plugin_command(
    plugin_id: i32,
    variables: Value,
    document_id: Option<i32>,
    selected_text: Option<String>,
    cursor_position: Option<i32>,
) -> Result<PluginExecutionResult> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    let request = PluginExecutionRequest {
        plugin_id,
        variables,
        document_id,
        selected_text,
        cursor_position,
    };

    let result = plugin_ops::record_plugin_execution(&pool, request, todo!()).await?;

    Ok(result)
}

/// Rate plugin
#[tauri::command]
pub async fn rate_plugin(
    plugin_id: i32,
    rating: i32,
    review: Option<String>,
    user_identifier: String,
) -> Result<PluginRating> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    if !(1..=5).contains(&rating) {
        return Err(StoryWeaverError::invalid_input(
            "Rating must be between 1 and 5".to_string(),
        ));
    }

    plugin_ops::create_plugin_rating(
        &pool,
        plugin_id,
        &user_identifier,
        rating,
        review.as_deref(),
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to rate plugin: {}", e)))
}

/// Get plugin ratings
#[tauri::command]
pub async fn get_plugin_ratings(
    plugin_id: i32,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<PluginRating>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    plugin_ops::get_plugin_ratings(&pool, plugin_id, limit.unwrap_or(10), offset.unwrap_or(0))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin ratings: {}", e)))
}

/// Get plugin usage statistics
#[tauri::command]
pub async fn get_plugin_daily_stats(
    plugin_id: i32,
) -> Result<Vec<crate::database::models::plugin::PluginDailyStats>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    plugin_ops::get_plugin_daily_stats(&pool, plugin_id, 30)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin usage stats: {}", e)))
}

/// Get plugin execution history
#[tauri::command]
pub async fn get_plugin_execution_history(
    plugin_id: Option<i32>,
    user_id: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<PluginExecutionHistory>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    plugin_ops::get_plugin_execution_history(
        &pool,
        plugin_id,
        user_id.as_deref(),
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    )
    .await
    .map_err(|e| {
        StoryWeaverError::database(format!("Failed to get plugin execution history: {}", e))
    })
}

/// Get plugin templates
#[tauri::command]
pub async fn get_plugin_templates(category: Option<String>) -> Result<Vec<PluginTemplate>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    let category_enum = category
        .map(|c| PluginCategory::from_str(&c))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?;

    plugin_ops::get_plugin_templates(&pool, category_enum)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin templates: {}", e)))
}

/// Get all plugins
#[tauri::command]
pub async fn get_plugins(
    category: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Plugin>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    let category_enum = category
        .map(|c| PluginCategory::from_str(&c))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?;

    plugin_ops::get_plugins(
        &pool,
        category_enum,
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to get plugins: {}", e)))
}

/// Record plugin execution
#[tauri::command]
pub async fn record_plugin_execution(
    plugin_id: i32,
    result_text: Option<String>,
    success: bool,
    error_message: Option<String>,
    credits_used: i32,
    execution_time_ms: i64,
) -> Result<PluginExecutionResult> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    let request = PluginExecutionRequest {
        plugin_id,
        variables: Value::Null,
        document_id: None,
        selected_text: None,
        cursor_position: None,
    };

    let result = PluginExecutionResult {
        success,
        result_text,
        error_message,
        credits_used,
        execution_time_ms,
        stage_results: None,
    };

    plugin_ops::record_plugin_execution(&pool, request, result)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record plugin execution: {}", e)))
}

/// Apply plugin template
#[tauri::command]
pub async fn apply_plugin_template(
    template_id: i32,
    name: String,
    variables: Option<Value>,
) -> Result<Plugin> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    // Get the template
    let template = plugin_ops::get_plugin_template_by_id(&pool, template_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin template: {}", e)))?
        .ok_or_else(|| StoryWeaverError::not_found("Plugin template", template_id.to_string()))?;

    // Create plugin from template
    let plugin = Plugin {
        id: 0,
        name,
        description: template.description,
        prompt_template: template.template_data,
        variables: variables
            .map(|v| serde_json::to_string(&v).unwrap_or_default())
            .unwrap_or_default(),
        ai_model: "gpt-3.5-turbo".to_string(),
        temperature: Some(0.7),
        max_tokens: Some(1000),
        stop_sequences: None,
        category: template.category,
        tags: Some("[]".to_string()),
        is_multi_stage: false,
        stage_count: Some(1),
        creator_id: Some("system".to_string()),
        is_public: false,
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    plugin_ops::create_plugin_from_struct(&pool, plugin)
        .await
        .map_err(|e| {
            StoryWeaverError::database(format!("Failed to create plugin from template: {}", e))
        })
}

/// Create plugin template
#[tauri::command]
pub async fn create_plugin_template(
    name: String,
    description: String,
    category: String,
    template_code: String,
    example_variables: Option<Value>,
) -> Result<PluginTemplate> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    let category_enum = PluginCategory::from_str(&category).map_err(|e| StoryWeaverError::invalid_input(e))?;

    plugin_ops::create_plugin_template(
        &pool,
        &name,
        &description,
        category_enum,
        &template_code,
        example_variables,
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin template: {}", e)))
}
