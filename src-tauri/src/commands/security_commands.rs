//! Security-related commands for the Tauri application
//!
//! This module provides commands for managing security features such as
//! API key storage, encryption, and privacy settings.

use crate::error::StoryWeaverError;
use crate::security::{
    api_keys::{ApiProvider, get_api_key_manager},
    privacy::{PrivacySettings, get_privacy_manager, save_privacy_settings},
    audit::{AuditSeverity, log_api_key_event},
    rate_limit::{rl_create, rl_update, rl_delete, rl_list, rl_search, validate_request_body_size},
};
use serde::{Serialize, Deserialize};
use tauri::command;

/// Request to save an API key
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveApiKeyRequest {
    /// The API provider (OpenAI, Claude, etc.)
    pub provider: String,
    
    /// The API key to save
    pub api_key: String,
}

/// Response for API key operations
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    /// Whether the operation was successful
    pub success: bool,
    
    /// Optional error message
    pub error: Option<String>,
}

/// Response for API key existence check
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyExistsResponse {
    /// Whether the API key exists
    pub exists: bool,
}

/// Privacy settings response
#[derive(Debug, Serialize, Deserialize)]
pub struct PrivacySettingsResponse {
    /// The current privacy settings
    pub settings: PrivacySettings,
}

/// Save an API key to secure storage
#[command]
pub async fn save_api_key(request: SaveApiKeyRequest) -> Result<ApiKeyResponse, StoryWeaverError> {
    let provider = match request.provider.as_str() {
        "openai" => ApiProvider::OpenAI,
        "claude" => ApiProvider::Claude,
        _ => return Ok(ApiKeyResponse {
            success: false,
            error: Some(format!("Unsupported API provider: {}", request.provider)),
        }),
    };
    
    let manager = get_api_key_manager()?;
    
    match manager.save_api_key(provider.clone(), &request.api_key).await {
        Ok(_) => {
            // Log the API key change event
            let provider_str = match provider {
                ApiProvider::OpenAI => "OpenAI",
                ApiProvider::Claude => "Claude",
            };
            
            let _ = log_api_key_event(
                "api_key_saved",
                &format!("{} API key saved", provider_str),
                AuditSeverity::Info,
            ).await;
            
            Ok(ApiKeyResponse {
                success: true,
                error: None,
            })
        },
        Err(e) => Ok(ApiKeyResponse {
            success: false,
            error: Some(e.to_string()),
        }),
    }
}

/// Check if an API key exists
#[command]
pub async fn has_api_key(provider: String) -> Result<ApiKeyExistsResponse, StoryWeaverError> {
    let provider = match provider.as_str() {
        "openai" => ApiProvider::OpenAI,
        "claude" => ApiProvider::Claude,
        _ => return Ok(ApiKeyExistsResponse { exists: false }),
    };
    
    let manager = get_api_key_manager()?;
    
    match manager.has_api_key(provider).await {
        Ok(exists) => Ok(ApiKeyExistsResponse { exists }),
        Err(_) => Ok(ApiKeyExistsResponse { exists: false }),
    }
}

/// Delete an API key
#[command]
pub async fn delete_api_key(provider: String) -> Result<ApiKeyResponse, StoryWeaverError> {
    let provider = match provider.as_str() {
        "openai" => ApiProvider::OpenAI,
        "claude" => ApiProvider::Claude,
        _ => return Ok(ApiKeyResponse {
            success: false,
            error: Some(format!("Unsupported API provider: {}", provider)),
        }),
    };
    
    let manager = get_api_key_manager()?;
    
    match manager.delete_api_key(provider.clone()).await {
        Ok(_) => {
            // Log the API key deletion event
            let provider_str = match provider {
                ApiProvider::OpenAI => "OpenAI",
                ApiProvider::Claude => "Claude",
            };
            
            let _ = log_api_key_event(
                "api_key_deleted",
                &format!("{} API key deleted", provider_str),
                AuditSeverity::Warning,
            ).await;
            
            Ok(ApiKeyResponse {
                success: true,
                error: None,
            })
        },
        Err(e) => Ok(ApiKeyResponse {
            success: false,
            error: Some(e.to_string()),
        }),
    }
}

/// Get the current privacy settings
#[command]
pub async fn get_privacy_settings() -> Result<PrivacySettingsResponse, StoryWeaverError> {
    let manager = get_privacy_manager()?;
    let settings = manager.get_settings().await;
    
    Ok(PrivacySettingsResponse { settings })
}

/// Update the privacy settings
#[command]
pub async fn update_privacy_settings(settings: PrivacySettings) -> Result<ApiKeyResponse, StoryWeaverError> {
    let manager = get_privacy_manager()?;
    
    match manager.update_settings(settings.clone()).await {
        Ok(_) => {
            // Save the settings to the database
            match save_privacy_settings(&settings).await {
                Ok(_) => Ok(ApiKeyResponse {
                    success: true,
                    error: None,
                }),
                Err(e) => Ok(ApiKeyResponse {
                    success: false,
                    error: Some(e.to_string()),
                }),
            }
        },
        Err(e) => Ok(ApiKeyResponse {
            success: false,
            error: Some(e.to_string()),
        }),
    }
}
