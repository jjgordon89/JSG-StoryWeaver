//! Audit logging for security events
//!
//! This module provides functionality for logging security-related events
//! such as login attempts, API key changes, and other sensitive operations.

use crate::error::StoryWeaverError;
use crate::database::get_pool;
use sqlx::{Pool, Sqlite};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

/// Audit event severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl ToString for AuditSeverity {
    fn to_string(&self) -> String {
        match self {
            AuditSeverity::Info => "info".to_string(),
            AuditSeverity::Warning => "warning".to_string(),
            AuditSeverity::Error => "error".to_string(),
            AuditSeverity::Critical => "critical".to_string(),
        }
    }
}

/// Audit event categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditCategory {
    Authentication,
    Authorization,
    DataAccess,
    Configuration,
    ApiKeyChange,
    UserAction,
    SystemEvent,
}

impl ToString for AuditCategory {
    fn to_string(&self) -> String {
        match self {
            AuditCategory::Authentication => "authentication".to_string(),
            AuditCategory::Authorization => "authorization".to_string(),
            AuditCategory::DataAccess => "data_access".to_string(),
            AuditCategory::Configuration => "configuration".to_string(),
            AuditCategory::ApiKeyChange => "api_key_change".to_string(),
            AuditCategory::UserAction => "user_action".to_string(),
            AuditCategory::SystemEvent => "system_event".to_string(),
        }
    }
}

/// Audit event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Option<i64>,
    pub event_type: String,
    pub category: AuditCategory,
    pub severity: AuditSeverity,
    pub description: String,
    pub context_data: Option<String>,
    pub project_id: Option<i64>,
    pub document_id: Option<i64>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Audit logger for security events
#[derive(Debug)]
pub struct AuditLogger {
    pool: Arc<Pool<Sqlite>>,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }

    /// Log an audit event
    pub async fn log_event(&self, event: AuditEvent) -> Result<i64, StoryWeaverError> {
        let category_str = event.category.to_string();
        let severity_str = event.severity.to_string();
        
        let event_id = sqlx::query!(
            r#"
            INSERT INTO audit_logs (
                event_type, category, severity, description, context_data, 
                project_id, document_id, created_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
            RETURNING id
            "#,
            event.event_type,
            category_str,
            severity_str,
            event.description,
            event.context_data,
            event.project_id,
            event.document_id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| StoryWeaverError::Database { message: format!("Failed to log audit event: {}", e) })?
        .id;

        Ok(event_id)
    }

    /// Get audit events with optional filtering
    pub async fn get_events(
        &self,
        category: Option<AuditCategory>,
        severity: Option<AuditSeverity>,
        limit: Option<i64>,
    ) -> Result<Vec<AuditEvent>, StoryWeaverError> {
        let category_str = category.map(|c| c.to_string());
        let severity_str = severity.map(|s| s.to_string());
        let limit_val = limit.unwrap_or(100);

        let rows = sqlx::query!(
            r#"
            SELECT 
                id, event_type, category, severity, description, 
                context_data, project_id, document_id, created_at
            FROM audit_logs
            WHERE 
                (? IS NULL OR category = ?) AND
                (? IS NULL OR severity = ?)
            ORDER BY created_at DESC
            LIMIT ?
            "#,
            category_str,
            category_str,
            severity_str,
            severity_str,
            limit_val
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| StoryWeaverError::Database { message: format!("Failed to fetch audit events: {}", e) })?;

        let events = rows
            .into_iter()
            .map(|row| {
                let category = match row.category.as_str() {
                    "authentication" => AuditCategory::Authentication,
                    "authorization" => AuditCategory::Authorization,
                    "data_access" => AuditCategory::DataAccess,
                    "configuration" => AuditCategory::Configuration,
                    "api_key_change" => AuditCategory::ApiKeyChange,
                    "user_action" => AuditCategory::UserAction,
                    "system_event" => AuditCategory::SystemEvent,
                    _ => AuditCategory::SystemEvent,
                };

                let severity = match row.severity.as_str() {
                    "info" => AuditSeverity::Info,
                    "warning" => AuditSeverity::Warning,
                    "error" => AuditSeverity::Error,
                    "critical" => AuditSeverity::Critical,
                    _ => AuditSeverity::Info,
                };

                AuditEvent {
                    id: row.id,
                    event_type: row.event_type,
                    category,
                    severity,
                    description: row.description,
                    context_data: row.context_data,
                    project_id: row.project_id,
                    document_id: row.document_id,
                    created_at: row.created_at.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
                }
            })
            .collect();

        Ok(events)
    }
}

/// Global instance of the audit logger
static mut AUDIT_LOGGER: Option<Arc<AuditLogger>> = None;

/// Initialize the audit logger
pub async fn init() -> Result<(), StoryWeaverError> {
    // Create the audit_logs table if it doesn't exist
    let pool = get_pool()?;
    
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY,
            event_type TEXT NOT NULL,
            category TEXT NOT NULL,
            severity TEXT NOT NULL,
            description TEXT NOT NULL,
            context_data TEXT,
            project_id INTEGER,
            document_id INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (document_id) REFERENCES documents(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::Database { message: format!("Failed to create audit_logs table: {}", e) })?;

    // Create index on created_at for faster queries
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at)"
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::Database { message: format!("Failed to create audit logs index: {}", e) })?;

    let logger = AuditLogger::new(pool);
    
    unsafe {
        AUDIT_LOGGER = Some(Arc::new(logger));
    }
    
    Ok(())
}

/// Get the global audit logger instance
pub fn get_audit_logger() -> Result<Arc<AuditLogger>, StoryWeaverError> {
    unsafe {
        match &AUDIT_LOGGER {
            Some(logger) => Ok(logger.clone()),
            None => Err(StoryWeaverError::SecurityError{ message: "Audit logger not initialized".to_string() }),
        }
    }
}

/// Log an authentication event
pub async fn log_auth_event(
    event_type: &str,
    description: &str,
    severity: AuditSeverity,
    context_data: Option<String>,
) -> Result<(), StoryWeaverError> {
    let logger = get_audit_logger()?;
    
    let event = AuditEvent {
        id: None,
        event_type: event_type.to_string(),
        category: AuditCategory::Authentication,
        severity,
        description: description.to_string(),
        context_data,
        project_id: None,
        document_id: None,
        created_at: None,
    };
    
    logger.log_event(event).await?;
    
    Ok(())
}

/// Log an API key change event
pub async fn log_api_key_event(
    event_type: &str,
    description: &str,
    severity: AuditSeverity,
) -> Result<(), StoryWeaverError> {
    let logger = get_audit_logger()?;
    
    let event = AuditEvent {
        id: None,
        event_type: event_type.to_string(),
        category: AuditCategory::ApiKeyChange,
        severity,
        description: description.to_string(),
        context_data: None,
        project_id: None,
        document_id: None,
        created_at: None,
    };
    
    logger.log_event(event).await?;
    
    Ok(())
}

/// Log a data access event
pub async fn log_data_access(
    event_type: &str,
    description: &str,
    project_id: Option<i64>,
    document_id: Option<i64>,
    severity: AuditSeverity,
) -> Result<(), StoryWeaverError> {
    let logger = get_audit_logger()?;
    
    let event = AuditEvent {
        id: None,
        event_type: event_type.to_string(),
        category: AuditCategory::DataAccess,
        severity,
        description: description.to_string(),
        context_data: None,
        project_id,
        document_id,
        created_at: None,
    };
    
    logger.log_event(event).await?;
    
    Ok(())
}
