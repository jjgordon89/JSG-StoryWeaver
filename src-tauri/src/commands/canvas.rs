//! Tauri commands for canvas and visual story planning features

use crate::database::{get_pool, models::canvas::*, operations::canvas::*};
use crate::error::{Result, StoryWeaverError};
use serde_json::Value;

/// Create a new canvas
#[tauri::command]
pub async fn create_canvas(
    project_id: String,
    name: String,
    description: Option<String>,
    canvas_type: String,
    settings: Option<Value>,
) -> Result<Canvas, String> {
    let pool = get_pool()?;
    
    let canvas_type_enum = match canvas_type.as_str() {
        "story_outline" => CanvasType::StoryOutline,
        "character_map" => CanvasType::CharacterMap,
        "world_building" => CanvasType::WorldBuilding,
        "timeline" => CanvasType::Timeline,
        "plot_structure" => CanvasType::PlotStructure,
        "mind_map" => CanvasType::MindMap,
        "free_form" => CanvasType::FreeForm,
        _ => return Err("Invalid canvas type".to_string()),
    };
    
    let canvas = Canvas {
        id: String::new(), // Will be set by database
        project_id,
        name,
        description,
        canvas_type: canvas_type_enum,
        settings: settings.unwrap_or(Value::Object(serde_json::Map::new())),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    create_canvas(&pool, canvas)
        .await
        .map_err(|e| format!("Failed to create canvas: {}", e))
}

/// Get canvas by ID
#[tauri::command]
pub async fn get_canvas(
    canvas_id: String,
) -> Result<Option<Canvas>, String> {
    let pool = get_pool()?;
    
    get_canvas_by_id(&pool, &canvas_id)
        .await
        .map_err(|e| format!("Failed to get canvas: {}", e))
}

/// Get canvases for a project
#[tauri::command]
pub async fn get_project_canvases(
    project_id: String,
) -> Result<Vec<Canvas>, String> {
    let pool = get_pool()?;
    
    get_canvases_by_project(&pool, &project_id)
        .await
        .map_err(|e| format!("Failed to get project canvases: {}", e))
}

/// Update canvas
#[tauri::command]
pub async fn update_canvas(
    canvas_id: String,
    name: Option<String>,
    description: Option<String>,
    settings: Option<Value>,
) -> Result<(), String> {
    let pool = get_pool()?;
    
    update_canvas(&pool, &canvas_id, name.as_deref(), description, settings.as_ref())
        .await
        .map_err(|e| format!("Failed to update canvas: {}", e))
}

/// Delete canvas
#[tauri::command]
pub async fn delete_canvas(
    canvas_id: String,
) -> Result<(), String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    delete_canvas(&pool, &canvas_id)
        .await
        .map_err(|e| format!("Failed to delete canvas: {}", e))
}

/// Create canvas element
#[tauri::command]
pub async fn create_canvas_element(
    canvas_id: String,
    element_type: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    content: Value,
    style: Option<Value>,
    z_index: Option<i32>,
) -> Result<CanvasElement, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    let element_type_enum = match element_type.as_str() {
        "text_box" => CanvasElementType::TextBox,
        "sticky_note" => CanvasElementType::StickyNote,
        "character_card" => CanvasElementType::CharacterCard,
        "scene_card" => CanvasElementType::SceneCard,
        "plot_point" => CanvasElementType::PlotPoint,
        "timeline_event" => CanvasElementType::TimelineEvent,
        "connection_line" => CanvasElementType::ConnectionLine,
        "image" => CanvasElementType::Image,
        "shape" => CanvasElementType::Shape,
        "group" => CanvasElementType::Group,
        _ => return Err("Invalid canvas element type".to_string()),
    };
    
    let element = CanvasElement {
        id: String::new(), // Will be set by database
        canvas_id,
        element_type: element_type_enum,
        x,
        y,
        width,
        height,
        content,
        style: style.unwrap_or(Value::Object(serde_json::Map::new())),
        z_index: z_index.unwrap_or(0),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    create_canvas_element(&pool, element)
        .await
        .map_err(|e| format!("Failed to create canvas element: {}", e))
}

