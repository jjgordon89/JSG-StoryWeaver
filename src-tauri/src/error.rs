//! StoryWeaver Error Types
//! Comprehensive error handling for all application components

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Main error type for StoryWeaver application
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StoryWeaverError {
    // Database Errors
    #[error("Database error: {message}")]
    Database { message: String },
    
    #[error("Database connection failed: {message}")]
    DatabaseConnection { message: String },
    
    #[error("Migration failed: {message}")]
    Migration { message: String },
    
    // File System Errors
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("File access denied: {path}")]
    FileAccessDenied { path: String },
    
    #[error("File operation failed: {operation} on {path} - {message}")]
    FileOperation {
        operation: String,
        path: String,
        message: String,
    },
    
    // Project Errors
    #[error("Project not found: {id}")]
    ProjectNotFound { id: String },
    
    #[error("Project already exists: {name}")]
    ProjectAlreadyExists { name: String },
    
    #[error("Invalid project configuration: {message}")]
    InvalidProjectConfig { message: String },
    
    // Document Errors
    #[error("Document not found: {id}")]
    DocumentNotFound { id: String },
    
    #[error("Document parsing failed: {format} - {message}")]
    DocumentParsing { format: String, message: String },
    
    #[error("Document too large: {size} bytes (max: {max_size})")]
    DocumentTooLarge { size: u64, max_size: u64 },
    
    // Folder Errors
    #[error("Folder not found: {id}")]
    FolderNotFound { id: String },
    
    #[error("Folder not empty: {id}")]
    FolderNotEmpty { id: String },
    
    // Series Errors
    #[error("Series not found: {id}")]
    SeriesNotFound { id: String },
    
    #[error("Series not empty: {id}")]
    SeriesNotEmpty { id: String },
    
    // Document Link Errors
    #[error("Document link not found: {id}")]
    DocumentLinkNotFound { id: String },
    
    // AI Provider Errors
    #[error("AI provider error: {provider} - {message}")]
    AIProvider { provider: String, message: String },
    
    #[error("AI request failed: {provider} - {status_code} - {message}")]
    AIRequest {
        provider: String,
        status_code: u16,
        message: String,
    },
    
    #[error("AI rate limit exceeded: {provider} - retry after {retry_after} seconds")]
    AIRateLimit {
        provider: String,
        retry_after: u64,
    },
    
    #[error("Invalid API key for provider: {provider}")]
    InvalidAPIKey { provider: String },
    
    #[error("Token limit exceeded: {used} / {limit} tokens")]
    TokenLimitExceeded { used: u32, limit: u32 },
    
    // AI Generation Errors
    #[error("AI generation failed: {message}")]
    AIGenerationError { message: String },
    
    #[error("AI generation timeout: {message}")]
    AIGenerationTimeout { message: String },
    
    #[error("AI content filtered: {reason}")]
    AIContentFiltered { reason: String },
    
    // Not Found Errors
    #[error("Resource not found: {resource_type} - {id}")]
    NotFound { resource_type: String, id: String },
    
    // Vector Database Errors
    #[error("Vector database error: {message}")]
    VectorDatabase { message: String },
    
    #[error("Embedding generation failed: {message}")]
    EmbeddingGeneration { message: String },
    
    #[error("Similarity search failed: {message}")]
    SimilaritySearch { message: String },
    
    // Plugin System Errors
    #[error("Plugin not found: {name}")]
    PluginNotFound { name: String },
    
    #[error("Plugin execution failed: {name} - {message}")]
    PluginExecution { name: String, message: String },
    
    #[error("Plugin security violation: {name} - {violation}")]
    PluginSecurity { name: String, violation: String },
    
    #[error("WASM runtime error: {message}")]
    WasmRuntime { message: String },
    
    // Security Errors
    #[error("Keychain access failed: {message}")]
    KeychainAccess { message: String },
    
    #[error("Encryption failed: {message}")]
    Encryption { message: String },
    
    #[error("Decryption failed: {message}")]
    Decryption { message: String },
    
    #[error("Security error: {message}")]
    SecurityError { message: String },
    
    #[error("Privacy error: {message}")]
    PrivacyError { message: String },
    
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Input validation failed: {field} - {message}")]
    InputValidation { field: String, message: String },
    
    // Network Errors
    #[error("Network error: {message}")]
    Network { message: String },
    
    #[error("Request timeout: {url}")]
    RequestTimeout { url: String },
    
    #[error("Connection failed: {url} - {message}")]
    ConnectionFailed { url: String, message: String },
    
    #[error("Failed to emit event: {0}")]
    EventEmitError(String),
    
    // Configuration Errors
    #[error("Configuration error: {key} - {message}")]
    Configuration { key: String, message: String },
    
    #[error("Missing required configuration: {key}")]
    MissingConfiguration { key: String },
    
    // Serialization Errors
    #[error("Serialization failed: {message}")]
    Serialization { message: String },
    
    #[error("Deserialization failed: {message}")]
    Deserialization { message: String },
    
    #[error("Parse error: {message}")]
    ParseError { message: String },
    
    #[error("Version not found: {id}")]
    VersionNotFound { id: String },
    
    #[error("Deleted item not found: {id}")]
    DeletedItemNotFound { id: String },
    
    #[error("Operation not supported: {operation}")]
    UnsupportedOperation { operation: String },
    
    // Generic Errors
    #[error("Internal error: {message}")]
    Internal { message: String },
    
    #[error("Operation not supported: {operation}")]
    NotSupported { operation: String },
    
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    
    #[error("Resource not available: {resource}")]
    ResourceUnavailable { resource: String },
}

