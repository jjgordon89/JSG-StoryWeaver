//! AI Task Processor for StoryWeaver
//! Handles background processing of AI generation tasks

use crate::ai::{AIContext, AIProvider, AIProviderManager};
use crate::background::{Task, TaskProcessor, TaskStatus, TaskType};
use crate::error::{Result, StoryWeaverError};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{Emitter, Manager};

/// AI Task Processor
pub struct AITaskProcessor {
    app_handle: tauri::AppHandle,
}

impl AITaskProcessor {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    /// Process an AI generation task
    async fn process_ai_generation(&self, task: &mut Task) -> Result<String> {
        // Extract task metadata
        let prompt = task.metadata.get("prompt")
            .and_then(|v| v.as_str())
            .ok_or_else(|| StoryWeaverError::internal("Missing prompt in AI generation task"))?;
        
        let provider_name = task.metadata.get("provider")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        
        // Get AI provider manager from app state
        let ai_manager = self.app_handle.state::<AIProviderManager>();
        
        // Get the specified provider or default
        let provider = if provider_name == "default" {
            ai_manager.get_default_provider()
                .ok_or_else(|| StoryWeaverError::internal("No default AI provider configured"))?
        } else {
            ai_manager.get_provider(provider_name)
                .ok_or_else(|| StoryWeaverError::internal(format!("AI provider '{}' not found", provider_name)))?
        };
        
        // Create AI context
        let context = AIContext {
            project_id: task.project_id.clone(),
            document_id: task.document_id.clone(),
            user_preferences: None,
            story_context: task.metadata.get("story_context")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            preceding_text: None,
            following_text: None,
            selected_text: None,
            characters: None,
            locations: None,
            plot_threads: None,
            writing_style: None,
            tone: None,
            creativity_level: None,
            feature_type: None,
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };
        
        // Generate text
        let result = provider.generate_text(prompt, &context).await
            .map_err(|e| StoryWeaverError::ai_provider(
                provider_name, 
                format!("AI generation failed: {}", e)
            ))?;
        
        // Store result in task metadata
        task.metadata["result"] = serde_json::Value::String(result.clone());
        
        // Emit event to notify frontend
        if let Err(e) = self.app_handle.emit(
            "ai-generation-completed",
            serde_json::json!({
                "taskId": task.id,
                "result": result,
                "projectId": task.project_id,
                "documentId": task.document_id
            })
        ) {
            eprintln!("Failed to emit ai-generation-completed event: {}", e);
        }
        
        Ok(result)
    }
}

#[async_trait::async_trait]
impl TaskProcessor for AITaskProcessor {
    async fn process_task(&self, task: Arc<Mutex<Task>>) -> Result<()> {
        let mut task_lock = task.lock().await;
        
        // Check if task is still valid
        if task_lock.status != TaskStatus::Running {
            return Err(StoryWeaverError::internal(
                format!("Task {} is not in running state", task_lock.id)
            ));
        }
        
        match task_lock.task_type {
            TaskType::AIGeneration => {
                // Update progress
                task_lock.update_progress(0.1);
                
                // Process AI generation
                let result = self.process_ai_generation(&mut task_lock).await?;
                
                // Update progress
                task_lock.update_progress(1.0);
                
                // Log success
                println!("AI generation task {} completed successfully", task_lock.id);
                println!("Generated text length: {} characters", result.len());
                
                Ok(())
            },
            _ => Err(StoryWeaverError::internal(
                format!("AI task processor cannot handle task type {:?}", task_lock.task_type)
            )),
        }
    }
    
    fn can_process(&self, task_type: &TaskType) -> bool {
        matches!(task_type, TaskType::AIGeneration)
    }
}
