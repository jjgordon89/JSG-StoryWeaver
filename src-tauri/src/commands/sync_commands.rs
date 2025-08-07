use crate::error::Result;
use crate::commands::CommandResponse;
use serde::{Deserialize, Serialize};
use tauri::Manager;

/// Event types for state synchronization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyncEventType {
    DocumentUpdated,
    DocumentCreated,
    DocumentDeleted,
    SettingsUpdated,
    CardUpdated,
    CardCreated,
    CardDeleted,
    ProjectUpdated,
    FolderUpdated,
    SeriesUpdated,
}

/// Generic payload for sync events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncPayload {
    pub event_type: SyncEventType,
    pub data: serde_json::Value,
}

/// Request to emit a sync event
#[derive(Debug, Deserialize)]
pub struct EmitSyncEventRequest {
    pub event_type: String,
    pub payload: serde_json::Value,
}

/// Emit a sync event from the backend to all frontend windows
#[tauri::command]
pub async fn emit_sync_event(
    window: tauri::Window,
    request: EmitSyncEventRequest,
) -> CommandResponse<()> {
    async fn emit(window: tauri::Window, request: EmitSyncEventRequest) -> Result<()> {
        // Get the app handle to emit events to all windows
        let app_handle = window.app_handle();
        
        // Emit the event to all windows
        app_handle
            .emit_to(window.label(), &request.event_type, request.payload)
            .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
        
        Ok(())
    }
    
    emit(window, request).await.into()
}

/// Helper function to emit document update events
pub fn emit_document_update(
    app_handle: &tauri::AppHandle,
    document_id: i64,
    project_id: i64,
    content: Option<&str>,
    name: Option<&str>,
) -> Result<()> {
    let mut payload = serde_json::json!({
        "documentId": document_id,
        "projectId": project_id,
    });
    
    if let Some(content) = content {
        payload["content"] = serde_json::Value::String(content.to_string());
    }
    
    if let Some(name) = name {
        payload["name"] = serde_json::Value::String(name.to_string());
    }
    
    app_handle
        .emit_to("main", "document_updated", payload)
        .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
    
    Ok(())
}

/// Helper function to emit settings update events
pub fn emit_settings_update(
    app_handle: &tauri::AppHandle,
    category: &str,
    key: &str,
    value: serde_json::Value,
) -> Result<()> {
    let payload = serde_json::json!({
        "category": category,
        "key": key,
        "value": value,
    });
    
    app_handle
        .emit_to("main", "settings_updated", payload)
        .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
    
    Ok(())
}

/// Helper function to emit card update events
pub fn emit_card_update(
    app_handle: &tauri::AppHandle,
    card_id: i64,
    project_id: i64,
    document_id: Option<i64>,
    is_starred: Option<bool>,
    is_collapsed: Option<bool>,
) -> Result<()> {
    let mut payload = serde_json::json!({
        "cardId": card_id,
        "projectId": project_id,
    });
    
    if let Some(document_id) = document_id {
        payload["documentId"] = serde_json::Value::Number(serde_json::Number::from(document_id));
    }
    
    if let Some(is_starred) = is_starred {
        payload["isStarred"] = serde_json::Value::Bool(is_starred);
    }
    
    if let Some(is_collapsed) = is_collapsed {
        payload["isCollapsed"] = serde_json::Value::Bool(is_collapsed);
    }
    
    app_handle
        .emit_to("main", "card_updated", payload)
        .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
    
    Ok(())
}

/// Helper function to emit project update events
pub fn emit_project_update(
    app_handle: &tauri::AppHandle,
    project_id: i64,
    name: Option<&str>,
    description: Option<&str>,
    folder_id: Option<i64>,
    series_id: Option<i64>,
) -> Result<()> {
    let mut payload = serde_json::json!({
        "projectId": project_id,
    });
    
    if let Some(name) = name {
        payload["name"] = serde_json::Value::String(name.to_string());
    }
    
    if let Some(description) = description {
        payload["description"] = serde_json::Value::String(description.to_string());
    }
    
    if let Some(folder_id) = folder_id {
        payload["folderId"] = serde_json::Value::Number(serde_json::Number::from(folder_id));
    }
    
    if let Some(series_id) = series_id {
        payload["seriesId"] = serde_json::Value::Number(serde_json::Number::from(series_id));
    }
    
    app_handle
        .emit_to("main", "project_updated", payload)
        .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
    
    Ok(())
}

/// Helper function to emit folder update events
pub fn emit_folder_update(
    app_handle: &tauri::AppHandle,
    folder_id: i64,
    name: Option<&str>,
    parent_folder_id: Option<i64>,
) -> Result<()> {
    let mut payload = serde_json::json!({
        "folderId": folder_id,
    });
    
    if let Some(name) = name {
        payload["name"] = serde_json::Value::String(name.to_string());
    }
    
    if let Some(parent_folder_id) = parent_folder_id {
        payload["parentFolderId"] = serde_json::Value::Number(serde_json::Number::from(parent_folder_id));
    }
    
    app_handle
        .emit_to("main", "folder_updated", payload)
        .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
    
    Ok(())
}

/// Helper function to emit series update events
pub fn emit_series_update(
    app_handle: &tauri::AppHandle,
    series_id: i64,
    name: Option<&str>,
    description: Option<&str>,
    folder_id: Option<i64>,
) -> Result<()> {
    let mut payload = serde_json::json!({
        "seriesId": series_id,
    });
    
    if let Some(name) = name {
        payload["name"] = serde_json::Value::String(name.to_string());
    }
    
    if let Some(description) = description {
        payload["description"] = serde_json::Value::String(description.to_string());
    }
    
    if let Some(folder_id) = folder_id {
        payload["folderId"] = serde_json::Value::Number(serde_json::Number::from(folder_id));
    }
    
    app_handle
        .emit_to("main", "series_updated", payload)
        .map_err(|e| crate::error::StoryWeaverError::EventEmitError(e.to_string()))?;
    
    Ok(())
}
