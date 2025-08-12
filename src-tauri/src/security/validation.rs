//! Input sanitization and validation
//!
//! This module provides functionality for validating and sanitizing user input
//! to prevent security vulnerabilities such as SQL injection and XSS attacks.

use crate::error::StoryWeaverError;
use regex::Regex;
use lazy_static::lazy_static;
use std::path::Path;

#[allow(clippy::unwrap_used)]
lazy_static! {
    // Enhanced regex patterns for validation
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref FILENAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_\-\. ]+$").unwrap();
    static ref PATH_TRAVERSAL_REGEX: Regex = Regex::new(r"(\.\.[\\/]|\\|/\.\./|^\.\./|/\.\.\\)").unwrap();
    static ref SQL_INJECTION_REGEX: Regex = Regex::new(r"(?i)('|--|;|/\*|\*/|xp_|sp_|exec|execute|select|insert|update|delete|drop|create|alter|union|script|javascript|vbscript)").unwrap();
    static ref HTML_TAG_REGEX: Regex = Regex::new(r"<[^>]*>").unwrap();
    static ref XSS_REGEX: Regex = Regex::new(r"(?i)(javascript:|data:|vbscript:|on\w+\s*=|<script|</script>|<iframe|</iframe>|<object|</object>|<embed|</embed>)").unwrap();
    static ref API_KEY_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9\-_]{8,128}$").unwrap();
    static ref SAFE_NAME_REGEX: Regex = Regex::new(r"^[\p{L}\p{N}_\-\. ]{1,100}$").unwrap();
}

/// Validate an email address
pub fn validate_email(email: &str) -> Result<(), StoryWeaverError> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(StoryWeaverError::validation("Invalid email address format"));
    }
    Ok(())
}

/// Validate a filename
pub fn validate_filename(filename: &str) -> Result<(), StoryWeaverError> {
    if filename.is_empty() {
        return Err(StoryWeaverError::validation("Filename cannot be empty"));
    }
    
    if !FILENAME_REGEX.is_match(filename) {
        return Err(StoryWeaverError::validation("Filename contains invalid characters"));
    }
    
    // Check for Windows reserved filenames
    let filename_upper = filename.to_uppercase();
    let reserved_names = ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"];
    
    for reserved in &reserved_names {
        if filename_upper == *reserved || filename_upper.starts_with(&format!("{}." , reserved)) {
            return Err(StoryWeaverError::validation("Filename uses a reserved system name"));
        }
    }
    
    Ok(())
}

/// Validate a file path to prevent path traversal attacks
pub fn validate_path(path: &str) -> Result<(), StoryWeaverError> {
    if path.is_empty() {
        return Err(StoryWeaverError::validation("Path cannot be empty"));
    }
    
    if PATH_TRAVERSAL_REGEX.is_match(path) {
        return Err(StoryWeaverError::validation("Path contains directory traversal sequences"));
    }
    
    // Allow both relative and absolute paths, but ensure they don't contain traversal sequences
    // The PATH_TRAVERSAL_REGEX already handles the main security concerns
    
    Ok(())
}

/// Sanitize text input to prevent SQL injection
pub fn sanitize_sql_input(input: &str) -> String {
    // First remove SQL injection patterns, then escape remaining single quotes
    let cleaned = SQL_INJECTION_REGEX.replace_all(input, "");
    cleaned.replace("'", "''") // Escape single quotes for SQL
}

/// Sanitize text input to prevent XSS attacks
pub fn sanitize_html(input: &str) -> String {
    // Remove HTML tags and XSS patterns
    let no_tags = HTML_TAG_REGEX.replace_all(input, "");
    let no_xss = XSS_REGEX.replace_all(&no_tags, "");
    // Escape remaining dangerous characters
    no_xss.replace("&", "&amp;")
          .replace("<", "&lt;")
          .replace(">", "&gt;")
          .replace("\"", "&quot;")
          .replace("'", "&#x27;")
}

/// Enhanced XSS detection and prevention
pub fn detect_xss_attempt(input: &str) -> bool {
    XSS_REGEX.is_match(input)
}

/// Validate input for potential security threats
pub fn validate_security_input(input: &str) -> Result<(), StoryWeaverError> {
    if SQL_INJECTION_REGEX.is_match(input) {
        return Err(StoryWeaverError::validation("Input contains potential SQL injection patterns"));
    }
    
    if detect_xss_attempt(input) {
        return Err(StoryWeaverError::validation("Input contains potential XSS patterns"));
    }
    
    Ok(())
}

