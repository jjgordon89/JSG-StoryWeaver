//! Database operations for canvas and visual story planning features

use crate::database::models::canvas::{
    Canvas, CanvasCollaborationSession, CanvasElement, CanvasOperation, CanvasSnapshot,
    OutlineTemplate, OutlineTemplateType,
};
use crate::database::models::canvas::{CanvasElementType, CanvasExportResult, ExportFormat};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, Row, SqlitePool};
use uuid::Uuid;

/// Create a new canvas
pub async fn create_canvas(
    pool: &SqlitePool,
    project_id: &str,
    name: &str,
    description: Option<&str>,
) -> Result<Canvas, sqlx::Error> {
    let now = Utc::now();
    let naive_now = now.naive_utc();

    let result = sqlx::query!(
        r#"
        INSERT INTO canvas (
            project_id, name, description, canvas_data, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        project_id,
        name,
        description,
        "[]", // Empty canvas data initially
        naive_now,
        naive_now
    )
    .execute(&*pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(Canvas {
        id,
        project_id: project_id.to_string(),
        name: name.to_string(),
        description: description.map(|s| s.to_string()),
        canvas_data: "[]".to_string(),
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
    canvas_id: i32,
) -> Result<Option<Canvas>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, project_id, name, description, canvas_data, template_type, width, height,
               zoom_level, viewport_x, viewport_y, created_at, updated_at
        FROM canvas
        WHERE id = ?
        "#,
    )
    .bind(canvas_id)
    .fetch_optional(&*pool)
    .await
}

/// Get canvases for a project
pub async fn get_project_canvases(
    pool: &SqlitePool,
    project_id: &str,
) -> Result<Vec<Canvas>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, project_id, name, description, canvas_data, template_type, width, height,
               zoom_level, viewport_x, viewport_y, created_at, updated_at
        FROM canvas
        WHERE project_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(project_id)
    .fetch_all(&*pool)
    .await
}

/// Create a canvas element
pub async fn create_canvas_element(
    pool: &SqlitePool,
    canvas_id: i32,
    element_type: CanvasElementType,
    title: &str,
    content: &str,
    position_x: f32,
    position_y: f32,
    width: f32,
    height: f32,
    color: &str,
    metadata: &str,
    connections: &str,
    order_index: i32,
) -> Result<CanvasElement, sqlx::Error> {
    let now = Utc::now();
    let naive_now = Utc::now().naive_utc();
    let element_type_str = element_type.to_string();

    let result = sqlx::query!(
        r#"
        INSERT INTO canvas_elements (
            canvas_id, element_type, title, content, position_x, position_y, width, height,
            color, metadata, connections, order_index, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        canvas_id,
        element_type_str,
        title,
        content,
        position_x,
        position_y,
        width,
        height,
        color,
        metadata,
        connections,
        order_index,
        naive_now,
        naive_now
    )
    .execute(&*pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(CanvasElement {
        id,
        canvas_id,
        element_type,
        title: title.to_string(),
        content: content.to_string(),
        position_x,
        position_y,
        width,
        height,
        color: color.to_string(),
        metadata: metadata.to_string(),
        connections: connections.to_string(),
        order_index,
        created_at: now,
        updated_at: now,
    })
}

/// Get canvas elements
pub async fn get_canvas_elements(
    pool: &SqlitePool,
    canvas_id: i32,
) -> Result<Vec<CanvasElement>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, canvas_id, element_type, title, content, position_x, position_y, width, height,
               color, metadata, connections, order_index, created_at, updated_at
        FROM canvas_elements
        WHERE canvas_id = ?
        ORDER BY order_index ASC, created_at ASC
        "#,
    )
    .bind(canvas_id)
    .fetch_all(&*pool)
    .await
}

/// Update canvas element
pub async fn update_canvas_element(
    pool: &SqlitePool,
    element_id: i32,
    position_x: Option<f32>,
    position_y: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    content: Option<&str>,
    color: Option<&str>,
    title: Option<&str>,
    order_index: Option<i32>,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().naive_utc();
    let mut query_builder = sqlx::QueryBuilder::new("UPDATE canvas_elements SET ");
    let mut separated = query_builder.separated(", ");

    if let Some(x) = position_x {
        separated.push("position_x = ");
        separated.push_bind_unseparated(x);
    }
    if let Some(y) = position_y {
        separated.push("position_y = ");
        separated.push_bind_unseparated(y);
    }
    if let Some(w) = width {
        separated.push("width = ");
        separated.push_bind_unseparated(w);
    }
    if let Some(h) = height {
        separated.push("height = ");
        separated.push_bind_unseparated(h);
    }
    if let Some(c) = content {
        separated.push("content = ");
        separated.push_bind_unseparated(c);
    }
    if let Some(c) = color {
        separated.push("color = ");
        separated.push_bind_unseparated(c);
    }
    if let Some(t) = title {
        separated.push("title = ");
        separated.push_bind_unseparated(t);
    }
    if let Some(oi) = order_index {
        separated.push("order_index = ");
        separated.push_bind_unseparated(oi);
    }

    separated.push("updated_at = ");
    separated.push_bind_unseparated(now);

    query_builder.push(" WHERE id = ");
    query_builder.push_bind(element_id);

    query_builder.build().execute(&*pool).await?;

    Ok(())
}

/// Delete canvas element
pub async fn delete_canvas_element(
    pool: &SqlitePool,
    element_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM canvas_elements WHERE id = ?", element_id)
        .execute(&*pool)
        .await?;
    Ok(())
}

/// Create outline template
pub async fn create_outline_template(
    pool: &SqlitePool,
    name: &str,
    description: &str,
    template_type: OutlineTemplateType,
    structure: &str,
    is_official: bool,
) -> Result<OutlineTemplate, sqlx::Error> {
    let now = Utc::now();
    let naive_now = now.naive_utc();

    let result = sqlx::query!(
        r#"
        INSERT INTO outline_templates (
            name, description, template_type, template_data, is_official, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        name,
        description,
        template_type.to_string(),
        structure,
        is_official,
        naive_now
    )
    .execute(&*pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(OutlineTemplate {
        id,
        name: name.to_string(),
        description: description.to_string(),
        template_type,
        template_data: structure.to_string(),
        is_official,
        created_at: now,
    })
}

/// Get outline templates
pub async fn get_outline_templates(
    pool: &SqlitePool,
    template_type: Option<OutlineTemplateType>,
) -> Result<Vec<OutlineTemplate>, sqlx::Error> {
    let mut builder = sqlx::QueryBuilder::new(
        "SELECT id, name, description, template_type, template_data, is_official, created_at FROM outline_templates",
    );

    if let Some(t_type) = template_type {
        builder.push(" WHERE template_type = ");
        builder.push_bind(t_type.to_string());
    }

    builder.push(" ORDER BY created_at DESC");

    builder.build_query_as().fetch_all(&*pool).await
}

/// Increment template usage count
pub async fn increment_template_usage(
    pool: &SqlitePool,
    template_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE outline_templates SET usage_count = usage_count + 1 WHERE id = ?",
        template_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Create canvas snapshot
pub async fn create_canvas_snapshot(
    pool: &SqlitePool,
    canvas_id: i32,
    name: &str,
    snapshot_data: &str,
) -> Result<CanvasSnapshot, sqlx::Error> {
    let now = Utc::now();
    let naive_now = now.naive_utc();

    let result = sqlx::query!(
        r#"
        INSERT INTO canvas_snapshots (canvas_id, snapshot_name, canvas_data, created_at)
        VALUES (?, ?, ?, ?)
        "#,
        canvas_id,
        name,
        snapshot_data,
        naive_now
    )
    .execute(&*pool)
    .await?;

    let id = result.last_insert_rowid() as i32;

    Ok(CanvasSnapshot {
        id,
        canvas_id,
        snapshot_name: name.to_string(),
        canvas_data: snapshot_data.to_string(),
        created_at: now,
    })
}

/// Get canvas snapshots
pub async fn get_canvas_snapshots(
    pool: &SqlitePool,
    canvas_id: i32,
) -> Result<Vec<CanvasSnapshot>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, canvas_id, snapshot_name, canvas_data, created_at
        FROM canvas_snapshots
        WHERE canvas_id = ?
        ORDER BY created_at DESC
        "#,
    )
    .bind(canvas_id)
    .fetch_all(&*pool)
    .await
}

/// Restore canvas snapshot
pub async fn restore_canvas_snapshot(
    pool: &SqlitePool,
    snapshot_id: i32,
) -> Result<(), sqlx::Error> {
    let snapshot = sqlx::query_as::<_, CanvasSnapshot>(
        "SELECT * FROM canvas_snapshots WHERE id = ?",
    )
    .bind(snapshot_id)
    .fetch_one(&*pool)
    .await?;

    let now = Utc::now().naive_utc();
    sqlx::query!(
        "UPDATE canvas SET canvas_data = ?, updated_at = ? WHERE id = ?",
        snapshot.canvas_data,
        now,
        snapshot.canvas_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Create canvas collaboration session
pub async fn create_canvas_collaboration_session(
    pool: &SqlitePool,
    canvas_id: i32,
    max_participants: i32,
    expires_in_hours: Option<i64>,
) -> Result<CanvasCollaborationSession, sqlx::Error> {
    let session_token = Uuid::new_v4().to_string();
    let now = Utc::now();
    let naive_now = now.naive_utc();
    let expires_at = expires_in_hours.map(|h| naive_now + chrono::Duration::hours(h));

    let result = sqlx::query!(
        r#"
        INSERT INTO canvas_collaboration_sessions (canvas_id, session_token, is_active, max_participants, created_at, expires_at)
        VALUES (?, ?, 1, ?, ?, ?)
        "#,
        canvas_id,
        session_token,
        max_participants,
        naive_now,
        expires_at,
    )
    .execute(&*pool)
    .await?;

    Ok(CanvasCollaborationSession {
        id: result.last_insert_rowid(),
        canvas_id,
        session_token,
        is_active: true,
        max_participants,
        current_participants: 0,
        host_user: String::new(),
        participants: String::new(),
        created_at: now,
        updated_at: now,
        expires_at: expires_at.map(|ndt| DateTime::from_naive_utc_and_offset(ndt, Utc)),
    })
}

/// Get canvas collaboration session by token
pub async fn get_canvas_collaboration_session_by_token(
    pool: &SqlitePool,
    session_token: &str,
) -> Result<Option<CanvasCollaborationSession>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT * FROM canvas_collaboration_sessions WHERE session_token = ? AND is_active = 1
        "#,
    )
    .bind(session_token)
    .fetch_optional(&*pool)
    .await
}

/// Get active canvas collaboration session by canvas ID
pub async fn get_canvas_collaboration_session_by_canvas_id(
    pool: &SqlitePool,
    canvas_id: i32,
) -> Result<Option<CanvasCollaborationSession>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT * FROM canvas_collaboration_sessions
        WHERE canvas_id = ? AND is_active = 1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(canvas_id)
    .fetch_optional(&*pool)
    .await
}

/// Update canvas collaboration session
pub async fn update_canvas_collaboration_session(
    pool: &SqlitePool,
    session_id: i64,
    is_active: Option<bool>,
    current_participants: Option<i32>,
) -> Result<(), sqlx::Error> {
    let mut builder = sqlx::QueryBuilder::new("UPDATE canvas_collaboration_sessions SET ");
    let mut separated = builder.separated(", ");

    if let Some(active) = is_active {
        separated.push("is_active = ");
        separated.push_bind_unseparated(active);
    }

    if let Some(participants) = current_participants {
        separated.push("current_participants = ");
        separated.push_bind_unseparated(participants);
    }

    separated.push("updated_at = ");
    separated.push_bind_unseparated(Utc::now().naive_utc());

    builder.push(" WHERE id = ");
    builder.push_bind(session_id);

    builder.build().execute(&*pool).await?;

    Ok(())
}


/// Export canvas data
pub async fn export_canvas(
    pool: &SqlitePool,
    canvas_id: i32,
    format: ExportFormat,
) -> Result<CanvasExportResult, sqlx::Error> {
    let canvas = get_canvas_by_id(&*pool, canvas_id).await?
        .ok_or_else(|| sqlx::Error::RowNotFound)?;
    let elements = get_canvas_elements(&*pool, canvas_id).await?;

    let export_data = serde_json::json!({
        "canvas": canvas,
        "elements": elements,
    });

    let data_string = export_data.to_string();

    Ok(CanvasExportResult {
        canvas_id: canvas_id.to_string(),
        format,
        data: data_string.clone(),
        file_size: data_string.len() as i64,
        exported_at: Utc::now(),
    })
}

/// Update canvas settings
pub async fn update_canvas(
    pool: &SqlitePool,
    canvas_id: i32,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<(), sqlx::Error> {
    let mut builder = sqlx::QueryBuilder::new("UPDATE canvas SET ");
    let mut separated = builder.separated(", ");

    if let Some(n) = name {
        separated.push("name = ");
        separated.push_bind_unseparated(n);
    }
    if let Some(d) = description {
        separated.push("description = ");
        separated.push_bind_unseparated(d);
    }
    
    separated.push("updated_at = ");
    separated.push_bind_unseparated(Utc::now().naive_utc());

    builder.push(" WHERE id = ");
    builder.push_bind(canvas_id);

    builder.build().execute(&*pool).await?;

    Ok(())
}

/// Delete canvas (soft delete not implemented, this is a hard delete)
pub async fn delete_canvas(
    pool: &SqlitePool,
    canvas_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM canvas WHERE id = ?", canvas_id)
        .execute(&*pool)
        .await?;
    Ok(())
}

/// Record canvas operation for real-time collaboration
pub async fn record_canvas_operation(
    pool: &SqlitePool,
    operation: &CanvasOperation,
) -> Result<(), sqlx::Error> {
    let operation_type_str = operation.operation_type.to_string();
    sqlx::query!(
        r#"
        INSERT INTO canvas_operations (
            id, canvas_id, operation_type, element_id, data, user_token, timestamp
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        operation.id,
        operation.canvas_id,
        operation_type_str,
        operation.element_id,
        operation.data,
        operation.user_token,
        operation.timestamp
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Get canvas operations for history/undo functionality
pub async fn get_canvas_operations(
    pool: &SqlitePool,
    canvas_id: i32,
    limit: i32,
    offset: i32,
) -> Result<Vec<CanvasOperation>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT * FROM canvas_operations
        WHERE canvas_id = ?
        ORDER BY timestamp DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(canvas_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&*pool)
    .await
}
