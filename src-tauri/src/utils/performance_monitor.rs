//! Performance monitoring utilities
//! Provides tools for tracking application performance

use crate::database::models::*;
use crate::database::operations::PerformanceMetricOps;
use crate::error::Result;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use sqlx::QueryBuilder;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use serde_json::json;
use once_cell::sync::Lazy;

// Global flag to enable/disable performance monitoring
static MONITORING_ENABLED: AtomicBool = AtomicBool::new(true);

/// Set the global monitoring enabled flag
pub fn set_monitoring_enabled(enabled: bool) {
    MONITORING_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Check if monitoring is enabled
pub fn is_monitoring_enabled() -> bool {
    MONITORING_ENABLED.load(Ordering::SeqCst)
}

/// Performance timer for measuring function execution time
pub struct PerformanceTimer {
    start_time: Instant,
    metric_name: String,
    component: ComponentType,
    context_data: Option<String>,
}

impl PerformanceTimer {
    /// Create a new performance timer
    pub fn new(metric_name: &str, component: ComponentType) -> Self {
        Self {
            start_time: Instant::now(),
            metric_name: metric_name.to_string(),
            component,
            context_data: None,
        }
    }
    
    /// Create a new performance timer with context data
    pub fn with_context(metric_name: &str, component: ComponentType, context_data: serde_json::Value) -> Self {
        Self {
            start_time: Instant::now(),
            metric_name: metric_name.to_string(),
            component,
            context_data: Some(context_data.to_string()),
        }
    }
    
    /// Stop the timer and record the metric
    pub async fn stop(self) -> Result<PerformanceMetric> {
        if !is_monitoring_enabled() {
            // Return a dummy metric if monitoring is disabled
            return Ok(PerformanceMetric::new(
                self.metric_name,
                0.0,
                Some("ms".to_string()),
                self.component,
                self.context_data,
            ));
        }
        
        let elapsed_ms = self.start_time.elapsed().as_secs_f64() * 1000.0;
        PerformanceMetricOps::record_metric(
            self.metric_name,
            elapsed_ms,
            Some("ms".to_string()),
            self.component,
            self.context_data,
        ).await
    }
    
    /// Get the elapsed time without stopping the timer
    pub fn elapsed_ms(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64() * 1000.0
    }
}

/// Database query performance tracker
pub struct QueryPerformanceTracker {
    query_type: QueryType,
    table_name: String,
    start_time: Instant,
    query_hash: String,
}

impl QueryPerformanceTracker {
    /// Create a new query performance tracker
    pub fn new<Q: AsRef<str>>(query: Q, query_type: QueryType, table_name: &str) -> Self {
        // Create a hash of the query for identification
        let mut hasher = DefaultHasher::new();
        query.as_ref().hash(&mut hasher);
        let query_hash = format!("{:x}", hasher.finish());
        
        Self {
            query_type,
            table_name: table_name.to_string(),
            start_time: Instant::now(),
            query_hash,
        }
    }
    
    /// Stop tracking and record the query performance
    pub async fn stop(self, row_count: Option<i32>, query_plan: Option<String>) -> Result<QueryPerformance> {
        if !is_monitoring_enabled() {
            // Return a dummy metric if monitoring is disabled
            return Ok(QueryPerformance::new(
                self.query_hash,
                self.query_type,
                self.table_name,
                0.0,
                row_count,
                false,
                query_plan,
            ));
        }
        
        let execution_time_ms = self.start_time.elapsed().as_secs_f64() * 1000.0;
        
        PerformanceMetricOps::record_query_performance(
            self.query_hash,
            self.query_type,
            self.table_name,
            execution_time_ms,
            row_count,
            query_plan,
        ).await
    }
}

/// Memory usage tracker
pub struct MemoryTracker {
    component_breakdown: HashMap<String, f64>,
    last_snapshot_time: Instant,
    snapshot_interval: Duration,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new(snapshot_interval_secs: u64) -> Self {
        Self {
            component_breakdown: HashMap::new(),
            last_snapshot_time: Instant::now(),
            snapshot_interval: Duration::from_secs(snapshot_interval_secs),
        }
    }
    
    /// Register memory usage for a component
    pub fn register_component_usage(&mut self, component: &str, memory_mb: f64) {
        self.component_breakdown.insert(component.to_string(), memory_mb);
    }
    
    /// Take a memory snapshot if the interval has elapsed
    pub async fn maybe_take_snapshot(&mut self, total_memory_mb: f64, used_memory_mb: f64, peak_memory_mb: f64) -> Result<Option<MemorySnapshot>> {
        if !is_monitoring_enabled() {
            return Ok(None);
        }
        
        if self.last_snapshot_time.elapsed() >= self.snapshot_interval {
            self.last_snapshot_time = Instant::now();
            
            let component_breakdown = serde_json::to_value(&self.component_breakdown)
                .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to serialize component breakdown: {}", e)))?;
            
            let snapshot = PerformanceMetricOps::record_memory_snapshot(
                total_memory_mb,
                used_memory_mb,
                peak_memory_mb,
                component_breakdown,
            ).await?;
            
            Ok(Some(snapshot))
        } else {
            Ok(None)
        }
    }
}

/// Bottleneck detector
pub struct BottleneckDetector {
    thresholds: HashMap<String, (f64, BottleneckSeverity)>,
}

impl BottleneckDetector {
    /// Create a new bottleneck detector
    pub fn new() -> Self {
        let mut detector = Self {
            thresholds: HashMap::new(),
        };
        
        // Add default thresholds
        detector.add_threshold("ui.render_time", 100.0, BottleneckSeverity::Low);
        detector.add_threshold("ui.render_time", 300.0, BottleneckSeverity::Medium);
        detector.add_threshold("ui.render_time", 1000.0, BottleneckSeverity::High);
        
        detector.add_threshold("database.query_time", 100.0, BottleneckSeverity::Low);
        detector.add_threshold("database.query_time", 500.0, BottleneckSeverity::Medium);
        detector.add_threshold("database.query_time", 2000.0, BottleneckSeverity::High);
        
        detector.add_threshold("ai.response_time", 2000.0, BottleneckSeverity::Low);
        detector.add_threshold("ai.response_time", 5000.0, BottleneckSeverity::Medium);
        detector.add_threshold("ai.response_time", 10000.0, BottleneckSeverity::High);
        
        detector
    }
    
    /// Add a threshold for bottleneck detection
    pub fn add_threshold(&mut self, operation: &str, threshold_value: f64, severity: BottleneckSeverity) -> &mut Self {
        self.thresholds.insert(
            operation.to_string(),
            (threshold_value, severity),
        );
        self
    }
    
    /// Check if a metric exceeds any threshold and record a bottleneck if it does
    pub async fn check_metric(&self, component: ComponentType, operation: &str, value: f64) -> Result<Option<PerformanceBottleneck>> {
        if !is_monitoring_enabled() {
            return Ok(None);
        }
        
        // Find the highest severity threshold that is exceeded
        let mut highest_severity: Option<(f64, BottleneckSeverity)> = None;
        
        for (op, (threshold, severity)) in &self.thresholds {
            if op == operation && value > *threshold {
                match &highest_severity {
                    Some((_, current_severity)) => {
                        // Compare severity levels
                        let current_level = match current_severity {
                            BottleneckSeverity::Low => 1,
                            BottleneckSeverity::Medium => 2,
                            BottleneckSeverity::High => 3,
                            BottleneckSeverity::Critical => 4,
                        };
                        
                        let new_level = match severity {
                            BottleneckSeverity::Low => 1,
                            BottleneckSeverity::Medium => 2,
                            BottleneckSeverity::High => 3,
                            BottleneckSeverity::Critical => 4,
                        };
                        
                        if new_level > current_level {
                            highest_severity = Some((*threshold, severity.clone()));
                        }
                    },
                    None => {
                        highest_severity = Some((*threshold, severity.clone()));
                    }
                }
            }
        }
        
        // If a threshold was exceeded, record a bottleneck
        if let Some((threshold, severity)) = highest_severity {
            let bottleneck = PerformanceMetricOps::record_bottleneck(
                component,
                operation.to_string(),
                threshold,
                value,
                severity,
            ).await?;
            
            Ok(Some(bottleneck))
        } else {
            Ok(None)
        }
    }
}

/// Global performance monitoring context
pub struct PerformanceContext {
    pub bottleneck_detector: BottleneckDetector,
    pub memory_tracker: MemoryTracker,
}

impl PerformanceContext {
    /// Create a new performance monitoring context
    pub fn new() -> Self {
        Self {
            bottleneck_detector: BottleneckDetector::new(),
            memory_tracker: MemoryTracker::new(60), // Take memory snapshots every 60 seconds
        }
    }
    
    /// Initialize the performance monitoring context
    pub async fn initialize(&self) -> Result<()> {
        // Load monitoring settings from database
        let pool = crate::database::get_pool()?;
        
        let monitoring_enabled: Option<String> = sqlx::query_scalar("SELECT value FROM settings WHERE key = 'performance_monitoring_enabled'")
            .fetch_optional(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get monitoring setting: {}", e)))?;
        
        if let Some(value) = monitoring_enabled {
            set_monitoring_enabled(value.to_lowercase() == "true");
        }
        
        Ok(())
    }
}

// Create a global performance context
static PERFORMANCE_CONTEXT: Lazy<Arc<Mutex<PerformanceContext>>> = Lazy::new(|| {
    Arc::new(Mutex::new(PerformanceContext::new()))
});

/// Initialize the performance monitoring system
pub async fn initialize_performance_monitoring() -> Result<()> {
    let context = PERFORMANCE_CONTEXT.lock().await;
    context.initialize().await
}

/// Measure the execution time of an async function
pub async fn measure_async<F, T, E>(name: &str, component: ComponentType, f: F) -> std::result::Result<T, E>
where
    F: std::future::Future<Output = std::result::Result<T, E>>,
{
    let timer = PerformanceTimer::new(name, component);
    let result = f.await;
    
    if result.is_ok() {
        // Only record metrics for successful operations
        let _ = timer.stop().await;
    }
    
    result
}

/// Measure the execution time of a database query
pub async fn measure_query<T, E>(
    query: &str,
    query_type: QueryType,
    table_name: &str,
    row_count: Option<i32>,
    f: impl std::future::Future<Output = std::result::Result<T, E>>,
) -> std::result::Result<T, E> {
    let tracker = QueryPerformanceTracker::new(query, query_type, table_name);
    let result = f.await;
    
    if result.is_ok() {
        // Only record metrics for successful queries
        let _ = tracker.stop(row_count, None).await;
    }
    
    result
}
