//! Secure API key storage and management
//!
//! This module provides functionality for securely storing and retrieving API keys
//! using the operating system's secure storage (keychain/credential store).

use crate::error::StoryWeaverError;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager};
use log;

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
    app_handle: AppHandle,
}

impl ApiKeyManager {
    /// Create a new API key manager
    pub async fn new(app_handle: AppHandle) -> Result<Self, StoryWeaverError> {
        Ok(Self { app_handle })
    }

    /// Save an API key to secure storage
    pub async fn save_api_key(&self, provider: ApiProvider, api_key: &str) -> Result<(), StoryWeaverError> {
        let key = match provider {
            ApiProvider::OpenAI => format!("{}-{}", SERVICE, OPENAI_KEY),
            ApiProvider::Claude => format!("{}-{}", SERVICE, CLAUDE_KEY),
        };
        
        // Note: Keychain functionality needs to be implemented with proper Tauri plugin
        // For now, we'll store in a secure location or use alternative storage
        // TODO: Implement proper keychain storage
        
        // Placeholder implementation - in production, use proper secure storage
        log::warn!("API key storage not yet implemented - using placeholder");
        
        // Store in app data directory with encryption (placeholder)
        // This should be replaced with proper keychain integration
            
        Ok(())
    }

    /// Get an API key from secure storage
    pub async fn get_api_key(&self, provider: ApiProvider) -> Result<Option<String>, StoryWeaverError> {
        let key = match provider {
            ApiProvider::OpenAI => format!("{}-{}", SERVICE, OPENAI_KEY),
            ApiProvider::Claude => format!("{}-{}", SERVICE, CLAUDE_KEY),
        };

        // TODO: Implement proper keychain retrieval
        log::warn!("API key retrieval not yet implemented - using placeholder");
        
        // Placeholder implementation - return None for now
        // In production, retrieve from secure storage
        Ok(None)
    }

    /// Delete an API key from secure storage
    pub async fn delete_api_key(&self, provider: ApiProvider) -> Result<(), StoryWeaverError> {
        let key = match provider {
            ApiProvider::OpenAI => format!("{}-{}", SERVICE, OPENAI_KEY),
            ApiProvider::Claude => format!("{}-{}", SERVICE, CLAUDE_KEY),
        };

        // TODO: Implement proper keychain deletion
        log::warn!("API key deletion not yet implemented - using placeholder");
        
        // Placeholder implementation - in production, delete from secure storage
        // This should be replaced with proper keychain integration
            
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
pub async fn init(app_handle: AppHandle) -> Result<(), StoryWeaverError> {
    let manager = ApiKeyManager::new(app_handle).await?;
    
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
