//! Database optimization module for StoryWeaver
//! Implements performance optimization strategies including indexing, caching, and memory management

pub mod index_manager;
pub mod memory_optimizer;
pub mod ai_response_cache;

pub use index_manager::*;
pub use memory_optimizer::*;
pub use ai_response_cache::*;

use crate::error::Result;
use tracing::info;

/// Initialize all optimization components
pub async fn initialize_optimization_components() -> Result<OptimizationManager> {
    info!("Initializing database optimization components");
    
    let index_manager = IndexManager::new();
    let memory_optimizer = MemoryOptimizedProcessor::new(MemoryConfig::default());
    let ai_cache = initialize_ai_response_cache(None);
    
    Ok(OptimizationManager {
        index_manager,
        memory_optimizer,
        ai_cache,
    })
}

/// Central manager for all optimization components
pub struct OptimizationManager {
    pub index_manager: IndexManager,
    pub memory_optimizer: MemoryOptimizedProcessor,
    pub ai_cache: AIResponseCache,
}

impl OptimizationManager {
    /// Create a new OptimizationManager with database pool
    pub async fn new(pool: std::sync::Arc<crate::database::DbPool>) -> Result<Self> {
        let index_manager = IndexManager::new_with_pool(pool.clone()).await?;
        let memory_optimizer = MemoryOptimizedProcessor::new(MemoryConfig::default());
        let ai_cache = initialize_ai_response_cache(None);
        
        Ok(Self {
            index_manager,
            memory_optimizer,
            ai_cache,
        })
    }
    
    /// Get comprehensive optimization statistics
    pub async fn get_optimization_stats(&self) -> Result<DatabaseOptimizationStats> {
        // Return database stats directly as expected by optimization_commands.rs
        self.index_manager.get_database_stats().await
    }
    
    /// Perform comprehensive optimization cleanup
    pub async fn perform_maintenance(&self) -> Result<()> {
        info!("Performing optimization maintenance");
        
        // Clean up unused indexes
        self.index_manager.cleanup_unused_indexes().await?;
        
        // Clear memory caches if needed
        self.memory_optimizer.clear_caches().await;
        
        // Cleanup AI response cache
        // Note: The cache has its own internal cleanup, but we could trigger it here if needed
        
        info!("Optimization maintenance completed");
        Ok(())
    }
    
    /// Create recommended indexes based on usage patterns
    pub async fn create_recommended_indexes(&self) -> Result<()> {
        self.index_manager.create_recommended_indexes().await
    }
    
    /// Optimize memory usage to target size
    pub async fn optimize_memory_usage(&self, target_mb: usize) -> Result<()> {
        info!("Optimizing memory usage to target: {}MB", target_mb);
        
        // Clear caches if memory usage is too high
        let current_stats = self.memory_optimizer.get_cache_stats().await;
        if current_stats.total_memory_usage_mb > target_mb as f64 {
            self.memory_optimizer.clear_caches().await?;
        }
        
        Ok(())
    }
    
    /// Run performance analysis
    pub async fn run_performance_analysis(&self) -> Result<serde_json::Value> {
        let db_stats = self.index_manager.get_database_stats().await?;
        let memory_stats = self.memory_optimizer.get_cache_stats().await;
        
        let analysis = serde_json::json!({
            "database": {
                "total_tables": db_stats.total_tables,
                "total_indexes": db_stats.total_indexes,
                "unused_indexes": db_stats.unused_indexes,
                "avg_query_time_ms": db_stats.avg_query_time_ms,
                "slow_queries": db_stats.slow_queries
            },
            "memory": {
                "total_usage_mb": memory_stats.total_memory_usage_mb,
                "cache_efficiency": memory_stats.total_memory_usage_mb / 256.0 // Efficiency ratio
            },
            "recommendations": [
                if db_stats.unused_indexes > 5 { Some("Remove unused indexes") } else { None },
                if db_stats.avg_query_time_ms > 100.0 { Some("Optimize slow queries") } else { None },
                if memory_stats.total_memory_usage_mb > 512.0 { Some("Reduce memory usage") } else { None }
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
        });
        
        Ok(analysis)
    }
    
    /// Schedule maintenance tasks
    pub async fn schedule_maintenance(&self, maintenance_type: &str, schedule_cron: &str) -> Result<()> {
        info!("Scheduling {} maintenance with cron: {}", maintenance_type, schedule_cron);
        
        // For now, just log the scheduling request
        // In a full implementation, this would integrate with a job scheduler
        match maintenance_type {
            "cleanup" => info!("Scheduled cleanup maintenance"),
            "optimization" => info!("Scheduled optimization maintenance"),
            "backup" => info!("Scheduled backup maintenance"),
            _ => info!("Scheduled {} maintenance", maintenance_type),
        }
        
        Ok(())
    }
    
    /// Get cache statistics from memory optimizer
    pub async fn get_cache_statistics(&self) -> Result<serde_json::Value> {
        let cache_stats = self.memory_optimizer.get_cache_stats().await;
        Ok(serde_json::to_value(cache_stats)
            .map_err(|e| crate::error::StoryWeaverError::system(format!("Failed to serialize cache stats: {}", e)))?)
    }

    pub async fn create_custom_index(
        &self,
        table_name: &str,
        columns: &[String],
        index_type: Option<&str>,
    ) -> Result<String> {
        self.index_manager.create_custom_index(table_name, columns, index_type).await
    }

    pub async fn cleanup_unused_indexes(
        &self,
        threshold: f64,
    ) -> Result<Vec<String>> {
        self.index_manager.cleanup_unused_indexes(threshold).await
    }

    pub async fn clear_ai_cache(
        &self,
        _older_than_hours: u64,
    ) -> Result<usize> {
        // For now, clear all cache entries
        // TODO: Implement time-based clearing in AIResponseCache
        let stats_before = self.ai_cache.get_statistics().await;
        let entries_before = stats_before.total_entries;
        
        self.ai_cache.clear_cache().await?;
        
        Ok(entries_before)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizationStats {
    pub database: DatabaseOptimizationStats,
    pub memory: MemoryStats,
    pub ai_cache: CacheStatistics,
}