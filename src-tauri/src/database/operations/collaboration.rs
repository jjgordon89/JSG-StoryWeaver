//! Database operations for collaboration features

use crate::database::models::collaboration::{
    CollaborationNotification, CollaborationSession, Comment, CommentRequest, CommentThread,
    CommentType, NotificationType, ShareSettings, ShareType, SharedDocument,
};
use crate::database::models::{Document, DocumentType};
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;
use std::str::FromStr;

/// Create a new shared document link
pub async fn create_shared_document(
    pool: &SqlitePool,
    document_id: &str,
    project_id: &str,
    share_type: ShareType,
    settings: ShareSettings,
    created_by: Option<&str>,
) -> Result<SharedDocument, sqlx::Error> {
    let share_token = Uuid::new_v4().to_string();
    let now = Utc::now();
    let share_type_str = share_type.to_string();
    let result = sqlx::query!(
        r#"
        INSERT INTO shared_documents (
            document_id, project_id, share_token, share_type,
            expires_at, is_active, created_by, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        document_id,
        project_id,
        share_token,
        share_type_str,
        settings.expires_at,
        true, // is_active
        created_by,
        now,
        now
    )
    .fetch_one(&*pool)
    .await?;

    let id = result.id
        .and_then(|id| i32::try_from(id).ok())
        .ok_or_else(|| sqlx::Error::RowNotFound)?;

    Ok(SharedDocument {
        id,
        document_id: document_id.to_string(),
        project_id: project_id.to_string(),
        share_token,
        share_type,
        password_hash: settings.password,
        expires_at: settings.expires_at,
        current_uses: 0,
        is_active: true,
        created_by: created_by.map(|s| s.to_string()),
        created_at: now,
        updated_at: now,
    })
}

/// Get shared document by token
pub async fn get_shared_document_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<SharedDocument>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, document_id, project_id, share_token, share_type, 
               password_hash, expires_at, current_uses, is_active, 
               created_by, created_at, updated_at
        FROM shared_documents
        WHERE share_token = ? AND is_active = 1
        "#,
        token
    )
    .fetch_optional(&*pool)
    .await?;

    if let Some(row) = row {
        let share_type = ShareType::from_str(&row.share_type.unwrap_or_default())
            .unwrap_or(ShareType::ReadOnly); // Default to ReadOnly if parsing fails
        
        Ok(Some(SharedDocument {
            id: row.id.map(|id| id as i32).unwrap_or(0),
            document_id: row.document_id.to_string(),
            project_id: row.project_id.to_string(),
            share_token: row.share_token,
            share_type,
            password_hash: row.password_hash,
            expires_at: row.expires_at.map(|dt| dt.and_utc()),
            current_uses: row.current_uses.unwrap_or(0) as i32,
            is_active: row.is_active.unwrap_or(false),
            created_by: row.created_by,
            created_at: row.created_at.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
            updated_at: row.updated_at.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
        }))
    } else {
        Ok(None)
    }
}

