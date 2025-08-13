//! Tauri commands for canvas and visual story planning features

use crate::database::{get_pool, models::canvas::*, operations::canvas as canvas_ops};
use crate::error::{Result, StoryWeaverError};
use crate::security::rate_limit::{rl_create, rl_update, rl_delete, rl_list};
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
    // Rate limiting
    rl_create("canvas", Some(&project_id))?;
    // Input validation
    crate::security::validation::validate_security_input(&project_id.to_string())?;
    crate::security::validation::validate_security_input(&name)?;
    crate::security::validation::validate_content_length(&name, 255)?;
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Canvas name cannot be empty"));
    }
    if let Some(ref desc) = description {
        crate::security::validation::validate_security_input(desc)?;
        crate::security::validation::validate_content_length(desc, 5000)?;
    }
    
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
    // Rate limiting
    rl_list("canvas", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    
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
    // Rate limiting
    rl_list("canvases", Some(&project_id))?;
    // Input validation
    crate::security::validation::validate_security_input(&project_id.to_string())?;
    
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
    // Rate limiting
    rl_update("canvas", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    if let Some(ref n) = name {
        crate::security::validation::validate_security_input(n)?;
        crate::security::validation::validate_content_length(n, 255)?;
        if n.trim().is_empty() {
            return Err(StoryWeaverError::validation("Canvas name cannot be empty"));
        }
    }
    if let Some(ref desc) = description {
        crate::security::validation::validate_security_input(desc)?;
        crate::security::validation::validate_content_length(desc, 5000)?;
    }
    
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
    // Rate limiting
    rl_delete("canvas", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    
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
    // Rate limiting
    rl_create("canvas_element", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    crate::security::validation::validate_security_input(&element_type)?;
    crate::security::validation::validate_content_length(&element_type, 50)?;
    if element_type.trim().is_empty() {
        return Err(StoryWeaverError::validation("Element type cannot be empty"));
    }
    crate::security::validation::validate_security_input(&title)?;
    crate::security::validation::validate_content_length(&title, 255)?;
    if title.trim().is_empty() {
        return Err(StoryWeaverError::validation("Title cannot be empty"));
    }
    crate::security::validation::validate_security_input(&content)?;
    crate::security::validation::validate_content_length(&content, 10000)?;
    crate::security::validation::validate_security_input(&color)?;
    crate::security::validation::validate_content_length(&color, 50)?;
    if color.trim().is_empty() {
        return Err(StoryWeaverError::validation("Color cannot be empty"));
    }
    crate::security::validation::validate_security_input(&metadata)?;
    crate::security::validation::validate_content_length(&metadata, 5000)?;
    crate::security::validation::validate_security_input(&connections)?;
    crate::security::validation::validate_content_length(&connections, 5000)?;
    
    // Validate numeric ranges
    if width < 0.0 || width > 10000.0 {
        return Err(StoryWeaverError::invalid_input("Width must be between 0 and 10000"));
    }
    if height < 0.0 || height > 10000.0 {
        return Err(StoryWeaverError::invalid_input("Height must be between 0 and 10000"));
    }
    if order_index < 0 {
        return Err(StoryWeaverError::invalid_input("Order index must be non-negative"));
    }
    
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
    // Rate limiting
    rl_list("canvas_elements", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    
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
    // Rate limiting
    rl_update("canvas_element", Some(&element_id.to_string()))?;
    // Input validation
    if element_id <= 0 {
        return Err(StoryWeaverError::validation("Element ID must be positive"));
    }
    if let Some(w) = width {
        if w < 0.0 || w > 10000.0 {
            return Err(StoryWeaverError::invalid_input("Width must be between 0 and 10000"));
        }
    }
    if let Some(h) = height {
        if h < 0.0 || h > 10000.0 {
            return Err(StoryWeaverError::invalid_input("Height must be between 0 and 10000"));
        }
    }
    if let Some(ref c) = content {
        crate::security::validation::validate_security_input(c)?;
        crate::security::validation::validate_content_length(c, 10000)?;
    }
    if let Some(ref col) = color {
        crate::security::validation::validate_security_input(col)?;
        crate::security::validation::validate_content_length(col, 50)?;
        if col.trim().is_empty() {
            return Err(StoryWeaverError::validation("Color cannot be empty"));
        }
    }
    if let Some(ref t) = title {
        crate::security::validation::validate_security_input(t)?;
        crate::security::validation::validate_content_length(t, 255)?;
        if t.trim().is_empty() {
            return Err(StoryWeaverError::validation("Title cannot be empty"));
        }
    }
    if let Some(oi) = order_index {
        if oi < 0 {
            return Err(StoryWeaverError::invalid_input("Order index must be non-negative"));
        }
    }
    
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
    // Rate limiting
    rl_delete("canvas_element", Some(&element_id.to_string()))?;
    // Input validation
    if element_id <= 0 {
        return Err(StoryWeaverError::validation("Element ID must be positive"));
    }
    
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
    // Rate limiting
    rl_list("outline_templates", None)?;
    // Input validation
    if let Some(ref tt) = template_type {
        crate::security::validation::validate_security_input(tt)?;
        crate::security::validation::validate_content_length(tt, 50)?;
        if tt.trim().is_empty() {
            return Err(StoryWeaverError::validation("Template type cannot be empty"));
        }
    }
    
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
    // Rate limiting
    rl_create("outline_template", None)?;
    // Input validation
    crate::security::validation::validate_security_input(&name)?;
    crate::security::validation::validate_content_length(&name, 255)?;
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Name cannot be empty"));
    }
    crate::security::validation::validate_security_input(&description)?;
    crate::security::validation::validate_content_length(&description, 5000)?;
    crate::security::validation::validate_security_input(&template_type)?;
    crate::security::validation::validate_content_length(&template_type, 50)?;
    if template_type.trim().is_empty() {
        return Err(StoryWeaverError::validation("Template type cannot be empty"));
    }
    crate::security::validation::validate_security_input(&structure)?;
    crate::security::validation::validate_content_length(&structure, 50000)?;
    if structure.trim().is_empty() {
        return Err(StoryWeaverError::validation("Structure cannot be empty"));
    }
    
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
    // Rate limiting
    rl_create("canvas_snapshot", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    crate::security::validation::validate_security_input(&name)?;
    crate::security::validation::validate_content_length(&name, 255)?;
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Name cannot be empty"));
    }
    crate::security::validation::validate_security_input(&snapshot_data)?;
    crate::security::validation::validate_content_length(&snapshot_data, 1000000)?; // 1MB limit for snapshot data
    if snapshot_data.trim().is_empty() {
        return Err(StoryWeaverError::validation("Snapshot data cannot be empty"));
    }
    
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
    // Rate limiting
    rl_list("canvas_snapshots", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    
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
    // Rate limiting
    rl_update("canvas_snapshot", Some(&snapshot_id.to_string()))?;
    // Input validation
    if snapshot_id <= 0 {
        return Err(StoryWeaverError::validation("Snapshot ID must be positive"));
    }
    
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
    // Rate limiting
    rl_list("canvas_export", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    crate::security::validation::validate_security_input(&format)?;
    crate::security::validation::validate_content_length(&format, 50)?;
    if format.trim().is_empty() {
        return Err(StoryWeaverError::validation("Format cannot be empty"));
    }
    
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
    // Rate limiting
    rl_create("canvas_collaboration", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    if max_participants < 1 || max_participants > 100 {
        return Err(StoryWeaverError::invalid_input("Max participants must be between 1 and 100"));
    }
    if let Some(hours) = expires_in_hours {
        if hours < 1 || hours > 8760 { // Max 1 year
            return Err(StoryWeaverError::invalid_input("Expires in hours must be between 1 and 8760"));
        }
    }
    
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    canvas_ops::create_canvas_collaboration_session(&pool, canvas_id, max_participants, expires_in_hours)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get canvas collaboration session
#[tauri::command]
pub async fn get_canvas_collaboration_session(
    session_token: String,
) -> Result<Option<CanvasCollaborationSession>> {
    // Rate limiting
    rl_list("canvas_collaboration", Some(&session_token))?;
    // Input validation
    if session_token.trim().is_empty() {
        return Err(StoryWeaverError::validation("Session token cannot be empty"));
    }
    if session_token.len() > 255 {
        return Err(StoryWeaverError::validation("Session token too long"));
    }
    
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
    // Rate limiting
    rl_update("canvas_collaboration", Some(&session_token))?;
    // Input validation
    if session_token.trim().is_empty() {
        return Err(StoryWeaverError::validation("Session token cannot be empty"));
    }
    if session_token.len() > 255 {
        return Err(StoryWeaverError::validation("Session token too long"));
    }
    
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
    // Rate limiting
    rl_update("canvas_collaboration", Some(&session_token))?;
    // Input validation
    if session_token.trim().is_empty() {
        return Err(StoryWeaverError::validation("Session token cannot be empty"));
    }
    if session_token.len() > 255 {
        return Err(StoryWeaverError::validation("Session token too long"));
    }
    
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
    // Rate limiting
    rl_update("canvas_collaboration", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    crate::security::validation::validate_security_input(&user_name)?;
    crate::security::validation::validate_content_length(&user_name, 100)?;
    if user_name.trim().is_empty() {
        return Err(StoryWeaverError::validation("User name cannot be empty"));
    }
    
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
    // Rate limiting
    rl_update("canvas_collaboration", Some(&session_token))?;
    // Input validation
    crate::security::validation::validate_security_input(&session_token)?;
    crate::security::validation::validate_content_length(&session_token, 255)?;
    if session_token.trim().is_empty() {
        return Err(StoryWeaverError::validation("Session token cannot be empty"));
    }
    crate::security::validation::validate_security_input(&user_name)?;
    crate::security::validation::validate_content_length(&user_name, 100)?;
    if user_name.trim().is_empty() {
        return Err(StoryWeaverError::validation("User name cannot be empty"));
    }
    
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
    // Rate limiting
    rl_create("canvas_operation", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    crate::security::validation::validate_security_input(&operation_type)?;
    crate::security::validation::validate_content_length(&operation_type, 50)?;
    if operation_type.trim().is_empty() {
        return Err(StoryWeaverError::validation("Operation type cannot be empty"));
    }
    if let Some(eid) = element_id {
        if eid <= 0 {
            return Err(StoryWeaverError::validation("Element ID must be positive"));
        }
    }
    crate::security::validation::validate_security_input(&user_token)?;
    crate::security::validation::validate_content_length(&user_token, 255)?;
    if user_token.trim().is_empty() {
        return Err(StoryWeaverError::validation("User token cannot be empty"));
    }
    
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
    // Rate limiting
    rl_list("canvas_operations", Some(&canvas_id.to_string()))?;
    // Input validation
    if canvas_id <= 0 {
        return Err(StoryWeaverError::validation("Canvas ID must be positive"));
    }
    if let Some(l) = limit {
        if l < 1 || l > 1000 {
            return Err(StoryWeaverError::invalid_input("Limit must be between 1 and 1000"));
        }
    }
    if let Some(o) = offset {
        if o < 0 {
            return Err(StoryWeaverError::invalid_input("Offset must be non-negative"));
        }
    }
    
    let pool = get_pool().map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    canvas_ops::get_canvas_operations(&pool, canvas_id, limit.unwrap_or(50), offset.unwrap_or(0))
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}
