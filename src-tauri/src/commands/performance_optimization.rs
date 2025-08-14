//! Performance optimization commands for StoryWeaver
//! Provides Tauri commands for database optimization, caching, and performance monitoring

use crate::error::{Result, StoryWeaverError};
use crate::database::optimization::OptimizationManager;
use crate::database::get_pool;
use crate::ai::cache::get_ai_cache;
use crate::ai::streaming_optimizer::{get_streaming_optimizer, StreamInfo};
use crate::documents::get_lazy_loader;
use serde::{Deserialize, Serialize};
use tauri::{command, State};
use crate::database::DbPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceOverview {
    pub database: DatabasePerformanceInfo,
    pub ai_cache: AICacheInfo,
    pub streaming: StreamingPerformanceInfo,
    pub document_cache: DocumentCacheInfo,
    pub memory_usage: MemoryUsageInfo,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabasePerformanceInfo {
    pub total_tables: usize,
    pub total_indexes: usize,
    pub unused_indexes: usize,
    pub avg_query_time_ms: f64,
    pub memory_usage_mb: f64,
    pub slow_queries: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AICacheInfo {
    pub size: usize,
    pub max_size: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub total_cost_saved: f64,
    pub total_tokens_saved: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingPerformanceInfo {
    pub active_streams: usize,
    pub total_streams_created: u64,
    pub total_streams_completed: u64,
    pub total_memory_usage: usize,
    pub peak_memory_usage: usize,
    pub backpressure_events: u64,
    pub cleanup_events: u64,
    pub average_stream_duration_ms: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentCacheInfo {
    pub total_chunks: usize,
    pub max_chunks: usize,
    pub total_memory_kb: usize,
    pub total_accesses: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub hit_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryUsageInfo {
    pub total_usage_mb: f64,
    pub database_mb: f64,
    pub ai_cache_mb: f64,
    pub streaming_mb: f64,
    pub document_cache_mb: f64,
    pub pressure_level: f64, // 0.0 to 1.0
}

/// Get comprehensive performance overview
#[command]
pub async fn get_performance_overview() -> Result<PerformanceOverview> {
    // Get database performance info
    let pool = get_pool()?;
    let optimization_manager = OptimizationManager::new(pool).await?;
    let db_stats = optimization_manager.get_optimization_stats().await?;
    
    let database = DatabasePerformanceInfo {
        total_tables: db_stats.total_tables,
        total_indexes: db_stats.total_indexes,
        unused_indexes: db_stats.unused_indexes,
        avg_query_time_ms: db_stats.avg_query_time_ms,
        memory_usage_mb: db_stats.memory_usage_mb,
        slow_queries: db_stats.slow_queries,
    };

    // Get AI cache info
    let ai_cache_info = match get_ai_cache() {
        Ok(cache) => {
            let stats = cache.get_stats().await;
            AICacheInfo {
                size: stats.size,
                max_size: stats.max_size,
                hit_count: stats.hit_count,
                miss_count: stats.miss_count,
                hit_rate: stats.hit_rate,
                total_cost_saved: stats.total_cost_saved,
                total_tokens_saved: stats.total_tokens_saved,
            }
        }
        Err(_) => AICacheInfo {
            size: 0,
            max_size: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            total_cost_saved: 0.0,
            total_tokens_saved: 0,
        }
    };

    // Get streaming performance info
    let streaming_info = match get_streaming_optimizer() {
        Ok(optimizer) => {
            let stats = optimizer.get_stats().await;
            StreamingPerformanceInfo {
                active_streams: stats.active_streams,
                total_streams_created: stats.total_streams_created,
                total_streams_completed: stats.total_streams_completed,
                total_memory_usage: stats.total_memory_usage,
                peak_memory_usage: stats.peak_memory_usage,
                backpressure_events: stats.backpressure_events,
                cleanup_events: stats.cleanup_events,
                average_stream_duration_ms: stats.average_stream_duration_ms,
            }
        }
        Err(_) => StreamingPerformanceInfo {
            active_streams: 0,
            total_streams_created: 0,
            total_streams_completed: 0,
            total_memory_usage: 0,
            peak_memory_usage: 0,
            backpressure_events: 0,
            cleanup_events: 0,
            average_stream_duration_ms: 0.0,
        }
    };

    // Get document cache info
    let document_cache_info = match get_lazy_loader() {
        Ok(loader) => {
            let stats = loader.get_cache_stats().await;
            DocumentCacheInfo {
                total_chunks: stats.total_chunks,
                max_chunks: stats.max_chunks,
                total_memory_kb: stats.total_memory_kb,
                total_accesses: stats.total_accesses,
                cache_hits: stats.cache_hits,
                cache_misses: stats.cache_misses,
                hit_rate: stats.hit_rate,
            }
        }
        Err(_) => DocumentCacheInfo {
            total_chunks: 0,
            max_chunks: 0,
            total_memory_kb: 0,
            total_accesses: 0,
            cache_hits: 0,
            cache_misses: 0,
            hit_rate: 0.0,
        }
    };

    // Calculate memory usage
    let database_mb = database.memory_usage_mb;
    let ai_cache_mb = (ai_cache_info.size * 1000) as f64 / 1024.0 / 1024.0; // Rough estimate
    let streaming_mb = streaming_info.total_memory_usage as f64 / 1024.0 / 1024.0;
    let document_cache_mb = document_cache_info.total_memory_kb as f64 / 1024.0;
    let total_usage_mb = database_mb + ai_cache_mb + streaming_mb + document_cache_mb;

    let memory_usage = MemoryUsageInfo {
        total_usage_mb,
        database_mb,
        ai_cache_mb,
        streaming_mb,
        document_cache_mb,
        pressure_level: total_usage_mb / 512.0, // Assume 512MB as baseline
    };

    // Generate recommendations
    let mut recommendations = Vec::new();
    
    if database.unused_indexes > 5 {
        recommendations.push("Consider removing unused database indexes to improve performance".to_string());
    }
    
    if database.avg_query_time_ms > 100.0 {
        recommendations.push("Database queries are slow - consider optimizing or adding indexes".to_string());
    }
    
    if ai_cache_info.hit_rate < 0.5 && ai_cache_info.hit_count + ai_cache_info.miss_count > 100 {
        recommendations.push("AI cache hit rate is low - consider increasing cache size or TTL".to_string());
    }
    
    if streaming_info.backpressure_events > 10 {
        recommendations.push("High streaming backpressure - consider increasing memory limits".to_string());
    }
    
    if memory_usage.pressure_level > 0.8 {
        recommendations.push("High memory usage detected - consider clearing caches or optimizing".to_string());
    }
    
    if document_cache_info.hit_rate < 0.6 && document_cache_info.total_accesses > 50 {
        recommendations.push("Document cache efficiency is low - consider tuning chunk size or cache settings".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("Performance looks good! No immediate optimizations needed.".to_string());
    }

    Ok(PerformanceOverview {
        database,
        ai_cache: ai_cache_info,
        streaming: streaming_info,
        document_cache: document_cache_info,
        memory_usage,
        recommendations,
    })
}

/// Optimize database indexes
#[command]
pub async fn optimize_database_indexes() -> Result<String> {
    let pool = get_pool()?;
    let optimization_manager = OptimizationManager::new(pool).await?;
    
    optimization_manager.create_recommended_indexes().await?;
    
    Ok("Database indexes optimized successfully".to_string())
}

/// Clean up unused database indexes
#[command]
pub async fn cleanup_unused_indexes(threshold: f64) -> Result<Vec<String>> {
    let pool = get_pool()?;
    let optimization_manager = OptimizationManager::new(pool).await?;
    
    optimization_manager.cleanup_unused_indexes(threshold).await
}

/// Clear AI response cache (enhanced version)
#[command]
pub async fn clear_ai_response_cache(older_than_hours: Option<u64>) -> Result<String> {
    match get_ai_cache() {
        Ok(cache) => {
            if let Some(hours) = older_than_hours {
                cache.remove_older_than(hours as i64).await;
                Ok(format!("Cleared AI cache entries older than {} hours", hours))
            } else {
                cache.clear().await;
                Ok("AI cache cleared successfully".to_string())
            }
        }
        Err(_) => Err(StoryWeaverError::system("AI cache not initialized"))
    }
}

/// Clear streaming buffers
#[command]
pub async fn clear_streaming_buffers() -> Result<String> {
    match get_streaming_optimizer() {
        Ok(optimizer) => {
            optimizer.cleanup_all_streams().await?;
            Ok("Streaming buffers cleared successfully".to_string())
        }
        Err(_) => Err(StoryWeaverError::system("Streaming optimizer not initialized"))
    }
}

/// Clear document cache
#[command]
pub async fn clear_document_cache(document_id: Option<String>) -> Result<String> {
    match get_lazy_loader() {
        Ok(loader) => {
            if let Some(doc_id) = document_id {
                loader.clear_document_cache(&doc_id).await;
                Ok(format!("Document cache cleared for document: {}", doc_id))
            } else {
                // Clear all document caches - would need to implement this method
                Ok("Document cache clearing not fully implemented for all documents".to_string())
            }
        }
        Err(_) => Err(StoryWeaverError::system("Document lazy loader not initialized"))
    }
}

/// Get streaming performance details
#[command]
pub async fn get_streaming_details() -> Result<Vec<StreamInfo>> {
    match get_streaming_optimizer() {
        Ok(_optimizer) => {
            // This would need to be implemented in the streaming optimizer
            // For now, return empty list
            Ok(vec![])
        }
        Err(_) => Err(StoryWeaverError::system("Streaming optimizer not initialized"))
    }
}

/// Force garbage collection and memory optimization
/// NOTE: This command is provided by `optimization_commands::optimize_memory_usage`.
/// Kept as a no-op wrapper (not exposed as a Tauri command here) to avoid duplicate
/// command symbol definitions. Call the canonical implementation in
/// `commands::optimization_commands` from the frontend instead.
pub async fn optimize_memory_usage_internal(pool: State<'_, DbPool>, target_mb: Option<usize>) -> Result<String> {
    // Delegate to the optimization_commands implementation
    crate::commands::optimization_commands::optimize_memory_usage(pool, target_mb).await
}

/// Run comprehensive performance analysis
pub async fn run_performance_analysis() -> Result<serde_json::Value> {
    let pool = get_pool()?;
    let optimization_manager = OptimizationManager::new(pool).await?;
    
    optimization_manager.run_performance_analysis().await
}

/// Get cache statistics for monitoring
pub async fn get_cache_statistics() -> Result<serde_json::Value> {
    let mut stats = serde_json::Map::new();
    
    // AI Cache stats
    if let Ok(cache) = get_ai_cache() {
        let ai_stats = cache.get_stats().await;
        stats.insert("ai_cache".to_string(), serde_json::to_value(ai_stats)?);
    }
    
    // Streaming stats
    if let Ok(optimizer) = get_streaming_optimizer() {
        let streaming_stats = optimizer.get_stats().await;
        stats.insert("streaming".to_string(), serde_json::to_value(streaming_stats)?);
    }
    
    // Document cache stats
    if let Ok(loader) = get_lazy_loader() {
        let doc_stats = loader.get_cache_stats().await;
        stats.insert("document_cache".to_string(), serde_json::to_value(doc_stats)?);
    }
    
    Ok(serde_json::Value::Object(stats))
}

/// Schedule maintenance tasks
pub async fn schedule_maintenance(maintenance_type: String, schedule_cron: String) -> Result<String> {
    let pool = get_pool()?;
    let optimization_manager = OptimizationManager::new(pool).await?;
    
    optimization_manager.schedule_maintenance(&maintenance_type, &schedule_cron).await?;
    
    Ok(format!("Scheduled {} maintenance with cron: {}", maintenance_type, schedule_cron))
}

/// Create custom database index
#[command]
pub async fn create_custom_index(
    table_name: String,
    columns: Vec<String>,
    index_type: Option<String>,
) -> Result<String> {
    let pool = get_pool()?;
    let optimization_manager = OptimizationManager::new(pool).await?;
    
    optimization_manager.create_custom_index(&table_name, &columns, index_type.as_deref()).await
}

/// Get memory pressure level
#[command]
pub async fn get_memory_pressure() -> Result<f64> {
    let mut total_pressure = 0.0;
    let mut components = 0;
    
    // Streaming memory pressure
    if let Ok(optimizer) = get_streaming_optimizer() {
        total_pressure += optimizer.get_memory_pressure().await;
        components += 1;
    }
    
    // Could add other memory pressure sources here
    
    if components > 0 {
        Ok(total_pressure / components as f64)
    } else {
        Ok(0.0)
    }
}
