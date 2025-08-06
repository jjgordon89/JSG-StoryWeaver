use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// DocumentLink model - represents links between documents for continuity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DocumentLink {
    pub id: String,
    pub from_document_id: String,
    pub to_document_id: String,
    pub link_order: i32,
    pub created_at: DateTime<Utc>,
}

impl DocumentLink {
    pub fn new(from_document_id: String, to_document_id: String, link_order: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_document_id,
            to_document_id,
            link_order,
            created_at: Utc::now(),
        }
    }
}
