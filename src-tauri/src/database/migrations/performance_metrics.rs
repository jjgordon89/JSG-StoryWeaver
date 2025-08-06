//! Performance metrics migration
//! Creates tables and indexes for performance monitoring

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

/// Create the performance metrics table
pub async fn create_performance_metrics_table(pool: &Pool<Sqlite>) -> Result<()> {
    // Create the performance_metrics table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS performance_metrics (
            id TEXT PRIMARY KEY,
            metric_name TEXT NOT NULL,
            metric_value REAL NOT NULL,
            metric_unit TEXT, -- 'ms', 'mb', 'count', 'percentage'
            context_data TEXT, -- JSON string with additional context
            component TEXT NOT NULL, -- 'database', 'ui', 'ai', 'system'
            recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create performance_metrics table: {}", e)))?;
    
    // Create the performance_bottlenecks table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS performance_bottlenecks (
            id TEXT PRIMARY KEY,
            component TEXT NOT NULL,
            operation TEXT NOT NULL,
            threshold_value REAL NOT NULL,
            actual_value REAL NOT NULL,
            severity TEXT NOT NULL, -- 'low', 'medium', 'high', 'critical'
            detected_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            resolved BOOLEAN DEFAULT 0,
            resolved_at DATETIME,
            resolution_notes TEXT
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create performance_bottlenecks table: {}", e)))?;
    
    // Create the memory_snapshots table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS memory_snapshots (
            id TEXT PRIMARY KEY,
            total_memory_mb REAL NOT NULL,
            used_memory_mb REAL NOT NULL,
            peak_memory_mb REAL NOT NULL,
            component_breakdown TEXT NOT NULL, -- JSON string with component memory usage
            recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create memory_snapshots table: {}", e)))?;
    
    // Create the query_performance table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS query_performance (
            id TEXT PRIMARY KEY,
            query_hash TEXT NOT NULL, -- Hash of the query for identification
            query_type TEXT NOT NULL, -- 'select', 'insert', 'update', 'delete'
            table_name TEXT NOT NULL,
            execution_time_ms REAL NOT NULL,
            row_count INTEGER,
            is_slow BOOLEAN DEFAULT 0,
            query_plan TEXT, -- Execution plan if available
            recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create query_performance table: {}", e)))?;
    
    // Create indexes for performance tables
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_perf_metrics_name ON performance_metrics(metric_name)",
        "CREATE INDEX IF NOT EXISTS idx_perf_metrics_component ON performance_metrics(component)",
        "CREATE INDEX IF NOT EXISTS idx_perf_metrics_recorded_at ON performance_metrics(recorded_at)",
        "CREATE INDEX IF NOT EXISTS idx_perf_bottlenecks_component ON performance_bottlenecks(component)",
        "CREATE INDEX IF NOT EXISTS idx_perf_bottlenecks_severity ON performance_bottlenecks(severity)",
        "CREATE INDEX IF NOT EXISTS idx_perf_bottlenecks_resolved ON performance_bottlenecks(resolved)",
        "CREATE INDEX IF NOT EXISTS idx_memory_snapshots_recorded_at ON memory_snapshots(recorded_at)",
        "CREATE INDEX IF NOT EXISTS idx_query_perf_query_hash ON query_performance(query_hash)",
        "CREATE INDEX IF NOT EXISTS idx_query_perf_table_name ON query_performance(table_name)",
        "CREATE INDEX IF NOT EXISTS idx_query_perf_is_slow ON query_performance(is_slow)",
    ];
    
    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    }
    
    // Add default settings for performance monitoring
    let default_settings = [
        ("perf_metrics_retention_days", "30"),
        ("slow_query_threshold_ms", "100"),
        ("memory_snapshot_interval_minutes", "60"),
        ("performance_monitoring_enabled", "true"),
        ("bottleneck_detection_enabled", "true"),
        ("memory_monitoring_enabled", "true"),
        ("query_performance_tracking_enabled", "true"),
    ];
    
    for (key, value) in default_settings {
        sqlx::query(
            "INSERT OR IGNORE INTO settings (key, value) VALUES (?, ?)"
        )
        .bind(key)
        .bind(value)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to insert performance setting: {}", e)))?;
    }
    
    Ok(())
}
