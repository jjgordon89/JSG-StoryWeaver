use crate::database::models::{AppSettings, UserPreference, PreferenceDataType};
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// AppSettings operations
pub struct AppSettingsOps;

impl AppSettingsOps {
    /// Get a setting by key
    pub async fn get_setting(pool: &Pool<Sqlite>, key: &str) -> Result<Option<AppSettings>> {
        let setting = sqlx::query_as::<_, AppSettings>("SELECT * FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get setting: {}", e)))?;
        
        Ok(setting)
    }
    
    /// Get all settings
    pub async fn get_all_settings(pool: &Pool<Sqlite>) -> Result<Vec<AppSettings>> {
        let settings = sqlx::query_as::<_, AppSettings>("SELECT * FROM settings")
            .fetch_all(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get all settings: {}", e)))?;
        
        Ok(settings)
    }
    
    /// Set a setting
    pub async fn set_setting(pool: &Pool<Sqlite>, key: &str, value: &str) -> Result<()> {
        // Check if setting exists
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM settings WHERE key = ?)"
        )
        .bind(key)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check if setting exists: {}", e)))?;
        
        if exists {
            // Update existing setting
            sqlx::query(
                "UPDATE settings SET value = ?, updated_at = ? WHERE key = ?"
            )
            .bind(value)
            .bind(Utc::now())
            .bind(key)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to update setting: {}", e)))?;
        } else {
            // Insert new setting
            sqlx::query(
                "INSERT INTO settings (id, key, value, updated_at) VALUES (?, ?, ?, ?)"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(key)
            .bind(value)
            .bind(Utc::now())
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to insert setting: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Delete a setting
    pub async fn delete_setting(pool: &Pool<Sqlite>, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM settings WHERE key = ?")
            .bind(key)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete setting: {}", e)))?;
        
        Ok(())
    }
}

/// UserPreference operations
pub struct UserPreferenceOps;

impl UserPreferenceOps {
    /// Get a user preference
    pub async fn get_preference(
        pool: &Pool<Sqlite>, 
        category: &str, 
        key: &str
    ) -> Result<Option<UserPreference>> {
        let preference = sqlx::query_as::<_, UserPreference>(
            "SELECT * FROM user_preferences WHERE preference_category = ? AND preference_key = ?"
        )
        .bind(category)
        .bind(key)
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get preference: {}", e)))?;
        
        Ok(preference)
    }
    
    /// Get all preferences in a category
    pub async fn get_preferences_by_category(
        pool: &Pool<Sqlite>, 
        category: &str
    ) -> Result<Vec<UserPreference>> {
        let preferences = sqlx::query_as::<_, UserPreference>(
            "SELECT * FROM user_preferences WHERE preference_category = ?"
        )
        .bind(category)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get preferences by category: {}", e)))?;
        
        Ok(preferences)
    }
    
    /// Get all preferences
    pub async fn get_all_preferences(pool: &Pool<Sqlite>) -> Result<Vec<UserPreference>> {
        let preferences = sqlx::query_as::<_, UserPreference>("SELECT * FROM user_preferences")
            .fetch_all(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get all preferences: {}", e)))?;
        
        Ok(preferences)
    }
    
    /// Set a preference
    pub async fn set_preference(
        pool: &Pool<Sqlite>, 
        category: &str, 
        key: &str, 
        value: &str,
        data_type: PreferenceDataType
    ) -> Result<()> {
        // Check if preference exists
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM user_preferences WHERE preference_category = ? AND preference_key = ?)"
        )
        .bind(category)
        .bind(key)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check if preference exists: {}", e)))?;
        
        if exists {
            // Update existing preference
            sqlx::query(
                r#"
                UPDATE user_preferences 
                SET preference_value = ?, data_type = ?, updated_at = ? 
                WHERE preference_category = ? AND preference_key = ?
                "#
            )
            .bind(value)
            .bind(&data_type)
            .bind(Utc::now())
            .bind(category)
            .bind(key)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to update preference: {}", e)))?;
        } else {
            // Insert new preference
            sqlx::query(
                r#"
                INSERT INTO user_preferences (
                    id, preference_category, preference_key, preference_value, 
                    data_type, updated_at
                ) 
                VALUES (?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(Uuid::new_v4().to_string())
            .bind(category)
            .bind(key)
            .bind(value)
            .bind(&data_type)
            .bind(Utc::now())
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to insert preference: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Delete a preference
    pub async fn delete_preference(
        pool: &Pool<Sqlite>, 
        category: &str, 
        key: &str
    ) -> Result<()> {
        sqlx::query(
            "DELETE FROM user_preferences WHERE preference_category = ? AND preference_key = ?"
        )
        .bind(category)
        .bind(key)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete preference: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete all preferences in a category
    pub async fn delete_category(pool: &Pool<Sqlite>, category: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_preferences WHERE preference_category = ?")
            .bind(category)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete preference category: {}", e)))?;
        
        Ok(())
    }
    
    /// Get preferences as a structured object
    pub async fn get_preferences_as_object(
        pool: &Pool<Sqlite>, 
        category: &str
    ) -> Result<serde_json::Value> {
        let preferences = Self::get_preferences_by_category(pool, category).await?;
        
        let mut result = serde_json::Map::new();
        
        for pref in preferences {
            let value = match pref.data_type {
                PreferenceDataType::String => serde_json::Value::String(pref.preference_value),
                PreferenceDataType::Integer => {
                    let int_value = pref.preference_value.parse::<i64>()
                        .map_err(|e| StoryWeaverError::ParseError { 
                            message: format!("Failed to parse integer preference: {}", e) 
                        })?;
                    serde_json::Value::Number(serde_json::Number::from(int_value))
                },
                PreferenceDataType::Boolean => {
                    let bool_value = pref.preference_value.parse::<bool>()
                        .map_err(|e| StoryWeaverError::ParseError { 
                            message: format!("Failed to parse boolean preference: {}", e) 
                        })?;
                    serde_json::Value::Bool(bool_value)
                },
                PreferenceDataType::Json => {
                    serde_json::from_str(&pref.preference_value)
                        .map_err(|e| StoryWeaverError::ParseError { 
                            message: format!("Failed to parse JSON preference: {}", e) 
                        })?
                },
            };
            
            result.insert(pref.preference_key, value);
        }
        
        Ok(serde_json::Value::Object(result))
    }
    
    /// Set preferences from a structured object
    pub async fn set_preferences_from_object(
        pool: &Pool<Sqlite>, 
        category: &str, 
        preferences: serde_json::Value
    ) -> Result<()> {
        // Start a transaction
        let mut tx = pool.begin().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to start transaction: {}", e)))?;
        
        if let serde_json::Value::Object(map) = preferences {
            for (key, value) in map {
                let (value_str, data_type) = match value {
                    serde_json::Value::String(s) => (s, PreferenceDataType::String),
                    serde_json::Value::Number(n) => (n.to_string(), PreferenceDataType::Integer),
                    serde_json::Value::Bool(b) => (b.to_string(), PreferenceDataType::Boolean),
                    serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                        (value.to_string(), PreferenceDataType::Json)
                    },
                    serde_json::Value::Null => continue, // Skip null values
                };
                
                // Check if preference exists
                let exists = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM user_preferences WHERE preference_category = ? AND preference_key = ?)"
                )
                .bind(category)
                .bind(&key)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to check if preference exists: {}", e)))?;
                
                if exists {
                    // Update existing preference
                    sqlx::query(
                        r#"
                        UPDATE user_preferences 
                        SET preference_value = ?, data_type = ?, updated_at = ? 
                        WHERE preference_category = ? AND preference_key = ?
                        "#
                    )
                    .bind(&value_str)
                    .bind(&data_type)
                    .bind(Utc::now())
                    .bind(category)
                    .bind(&key)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| StoryWeaverError::database(format!("Failed to update preference: {}", e)))?;
                } else {
                    // Insert new preference
                    sqlx::query(
                        r#"
                        INSERT INTO user_preferences (
                            id, preference_category, preference_key, preference_value, 
                            data_type, updated_at
                        ) 
                        VALUES (?, ?, ?, ?, ?, ?)
                        "#
                    )
                    .bind(Uuid::new_v4().to_string())
                    .bind(category)
                    .bind(&key)
                    .bind(&value_str)
                    .bind(&data_type)
                    .bind(Utc::now())
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| StoryWeaverError::database(format!("Failed to insert preference: {}", e)))?;
                }
            }
        }
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(())
    }
}
