//! Tauri commands for canvas and visual story planning features

use crate::database::{get_pool, models::canvas::*, operations::canvas as canvas_ops};
use crate::error::{Result, StoryWeaverError};
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;
use std::str::FromStr;

/// Create a new canvas
#[tauri::command]
pub async fn create_canvas(
    project_id: String,
    name: String,
    description: Option<String>,
) -> Result<Canvas> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::create_canvas(&pool, &project_id, &name, description.as_deref())
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get a canvas by ID
#[tauri::command]
pub async fn get_canvas(
    canvas_id: i32,
) -> Result<Option<Canvas>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::get_canvas_by_id(&pool, canvas_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get all canvases for a project
#[tauri::command]
pub async fn get_project_canvases(
    project_id: String,
) -> Result<Vec<Canvas>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::get_project_canvases(&pool, &project_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Update a canvas
#[tauri::command]
pub async fn update_canvas(
    canvas_id: i32,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::update_canvas(&pool, canvas_id, name.as_deref(), description.as_deref())
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Delete canvas
#[tauri::command]
pub async fn delete_canvas(
    canvas_id: i32,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::delete_canvas(&pool, canvas_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Create canvas element
#[tauri::command]
pub async fn create_canvas_element(
    canvas_id: i32,
    element_type: String,
    title: String,
    content: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: String,
    metadata: String,
    connections: String,
    order_index: i32,
) -> Result<CanvasElement> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    let element_type_enum = CanvasElementType::from_str(&element_type)
        .map_err(|e| StoryWeaverError::invalid_input(e))?;
    canvas_ops::create_canvas_element(
        &pool,
        canvas_id,
        element_type_enum,
        &title,
        &content,
        x,
        y,
        width,
        height,
        &color,
        &metadata,
        &connections,
        order_index,
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create canvas element: {}", e)))
}

/// Get canvas elements
#[tauri::command]
pub async fn get_canvas_elements(
    canvas_id: i32,
) -> Result<Vec<CanvasElement>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::get_canvas_elements(&pool, canvas_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get canvas elements: {}", e)))
}

/// Update canvas element
#[tauri::command]
pub async fn update_canvas_element(
    element_id: i32,
    x: Option<f32>,
    y: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    content: Option<String>,
    color: Option<String>,
    title: Option<String>,
    order_index: Option<i32>,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::update_canvas_element(
        &pool,
        element_id,
        x,
        y,
        width,
        height,
        content.as_deref(),
        color.as_deref(),
        title.as_deref(),
        order_index,
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to update canvas element: {}", e)))
}

/// Delete canvas element
#[tauri::command]
pub async fn delete_canvas_element(
    element_id: i32,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::delete_canvas_element(&pool, element_id)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete canvas element: {}", e)))
}

/// Get outline templates
#[tauri::command]
pub async fn get_outline_templates(
    template_type: Option<String>,
) -> Result<Vec<OutlineTemplate>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    let template_type_enum = template_type
        .map(|s| OutlineTemplateType::from_str(&s))
        .transpose()
        .map_err(|e| StoryWeaverError::invalid_input(e))?;
    canvas_ops::get_outline_templates(&pool, template_type_enum)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outline templates: {}", e)))
}

/// Create outline template
#[tauri::command]
pub async fn create_outline_template(
    name: String,
    description: String,
    template_type: String,
    structure: String,
    is_official: bool,
) -> Result<OutlineTemplate> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    let template_type_enum = OutlineTemplateType::from_str(&template_type)
        .map_err(|e| StoryWeaverError::invalid_input(e))?;
    canvas_ops::create_outline_template(
        &pool,
        &name,
        &description,
        template_type_enum,
        &structure,
        is_official,
    )
    .await
    .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Create canvas snapshot
#[tauri::command]
pub async fn create_canvas_snapshot(
    canvas_id: i32,
    name: String,
    snapshot_data: String,
) -> Result<CanvasSnapshot> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::create_canvas_snapshot(&pool, canvas_id, &name, &snapshot_data)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get canvas snapshots
#[tauri::command]
pub async fn get_canvas_snapshots(
    canvas_id: i32,
) -> Result<Vec<CanvasSnapshot>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::get_canvas_snapshots(&pool, canvas_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Restore canvas snapshot
#[tauri::command]
pub async fn restore_canvas_snapshot(
    snapshot_id: i32,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::restore_canvas_snapshot(&pool, snapshot_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Export canvas
#[tauri::command]
pub async fn export_canvas(
    canvas_id: i32,
    format: String,
) -> Result<CanvasExportResult> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    let export_format =
        ExportFormat::from_str(&format).map_err(|e| StoryWeaverError::invalid_input(e))?;
    canvas_ops::export_canvas(&pool, canvas_id, export_format)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Create canvas collaboration session
#[tauri::command]
pub async fn create_canvas_collaboration_session(
    canvas_id: i32,
    max_participants: i32,
    expires_in_hours: Option<i64>,
) -> Result<CanvasCollaborationSession> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::create_canvas_collaboration_session(&pool, canvas_id, max_participants, expires_in_hours)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get canvas collaboration session by token
#[tauri::command]
pub async fn get_canvas_collaboration_session(
    session_token: String,
) -> Result<Option<CanvasCollaborationSession>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::get_canvas_collaboration_session_by_token(&pool, &session_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Join canvas collaboration session
#[tauri::command]
pub async fn join_canvas_collaboration_session(
    session_token: String,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    let session = canvas_ops::get_canvas_collaboration_session_by_token(&pool, &session_token)
        .await?
        .ok_or_else(|| StoryWeaverError::validation("Canvas collaboration session not found"))?;

    if let Some(expires_at) = session.expires_at {
        if Utc::now() > expires_at {
            return Err(StoryWeaverError::validation("Canvas collaboration session has expired"));
        }
    }

    let updated_participants = session.current_participants + 1;

    if updated_participants > session.max_participants {
        return Err(StoryWeaverError::validation("Session is full"));
    }

    canvas_ops::update_canvas_collaboration_session(
        &pool,
        session.id,
        None,
        Some(updated_participants),
    )
    .await?;
    Ok(())
}

/// Leave canvas collaboration session
#[tauri::command]
pub async fn leave_canvas_collaboration_session(
    session_token: String,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    let session = canvas_ops::get_canvas_collaboration_session_by_token(&pool, &session_token)
        .await?
        .ok_or_else(|| StoryWeaverError::validation("Canvas collaboration session not found"))?;

    let updated_participants = session.current_participants - 1;

    canvas_ops::update_canvas_collaboration_session(
        &pool,
        session.id,
        None,
        Some(updated_participants),
    )
    .await?;

    Ok(())
}

/// Join canvas collaboration
#[tauri::command]
pub async fn join_canvas_collaboration(
    canvas_id: i32,
    user_name: String,
) -> Result<String> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let session = match canvas_ops::get_canvas_collaboration_session_by_canvas_id(&pool, canvas_id).await? {
        Some(mut s) => {
            let mut participants: Vec<String> = serde_json::from_str(&s.participants).unwrap_or_default();
            if !participants.contains(&user_name) {
                participants.push(user_name);
                s.participants = serde_json::to_string(&participants)?;
                s.current_participants = participants.len() as i32;
                canvas_ops::update_canvas_collaboration_session(&pool, s.id, None, Some(s.current_participants)).await?;
            }
            s
        },
        None => {
            let new_session = canvas_ops::create_canvas_collaboration_session(&pool, canvas_id, 10, Some(24)).await?;
            let mut s = new_session;
            let participants = vec![user_name];
            s.participants = serde_json::to_string(&participants)?;
            s.current_participants = 1;
            canvas_ops::update_canvas_collaboration_session(&pool, s.id, None, Some(1)).await?;
            s
        }
    };
    
    Ok(session.session_token)
}

/// Leave canvas collaboration
#[tauri::command]
pub async fn leave_canvas_collaboration(
    session_token: String,
    user_name: String,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(mut session) = canvas_ops::get_canvas_collaboration_session_by_token(&pool, &session_token).await? {
        let mut participants: Vec<String> = serde_json::from_str(&session.participants).unwrap_or_default();
        participants.retain(|p| p != &user_name);
        session.participants = serde_json::to_string(&participants)?;
        session.current_participants = participants.len() as i32;
        canvas_ops::update_canvas_collaboration_session(&pool, session.id, None, Some(session.current_participants)).await?;
    }
    
    Ok(())
}

/// Record canvas operation (for real-time collaboration)
#[tauri::command]
pub async fn record_canvas_operation(
    canvas_id: i32,
    operation_type: String,
    element_id: Option<i32>,
    operation_data: Value,
    user_token: String,
) -> Result<()> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    let operation_type_enum = CanvasOperationType::from_str(&operation_type).map_err(|e| StoryWeaverError::invalid_input(e))?;
    
    let operation = CanvasOperation {
        id: Uuid::new_v4().to_string(),
        canvas_id,
        operation_type: operation_type_enum,
        element_id,
        data: serde_json::to_string(&operation_data).unwrap_or_default(),
        user_token,
        timestamp: chrono::Utc::now().timestamp_millis(),
    };    
    canvas_ops::record_canvas_operation(&pool, &operation)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record canvas operation: {}", e)))
}

/// Get canvas operations (for operation history/undo)
#[tauri::command]
pub async fn get_canvas_operations(
    canvas_id: i32,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<CanvasOperation>> {
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    canvas_ops::get_canvas_operations(&pool, canvas_id, limit.unwrap_or(50), offset.unwrap_or(0))
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}
