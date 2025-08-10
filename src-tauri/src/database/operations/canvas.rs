//! Database operations for canvas and visual story planning features

use crate::database::models::*;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

/// Create a new canvas
pub async fn create_canvas(
    pool: &SqlitePool,
    project_id: &str,
    name: &str,
    description: Option<&str>,
    canvas_type: CanvasType,
    settings: Option<Value>,
) -> Result<Canvas, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO canvas (
            id, project_id, name, description, canvas_type, settings,
            is_active, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        project_id,
        name,
        description,
        canvas_type.to_string(),
        settings.map(|s| s.to_string()),
        true,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(Canvas {
        id: 0, // Will be set by database auto-increment
        project_id: project_id.to_string(),
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        canvas_data: String::new(), // Empty canvas data initially
        template_type: None,
        width: 1920,
        height: 1080,
        zoom_level: 1.0,
        viewport_x: 0.0,
        viewport_y: 0.0,
        created_at: now,
        updated_at: now,
    })
}

/// Get canvas by ID
pub async fn get_canvas_by_id(
    pool: &SqlitePool,
    canvas_id: &str,
) -> Result<Option<Canvas>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, project_id, canvas_data, template_type, width, height,
               zoom_level, viewport_x, viewport_y, created_at, updated_at
        FROM canvas
        WHERE id = ?
        "#,
        canvas_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = result {
        Ok(Some(Canvas {
            id: row.id,
            project_id: row.project_id,
            name: String::new(),
            description: None,
            canvas_data: row.canvas_data.unwrap_or_default(),
            template_type: row.template_type,
            width: row.width.unwrap_or(1920),
            height: row.height.unwrap_or(1080),
            zoom_level: row.zoom_level.unwrap_or(1.0),
            viewport_x: row.viewport_x.unwrap_or(0.0),
            viewport_y: row.viewport_y.unwrap_or(0.0),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    } else {
        Ok(None)
    }
}

/// Get canvases for a project
pub async fn get_project_canvases(
    pool: &SqlitePool,
    project_id: &str,
) -> Result<Vec<Canvas>, sqlx::Error> {
    let results = sqlx::query!(
        r#"
        SELECT id, project_id, canvas_data, template_type, width, height,
               zoom_level, viewport_x, viewport_y, created_at, updated_at
        FROM canvas
        WHERE project_id = ?
        ORDER BY created_at DESC
        "#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    let mut canvases = Vec::new();
    for row in results {
        canvases.push(Canvas {
            id: row.id,
            project_id: row.project_id,
            name: String::new(),
            description: None,
            canvas_data: row.canvas_data.unwrap_or_default(),
            template_type: row.template_type,
            width: row.width.unwrap_or(1920),
            height: row.height.unwrap_or(1080),
            zoom_level: row.zoom_level.unwrap_or(1.0),
            viewport_x: row.viewport_x.unwrap_or(0.0),
            viewport_y: row.viewport_y.unwrap_or(0.0),
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }

    Ok(canvases)
}

/// Create a canvas element
pub async fn create_canvas_element(
    pool: &SqlitePool,
    canvas_id: &str,
    element_type: CanvasElementType,
    position_x: f64,
    position_y: f64,
    width: f64,
    height: f64,
    content: Value,
    style: Option<Value>,
) -> Result<CanvasElement, sqlx::Error> {
    let now = Utc::now();
    let canvas_id_int: i32 = canvas_id.parse().unwrap_or(0);

    let result = sqlx::query!(
        r#"
        INSERT INTO canvas_elements (
            canvas_id, element_type, position_x, position_y, width, height,
            title, content, color, metadata, connections, order_index, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        canvas_id_int,
        element_type.to_string(),
        position_x,
        position_y,
        width,
        height,
        "", // title
        content.to_string(),
        "#000000", // default color
        "{}", // empty metadata
        "[]", // empty connections
        0, // order_index
        now,
        now
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(CanvasElement {
        id,
        canvas_id: canvas_id_int,
        element_type,
        title: String::new(),
        content: content.to_string(),
        position_x: position_x as f32,
        position_y: position_y as f32,
        width: width as f32,
        height: height as f32,
        color: "#000000".to_string(),
        metadata: "{}".to_string(),
        connections: "[]".to_string(),
        order_index: 0,
        created_at: now,
        updated_at: now,
    })
}

/// Get canvas elements
pub async fn get_canvas_elements(
    pool: &SqlitePool,
    canvas_id: &str,
) -> Result<Vec<CanvasElement>, sqlx::Error> {
    let results = sqlx::query!(
        r#"
        SELECT id, canvas_id, element_type, position_x, position_y, width, height,
               content, style, z_index, is_locked, is_visible, created_at, updated_at
        FROM canvas_elements
        WHERE canvas_id = ? AND is_visible = 1
        ORDER BY z_index ASC, created_at ASC
        "#,
        canvas_id
    )
    .fetch_all(pool)
    .await?;

    let mut elements = Vec::new();
    for row in results {
        elements.push(CanvasElement {
            id: row.id,
            canvas_id: row.canvas_id,
            element_type: row.element_type.parse().unwrap_or(CanvasElementType::TextBox),
            position_x: row.position_x,
            position_y: row.position_y,
            width: row.width,
            height: row.height,
            content: serde_json::from_str(&row.content).unwrap_or(Value::Null),
            style: row.style.and_then(|s| serde_json::from_str(&s).ok()),
            z_index: row.z_index,
            is_locked: row.is_locked,
            is_visible: row.is_visible,
            created_at: row.created_at,
            updated_at: row.updated_at,
        });
    }

    Ok(elements)
}

/// Update canvas element
pub async fn update_canvas_element(
    pool: &SqlitePool,
    element_id: &str,
    position_x: Option<f64>,
    position_y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    content: Option<Value>,
    style: Option<Value>,
    z_index: Option<i32>,
    is_locked: Option<bool>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    
    // For simplicity, we'll update all provided fields
    // In a real implementation, you'd want to build a dynamic query
    if let Some(x) = position_x {
        sqlx::query!(
            "UPDATE canvas_elements SET position_x = ?, updated_at = ? WHERE id = ?",
            x, now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(y) = position_y {
        sqlx::query!(
            "UPDATE canvas_elements SET position_y = ?, updated_at = ? WHERE id = ?",
            y, now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(w) = width {
        sqlx::query!(
            "UPDATE canvas_elements SET width = ?, updated_at = ? WHERE id = ?",
            w, now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(h) = height {
        sqlx::query!(
            "UPDATE canvas_elements SET height = ?, updated_at = ? WHERE id = ?",
            h, now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(c) = content {
        sqlx::query!(
            "UPDATE canvas_elements SET content = ?, updated_at = ? WHERE id = ?",
            c.to_string(), now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(s) = style {
        sqlx::query!(
            "UPDATE canvas_elements SET style = ?, updated_at = ? WHERE id = ?",
            s.to_string(), now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(z) = z_index {
        sqlx::query!(
            "UPDATE canvas_elements SET z_index = ?, updated_at = ? WHERE id = ?",
            z, now, element_id
        ).execute(pool).await?;
    }
    
    if let Some(locked) = is_locked {
        sqlx::query!(
            "UPDATE canvas_elements SET is_locked = ?, updated_at = ? WHERE id = ?",
            locked, now, element_id
        ).execute(pool).await?;
    }

    Ok(())
}

/// Delete canvas element
pub async fn delete_canvas_element(
    pool: &SqlitePool,
    element_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE canvas_elements SET is_visible = 0, updated_at = ? WHERE id = ?",
        Utc::now(),
        element_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create outline template
pub async fn create_outline_template(
    pool: &SqlitePool,
    name: &str,
    description: &str,
    template_type: OutlineTemplateType,
    structure: Value,
    is_public: bool,
    created_by: Option<&str>,
) -> Result<OutlineTemplate, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO outline_templates (
            id, name, description, template_type, structure, is_public,
            usage_count, created_by, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        id,
        name,
        description,
        template_type.to_string(),
        structure.to_string(),
        is_public,
        0,
        created_by,
        now,
        now
    )
    .execute(pool)
    .await?;

    Ok(OutlineTemplate {
        id,
        name: name.to_string(),
        description: description.to_string(),
        template_type,
        structure,
        is_public,
        usage_count: 0,
        created_by: created_by.map(|s| s.to_string()),
        created_at: now,
        updated_at: now,
    })
}

/// Get outline templates
pub async fn get_outline_templates(
    pool: &SqlitePool,
    template_type: Option<OutlineTemplateType>,
    include_private: bool,
) -> Result<Vec<OutlineTemplate>, sqlx::Error> {
    let mut sql = String::from(
        r#"
        SELECT id, name, description, template_type, template_data, is_official, created_at
        FROM outline_templates
        WHERE 1=1
        "#
    );

    if !include_private {
        sql.push_str(" AND is_official = 1");
    }

    if let Some(t_type) = template_type {
        sql.push_str(&format!(" AND template_type = '{}'", t_type.to_string()));
    }

    sql.push_str(" ORDER BY created_at DESC");

    let results = sqlx::query(&sql)
        .fetch_all(pool)
        .await?;

    let mut templates = Vec::new();
    for row in results {
        templates.push(OutlineTemplate {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            template_type: row.get::<String, _>("template_type").parse().unwrap_or(OutlineTemplateType::ThreeAct),
            template_data: row.get("template_data"),
            is_official: row.get("is_official"),
            created_at: row.get("created_at"),
        });
    }

    Ok(templates)
}

/// Increment template usage count
pub async fn increment_template_usage(
    pool: &SqlitePool,
    template_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE outline_templates SET usage_count = usage_count + 1, updated_at = ? WHERE id = ?",
        Utc::now(),
        template_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create canvas snapshot
pub async fn create_canvas_snapshot(
    pool: &SqlitePool,
    canvas_id: &str,
    name: &str,
    snapshot_data: Value,
) -> Result<CanvasSnapshot, sqlx::Error> {
    let canvas_id_int: i32 = canvas_id.parse().unwrap_or(0);
    let now = Utc::now();
    let canvas_data_str = serde_json::to_string(&snapshot_data).unwrap_or_default();

    let result = sqlx::query!(
        r#"
        INSERT INTO canvas_snapshots (
            canvas_id, snapshot_name, canvas_data, created_at
        )
        VALUES (?, ?, ?, ?)
        "#,
        canvas_id_int,
        name,
        canvas_data_str,
        now
    )
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(CanvasSnapshot {
        id,
        canvas_id: canvas_id_int,
        snapshot_name: name.to_string(),
        canvas_data: canvas_data_str,
        created_at: now,
    })
}

/// Get canvas snapshots
pub async fn get_canvas_snapshots(
    pool: &SqlitePool,
    canvas_id: &str,
) -> Result<Vec<CanvasSnapshot>, sqlx::Error> {
    let canvas_id_int: i32 = canvas_id.parse().unwrap_or(0);
    
    let results = sqlx::query!(
        r#"
        SELECT id, canvas_id, snapshot_name, canvas_data, created_at
        FROM canvas_snapshots
        WHERE canvas_id = ?
        ORDER BY created_at DESC
        "#,
        canvas_id_int
    )
    .fetch_all(pool)
    .await?;

    let mut snapshots = Vec::new();
    for row in results {
        snapshots.push(CanvasSnapshot {
            id: row.id,
            canvas_id: row.canvas_id,
            snapshot_name: row.snapshot_name,
            canvas_data: row.canvas_data,
            created_at: row.created_at,
        });
    }

    Ok(snapshots)
}

/// Create canvas collaboration session
pub async fn create_canvas_collaboration_session(
    pool: &SqlitePool,
    canvas_id: &str,
    max_participants: i32,
    expires_in_hours: Option<i32>,
) -> Result<CanvasCollaborationSession, sqlx::Error> {
    let session_token = Uuid::new_v4().to_string();
    let expires_at = expires_in_hours.map(|hours| {
        Utc::now() + chrono::Duration::hours(hours as i64)
    });
    let now = Utc::now();

    sqlx::query!(
        r#"
        INSERT INTO canvas_collaboration_sessions (
            canvas_id, session_token, is_active, max_participants, expires_at, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        canvas_id,
        session_token,
        true,
        max_participants,
        expires_at,
        now
    )
    .execute(pool)
    .await?;

    Ok(CanvasCollaborationSession {
        canvas_id: canvas_id.to_string(),
        session_token,
        is_active: true,
        max_participants,
        current_participants: 0,
        created_at: now,
        expires_at,
    })
}

/// Get canvas collaboration session
pub async fn get_canvas_collaboration_session(
    pool: &SqlitePool,
    session_token: &str,
) -> Result<Option<CanvasCollaborationSession>, sqlx::Error> {
    let result = sqlx::query_as!(
        CanvasCollaborationSession,
        r#"
        SELECT id, canvas_id, session_token, host_user, participants,
               is_active, created_at, updated_at, expires_at
        FROM canvas_collaboration_sessions
        WHERE session_token = ? AND is_active = 1
        "#,
        session_token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Get canvas collaboration session by canvas ID
pub async fn get_canvas_collaboration_session_by_canvas(
    pool: &SqlitePool,
    canvas_id: &str,
) -> Result<Option<CanvasCollaborationSession>, sqlx::Error> {
    let canvas_id_int: i32 = canvas_id.parse().unwrap_or(0);
    
    let result = sqlx::query_as!(
        CanvasCollaborationSession,
        r#"
        SELECT id, canvas_id, session_token, host_user, participants,
               is_active, created_at, updated_at, expires_at
        FROM canvas_collaboration_sessions
        WHERE canvas_id = ? AND is_active = 1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        canvas_id_int
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Create canvas collaboration session from struct
pub async fn create_canvas_collaboration_session_from_struct(
    pool: &SqlitePool,
    session: CanvasCollaborationSession,
) -> Result<CanvasCollaborationSession, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO canvas_collaboration_sessions (
            canvas_id, session_token, host_user, participants, is_active, created_at, updated_at, expires_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        session.canvas_id,
        session.session_token,
        session.host_user,
        session.participants,
        session.is_active,
        session.created_at,
        session.updated_at,
        session.expires_at
    )
    .execute(pool)
    .await?;

    Ok(session)
}

/// Update canvas collaboration session participants
pub async fn update_canvas_session_participants(
    pool: &SqlitePool,
    session_token: &str,
    participants_json: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE canvas_collaboration_sessions SET participants = ?, updated_at = CURRENT_TIMESTAMP WHERE session_token = ?",
        participants_json,
        session_token
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Export canvas data
pub async fn export_canvas(
    pool: &SqlitePool,
    canvas_id: &str,
    format: ExportFormat,
) -> Result<CanvasExportResult, sqlx::Error> {
    // Get canvas and its elements
    let canvas = get_canvas_by_id(pool, canvas_id).await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;
    let elements = get_canvas_elements(pool, canvas_id).await?;

    // Create export data structure
    let export_data = serde_json::json!({
        "canvas": canvas,
        "elements": elements,
        "exported_at": Utc::now(),
        "format": format.to_string()
    });

    Ok(CanvasExportResult {
        canvas_id: canvas_id.to_string(),
        format,
        data: export_data,
        file_size: export_data.to_string().len() as i64,
        exported_at: Utc::now(),
    })
}

/// Update canvas settings
pub async fn update_canvas(
    pool: &SqlitePool,
    canvas_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    settings: Option<Value>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    
    if let Some(n) = name {
        sqlx::query!(
            "UPDATE canvas SET name = ?, updated_at = ? WHERE id = ?",
            n, now, canvas_id
        ).execute(pool).await?;
    }
    
    if let Some(d) = description {
        sqlx::query!(
            "UPDATE canvas SET description = ?, updated_at = ? WHERE id = ?",
            d, now, canvas_id
        ).execute(pool).await?;
    }
    
    if let Some(s) = settings {
        sqlx::query!(
            "UPDATE canvas SET settings = ?, updated_at = ? WHERE id = ?",
            s.to_string(), now, canvas_id
        ).execute(pool).await?;
    }

    Ok(())
}

/// Delete canvas (soft delete)
pub async fn delete_canvas(
    pool: &SqlitePool,
    canvas_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE canvas SET is_active = 0, updated_at = ? WHERE id = ?",
        Utc::now(),
        canvas_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Record canvas operation for real-time collaboration
pub async fn record_canvas_operation(
    pool: &SqlitePool,
    operation: CanvasOperation,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO canvas_operations (
            canvas_id, operation_type, element_id, data, user_token, timestamp
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        operation.canvas_id,
        operation.operation_type.to_string(),
        operation.element_id,
        operation.data,
        operation.user_token,
        operation.timestamp
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get canvas operations for history/undo functionality
pub async fn get_canvas_operations(
    pool: &SqlitePool,
    canvas_id: &str,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<CanvasOperation>, sqlx::Error> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);
    
    let rows = sqlx::query!(
        r#"
        SELECT operation_type, element_id, data, user_token, timestamp
        FROM canvas_operations
        WHERE canvas_id = ?
        ORDER BY timestamp DESC
        LIMIT ? OFFSET ?
        "#,
        canvas_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    let mut operations = Vec::new();
    for row in rows {
        let operation_type = match row.operation_type.as_str() {
            "CreateElement" => CanvasOperationType::CreateElement,
            "UpdateElement" => CanvasOperationType::UpdateElement,
            "DeleteElement" => CanvasOperationType::DeleteElement,
            "MoveElement" => CanvasOperationType::MoveElement,
            "CreateConnection" => CanvasOperationType::CreateConnection,
            "DeleteConnection" => CanvasOperationType::DeleteConnection,
            "UpdateCanvas" => CanvasOperationType::UpdateCanvas,
            _ => continue, // Skip unknown operation types
        };
        
        operations.push(CanvasOperation {
            canvas_id: canvas_id.to_string(),
            operation_type,
            element_id: row.element_id,
            data: row.data,
            user_token: row.user_token,
            timestamp: row.timestamp.parse().unwrap_or_default(),
        });
    }

    Ok(operations)
}