/// Validate API key format with enhanced security checks
pub fn validate_api_key(api_key: &str) -> Result<(), StoryWeaverError> {
    // Check for empty or whitespace-only keys
    if api_key.trim().is_empty() {
        return Err(StoryWeaverError::validation("API key cannot be empty"));
    }
    
    // Check length constraints
    if api_key.len() < 8 {
        return Err(StoryWeaverError::validation("API key is too short (minimum 8 characters)"));
    }
    
    if api_key.len() > 128 {
        return Err(StoryWeaverError::validation("API key is too long (maximum 128 characters)"));
    }
    
    // Use enhanced regex for validation
    if !API_KEY_REGEX.is_match(api_key) {
        return Err(StoryWeaverError::validation("API key contains invalid characters (only alphanumeric, hyphens, and underscores allowed)"));
    }
    
    // Check for common weak patterns
    if let Some(first) = api_key.chars().next() {
        if api_key.chars().all(|c| c == first) {
            return Err(StoryWeaverError::validation("API key appears to be a repeated character pattern"));
        }
    }
    
    // Check for obvious test/placeholder values
    let lowercase_key = api_key.to_lowercase();
    if lowercase_key.contains("test") || lowercase_key.contains("demo") || 
       lowercase_key.contains("example") || lowercase_key.contains("placeholder") {
        return Err(StoryWeaverError::validation("API key appears to be a test or placeholder value"));
    }
    
    Ok(())
}

/// Generic name validation with enhanced security
pub fn validate_safe_name(name: &str, name_type: &str) -> Result<(), StoryWeaverError> {
    // Check for empty or whitespace-only names
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation(format!("{} cannot be empty", name_type)));
    }
    
    // Use enhanced regex for validation
    if !SAFE_NAME_REGEX.is_match(name) {
        return Err(StoryWeaverError::validation(format!("{} contains invalid characters or exceeds length limit", name_type)));
    }
    
    // Additional security checks
    validate_security_input(name)?;
    
    // Check for reserved names
    let lowercase_name = name.to_lowercase();
    let reserved_names = ["con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9"];
    if reserved_names.contains(&lowercase_name.as_str()) {
        return Err(StoryWeaverError::validation(format!("{} uses a reserved system name", name_type)));
    }
    
    Ok(())
}

/// Validate project name
pub fn validate_project_name(name: &str) -> Result<(), StoryWeaverError> {
    validate_safe_name(name, "Project name")
}

/// Validate document name
pub fn validate_document_name(name: &str) -> Result<(), StoryWeaverError> {
    validate_safe_name(name, "Document name")
}

/// Validate folder name
pub fn validate_folder_name(name: &str) -> Result<(), StoryWeaverError> {
    validate_safe_name(name, "Folder name")
}

/// Validate series name
pub fn validate_series_name(name: &str) -> Result<(), StoryWeaverError> {
    validate_safe_name(name, "Series name")
}

/// Validate content length with enhanced security checks
pub fn validate_content_length(content: &str, max_length: usize) -> Result<(), StoryWeaverError> {
    // Check for null bytes which can cause security issues
    if content.contains('\0') {
        return Err(StoryWeaverError::validation("Content contains null bytes which are not allowed"));
    }
    
    // Check content length
    if content.len() > max_length {
        return Err(StoryWeaverError::validation(format!("Content exceeds maximum length of {} characters", max_length)));
    }
    
    // Check for excessive whitespace (potential DoS)
    let whitespace_count = content.chars().filter(|c| c.is_whitespace()).count();
    let total_chars = content.chars().count();
    if total_chars > 0 && (whitespace_count as f64 / total_chars as f64) > 0.95 {
        return Err(StoryWeaverError::validation("Content contains excessive whitespace"));
    }
    
    // Check for potential XSS in content
    if detect_xss_attempt(content) {
        return Err(StoryWeaverError::validation("Content contains potentially malicious scripts"));
    }
    
    Ok(())
}

/// Initialize the validation module
pub async fn init() -> Result<(), StoryWeaverError> {
    // Test regex compilation to ensure they're valid
    lazy_static::initialize(&EMAIL_REGEX);
    lazy_static::initialize(&FILENAME_REGEX);
    lazy_static::initialize(&PATH_TRAVERSAL_REGEX);
    lazy_static::initialize(&SQL_INJECTION_REGEX);
    lazy_static::initialize(&XSS_REGEX);
    lazy_static::initialize(&API_KEY_REGEX);
    lazy_static::initialize(&SAFE_NAME_REGEX);
    
    // Log initialization
    log::info!("Security validation module initialized successfully");
    Ok(())
}
