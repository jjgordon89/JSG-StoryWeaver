//! Background processing commands for StoryWeaver
//! Provides commands for managing background tasks

use crate::background::{Task, TaskPriority, TaskStatus, TaskType, BackgroundTaskManager};
use crate::database::{get_pool, operations::BackgroundTaskOps};
use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::str::FromStr;

/// Create a new background task
#[tauri::command]
pub async fn create_background_task(
    task_type: String,
    description: String,
    priority: u8,
    user_initiated: bool,
    project_id: Option<String>,
    document_id: Option<String>,
    metadata: Option<serde_json::Value>,
    task_manager: State<'_, BackgroundTaskManager>,
) -> Result<String> {
    let task_type_enum = match task_type.as_str() {
        "ai_generation" => TaskType::AIGeneration,
        "database_operation" => TaskType::DatabaseOperation,
        "file_operation" => TaskType::FileOperation,
        "export" => TaskType::Export,
        "import" => TaskType::Import,
        "backup" => TaskType::Backup,
        _ => TaskType::Other(task_type),
    };
    
    let priority_enum = match priority {
        0 => TaskPriority::Low,
        1 => TaskPriority::Normal,
        2 => TaskPriority::High,
        3 => TaskPriority::Critical,
        _ => TaskPriority::Normal,
    };
    
    let task = Task::new(
        task_type_enum,
        description,
        priority_enum,
        user_initiated,
        project_id,
        document_id,
        metadata,
    );

    let task_id = task_manager.enqueue_task(task).await?;
    Ok(task_id)
}

/// Get a task by ID
#[tauri::command]
pub async fn get_background_task(task_id: String) -> Result<TaskResponse> {
    let pool = get_pool()?;
    let task = BackgroundTaskOps::get_task(&pool, &task_id).await?;
    Ok(TaskResponse::from(task))
}

/// Get all tasks
#[tauri::command]
pub async fn get_all_background_tasks(
    status: Option<String>,
    _limit: Option<i32>,
    _offset: Option<i32>,
) -> Result<Vec<TaskResponse>> {
    let pool = get_pool()?;

    let tasks = if let Some(s) = status {
        let status_enum = TaskStatus::from_str(&s)
            .map_err(|e| StoryWeaverError::invalid_input(e.to_string()))?;
        BackgroundTaskOps::get_tasks_by_status(&pool, status_enum).await?
    } else {
        BackgroundTaskOps::get_all_tasks(&pool).await?
    };

    Ok(tasks.into_iter().map(TaskResponse::from).collect())
}

/// Cancel a task
#[tauri::command]
pub async fn cancel_background_task(
    task_id: String,
    task_manager: State<'_, BackgroundTaskManager>,
) -> Result<()> {
    task_manager.cancel_task(&task_id).await
}

#[tauri::command]
pub async fn cleanup_old_background_tasks(days: i64) -> Result<usize> {
    let pool = get_pool()?;
    BackgroundTaskOps::cleanup_old_tasks(&pool, days).await
}

/// Task response for frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResponse {
    pub id: String,
    pub task_type: String,
    pub description: String,
    pub status: String,
    pub priority: u8,
    pub progress: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub user_initiated: bool,
    pub project_id: Option<String>,
    pub document_id: Option<String>,
    pub metadata: serde_json::Value,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            task_type: task.task_type.to_string(),
            description: task.description,
            status: task.status.to_string(),
            priority: task.priority as u8,
            progress: task.progress,
            created_at: task.created_at,
            started_at: task.started_at,
            completed_at: task.completed_at,
            error_message: task.error_message,
            user_initiated: task.user_initiated,
            project_id: task.project_id,
            document_id: task.document_id,
            metadata: task.metadata,
        }
    }
}
