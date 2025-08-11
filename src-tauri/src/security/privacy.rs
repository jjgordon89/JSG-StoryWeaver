//! Privacy-first data handling
//!
//! This module provides functionality for implementing privacy-first data handling
//! practices, including data minimization, anonymization, and consent management.

use crate::error::StoryWeaverError;
use crate::security::encryption::encrypt_string;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::database::get_pool;
use regex::Regex;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Privacy settings for the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySettings {
    /// Whether to collect anonymous usage statistics
    pub collect_usage_stats: bool,
    
    /// Whether to send error reports
    pub send_error_reports: bool,
    
    /// Whether to store API keys locally
    pub store_api_keys_locally: bool,
    
    /// Whether to encrypt sensitive data
    pub encrypt_sensitive_data: bool,
    
    /// Whether to use secure connections for API calls
    pub use_secure_connections: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            collect_usage_stats: false,
            send_error_reports: false,
            store_api_keys_locally: true,
            encrypt_sensitive_data: true,
            use_secure_connections: true,
        }
    }
}

/// Privacy manager for handling privacy-related functionality
#[derive(Debug)]
pub struct PrivacyManager {
    settings: Arc<RwLock<PrivacySettings>>,
}

impl PrivacyManager {
    /// Create a new privacy manager
    pub fn new(settings: PrivacySettings) -> Self {
        Self {
            settings: Arc::new(RwLock::new(settings)),
        }
    }

    /// Get the current privacy settings
    pub async fn get_settings(&self) -> PrivacySettings {
        self.settings.read().await.clone()
    }

    /// Update the privacy settings
    pub async fn update_settings(&self, settings: PrivacySettings) -> Result<(), StoryWeaverError> {
        let mut current_settings = self.settings.write().await;
        *current_settings = settings;
        Ok(())
    }

    /// Check if usage statistics collection is enabled
    pub async fn can_collect_usage_stats(&self) -> bool {
        self.settings.read().await.collect_usage_stats
    }

    /// Check if error reporting is enabled
    pub async fn can_send_error_reports(&self) -> bool {
        self.settings.read().await.send_error_reports
    }

    /// Check if API keys should be stored locally
    pub async fn should_store_api_keys_locally(&self) -> bool {
        self.settings.read().await.store_api_keys_locally
    }

    /// Check if sensitive data should be encrypted
    pub async fn should_encrypt_sensitive_data(&self) -> bool {
        self.settings.read().await.encrypt_sensitive_data
    }

    /// Check if secure connections should be used
    pub async fn should_use_secure_connections(&self) -> bool {
        self.settings.read().await.use_secure_connections
    }
}

/// Data categories for privacy purposes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataCategory {
    /// Personal identifiable information
    PII,
    
    /// API keys and credentials
    Credentials,
    
    /// User content (documents, projects, etc.)
    UserContent,
    
    /// Application settings
    Settings,
    
    /// Usage statistics
    UsageStats,
}

/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Data category
    pub category: DataCategory,
    
    /// Retention period in days (None for indefinite)
    pub retention_days: Option<u32>,
    
    /// Whether to encrypt the data
    pub encrypt: bool,
    
    /// Whether to anonymize the data
    pub anonymize: bool,
}

/// Global instance of the privacy manager
static mut PRIVACY_MANAGER: Option<Arc<PrivacyManager>> = None;

/// Initialize the privacy manager
pub async fn init() -> Result<(), StoryWeaverError> {
    // Load privacy settings from database or use defaults
    let settings = load_privacy_settings().await?;
    
    let manager = PrivacyManager::new(settings);
    
    unsafe {
        PRIVACY_MANAGER = Some(Arc::new(manager));
    }
    
    Ok(())
}

/// Get the global privacy manager instance
pub fn get_privacy_manager() -> Result<Arc<PrivacyManager>, StoryWeaverError> {
    unsafe {
        match &PRIVACY_MANAGER {
            Some(manager) => Ok(manager.clone()),
            None => Err(StoryWeaverError::SecurityError{ message: "Privacy manager not initialized".to_string() }),
        }
    }
}