/// Increment share usage count
pub async fn increment_share_usage(
    pool: &SqlitePool,
    shared_doc_id: i32,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        UPDATE shared_documents
        SET current_uses = current_uses + 1, updated_at = ?
        WHERE id = ?
        "#,
        now,
        shared_doc_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Create a new comment
pub async fn create_comment(
    pool: &SqlitePool,
    request: CommentRequest,
) -> Result<Comment, sqlx::Error> {
    let now = Utc::now();
    let comment_type_str = request.comment_type.to_string();
    let result = sqlx::query!(
        r#"
        INSERT INTO document_comments (
            document_id, parent_comment_id, author_name, author_identifier,
            content, position_start, position_end, selected_text, comment_type,
            status, is_resolved, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        request.document_id,
        request.parent_comment_id,
        request.author_name,
        request.author_identifier,
        request.content,
        request.position_start,
        request.position_end,
        request.selected_text,
        comment_type_str,
        "open", // default status
        false, // default is_resolved
        now,
        now
    )
    .fetch_one(&*pool)
    .await?;

    let id = result.id
        .and_then(|id| i64::try_from(id).ok())
        .unwrap_or(0) as i32;

    Ok(Comment {
        id,
        document_id: request.document_id,
        parent_comment_id: request.parent_comment_id,
        author_name: request.author_name,
        author_identifier: request.author_identifier,
        content: request.content,
        position_start: request.position_start,
        position_end: request.position_end,
        selected_text: request.selected_text,
        comment_type: request.comment_type,
        status: "open".to_string(),
        is_resolved: false,
        resolved_by: None,
        resolved_at: None,
        created_at: now,
        updated_at: now,
    })
}

/// Get comments for a document
pub async fn get_document_comments(
    pool: &SqlitePool,
    document_id: &str,
) -> Result<Vec<Comment>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, document_id, parent_comment_id, author_name, author_identifier,
               content, position_start, position_end, selected_text, comment_type,
               status, is_resolved, resolved_by, resolved_at, created_at, updated_at
        FROM document_comments
        WHERE document_id = ?
        ORDER BY created_at ASC
        "#,
        document_id
    )
    .fetch_all(&*pool)
    .await?;

    let comments = rows.into_iter().map(|row| {
        Comment {
            id: row.id.unwrap_or(0) as i32,
            document_id: row.document_id.to_string(),
            parent_comment_id: row.parent_comment_id.map(|id| id as i32),
            author_name: row.author_name,
            author_identifier: Some(row.author_identifier),
            content: row.content,
            position_start: row.position_start.map(|pos| pos as i32),
            position_end: row.position_end.map(|pos| pos as i32),
            selected_text: row.selected_text,
            comment_type: CommentType::from_str(&row.comment_type.unwrap_or_default()).unwrap_or_default(),
            status: row.status.unwrap_or_default(),
            is_resolved: row.is_resolved.unwrap_or(false),
            resolved_by: row.resolved_by,
            resolved_at: row.resolved_at.map(|dt| dt.and_utc()),
            created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now().naive_utc()).and_utc(),
            updated_at: row.updated_at.unwrap_or_else(|| chrono::Utc::now().naive_utc()).and_utc(),
        }
    }).collect();

    Ok(comments)
}

