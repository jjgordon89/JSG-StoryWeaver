//! Comprehensive logging system for StoryWeaver
//! 
//! This module provides structured logging capabilities using the tracing crate,
//! with support for different log levels, structured data, and performance monitoring.

use tracing::{info, warn, error, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Log levels for the application
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

/// Structured log entry for database storage
#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub fields: serde_json::Value,
}

/// Performance metrics for logging
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operation: String,
    pub duration_ms: u64,
    pub memory_used: Option<u64>,
    pub cpu_usage: Option<f64>,
    pub success: bool,
    pub error_message: Option<String>,
}

/// Initialize the logging system
pub fn init_logging(log_level: Option<LogLevel>) -> Result<(), Box<dyn std::error::Error>> {
    let level = log_level.unwrap_or(LogLevel::Info);
    
    // Create a filter based on the log level
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            match level {
                LogLevel::Trace => EnvFilter::new("trace"),
                LogLevel::Debug => EnvFilter::new("debug"),
                LogLevel::Info => EnvFilter::new("info"),
                LogLevel::Warn => EnvFilter::new("warn"),
                LogLevel::Error => EnvFilter::new("error"),
            }
        });

    // Initialize tracing subscriber with console output
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true))
        .init();

    info!("Logging system initialized with level: {:?}", level);
    Ok(())
}

/// Log application startup
pub fn log_startup(version: &str, build_info: &str) {
    info!(
        version = version,
        build_info = build_info,
        "StoryWeaver application starting"
    );
}

/// Log application shutdown
pub fn log_shutdown(reason: &str) {
    info!(
        reason = reason,
        "StoryWeaver application shutting down"
    );
}

/// Log database operations
pub fn log_database_operation(operation: &str, table: &str, duration_ms: u64, success: bool, error: Option<&str>) {
    if success {
        info!(
            operation = operation,
            table = table,
            duration_ms = duration_ms,
            "Database operation completed successfully"
        );
    } else {
        error!(
            operation = operation,
            table = table,
            duration_ms = duration_ms,
            error = error.unwrap_or("Unknown error"),
            "Database operation failed"
        );
    }
}

/// Log AI provider operations
pub fn log_ai_operation(provider: &str, operation: &str, model: Option<&str>, tokens_used: Option<u32>, cost: Option<f64>, duration_ms: u64, success: bool, error: Option<&str>) {
    if success {
        info!(
            provider = provider,
            operation = operation,
            model = model,
            tokens_used = tokens_used,
            cost = cost,
            duration_ms = duration_ms,
            "AI operation completed successfully"
        );
    } else {
        error!(
            provider = provider,
            operation = operation,
            model = model,
            duration_ms = duration_ms,
            error = error.unwrap_or("Unknown error"),
            "AI operation failed"
        );
    }
}

/// Log security events
pub fn log_security_event(event_type: &str, severity: &str, description: &str, user_id: Option<&str>) {
    match severity {
        "critical" | "high" => error!(
            event_type = event_type,
            severity = severity,
            description = description,
            user_id = user_id,
            "Security event detected"
        ),
        "medium" => warn!(
            event_type = event_type,
            severity = severity,
            description = description,
            user_id = user_id,
            "Security event detected"
        ),
        _ => info!(
            event_type = event_type,
            severity = severity,
            description = description,
            user_id = user_id,
            "Security event detected"
        ),
    }
}

/// Log performance metrics
pub fn log_performance_metrics(metrics: &PerformanceMetrics) {
    if metrics.success {
        info!(
            operation = metrics.operation,
            duration_ms = metrics.duration_ms,
            memory_used = metrics.memory_used,
            cpu_usage = metrics.cpu_usage,
            "Performance metrics recorded"
        );
    } else {
        warn!(
            operation = metrics.operation,
            duration_ms = metrics.duration_ms,
            memory_used = metrics.memory_used,
            cpu_usage = metrics.cpu_usage,
            error = metrics.error_message.as_deref().unwrap_or("Unknown error"),
            "Performance metrics recorded (with error)"
        );
    }
}

/// Log file operations
pub fn log_file_operation(operation: &str, path: &str, size: Option<u64>, duration_ms: u64, success: bool, error: Option<&str>) {
    if success {
        debug!(
            operation = operation,
            path = path,
            size = size,
            duration_ms = duration_ms,
            "File operation completed"
        );
    } else {
        error!(
            operation = operation,
            path = path,
            duration_ms = duration_ms,
            error = error.unwrap_or("Unknown error"),
            "File operation failed"
        );
    }
}

