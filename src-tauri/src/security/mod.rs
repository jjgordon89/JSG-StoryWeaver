//! Security module for StoryWeaver
//! 
//! This module provides security-related functionality including:
//! - API key storage and management
//! - Data encryption for sensitive information
//! - Input sanitization and validation
//! - Audit logging for security events
//! - Privacy-first data handling

pub mod api_keys;
pub mod encryption;
pub mod validation;
pub mod audit;
pub mod privacy;

pub use api_keys::*;
pub use encryption::*;
pub use validation::*;
pub use audit::*;
pub use privacy::*;

use crate::error::StoryWeaverError;
use tauri::AppHandle;

/// Initialize the security module
pub async fn init(app_handle: &AppHandle) -> Result<(), StoryWeaverError> {
    // Initialize encryption
    encryption::init(app_handle).await?;
    
    // Initialize API key manager
    api_keys::init(app_handle.clone()).await?;
    
    // Initialize audit logging
    audit::init().await?;
    
    Ok(())
}
