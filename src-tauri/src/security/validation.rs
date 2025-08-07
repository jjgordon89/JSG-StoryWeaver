//! Input sanitization and validation
//!
//! This module provides functionality for validating and sanitizing user input
//! to prevent security vulnerabilities such as SQL injection and XSS attacks.

use crate::error::StoryWeaverError;
use regex::Regex;
use lazy_static::lazy_static;
use std::path::Path;

lazy_static! {
    // Regex patterns for validation
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref FILENAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_\-\. ]+$").unwrap();
    static ref PATH_TRAVERSAL_REGEX: Regex = Regex::new(r"(\.\.[\\/])").unwrap();
    static ref SQL_INJECTION_REGEX: Regex = Regex::new(r"('|--|;|/\*|\*/|xp_)").unwrap();
    static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]*>").unwrap();
}

/// Validate an email address
pub fn validate_email(email: &str) -> Result<(), StoryWeaverError> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(StoryWeaverError::ValidationError{ message: "Invalid email address format".to_string() });
    }
    Ok(())
}

/// Validate a filename
pub fn validate_filename(filename: &str) -> Result<(), StoryWeaverError> {
    if filename.is_empty() {
        return Err(StoryWeaverError::ValidationError{ message: "Filename cannot be empty".to_string() });
    }
    
    if !FILENAME_REGEX.is_match(filename) {
        return Err(StoryWeaverError::ValidationError{ message: "Filename contains invalid characters".to_string() });
    }
    
    Ok(())
}

/// Validate a file path to prevent path traversal attacks
pub fn validate_path(path: &str) -> Result<(), StoryWeaverError> {
    if PATH_TRAVERSAL_REGEX.is_match(path) {
        return Err(StoryWeaverError::ValidationError{ message: "Path contains directory traversal sequences".to_string() });
    }
    
    // Ensure the path is within the allowed directories
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        return Err(StoryWeaverError::ValidationError{ message: "Absolute paths are not allowed".to_string() });
    }
    
    Ok(())
}

/// Sanitize text input to prevent SQL injection
pub fn sanitize_sql_input(input: &str) -> String {
    SQL_INJECTION_REGEX.replace_all(input, "").to_string()
}

/// Sanitize text input to prevent XSS attacks
pub fn sanitize_html(input: &str) -> String {
    HTML_TAG_REGEX.replace_all(input, "").to_string()
}

/// Validate API key format
pub fn validate_api_key(api_key: &str) -> Result<(), StoryWeaverError> {
    // Basic validation for API keys
    if api_key.len() < 8 {
        return Err(StoryWeaverError::ValidationError{ message: "API key is too short".to_string() });
    }
    
    if api_key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        Ok(())
    } else {
        Err(StoryWeaverError::ValidationError{ message: "API key contains invalid characters".to_string() })
    }
}

/// Validate project name
pub fn validate_project_name(name: &str) -> Result<(), StoryWeaverError> {
    if name.is_empty() {
        return Err(StoryWeaverError::ValidationError{ message: "Project name cannot be empty".to_string() });
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::ValidationError{ message: "Project name is too long".to_string() });
    }
    
    Ok(())
}

/// Validate document name
pub fn validate_document_name(name: &str) -> Result<(), StoryWeaverError> {
    if name.is_empty() {
        return Err(StoryWeaverError::ValidationError{ message: "Document name cannot be empty".to_string() });
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::ValidationError{ message: "Document name is too long".to_string() });
    }
    
    Ok(())
}

/// Validate folder name
pub fn validate_folder_name(name: &str) -> Result<(), StoryWeaverError> {
    if name.is_empty() {
        return Err(StoryWeaverError::ValidationError{ message: "Folder name cannot be empty".to_string() });
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::ValidationError{ message: "Folder name is too long".to_string() });
    }
    
    Ok(())
}

/// Validate series name
pub fn validate_series_name(name: &str) -> Result<(), StoryWeaverError> {
    if name.is_empty() {
        return Err(StoryWeaverError::ValidationError{ message: "Series name cannot be empty".to_string() });
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::ValidationError{ message: "Series name is too long".to_string() });
    }
    
    Ok(())
}

/// Validate text content length
pub fn validate_content_length(content: &str, max_length: usize) -> Result<(), StoryWeaverError> {
    if content.len() > max_length {
        return Err(StoryWeaverError::ValidationError{ message: format!("Content exceeds maximum length of {} characters", max_length) });
    }
    
    Ok(())
}

/// Initialize the validation module
pub async fn init() -> Result<(), StoryWeaverError> {
    // No initialization needed for this module
    Ok(())
}
