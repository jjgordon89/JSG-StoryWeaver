//! Tauri commands for collaboration features

use crate::database::{get_pool, operations::collaboration::*};
use crate::database::models::{SharedDocument, ShareType, ShareSettings, Comment, CommentType, CollaborationSession, CollaborationNotification, NotificationType, CommentThread};
use crate::database::models::collaboration::CommentRequest;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use serde_json::Value;

/// Create a shared document link
#[tauri::command]
pub async fn create_shared_document_link(
    document_id: String,
    project_id: String,
    share_type: String,
    password: Option<String>,
    expires_in_hours: Option<i32>,
    max_uses: Option<i32>,
    created_by: Option<String>,
) -> Result<SharedDocument> {
    let pool = get_pool()?;
    
    let share_type_enum = match share_type.as_str() {
        "read_only" => ShareType::ReadOnly,
        "comment" => ShareType::Comment,
        "edit" => ShareType::Edit,
        _ => return Err(StoryWeaverError::InvalidInput("Invalid share type".to_string())),
    };
    
    let settings = ShareSettings {
        allow_comments: true, // Default to allowing comments
        allow_anonymous: true, // Default to allowing anonymous users
        max_participants: None, // No limit by default
        expires_at: expires_in_hours.map(|hours| Utc::now() + chrono::Duration::hours(hours as i64)),
        password,
        expires_in_hours,
        max_uses,
    };
    
    create_shared_document(
        &pool,
        &document_id,
        &project_id,
        share_type_enum,
        settings,
        created_by.as_deref(),
    )
    .await
    .map_err(StoryWeaverError::database)
}

