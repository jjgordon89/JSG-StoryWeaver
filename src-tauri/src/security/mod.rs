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
pub mod rate_limit;
pub mod validators;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod rate_limit_tests;

pub use api_keys::*;
pub use encryption::*;
pub use validation::*;
pub use audit::*;
pub use privacy::*;
pub use rate_limit::*;
pub use validators::*;

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

/// Lightweight boolean wrapper used by various command handlers to quickly
/// assess if an input string is considered safe. Internally delegates to the
/// validation module and performs a null-byte check.
pub fn is_safe_input(input: &str) -> bool {
    if input.contains('\0') {
        return false;
    }
    validation::validate_security_input(input).is_ok()
}