/// Get canvas elements
#[tauri::command]
pub async fn get_canvas_elements(
    canvas_id: String,
) -> Result<Vec<CanvasElement>, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    get_canvas_elements(&pool, &canvas_id)
        .await
        .map_err(|e| format!("Failed to get canvas elements: {}", e))
}

/// Update canvas element
#[tauri::command]
pub async fn update_canvas_element(
    element_id: String,
    x: Option<f64>,
    y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    content: Option<Value>,
    style: Option<Value>,
    z_index: Option<i32>,
) -> Result<(), String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    update_canvas_element(&pool, &element_id, x, y, width, height, content.as_ref(), style.as_ref(), z_index)
        .await
        .map_err(|e| format!("Failed to update canvas element: {}", e))
}

/// Delete canvas element
#[tauri::command]
pub async fn delete_canvas_element(
    element_id: String,
) -> Result<(), String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    delete_canvas_element(&pool, &element_id)
        .await
        .map_err(|e| format!("Failed to delete canvas element: {}", e))
}

/// Get outline templates
#[tauri::command]
pub async fn get_outline_templates(
    template_type: Option<String>,
) -> Result<Vec<OutlineTemplate>, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    let template_type_enum = if let Some(t_type) = template_type {
        Some(match t_type.as_str() {
            "three_act" => OutlineTemplateType::ThreeAct,
            "heros_journey" => OutlineTemplateType::HerosJourney,
            "save_the_cat" => OutlineTemplateType::SaveTheCat,
            "snowflake" => OutlineTemplateType::Snowflake,
            "freytag_pyramid" => OutlineTemplateType::FreytagPyramid,
            "seven_point" => OutlineTemplateType::SevenPoint,
            "custom" => OutlineTemplateType::Custom,
            _ => return Err("Invalid outline template type".to_string()),
        })
    } else {
        None
    };
    
    get_outline_templates(&pool, template_type_enum, false)
        .await
        .map_err(|e| format!("Failed to get outline templates: {}", e))
}

/// Create outline template
#[tauri::command]
pub async fn create_outline_template(
    name: String,
    description: String,
    template_type: String,
    structure: Value,
) -> Result<OutlineTemplate, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    let template_type_enum = match template_type.as_str() {
        "three_act" => OutlineTemplateType::ThreeAct,
        "heros_journey" => OutlineTemplateType::HerosJourney,
        "save_the_cat" => OutlineTemplateType::SaveTheCat,
        "snowflake" => OutlineTemplateType::Snowflake,
        "freytag_pyramid" => OutlineTemplateType::FreytagPyramid,
        "seven_point" => OutlineTemplateType::SevenPoint,
        "custom" => OutlineTemplateType::Custom,
        _ => return Err("Invalid outline template type".to_string()),
    };
    
    let template_data = serde_json::to_string(&structure).unwrap_or_default();
    
    let template = OutlineTemplate {
        id: 0, // Will be set by database
        name,
        description,
        template_type: template_type_enum,
        template_data,
        is_official: false,
        created_at: chrono::Utc::now(),
    };
    
    create_outline_template(&pool, template)
        .await
        .map_err(|e| format!("Failed to create outline template: {}", e))
}

/// Create canvas snapshot
#[tauri::command]
pub async fn create_canvas_snapshot(
    canvas_id: String,
    name: String,
    snapshot_data: serde_json::Value,
) -> Result<CanvasSnapshot, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    create_canvas_snapshot(&pool, &canvas_id, &name, snapshot_data)
        .await
        .map_err(|e| format!("Failed to create canvas snapshot: {}", e))
}

/// Get canvas snapshots
#[tauri::command]
pub async fn get_canvas_snapshots(
    canvas_id: String,
) -> Result<Vec<CanvasSnapshot>, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    get_canvas_snapshots(&pool, &canvas_id)
        .await
        .map_err(|e| format!("Failed to get canvas snapshots: {}", e))
}

