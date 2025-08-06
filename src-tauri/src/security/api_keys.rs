//! Secure API key storage and management
//!
//! This module provides functionality for securely storing and retrieving API keys
//! using the operating system's secure storage (keychain/credential store).

use tauri::api::secret::{SecretKey, SecretStore};
use crate::error::StoryWeaverError;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

const SECRET_STORE_NAME: &str = "storyweaver_secrets";
const OPENAI_API_KEY_ID: &str = "openai_api_key";
const CLAUDE_API_KEY_ID: &str = "claude_api_key";

/// Supported API providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiProvider {
    OpenAI,
    Claude,
}

/// API key manager for secure storage and retrieval
#[derive(Debug)]
pub struct ApiKeyManager {
    store: Arc<RwLock<SecretStore>>,
}

impl ApiKeyManager {
    /// Create a new API key manager
    pub async fn new() -> Result<Self, StoryWeaverError> {
        let store = SecretStore::new(SECRET_STORE_NAME)
            .map_err(|e| StoryWeaverError::SecurityError(format!("Failed to create secret store: {}", e)))?;
        
        Ok(Self {
            store: Arc::new(RwLock::new(store)),
        })
    }

    /// Save an API key to secure storage
    pub async fn save_api_key(&self, provider: ApiProvider, api_key: &str) -> Result<(), StoryWeaverError> {
        let key_id = match provider {
            ApiProvider::OpenAI => OPENAI_API_KEY_ID,
            ApiProvider::Claude => CLAUDE_API_KEY_ID,
        };

        let mut store = self.store.write().await;
        let secret_key = SecretKey::new(api_key.to_string());
        
        store.save(key_id, secret_key)
            .map_err(|e| StoryWeaverError::SecurityError(format!("Failed to save API key: {}", e)))?;
        
        Ok(())
    }

    /// Get an API key from secure storage
    pub async fn get_api_key(&self, provider: ApiProvider) -> Result<Option<String>, StoryWeaverError> {
        let key_id = match provider {
            ApiProvider::OpenAI => OPENAI_API_KEY_ID,
            ApiProvider::Claude => CLAUDE_API_KEY_ID,
        };

        let store = self.store.read().await;
        
        match store.get(key_id) {
            Ok(secret) => Ok(Some(secret.value().to_string())),
            Err(_) => Ok(None), // Key not found
        }
    }

    /// Delete an API key from secure storage
    pub async fn delete_api_key(&self, provider: ApiProvider) -> Result<(), StoryWeaverError> {
        let key_id = match provider {
            ApiProvider::OpenAI => OPENAI_API_KEY_ID,
            ApiProvider::Claude => CLAUDE_API_KEY_ID,
        };

        let mut store = self.store.write().await;
        
        store.delete(key_id)
            .map_err(|e| StoryWeaverError::SecurityError(format!("Failed to delete API key: {}", e)))?;
        
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
            None => Err(StoryWeaverError::SecurityError("API key manager not initialized".to_string())),
        }
    }
}
