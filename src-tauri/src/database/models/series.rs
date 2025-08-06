use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Series model - represents a series of related projects
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Series {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub folder_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Series {
    pub fn new(name: String, description: Option<String>, folder_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            folder_id,
            created_at: Utc::now(),
        }
    }
}
