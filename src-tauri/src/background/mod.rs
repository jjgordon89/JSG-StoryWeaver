//! Background Processing Foundation for StoryWeaver
//! Provides a task queue system for managing long-running operations

pub mod ai_processor;

use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{Duration, Instant};
use uuid::Uuid;

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Task status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Task type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    AIGeneration,
    DatabaseOperation,
    FileOperation,
    Export,
    Import,
    Backup,
    Other(String),
}

/// Task data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub task_type: TaskType,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub progress: f32, // 0.0 to 1.0
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
    pub user_initiated: bool,
    pub project_id: Option<String>,
    pub document_id: Option<String>,
    pub metadata: serde_json::Value,
}

impl Task {
    pub fn new(
        task_type: TaskType,
        description: String,
        priority: TaskPriority,
        user_initiated: bool,
        project_id: Option<String>,
        document_id: Option<String>,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            task_type,
            description,
            status: TaskStatus::Queued,
            priority,
            progress: 0.0,
            created_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            error_message: None,
            user_initiated,
            project_id,
            document_id,
            metadata: metadata.unwrap_or_else(|| serde_json::json!({})),
        }
    }

    pub fn mark_started(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(chrono::Utc::now());
    }

    pub fn mark_completed(&mut self) {
        self.status = TaskStatus::Completed;
        self.progress = 1.0;
        self.completed_at = Some(chrono::Utc::now());
    }

    pub fn mark_failed(&mut self, error_message: String) {
        self.status = TaskStatus::Failed;
        self.error_message = Some(error_message);
        self.completed_at = Some(chrono::Utc::now());
    }

    pub fn mark_cancelled(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.completed_at = Some(chrono::Utc::now());
    }

    pub fn update_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }
}

/// Task queue manager
pub struct TaskQueue {
    tasks: RwLock<VecDeque<Arc<Mutex<Task>>>>,
    history: RwLock<Vec<Arc<Mutex<Task>>>>,
    max_history_size: usize,
    max_concurrent_tasks: usize,
    running_tasks: RwLock<Vec<Arc<Mutex<Task>>>>,
    last_cleanup: Mutex<Instant>,
}

impl TaskQueue {
    pub fn new(max_concurrent_tasks: usize, max_history_size: usize) -> Self {
        Self {
            tasks: RwLock::new(VecDeque::new()),
            history: RwLock::new(Vec::new()),
            max_history_size,
            max_concurrent_tasks,
            running_tasks: RwLock::new(Vec::new()),
            last_cleanup: Mutex::new(Instant::now()),
        }
    }

    /// Add a task to the queue
    pub async fn enqueue_task(&self, task: Task) -> Result<String> {
        let task = Arc::new(Mutex::new(task));
        let task_id = {
            let locked_task = task.lock().await;
            locked_task.id.clone()
        };

        let mut tasks = self.tasks.write().await;
        tasks.push_back(task);

        // Sort tasks by priority (higher priority first)
        let mut tasks_vec: Vec<_> = tasks.drain(..).collect();
        tasks_vec.sort_by(|a, b| {
            let a_priority = a.try_lock().map(|t| t.priority).unwrap_or(TaskPriority::Normal);
            let b_priority = b.try_lock().map(|t| t.priority).unwrap_or(TaskPriority::Normal);
            b_priority.cmp(&a_priority)
        });

        tasks.extend(tasks_vec);
        
        Ok(task_id)
    }

    /// Get the next task to process
    pub async fn get_next_task(&self) -> Option<Arc<Mutex<Task>>> {
        let mut tasks = self.tasks.write().await;
        let running_tasks = self.running_tasks.read().await;
        
        if running_tasks.len() >= self.max_concurrent_tasks {
            return None;
        }
        
        tasks.pop_front()
    }

    /// Mark a task as running
    pub async fn mark_task_running(&self, task: Arc<Mutex<Task>>) -> Result<()> {
        let mut task_lock = task.lock().await;
        task_lock.mark_started();
        
        let mut running_tasks = self.running_tasks.write().await;
        running_tasks.push(Arc::clone(&task));
        
        Ok(())
    }

    /// Complete a task
    pub async fn complete_task(&self, task_id: &str, success: bool, error_message: Option<String>) -> Result<()> {
        let mut running_tasks = self.running_tasks.write().await;
        let task_index = running_tasks.iter().position(|t| {
            t.try_lock().map(|task| task.id == task_id).unwrap_or(false)
        });
        
        if let Some(index) = task_index {
            let task = running_tasks.remove(index);
            
            {
                let mut task_lock = task.lock().await;
                if success {
                    task_lock.mark_completed();
                } else {
                    task_lock.mark_failed(error_message.unwrap_or_else(|| "Unknown error".to_string()));
                }
            }
            
            let mut history = self.history.write().await;
            history.push(task);
            
            // Trim history if needed
            if history.len() > self.max_history_size {
                history.remove(0);
            }
            
            Ok(())
        } else {
            Err(StoryWeaverError::internal(format!("Task {} not found in running tasks", task_id)))
        }
    }

