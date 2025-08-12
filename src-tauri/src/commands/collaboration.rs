//! Tauri commands for collaboration features

use crate::database::{
    get_pool,
    models::{
        collaboration::{
            CollaborationNotification, CollaborationSession, Comment, CommentRequest,
            CommentThread, CommentType, NotificationType, ShareSettings, ShareType, SharedDocument,
        },
        Document,
    },
    operations::collaboration as collaboration_ops,
};
use crate::error::{Result, StoryWeaverError};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use std::str::FromStr;

/// Create a shared document link
#[tauri::command]
pub async fn create_shared_document_link(
    document_id: String,
    project_id: String,
    share_type: String,
    password: Option<String>,
    expires_in_hours: Option<i32>,
) -> Result<SharedDocument> {
    let pool = get_pool()?;

    let share_type_enum = ShareType::from_str(&share_type)
        .map_err(|_| StoryWeaverError::invalid_input("Invalid share type".to_string()))?;

    let hashed_password = password
        .map(|p| bcrypt::hash(p, bcrypt::DEFAULT_COST).unwrap());

    let settings = ShareSettings {
        allow_comments: true, // Default to allowing comments
        allow_anonymous: true, // Default to allowing anonymous users
        max_participants: None, // No limit by default
        expires_at: expires_in_hours
            .map(|hours| Utc::now() + chrono::Duration::hours(hours as i64)),
        password: hashed_password,
    };

    collaboration_ops::create_shared_document(
        pool.as_ref(),
        &document_id,
        &project_id,
        share_type_enum,
        settings,
        None,
    )
    .await
    .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get shared document by token
#[tauri::command]
pub async fn get_shared_document(
    token: String,
    password: Option<String>,
) -> Result<Option<SharedDocument>> {
    let pool = get_pool()?;

    let shared_doc = collaboration_ops::get_shared_document_by_token(pool.as_ref(), &token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;

    if let Some(doc) = &shared_doc {
        // Check if password is required and matches
        if let Some(stored_hash) = &doc.password_hash {
            match password {
                Some(provided_password) => {
                    if !bcrypt::verify(&provided_password, stored_hash).unwrap_or(false) {
                        return Err(StoryWeaverError::authentication(
                            "Invalid password".to_string(),
                        ));
                    }
                }
                None => {
                    return Err(StoryWeaverError::authentication(
                        "Password required".to_string(),
                    ))
                }
            }
        }

        // Check if link has expired
        if let Some(expires_at) = doc.expires_at {
            if Utc::now() > expires_at {
                return Err(StoryWeaverError::invalid_input(
                    "Share link has expired".to_string(),
                ));
            }
        }

        // Increment usage count
        collaboration_ops::increment_share_usage(pool.as_ref(), doc.id)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    }

    Ok(shared_doc)
}

/// Add a comment to a document
#[tauri::command]
pub async fn add_comment(
    document_id: String,
    content: String,
    author_name: String,
    author_identifier: Option<String>,
    parent_comment_id: Option<i32>,
    position_start: Option<i32>,
    position_end: Option<i32>,
    selected_text: Option<String>,
    comment_type: String,
) -> Result<Comment> {
    let pool = get_pool()?;
    
    let comment_type_enum = CommentType::from_str(&comment_type)
        .map_err(|e| StoryWeaverError::invalid_input(e))?;
    
    let request = CommentRequest {
        document_id,
        parent_comment_id,
        author_name,
        author_identifier,
        content,
        position_start,
        position_end,
        selected_text,
        comment_type: comment_type_enum,
    };

    collaboration_ops::create_comment(pool.as_ref(), request)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get comments for a document
#[tauri::command]
pub async fn get_comments(document_id: String) -> Result<Vec<Comment>> {
    let pool = get_pool()?;

    collaboration_ops::get_document_comments(pool.as_ref(), &document_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get comment threads for a document
#[tauri::command]
pub async fn get_comment_threads(document_id: String) -> Result<Vec<CommentThread>> {
    let pool = get_pool()?;

    collaboration_ops::get_comment_threads(pool.as_ref(), &document_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Resolve a comment
#[tauri::command]
pub async fn resolve_comment(comment_id: i32, resolved_by: String) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::resolve_comment(pool.as_ref(), comment_id, &resolved_by)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Delete a comment
#[tauri::command]
pub async fn delete_comment(comment_id: i32) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::delete_comment(pool.as_ref(), comment_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Create a collaboration session
#[tauri::command]
pub async fn create_collaboration_session(
    document_id: String,
    max_participants: i32,
    expires_in_hours: Option<i32>,
) -> Result<CollaborationSession> {
    let pool = get_pool()?;

    collaboration_ops::create_collaboration_session(
        pool.as_ref(),
        &document_id,
        max_participants,
        expires_in_hours,
    )
    .await
    .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Join a collaboration session
#[tauri::command]
pub async fn join_collaboration_session(
    session_token: String,
) -> Result<Option<CollaborationSession>> {
    let pool = get_pool()?;

    let session =
        collaboration_ops::get_collaboration_session_by_token(pool.as_ref(), &session_token)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;

    if let Some(ref sess) = session {
        // Check if session has expired
        if let Some(expires_at) = sess.expires_at {
            if Utc::now() > expires_at {
                return Err(StoryWeaverError::validation(
                    "Collaboration session has expired".to_string(),
                ));
            }
        }
    }

    Ok(session)
}

/// Leave a collaboration session
#[tauri::command]
pub async fn leave_collaboration_session(session_token: String) -> Result<()> {
    let pool = get_pool()?;

    let session =
        collaboration_ops::get_collaboration_session_by_token(&pool, &session_token)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;

    if let Some(sess) = session {
        // Participant logic would be handled by a dedicated service
    }

    Ok(())
}

/// Duplicate a document for sharing
#[tauri::command]
pub async fn duplicate_document_for_sharing(
    document_id: String,
    new_title: String,
) -> Result<String> {
    let pool = get_pool()?;

    collaboration_ops::duplicate_document_for_sharing(pool.as_ref(), &document_id, &new_title)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Unpublish a shared document (deactivate the share link)
#[tauri::command]
pub async fn unpublish_shared_document(share_token: String) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::unpublish_shared_document(pool.as_ref(), &share_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Republish a shared document (reactivate the share link)
#[tauri::command]
pub async fn republish_shared_document(share_token: String) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::republish_shared_document(pool.as_ref(), &share_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get all shared documents for a project
#[tauri::command]
pub async fn get_project_shared_documents(project_id: String) -> Result<Vec<SharedDocument>> {
    let pool = get_pool()?;

    collaboration_ops::get_project_shared_documents(pool.as_ref(), &project_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Create a new collaboration notification
#[tauri::command]
pub async fn create_notification(
    document_id: String,
    notification_type: String,
    message: String,
    recipient_token: Option<String>,
) -> Result<CollaborationNotification> {
    let pool = get_pool()?;

    let notification_type_enum = NotificationType::from_str(&notification_type)
        .map_err(|e| StoryWeaverError::invalid_input(e))?;

    collaboration_ops::create_notification(
        pool.as_ref(),
        &document_id,
        notification_type_enum,
        &message,
        recipient_token.as_deref(),
    )
    .await
    .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get notifications for a user
#[tauri::command]
pub async fn get_notifications_for_user(
    recipient_token: String,
    limit: Option<i32>,
) -> Result<Vec<CollaborationNotification>> {
    let pool = get_pool()?;

    collaboration_ops::get_notifications_for_user(pool.as_ref(), &recipient_token, limit)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Mark notification as read
#[tauri::command]
pub async fn mark_notification_read(notification_id: i32) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::mark_notification_read(pool.as_ref(), notification_id)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Mark all notifications as read for a user
#[tauri::command]
pub async fn mark_all_notifications_read(recipient_token: String) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::mark_all_notifications_read(pool.as_ref(), &recipient_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Get unread notification count for a user
#[tauri::command]
pub async fn get_unread_notification_count(recipient_token: String) -> Result<i32> {
    let pool = get_pool()?;

    collaboration_ops::get_unread_notification_count(pool.as_ref(), &recipient_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}

/// Delete old notifications (cleanup)
#[tauri::command]
pub async fn delete_old_notifications(days_old: i32) -> Result<()> {
    let pool = get_pool()?;

    collaboration_ops::delete_old_notifications(pool.as_ref(), days_old)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))
}