/// Restore canvas snapshot
#[tauri::command]
pub async fn restore_canvas_snapshot(
    snapshot_id: i32,
) -> Result<(), String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    restore_canvas_snapshot(&pool, snapshot_id)
        .await
        .map_err(|e| format!("Failed to restore canvas snapshot: {}", e))
}

/// Export canvas
#[tauri::command]
pub async fn export_canvas(
    canvas_id: String,
    format: String,
    options: Option<Value>,
) -> Result<CanvasExportResult, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    let export_format = match format.as_str() {
        "png" => ExportFormat::PNG,
        "svg" => ExportFormat::SVG,
        "pdf" => ExportFormat::PDF,
        "json" => ExportFormat::JSON,
        _ => return Err("Invalid export format".to_string()),
    };
    
    let canvas_id_int: i32 = canvas_id.parse().unwrap_or(0);
    let request = CanvasExportRequest {
        canvas_id: canvas_id_int,
        export_format,
        include_connections: true,
        include_metadata: true,
    };

    crate::database::operations::canvas::export_canvas_data(&pool, &canvas_id, export_format)
        .await
        .map_err(|e| format!("Failed to export canvas: {}", e))
}

/// Create canvas collaboration session
#[tauri::command]
pub async fn create_canvas_collaboration_session(
    canvas_id: String,
    max_participants: i32,
    expires_in_hours: Option<i32>,
) -> Result<CanvasCollaborationSession, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    create_canvas_collaboration_session(&pool, canvas_id, max_participants, expires_in_hours)
        .await
        .map_err(|e| format!("Failed to create canvas collaboration session: {}", e))
}

/// Get canvas collaboration session
#[tauri::command]
pub async fn get_canvas_collaboration_session(
    session_token: String,
) -> Result<Option<CanvasCollaborationSession>, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    crate::database::operations::canvas::get_canvas_collaboration_session(&pool, &session_token)
        .await
        .map_err(|e| format!("Failed to get canvas collaboration session: {}", e))
}

