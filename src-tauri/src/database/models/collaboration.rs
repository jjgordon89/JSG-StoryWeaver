//! Collaboration models for document sharing and commenting

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Comment type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommentType {
    #[serde(rename = "general")]
    General,
    #[serde(rename = "suggestion")]
    Suggestion,
    #[serde(rename = "question")]
    Question,
    #[serde(rename = "issue")]
    Issue,
}

impl ToString for CommentType {
    fn to_string(&self) -> String {
        match self {
            CommentType::General => "general".to_string(),
            CommentType::Suggestion => "suggestion".to_string(),
            CommentType::Question => "question".to_string(),
            CommentType::Issue => "issue".to_string(),
        }
    }
}

/// Share type enumeration for document sharing permissions
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ShareType {
    #[sqlx(rename = "read_only")]
    ReadOnly,
    #[sqlx(rename = "comment")]
    Comment,
    #[sqlx(rename = "edit")]
    Edit,
}

impl ToString for ShareType {
    fn to_string(&self) -> String {
        match self {
            ShareType::ReadOnly => "read_only".to_string(),
            ShareType::Comment => "comment".to_string(),
            ShareType::Edit => "edit".to_string(),
        }
    }
}

/// Shared document model for document sharing functionality
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SharedDocument {
    pub id: i32,
    pub document_id: String,
    pub project_id: String,
    pub share_token: String,
    pub share_type: String,
    pub password_hash: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_uses: Option<i32>,
    pub current_uses: i32,
    pub is_active: bool,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Collaboration session model for managing active collaboration sessions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollaborationSession {
    pub id: i32,
    pub document_id: String,
    pub session_token: String,
    pub session_name: Option<String>,
    pub is_active: bool,
    pub allow_anonymous: bool,
    pub max_participants: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Comment model for document comments
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i32,
    pub document_id: String,
    pub parent_comment_id: Option<i32>,
    pub author_name: String,
    pub author_identifier: String,
    pub content: String,
    pub position_start: Option<i32>,
    pub position_end: Option<i32>,
    pub selected_text: Option<String>,
    pub comment_type: String,
    pub status: String,
    pub is_resolved: bool,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Comment request model for creating new comments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRequest {
    pub document_id: String,
    pub parent_comment_id: Option<i32>,
    pub author_name: String,
    pub author_identifier: String,
    pub content: String,
    pub position_start: Option<i32>,
    pub position_end: Option<i32>,
    pub selected_text: Option<String>,
    pub comment_type: CommentType,
}

/// Share settings for configuring document sharing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareSettings {
    pub allow_comments: bool,
    pub allow_anonymous: bool,
    pub max_participants: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub password: Option<String>,
    pub expires_in_hours: Option<i32>,
    pub max_uses: Option<i32>,
}

/// Share link response model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLink {
    pub token: String,
    pub url: String,
    pub document_id: String,
    pub settings: ShareSettings,
    pub created_at: DateTime<Utc>,
}

/// Comment thread for organizing comments hierarchically
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentThread {
    pub parent_comment: Comment,
    pub replies: Vec<Comment>,
    pub total_replies: i32,
}

/// Collaboration participant model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollaborationParticipant {
    pub id: i32,
    pub session_id: i32,
    pub user_token: String,
    pub user_name: Option<String>,
    pub is_anonymous: bool,
    pub joined_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

/// Notification model for collaboration events
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CollaborationNotification {
    pub id: i32,
    pub document_id: String,
    pub notification_type: NotificationType,
    pub message: String,
    pub recipient_token: Option<String>,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

/// Notification type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum NotificationType {
    #[sqlx(rename = "new_comment")]
    NewComment,
    #[sqlx(rename = "comment_reply")]
    CommentReply,
    #[sqlx(rename = "comment_resolved")]
    CommentResolved,
    #[sqlx(rename = "participant_joined")]
    ParticipantJoined,
    #[sqlx(rename = "document_updated")]
    DocumentUpdated,
}