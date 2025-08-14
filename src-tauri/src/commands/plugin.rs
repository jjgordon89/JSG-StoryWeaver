//! Tauri commands for plugin system features

use crate::database::models::plugin::{
    Plugin, PluginCategory, PluginExecutionHistory, PluginExecutionRequest, PluginExecutionResult,
    PluginRating, PluginSearchResult, PluginSortOrder, PluginTemplate, PluginVisibility,
};
use crate::database::{get_pool, operations::plugin as plugin_ops};
use crate::error::{Result, StoryWeaverError};
use crate::security::plugin_security::{
    validate_plugin_security, validate_plugin_execution_context, quick_plugin_security_check,
};
use serde_json::Value;
use std::str::FromStr;
use tauri::State;
use std::sync::Arc;
use crate::ai::{AIProviderManager, AIContext};

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
    // Quick security check for plugin creation
    quick_plugin_security_check(&name, &description, &prompt_template, &variables)?;

    // Additional input validation
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Plugin name cannot be empty".to_string()));
    }
    if name.len() > 255 {
        return Err(StoryWeaverError::validation("Plugin name too long (max 255 characters)".to_string()));
    }
    if description.trim().is_empty() {
        return Err(StoryWeaverError::validation("Plugin description cannot be empty".to_string()));
    }
    if description.len() > 2000 {
        return Err(StoryWeaverError::validation("Plugin description too long (max 2000 characters)".to_string()));
    }
    if version.trim().is_empty() {
        return Err(StoryWeaverError::validation("Plugin version cannot be empty".to_string()));
    }
    if version.len() > 50 {
        return Err(StoryWeaverError::validation("Plugin version too long (max 50 characters)".to_string()));
    }
    crate::security::validate_security_input(&version)?;
    crate::security::validate_security_input(&creator_id)?;
    if let Some(temp) = temperature {
        if temp < 0.0 || temp > 2.0 {
            return Err(StoryWeaverError::validation("Temperature must be between 0.0 and 2.0".to_string()));
        }
    }
    if let Some(tokens) = max_tokens {
        if tokens <= 0 || tokens > 100000 {
            return Err(StoryWeaverError::validation("Max tokens must be between 1 and 100000".to_string()));
        }
    }
    if let Some(ref model) = ai_model {
        crate::security::validate_security_input(model)?;
    }

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
        creator_id: Some(creator_id.clone()),
        is_public: visibility_enum == PluginVisibility::Published,
        version,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = plugin_ops::create_plugin_from_struct(&pool, plugin)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create plugin: {}", e)))?;
    // Upsert marketplace entry to reflect visibility/creator
    let _ = plugin_ops::upsert_plugin_marketplace_entry(
        &pool,
        created.id as i32,
        &creator_id,
        visibility_enum,
        false,
    )
    .await;
    Ok(created)
}

/// Get plugin by ID
#[tauri::command]
pub async fn get_plugin(plugin_id: i32) -> Result<Option<Plugin>> {
    // Input validation
    if plugin_id <= 0 {
        return Err(StoryWeaverError::validation("Invalid plugin_id".to_string()));
    }

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
    // Input validation
    if let Some(ref q) = query {
        if q.len() > 500 {
            return Err(StoryWeaverError::validation("Search query too long (max 500 characters)".to_string()));
        }
        crate::security::validate_security_input(q)?;
    }
    if let Some(lim) = limit {
        if lim <= 0 || lim > 100 {
            return Err(StoryWeaverError::validation("Limit must be between 1 and 100".to_string()));
        }
    }
    if let Some(off) = offset {
        if off < 0 {
            return Err(StoryWeaverError::validation("Offset cannot be negative".to_string()));
        }
    }

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
    state: State<'_, Arc<AIProviderManager>>,
    plugin_id: i32,
    variables: Value,
    document_id: Option<i32>,
    selected_text: Option<String>,
    cursor_position: Option<i32>,
) -> Result<PluginExecutionResult> {
    // Basic input safeguards
    if plugin_id <= 0 {
        return Err(StoryWeaverError::validation("Invalid plugin_id".to_string()));
    }

    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    // Fetch plugin definition
    let plugin = plugin_ops::get_plugin_by_id(&pool, &plugin_id.to_string())
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to load plugin: {}", e)))?
        .ok_or_else(|| StoryWeaverError::not_found("Plugin", &plugin_id.to_string()))?;

    // Comprehensive plugin security validation
    let security_result = validate_plugin_security(&plugin)?;
    if !security_result.is_safe {
        return Err(StoryWeaverError::validation(format!(
            "Plugin failed security validation: {}",
            security_result.errors.join("; ")
        )));
    }

    // Validate execution context
    validate_plugin_execution_context(&plugin, &variables, &selected_text)?;

    // Render prompt by injecting variables into the template
    fn render_prompt(template: &str, vars: &Value, selected_text: &Option<String>) -> String {
        let mut result = template.to_string();

        if let Some(obj) = vars.as_object() {
            for (k, v) in obj {
                let placeholder = format!("{{{{{}}}}}", k);
                let replacement = match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                };
                result = result.replace(&placeholder, &replacement);
            }
        }

        // Support {{selected_text}} placeholder
        if let Some(sel) = selected_text {
            result = result.replace("{{selected_text}}", sel);
        }

        result
    }

    // Validate prompt template size and substitute
    crate::security::validation::validate_content_length(&plugin.prompt_template, 20000)?;
    let prompt = render_prompt(&plugin.prompt_template, &variables, &selected_text);
    crate::security::validation::validate_content_length(&prompt, 20000)?;
    crate::security::validation::validate_security_input(&prompt)?;

    // Build AI context
    let mut ai_ctx = AIContext::default();
    ai_ctx.document_id = document_id.map(|d| d.to_string());
    ai_ctx.selected_text = selected_text.clone();

    // Execute via default AI provider (model routing can be added later)
    let provider = state
        .get_default_provider()
        .ok_or_else(|| StoryWeaverError::ai("No AI provider available"))?;

    let start = std::time::Instant::now();
    let generated = provider
        .generate_text(&prompt, &ai_ctx)
        .await
        .map_err(|e| StoryWeaverError::ai(e.to_string()))?;

    let elapsed_ms = start.elapsed().as_millis() as i64;
    let token_estimate = (generated.len() as f32 / 4.0) as i32;

    let result = PluginExecutionResult {
        success: true,
        result_text: Some(generated.clone()),
        error_message: None,
        credits_used: token_estimate, // simple estimate; can wire to real credit tracking later
        execution_time_ms: elapsed_ms,
        stage_results: None,
    };

    // Record execution
    let request = PluginExecutionRequest {
        plugin_id,
        variables,
        document_id,
        selected_text,
        cursor_position,
    };
    let recorded = plugin_ops::record_plugin_execution(&pool, request, result.clone()).await?;

    Ok(recorded)
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
        .ok_or_else(|| StoryWeaverError::not_found("Plugin template", &template_id.to_string()))?;

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

/// Validate plugin security
#[tauri::command]
pub async fn validate_plugin_security_command(plugin_id: i32) -> Result<crate::security::plugin_security::PluginSecurityValidationResult> {
    // Input validation
    if plugin_id <= 0 {
        return Err(StoryWeaverError::validation("Invalid plugin_id".to_string()));
    }

    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;

    // Fetch plugin definition
    let plugin = plugin_ops::get_plugin_by_id(&pool, &plugin_id.to_string())
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to load plugin: {}", e)))?
        .ok_or_else(|| StoryWeaverError::not_found("Plugin", &plugin_id.to_string()))?;

    // Run comprehensive security validation
    validate_plugin_security(&plugin)
}
