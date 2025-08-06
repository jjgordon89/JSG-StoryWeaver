use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// DocumentVersion model - represents version history for documents
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DocumentVersion {
    pub id: String,
    pub document_id: String,
    pub content: String,
    pub word_count: i32,
    pub version_number: i32,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub comment: Option<String>,
}

impl DocumentVersion {
    pub fn new(
        document_id: String, 
        content: String, 
        word_count: i32, 
        version_number: i32,
        created_by: Option<String>,
        comment: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            document_id,
            content,
            word_count,
            version_number,
            created_at: Utc::now(),
            created_by,
            comment,
        }
    }
}