impl StoryWeaverError {
    /// Create a database error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Self::Database {
            message: message.into(),
        }
    }
    
    /// Create a file operation error
    pub fn file_operation<S: Into<String>>(operation: S, path: S, message: S) -> Self {
        Self::FileOperation {
            operation: operation.into(),
            path: path.into(),
            message: message.into(),
        }
    }
    
    /// Create an AI provider error
    pub fn ai_provider<S: Into<String>>(provider: S, message: S) -> Self {
        Self::AIProvider {
            provider: provider.into(),
            message: message.into(),
        }
    }

    /// Create an AI generation error from a message
    pub fn ai<S: Into<String>>(message: S) -> Self {
        Self::AIGenerationError {
            message: message.into(),
        }
    }
    
    /// Create an internal error
    pub fn internal<S: Into<String>>(message: S) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
    
    /// Create a validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::InputValidation {
            field: "general".to_string(),
            message: message.into(),
        }
    }
    
    /// Create a serialization error
    pub fn serialization<S: Into<String>>(message: S) -> Self {
        Self::Serialization {
            message: message.into(),
        }
    }
    
    /// Create a deserialization error
    pub fn deserialization<S: Into<String>>(message: S) -> Self {
        Self::Deserialization {
            message: message.into(),
        }
    }
    
    /// Create a parse error
    pub fn parse_error<S: Into<String>>(message: S) -> Self {
        Self::ParseError {
            message: message.into(),
        }
    }
    
    /// Check if the error is recoverable (can be retried)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::Network { .. }
                | Self::RequestTimeout { .. }
                | Self::ConnectionFailed { .. }
                | Self::AIRateLimit { .. }
                | Self::DatabaseConnection { .. }
        )
    }
    
    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Self::FileNotFound { .. } => "The requested file could not be found.".to_string(),
            Self::FileAccessDenied { .. } => "Access to the file was denied. Please check permissions.".to_string(),
            Self::ProjectNotFound { .. } => "The requested project could not be found.".to_string(),
            Self::DocumentNotFound { .. } => "The requested document could not be found.".to_string(),
            Self::AIRateLimit { retry_after, .. } => {
                format!("AI service rate limit exceeded. Please try again in {} seconds.", retry_after)
            }
            Self::InvalidAPIKey { provider } => {
                format!("Invalid API key for {}. Please check your configuration.", provider)
            }
            Self::TokenLimitExceeded { .. } => "The request exceeds the token limit. Please reduce the content size.".to_string(),
            Self::Network { .. } => "Network connection failed. Please check your internet connection.".to_string(),
            Self::DatabaseConnection { .. } => "Database connection failed. Please try again.".to_string(),
            _ => "An unexpected error occurred. Please try again.".to_string(),
        }
    }
}

// Implement From traits for common error types
impl From<sqlx::Error> for StoryWeaverError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for StoryWeaverError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::FileNotFound {
                path: "unknown".to_string(),
            },
            std::io::ErrorKind::PermissionDenied => Self::FileAccessDenied {
                path: "unknown".to_string(),
            },
            _ => Self::FileOperation {
                operation: "unknown".to_string(),
                path: "unknown".to_string(),
                message: err.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for StoryWeaverError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for StoryWeaverError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            Self::RequestTimeout {
                url: err.url().map(|u| u.to_string()).unwrap_or_default(),
            }
        } else if err.is_connect() {
            Self::ConnectionFailed {
                url: err.url().map(|u| u.to_string()).unwrap_or_default(),
                message: err.to_string(),
            }
        } else {
            Self::Network {
                message: err.to_string(),
            }
        }
    }
}

// Additional From implementations for common types
impl From<String> for StoryWeaverError {
    fn from(err: String) -> Self {
        Self::Internal { message: err }
    }
}

impl From<&str> for StoryWeaverError {
    fn from(err: &str) -> Self {
        Self::Internal { message: err.to_string() }
    }
}

impl From<i64> for StoryWeaverError {
    fn from(err: i64) -> Self {
        Self::Internal { message: err.to_string() }
    }
}

impl From<i32> for StoryWeaverError {
    fn from(err: i32) -> Self {
        Self::Internal { message: err.to_string() }
    }
}

impl From<chrono::NaiveDateTime> for StoryWeaverError {
    fn from(err: chrono::NaiveDateTime) -> Self {
        Self::Internal { message: err.to_string() }
    }
}

/// Result type alias for StoryWeaver operations
pub type Result<T> = std::result::Result<T, StoryWeaverError>;

/// Helper macro for creating errors with context
#[macro_export]
macro_rules! sw_error {
    ($variant:ident, $($field:ident: $value:expr),*) => {
        $crate::error::StoryWeaverError::$variant {
            $($field: $value.into()),*
        }
    };
}

/// Helper macro for creating internal errors
#[macro_export]
macro_rules! internal_error {
    ($msg:expr) => {
        $crate::error::StoryWeaverError::internal($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::StoryWeaverError::internal(format!($fmt, $($arg)*))
    };
}
