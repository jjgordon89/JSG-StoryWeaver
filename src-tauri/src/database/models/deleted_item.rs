use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// DeletedItem model - represents items in the trash for potential recovery
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeletedItem {
    pub id: String,
    pub item_type: DeletedItemType,
    pub item_id: String,
    pub item_data: String, // JSON string of the original item
    pub parent_id: Option<String>,
    pub deletion_reason: Option<String>,
    pub deleted_at: DateTime<Utc>,
    pub can_restore: bool,
}

/// Type of deleted item
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum DeletedItemType {
    #[sqlx(rename = "project")]
    Project,
    #[sqlx(rename = "folder")]
    Folder,
    #[sqlx(rename = "document")]
    Document,
    #[sqlx(rename = "series")]
    Series,
    #[sqlx(rename = "character")]
    Character,
    #[sqlx(rename = "location")]
    Location,
}

impl DeletedItem {
    pub fn new(
        item_type: DeletedItemType,
        item_id: String,
        item_data: String,
        parent_id: Option<String>,
        deletion_reason: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            item_type,
            item_id,
            item_data,
            parent_id,
            deletion_reason,
            deleted_at: Utc::now(),
            can_restore: true,
        }
    }
}
