//! Tauri command handlers for StoryWeaver
//! These commands provide the interface between the frontend and backend

use crate::database::{get_pool, models::*, operations::*};
use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use tauri::State;

// Re-export command modules
pub mod projects;
pub mod documents;
pub mod characters;
pub mod locations;
pub mod story_bible;
pub mod ai_history;
pub mod ai_writing;
pub mod ai_cards;
pub mod folder_commands;
pub mod series_commands;
pub mod document_link_commands;
pub mod backup_commands;
pub mod trash_commands;
pub mod version_commands;
pub mod settings_commands;
pub mod sync_commands;
pub mod register_commands;
pub mod background_commands;
pub mod performance_commands;
pub mod security_commands;
pub mod project_preview_commands;

/// Response wrapper for Tauri commands
#[derive(Debug, Serialize)]
pub struct CommandResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

/// Convert Result to CommandResponse
impl<T> From<Result<T>> for CommandResponse<T> {
    fn from(result: Result<T>) -> Self {
        match result {
            Ok(data) => CommandResponse::success(data),
            Err(e) => CommandResponse::error(e.user_message()),
        }
    }
}

/// Health check command
#[tauri::command]
pub async fn health_check() -> CommandResponse<String> {
    match get_pool() {
        Ok(pool) => {
            match sqlx::query("SELECT 1").execute(&*pool).await {
                Ok(_) => CommandResponse::success("Database connection healthy".to_string()),
                Err(e) => CommandResponse::error(format!("Database error: {}", e)),
            }
        }
        Err(e) => CommandResponse::error(format!("Failed to get database pool: {}", e)),
    }
}

/// Get database statistics
#[tauri::command]
pub async fn get_database_stats() -> CommandResponse<DatabaseStats> {
    async fn get_stats() -> Result<DatabaseStats> {
        let pool = get_pool()?;
        
        let project_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count projects: {}", e)))? as i32;
        
        let document_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM documents")
            .fetch_one(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count documents: {}", e)))? as i32;
        
        let character_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM characters")
            .fetch_one(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count characters: {}", e)))? as i32;
        
        let location_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM locations")
            .fetch_one(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count locations: {}", e)))? as i32;
        
        Ok(DatabaseStats {
            project_count,
            document_count,
            character_count,
            location_count,
        })
    }
    
    get_stats().await.into()
}

/// Database statistics structure
#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub project_count: i32,
    pub document_count: i32,
    pub character_count: i32,
    pub location_count: i32,
}

/// Initialize database command
#[tauri::command]
pub async fn init_database() -> CommandResponse<String> {
    async fn init() -> Result<String> {
        let pool = get_pool()?;
        crate::database::migrations::run_migrations(&*pool).await?;
        Ok("Database initialized successfully".to_string())
    }
    
    init().await.into()
}

/// Test command for development
#[tauri::command]
pub async fn greet(name: &str) -> Result<String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}
