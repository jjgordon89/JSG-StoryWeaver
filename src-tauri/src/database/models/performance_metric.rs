//! Performance metrics models
//! Defines data structures for performance monitoring

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use std::fmt;

/// Performance metric model - represents a single performance measurement
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PerformanceMetric {
    pub id: String,
    pub metric_name: String,
    pub metric_value: f64,
    pub metric_unit: Option<String>,
    pub context_data: Option<String>, // JSON string with additional context
    pub component: ComponentType,
    pub recorded_at: DateTime<Utc>,
}

/// Performance bottleneck model - represents a detected performance issue
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PerformanceBottleneck {
    pub id: String,
    pub component: ComponentType,
    pub operation: String,
    pub threshold_value: f64,
    pub actual_value: f64,
    pub severity: BottleneckSeverity,
    pub detected_at: DateTime<Utc>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
}

/// Memory snapshot model - represents a point-in-time memory usage measurement
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MemorySnapshot {
    pub id: String,
    pub total_memory_mb: f64,
    pub used_memory_mb: f64,
    pub peak_memory_mb: f64,
    pub component_breakdown: String, // JSON string with component memory usage
    pub recorded_at: DateTime<Utc>,
}

/// Query performance model - represents a database query execution measurement
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QueryPerformance {
    pub id: String,
    pub query_hash: String,
    pub query_type: QueryType,
    pub table_name: String,
    pub execution_time_ms: f64,
    pub row_count: Option<i32>,
    pub is_slow: bool,
    pub query_plan: Option<String>,
    pub recorded_at: DateTime<Utc>,
}

/// Component type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ComponentType {
    #[sqlx(rename = "database")]
    Database,
    #[sqlx(rename = "ui")]
    UI,
    #[sqlx(rename = "ai")]
    AI,
    #[sqlx(rename = "system")]
    System,
    #[sqlx(rename = "editor")]
    Editor,
    #[sqlx(rename = "file_io")]
    FileIO,
    #[sqlx(rename = "network")]
    Network,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentType::Database => write!(f, "database"),
            ComponentType::UI => write!(f, "ui"),
            ComponentType::AI => write!(f, "ai"),
            ComponentType::System => write!(f, "system"),
            ComponentType::Editor => write!(f, "editor"),
            ComponentType::FileIO => write!(f, "file_io"),
            ComponentType::Network => write!(f, "network"),
        }
    }
}

/// Bottleneck severity enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum BottleneckSeverity {
    #[sqlx(rename = "low")]
    Low,
    #[sqlx(rename = "medium")]
    Medium,
    #[sqlx(rename = "high")]
    High,
    #[sqlx(rename = "critical")]
    Critical,
}

impl fmt::Display for BottleneckSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BottleneckSeverity::Low => write!(f, "low"),
            BottleneckSeverity::Medium => write!(f, "medium"),
            BottleneckSeverity::High => write!(f, "high"),
            BottleneckSeverity::Critical => write!(f, "critical"),
        }
    }
}

/// Query type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum QueryType {
    #[sqlx(rename = "select")]
    Select,
    #[sqlx(rename = "insert")]
    Insert,
    #[sqlx(rename = "update")]
    Update,
    #[sqlx(rename = "delete")]
    Delete,
    #[sqlx(rename = "other")]
    Other,
}

impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryType::Select => write!(f, "select"),
            QueryType::Insert => write!(f, "insert"),
            QueryType::Update => write!(f, "update"),
            QueryType::Delete => write!(f, "delete"),
            QueryType::Other => write!(f, "other"),
        }
    }
}

/// Performance metrics summary - aggregated metrics for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsSummary {
    pub total_metrics_count: i32,
    pub metrics_by_component: Vec<ComponentMetricCount>,
    pub active_bottlenecks: i32,
    pub resolved_bottlenecks: i32,
    pub average_query_time_ms: f64,
    pub slow_query_percentage: f64,
    pub memory_usage_trend: Vec<MemoryTrendPoint>,
}

/// Component metric count - for summary reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentMetricCount {
    pub component: ComponentType,
    pub count: i32,
}

/// Memory trend point - for memory usage trend reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTrendPoint {
    pub timestamp: DateTime<Utc>,
    pub used_memory_mb: f64,
}

// Helper functions for model creation
impl PerformanceMetric {
    pub fn new(
        metric_name: String,
        metric_value: f64,
        metric_unit: Option<String>,
        component: ComponentType,
        context_data: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            metric_name,
            metric_value,
            metric_unit,
            context_data,
            component,
            recorded_at: Utc::now(),
        }
    }
}

impl PerformanceBottleneck {
    pub fn new(
        component: ComponentType,
        operation: String,
        threshold_value: f64,
        actual_value: f64,
        severity: BottleneckSeverity,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            component,
            operation,
            threshold_value,
            actual_value,
            severity,
            detected_at: Utc::now(),
            resolved: false,
            resolved_at: None,
            resolution_notes: None,
        }
    }
}

impl MemorySnapshot {
    pub fn new(
        total_memory_mb: f64,
        used_memory_mb: f64,
        peak_memory_mb: f64,
        component_breakdown: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            total_memory_mb,
            used_memory_mb,
            peak_memory_mb,
            component_breakdown,
            recorded_at: Utc::now(),
        }
    }
}

impl QueryPerformance {
    pub fn new(
        query_hash: String,
        query_type: QueryType,
        table_name: String,
        execution_time_ms: f64,
        row_count: Option<i32>,
        is_slow: bool,
        query_plan: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            query_hash,
            query_type,
            table_name,
            execution_time_ms,
            row_count,
            is_slow,
            query_plan,
            recorded_at: Utc::now(),
        }
    }
}
