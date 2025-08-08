//! Background task database operations
//! Provides functions to interact with the background_tasks table

use crate::background::{Task, TaskPriority, TaskStatus, TaskType};
use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Background task database operations
pub struct BackgroundTaskOps;

impl BackgroundTaskOps {
    /// Save a task to the database
    pub async fn save_task(pool: &Pool<Sqlite>, task: &Task) -> Result<()> {
        // Convert task status to string
        let status_str = match task.status {
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        };
        
        // Convert task type to string
        let task_type_str = match &task.task_type {
            TaskType::AIGeneration => "ai_generation",
            TaskType::DatabaseOperation => "database_operation",
            TaskType::FileOperation => "file_operation",
            TaskType::Export => "export",
            TaskType::Import => "import",
            TaskType::Backup => "backup",
            TaskType::Other(name) => name,
        };
        
        // Serialize metadata to JSON
        let metadata_json = serde_json::to_string(&task.metadata)
            .map_err(|e| StoryWeaverError::Serialization { message: e.to_string() })?;
        
        // Insert or update task
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO background_tasks (
                id, task_type, description, status, priority, progress,
                created_at, started_at, completed_at, error_message,
                user_initiated, project_id, document_id, metadata
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&task.id)
        .bind(task_type_str)
        .bind(&task.description)
        .bind(status_str)
        .bind(task.priority as i32)
        .bind(task.progress)
        .bind(task.created_at)
        .bind(task.started_at)
        .bind(task.completed_at)
        .bind(&task.error_message)
        .bind(task.user_initiated)
        .bind(&task.project_id)
        .bind(&task.document_id)
        .bind(metadata_json)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to save task: {}", e)))?;
        
        Ok(())
    }
    
    /// Get a task by ID
    pub async fn get_task(pool: &Pool<Sqlite>, task_id: &str) -> Result<Task> {
        let record = sqlx::query!(
            r#"
            SELECT * FROM background_tasks WHERE id = ?
            "#,
            task_id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get task: {}", e)))?
        .ok_or_else(|| StoryWeaverError::internal(format!("Task not found: {}", task_id)))?;
        
        Self::record_to_task(record)
    }
    
    /// Get all tasks
    pub async fn get_all_tasks(pool: &Pool<Sqlite>) -> Result<Vec<Task>> {
        let records = sqlx::query!(
            r#"
            SELECT * FROM background_tasks ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get tasks: {}", e)))?;
        
        let mut tasks = Vec::with_capacity(records.len());
        for record in records {
            tasks.push(Self::record_to_task(record)?);
        }
        
        Ok(tasks)
    }
    
    /// Get tasks by status
    pub async fn get_tasks_by_status(pool: &Pool<Sqlite>, status: TaskStatus) -> Result<Vec<Task>> {
        let status_str = match status {
            TaskStatus::Queued => "queued",
            TaskStatus::Running => "running",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        };
        
        let records = sqlx::query!(
            r#"
            SELECT * FROM background_tasks WHERE status = ? ORDER BY created_at DESC
            "#,
            status_str
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get tasks by status: {}", e)))?;
        
        let mut tasks = Vec::with_capacity(records.len());
        for record in records {
            tasks.push(Self::record_to_task(record)?);
        }
        
        Ok(tasks)
    }
    
    /// Get tasks by project ID
    pub async fn get_tasks_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Task>> {
        let records = sqlx::query!(
            r#"
            SELECT * FROM background_tasks WHERE project_id = ? ORDER BY created_at DESC
            "#,
            project_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get tasks by project: {}", e)))?;
        
        let mut tasks = Vec::with_capacity(records.len());
        for record in records {
            tasks.push(Self::record_to_task(record)?);
        }
        
        Ok(tasks)
    }
    
    /// Get tasks by document ID
    pub async fn get_tasks_by_document(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<Task>> {
        let records = sqlx::query!(
            r#"
            SELECT * FROM background_tasks WHERE document_id = ? ORDER BY created_at DESC
            "#,
            document_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get tasks by document: {}", e)))?;
        
        let mut tasks = Vec::with_capacity(records.len());
        for record in records {
            tasks.push(Self::record_to_task(record)?);
        }
        
        Ok(tasks)
    }
    
    /// Delete a task
    pub async fn delete_task(pool: &Pool<Sqlite>, task_id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM background_tasks WHERE id = ?
            "#,
            task_id
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete task: {}", e)))?;
        
        Ok(())
    }
    
    /// Clean up old completed tasks
    pub async fn cleanup_old_tasks(pool: &Pool<Sqlite>, days: i64) -> Result<usize> {
        let result = sqlx::query!(
            r#"
            DELETE FROM background_tasks 
            WHERE status IN ('completed', 'failed', 'cancelled') 
            AND completed_at < datetime('now', ?)
            "#,
            format!("-{} days", days)
        )
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to clean up old tasks: {}", e)))?;
        
        Ok(result.rows_affected() as usize)
    }
    
    /// Convert a database record to a Task
    fn record_to_task(record: sqlx::sqlite::SqliteRow) -> Result<Task> {
        // Parse task type
        let task_type = match record.get::<&str, _>("task_type") {
            "ai_generation" => TaskType::AIGeneration,
            "database_operation" => TaskType::DatabaseOperation,
            "file_operation" => TaskType::FileOperation,
            "export" => TaskType::Export,
            "import" => TaskType::Import,
            "backup" => TaskType::Backup,
            other => TaskType::Other(other.to_string()),
        };
        
        // Parse task status
        let status = match record.get::<&str, _>("status") {
            "queued" => TaskStatus::Queued,
            "running" => TaskStatus::Running,
            "completed" => TaskStatus::Completed,
            "failed" => TaskStatus::Failed,
            "cancelled" => TaskStatus::Cancelled,
            _ => TaskStatus::Queued, // Default to queued if unknown
        };
        
        // Parse priority
        let priority_int: i32 = record.get("priority");
        let priority = match priority_int {
            0 => TaskPriority::Low,
            1 => TaskPriority::Normal,
            2 => TaskPriority::High,
            3 => TaskPriority::Critical,
            _ => TaskPriority::Normal, // Default to normal if unknown
        };
        
        // Parse metadata
        let metadata_str: &str = record.get("metadata");
        let metadata = serde_json::from_str(metadata_str)
            .map_err(|e| StoryWeaverError::Deserialization { message: format!("Failed to parse task metadata: {}", e) })?;
        
        // Parse dates
        let created_at: DateTime<Utc> = record.get("created_at");
        let started_at: Option<DateTime<Utc>> = record.get("started_at");
        let completed_at: Option<DateTime<Utc>> = record.get("completed_at");
        
        // Create task
        Ok(Task {
            id: record.get("id"),
            task_type,
            description: record.get("description"),
            status,
            priority,
            progress: record.get("progress"),
            created_at,
            started_at,
            completed_at,
            error_message: record.get("error_message"),
            user_initiated: record.get("user_initiated"),
            project_id: record.get("project_id"),
            document_id: record.get("document_id"),
            metadata,
        })
    }
}
