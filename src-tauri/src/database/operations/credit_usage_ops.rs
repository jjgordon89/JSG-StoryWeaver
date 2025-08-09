//! Credit Usage database operations
//! Provides functions to interact with the credit_usage table

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditUsage {
    pub id: Option<i32>,
    pub project_id: i32,
    pub operation_type: String, // "text_generation", "image_generation", "brainstorming", "style_analysis"
    pub model_used: String,
    pub tokens_used: Option<i32>,
    pub credits_consumed: f64,
    pub operation_details: Option<String>, // JSON
    pub session_id: Option<String>,
    pub created_at: Option<String>,
}

/// Credit Usage database operations
impl super::CreditUsageOps {
    /// Create a new credit usage record
    pub async fn create(pool: &Pool<Sqlite>, usage: &CreditUsage) -> Result<i64> {
        let result = sqlx::query!(
            r#"
            INSERT INTO credit_usage (
                project_id, operation_type, model_used, tokens_used, credits_consumed,
                operation_details, session_id
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            usage.project_id,
            usage.operation_type,
            usage.model_used,
            usage.tokens_used,
            usage.credits_consumed,
            usage.operation_details,
            usage.session_id
        )
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create credit usage record: {}", e)))?;

        Ok(result.last_insert_rowid())
    }

    /// Get a credit usage record by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: i32) -> Result<Option<CreditUsage>> {
        let row = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage record: {}", e)))?;

        Ok(row.map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed.unwrap_or(0.0),
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }))
    }

    /// Get credit usage records by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<Vec<CreditUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage WHERE project_id = ? ORDER BY created_at DESC
            "#,
            project_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage by project: {}", e)))?;

        Ok(rows.into_iter().map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed.unwrap_or(0.0),
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get credit usage records by operation type
    pub async fn get_by_operation_type(pool: &Pool<Sqlite>, operation_type: &str) -> Result<Vec<CreditUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage WHERE operation_type = ? ORDER BY created_at DESC
            "#,
            operation_type
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage by operation type: {}", e)))?;

        Ok(rows.into_iter().map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed.unwrap_or(0.0),
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get credit usage records by session ID
    pub async fn get_by_session(pool: &Pool<Sqlite>, session_id: &str) -> Result<Vec<CreditUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage WHERE session_id = ? ORDER BY created_at DESC
            "#,
            session_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage by session: {}", e)))?;

        Ok(rows.into_iter().map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed.unwrap_or(0.0),
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// List all credit usage records
    pub async fn list_all(pool: &Pool<Sqlite>) -> Result<Vec<CreditUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage ORDER BY created_at DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to list credit usage records: {}", e)))?;

        Ok(rows.into_iter().map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed.unwrap_or(0.0),
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Delete a credit usage record
    pub async fn delete(pool: &Pool<Sqlite>, id: i32) -> Result<()> {
        sqlx::query!("DELETE FROM credit_usage WHERE id = ?", id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete credit usage record: {}", e)))?;

        Ok(())
    }

    /// Get total credits consumed by project
    pub async fn get_total_by_project(pool: &Pool<Sqlite>, project_id: i32) -> Result<f64> {
        let row = sqlx::query!(
            "SELECT COALESCE(SUM(credits_consumed), 0.0) as total_credits! FROM credit_usage WHERE project_id = ?",
            project_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total credits for project: {}", e)))?;

        Ok(row.total_credits)
    }

    /// Get total credits consumed by operation type
    pub async fn get_total_by_operation_type(pool: &Pool<Sqlite>, operation_type: &str) -> Result<f64> {
        let row = sqlx::query!(
            "SELECT COALESCE(SUM(credits_consumed), 0.0) as total_credits FROM credit_usage WHERE operation_type = ?",
            operation_type
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get total credits for operation type: {}", e)))?;

        Ok(row.total_credits)
    }

    /// Get daily credit usage for a project
    pub async fn get_daily_usage(pool: &Pool<Sqlite>, project_id: i32, date: &str) -> Result<f64> {
        let row = sqlx::query!(
            r#"
            SELECT COALESCE(SUM(credits_consumed), 0.0) as daily_credits! 
            FROM credit_usage 
            WHERE project_id = ? AND DATE(created_at) = ?
            "#,
            project_id,
            date
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get daily credit usage: {}", e)))?;

        Ok(row.daily_credits)
    }

    /// Get credit usage within date range
    pub async fn get_usage_in_range(pool: &Pool<Sqlite>, project_id: i32, start_date: &str, end_date: &str) -> Result<Vec<CreditUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage 
            WHERE project_id = ? AND DATE(created_at) BETWEEN ? AND ?
            ORDER BY created_at DESC
            "#,
            project_id,
            start_date,
            end_date
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage in range: {}", e)))?;

        Ok(rows.into_iter().map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get recent credit usage records (last N)
    pub async fn get_recent(pool: &Pool<Sqlite>, limit: i32) -> Result<Vec<CreditUsage>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, project_id!, operation_type!, model_used, tokens_used, credits_consumed,
                   operation_details, session_id, created_at
            FROM credit_usage ORDER BY created_at DESC LIMIT ?
            "#,
            limit
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get recent credit usage: {}", e)))?;

        Ok(rows.into_iter().map(|r| CreditUsage {
            id: r.id.map(|id| id as i32),
            project_id: r.project_id as i32,
            operation_type: r.operation_type,
            model_used: r.model_used.unwrap_or_default(),
            tokens_used: r.tokens_used.map(|t| t as i32),
            credits_consumed: r.credits_consumed,
            operation_details: r.operation_details,
            session_id: r.session_id,
            created_at: r.created_at.map(|dt| dt.to_string()),
        }).collect())
    }

    /// Get credit usage statistics by model
    pub async fn get_stats_by_model(pool: &Pool<Sqlite>) -> Result<Vec<(String, f64, i32)>> {
        let rows = sqlx::query!(
            r#"
            SELECT model_used, 
                   COALESCE(SUM(credits_consumed), 0.0) as total_credits,
                   COUNT(*) as usage_count
            FROM credit_usage 
            GROUP BY model_used 
            ORDER BY total_credits DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage stats by model: {}", e)))?;

        Ok(rows.into_iter().map(|r| (r.model_used.unwrap_or_default(), r.total_credits, r.usage_count as i32)).collect())
    }

    /// Get credit usage statistics by operation type
    pub async fn get_stats_by_operation(pool: &Pool<Sqlite>) -> Result<Vec<(String, f64, i32)>> {
        let rows = sqlx::query!(
            r#"
            SELECT operation_type, 
                   COALESCE(SUM(credits_consumed), 0.0) as total_credits,
                   COUNT(*) as usage_count
            FROM credit_usage 
            GROUP BY operation_type 
            ORDER BY total_credits DESC
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get credit usage stats by operation: {}", e)))?;

        Ok(rows.into_iter().map(|r| (r.operation_type, r.total_credits, r.usage_count as i32)).collect())
    }
}