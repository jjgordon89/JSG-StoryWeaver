//! Security module for StoryWeaver
//! 
//! This module provides security-related functionality including:
//! - API key storage and management
//! - Data encryption for sensitive information
//! - Input sanitization and validation
//! - Audit logging for security events
//! - Privacy-first data handling

mod api_keys;
mod encryption;
mod validation;
mod audit;
mod privacy;

pub use api_keys::*;
pub use encryption::*;
pub use validation::*;
pub use audit::*;
pub use privacy::*;

use crate::error::StoryWeaverError;

/// Initialize the security module
pub async fn init() -> Result<(), StoryWeaverError> {
    // Initialize encryption
    encryption::init().await?;
    
    // Initialize audit logging
    audit::init().await?;
    
    Ok(())
}
