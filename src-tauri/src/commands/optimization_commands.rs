use crate::database::optimization::{OptimizationManager, DatabaseOptimizationStats};
use crate::database::DbPool;
use crate::error::StoryWeaverError;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::security::validation::{validate_security_input, validate_content_length};
use crate::security::rate_limit::{rl_create, rl_update, rl_delete, rl_list};

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub enable_auto_indexing: bool,
    pub memory_cache_size_mb: usize,
    pub ai_cache_ttl_hours: u64,
    pub cleanup_interval_hours: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizationReport {
    pub database_stats: DatabaseOptimizationStats,
    pub recommendations: Vec<String>,
    pub performance_score: f64,
    pub last_optimization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexRecommendation {
    pub table_name: String,
    pub columns: Vec<String>,
    pub index_type: String,
    pub estimated_benefit: f64,
    pub reason: String,
}

#[tauri::command]
pub async fn get_optimization_stats(
    pool: State<'_, DbPool>,
) -> Result<DatabaseOptimizationStats, StoryWeaverError> {
    rl_list("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    optimization_manager
        .get_optimization_stats()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get optimization stats: {}", e)))
}

#[tauri::command]
pub async fn run_database_optimization(
    pool: State<'_, DbPool>,
    config: OptimizationConfig,
) -> Result<OptimizationReport, StoryWeaverError> {
    rl_update("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    // Run optimization based on config
    if config.enable_auto_indexing {
        optimization_manager
            .create_recommended_indexes()
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create indexes: {}", e)))?;
    }
    
    // Perform maintenance
    optimization_manager
        .perform_maintenance()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to perform maintenance: {}", e)))?;
    
    // Get updated stats
    let stats = optimization_manager
        .get_optimization_stats()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get stats after optimization: {}", e)))?;
    
    // Generate recommendations
    let recommendations = generate_recommendations(&stats);
    let performance_score = calculate_performance_score(&stats);
    
    Ok(OptimizationReport {
        database_stats: stats,
        recommendations,
        performance_score,
        last_optimization: Some(chrono::Utc::now().to_rfc3339()),
    })
}

#[tauri::command]
pub async fn get_index_recommendations(
    pool: State<'_, DbPool>,
) -> Result<Vec<IndexRecommendation>, StoryWeaverError> {
    rl_list("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let recommendations = optimization_manager
        .index_manager
        .analyze_query_patterns()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to analyze query patterns: {}", e)))?;
    
    Ok(recommendations
        .into_iter()
        .map(|rec| IndexRecommendation {
            table_name: rec.table_name,
            columns: rec.column_names,
            index_type: format!("{:?}", rec.index_type),
            estimated_benefit: rec.estimated_benefit,
            reason: format!("Priority: {:?}", rec.priority),
        })
        .collect())
}

#[tauri::command]
pub async fn create_index(
    pool: State<'_, DbPool>,
    table_name: String,
    columns: Vec<String>,
    index_type: Option<String>,
) -> Result<String, StoryWeaverError> {
    rl_create("optimization", None)?;
    
    // Input validation
    if table_name.trim().is_empty() {
        return Err(StoryWeaverError::validation("table_name cannot be empty"));
    }
    validate_content_length(&table_name, 128)?;
    validate_security_input(&table_name)?;
    if columns.is_empty() {
        return Err(StoryWeaverError::validation("columns must contain at least one column"));
    }
    for col in &columns {
        if col.trim().is_empty() {
            return Err(StoryWeaverError::validation("column names cannot be empty"));
        }
        validate_content_length(col, 128)?;
        validate_security_input(col)?;
    }
    if let Some(ref idx_ty) = index_type {
        validate_content_length(idx_ty, 64)?;
        validate_security_input(idx_ty)?;
    }

    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let index_name = optimization_manager
        .create_custom_index(&table_name, &columns, index_type.as_deref())
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    
    Ok(format!("Created index: {}", index_name))
}

#[tauri::command]
pub async fn drop_unused_indexes(
    pool: State<'_, DbPool>,
    min_usage_threshold: Option<f64>,
) -> Result<Vec<String>, StoryWeaverError> {
    rl_delete("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let threshold = min_usage_threshold.unwrap_or(0.1); // 10% default threshold
    if !(0.0..=1.0).contains(&threshold) {
        return Err(StoryWeaverError::validation("min_usage_threshold must be between 0.0 and 1.0 inclusive"));
    }
    
    let dropped_indexes = optimization_manager
        .cleanup_unused_indexes(threshold)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to cleanup indexes: {}", e)))?;
    
    Ok(dropped_indexes)
}

#[tauri::command]
pub async fn clear_ai_cache(
    pool: State<'_, DbPool>,
    older_than_hours: Option<u64>,
) -> Result<usize, StoryWeaverError> {
    rl_delete("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let hours = older_than_hours.unwrap_or(24); // Default to 24 hours
    
    let cleared_count = optimization_manager
        .clear_ai_cache(hours)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to clear AI cache: {}", e)))?;
    
    Ok(cleared_count)
}

#[tauri::command]
pub async fn optimize_memory_usage(
    pool: State<'_, DbPool>,
    target_mb: Option<usize>,
) -> Result<String, StoryWeaverError> {
    rl_update("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let target = target_mb.unwrap_or(256); // Default to 256MB
    if target < 16 || target > 65536 {
        return Err(StoryWeaverError::validation("target_mb must be between 16 and 65536"));
    }
    
    optimization_manager
        .optimize_memory_usage(target)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to optimize memory usage: {}", e)))?;
    
    Ok(format!("Memory usage optimized to target: {}MB", target))
}

#[tauri::command]
pub async fn get_cache_statistics(
    pool: State<'_, DbPool>,
) -> Result<serde_json::Value, StoryWeaverError> {
    rl_list("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let stats = optimization_manager
        .get_cache_statistics()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get cache statistics: {}", e)))?;
    
    Ok(serde_json::to_value(stats)
        .map_err(|e| StoryWeaverError::serialization(format!("Failed to serialize cache statistics: {}", e)))?)
}

#[tauri::command]
pub async fn run_performance_analysis(
    pool: State<'_, DbPool>,
) -> Result<serde_json::Value, StoryWeaverError> {
    rl_list("optimization", None)?;
    
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let analysis = optimization_manager
        .run_performance_analysis()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to run performance analysis: {}", e)))?;
    
    Ok(serde_json::to_value(analysis)
        .map_err(|e| StoryWeaverError::serialization(format!("Failed to serialize performance analysis: {}", e)))?)
}

#[tauri::command]
pub async fn schedule_maintenance(
    pool: State<'_, DbPool>,
    maintenance_type: String,
    schedule_cron: String,
) -> Result<String, StoryWeaverError> {
    rl_create("optimization", None)?;
    
    // Input validation
    if maintenance_type.trim().is_empty() {
        return Err(StoryWeaverError::validation("maintenance_type cannot be empty"));
    }
    validate_content_length(&maintenance_type, 50)?;
    validate_security_input(&maintenance_type)?;
    if schedule_cron.trim().is_empty() {
        return Err(StoryWeaverError::validation("schedule_cron cannot be empty"));
    }
    validate_content_length(&schedule_cron, 200)?;
    validate_security_input(&schedule_cron)?;

    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    optimization_manager
        .schedule_maintenance(&maintenance_type, &schedule_cron)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to schedule maintenance: {}", e)))?;
    
    Ok(format!(
        "Scheduled {} maintenance with cron: {}",
        maintenance_type, schedule_cron
    ))
}

fn generate_recommendations(stats: &DatabaseOptimizationStats) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    // Index recommendations
    if stats.total_indexes < 10 {
        recommendations.push("Consider adding more indexes for frequently queried columns".to_string());
    }
    
    if stats.unused_indexes > 5 {
        recommendations.push(format!(
            "Remove {} unused indexes to improve write performance",
            stats.unused_indexes
        ));
    }
    
    // Memory recommendations
    if stats.memory_usage_mb > 512.0 {
        recommendations.push("Consider reducing memory cache size or clearing old data".to_string());
    }
    
    // Cache recommendations
    if stats.cache_hit_rate < 0.8 {
        recommendations.push("Cache hit rate is low - consider increasing cache size or TTL".to_string());
    }
    
    // Query performance recommendations
    if stats.avg_query_time_ms > 100.0 {
        recommendations.push("Average query time is high - consider adding indexes or optimizing queries".to_string());
    }
    
    if recommendations.is_empty() {
        recommendations.push("Database is well optimized - no immediate recommendations".to_string());
    }
    
    recommendations
}

fn calculate_performance_score(stats: &DatabaseOptimizationStats) -> f64 {
    let mut score = 100.0;
    
    // Penalize for unused indexes
    if stats.unused_indexes > 0 {
        score -= (stats.unused_indexes as f64) * 2.0;
    }
    
    // Penalize for low cache hit rate
    if stats.cache_hit_rate < 0.9 {
        score -= (0.9 - stats.cache_hit_rate) * 50.0;
    }
    
    // Penalize for high memory usage
    if stats.memory_usage_mb > 256.0 {
        score -= (stats.memory_usage_mb - 256.0) * 0.1;
    }
    
    // Penalize for slow queries
    if stats.avg_query_time_ms > 50.0 {
        score -= (stats.avg_query_time_ms - 50.0) * 0.5;
    }
    
    // Ensure score is between 0 and 100
    score.max(0.0).min(100.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_score_calculation() {
        let stats = DatabaseOptimizationStats {
            total_tables: 10,
            total_indexes: 15,
            active_indexes: 15,
            unused_indexes: 0,
            memory_usage_mb: 128.0,
            cache_hit_rate: 0.95,
            avg_query_time_ms: 25.0,
            total_queries: 1000,
            slow_queries: 5,
            average_effectiveness_score: 0.9,
        };
        
        let score = calculate_performance_score(&stats);
        assert!(score > 90.0);
    }
    
    #[test]
    fn test_recommendations_generation() {
        let stats = DatabaseOptimizationStats {
            total_tables: 10,
            total_indexes: 5,
            active_indexes: 5,
            unused_indexes: 10,
            memory_usage_mb: 600.0,
            cache_hit_rate: 0.6,
            avg_query_time_ms: 150.0,
            total_queries: 1000,
            slow_queries: 100,
            average_effectiveness_score: 0.5,
        };
        
        let recommendations = generate_recommendations(&stats);
        assert!(recommendations.len() > 3);
        assert!(recommendations.iter().any(|r| r.contains("indexes")));
        assert!(recommendations.iter().any(|r| r.contains("memory")));
        assert!(recommendations.iter().any(|r| r.contains("cache")));
    }
}
