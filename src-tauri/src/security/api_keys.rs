//! Secure API key storage and management
//!
//! This module provides functionality for securely storing and retrieving API keys
//! using the operating system's secure storage (keychain/credential store).

use tauri_plugin_keychain::Keychain;
use crate::error::StoryWeaverError;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

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
pub struct ApiKeyManager;

impl ApiKeyManager {
    /// Create a new API key manager
    pub async fn new() -> Result<Self, StoryWeaverError> {
        Ok(Self)
    }

    /// Save an API key to secure storage
    pub async fn save_api_key(&self, provider: ApiProvider, api_key: &str) -> Result<(), StoryWeaverError> {
        let key = match provider {
            ApiProvider::OpenAI => OPENAI_KEY,
            ApiProvider::Claude => CLAUDE_KEY,
        };
        
        Keychain::set(SERVICE, key, api_key)
            .map_err(|e| StoryWeaverError::SecurityError{ message: format!("Failed to save API key: {}", e) })?;
            
        Ok(())
    }

    /// Get an API key from secure storage
    pub async fn get_api_key(&self, provider: ApiProvider) -> Result<Option<String>, StoryWeaverError> {
        let key = match provider {
            ApiProvider::OpenAI => OPENAI_KEY,
            ApiProvider::Claude => CLAUDE_KEY,
        };

        match Keychain::get(SERVICE, key) {
            Ok(Some(password)) => Ok(Some(password)),
            Ok(None) => Ok(None),
            Err(e) => Err(StoryWeaverError::SecurityError{ message: format!("Failed to get API key: {}", e) }),
        }
    }

    /// Delete an API key from secure storage
    pub async fn delete_api_key(&self, provider: ApiProvider) -> Result<(), StoryWeaverError> {
        let key = match provider {
            ApiProvider::OpenAI => OPENAI_KEY,
            ApiProvider::Claude => CLAUDE_KEY,
        };

        Keychain::delete(SERVICE, key)
            .map_err(|e| StoryWeaverError::SecurityError{ message: format!("Failed to delete API key: {}", e) })?;
            
        Ok(())
    }

    /// Check if an API key exists
    pub async fn has_api_key(&self, provider: ApiProvider) -> Result<bool, StoryWeaverError> {
        let key = self.get_api_key(provider).await?;
        Ok(key.is_some())
    }
}

/// Global instance of the API key manager
static mut API_KEY_MANAGER: Option<Arc<ApiKeyManager>> = None;

/// Initialize the API key manager
pub async fn init() -> Result<(), StoryWeaverError> {
    let manager = ApiKeyManager::new().await?;
    
    unsafe {
        API_KEY_MANAGER = Some(Arc::new(manager));
    }
    
    Ok(())
}

/// Get the global API key manager instance
pub fn get_api_key_manager() -> Result<Arc<ApiKeyManager>, StoryWeaverError> {
    unsafe {
        match &API_KEY_MANAGER {
            Some(manager) => Ok(manager.clone()),
            None => Err(StoryWeaverError::SecurityError{ message: "API key manager not initialized".to_string() }),
        }
    }
}
