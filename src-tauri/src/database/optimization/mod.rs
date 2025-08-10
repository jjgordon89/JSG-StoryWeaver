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
    /// Get comprehensive optimization statistics
    pub async fn get_optimization_stats(&self) -> Result<OptimizationStats> {
        let db_stats = self.index_manager.get_optimization_stats().await?;
        let memory_stats = self.memory_optimizer.get_memory_stats().await;
        let cache_stats = self.ai_cache.get_statistics().await;
        
        Ok(OptimizationStats {
            database: db_stats,
            memory: memory_stats,
            ai_cache: cache_stats,
        })
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
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizationStats {
    pub database: DatabaseOptimizationStats,
    pub memory: MemoryStats,
    pub ai_cache: CacheStatistics,
}