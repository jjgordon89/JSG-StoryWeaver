use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

pub async fn up(pool: &Pool<Sqlite>) -> Result<()> {
    // Create optimization_config table for storing optimization settings
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS optimization_config (
            id INTEGER PRIMARY KEY,
            enable_auto_indexing BOOLEAN NOT NULL DEFAULT 1,
            memory_cache_size_mb INTEGER NOT NULL DEFAULT 256,
            ai_cache_ttl_hours INTEGER NOT NULL DEFAULT 24,
            cleanup_interval_hours INTEGER NOT NULL DEFAULT 168,
            last_optimization DATETIME,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization_config table: {}", e)))?;

    // Create index_usage_stats table for tracking index performance
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS index_usage_stats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            index_name TEXT NOT NULL,
            table_name TEXT NOT NULL,
            usage_count INTEGER NOT NULL DEFAULT 0,
            last_used DATETIME,
            effectiveness_score REAL DEFAULT 0.0,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(index_name, table_name)
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create index_usage_stats table: {}", e)))?;

    // Create ai_response_cache table for caching AI responses
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ai_response_cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cache_key TEXT NOT NULL UNIQUE,
            request_hash TEXT NOT NULL,
            response_data TEXT NOT NULL,
            similarity_tokens TEXT, -- JSON array of tokens for similarity matching
            hit_count INTEGER NOT NULL DEFAULT 0,
            last_accessed DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create ai_response_cache table: {}", e)))?;

    // Create query_analysis table for tracking slow queries and patterns
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS query_analysis (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            query_pattern TEXT NOT NULL,
            execution_time_ms REAL NOT NULL,
            table_names TEXT, -- JSON array of tables involved
            suggested_indexes TEXT, -- JSON array of suggested indexes
            frequency_count INTEGER NOT NULL DEFAULT 1,
            last_seen DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create query_analysis table: {}", e)))?;

    // Create optimization_reports table for storing optimization history
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS optimization_reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            performance_score REAL NOT NULL,
            recommendations TEXT, -- JSON array of recommendations
            actions_taken TEXT, -- JSON array of actions performed
            before_stats TEXT, -- JSON object of stats before optimization
            after_stats TEXT, -- JSON object of stats after optimization
            optimization_type TEXT NOT NULL, -- 'manual', 'automatic', 'scheduled'
            duration_ms INTEGER,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization_reports table: {}", e)))?;

    // Create essential indexes for optimization tables
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_index_usage_stats_table_name ON index_usage_stats(table_name)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_index_usage_stats_last_used ON index_usage_stats(last_used)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ai_response_cache_request_hash ON ai_response_cache(request_hash)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ai_response_cache_expires_at ON ai_response_cache(expires_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ai_response_cache_last_accessed ON ai_response_cache(last_accessed)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_query_analysis_pattern ON query_analysis(query_pattern)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_query_analysis_execution_time ON query_analysis(execution_time_ms)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_optimization_reports_created_at ON optimization_reports(created_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_optimization_reports_performance_score ON optimization_reports(performance_score)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    // Create performance indexes for existing tables to improve optimization queries
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_characters_updated_at ON characters(updated_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_locations_updated_at ON locations(updated_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ai_history_created_at ON ai_history(created_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ai_response_cards_created_at ON ai_response_cards(created_at)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    // Create composite indexes for common query patterns
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_documents_project_status ON documents(project_id, status)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_characters_project_series ON characters(project_id, series_id)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_ai_history_project_type ON ai_history(project_id, generation_type)")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;

    // Insert default optimization configuration
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO optimization_config (
            id, enable_auto_indexing, memory_cache_size_mb, 
            ai_cache_ttl_hours, cleanup_interval_hours
        ) VALUES (1, 1, 256, 24, 168)
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to insert default config: {}", e)))?;

    // Create triggers to update the updated_at timestamp
    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS update_optimization_config_timestamp 
        AFTER UPDATE ON optimization_config
        BEGIN
            UPDATE optimization_config SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
        END
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create trigger: {}", e)))?;

    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS update_index_usage_stats_timestamp 
        AFTER UPDATE ON index_usage_stats
        BEGIN
            UPDATE index_usage_stats SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
        END
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create trigger: {}", e)))?;

    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS update_ai_response_cache_timestamp 
        AFTER UPDATE ON ai_response_cache
        BEGIN
            UPDATE ai_response_cache SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
        END
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create trigger: {}", e)))?;

    sqlx::query(
        r#"
        CREATE TRIGGER IF NOT EXISTS update_query_analysis_timestamp 
        AFTER UPDATE ON query_analysis
        BEGIN
            UPDATE query_analysis SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
        END
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create trigger: {}", e)))?;

    // Create view for optimization dashboard statistics
    sqlx::query(
        r#"
        CREATE VIEW IF NOT EXISTS optimization_dashboard_stats AS
        SELECT 
            (SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%') as total_indexes,
            (SELECT COUNT(*) FROM index_usage_stats WHERE usage_count = 0) as unused_indexes,
            (SELECT AVG(execution_time_ms) FROM query_analysis WHERE last_seen > datetime('now', '-24 hours')) as avg_query_time_ms,
            (SELECT COUNT(*) FROM query_analysis WHERE execution_time_ms > 100) as slow_queries,
            (SELECT COUNT(*) FROM query_analysis) as total_queries,
            (SELECT CAST(SUM(hit_count) AS REAL) / CAST(SUM(hit_count + 1) AS REAL) FROM ai_response_cache) as cache_hit_rate,
            (SELECT COUNT(*) FROM ai_response_cache) as cache_entries,
            (SELECT performance_score FROM optimization_reports ORDER BY created_at DESC LIMIT 1) as last_performance_score,
            (SELECT created_at FROM optimization_reports ORDER BY created_at DESC LIMIT 1) as last_optimization_date
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create view: {}", e)))?;

    println!("Migration 015: Phase 6 optimization tables and indexes created successfully");
    Ok(())
}