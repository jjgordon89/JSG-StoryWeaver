//! Performance monitoring commands for StoryWeaver
//! Provides commands for tracking and analyzing application performance

use crate::database::models::*;
use crate::database::operations::*;
use crate::error::Result;
use crate::commands::CommandResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;

/// Record a performance metric
#[tauri::command]
pub async fn record_performance_metric(
    metric_name: String,
    metric_value: f64,
    metric_unit: Option<String>,
    component: String,
    context_data: Option<String>,
) -> Result<PerformanceMetric> {
    async fn record(
        metric_name: String,
        metric_value: f64,
        metric_unit: Option<String>,
        component: String,
        context_data: Option<String>,
    ) -> Result<PerformanceMetric> {
        // Parse component type from string
        let component_type = match component.to_lowercase().as_str() {
            "database" => ComponentType::Database,
            "ui" => ComponentType::UI,
            "ai" => ComponentType::AI,
            "system" => ComponentType::System,
            "editor" => ComponentType::Editor,
            "file_io" => ComponentType::FileIO,
            "network" => ComponentType::Network,
            _ => ComponentType::System, // Default
        };
        
        PerformanceMetricOps::record_metric(metric_name, metric_value, metric_unit, component_type, context_data).await
    }
    
    record(metric_name, metric_value, metric_unit, component, context_data).await
}

/// Get metrics by name
#[tauri::command]
pub async fn get_metrics_by_name(metric_name: String, limit: Option<i64>) -> Result<Vec<PerformanceMetric>> {
    async fn get(metric_name: String, limit: Option<i64>) -> Result<Vec<PerformanceMetric>> {
        PerformanceMetricOps::get_metrics_by_name(&metric_name, limit.unwrap_or(100)).await
    }
    
    get(metric_name, limit).await
}

/// Get metrics by component
#[tauri::command]
pub async fn get_metrics_by_component(component: String, limit: Option<i64>) -> Result<Vec<PerformanceMetric>> {
    async fn get(component: String, limit: Option<i64>) -> Result<Vec<PerformanceMetric>> {
        // Parse component type from string
        let component_type = match component.to_lowercase().as_str() {
            "database" => ComponentType::Database,
            "ui" => ComponentType::UI,
            "ai" => ComponentType::AI,
            "system" => ComponentType::System,
            "editor" => ComponentType::Editor,
            "file_io" => ComponentType::FileIO,
            "network" => ComponentType::Network,
            _ => ComponentType::System, // Default
        };
        
        PerformanceMetricOps::get_metrics_by_component(component_type, limit.unwrap_or(100)).await
    }
    
    get(component, limit).await
}

/// Get metrics within a time range
#[tauri::command]
pub async fn get_metrics_in_timerange(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    limit: Option<i64>,
) -> Result<Vec<PerformanceMetric>> {
    async fn get(start_time: DateTime<Utc>, end_time: DateTime<Utc>, limit: Option<i64>) -> Result<Vec<PerformanceMetric>> {
        PerformanceMetricOps::get_metrics_in_timerange(start_time, end_time, limit.unwrap_or(100)).await
    }
    
    get(start_time, end_time, limit).await
}

/// Get performance metrics summary
#[tauri::command]
pub async fn get_performance_summary() -> Result<PerformanceMetricsSummary> {
    async fn get() -> Result<PerformanceMetricsSummary> {
        PerformanceMetricOps::get_metrics_summary().await
    }
    
    get().await
}

/// Record a performance bottleneck
#[tauri::command]
pub async fn record_performance_bottleneck(
    component: String,
    operation: String,
    threshold_value: f64,
    actual_value: f64,
    severity: String,
) -> Result<PerformanceBottleneck> {
    async fn record(
        component: String,
        operation: String,
        threshold_value: f64,
        actual_value: f64,
        severity: String,
    ) -> Result<PerformanceBottleneck> {
        // Parse component type from string
        let component_type = match component.to_lowercase().as_str() {
            "database" => ComponentType::Database,
            "ui" => ComponentType::UI,
            "ai" => ComponentType::AI,
            "system" => ComponentType::System,
            "editor" => ComponentType::Editor,
            "file_io" => ComponentType::FileIO,
            "network" => ComponentType::Network,
            _ => ComponentType::System, // Default
        };
        
        // Parse severity from string
        let severity_level = match severity.to_lowercase().as_str() {
            "low" => BottleneckSeverity::Low,
            "medium" => BottleneckSeverity::Medium,
            "high" => BottleneckSeverity::High,
            "critical" => BottleneckSeverity::Critical,
            _ => BottleneckSeverity::Medium, // Default
        };
        
        PerformanceMetricOps::record_bottleneck(component_type, operation, threshold_value, actual_value, severity_level).await
    }
    
    record(component, operation, threshold_value, actual_value, severity).await
}