/// Log user actions
pub fn log_user_action(action: &str, resource_type: &str, resource_id: Option<&str>, user_id: Option<&str>) {
    info!(
        action = action,
        resource_type = resource_type,
        resource_id = resource_id,
        user_id = user_id,
        "User action performed"
    );
}

/// Log plugin operations
pub fn log_plugin_operation(plugin_name: &str, operation: &str, duration_ms: u64, success: bool, error: Option<&str>) {
    if success {
        info!(
            plugin_name = plugin_name,
            operation = operation,
            duration_ms = duration_ms,
            "Plugin operation completed"
        );
    } else {
        error!(
            plugin_name = plugin_name,
            operation = operation,
            duration_ms = duration_ms,
            error = error.unwrap_or("Unknown error"),
            "Plugin operation failed"
        );
    }
}

/// Log memory usage
pub fn log_memory_usage(component: &str, memory_mb: u64, peak_memory_mb: Option<u64>) {
    debug!(
        component = component,
        memory_mb = memory_mb,
        peak_memory_mb = peak_memory_mb,
        "Memory usage recorded"
    );
}

/// Log cache operations
pub fn log_cache_operation(cache_type: &str, operation: &str, hit_rate: Option<f64>, size: Option<usize>) {
    debug!(
        cache_type = cache_type,
        operation = operation,
        hit_rate = hit_rate,
        size = size,
        "Cache operation performed"
    );
}

/// Log network operations
pub fn log_network_operation(url: &str, method: &str, status_code: Option<u16>, duration_ms: u64, success: bool, error: Option<&str>) {
    if success {
        debug!(
            url = url,
            method = method,
            status_code = status_code,
            duration_ms = duration_ms,
            "Network operation completed"
        );
    } else {
        error!(
            url = url,
            method = method,
            status_code = status_code,
            duration_ms = duration_ms,
            error = error.unwrap_or("Unknown error"),
            "Network operation failed"
        );
    }
}

/// Macro for timing operations
#[macro_export]
macro_rules! time_operation {
    ($operation:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();
        
        match &result {
            Ok(_) => {
                tracing::debug!(
                    operation = $operation,
                    duration_ms = duration.as_millis() as u64,
                    "Operation completed successfully"
                );
            }
            Err(e) => {
                tracing::error!(
                    operation = $operation,
                    duration_ms = duration.as_millis() as u64,
                    error = %e,
                    "Operation failed"
                );
            }
        }
        
        result
    }};
}

/// Macro for logging function entry and exit
#[macro_export]
macro_rules! trace_function {
    ($func_name:expr) => {
        tracing::trace!(function = $func_name, "Entering function");
        
        // Create a guard that logs on drop
        struct FunctionTraceGuard<'a> {
            func_name: &'a str,
        }
        
        impl<'a> Drop for FunctionTraceGuard<'a> {
            fn drop(&mut self) {
                tracing::trace!(function = self.func_name, "Exiting function");
            }
        }
        
        let _guard = FunctionTraceGuard { func_name: $func_name };
    };
}

/// Structured error logging
pub fn log_error_with_context(error: &dyn std::error::Error, context: &str, additional_fields: Option<serde_json::Value>) {
    error!(
        error = %error,
        context = context,
        fields = ?additional_fields,
        "Error occurred with context"
    );
}

/// Log configuration changes
pub fn log_config_change(setting: &str, old_value: Option<&str>, new_value: &str, user_id: Option<&str>) {
    info!(
        setting = setting,
        old_value = old_value,
        new_value = new_value,
        user_id = user_id,
        "Configuration changed"
    );
}

/// Log backup operations
pub fn log_backup_operation(operation: &str, backup_name: &str, size_mb: Option<u64>, duration_ms: u64, success: bool, error: Option<&str>) {
    if success {
        info!(
            operation = operation,
            backup_name = backup_name,
            size_mb = size_mb,
            duration_ms = duration_ms,
            "Backup operation completed"
        );
    } else {
        error!(
            operation = operation,
            backup_name = backup_name,
            duration_ms = duration_ms,
            error = error.unwrap_or("Unknown error"),
            "Backup operation failed"
        );
    }
}