/// Join canvas collaboration session
#[tauri::command]
pub async fn join_canvas_collaboration_session(
    session_token: String,
    participant_name: String,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool()?;
    
    let session = crate::database::operations::canvas::get_canvas_collaboration_session(&pool, &session_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(sess) = session {
        // Check if session has expired
        if let Some(expires_at) = sess.expires_at {
            if chrono::Utc::now() > expires_at {
                return Err(StoryWeaverError::validation("Canvas collaboration session has expired"));
            }
        }
        
        // Parse current participants
        let mut participants: Vec<String> = serde_json::from_str(&sess.participants)
            .unwrap_or_else(|_| Vec::new());
        
        // Check if participant already exists
        if participants.contains(&participant_name) {
            return Err(StoryWeaverError::validation("Participant already in session"));
        }
        
        // Add new participant
        participants.push(participant_name);
        let participants_json = serde_json::to_string(&participants)
            .map_err(|e| StoryWeaverError::validation(format!("Failed to serialize participants: {}", e)))?;
        
        // Update participants list
        update_canvas_session_participants(&pool, &session_token, &participants_json)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    } else {
        return Err(StoryWeaverError::validation("Canvas collaboration session not found"));
    }
    
    Ok(())
}

/// Leave canvas collaboration session
#[tauri::command]
pub async fn leave_canvas_collaboration_session(
    session_token: String,
    participant_name: String,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool()?;
    
    let session = crate::database::operations::canvas::get_canvas_collaboration_session(&pool, &session_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(sess) = session {
        // Parse current participants
        let mut participants: Vec<String> = serde_json::from_str(&sess.participants)
            .unwrap_or_else(|_| Vec::new());
        
        // Remove participant if they exist
        participants.retain(|p| p != &participant_name);
        let participants_json = serde_json::to_string(&participants)
            .map_err(|e| StoryWeaverError::validation(format!("Failed to serialize participants: {}", e)))?;
        
        // Update participants list
        update_canvas_session_participants(&pool, &session_token, &participants_json)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    }
    
    Ok(())
}

/// Join canvas collaboration session
#[tauri::command]
pub async fn join_canvas_collaboration(
    canvas_id: String,
    user_name: String,
) -> Result<String, StoryWeaverError> {
    let pool = get_pool()?;
    
    // Check if there's an existing session for this canvas
    let existing_session = crate::database::operations::canvas::get_canvas_collaboration_session_by_canvas(&pool, &canvas_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(session) = existing_session {
        // Parse current participants
        let mut participants: Vec<String> = serde_json::from_str(&session.participants)
            .unwrap_or_else(|_| Vec::new());
        
        // Add participant if not already present
        if !participants.contains(&user_name) {
            participants.push(user_name);
            let participants_json = serde_json::to_string(&participants)
                .map_err(|e| StoryWeaverError::validation(format!("Failed to serialize participants: {}", e)))?;
            
            // Update participants list
            update_canvas_session_participants(&pool, &session.session_token, &participants_json)
                .await
                .map_err(|e| StoryWeaverError::database(e.to_string()))?;
        }
        
        Ok(session.session_token)
    } else {
        // Create new collaboration session
        let session_token = uuid::Uuid::new_v4().to_string();
        let participants = vec![user_name];
        let participants_json = serde_json::to_string(&participants)
            .map_err(|e| StoryWeaverError::validation(format!("Failed to serialize participants: {}", e)))?;
        
        let session = CanvasCollaborationSession {
            id: 0,
            canvas_id: canvas_id.clone(),
            session_token: session_token.clone(),
            host_user: participants[0].clone(),
            participants: participants_json,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        crate::database::operations::canvas::create_canvas_collaboration_session(&pool, session)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
        
        Ok(session_token)
    }
}

/// Leave canvas collaboration session
#[tauri::command]
pub async fn leave_canvas_collaboration(
    session_token: String,
    user_name: String,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool()?;
    
    let session = crate::database::operations::canvas::get_canvas_collaboration_session(&pool, &session_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(sess) = session {
        // Parse current participants
        let mut participants: Vec<String> = serde_json::from_str(&sess.participants)
            .unwrap_or_else(|_| Vec::new());
        
        // Remove participant if they exist
        participants.retain(|p| p != &user_name);
        let participants_json = serde_json::to_string(&participants)
            .map_err(|e| StoryWeaverError::validation(format!("Failed to serialize participants: {}", e)))?;
        
        // Update participants list
        update_canvas_session_participants(&pool, &session_token, &participants_json)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    }
    
    Ok(())
}

/// Record canvas operation (for real-time collaboration)
#[tauri::command]
pub async fn record_canvas_operation(
    canvas_id: String,
    operation_type: String,
    element_id: Option<String>,
    operation_data: Value,
    user_id: Option<String>,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool()?;
    
    let operation_type_enum = match operation_type.as_str() {
        "create_element" => CanvasOperationType::CreateElement,
        "update_element" => CanvasOperationType::UpdateElement,
        "delete_element" => CanvasOperationType::DeleteElement,
        "move_element" => CanvasOperationType::MoveElement,
        "resize_element" => CanvasOperationType::ResizeElement,
        "update_canvas" => CanvasOperationType::UpdateCanvas,
        _ => return Err(StoryWeaverError::validation("Invalid canvas operation type")),
    };
    
    let operation = CanvasOperation {
        canvas_id: canvas_id.clone(),
        operation_type: operation_type_enum,
        element_id: element_id.and_then(|s| s.parse::<i32>().ok()),
        data: serde_json::to_string(&operation_data).unwrap_or_default(),
        user_token: user_id.unwrap_or_default(),
        timestamp: chrono::Utc::now(),
    };    
    crate::database::operations::canvas::record_canvas_operation(&pool, operation)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record canvas operation: {}", e)))
}

/// Get canvas operations (for operation history/undo)
#[tauri::command]
pub async fn get_canvas_operations(
    canvas_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<CanvasOperation>, String> {
    let pool = get_pool().map_err(|e| format!("Failed to get database pool: {}", e))?;
    
    crate::database::operations::canvas::get_canvas_operations(&pool, &canvas_id, Some(limit.unwrap_or(50)), Some(offset.unwrap_or(0)))
        .await
        .map_err(|e| format!("Failed to get canvas operations: {}", e))
}