/// Resolve a comment
pub async fn resolve_comment(
    pool: &SqlitePool,
    comment_id: i32,
    resolved_by: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        UPDATE document_comments 
        SET is_resolved = 1, status = 'resolved', resolved_by = ?, resolved_at = ?, updated_at = ?
        WHERE id = ?
        "#,
        resolved_by,
        now,
        now,
        comment_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Delete a comment
pub async fn delete_comment(
    pool: &SqlitePool,
    comment_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM document_comments WHERE id = ?", comment_id)
        .execute(&*pool)
        .await?;

    Ok(())
}

/// Create a collaboration session
pub async fn create_collaboration_session(
    pool: &SqlitePool,
    document_id: &str,
    max_participants: i32,
    expires_in_hours: Option<i32>,
) -> Result<CollaborationSession, sqlx::Error> {
    let now = Utc::now();
    let session_token = Uuid::new_v4().to_string();
    let expires_at = expires_in_hours.map(|hours| {
        now + chrono::Duration::hours(hours as i64)
    });

    let result = sqlx::query!(
        r#"
        INSERT INTO collaboration_sessions (
            document_id, session_token, max_participants, expires_at, created_at
        )
        VALUES (?, ?, ?, ?, ?)
        RETURNING id
        "#,
        document_id,
        session_token,
        max_participants,
        expires_at,
        now
    )
    .fetch_one(&*pool)
    .await?;

    let id = result.id
        .and_then(|id| i32::try_from(id).ok())
        .unwrap_or(0);

    Ok(CollaborationSession {
        id,
        document_id: document_id.to_string(),
        session_token,
        is_active: true,
        max_participants,
        current_participants: 0,
        created_at: now,
        expires_at,
    })
}

/// Get collaboration session by token
pub async fn get_collaboration_session_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<CollaborationSession>, sqlx::Error> {
    sqlx::query_as(
        r#"
        SELECT id, document_id, session_token, is_active,
               max_participants, current_participants, created_at, expires_at
        FROM collaboration_sessions
        WHERE session_token = ? AND is_active = 1
        "#,
    )
    .bind(token)
    .fetch_optional(&*pool)
    .await
}

/// Get comment threads (organized by parent-child relationships)
pub async fn get_comment_threads(
    pool: &SqlitePool,
    document_id: &str,
) -> Result<Vec<CommentThread>, sqlx::Error> {
    let comments = get_document_comments(&*pool, document_id).await?;
    let mut threads = Vec::new();
    let mut comment_map: HashMap<i32, Comment> = HashMap::new();
    let mut children_map: HashMap<Option<i32>, Vec<i32>> = HashMap::new();

    // Build maps for efficient lookup
    for comment in comments {
        children_map.entry(comment.parent_comment_id)
            .or_insert_with(Vec::new)
            .push(comment.id);
        comment_map.insert(comment.id, comment);
    }

    // Build threads starting from root comments (no parent)
    if let Some(root_comment_ids) = children_map.get(&None) {
        for &root_id in root_comment_ids {
            if let Some(root_comment) = comment_map.get(&root_id) {
                let replies = build_comment_replies(&comment_map, &children_map, root_id);
                threads.push(CommentThread {
                    parent_comment: root_comment.clone(),
                    replies,
                    total_replies: count_total_replies(&children_map, root_id),
                });
            }
        }
    }

    Ok(threads)
}

/// Helper function to build comment replies recursively
fn build_comment_replies(
    comment_map: &HashMap<i32, Comment>,
    children_map: &HashMap<Option<i32>, Vec<i32>>,
    parent_id: i32,
) -> Vec<Comment> {
    let mut replies = Vec::new();
    
    if let Some(child_ids) = children_map.get(&Some(parent_id)) {
        for &child_id in child_ids {
            if let Some(child_comment) = comment_map.get(&child_id) {
                replies.push(child_comment.clone());
                // Recursively add nested replies
                replies.extend(build_comment_replies(comment_map, children_map, child_id));
            }
        }
    }
    
    replies
}

/// Helper function to count total replies in a thread
fn count_total_replies(
    children_map: &HashMap<Option<i32>, Vec<i32>>,
    parent_id: i32,
) -> i32 {
    let mut count = 0;
    
    if let Some(child_ids) = children_map.get(&Some(parent_id)) {
        count += child_ids.len() as i32;
        for &child_id in child_ids {
            count += count_total_replies(children_map, child_id);
        }
    }
    
    count
}

/// Duplicate document for sharing (creates a copy)
pub async fn duplicate_document_for_sharing(
    pool: &SqlitePool,
    original_document_id: &str,
    new_title: &str,
) -> Result<String, sqlx::Error> {
    // Get original document
    let row = sqlx::query!(
        r#"
        SELECT id, project_id, title, content, document_type,
               order_index, word_count, parent_id, created_at, updated_at, metadata, folder_id
        FROM documents
        WHERE id = ?
        "#,
        original_document_id
    )
    .fetch_one(&*pool)
    .await?;
    
    let document_type = DocumentType::from_str(&row.document_type)
        .unwrap_or(DocumentType::Chapter); // Default to Chapter if parsing fails
    
    let original = Document {
            id: row.id.unwrap_or_default(),
            project_id: row.project_id.to_string(),
            title: row.title,
            content: row.content,
            document_type,
            order_index: row.order_index as i32,
            word_count: row.word_count as i32,
            parent_id: row.parent_id,
            created_at: row.created_at.and_utc(),
            updated_at: row.updated_at.and_utc(),
            metadata: row.metadata,
            folder_id: row.folder_id,
    };

    // Create new document with copied content
    let new_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    let document_type_str = original.document_type.to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO documents (
            id, project_id, title, content, document_type, order_index,
            word_count, parent_id, created_at, updated_at, metadata, folder_id
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        new_id,
        original.project_id,
        new_title,
        original.content,
        document_type_str,
        original.order_index,
        original.word_count,
        original.parent_id,
        now,
        now,
        original.metadata,
        original.folder_id
    )
    .execute(&*pool)
    .await?;

    Ok(new_id)
}

/// Unpublish a shared document (set is_active to false)
pub async fn unpublish_shared_document(
    pool: &SqlitePool,
    share_token: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        UPDATE shared_documents
        SET is_active = false, updated_at = ?
        WHERE share_token = ?
        "#,
        now,
        share_token
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Republish a shared document (set is_active to true)
pub async fn republish_shared_document(
    pool: &SqlitePool,
    share_token: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        UPDATE shared_documents
        SET is_active = true, updated_at = ?
        WHERE share_token = ?
        "#,
        now,
        share_token
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Get all shared documents for a project
pub async fn get_project_shared_documents(
    pool: &SqlitePool,
    project_id: &str,
) -> Result<Vec<SharedDocument>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, document_id, project_id, share_token, share_type, 
               password_hash, expires_at, current_uses, is_active, 
               created_by, created_at, updated_at
        FROM shared_documents
        WHERE project_id = ?
        ORDER BY created_at DESC
        "#,
        project_id
    )
    .fetch_all(&*pool)
    .await?;
    
    let shared_documents = rows
        .into_iter()
        .map(|row| SharedDocument {
            id: row.id.unwrap_or(0) as i32,
            document_id: row.document_id.to_string(),
            project_id: row.project_id.to_string(),
            share_token: row.share_token,
            share_type: ShareType::from_str(&row.share_type.unwrap_or_default()).unwrap_or_default(),
            password_hash: row.password_hash,
            expires_at: row.expires_at.map(|dt| dt.and_utc()),
            current_uses: row.current_uses.unwrap_or(0) as i32,
            is_active: row.is_active.unwrap_or(false),
            created_by: row.created_by,
            created_at: row.created_at.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
            updated_at: row.updated_at.map(|dt| dt.and_utc()).unwrap_or_else(|| Utc::now()),
        })
        .collect();
    
    Ok(shared_documents)
}

/// Create a new collaboration notification
pub async fn create_notification(
    pool: &SqlitePool,
    document_id: &str,
    notification_type: NotificationType,
    message: &str,
    recipient_token: Option<&str>,
) -> Result<CollaborationNotification, sqlx::Error> {
    let now = Utc::now();
    let notification_type_str = notification_type.to_string();
    let result = sqlx::query!(
        r#"
        INSERT INTO collaboration_notifications (
            document_id, notification_type, message, recipient_token, is_read, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        document_id,
        notification_type_str,
        message,
        recipient_token,
        false, // is_read starts as false
        now
    )
    .fetch_one(&*pool)
    .await?;

    Ok(CollaborationNotification {
        id: result.id
            .and_then(|id| i32::try_from(id).ok())
            .unwrap_or(0),
        document_id: document_id.to_string(),
        notification_type,
        message: message.to_string(),
        recipient_token: recipient_token.map(|s| s.to_string()),
        is_read: false,
        created_at: now,
    })
}

/// Get notifications for a user/token
pub async fn get_notifications_for_user(
    pool: &SqlitePool,
    recipient_token: &str,
    limit: Option<i32>,
) -> Result<Vec<CollaborationNotification>, sqlx::Error> {
    let limit = limit.unwrap_or(50);
    
    let rows = sqlx::query!(
        r#"
        SELECT id, document_id, notification_type, message, recipient_token, is_read, created_at
        FROM collaboration_notifications
        WHERE recipient_token = ?
        ORDER BY created_at DESC
        LIMIT ?
        "#,
        recipient_token,
        limit
    )
    .fetch_all(&*pool)
    .await?;

    let results = rows
        .into_iter()
        .map(|row| {
            let notification_type = NotificationType::from_str(&row.notification_type)
                .unwrap_or(NotificationType::default());
            
            CollaborationNotification {
                id: row.id.unwrap_or(0) as i32,
                document_id: row.document_id,
                notification_type,
                message: row.message,
                recipient_token: row.recipient_token,
                is_read: row.is_read,
                created_at: row.created_at.and_utc(),
            }
        })
        .collect();

    Ok(results)
}

/// Mark notification as read
pub async fn mark_notification_read(
    pool: &SqlitePool,
    notification_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE collaboration_notifications SET is_read = true WHERE id = ?",
        notification_id
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Mark all notifications as read for a user
pub async fn mark_all_notifications_read(
    pool: &SqlitePool,
    recipient_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE collaboration_notifications SET is_read = true WHERE recipient_token = ?",
        recipient_token
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

/// Get unread notification count for a user
pub async fn get_unread_notification_count(
    pool: &SqlitePool,
    recipient_token: &str,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM collaboration_notifications WHERE recipient_token = ? AND is_read = false",
        recipient_token
    )
    .fetch_one(&*pool)
    .await?;

    Ok(i32::try_from(result.count)
        .map_err(|_| sqlx::Error::ColumnDecode { index: "count".to_string(), source: Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "Count conversion failed")) })?
        .max(0))
}

/// Delete old notifications (cleanup)
pub async fn delete_old_notifications(
    pool: &SqlitePool,
    days_old: i32,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let cutoff_date = now - chrono::Duration::days(days_old as i64);
    
    sqlx::query!(
        "DELETE FROM collaboration_notifications WHERE created_at < ?",
        cutoff_date
    )
    .execute(&*pool)
    .await?;

    Ok(())
}