/// Get shared document by token
#[tauri::command]
pub async fn get_shared_document(
    token: String,
    password: Option<String>,
) -> Result<Option<SharedDocument>> {
    let pool = get_pool()?;
    
    let shared_doc = get_shared_document_by_token(&pool, &token)
        .await
        .map_err(StoryWeaverError::database)?;
    
    if let Some(doc) = &shared_doc {
        // Check if password is required and matches
        if let Some(stored_hash) = &doc.password_hash {
            match password {
                Some(provided_password) => {
                    // In a real implementation, use proper password verification
                    let expected_hash = format!("hashed_{}", provided_password);
                    if stored_hash != &expected_hash {
                        return Err(StoryWeaverError::authentication("Invalid password"));
                    }
                }
                None => return Err(StoryWeaverError::authentication("Password required")),
            }
        }
        
        // Check if link has expired
        if let Some(expires_at) = doc.expires_at {
            if chrono::Utc::now() > expires_at {
                return Err(StoryWeaverError::InvalidInput("Share link has expired".to_string()));
            }
        }
        
        // Check usage limits
        if let Some(max_uses) = doc.max_uses {
            if doc.current_uses >= max_uses {
                return Err(StoryWeaverError::InvalidInput("Share link usage limit exceeded".to_string()));
            }
        }
        
        // Increment usage count
        increment_share_usage(&pool, doc.id)
            .await
            .map_err(StoryWeaverError::database)?;
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
    
    let comment_type_enum = match comment_type.as_str() {
        "general" => CommentType::General,
        "suggestion" => CommentType::Suggestion,
        "question" => CommentType::Question,
        "praise" => CommentType::Praise,
        "criticism" => CommentType::Criticism,
        _ => return Err(StoryWeaverError::InvalidInput("Invalid comment type".to_string())),
    };
    
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
    
    create_comment(&pool, request)
        .await
        .map_err(StoryWeaverError::database)
}

/// Get comments for a document
#[tauri::command]
pub async fn get_comments(
    document_id: String,
) -> Result<Vec<Comment>> {
    let pool = get_pool()?;
    
    get_document_comments(&pool, &document_id)
        .await
        .map_err(StoryWeaverError::database)
}

/// Get comment threads for a document
#[tauri::command]
pub async fn get_comment_threads(
    document_id: String,
) -> Result<Vec<CommentThread>> {
    let pool = get_pool()?;
    
    get_comment_threads(&pool, &document_id)
        .await
        .map_err(StoryWeaverError::database)
}

/// Resolve a comment
#[tauri::command]
pub async fn resolve_comment(
    comment_id: i32,
    resolved_by: String,
) -> Result<()> {
    let pool = get_pool()?;
    
    resolve_comment(&pool, comment_id, &resolved_by)
        .await
        .map_err(StoryWeaverError::database)
}

/// Delete a comment
#[tauri::command]
pub async fn delete_comment(
    comment_id: i32,
) -> Result<()> {
    let pool = get_pool()?;
    
    delete_comment(&pool, comment_id)
        .await
        .map_err(StoryWeaverError::database)
}

/// Create a collaboration session
#[tauri::command]
pub async fn create_collaboration_session(
    document_id: String,
    max_participants: i32,
    expires_in_hours: Option<i32>,
) -> Result<CollaborationSession> {
    let pool = get_pool()?;
    
    create_collaboration_session(&pool, &document_id, max_participants, expires_in_hours)
        .await
        .map_err(StoryWeaverError::database)
}

/// Join a collaboration session
#[tauri::command]
pub async fn join_collaboration_session(
    session_token: String,
) -> Result<Option<CollaborationSession>, StoryWeaverError> {
    let pool = get_pool()?;
    
    let session = get_collaboration_session_by_token(&pool, &session_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(ref sess) = session {
        // Check if session has expired
        if let Some(expires_at) = sess.expires_at {
            if chrono::Utc::now() > expires_at {
                return Err(StoryWeaverError::validation("Collaboration session has expired"));
            }
        }
        
        // Check participant limits
        if sess.current_participants >= sess.max_participants {
            return Err(StoryWeaverError::validation("Collaboration session is full"));
        }
        
        // Update participant count (this would be handled by WebSocket in real implementation)
        update_session_participants(&pool, sess.id, sess.current_participants + 1)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    }
    
    Ok(session)
}

/// Leave a collaboration session
#[tauri::command]
pub async fn leave_collaboration_session(
    session_token: String,
) -> Result<(), StoryWeaverError> {
    let pool = get_pool()?;
    
    let session = get_collaboration_session_by_token(&pool, &session_token)
        .await
        .map_err(|e| StoryWeaverError::database(e.to_string()))?;
    
    if let Some(sess) = session {
        let new_count = std::cmp::max(0, sess.current_participants - 1);
        update_session_participants(&pool, sess.id, new_count)
            .await
            .map_err(|e| StoryWeaverError::database(e.to_string()))?;
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
    
    duplicate_document_for_sharing(&pool, &document_id, &new_title)
        .await
        .map_err(StoryWeaverError::database)
}

/// Unpublish a shared document (deactivate the share link)
#[tauri::command]
pub async fn unpublish_shared_document(
    share_token: String,
) -> Result<()> {
    let pool = get_pool()?;
    
    unpublish_shared_document(&pool, &share_token)
        .await
        .map_err(StoryWeaverError::database)
}

/// Republish a shared document (reactivate the share link)
#[tauri::command]
pub async fn republish_shared_document(
    share_token: String,
) -> Result<()> {
    let pool = get_pool()?;
    
    republish_shared_document(&pool, &share_token)
        .await
        .map_err(StoryWeaverError::database)
}

/// Get all shared documents for a project
#[tauri::command]
pub async fn get_project_shared_documents(
    project_id: String,
) -> Result<Vec<SharedDocument>> {
    let pool = get_pool()?;
    
    get_project_shared_documents(&pool, &project_id)
        .await
        .map_err(StoryWeaverError::database)
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
    
    let notification_type = match notification_type.as_str() {
        "new_comment" => NotificationType::NewComment,
        "comment_reply" => NotificationType::CommentReply,
        "comment_resolved" => NotificationType::CommentResolved,
        "participant_joined" => NotificationType::ParticipantJoined,
        "document_updated" => NotificationType::DocumentUpdated,
        _ => return Err(StoryWeaverError::InvalidInput("Invalid notification type".to_string())),
    };
    
    create_notification(&pool, &document_id, notification_type, &message, recipient_token.as_deref())
        .await
        .map_err(StoryWeaverError::database)
}

/// Get notifications for a user
#[tauri::command]
pub async fn get_notifications_for_user(
    recipient_token: String,
    limit: Option<i32>,
) -> Result<Vec<CollaborationNotification>> {
    let pool = get_pool()?;
    
    get_notifications_for_user(&pool, &recipient_token, limit)
        .await
        .map_err(StoryWeaverError::database)
}

/// Mark notification as read
#[tauri::command]
pub async fn mark_notification_read(
    notification_id: i32,
) -> Result<()> {
    let pool = get_pool()?;
    
    mark_notification_read(&pool, notification_id)
        .await
        .map_err(StoryWeaverError::database)
}

/// Mark all notifications as read for a user
#[tauri::command]
pub async fn mark_all_notifications_read(
    recipient_token: String,
) -> Result<()> {
    let pool = get_pool()?;
    
    mark_all_notifications_read(&pool, &recipient_token)
        .await
        .map_err(StoryWeaverError::database)
}

/// Get unread notification count for a user
#[tauri::command]
pub async fn get_unread_notification_count(
    recipient_token: String,
) -> Result<i32> {
    let pool = get_pool()?;
    
    get_unread_notification_count(&pool, &recipient_token)
        .await
        .map_err(StoryWeaverError::database)
}

/// Delete old notifications (cleanup)
#[tauri::command]
pub async fn delete_old_notifications(
    days_old: i32,
) -> Result<()> {
    let pool = get_pool()?;
    
    delete_old_notifications(&pool, days_old)
        .await
        .map_err(StoryWeaverError::database)
}