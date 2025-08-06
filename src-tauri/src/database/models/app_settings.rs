use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// AppSettings model - represents application settings and preferences
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppSettings {
    pub id: String,
    pub key: String,
    pub value: String, // JSON string for complex settings
    pub updated_at: DateTime<Utc>,
}

/// UserPreference model - represents user-specific preferences
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreference {
    pub id: String,
    pub preference_category: String,
    pub preference_key: String,
    pub preference_value: String,
    pub data_type: PreferenceDataType,
    pub updated_at: DateTime<Utc>,
}

/// Preference data type
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum PreferenceDataType {
    #[sqlx(rename = "string")]
    String,
    #[sqlx(rename = "integer")]
    Integer,
    #[sqlx(rename = "boolean")]
    Boolean,
    #[sqlx(rename = "json")]
    Json,
}

impl AppSettings {
    pub fn new(key: String, value: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            key,
            value,
            updated_at: Utc::now(),
        }
    }
}

impl UserPreference {
    pub fn new(
        preference_category: String,
        preference_key: String,
        preference_value: String,
        data_type: PreferenceDataType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            preference_category,
            preference_key,
            preference_value,
            data_type,
            updated_at: Utc::now(),
        }
    }
}
