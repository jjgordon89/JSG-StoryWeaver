use crate::database::{get_pool, models::{AppSettings, UserPreference, PreferenceDataType}, operations::{AppSettingsOps, UserPreferenceOps}};
use crate::error::Result;
use crate::commands::CommandResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Get a setting by key
#[tauri::command]
pub async fn get_setting(key: String) -> CommandResponse<Option<AppSettings>> {
    async fn get(key: String) -> Result<Option<AppSettings>> {
        let pool = get_pool()?;
        AppSettingsOps::get_setting(pool, &key).await
    }
    
    get(key).await.into()
}

/// Get all settings
#[tauri::command]
pub async fn get_all_settings() -> CommandResponse<Vec<AppSettings>> {
    async fn get_all() -> Result<Vec<AppSettings>> {
        let pool = get_pool()?;
        AppSettingsOps::get_all_settings(pool).await
    }
    
    get_all().await.into()
}

/// Set a setting
#[tauri::command]
pub async fn set_setting(key: String, value: String) -> CommandResponse<()> {
    async fn set(key: String, value: String) -> Result<()> {
        let pool = get_pool()?;
        AppSettingsOps::set_setting(pool, &key, &value).await
    }
    
    set(key, value).await.into()
}

/// Delete a setting
#[tauri::command]
pub async fn delete_setting(key: String) -> CommandResponse<()> {
    async fn delete(key: String) -> Result<()> {
        let pool = get_pool()?;
        AppSettingsOps::delete_setting(pool, &key).await
    }
    
    delete(key).await.into()
}

/// Get a user preference
#[tauri::command]
pub async fn get_preference(category: String, key: String) -> CommandResponse<Option<UserPreference>> {
    async fn get(category: String, key: String) -> Result<Option<UserPreference>> {
        let pool = get_pool()?;
        UserPreferenceOps::get_preference(pool, &category, &key).await
    }
    
    get(category, key).await.into()
}

/// Get all preferences in a category
#[tauri::command]
pub async fn get_preferences_by_category(category: String) -> CommandResponse<Vec<UserPreference>> {
    async fn get_by_category(category: String) -> Result<Vec<UserPreference>> {
        let pool = get_pool()?;
        UserPreferenceOps::get_preferences_by_category(pool, &category).await
    }
    
    get_by_category(category).await.into()
}

/// Get all preferences
#[tauri::command]
pub async fn get_all_preferences() -> CommandResponse<Vec<UserPreference>> {
    async fn get_all() -> Result<Vec<UserPreference>> {
        let pool = get_pool()?;
        UserPreferenceOps::get_all_preferences(pool).await
    }
    
    get_all().await.into()
}

/// Set a preference
#[derive(Debug, Deserialize)]
pub struct SetPreferenceRequest {
    pub category: String,
    pub key: String,
    pub value: String,
    pub data_type: String,
}

#[tauri::command]
pub async fn set_preference(request: SetPreferenceRequest) -> CommandResponse<()> {
    async fn set(request: SetPreferenceRequest) -> Result<()> {
        let pool = get_pool()?;
        
        let data_type = match request.data_type.as_str() {
            "string" => PreferenceDataType::String,
            "integer" => PreferenceDataType::Integer,
            "boolean" => PreferenceDataType::Boolean,
            "json" => PreferenceDataType::Json,
            _ => PreferenceDataType::String, // Default to string
        };
        
        UserPreferenceOps::set_preference(
            pool, 
            &request.category, 
            &request.key, 
            &request.value, 
            data_type
        ).await
    }
    
    set(request).await.into()
}

/// Delete a preference
#[tauri::command]
pub async fn delete_preference(category: String, key: String) -> CommandResponse<()> {
    async fn delete(category: String, key: String) -> Result<()> {
        let pool = get_pool()?;
        UserPreferenceOps::delete_preference(pool, &category, &key).await
    }
    
    delete(category, key).await.into()
}

/// Delete all preferences in a category
#[tauri::command]
pub async fn delete_preference_category(category: String) -> CommandResponse<()> {
    async fn delete_category(category: String) -> Result<()> {
        let pool = get_pool()?;
        UserPreferenceOps::delete_category(pool, &category).await
    }
    
    delete_category(category).await.into()
}

/// Get preferences as a structured object
#[tauri::command]
pub async fn get_preferences_as_object(category: String) -> CommandResponse<Value> {
    async fn get_as_object(category: String) -> Result<Value> {
        let pool = get_pool()?;
        UserPreferenceOps::get_preferences_as_object(pool, &category).await
    }
    
    get_as_object(category).await.into()
}

/// Set preferences from a structured object
#[tauri::command]
pub async fn set_preferences_from_object(category: String, preferences: Value) -> CommandResponse<()> {
    async fn set_from_object(category: String, preferences: Value) -> Result<()> {
        let pool = get_pool()?;
        UserPreferenceOps::set_preferences_from_object(pool, &category, preferences).await
    }
    
    set_from_object(category, preferences).await.into()
}

/// Sync settings between frontend and backend
#[derive(Debug, Deserialize)]
pub struct SyncSettingsRequest {
    pub settings: Value,
}

#[tauri::command]
pub async fn sync_settings(request: SyncSettingsRequest) -> CommandResponse<Value> {
    async fn sync(request: SyncSettingsRequest) -> Result<Value> {
        let pool = get_pool()?;
        
        // Store settings in the database
        UserPreferenceOps::set_preferences_from_object(pool, "app", request.settings.clone()).await?;
        
        // Return the current settings from the database
        UserPreferenceOps::get_preferences_as_object(pool, "app").await
    }
    
    sync(request).await.into()
}