/// Load privacy settings from the database
async fn load_privacy_settings() -> Result<PrivacySettings, StoryWeaverError> {
    let pool = get_pool()?;
    
    // Check if privacy settings exist in the database
    let row = sqlx::query!(
        r#"
        SELECT value FROM settings WHERE key = 'privacy_settings'
        "#
    )
    .fetch_optional(&*pool)
    .await
    .map_err(|e| StoryWeaverError::Database{ message: format!("Failed to fetch privacy settings: {}", e) })?;
    
    match row {
        Some(row) => {
            // Parse the settings from JSON
            let value_str = row.value.as_deref().unwrap_or("{}");
            let settings: PrivacySettings = serde_json::from_str(value_str)
                .map_err(|e| StoryWeaverError::Deserialization{ message: format!("Failed to parse privacy settings: {}", e) })?;
            
            Ok(settings)
        },
        None => {
            // Create default settings
            let default_settings = PrivacySettings::default();
            
            // Save the default settings to the database
            let settings_json = serde_json::to_string(&default_settings)
                .map_err(|e| StoryWeaverError::Serialization{ message: format!("Failed to serialize privacy settings: {}", e) })?;
            
            sqlx::query!(
                r#"
                INSERT INTO settings (key, value, updated_at)
                VALUES ('privacy_settings', ?, CURRENT_TIMESTAMP)
                "#,
                settings_json
            )
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::Database{ message: format!("Failed to save privacy settings: {}", e) })?;
            
            Ok(default_settings)
        }
    }
}

/// Save privacy settings to the database
pub async fn save_privacy_settings(settings: &PrivacySettings) -> Result<(), StoryWeaverError> {
    let pool = get_pool()?;
    
    let settings_json = serde_json::to_string(settings)
        .map_err(|e| StoryWeaverError::Serialization{ message: format!("Failed to serialize privacy settings: {}", e) })?;
    
    sqlx::query!(
        r#"
        INSERT OR REPLACE INTO settings (key, value, updated_at)
        VALUES ('privacy_settings', ?, CURRENT_TIMESTAMP)
        "#,
        settings_json
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::Database{ message: format!("Failed to save privacy settings: {}", e) })?;
    
    Ok(())
}

/// Anonymize personal data
pub fn anonymize_data(data: &str) -> String {
    // Simple anonymization by replacing email addresses with [EMAIL]
    // and names with [NAME]
    let email_regex = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
    let name_regex = Regex::new(r"\b[A-Z][a-z]+ [A-Z][a-z]+\b").unwrap();
    
    let data = email_regex.replace_all(data, "[EMAIL]");
    let data = name_regex.replace_all(&data, "[NAME]");
    
    data.to_string()
}

/// Process data according to privacy settings
pub async fn process_data_for_privacy(
    data: &str,
    category: DataCategory,
) -> Result<String, StoryWeaverError> {
    let manager = get_privacy_manager()?;
    let settings = manager.get_settings().await;
    
    let mut processed_data = data.to_string();
    
    // Apply anonymization for usage stats and error reports
    match category {
        DataCategory::UsageStats => {
            if !settings.collect_usage_stats {
                return Err(StoryWeaverError::PrivacyError{ message: "Usage statistics collection is disabled".to_string() });
            }
            processed_data = anonymize_data(&processed_data);
        },
        DataCategory::PII => {
            processed_data = anonymize_data(&processed_data);
        },
        DataCategory::Credentials => {
            if !settings.store_api_keys_locally {
                return Err(StoryWeaverError::PrivacyError{ message: "API key storage is disabled".to_string() });
            }
            
            if settings.encrypt_sensitive_data {
                processed_data = encrypt_string(&processed_data).await?;
            }
        },
        DataCategory::UserContent => {
            if settings.encrypt_sensitive_data {
                processed_data = encrypt_string(&processed_data).await?;
            }
        },
        DataCategory::Settings => {
            // No special processing for settings
        },
    }
    
    Ok(processed_data)
}

/// Get data retention policies
pub fn get_retention_policies() -> HashMap<DataCategory, RetentionPolicy> {
    let mut policies = HashMap::new();
    
    policies.insert(
        DataCategory::PII,
        RetentionPolicy {
            category: DataCategory::PII,
            retention_days: Some(365), // 1 year
            encrypt: true,
            anonymize: true,
        },
    );
    
    policies.insert(
        DataCategory::Credentials,
        RetentionPolicy {
            category: DataCategory::Credentials,
            retention_days: None, // Indefinite
            encrypt: true,
            anonymize: false,
        },
    );
    
    policies.insert(
        DataCategory::UserContent,
        RetentionPolicy {
            category: DataCategory::UserContent,
            retention_days: None, // Indefinite
            encrypt: true,
            anonymize: false,
        },
    );
    
    policies.insert(
        DataCategory::Settings,
        RetentionPolicy {
            category: DataCategory::Settings,
            retention_days: None, // Indefinite
            encrypt: false,
            anonymize: false,
        },
    );
    
    policies.insert(
        DataCategory::UsageStats,
        RetentionPolicy {
            category: DataCategory::UsageStats,
            retention_days: Some(90), // 3 months
            encrypt: false,
            anonymize: true,
        },
    );
    
    policies
}
