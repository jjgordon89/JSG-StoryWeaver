//! Secure API key storage and management
//!
//! This module provides functionality for securely storing and retrieving API keys
//! using the operating system's secure storage (keychain/credential store).

use crate::error::StoryWeaverError;
use std::sync::{Arc, OnceLock};
use serde::{Serialize, Deserialize};
use tauri::AppHandle;
use keyring::{Entry, Error as KeyringError};
use tracing::{info, error, debug};

const SERVICE: &str = "storyweaver";
const OPENAI_KEY: &str = "openai";
const CLAUDE_KEY: &str = "claude";

/// Supported API providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiProvider {
    OpenAI,
    Claude,
}

/// API key manager for secure storage and retrieval
#[derive(Debug)]
pub struct ApiKeyManager {
    _app_handle: AppHandle,
}

impl ApiKeyManager {
    /// Create a new API key manager
    pub async fn new(app_handle: AppHandle) -> Result<Self, StoryWeaverError> {
        Ok(Self { _app_handle: app_handle })
    }

    /// Save an API key to secure storage
    pub async fn save_api_key(&self, provider: ApiProvider, api_key: &str) -> Result<(), StoryWeaverError> {
        let key_name = match provider {
            ApiProvider::OpenAI => OPENAI_KEY,
            ApiProvider::Claude => CLAUDE_KEY,
        };
        
        let entry = Entry::new(SERVICE, key_name)
            .map_err(|e| StoryWeaverError::SecurityError {
                message: format!("Failed to create keyring entry: {}", e),
            })?;
        
        entry.set_password(api_key)
            .map_err(|e| {
                error!("Failed to save API key for {}: {}", key_name, e);
                StoryWeaverError::SecurityError {
                    message: format!("Failed to save API key: {}", e),
                }
            })?;
        
        info!("Successfully saved API key for provider: {:?}", provider);
        Ok(())
    }

    /// Get an API key from secure storage
    pub async fn get_api_key(&self, provider: ApiProvider) -> Result<Option<String>, StoryWeaverError> {
        let key_name = match provider {
            ApiProvider::OpenAI => OPENAI_KEY,
            ApiProvider::Claude => CLAUDE_KEY,
        };

        let entry = Entry::new(SERVICE, key_name)
            .map_err(|e| StoryWeaverError::SecurityError {
                message: format!("Failed to create keyring entry: {}", e),
            })?;
        
        match entry.get_password() {
            Ok(password) => {
                debug!("Successfully retrieved API key for provider: {:?}", provider);
                Ok(Some(password))
            },
            Err(KeyringError::NoEntry) => {
                debug!("No API key found for provider: {:?}", provider);
                Ok(None)
            },
            Err(e) => {
                error!("Failed to retrieve API key for {}: {}", key_name, e);
                Err(StoryWeaverError::SecurityError {
                    message: format!("Failed to retrieve API key: {}", e),
                })
            }
        }
    }

    /// Delete an API key from secure storage
    pub async fn delete_api_key(&self, provider: ApiProvider) -> Result<(), StoryWeaverError> {
        let key_name = match provider {
            ApiProvider::OpenAI => OPENAI_KEY,
            ApiProvider::Claude => CLAUDE_KEY,
        };

        let entry = Entry::new(SERVICE, key_name)
            .map_err(|e| StoryWeaverError::SecurityError {
                message: format!("Failed to create keyring entry: {}", e),
            })?;
        
        match entry.delete_password() {
            Ok(()) => {
                info!("Successfully deleted API key for provider: {:?}", provider);
                Ok(())
            },
            Err(KeyringError::NoEntry) => {
                debug!("No API key to delete for provider: {:?}", provider);
                Ok(()) // Not an error if key doesn't exist
            },
            Err(e) => {
                error!("Failed to delete API key for {}: {}", key_name, e);
                Err(StoryWeaverError::SecurityError {
                    message: format!("Failed to delete API key: {}", e),
                })
            }
        }
    }

    /// Check if an API key exists
    pub async fn has_api_key(&self, provider: ApiProvider) -> Result<bool, StoryWeaverError> {
        let key = self.get_api_key(provider).await?;
        Ok(key.is_some())
    }
}

/// Global instance of the API key manager
static API_KEY_MANAGER: OnceLock<Arc<ApiKeyManager>> = OnceLock::new();

/// Initialize the API key manager
pub async fn init(app_handle: AppHandle) -> Result<(), StoryWeaverError> {
    let manager = ApiKeyManager::new(app_handle).await?;
    
    API_KEY_MANAGER.set(Arc::new(manager))
        .map_err(|_| StoryWeaverError::SecurityError{ message: "API key manager already initialized".to_string() })?;
    
    Ok(())
}

/// Get the global API key manager instance
pub fn get_api_key_manager() -> Result<Arc<ApiKeyManager>, StoryWeaverError> {
    API_KEY_MANAGER.get().cloned().ok_or_else(|| {
        StoryWeaverError::SecurityError{ message: "API key manager not initialized".to_string() }
    })
}
