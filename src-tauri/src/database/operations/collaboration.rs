//! Database operations for collaboration features

use crate::database::models::*;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;

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
    let expires_at = settings.expires_at;

    let result = sqlx::query!(
        r#"
        INSERT INTO shared_documents (
            document_id, project_id, share_token, share_type,
            expires_at, max_uses, current_uses, is_active, created_by, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        document_id,
        project_id,
        share_token,
        share_type.to_string(),
        expires_at,
        settings.max_uses,
        0, // current_uses starts at 0
        true, // is_active
        created_by,
        Utc::now(),
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(SharedDocument {
        id: result.id,
        document_id: document_id.to_string(),
        project_id: project_id.to_string(),
        share_token,
        share_type: share_type.to_string(),
        password_hash: settings.password.map(|p| format!("hashed_{}", p)), // Simple hash for now
        expires_at: settings.expires_at,
        max_uses: settings.max_uses,
        current_uses: 0,
        is_active: true,
        created_by: created_by.map(|s| s.to_string()),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

/// Get shared document by token
pub async fn get_shared_document_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<SharedDocument>, sqlx::Error> {
    let result = sqlx::query_as!(
        SharedDocument,
        r#"
        SELECT id, document_id as "document_id: String", project_id as "project_id: String", 
               share_token as "share_token: String", share_type as "share_type: String", 
               password_hash, expires_at, max_uses, current_uses, is_active, 
               created_by, created_at, updated_at
        FROM shared_documents
        WHERE share_token = ? AND is_active = 1
        "#,
        token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Increment share usage count
pub async fn increment_share_usage(
    pool: &SqlitePool,
    shared_doc_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE shared_documents
        SET current_uses = current_uses + 1, updated_at = ?
        WHERE id = ?
        "#,
        Utc::now(),
        shared_doc_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a new comment
pub async fn create_comment(
    pool: &SqlitePool,
    request: CommentRequest,
) -> Result<Comment, sqlx::Error> {
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
        request.comment_type.to_string(),
        "open", // default status
        false, // default is_resolved
        Utc::now(),
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(Comment {
        id: result.id,
        document_id: request.document_id.clone(),
        parent_comment_id: request.parent_comment_id,
        author_name: request.author_name.clone(),
        author_identifier: request.author_identifier.clone(),
        content: request.content.clone(),
        position_start: request.position_start,
        position_end: request.position_end,
        selected_text: request.selected_text.clone(),
        comment_type: request.comment_type.to_string(),
        status: "open".to_string(),
        is_resolved: false,
        resolved_by: None,
        resolved_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

/// Get comments for a document
pub async fn get_document_comments(
    pool: &SqlitePool,
    document_id: &str,
) -> Result<Vec<Comment>, sqlx::Error> {
    let comments = sqlx::query_as!(
        Comment,
        r#"
        SELECT id, document_id, parent_comment_id, user_name, user_token,
               comment_text, start_position, end_position, is_author_comment,
               is_resolved, created_at, updated_at
        FROM document_comments
        WHERE document_id = ?
        ORDER BY created_at ASC
        "#,
        document_id
    )
    .fetch_all(pool)
    .await?;

    Ok(comments)
}

/// Resolve a comment
pub async fn resolve_comment(
    pool: &SqlitePool,
    comment_id: i32,
    resolved_by: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE document_comments 
        SET is_resolved = 1, status = 'resolved', resolved_by = ?, resolved_at = ?, updated_at = ?
        WHERE id = ?
        "#,
        resolved_by,
        Utc::now(),
        Utc::now(),
        comment_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a comment
pub async fn delete_comment(
    pool: &SqlitePool,
    comment_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM document_comments WHERE id = ?", comment_id)
        .execute(pool)
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
    let session_token = Uuid::new_v4().to_string();
    let expires_at = expires_in_hours.map(|hours| {
        Utc::now() + chrono::Duration::hours(hours as i64)
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
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(CollaborationSession {
        id: result.id,
        document_id: document_id.to_string(),
        session_token,
        session_name: None,
        is_active: true,
        allow_anonymous: false,
        max_participants,
        created_at: Utc::now(),
        expires_at,
    })
}

/// Get collaboration session by token
pub async fn get_collaboration_session_by_token(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<CollaborationSession>, sqlx::Error> {
    let result = sqlx::query_as!(
        CollaborationSession,
        r#"
        SELECT id, document_id, session_token, is_active, max_participants,
               current_participants, created_at, expires_at
        FROM collaboration_sessions
        WHERE session_token = ? AND is_active = 1
        "#,
        token
    )
    .fetch_optional(pool)
    .await?;

    Ok(result)
}

/// Update collaboration session participant count
pub async fn update_session_participants(
    pool: &SqlitePool,
    session_id: i32,
    participant_count: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE collaboration_sessions SET current_participants = ? WHERE id = ?",
        participant_count,
        session_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get comment threads (organized by parent-child relationships)
pub async fn get_comment_threads(
    pool: &SqlitePool,
    document_id: &str,
) -> Result<Vec<CommentThread>, sqlx::Error> {
    let comments = get_document_comments(pool, document_id).await?;
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
    let original = sqlx::query_as!(
        Document,
        r#"
        SELECT id as "id: String", project_id as "project_id: String", title as "title: String", content as "content: String", document_type as "document_type: DocumentType",
               order_index as "order_index: i32", word_count as "word_count: i32", parent_id as "parent_id: Option<String>", created_at, updated_at, metadata, folder_id as "folder_id: Option<String>"
        FROM documents
        WHERE id = ?
        "#,
        original_document_id
    )
    .fetch_one(pool)
    .await?;

    // Create new document with copied content
    let new_id = Uuid::new_v4().to_string();
    let now = Utc::now();
    
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
        original.document_type.to_string(),
        original.order_index,
        original.word_count,
        original.parent_id,
        now,
        now,
        original.metadata,
        original.folder_id
    )
    .execute(pool)
    .await?;

    Ok(new_id)
}

/// Unpublish a shared document (set is_active to false)
pub async fn unpublish_shared_document(
    pool: &SqlitePool,
    share_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE shared_documents
        SET is_active = 0, updated_at = ?
        WHERE share_token = ?
        "#,
        Utc::now(),
        share_token
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Republish a shared document (set is_active to true)
pub async fn republish_shared_document(
    pool: &SqlitePool,
    share_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE shared_documents
        SET is_active = 1, updated_at = ?
        WHERE share_token = ?
        "#,
        Utc::now(),
        share_token
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get all shared documents for a project
pub async fn get_project_shared_documents(
    pool: &SqlitePool,
    project_id: &str,
) -> Result<Vec<SharedDocument>, sqlx::Error> {
    let results = sqlx::query_as!(
        SharedDocument,
        r#"
        SELECT id, document_id as "document_id: String", project_id as "project_id: String", 
               share_token as "share_token: String", share_type as "share_type: String", 
               password_hash, expires_at, max_uses, current_uses, is_active, 
               created_by, created_at, updated_at
        FROM shared_documents
        WHERE project_id = ?
        ORDER BY created_at DESC
        "#,
        project_id
    )
    .fetch_all(pool)
    .await?;

    Ok(results)
}

/// Create a new collaboration notification
pub async fn create_notification(
    pool: &SqlitePool,
    document_id: &str,
    notification_type: NotificationType,
    message: &str,
    recipient_token: Option<&str>,
) -> Result<CollaborationNotification, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO collaboration_notifications (
            document_id, notification_type, message, recipient_token, is_read, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        RETURNING id
        "#,
        document_id,
        notification_type.to_string(),
        message,
        recipient_token,
        false, // is_read starts as false
        Utc::now()
    )
    .fetch_one(pool)
    .await?;

    Ok(CollaborationNotification {
        id: result.id,
        document_id: document_id.to_string(),
        notification_type,
        message: message.to_string(),
        recipient_token: recipient_token.map(|s| s.to_string()),
        is_read: false,
        created_at: Utc::now(),
    })
}

/// Get notifications for a user/token
pub async fn get_notifications_for_user(
    pool: &SqlitePool,
    recipient_token: &str,
    limit: Option<i32>,
) -> Result<Vec<CollaborationNotification>, sqlx::Error> {
    let limit = limit.unwrap_or(50);
    
    let results = sqlx::query!(
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
    .fetch_all(pool)
    .await?;

    let notifications = results
        .into_iter()
        .map(|row| {
            let notification_type = match row.notification_type.as_str() {
                "new_comment" => NotificationType::NewComment,
                "comment_reply" => NotificationType::CommentReply,
                "comment_resolved" => NotificationType::CommentResolved,
                "participant_joined" => NotificationType::ParticipantJoined,
                "document_updated" => NotificationType::DocumentUpdated,
                _ => NotificationType::NewComment, // Default fallback
            };
            
            CollaborationNotification {
                id: row.id,
                document_id: row.document_id,
                notification_type,
                message: row.message,
                recipient_token: row.recipient_token,
                is_read: row.is_read,
                created_at: row.created_at,
            }
        })
        .collect();

    Ok(notifications)
}

/// Mark notification as read
pub async fn mark_notification_read(
    pool: &SqlitePool,
    notification_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE collaboration_notifications SET is_read = 1 WHERE id = ?",
        notification_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark all notifications as read for a user
pub async fn mark_all_notifications_read(
    pool: &SqlitePool,
    recipient_token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "UPDATE collaboration_notifications SET is_read = 1 WHERE recipient_token = ?",
        recipient_token
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get unread notification count for a user
pub async fn get_unread_notification_count(
    pool: &SqlitePool,
    recipient_token: &str,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM collaboration_notifications WHERE recipient_token = ? AND is_read = 0",
        recipient_token
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count)
}

/// Delete old notifications (cleanup)
pub async fn delete_old_notifications(
    pool: &SqlitePool,
    days_old: i32,
) -> Result<(), sqlx::Error> {
    let cutoff_date = Utc::now() - chrono::Duration::days(days_old as i64);
    
    sqlx::query!(
        "DELETE FROM collaboration_notifications WHERE created_at < ?",
        cutoff_date
    )
    .execute(pool)
    .await?;

    Ok(())
}