    /// Cancel a queued task
    pub async fn cancel_task(&self, task_id: &str) -> Result<()> {
        // Check queued tasks
        let mut tasks = self.tasks.write().await;
        let task_index = tasks.iter().position(|t| {
            t.try_lock().map(|task| task.id == task_id).unwrap_or(false)
        });
        
        if let Some(index) = task_index {
            let task = tasks.remove(index).unwrap();
            
            {
                let mut task_lock = task.lock().await;
                task_lock.mark_cancelled();
            }
            
            let mut history = self.history.write().await;
            history.push(task);
            
            return Ok(());
        }
        
        // Check running tasks
        let mut running_tasks = self.running_tasks.write().await;
        let task_index = running_tasks.iter().position(|t| {
            t.try_lock().map(|task| task.id == task_id).unwrap_or(false)
        });
        
        if let Some(index) = task_index {
            let task = running_tasks.remove(index);
            
            {
                let mut task_lock = task.lock().await;
                task_lock.mark_cancelled();
            }
            
            let mut history = self.history.write().await;
            history.push(task);
            
            return Ok(());
        }
        
        Err(StoryWeaverError::internal(format!("Task {} not found", task_id)))
    }

    /// Get a task by ID
    pub async fn get_task(&self, task_id: &str) -> Option<Arc<Mutex<Task>>> {
        // Check queued tasks
        {
            let tasks = self.tasks.read().await;
            for task in tasks.iter() {
                if let Ok(task_lock) = task.try_lock() {
                    if task_lock.id == task_id {
                        return Some(Arc::clone(task));
                    }
                }
            }
        }
        
        // Check running tasks
        {
            let running_tasks = self.running_tasks.read().await;
            for task in running_tasks.iter() {
                if let Ok(task_lock) = task.try_lock() {
                    if task_lock.id == task_id {
                        return Some(Arc::clone(task));
                    }
                }
            }
        }
        
        // Check history
        {
            let history = self.history.read().await;
            for task in history.iter() {
                if let Ok(task_lock) = task.try_lock() {
                    if task_lock.id == task_id {
                        return Some(Arc::clone(task));
                    }
                }
            }
        }
        
        None
    }

    /// Get all tasks (queued, running, and completed)
    pub async fn get_all_tasks(&self) -> Vec<Task> {
        let mut result = Vec::new();
        
        // Get queued tasks
        {
            let tasks = self.tasks.read().await;
            for task in tasks.iter() {
                if let Ok(task_lock) = task.try_lock() {
                    result.push(task_lock.clone());
                }
            }
        }
        
        // Get running tasks
        {
            let running_tasks = self.running_tasks.read().await;
            for task in running_tasks.iter() {
                if let Ok(task_lock) = task.try_lock() {
                    result.push(task_lock.clone());
                }
            }
        }
        
        // Get completed tasks from history
        {
            let history = self.history.read().await;
            for task in history.iter() {
                if let Ok(task_lock) = task.try_lock() {
                    result.push(task_lock.clone());
                }
            }
        }
        
        result
    }

    /// Get tasks by status
    pub async fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<Task> {
        let all_tasks = self.get_all_tasks().await;
        all_tasks.into_iter().filter(|t| t.status == status).collect()
    }

    /// Get tasks by project ID
    pub async fn get_tasks_by_project(&self, project_id: &str) -> Vec<Task> {
        let all_tasks = self.get_all_tasks().await;
        all_tasks.into_iter()
            .filter(|t| t.project_id.as_ref().map_or(false, |id| id == project_id))
            .collect()
    }

    /// Get tasks by document ID
    pub async fn get_tasks_by_document(&self, document_id: &str) -> Vec<Task> {
        let all_tasks = self.get_all_tasks().await;
        all_tasks.into_iter()
            .filter(|t| t.document_id.as_ref().map_or(false, |id| id == document_id))
            .collect()
    }

    /// Update task progress
    pub async fn update_task_progress(&self, task_id: &str, progress: f32) -> Result<()> {
        if let Some(task) = self.get_task(task_id).await {
            let mut task_lock = task.lock().await;
            task_lock.update_progress(progress);
            Ok(())
        } else {
            Err(StoryWeaverError::internal(format!("Task {} not found", task_id)))
        }
    }

