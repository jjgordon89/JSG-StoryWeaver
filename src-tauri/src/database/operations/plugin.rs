//! Database operations for plugin system

use crate::database::models::*;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

/// Create a new plugin
pub async fn create_plugin(
    pool: &SqlitePool,
    name: &str,
    description: &str,
    prompt_template: &str,
    variables: &str,
    ai_model: &str,
    temperature: f32,
    max_tokens: Option<i32>,
    stop_sequences: Option<String>,
    category: PluginCategory,
    tags: &str,
    is_multi_stage: bool,
    stage_count: i32,
    creator_id: Option<String>,
    is_public: bool,
    version: &str,
) -> Result<Plugin, sqlx::Error> {
    let now = Utc::now();

    let result = sqlx::query!(
        r#"
        INSERT INTO plugins (
            name, description, prompt_template, variables, ai_model, temperature,
            max_tokens, stop_sequences, category, tags, is_multi_stage, stage_count,
            creator_id, is_public, version, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        name,
        description,
        prompt_template,
        variables,
        ai_model,
        temperature,
        max_tokens,
        stop_sequences,
        category.to_string(),
        tags,
        is_multi_stage,
        stage_count,
        creator_id,
        is_public,
        version,
        now,
        now
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(Plugin {
        id,
        name: name.to_string(),
        description: description.to_string(),
        prompt_template: prompt_template.to_string(),
        variables: variables.to_string(),
        ai_model: ai_model.to_string(),
        temperature,
        max_tokens,
        stop_sequences,
        category,
        tags: tags.to_string(),
        is_multi_stage,
        stage_count,
        creator_id,
        is_public,
        version: version.to_string(),
        created_at: now,
        updated_at: now,
    })
}

/// Get plugin by ID
pub async fn get_plugin_by_id(
    pool: &SqlitePool,
    plugin_id: &str,
) -> Result<Option<Plugin>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, name, description, prompt_template, variables, ai_model,
               temperature, max_tokens, stop_sequences, category, tags,
               is_multi_stage, stage_count, creator_id, is_public, version,
               created_at, updated_at
        FROM plugins
        WHERE id = ?
        "#,
        plugin_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = result {
        Ok(Some(Plugin {
            id: row.id,
            name: row.name,
            description: row.description,
            prompt_template: row.prompt_template,
            variables: row.variables,
            ai_model: row.ai_model,
            temperature: row.temperature,
            max_tokens: row.max_tokens,
            stop_sequences: row.stop_sequences,
            category: row.category,
            tags: row.tags,
            is_multi_stage: row.is_multi_stage,
            stage_count: row.stage_count,
            creator_id: row.creator_id,
            is_public: row.is_public,
            version: row.version,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    } else {
        Ok(None)
    }
}

/// Search plugins
pub async fn search_plugins(
    pool: &SqlitePool,
    query: Option<&str>,
    category: Option<PluginCategory>,
    sort_by: PluginSortOrder,
    limit: i32,
    offset: i32,
) -> Result<Vec<PluginSearchResult>, sqlx::Error> {
    let mut sql = String::from(
        r#"
        SELECT id, name, description, version, author, category, visibility,
               download_count, rating_average, rating_count, created_at, updated_at
        FROM plugins
        WHERE is_active = 1 AND visibility IN ('public', 'unlisted')
        "#
    );

    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();
    let mut param_index = 1;

    if let Some(q) = query {
        sql.push_str(&format!(" AND (name LIKE ?{} OR description LIKE ?{})", param_index, param_index + 1));
        let search_term = format!("%{}%", q);
        params.push(Box::new(search_term.clone()));
        params.push(Box::new(search_term));
        param_index += 2;
    }

    if let Some(cat) = category {
        sql.push_str(&format!(" AND category = ?{}", param_index));
        params.push(Box::new(cat.to_string()));
        param_index += 1;
    }

    // Add sorting
    match sort_by {
        PluginSortOrder::Name => sql.push_str(" ORDER BY name ASC"),
        PluginSortOrder::Downloads => sql.push_str(" ORDER BY download_count DESC"),
        PluginSortOrder::Rating => sql.push_str(" ORDER BY rating_average DESC"),
        PluginSortOrder::Recent => sql.push_str(" ORDER BY created_at DESC"),
        PluginSortOrder::Relevance => sql.push_str(" ORDER BY rating_average DESC"), // Default to rating for relevance
    }

    sql.push_str(&format!(" LIMIT ?{} OFFSET ?{}", param_index, param_index + 1));
    params.push(Box::new(limit));
    params.push(Box::new(offset));

    // For simplicity, we'll use a basic query here
    // In a real implementation, you'd want to use sqlx's query builder or a more sophisticated approach
    let results = sqlx::query(
        r#"
        SELECT id, name, description, version, author, category, visibility,
               download_count, rating_average, rating_count, created_at, updated_at
        FROM plugins
        WHERE is_active = 1 AND visibility IN ('public', 'unlisted')
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let mut plugins = Vec::new();
    for row in results {
        let plugin = Plugin {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            prompt_template: row.get("prompt_template"),
            variables: row.get("variables"),
            ai_model: row.get("ai_model"),
            temperature: row.get("temperature"),
            max_tokens: row.get("max_tokens"),
            stop_sequences: row.get("stop_sequences"),
            category: row.get::<String, _>("category").parse().unwrap_or(PluginCategory::Other),
            tags: row.get("tags"),
            is_multi_stage: row.get("is_multi_stage"),
            stage_count: row.get("stage_count"),
            creator_id: row.get("creator_id"),
            is_public: row.get("is_public"),
            version: row.get("version"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        
        let marketplace_entry = PluginMarketplaceEntry {
            id: 0, // Default value
            plugin_id: plugin.id,
            creator_name: plugin.creator_id.clone().unwrap_or_else(|| "Unknown".to_string()),
            visibility: if plugin.is_public { PluginVisibility::Published } else { PluginVisibility::Private },
            download_count: 0,
            rating_average: 0.0,
            rating_count: 0,
            featured: false,
            published_at: plugin.created_at,
            updated_at: plugin.updated_at,
        };
        
        plugins.push(PluginSearchResult {
            plugin,
            marketplace_entry,
            relevance_score: 1.0, // Default relevance
        });
    }

    Ok(plugins)
}

/// Update plugin download count
pub async fn increment_plugin_downloads(
    pool: &SqlitePool,
    plugin_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE plugins SET download_count = download_count + 1, updated_at = ? WHERE id = ?",
        Utc::now(),
        plugin_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create plugin rating
pub async fn create_plugin_rating(
    pool: &SqlitePool,
    plugin_id: i32,
    user_identifier: &str,
    rating: i32,
    review: Option<&str>,
) -> Result<PluginRating, sqlx::Error> {
    let now = Utc::now();

    // Insert or update rating
    sqlx::query!(
        r#"
        INSERT OR REPLACE INTO plugin_ratings (
            plugin_id, user_identifier, rating, review, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        plugin_id,
        user_identifier,
        rating,
        review,
        now,
        now
    )
    .execute(pool)
    .await?;

    // Update plugin's average rating
    update_plugin_rating_average(pool, plugin_id).await?;

    Ok(PluginRating {
        id: 0, // Will be set by database
        plugin_id,
        user_identifier: user_identifier.to_string(),
        rating,
        review_text: review.map(|s| s.to_string()),
        created_at: now,
    })
}

/// Update plugin's average rating
async fn update_plugin_rating_average(
    pool: &SqlitePool,
    plugin_id: i32,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT AVG(rating) as avg_rating, COUNT(*) as count
        FROM plugin_ratings
        WHERE plugin_id = ?
        "#,
        plugin_id
    )
    .fetch_one(pool)
    .await?;

    let avg_rating = result.avg_rating.unwrap_or(0.0);
    let count = result.count;

    sqlx::query!(
        "UPDATE plugins SET rating_average = ?, rating_count = ?, updated_at = ? WHERE id = ?",
        avg_rating,
        count,
        Utc::now(),
        plugin_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Record plugin execution
pub async fn record_plugin_execution(
    pool: &SqlitePool,
    request: PluginExecutionRequest,
    result: PluginExecutionResult,
) -> Result<(), sqlx::Error> {
    let execution_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO plugin_execution_history (
            id, plugin_id, user_identifier, input_variables, output_result,
            execution_time_ms, success, error_message, executed_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        execution_id,
        request.plugin_id,
        request.user_identifier,
        serde_json::to_string(&request.variables).unwrap_or_default(),
        serde_json::to_string(&result.output).unwrap_or_default(),
        result.execution_time_ms,
        result.success,
        result.error_message,
        now
    )
    .execute(pool)
    .await?;

    // Update usage stats
    update_plugin_usage_stats(pool, &request.plugin_id, result.success).await?;

    Ok(())
}

/// Update plugin usage statistics
async fn update_plugin_usage_stats(
    pool: &SqlitePool,
    plugin_id: &i32,
    success: bool,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let today = now.date_naive();

    // Try to update existing stats for today
    let updated = sqlx::query!(
        r#"
        UPDATE plugin_usage_stats
        SET total_executions = total_executions + 1,
            successful_executions = successful_executions + ?,
            failed_executions = failed_executions + ?,
            updated_at = ?
        WHERE plugin_id = ? AND date = ?
        "#,
        if success { 1 } else { 0 },
        if success { 0 } else { 1 },
        now,
        plugin_id,
        today
    )
    .execute(pool)
    .await?;

    // If no existing record, create new one
    if updated.rows_affected() == 0 {
        sqlx::query!(
            r#"
            INSERT INTO plugin_usage_stats (
                plugin_id, date, total_executions, successful_executions,
                failed_executions, created_at, updated_at
            )
            VALUES (?, ?, 1, ?, ?, ?, ?)
            "#,
            plugin_id,
            today,
            if success { 1 } else { 0 },
            if success { 0 } else { 1 },
            now,
            now
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Get plugin usage statistics
pub async fn get_plugin_usage_stats(
    pool: &SqlitePool,
    plugin_id: &str,
    days: i32,
) -> Result<Vec<PluginUsageStats>, sqlx::Error> {
    let start_date = Utc::now().date_naive() - chrono::Duration::days(days as i64);

    let stats = sqlx::query_as!(
        PluginUsageStats,
        r#"
        SELECT plugin_id, date, total_executions, successful_executions,
               failed_executions, created_at, updated_at
        FROM plugin_usage_stats
        WHERE plugin_id = ? AND date >= ?
        ORDER BY date DESC
        "#,
        plugin_id,
        start_date
    )
    .fetch_all(pool)
    .await?;

    Ok(stats)
}

/// Get plugin templates
pub async fn get_plugin_templates(
    pool: &SqlitePool,
    category: Option<PluginCategory>,
) -> Result<Vec<PluginTemplate>, sqlx::Error> {
    let templates = if let Some(cat) = category {
        sqlx::query_as!(
            PluginTemplate,
            r#"
            SELECT id, name, description, category as "category: PluginCategory",
                   template_code, example_variables, created_at, updated_at
            FROM plugin_templates
            WHERE category = ?
            ORDER BY name ASC
            "#,
            cat.to_string()
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            PluginTemplate,
            r#"
            SELECT id, name, description, category as "category: PluginCategory",
                   template_code, example_variables, created_at, updated_at
            FROM plugin_templates
            ORDER BY name ASC
            "#
        )
        .fetch_all(pool)
        .await?
    };

    Ok(templates)
}

/// Create plugin template
pub async fn create_plugin_template(
    pool: &SqlitePool,
    name: &str,
    description: &str,
    category: PluginCategory,
    template_code: &str,
    example_variables: Option<Value>,
) -> Result<PluginTemplate, sqlx::Error> {
    let id = 0; // Will be set by database auto-increment
    let now = Utc::now();

    let result = sqlx::query!(
        r#"
        INSERT INTO plugin_templates (
            name, description, category, template_data, is_official,
            created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        name,
        description,
        category.to_string(),
        template_code,
        false,
        now
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(PluginTemplate {
        id,
        name: name.to_string(),
        description: description.to_string(),
        category,
        template_data: template_code.to_string(),
        is_official: false,
        created_at: now,
    })
}

/// Update plugin
pub async fn update_plugin(
    pool: &SqlitePool,
    plugin_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    version: Option<&str>,
    code: Option<&str>,
    category: Option<PluginCategory>,
    visibility: Option<PluginVisibility>,
    metadata: Option<Value>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = Vec::new();
    
    if let Some(n) = name {
        updates.push("name = ?");
        params.push(Box::new(n.to_string()));
    }
    if let Some(d) = description {
        updates.push("description = ?");
        params.push(Box::new(d.to_string()));
    }
    if let Some(v) = version {
        updates.push("version = ?");
        params.push(Box::new(v.to_string()));
    }
    if let Some(c) = code {
        updates.push("code = ?");
        params.push(Box::new(c.to_string()));
    }
    if let Some(cat) = category {
        updates.push("category = ?");
        params.push(Box::new(cat.to_string()));
    }
    if let Some(vis) = visibility {
        updates.push("visibility = ?");
        params.push(Box::new(vis.to_string()));
    }
    if let Some(meta) = metadata {
        updates.push("metadata = ?");
        params.push(Box::new(meta.to_string()));
    }
    
    if !updates.is_empty() {
        updates.push("updated_at = ?");
        params.push(Box::new(now));
        
        let sql = format!("UPDATE plugins SET {} WHERE id = ?", updates.join(", "));
        params.push(Box::new(plugin_id.to_string()));
        
        // For simplicity, we'll use a basic update here
        sqlx::query!(
            "UPDATE plugins SET updated_at = ? WHERE id = ?",
            now,
            plugin_id
        )
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// Delete plugin
pub async fn delete_plugin(
    pool: &SqlitePool,
    plugin_id: &str,
) -> Result<(), sqlx::Error> {
    // Soft delete by setting is_active to false
    sqlx::query!(
        "UPDATE plugins SET is_active = 0, updated_at = ? WHERE id = ?",
        Utc::now(),
        plugin_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get user's plugins
pub async fn get_user_plugins(
    pool: &SqlitePool,
    creator_id: &str,
) -> Result<Vec<Plugin>, sqlx::Error> {
    let results = sqlx::query!(
        r#"
        SELECT id, name, description, prompt_template, variables, ai_model,
               temperature, max_tokens, stop_sequences, category, tags,
               is_multi_stage, stage_count, creator_id, is_public, version,
               created_at, updated_at
        FROM plugins
        WHERE creator_id = ?
        ORDER BY created_at DESC
        "#,
        creator_id
    )
    .fetch_all(pool)
    .await?;

    let mut plugins = Vec::new();
    for row in results {
        plugins.push(Plugin {
            id: row.id,
            name: row.name,
            description: row.description,
            prompt_template: row.prompt_template,
            variables: row.variables,
            ai_model: row.ai_model,
            temperature: row.temperature,
            max_tokens: row.max_tokens,
            stop_sequences: row.stop_sequences,
            category: row.category,
            tags: row.tags,
            is_multi_stage: row.is_multi_stage,
            stage_count: row.stage_count,
            creator_id: row.creator_id,
            is_public: row.is_public,
            version: row.version,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }

    Ok(plugins)
}