use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Folder model - represents a folder in the project hierarchy
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub parent_folder_id: Option<String>,
    pub is_series: bool,
    pub created_at: DateTime<Utc>,
}

impl Folder {
    pub fn new(name: String, parent_folder_id: Option<String>, is_series: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            parent_folder_id,
            is_series,
            created_at: Utc::now(),
        }
    }
}