    /// Clean up old completed tasks
    pub async fn cleanup_old_tasks(&self, max_age: Duration) -> Result<usize> {
        let mut last_cleanup = self.last_cleanup.lock().await;
        let now = Instant::now();
        
        // Only clean up if it's been at least an hour since the last cleanup
        if now.duration_since(*last_cleanup) < Duration::from_secs(3600) {
            return Ok(0);
        }
        
        *last_cleanup = now;
        
        let mut history = self.history.write().await;
        let now_time = chrono::Utc::now();
        let before: Vec<_> = history.clone();
        
        // Keep tasks that are either not completed/failed/cancelled or are recent enough
        history.retain(|task| {
            if let Ok(task_lock) = task.try_lock() {
                match task_lock.status {
                    TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled => {
                        if let Some(completed_at) = task_lock.completed_at {
                            let age = now_time - completed_at;
                            let age_secs = age.num_seconds() as u64;
                            return age_secs < max_age.as_secs();
                        }
                        true
                    }
                    _ => true,
                }
            } else {
                true
            }
        });
        
        let removed = before.len() - history.len();
        Ok(removed)
    }
}

/// Task processor trait
#[async_trait::async_trait]
pub trait TaskProcessor: Send + Sync {
    async fn process_task(&self, task: Arc<Mutex<Task>>) -> Result<()>;
    fn can_process(&self, task_type: &TaskType) -> bool;
}

/// Background task manager
pub struct BackgroundTaskManager {
    task_queue: Arc<TaskQueue>,
    processors: Arc<RwLock<Vec<Arc<dyn TaskProcessor>>>>,
    running: Arc<RwLock<bool>>,
}

impl BackgroundTaskManager {
    pub fn new(max_concurrent_tasks: usize, max_history_size: usize) -> Self {
        Self {
            task_queue: Arc::new(TaskQueue::new(max_concurrent_tasks, max_history_size)),
            processors: Arc::new(RwLock::new(Vec::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Register a task processor
    pub async fn register_processor(&self, processor: Arc<dyn TaskProcessor>) {
        let mut processors = self.processors.write().await;
        processors.push(processor);
    }

    /// Start the background task processing
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        
        *running = true;
        
        let task_queue = Arc::clone(&self.task_queue);
        let processors = Arc::clone(&self.processors);
        let running_flag = Arc::clone(&self.running);
        
        tokio::spawn(async move {
            while *running_flag.read().await {
                // Process next task if available
                if let Some(task) = task_queue.get_next_task().await {
                    let (task_type, task_id) = {
                        let task_lock = task.lock().await;
                        (task_lock.task_type.clone(), task_lock.id.clone())
                    };
                    
                    let processors_read = processors.read().await;
                    let mut processor_found = false;
                    
                    for processor in processors_read.iter() {
                        if processor.can_process(&task_type) {
                            processor_found = true;
                            
                            // Mark task as running
                            if let Err(e) = task_queue.mark_task_running(Arc::clone(&task)).await {
                                eprintln!("Error marking task as running: {}", e);
                                continue;
                            }
                            
                            let processor_clone = Arc::clone(processor);
                            let task_queue_clone = Arc::clone(&task_queue);
                            let task_id_clone = task_id.clone();
                            
                            // Process task in a separate task
                            tokio::spawn(async move {
                                let result = processor_clone.process_task(Arc::clone(&task)).await;
                                
                                if let Err(e) = result {
                                    if let Err(e2) = task_queue_clone.complete_task(&task_id_clone, false, Some(e.to_string())).await {
                                        eprintln!("Error completing task: {}", e2);
                                    }
                                } else {
                                    if let Err(e) = task_queue_clone.complete_task(&task_id_clone, true, None).await {
                                        eprintln!("Error completing task: {}", e);
                                    }
                                }
                            });
                            
                            break;
                        }
                    }
                    
                    if !processor_found {
                        
                        if let Err(e) = task_queue.complete_task(&task_id, false, Some("No processor found for task type".to_string())).await {
                            eprintln!("Error completing task: {}", e);
                        }
                    }
                }
                
                // Clean up old tasks periodically
                if let Err(e) = task_queue.cleanup_old_tasks(Duration::from_secs(86400 * 7)).await {
                    eprintln!("Error cleaning up old tasks: {}", e);
                }
                
                // Sleep to avoid busy waiting
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });
        
        Ok(())
    }

    /// Stop the background task processing
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        Ok(())
    }

    /// Add a task to the queue
    pub async fn enqueue_task(&self, task: Task) -> Result<String> {
        self.task_queue.enqueue_task(task).await
    }

    /// Get a task by ID
    pub async fn get_task(&self, task_id: &str) -> Option<Arc<Mutex<Task>>> {
        self.task_queue.get_task(task_id).await
    }

    /// Get all tasks
    pub async fn get_all_tasks(&self) -> Vec<Task> {
        self.task_queue.get_all_tasks().await
    }

    /// Get tasks by status
    pub async fn get_tasks_by_status(&self, status: TaskStatus) -> Vec<Task> {
        self.task_queue.get_tasks_by_status(status).await
    }

    /// Cancel a task
    pub async fn cancel_task(&self, task_id: &str) -> Result<()> {
        self.task_queue.cancel_task(task_id).await
    }

    /// Update task progress
    pub async fn update_task_progress(&self, task_id: &str, progress: f32) -> Result<()> {
        self.task_queue.update_task_progress(task_id, progress).await
    }
}
