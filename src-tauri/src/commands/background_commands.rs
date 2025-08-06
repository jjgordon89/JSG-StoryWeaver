//! Background processing commands for StoryWeaver
//! Provides commands for managing background tasks

use crate::background::{Task, TaskPriority, TaskStatus, TaskType, BackgroundTaskManager};
use crate::database::{get_pool, operations::BackgroundTaskOps};
use crate::error::{Result, StoryWeaverError};
use crate::commands::CommandResponse;
use serde::{Deserialize, Serialize};
use tauri::State;

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
) -> CommandResponse<String> {
    async fn create(
        task_type: String,
        description: String,
        priority: u8,
        user_initiated: bool,
        project_id: Option<String>,
        document_id: Option<String>,
        metadata: Option<serde_json::Value>,
        task_manager: &BackgroundTaskManager,
    ) -> Result<String> {
        // Convert task type string to enum
        let task_type_enum = match task_type.as_str() {
            "ai_generation" => TaskType::AIGeneration,
            "database_operation" => TaskType::DatabaseOperation,
            "file_operation" => TaskType::FileOperation,
            "export" => TaskType::Export,
            "import" => TaskType::Import,
            "backup" => TaskType::Backup,
            _ => TaskType::Other(task_type),
        };
        
        // Convert priority to enum
        let priority_enum = match priority {
            0 => TaskPriority::Low,
            1 => TaskPriority::Normal,
            2 => TaskPriority::High,
            3 => TaskPriority::Critical,
            _ => TaskPriority::Normal,
        };
        
        // Create task
        let task = Task::new(
            task_type_enum,
            description,
            priority_enum,
            user_initiated,
            project_id,
            document_id,
            metadata,
        );
        
        // Enqueue task
        let task_id = task_manager.enqueue_task(task).await?;
        
        Ok(task_id)
    }
    
    create(
        task_type,
        description,
        priority,
        user_initiated,
        project_id,
        document_id,
        metadata,
        &task_manager,
    )
    .await
    .into()
}

/// Get a task by ID
#[tauri::command]
pub async fn get_background_task(task_id: String) -> CommandResponse<TaskResponse> {
    async fn get(task_id: String) -> Result<TaskResponse> {
        let pool = get_pool()?;
        let task = BackgroundTaskOps::get_task(pool, &task_id).await?;
        Ok(TaskResponse::from(task))
    }
    
    get(task_id).await.into()
}

/// Get all tasks
#[tauri::command]
pub async fn get_all_background_tasks() -> CommandResponse<Vec<TaskResponse>> {
    async fn get_all() -> Result<Vec<TaskResponse>> {
        let pool = get_pool()?;
        let tasks = BackgroundTaskOps::get_all_tasks(pool).await?;
        Ok(tasks.into_iter().map(TaskResponse::from).collect())
    }
    
    get_all().await.into()
}

/// Get tasks by status
#[tauri::command]
pub async fn get_background_tasks_by_status(status: String) -> CommandResponse<Vec<TaskResponse>> {
    async fn get_by_status(status: String) -> Result<Vec<TaskResponse>> {
        let pool = get_pool()?;
        
        // Convert status string to enum
        let status_enum = match status.as_str() {
            "queued" => TaskStatus::Queued,
            "running" => TaskStatus::Running,
            "completed" => TaskStatus::Completed,
            "failed" => TaskStatus::Failed,
            "cancelled" => TaskStatus::Cancelled,
            _ => return Err(StoryWeaverError::internal(format!("Invalid task status: {}", status))),
        };
        
        let tasks = BackgroundTaskOps::get_tasks_by_status(pool, status_enum).await?;
        Ok(tasks.into_iter().map(TaskResponse::from).collect())
    }
    
    get_by_status(status).await.into()
}

/// Get tasks by project ID
#[tauri::command]
pub async fn get_background_tasks_by_project(project_id: String) -> CommandResponse<Vec<TaskResponse>> {
    async fn get_by_project(project_id: String) -> Result<Vec<TaskResponse>> {
        let pool = get_pool()?;
        let tasks = BackgroundTaskOps::get_tasks_by_project(pool, &project_id).await?;
        Ok(tasks.into_iter().map(TaskResponse::from).collect())
    }
    
    get_by_project(project_id).await.into()
}

/// Get tasks by document ID
#[tauri::command]
pub async fn get_background_tasks_by_document(document_id: String) -> CommandResponse<Vec<TaskResponse>> {
    async fn get_by_document(document_id: String) -> Result<Vec<TaskResponse>> {
        let pool = get_pool()?;
        let tasks = BackgroundTaskOps::get_tasks_by_document(pool, &document_id).await?;
        Ok(tasks.into_iter().map(TaskResponse::from).collect())
    }
    
    get_by_document(document_id).await.into()
}

/// Cancel a task
#[tauri::command]
pub async fn cancel_background_task(
    task_id: String,
    task_manager: State<'_, BackgroundTaskManager>,
) -> CommandResponse<bool> {
    async fn cancel(task_id: String, task_manager: &BackgroundTaskManager) -> Result<bool> {
        task_manager.cancel_task(&task_id).await?;
        Ok(true)
    }
    
    cancel(task_id, &task_manager).await.into()
}

/// Clean up old tasks
#[tauri::command]
pub async fn cleanup_old_background_tasks(days: i64) -> CommandResponse<usize> {
    async fn cleanup(days: i64) -> Result<usize> {
        let pool = get_pool()?;
        let count = BackgroundTaskOps::cleanup_old_tasks(pool, days).await?;
        Ok(count)
    }
    
    cleanup(days).await.into()
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
        // Convert task type to string
        let task_type_str = match &task.task_type {
            TaskType::AIGeneration => "ai_generation".to_string(),
            TaskType::DatabaseOperation => "database_operation".to_string(),
            TaskType::FileOperation => "file_operation".to_string(),
            TaskType::Export => "export".to_string(),
            TaskType::Import => "import".to_string(),
            TaskType::Backup => "backup".to_string(),
            TaskType::Other(name) => name.clone(),
        };
        
        // Convert status to string
        let status_str = match task.status {
            TaskStatus::Queued => "queued".to_string(),
            TaskStatus::Running => "running".to_string(),
            TaskStatus::Completed => "completed".to_string(),
            TaskStatus::Failed => "failed".to_string(),
            TaskStatus::Cancelled => "cancelled".to_string(),
        };
        
        // Convert priority to u8
        let priority_u8 = match task.priority {
            TaskPriority::Low => 0,
            TaskPriority::Normal => 1,
            TaskPriority::High => 2,
            TaskPriority::Critical => 3,
        };
        
        Self {
            id: task.id,
            task_type: task_type_str,
            description: task.description,
            status: status_str,
            priority: priority_u8,
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
