//! Tauri commands for plugin system features

use crate::database::{get_pool, models::plugin::*, operations::plugin::*};
use crate::database::models::plugin::{Plugin, PluginCategory, PluginVisibility, PluginSearchResult, PluginExecutionResult, PluginRating, PluginUsageStats, PluginExecutionHistory, PluginTemplate, PluginSortOrder, PluginExecutionRequest, PluginDailyStats};
use crate::error::{Result, StoryWeaverError};
use serde_json::Value;

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
    temperature: Option<f64>,
    max_tokens: Option<i32>,
    tags: Option<Vec<String>>,
) -> Result<Plugin, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = match category.as_str() {
        "writing" => PluginCategory::Writing,
        "editing" => PluginCategory::Editing,
        "analysis" => PluginCategory::Analysis,
        "brainstorming" => PluginCategory::Brainstorming,
        "research" => PluginCategory::Research,
        "formatting" => PluginCategory::Formatting,
        "other" => PluginCategory::Other,
        _ => return Err(StoryWeaverError::invalid_input("Invalid plugin category")),
    };
    
    let visibility_enum = match visibility.as_str() {
        "published" => PluginVisibility::Published,
        "private" => PluginVisibility::Private,
        "unlisted" => PluginVisibility::Unlisted,
        _ => return Err(StoryWeaverError::invalid_input("Invalid plugin visibility")),
    };
    
    let plugin = Plugin {
        id: 0, // Will be set by database
        name,
        description,
        prompt_template,
        variables: variables.map(|v| serde_json::to_string(&v).unwrap_or_default()).unwrap_or_default(),
        ai_model: ai_model.unwrap_or_else(|| "gpt-3.5-turbo".to_string()),
        temperature,
        max_tokens,
        stop_sequences: None,
        category: category_enum,
        tags: tags.map(|t| t.join(",")),
        is_multi_stage: false,
        stage_count: Some(1),
        creator_id,
        is_public: visibility_enum == PluginVisibility::Published,
        version,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    create_plugin(&pool, plugin)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin: {}", e)))
}

/// Get plugin by ID
#[tauri::command]
pub async fn get_plugin(
    plugin_id: i32,
) -> Result<Option<Plugin>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    get_plugin_by_id(&pool, plugin_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin: {}", e)))
}

