//! Database operations for plugin system

use crate::database::models::plugin::{
    Plugin, PluginCategory, PluginDailyStats, PluginExecutionHistory, PluginExecutionRequest,
    PluginExecutionResult, PluginMarketplaceEntry, PluginRating, PluginSearchResult,
    PluginSortOrder, PluginTemplate, PluginVisibility,
};
use chrono::{NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, Row, SqlitePool};
use uuid::Uuid;

/// Create a new plugin from Plugin struct
pub async fn create_plugin_from_struct(
    pool: &SqlitePool,
    plugin: Plugin,
) -> Result<Plugin, sqlx::Error> {
    let now = Utc::now();
    let plugin_category_str = plugin.category.to_string();

    let result = sqlx::query!(
        r#"
        INSERT INTO plugins (
            name, description, prompt_template, variables, ai_model, temperature,
            max_tokens, stop_sequences, category, tags, is_multi_stage, stage_count,
            creator_id, is_public, version, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        plugin.name,
        plugin.description,
        plugin.prompt_template,
        plugin.variables,
        plugin.ai_model,
        plugin.temperature,
        plugin.max_tokens,
        plugin.stop_sequences,
        plugin_category_str,
        plugin.tags,
        plugin.is_multi_stage,
        plugin.stage_count,
        plugin.creator_id,
        plugin.is_public,
        plugin.version,
        now.naive_utc(),
        now.naive_utc()
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();

    Ok(Plugin {
        id,
        ..plugin
    })
}

/// Get plugin by ID
pub async fn get_plugin_by_id(
    pool: &SqlitePool,
    plugin_id: &str,
) -> Result<Option<Plugin>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, name, description, prompt_template, variables, ai_model,
               temperature, max_tokens, stop_sequences, category, tags,
               is_multi_stage, stage_count, creator_id, is_public, version,
               created_at, updated_at
        FROM plugins
        WHERE id = ?
        "#,
    )
    .bind(plugin_id)
    .fetch_optional(pool)
    .await?
    .map(|row| {
        Ok(Plugin {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            prompt_template: row.get("prompt_template"),
            variables: row.get("variables"),
            ai_model: row.get("ai_model"),
            temperature: row.get("temperature"),
            max_tokens: row.get("max_tokens"),
            stop_sequences: row.get("stop_sequences"),
            category: row
                .get::<'_, String, _>("category")
                .parse()
                .unwrap_or_default(),
            tags: row.get("tags"),
            is_multi_stage: row.get("is_multi_stage"),
            stage_count: row.get("stage_count"),
            creator_id: row.get("creator_id"),
            is_public: row.get("is_public"),
            version: row.get("version"),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    })
    .transpose()
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
    let mut sql = sqlx::QueryBuilder::new(
        r#"
        SELECT p.*,
               m.id as marketplace_id,
               m.creator_name,
               m.visibility,
               m.download_count,
               m.rating_average,
               m.rating_count,
               m.featured,
               m.published_at
        FROM plugins p
        LEFT JOIN plugin_marketplace_entries m ON p.id = m.plugin_id
        WHERE 1=1
        "#,
    );

    if let Some(q) = query {
        let search_term = format!("%{}%", q);
        sql.push(" AND (p.name LIKE ")
            .push_bind(search_term.clone())
            .push(" OR p.description LIKE ")
            .push_bind(search_term)
            .push(")");
    }

    if let Some(cat) = category {
        sql.push(" AND p.category = ").push_bind(cat.to_string());
    }

    match sort_by {
        PluginSortOrder::Name => sql.push(" ORDER BY p.name ASC"),
        PluginSortOrder::Downloads => sql.push(" ORDER BY download_count DESC"),
        PluginSortOrder::Rating => sql.push(" ORDER BY rating_average DESC"),
        PluginSortOrder::Recent => sql.push(" ORDER BY p.created_at DESC"),
        PluginSortOrder::Relevance => sql.push(" ORDER BY rating_average DESC"), // Default to rating for relevance
    };

    sql.push(" LIMIT ").push_bind(limit);
    sql.push(" OFFSET ").push_bind(offset);

    let results = sql.build().fetch_all(pool).await?;

    let plugins = results
        .into_iter()
        .map(|row| {
            let plugin = Plugin {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                version: row.get("version"),
                creator_id: row.get("creator_id"),
                category: row
                    .get::<'_, String, _>("category")
                    .parse()
                    .unwrap_or(PluginCategory::Other),
                is_public: row.get("is_public"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                prompt_template: row.get("prompt_template"),
                variables: row.get("variables"),
                ai_model: row.get("ai_model"),
                temperature: row.get("temperature"),
                max_tokens: row.get("max_tokens"),
                stop_sequences: row.get("stop_sequences"),
                tags: row.get("tags"),
                is_multi_stage: row.get("is_multi_stage"),
                stage_count: row.get("stage_count"),
            };

            let marketplace_entry = PluginMarketplaceEntry {
                id: row.get("marketplace_id"),
                plugin_id: plugin.id as i32,
                creator_name: row.get("creator_name"),
                visibility: row
                    .get::<'_, String, _>("visibility")
                    .parse()
                    .unwrap_or(PluginVisibility::Private),
                download_count: row.get("download_count"),
                rating_average: row.get("rating_average"),
                rating_count: row.get("rating_count"),
                featured: row.get("featured"),
                published_at: row.get("published_at"),
                updated_at: plugin.updated_at,
            };

            PluginSearchResult {
                plugin,
                marketplace_entry,
                relevance_score: 1.0, // Default relevance
            }
        })
        .collect();

    Ok(plugins)
}

/// Update plugin download count
pub async fn increment_plugin_downloads(
    pool: &SqlitePool,
    plugin_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE plugin_marketplace_entries SET download_count = download_count + 1 WHERE plugin_id = ?",
        plugin_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get plugin execution history
pub async fn get_plugin_execution_history(
    pool: &SqlitePool,
    plugin_id: Option<i32>,
    user_identifier: Option<&str>,
    limit: i32,
    offset: i32,
) -> Result<Vec<PluginExecutionHistory>, sqlx::Error> {
    match (plugin_id, user_identifier) {
        (Some(pid), Some(uid)) => {
            sqlx::query_as(
                r#"
                SELECT id, plugin_id, user_identifier, execution_request, execution_result,
                       credits_used, execution_time_ms, success, error_message, created_at
                FROM plugin_execution_history
                WHERE plugin_id = ? AND user_identifier = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(pid)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
        }
        (Some(pid), None) => {
            sqlx::query_as(
                r#"
                SELECT id, plugin_id, user_identifier, execution_request, execution_result,
                       credits_used, execution_time_ms, success, error_message, created_at
                FROM plugin_execution_history
                WHERE plugin_id = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(pid)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
        }
        (None, Some(uid)) => {
            sqlx::query_as(
                r#"
                SELECT id, plugin_id, user_identifier, execution_request, execution_result,
                       credits_used, execution_time_ms, success, error_message, created_at
                FROM plugin_execution_history
                WHERE user_identifier = ?
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
        }
        (None, None) => {
            sqlx::query_as(
                r#"
                SELECT id, plugin_id, user_identifier, execution_request, execution_result,
                       credits_used, execution_time_ms, success, error_message, created_at
                FROM plugin_execution_history
                ORDER BY created_at DESC
                LIMIT ? OFFSET ?
                "#,
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
        }
    }
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
    let naive_now = now.naive_utc();

    // Insert or update rating
    let result = sqlx::query!(
        r#"
        INSERT INTO plugin_ratings (plugin_id, user_identifier, rating, review, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?)
        ON CONFLICT(plugin_id, user_identifier) DO UPDATE SET
            rating = excluded.rating,
            review = excluded.review,
            updated_at = excluded.updated_at
        "#,
        plugin_id,
        user_identifier,
        rating,
        review,
        naive_now,
        naive_now,
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    // Update plugin's average rating
    update_plugin_rating_average(pool, plugin_id).await?;

    Ok(PluginRating {
        id,
        plugin_id,
        user_identifier: user_identifier.to_string(),
        rating,
        review: review.map(|s| s.to_string()),
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

    let avg_rating: f32 = result.avg_rating.unwrap_or(0.0) as f32;
    let count: i32 = result.count.unwrap_or(0) as i32;

    sqlx::query!(
        "UPDATE plugin_marketplace_entries SET rating_average = ?, rating_count = ? WHERE plugin_id = ?",
        avg_rating,
        count,
        plugin_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get plugin ratings
pub async fn get_plugin_ratings(
    pool: &SqlitePool,
    plugin_id: i32,
    limit: i32,
    offset: i32,
) -> Result<Vec<PluginRating>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, plugin_id, user_identifier, rating, review, created_at
        FROM plugin_ratings
        WHERE plugin_id = ?
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(plugin_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| {
        Ok(PluginRating {
            id: row.get("id"),
            plugin_id: row.get("plugin_id"),
            user_identifier: row.get("user_identifier"),
            rating: row.get("rating"),
            review: row.get("review"),
            created_at: row.get("created_at"),
        })
    })
    .collect()
}

/// Record plugin execution
pub async fn record_plugin_execution(
    pool: &SqlitePool,
    request: PluginExecutionRequest,
    result: PluginExecutionResult,
) -> Result<PluginExecutionResult, sqlx::Error> {
    let execution_id = Uuid::new_v4().to_string();
    let now = Utc::now();

    let request_json = serde_json::to_string(&request).unwrap_or_default();
    let result_json = serde_json::to_string(&result).unwrap_or_default();

    sqlx::query!(
        r#"
        INSERT INTO plugin_execution_history (
            id, plugin_id, user_identifier, execution_request, execution_result,
            credits_used, execution_time_ms, success, error_message, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        execution_id,
        request.plugin_id,
        "unknown_user", // user_identifier not available in current struct
        request_json,
        result_json,
        result.credits_used,
        result.execution_time_ms,
        result.success,
        result.error_message,
        now.naive_utc()
    )
    .execute(pool)
    .await?;

    // Update usage stats
    update_plugin_usage_stats(pool, request.plugin_id, result.success).await?;

    Ok(result)
}

/// Update plugin usage statistics
pub async fn update_plugin_usage_stats(
    pool: &SqlitePool,
    plugin_id: i32,
    success: bool,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let today = now.date_naive();
    let successful_increment = if success { 1 } else { 0 };
    let failed_increment = if success { 0 } else { 1 };

    sqlx::query!(
        r#"
        INSERT INTO plugin_daily_stats (plugin_id, date, total_executions, successful_executions, failed_executions, created_at, updated_at)
        VALUES (?, ?, 1, ?, ?, ?, ?)
        ON CONFLICT(plugin_id, date) DO UPDATE SET
            total_executions = total_executions + 1,
            successful_executions = successful_executions + excluded.successful_executions,
            failed_executions = failed_executions + excluded.failed_executions,
            updated_at = excluded.updated_at
        "#,
        plugin_id,
        today,
        successful_increment,
        failed_increment,
        now.naive_utc(),
        now.naive_utc()
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get plugin usage statistics
pub async fn get_plugin_daily_stats(
    pool: &SqlitePool,
    plugin_id: i32,
    days: i32,
) -> Result<Vec<PluginDailyStats>, sqlx::Error> {
    let start_date = Utc::now().date_naive() - chrono::Duration::days(days as i64);

    sqlx::query_as(
        r#"
        SELECT id, plugin_id, date, total_executions, successful_executions,
               failed_executions, created_at, updated_at
        FROM plugin_daily_stats
        WHERE plugin_id = ? AND date >= ?
        ORDER BY date DESC
        "#,
    )
    .bind(plugin_id)
    .bind(start_date)
    .fetch_all(pool)
    .await
}

/// Get plugin templates
pub async fn get_plugin_templates(
    pool: &SqlitePool,
    category: Option<PluginCategory>,
) -> Result<Vec<PluginTemplate>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::new(
        r#"
        SELECT id, name, description, category, template_data, is_official, created_at
        FROM plugin_templates
        "#,
    );

    if let Some(cat) = category {
        query.push(" WHERE category = ");
        query.push_bind(cat.to_string());
    }

    query.push(" ORDER BY name ASC");

    query
        .build()
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| {
            Ok(PluginTemplate {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                category: row
                    .get::<'_, String, _>("category")
                    .parse()
                    .unwrap_or_default(),
                template_data: row.get("template_data"),
                is_official: row.get("is_official"),
                created_at: row.get("created_at"),
            })
        })
        .collect()
}


/// Get plugin template by ID
pub async fn get_plugin_template_by_id(
    pool: &SqlitePool,
    template_id: i32,
) -> Result<Option<PluginTemplate>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, name, description, category, template_data, is_official, created_at
        FROM plugin_templates
        WHERE id = ?
        "#,
    )
    .bind(template_id)
    .fetch_optional(pool)
    .await?
    .map(|row| {
        Ok(PluginTemplate {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            category: row
                .get::<'_, String, _>("category")
                .parse()
                .unwrap_or_default(),
            template_data: row.get("template_data"),
            is_official: row.get("is_official"),
            created_at: row.get("created_at"),
        })
    })
    .transpose()
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
    let now = Utc::now();
    let category_str = category.to_string();

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
        category_str,
        template_code,
        false,
        now.naive_utc()
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
    let now = Utc::now();
    let mut transaction = pool.begin().await?;
    
    if name.is_some() || description.is_some() || version.is_some() || code.is_some() || category.is_some() {
        let mut update_plugin = sqlx::QueryBuilder::new("UPDATE plugins SET ");
        let mut separated = update_plugin.separated(", ");
    
        if let Some(n) = name {
            separated.push("name = ");
            separated.push_bind_unseparated(n);
        }
        if let Some(d) = description {
            separated.push("description = ");
            separated.push_bind_unseparated(d);
        }
        if let Some(v) = version {
            separated.push("version = ");
            separated.push_bind_unseparated(v);
        }
        if let Some(cat) = category {
            separated.push("category = ");
            separated.push_bind_unseparated(cat.to_string());
        }
    
        separated.push("updated_at = ");
        separated.push_bind_unseparated(now.naive_utc());
        update_plugin.push(" WHERE id = ");
        update_plugin.push_bind(plugin_id);
    
        update_plugin.build().execute(&mut *transaction).await?;
    }

    if visibility.is_some() {
        let mut update_marketplace = sqlx::QueryBuilder::new("UPDATE plugin_marketplace_entries SET ");
        let mut separated = update_marketplace.separated(", ");

        if let Some(vis) = visibility {
            separated.push("visibility = ");
            separated.push_bind_unseparated(vis.to_string());
        }
        
        separated.push("updated_at = ");
        separated.push_bind_unseparated(now.naive_utc());
        update_marketplace.push(" WHERE plugin_id = ");
        update_marketplace.push_bind(plugin_id);

        update_marketplace.build().execute(&mut *transaction).await?;
    }

    transaction.commit().await?;

    Ok(())
}

/// Delete plugin
pub async fn delete_plugin(
    pool: &SqlitePool,
    plugin_id: &str,
) -> Result<(), sqlx::Error> {
    // This should be a real delete, not a soft delete.
    sqlx::query!("DELETE FROM plugins WHERE id = ?", plugin_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Get plugins with optional filtering
pub async fn get_plugins(
    pool: &SqlitePool,
    category: Option<PluginCategory>,
    limit: i32,
    offset: i32,
) -> Result<Vec<Plugin>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::new(
        r#"
        SELECT id, name, description, prompt_template, variables, ai_model,
               temperature, max_tokens, stop_sequences, category, tags,
               is_multi_stage, stage_count, creator_id, is_public, version,
               created_at, updated_at
        FROM plugins
        WHERE 1=1
        "#,
    );

    if let Some(cat) = category {
        query.push(" AND category = ");
        query.push_bind(cat.to_string());
    }

    query.push(" ORDER BY created_at DESC LIMIT ");
    query.push_bind(limit);
    query.push(" OFFSET ");
    query.push_bind(offset);

    query.build().fetch_all(pool).await?
        .into_iter()
        .map(|row| Ok(Plugin::from_row(&row)?))
        .collect()
}

/// Get user's plugins
pub async fn get_user_plugins(
    pool: &SqlitePool,
    creator_id: &str,
) -> Result<Vec<Plugin>, sqlx::Error> {
    sqlx::query(
        r#"
        SELECT id, name, description, prompt_template, variables, ai_model,
               temperature, max_tokens, stop_sequences, category, tags,
               is_multi_stage, stage_count, creator_id, is_public, version,
               created_at, updated_at
        FROM plugins
        WHERE creator_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(creator_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| Ok(Plugin::from_row(&row)?))
    .collect()
}
