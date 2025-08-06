//! Performance metrics database operations
//! Provides functions for working with performance monitoring data

use crate::database::{get_pool, models::*};
use crate::error::{Result, StoryWeaverError};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::Row;
use uuid::Uuid;

/// Performance metrics operations
pub struct PerformanceMetricOps;

impl PerformanceMetricOps {
    /// Record a new performance metric
    pub async fn record_metric(
        metric_name: String,
        metric_value: f64,
        metric_unit: Option<String>,
        component: ComponentType,
        context_data: Option<String>,
    ) -> Result<PerformanceMetric> {
        let pool = get_pool()?;
        let metric = PerformanceMetric::new(metric_name, metric_value, metric_unit, component, context_data);
        
        sqlx::query(
            r#"
            INSERT INTO performance_metrics (
                id, metric_name, metric_value, metric_unit, context_data, component, recorded_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&metric.id)
        .bind(&metric.metric_name)
        .bind(metric.metric_value)
        .bind(&metric.metric_unit)
        .bind(&metric.context_data)
        .bind(&metric.component.to_string())
        .bind(metric.recorded_at)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record performance metric: {}", e)))?;
        
        Ok(metric)
    }
    
    /// Get metrics by name
    pub async fn get_metrics_by_name(metric_name: &str, limit: i64) -> Result<Vec<PerformanceMetric>> {
        let pool = get_pool()?;
        
        let metrics = sqlx::query_as::<_, PerformanceMetric>(
            r#"
            SELECT * FROM performance_metrics
            WHERE metric_name = ?
            ORDER BY recorded_at DESC
            LIMIT ?
            "#,
        )
        .bind(metric_name)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get metrics by name: {}", e)))?;
        
        Ok(metrics)
    }
    
    /// Get metrics by component
    pub async fn get_metrics_by_component(component: ComponentType, limit: i64) -> Result<Vec<PerformanceMetric>> {
        let pool = get_pool()?;
        
        let metrics = sqlx::query_as::<_, PerformanceMetric>(
            r#"
            SELECT * FROM performance_metrics
            WHERE component = ?
            ORDER BY recorded_at DESC
            LIMIT ?
            "#,
        )
        .bind(component.to_string())
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get metrics by component: {}", e)))?;
        
        Ok(metrics)
    }
    
    /// Get metrics within a time range
    pub async fn get_metrics_in_timerange(
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<PerformanceMetric>> {
        let pool = get_pool()?;
        
        let metrics = sqlx::query_as::<_, PerformanceMetric>(
            r#"
            SELECT * FROM performance_metrics
            WHERE recorded_at BETWEEN ? AND ?
            ORDER BY recorded_at DESC
            LIMIT ?
            "#,
        )
        .bind(start_time)
        .bind(end_time)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get metrics in timerange: {}", e)))?;
        
        Ok(metrics)
    }
    
    /// Get metrics summary
    pub async fn get_metrics_summary() -> Result<PerformanceMetricsSummary> {
        let pool = get_pool()?;
        
        // Get total metrics count
        let total_metrics_count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM performance_metrics")
            .fetch_one(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count metrics: {}", e)))?;
        
        // Get metrics by component
        let component_counts = sqlx::query(
            r#"
            SELECT component, COUNT(*) as count
            FROM performance_metrics
            GROUP BY component
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get metrics by component: {}", e)))?;
        
        let metrics_by_component = component_counts
            .iter()
            .map(|row| {
                let component_str: String = row.get("component");
                let component = match component_str.as_str() {
                    "database" => ComponentType::Database,
                    "ui" => ComponentType::UI,
                    "ai" => ComponentType::AI,
                    "system" => ComponentType::System,
                    "editor" => ComponentType::Editor,
                    "file_io" => ComponentType::FileIO,
                    "network" => ComponentType::Network,
                    _ => ComponentType::System, // Default
                };
                
                ComponentMetricCount {
                    component,
                    count: row.get("count"),
                }
            })
            .collect();
        
        // Get bottleneck counts
        let active_bottlenecks: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM performance_bottlenecks WHERE resolved = 0")
            .fetch_one(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count active bottlenecks: {}", e)))?;
        
        let resolved_bottlenecks: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM performance_bottlenecks WHERE resolved = 1")
            .fetch_one(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to count resolved bottlenecks: {}", e)))?;
        
        // Get query performance stats
        let query_stats = sqlx::query(
            r#"
            SELECT AVG(execution_time_ms) as avg_time,
                   (SUM(CASE WHEN is_slow = 1 THEN 1 ELSE 0 END) * 100.0 / COUNT(*)) as slow_percentage
            FROM query_performance
            "#,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get query stats: {}", e)))?;
        
        let (average_query_time_ms, slow_query_percentage) = if let Some(row) = query_stats {
            (
                row.get::<Option<f64>, _>("avg_time").unwrap_or(0.0),
                row.get::<Option<f64>, _>("slow_percentage").unwrap_or(0.0),
            )
        } else {
            (0.0, 0.0)
        };
        
        // Get memory usage trend
        let memory_snapshots = sqlx::query(
            r#"
            SELECT recorded_at, used_memory_mb
            FROM memory_snapshots
            ORDER BY recorded_at DESC
            LIMIT 10
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get memory snapshots: {}", e)))?;
        
        let memory_usage_trend = memory_snapshots
            .iter()
            .map(|row| MemoryTrendPoint {
                timestamp: row.get("recorded_at"),
                used_memory_mb: row.get("used_memory_mb"),
            })
            .collect();
        
        Ok(PerformanceMetricsSummary {
            total_metrics_count,
            metrics_by_component,
            active_bottlenecks,
            resolved_bottlenecks,
            average_query_time_ms,
            slow_query_percentage,
            memory_usage_trend,
        })
    }
    
    /// Record a performance bottleneck
    pub async fn record_bottleneck(
        component: ComponentType,
        operation: String,
        threshold_value: f64,
        actual_value: f64,
        severity: BottleneckSeverity,
    ) -> Result<PerformanceBottleneck> {
        let pool = get_pool()?;
        let bottleneck = PerformanceBottleneck::new(component, operation, threshold_value, actual_value, severity);
        
        sqlx::query(
            r#"
            INSERT INTO performance_bottlenecks (
                id, component, operation, threshold_value, actual_value, severity, detected_at, resolved
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&bottleneck.id)
        .bind(&bottleneck.component.to_string())
        .bind(&bottleneck.operation)
        .bind(bottleneck.threshold_value)
        .bind(bottleneck.actual_value)
        .bind(&bottleneck.severity.to_string())
        .bind(bottleneck.detected_at)
        .bind(bottleneck.resolved)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record performance bottleneck: {}", e)))?;
        
        Ok(bottleneck)
    }
    
    /// Resolve a performance bottleneck
    pub async fn resolve_bottleneck(id: &str, resolution_notes: Option<String>) -> Result<()> {
        let pool = get_pool()?;
        let now = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE performance_bottlenecks
            SET resolved = 1, resolved_at = ?, resolution_notes = ?
            WHERE id = ?
            "#,
        )
        .bind(now)
        .bind(resolution_notes)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to resolve bottleneck: {}", e)))?;
        
        Ok(())
    }
    
    /// Record a memory snapshot
    pub async fn record_memory_snapshot(
        total_memory_mb: f64,
        used_memory_mb: f64,
        peak_memory_mb: f64,
        component_breakdown: serde_json::Value,
    ) -> Result<MemorySnapshot> {
        let pool = get_pool()?;
        let component_breakdown_str = component_breakdown.to_string();
        let snapshot = MemorySnapshot::new(total_memory_mb, used_memory_mb, peak_memory_mb, component_breakdown_str);
        
        sqlx::query(
            r#"
            INSERT INTO memory_snapshots (
                id, total_memory_mb, used_memory_mb, peak_memory_mb, component_breakdown, recorded_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&snapshot.id)
        .bind(snapshot.total_memory_mb)
        .bind(snapshot.used_memory_mb)
        .bind(snapshot.peak_memory_mb)
        .bind(&snapshot.component_breakdown)
        .bind(snapshot.recorded_at)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record memory snapshot: {}", e)))?;
        
        Ok(snapshot)
    }
    
    /// Record query performance
    pub async fn record_query_performance(
        query_hash: String,
        query_type: QueryType,
        table_name: String,
        execution_time_ms: f64,
        row_count: Option<i32>,
        query_plan: Option<String>,
    ) -> Result<QueryPerformance> {
        let pool = get_pool()?;
        
        // Check if the query is slow based on threshold
        let threshold_ms: f64 = Self::get_slow_query_threshold().await?;
        let is_slow = execution_time_ms > threshold_ms;
        
        let query_perf = QueryPerformance::new(
            query_hash,
            query_type,
            table_name,
            execution_time_ms,
            row_count,
            is_slow,
            query_plan,
        );
        
        sqlx::query(
            r#"
            INSERT INTO query_performance (
                id, query_hash, query_type, table_name, execution_time_ms, row_count, is_slow, query_plan, recorded_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&query_perf.id)
        .bind(&query_perf.query_hash)
        .bind(&query_perf.query_type.to_string())
        .bind(&query_perf.table_name)
        .bind(query_perf.execution_time_ms)
        .bind(query_perf.row_count)
        .bind(query_perf.is_slow)
        .bind(&query_perf.query_plan)
        .bind(query_perf.recorded_at)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record query performance: {}", e)))?;
        
        Ok(query_perf)
    }
    
    /// Get slow query threshold from settings
    async fn get_slow_query_threshold() -> Result<f64> {
        let pool = get_pool()?;
        
        let threshold_str: Option<String> = sqlx::query_scalar("SELECT value FROM settings WHERE key = 'slow_query_threshold_ms'")
            .fetch_optional(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get slow query threshold: {}", e)))?;
        
        match threshold_str {
            Some(val) => val.parse::<f64>().map_err(|e| {
                StoryWeaverError::database(format!("Failed to parse slow query threshold: {}", e))
            }),
            None => Ok(100.0), // Default threshold: 100ms
        }
    }
    
    /// Clean up old performance metrics
    pub async fn cleanup_old_metrics() -> Result<usize> {
        let pool = get_pool()?;
        
        // Get retention period from settings
        let retention_days: i64 = match sqlx::query_scalar::<_, Option<String>>("SELECT value FROM settings WHERE key = 'perf_metrics_retention_days'")
            .fetch_optional(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get retention period: {}", e)))? {
                Some(val) => val.parse().unwrap_or(30),
                None => 30, // Default: 30 days
            };
        
        // Delete old metrics
        let result = sqlx::query(
            r#"
            DELETE FROM performance_metrics
            WHERE recorded_at < datetime('now', ? || ' days')
            "#,
        )
        .bind(format!("-{}", retention_days))
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to clean up old metrics: {}", e)))?;
        
        // Delete old query performance records
        sqlx::query(
            r#"
            DELETE FROM query_performance
            WHERE recorded_at < datetime('now', ? || ' days')
            "#,
        )
        .bind(format!("-{}", retention_days))
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to clean up old query performance records: {}", e)))?;
        
        // Delete old memory snapshots (keep fewer of these)
        sqlx::query(
            r#"
            DELETE FROM memory_snapshots
            WHERE recorded_at < datetime('now', ? || ' days')
            "#,
        )
        .bind(format!("-{}", retention_days / 2)) // Keep for half the time
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to clean up old memory snapshots: {}", e)))?;
        
        Ok(result.rows_affected() as usize)
    }
}