/// Search plugins
#[tauri::command]
pub async fn search_plugins(
    query: Option<String>,
    category: Option<String>,
    author: Option<String>,
    tags: Option<Vec<String>>,
    sort_by: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<PluginSearchResult>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = if let Some(cat) = category {
        Some(match cat.as_str() {
            "text_processing" => PluginCategory::TextProcessing,
            "ai_integration" => PluginCategory::AIIntegration,
            "export" => PluginCategory::Export,
            "import" => PluginCategory::Import,
            "formatting" => PluginCategory::Formatting,
            "analysis" => PluginCategory::Analysis,
            "utility" => PluginCategory::Utility,
            "theme" => PluginCategory::Theme,
            "workflow" => PluginCategory::Workflow,
            "collaboration" => PluginCategory::Collaboration,
            _ => return Err(StoryWeaverError::invalid_input("Invalid plugin category")),
        })
    } else {
        None
    };
    
    let sort_order = if let Some(sort) = sort_by {
        match sort.as_str() {
            "name" => PluginSortOrder::Name,
            "rating" => PluginSortOrder::Rating,
            "downloads" => PluginSortOrder::Downloads,
            "recent" => PluginSortOrder::Recent,
            "relevance" => PluginSortOrder::Relevance,
            _ => PluginSortOrder::Rating,
        }
    } else {
        PluginSortOrder::Rating
    };
    
    search_plugins(
        &pool,
        query.as_deref(),
        category_enum,
        author.as_deref(),
        tags.as_deref(),
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
    variables: Option<Value>,
    icon_url: Option<String>,
    documentation_url: Option<String>,
    repository_url: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = if let Some(cat) = category {
        Some(match cat.as_str() {
            "text_processing" => PluginCategory::TextProcessing,
            "ai_integration" => PluginCategory::AIIntegration,
            "export" => PluginCategory::Export,
            "import" => PluginCategory::Import,
            "formatting" => PluginCategory::Formatting,
            "analysis" => PluginCategory::Analysis,
            "utility" => PluginCategory::Utility,
            "theme" => PluginCategory::Theme,
            "workflow" => PluginCategory::Workflow,
            "collaboration" => PluginCategory::Collaboration,
            _ => return Err(StoryWeaverError::invalid_input("Invalid plugin category")),
        })
    } else {
        None
    };
    
    let visibility_enum = if let Some(vis) = visibility {
        Some(match vis.as_str() {
            "public" => PluginVisibility::Public,
            "private" => PluginVisibility::Private,
            "unlisted" => PluginVisibility::Unlisted,
            _ => return Err(StoryWeaverError::invalid_input("Invalid plugin visibility")),
        })
    } else {
        None
    };
    
    update_plugin(
        &pool,
        plugin_id,
        name.as_deref(),
        description.as_deref(),
        version.as_deref(),
        category_enum,
        visibility_enum,
        code.as_deref(),
        variables.as_ref(),
        icon_url.as_deref(),
        documentation_url.as_deref(),
        repository_url.as_deref(),
        tags.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to update plugin: {}", e))
}

/// Delete plugin
#[tauri::command]
pub async fn delete_plugin(
    plugin_id: i32,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    delete_plugin(&pool, plugin_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete plugin: {}", e)))
}

/// Execute plugin
#[tauri::command]
pub async fn execute_plugin(
    plugin_id: i32,
    input_data: Value,
    variables: Option<Value>,
    user_id: Option<String>,
) -> Result<PluginExecutionResult, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    // Get the plugin
    let plugin = get_plugin_by_id(&pool, plugin_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin: {}", e)))?
        .ok_or_else(|| StoryWeaverError::not_found("Plugin not found".to_string()))?;
    
    // Create execution request
    let request = PluginExecutionRequest {
        plugin_id,
        input_data,
        variables: variables.unwrap_or(Value::Null),
        user_id,
    };
    
    // In a real implementation, this would execute the plugin code in a sandboxed environment
    // For now, we'll create a mock result
    let result = PluginExecutionResult {
        id: 0, // Will be set by database
        plugin_id,
        input_data: request.input_data.clone(),
        output_data: Value::String(format!("Executed plugin: {}", plugin.name)),
        variables: request.variables.clone(),
        execution_time_ms: 100,
        success: true,
        error_message: None,
        user_id: request.user_id.clone(),
        executed_at: chrono::Utc::now(),
    };
    
    // Record execution history
    // Store execution record in database
    crate::database::operations::plugin::create_plugin_execution(
        &pool,
        plugin_id,
        &request.input_data,
        &result.output_data,
        execution_time_ms,
        success,
        error_message.as_deref(),
        user_id.as_deref()
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to record plugin execution: {}", e)))?;
    
    Ok(result)
    
    // Update usage statistics
          update_plugin_usage_stats(&*pool, &plugin_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update usage stats: {}", e)))?;
    
    Ok(result)
}

/// Rate plugin
#[tauri::command]
pub async fn rate_plugin(
    plugin_id: i32,
    rating: i32,
    review: Option<String>,
    user_id: String,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if rating < 1 || rating > 5 {
        return Err(StoryWeaverError::invalid_input("Rating must be between 1 and 5"));
    }
    
    let plugin_rating = PluginRating {
        id: 0, // Will be set by database
        plugin_id,
        user_id,
        rating,
        review,
        created_at: chrono::Utc::now(),
    };
    
    create_plugin_rating(&pool, plugin_rating)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to rate plugin: {}", e)))
}

/// Get plugin ratings
#[tauri::command]
pub async fn get_plugin_ratings(
    plugin_id: i32,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<PluginRating>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    database::operations::plugin::get_plugin_ratings(&pool, plugin_id, limit.unwrap_or(10), offset.unwrap_or(0))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin ratings: {}", e)))
}

/// Get plugin usage statistics
#[tauri::command]
pub async fn get_plugin_usage_stats(
    plugin_id: i32,
) -> Result<Vec<PluginDailyStats>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    database::operations::plugin::get_plugin_usage_stats(&pool, plugin_id, 30)
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
) -> Result<Vec<PluginExecutionHistory>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    database::operations::plugin::get_plugin_execution_history(
        &pool,
        plugin_id,
        user_id.as_deref(),
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin execution history: {}", e)))
}

/// Get plugin templates
#[tauri::command]
pub async fn get_plugin_templates(
    category: Option<String>,
) -> Result<Vec<PluginTemplate>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = if let Some(cat) = category {
        Some(match cat.as_str() {
            "text_processing" => PluginCategory::TextProcessing,
            "ai_integration" => PluginCategory::AIIntegration,
            "export" => PluginCategory::Export,
            "import" => PluginCategory::Import,
            "formatting" => PluginCategory::Formatting,
            "analysis" => PluginCategory::Analysis,
            "utility" => PluginCategory::Utility,
            "theme" => PluginCategory::Theme,
            "workflow" => PluginCategory::Workflow,
            "collaboration" => PluginCategory::Collaboration,
            _ => return Err(StoryWeaverError::invalid_input("Invalid plugin category")),
        })
    } else {
        None
    };
    
    database::operations::plugin::get_plugin_templates(&pool, category_enum)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin templates: {}", e)))
}

/// Get all plugins
#[tauri::command]
pub async fn get_plugins(
    category: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Plugin>, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = if let Some(cat) = category {
        Some(match cat.as_str() {
            "writing" => PluginCategory::Writing,
            "editing" => PluginCategory::Editing,
            "analysis" => PluginCategory::Analysis,
            "brainstorming" => PluginCategory::Brainstorming,
            "research" => PluginCategory::Research,
            "formatting" => PluginCategory::Formatting,
            "other" => PluginCategory::Other,
            _ => return Err(StoryWeaverError::invalid_input("Invalid plugin category")),
        })
    } else {
        None
    };
    
    database::operations::plugin::get_plugins(&pool, category_enum, limit.unwrap_or(20), offset.unwrap_or(0))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugins: {}", e)))
}

/// Record plugin execution
#[tauri::command]
pub async fn record_plugin_execution(
    plugin_id: i32,
    input_data: Value,
    output_data: Value,
    execution_time_ms: i32,
    success: bool,
    error_message: Option<String>,
    user_id: Option<String>,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let variables_map = std::collections::HashMap::new(); // Empty variables for now
    
    let request = PluginExecutionRequest {
        plugin_id,
        variables: variables_map,
        document_id: None,
        selected_text: None,
        cursor_position: None,
    };
    
    let result = PluginExecutionResult {
        success,
        result_text: Some(output_data.to_string()),
        error_message,
        credits_used: 1, // Default credit usage
        execution_time_ms: execution_time_ms as i64,
        stage_results: None,
    };
    
    // Record execution in database
    crate::database::operations::plugin::create_plugin_execution(
        &pool,
        plugin_id,
        &input_data,
        &output_data,
        execution_time_ms as i64,
        success,
        error_message.as_deref(),
        user_id.as_deref(),
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to record plugin execution: {}", e)))?;
    
    Ok(())
}

/// Apply plugin template
#[tauri::command]
pub async fn apply_plugin_template(
    template_id: i32,
    name: String,
    variables: Option<Value>,
) -> Result<Plugin, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    // Get the template
    let template = get_plugin_template_by_id(&pool, template_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get plugin template: {}", e)))?
        .ok_or_else(|| StoryWeaverError::NotFound { 
            resource_type: "Plugin template".to_string(), 
            id: template_id.to_string() 
        })?;
    
    // Create plugin from template
    let plugin = Plugin {
        id: 0,
        name,
        description: template.description,
        prompt_template: template.template_data,
        variables: variables.map(|v| serde_json::to_string(&v).unwrap_or_default()).unwrap_or_default(),
        ai_model: "gpt-3.5-turbo".to_string(),
        temperature: 0.7,
        max_tokens: Some(1000),
        stop_sequences: None,
        category: template.category,
        tags: "[]".to_string(),
        is_multi_stage: false,
        stage_count: 1,
        creator_id: Some("system".to_string()),
        is_public: false,
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    crate::database::operations::plugin::create_plugin_from_struct(&pool, plugin)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin from template: {}", e)))
}

/// Create plugin template
#[tauri::command]
pub async fn create_plugin_template(
    name: String,
    description: String,
    category: String,
    template_code: String,
    variables_schema: Option<Value>,
    example_usage: Option<String>,
) -> Result<PluginTemplate, StoryWeaverError> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let category_enum = match category.as_str() {
        "writing" => PluginCategory::Writing,
        "editing" => PluginCategory::Editing,
        "analysis" => PluginCategory::Analysis,
        "brainstorming" => PluginCategory::Brainstorming,
        "research" => PluginCategory::Research,
        "formatting" => PluginCategory::Formatting,
        "other" => PluginCategory::Other,
        _ => return Err(StoryWeaverError::invalid_input("Invalid plugin category")),
    };
    
    let template = PluginTemplate {
        id: 0, // Will be set by database
        name,
        description,
        category: category_enum,
        template_data: template_code,
        is_official: false,
        created_at: chrono::Utc::now(),
    };
    
    crate::database::operations::plugin::create_plugin_template(
        &pool,
        &template.name,
        &template.description,
        template.category,
        &template.template_data,
        variables_schema
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin template: {}", e)))
}