/// Resolve a performance bottleneck
#[tauri::command]
pub async fn resolve_bottleneck(id: String, resolution_notes: Option<String>) -> Result<()> {
    async fn resolve(id: String, resolution_notes: Option<String>) -> Result<()> {
        PerformanceMetricOps::resolve_bottleneck(&id, resolution_notes).await
    }
    
    resolve(id, resolution_notes).await
}

/// Record a memory snapshot
#[tauri::command]
pub async fn record_memory_snapshot(
    total_memory_mb: f64,
    used_memory_mb: f64,
    peak_memory_mb: f64,
    component_breakdown: HashMap<String, f64>,
) -> Result<MemorySnapshot> {
    async fn record(
        total_memory_mb: f64,
        used_memory_mb: f64,
        peak_memory_mb: f64,
        component_breakdown: HashMap<String, f64>,
    ) -> Result<MemorySnapshot> {
        // Convert HashMap to JSON
        let breakdown_json = serde_json::to_value(component_breakdown)
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to serialize component breakdown: {}", e)))?;
        
        PerformanceMetricOps::record_memory_snapshot(total_memory_mb, used_memory_mb, peak_memory_mb, breakdown_json).await
    }
    
    record(total_memory_mb, used_memory_mb, peak_memory_mb, component_breakdown).await
}

/// Record query performance
#[tauri::command]
pub async fn record_query_performance(
    query_hash: String,
    query_type: String,
    table_name: String,
    execution_time_ms: f64,
    row_count: Option<i32>,
    query_plan: Option<String>,
) -> Result<QueryPerformance> {
    async fn record(
        query_hash: String,
        query_type: String,
        table_name: String,
        execution_time_ms: f64,
        row_count: Option<i32>,
        query_plan: Option<String>,
    ) -> Result<QueryPerformance> {
        // Parse query type from string
        let query_type_enum = match query_type.to_lowercase().as_str() {
            "select" => QueryType::Select,
            "insert" => QueryType::Insert,
            "update" => QueryType::Update,
            "delete" => QueryType::Delete,
            _ => QueryType::Other, // Default
        };
        
        PerformanceMetricOps::record_query_performance(
            query_hash,
            query_type_enum,
            table_name,
            execution_time_ms,
            row_count,
            query_plan,
        ).await
    }
    
    record(query_hash, query_type, table_name, execution_time_ms, row_count, query_plan).await
}

/// Clean up old performance metrics
#[tauri::command]
pub async fn cleanup_old_metrics() -> Result<usize> {
    async fn cleanup() -> Result<usize> {
        PerformanceMetricOps::cleanup_old_metrics().await
    }
    
    cleanup().await
}

/// Performance timing wrapper for measuring function execution time
pub struct PerformanceTimer {
    start_time: Instant,
    metric_name: String,
    component: ComponentType,
}

impl PerformanceTimer {
    /// Create a new performance timer
    pub fn new(metric_name: &str, component: ComponentType) -> Self {
        Self {
            start_time: Instant::now(),
            metric_name: metric_name.to_string(),
            component,
        }
    }
    
    /// Stop the timer and record the metric
    pub async fn stop(self, context_data: Option<String>) -> Result<PerformanceMetric> {
        let elapsed_ms = self.start_time.elapsed().as_secs_f64() * 1000.0;
        PerformanceMetricOps::record_metric(
            self.metric_name,
            elapsed_ms,
            Some("ms".to_string()),
            self.component,
            context_data,
        ).await
    }
}
