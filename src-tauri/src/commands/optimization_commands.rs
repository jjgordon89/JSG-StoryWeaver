use crate::database::optimization::{OptimizationManager, DatabaseOptimizationStats};
use crate::database::DbPool;
use crate::error::StoryWeaverError;
use serde::{Deserialize, Serialize};
use tauri::State;

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
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let recommendations = optimization_manager
        .analyze_query_patterns()
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to analyze query patterns: {}", e)))?;
    
    Ok(recommendations
        .into_iter()
        .map(|rec| IndexRecommendation {
            table_name: rec.table_name,
            columns: rec.columns,
            index_type: rec.index_type,
            estimated_benefit: rec.estimated_benefit,
            reason: rec.reason,
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
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let threshold = min_usage_threshold.unwrap_or(0.1); // 10% default threshold
    
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
    let optimization_manager = OptimizationManager::new(std::sync::Arc::new(pool.inner().clone()))
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create optimization manager: {}", e)))?;
    
    let target = target_mb.unwrap_or(256); // Default to 256MB
    
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
    if stats.memory_usage_mb > 512 {
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
    if stats.memory_usage_mb > 256 {
        score -= ((stats.memory_usage_mb - 256) as f64) * 0.1;
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
            total_indexes: 15,
            unused_indexes: 0,
            memory_usage_mb: 128,
            cache_hit_rate: 0.95,
            avg_query_time_ms: 25.0,
            total_queries: 1000,
            slow_queries: 5,
        };
        
        let score = calculate_performance_score(&stats);
        assert!(score > 90.0);
    }
    
    #[test]
    fn test_recommendations_generation() {
        let stats = DatabaseOptimizationStats {
            total_indexes: 5,
            unused_indexes: 10,
            memory_usage_mb: 600,
            cache_hit_rate: 0.6,
            avg_query_time_ms: 150.0,
            total_queries: 1000,
            slow_queries: 100,
        };
        
        let recommendations = generate_recommendations(&stats);
        assert!(recommendations.len() > 3);
        assert!(recommendations.iter().any(|r| r.contains("indexes")));
        assert!(recommendations.iter().any(|r| r.contains("memory")));
        assert!(recommendations.iter().any(|r| r.contains("cache")));
    }
}