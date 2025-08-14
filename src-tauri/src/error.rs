//! StoryWeaver Error Types
//! Comprehensive error handling for all application components

use serde::{Deserialize, Serialize};
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
    
    #[error("Saliency engine error: {message}")]
    SaliencyEngineError { message: String },
    
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
    #[error("Authentication failed: {message}")]
    Authentication { message: String },
    
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
    /// Factory for not found errors (generic)
    pub fn not_found<S: Into<String>>(resource_type: S, id: S) -> Self {
        Self::NotFound {
            resource_type: resource_type.into(),
            id: id.into()
        }
    }

    /// Factory for project not found errors (strongly typed)
    pub fn project_not_found<S: Into<String>>(id: S) -> Self {
        Self::ProjectNotFound { id: id.into() }
    }

    /// Factory for folder not found errors
    pub fn folder_not_found<S: Into<String>>(id: S) -> Self {
        Self::FolderNotFound { id: id.into() }
    }

    /// Factory for series not found errors
    pub fn series_not_found<S: Into<String>>(id: S) -> Self {
        Self::SeriesNotFound { id: id.into() }
    }

    /// Factory for document not found errors
    pub fn document_not_found<S: Into<String>>(id: S) -> Self {
        Self::DocumentNotFound { id: id.into() }
    }

    /// Factory for document link not found errors
    pub fn document_link_not_found<S: Into<String>>(id: S) -> Self {
        Self::DocumentLinkNotFound { id: id.into() }
    }

    /// Factory for plugin not found errors
    pub fn plugin_not_found<S: Into<String>>(name: S) -> Self {
        Self::PluginNotFound { name: name.into() }
    }

    /// Factory for project already exists errors
    pub fn project_already_exists<S: Into<String>>(name: S) -> Self {
        Self::ProjectAlreadyExists { name: name.into() }
    }

    /// Factory for folder not empty errors
    pub fn folder_not_empty<S: Into<String>>(id: S) -> Self {
        Self::FolderNotEmpty { id: id.into() }
    }

    /// Factory for series not empty errors
    pub fn series_not_empty<S: Into<String>>(id: S) -> Self {
        Self::SeriesNotEmpty { id: id.into() }
    }

    /// Factory for version not found errors
    pub fn version_not_found<S: Into<String>>(id: S) -> Self {
        Self::VersionNotFound { id: id.into() }
    }

    /// Factory for deleted item not found errors
    pub fn deleted_item_not_found<S: Into<String>>(id: S) -> Self {
        Self::DeletedItemNotFound { id: id.into() }
    }

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
    
    /// Create a system error
    pub fn system<S: Into<String>>(message: S) -> Self {
        Self::Internal {
            message: message.into(),
        }
    }
    
    /// Create an invalid input error
    pub fn invalid_input<S: Into<String>>(message: S) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }
    
    /// Create a validation error with field
    pub fn input_validation<S: Into<String>>(field: S, message: S) -> Self {
        Self::InputValidation {
            field: field.into(),
            message: message.into(),
        }
    }
    
    /// Create a file not found error
    pub fn file_not_found<S: Into<String>>(path: S) -> Self {
        Self::FileNotFound {
            path: path.into(),
        }
    }
    
    /// Create a file access denied error
    pub fn file_access_denied<S: Into<String>>(path: S) -> Self {
        Self::FileAccessDenied {
            path: path.into(),
        }
    }
    
    /// Create a database connection error
    pub fn database_connection<S: Into<String>>(message: S) -> Self {
        Self::DatabaseConnection {
            message: message.into(),
        }
    }
    
    /// Create a migration error
    pub fn migration<S: Into<String>>(message: S) -> Self {
        Self::Migration {
            message: message.into(),
        }
    }
    
    /// Create an AI request error
    pub fn ai_request<S: Into<String>>(provider: S, status_code: u16, message: S) -> Self {
        Self::AIRequest {
            provider: provider.into(),
            status_code,
            message: message.into(),
        }
    }
    
    /// Create an AI rate limit error
    pub fn ai_rate_limit<S: Into<String>>(provider: S, retry_after: u64) -> Self {
        Self::AIRateLimit {
            provider: provider.into(),
            retry_after,
        }
    }
    
    /// Create an invalid API key error
    pub fn invalid_api_key<S: Into<String>>(provider: S) -> Self {
        Self::InvalidAPIKey {
            provider: provider.into(),
        }
    }
    
    /// Create a token limit exceeded error
    pub fn token_limit_exceeded(used: u32, limit: u32) -> Self {
        Self::TokenLimitExceeded {
            used,
            limit,
        }
    }
    
    /// Create an AI content filtered error
    pub fn ai_content_filtered<S: Into<String>>(reason: S) -> Self {
        Self::AIContentFiltered {
            reason: reason.into(),
        }
    }
    
    /// Create a vector database error
    pub fn vector_database<S: Into<String>>(message: S) -> Self {
        Self::VectorDatabase {
            message: message.into(),
        }
    }
    
    /// Create an embedding generation error
    pub fn embedding_generation<S: Into<String>>(message: S) -> Self {
        Self::EmbeddingGeneration {
            message: message.into(),
        }
    }
    
    /// Create a similarity search error
    pub fn similarity_search<S: Into<String>>(message: S) -> Self {
        Self::SimilaritySearch {
            message: message.into(),
        }
    }
    
    /// Create a plugin execution error
    pub fn plugin_execution<S: Into<String>>(name: S, message: S) -> Self {
        Self::PluginExecution {
            name: name.into(),
            message: message.into(),
        }
    }
    
    /// Create a plugin security violation error
    pub fn plugin_security<S: Into<String>>(name: S, violation: S) -> Self {
        Self::PluginSecurity {
            name: name.into(),
            violation: violation.into(),
        }
    }
    
    /// Create a WASM runtime error
    pub fn wasm_runtime<S: Into<String>>(message: S) -> Self {
        Self::WasmRuntime {
            message: message.into(),
        }
    }
    
    /// Create an authentication error
    pub fn authentication<S: Into<String>>(message: S) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }
    
    /// Create a keychain access error
    pub fn keychain_access<S: Into<String>>(message: S) -> Self {
        Self::KeychainAccess {
            message: message.into(),
        }
    }
    
    /// Create an encryption error
    pub fn encryption<S: Into<String>>(message: S) -> Self {
        Self::Encryption {
            message: message.into(),
        }
    }
    
    /// Create a decryption error
    pub fn decryption<S: Into<String>>(message: S) -> Self {
        Self::Decryption {
            message: message.into(),
        }
    }
    
    /// Create a security error
    pub fn security_error<S: Into<String>>(message: S) -> Self {
        Self::SecurityError {
            message: message.into(),
        }
    }
    
    /// Create a privacy error
    pub fn privacy_error<S: Into<String>>(message: S) -> Self {
        Self::PrivacyError {
            message: message.into(),
        }
    }
    
    /// Create a network error
    pub fn network<S: Into<String>>(message: S) -> Self {
        Self::Network {
            message: message.into(),
        }
    }
    
    /// Create a request timeout error
    pub fn request_timeout<S: Into<String>>(url: S) -> Self {
        Self::RequestTimeout {
            url: url.into(),
        }
    }
    
    /// Create a connection failed error
    pub fn connection_failed<S: Into<String>>(url: S, message: S) -> Self {
        Self::ConnectionFailed {
            url: url.into(),
            message: message.into(),
        }
    }
    
    /// Create an event emit error
    pub fn event_emit_error<S: Into<String>>(message: S) -> Self {
        Self::EventEmitError(message.into())
    }
    
    /// Create a configuration error
    pub fn configuration<S: Into<String>>(key: S, message: S) -> Self {
        Self::Configuration {
            key: key.into(),
            message: message.into(),
        }
    }
    
    /// Create a missing configuration error
    pub fn missing_configuration<S: Into<String>>(key: S) -> Self {
        Self::MissingConfiguration {
            key: key.into(),
        }
    }
    
    /// Create an unsupported operation error
    pub fn unsupported_operation<S: Into<String>>(operation: S) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
        }
    }
    
    /// Create a resource unavailable error
    pub fn resource_unavailable<S: Into<String>>(resource: S) -> Self {
        Self::ResourceUnavailable {
            resource: resource.into(),
        }
    }

    /// Helper: map any database error-like value into a Database error
    pub fn to_db_error<E: std::fmt::Display>(e: E) -> Self {
        Self::Database {
            message: e.to_string(),
        }
    }

    /// Helper: map an IO failure into a FileOperation error with context
    pub fn to_io_error<E: std::fmt::Display, S1: Into<String>, S2: Into<String>>(
        operation: S1,
        path: S2,
        e: E,
    ) -> Self {
        Self::FileOperation {
            operation: operation.into(),
            path: path.into(),
            message: e.to_string(),
        }
    }

    /// Helper: map any parsing failure into a ParseError
    pub fn to_parse_error<E: std::fmt::Display>(e: E) -> Self {
        Self::ParseError {
            message: e.to_string(),
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

impl From<&sqlx::Error> for StoryWeaverError {
    fn from(err: &sqlx::Error) -> Self {
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

impl From<&std::io::Error> for StoryWeaverError {
    fn from(err: &std::io::Error) -> Self {
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

impl From<&serde_json::Error> for StoryWeaverError {
    fn from(err: &serde_json::Error) -> Self {
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

impl From<&reqwest::Error> for StoryWeaverError {
    fn from(err: &reqwest::Error) -> Self {
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
        Self::internal(format!("Invalid datetime: {}", err))
    }
}

impl From<anyhow::Error> for StoryWeaverError {
    fn from(err: anyhow::Error) -> Self {
        // Check if the error is already a StoryWeaverError
        if let Some(sw_err) = err.downcast_ref::<StoryWeaverError>() {
            return sw_err.clone();
        }
        
        // Check for common error types that might be wrapped in anyhow
        if let Some(sqlx_err) = err.downcast_ref::<sqlx::Error>() {
            return sqlx_err.into();
        }
        
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return io_err.into();
        }
        
        if let Some(json_err) = err.downcast_ref::<serde_json::Error>() {
            return json_err.into();
        }
        
        if let Some(reqwest_err) = err.downcast_ref::<reqwest::Error>() {
            return reqwest_err.into();
        }
        
        // Default to internal error
        Self::internal(format!("Anyhow error: {}", err))
    }
}

impl From<Box<dyn std::error::Error>> for StoryWeaverError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::internal(err.to_string())
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
