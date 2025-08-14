//! Lazy loading system for large documents in StoryWeaver
//! Implements chunked loading, virtual scrolling support, and memory-efficient document handling

use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

/// Document chunk for lazy loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub chunk_id: String,
    pub document_id: String,
    pub start_position: usize,
    pub end_position: usize,
    pub content: String,
    pub word_count: usize,
    pub line_count: usize,
    pub loaded_at: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
}

/// Lazy loading configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LazyLoadingConfig {
    pub chunk_size: usize,           // Characters per chunk
    pub max_chunks_in_memory: usize, // Maximum chunks to keep in memory
    pub preload_chunks: usize,       // Number of chunks to preload around current position
    pub cache_ttl_seconds: u64,      // Time to live for cached chunks
    pub enable_virtual_scrolling: bool,
    pub min_document_size: usize,    // Minimum document size to enable lazy loading
}

impl Default for LazyLoadingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 10000,        // 10KB per chunk
            max_chunks_in_memory: 50, // ~500KB max in memory
            preload_chunks: 3,        // Load 3 chunks ahead/behind
            cache_ttl_seconds: 300,   // 5 minutes
            enable_virtual_scrolling: true,
            min_document_size: 50000, // 50KB minimum for lazy loading
        }
    }
}

/// Document metadata for lazy loading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub document_id: String,
    pub total_size: usize,
    pub total_chunks: usize,
    pub word_count: usize,
    pub line_count: usize,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub chunk_map: Vec<ChunkInfo>, // Map of chunk positions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub chunk_id: String,
    pub start_position: usize,
    pub end_position: usize,
    pub word_count: usize,
    pub line_count: usize,
}

/// Lazy document loader with intelligent caching
pub struct LazyDocumentLoader {
    config: LazyLoadingConfig,
    chunk_cache: Arc<RwLock<HashMap<String, DocumentChunk>>>,
    metadata_cache: Arc<RwLock<HashMap<String, DocumentMetadata>>>,
    access_stats: Arc<RwLock<HashMap<String, AccessStats>>>,
}

#[derive(Debug, Clone)]
struct AccessStats {
    total_accesses: u64,
    cache_hits: u64,
    cache_misses: u64,
    last_access: chrono::DateTime<chrono::Utc>,
}

impl LazyDocumentLoader {
    pub fn new(config: LazyLoadingConfig) -> Self {
        Self {
            config,
            chunk_cache: Arc::new(RwLock::new(HashMap::new())),
            metadata_cache: Arc::new(RwLock::new(HashMap::new())),
            access_stats: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if a document should use lazy loading
    pub async fn should_use_lazy_loading(&self, document_size: usize) -> bool {
        document_size >= self.config.min_document_size
    }

    /// Initialize lazy loading for a document
    pub async fn initialize_document(&self, document_id: &str, content: &str) -> Result<DocumentMetadata> {
        let total_size = content.len();
        
        if !self.should_use_lazy_loading(total_size).await {
            return Err(StoryWeaverError::validation("Document too small for lazy loading"));
        }

        let chunks = self.create_chunks(document_id, content).await?;
        let total_chunks = chunks.len();
        
        let word_count = content.split_whitespace().count();
        let line_count = content.lines().count();
        
        let chunk_map: Vec<ChunkInfo> = chunks.iter().map(|chunk| ChunkInfo {
            chunk_id: chunk.chunk_id.clone(),
            start_position: chunk.start_position,
            end_position: chunk.end_position,
            word_count: chunk.word_count,
            line_count: chunk.line_count,
        }).collect();

        let metadata = DocumentMetadata {
            document_id: document_id.to_string(),
            total_size,
            total_chunks,
            word_count,
            line_count,
            last_modified: chrono::Utc::now(),
            chunk_map,
        };

        // Cache the chunks
        let mut cache = self.chunk_cache.write().await;
        for chunk in chunks {
            cache.insert(chunk.chunk_id.clone(), chunk);
        }

        // Cache the metadata
        let mut metadata_cache = self.metadata_cache.write().await;
        metadata_cache.insert(document_id.to_string(), metadata.clone());

        // Initialize access stats
        let mut stats = self.access_stats.write().await;
        stats.insert(document_id.to_string(), AccessStats {
            total_accesses: 0,
            cache_hits: 0,
            cache_misses: 0,
            last_access: chrono::Utc::now(),
        });

        Ok(metadata)
    }

    /// Create chunks from document content
    async fn create_chunks(&self, document_id: &str, content: &str) -> Result<Vec<DocumentChunk>> {
        let mut chunks = Vec::new();
        let chunk_size = self.config.chunk_size;
        let content_len = content.len();
        
        let mut start = 0;
        let mut chunk_index = 0;
        
        while start < content_len {
            let mut end = std::cmp::min(start + chunk_size, content_len);
            
            // Try to break at word boundaries
            if end < content_len {
                if let Some(last_space) = content[start..end].rfind(' ') {
                    end = start + last_space;
                }
            }
            
            let chunk_content = &content[start..end];
            let chunk_id = format!("{}_{}", document_id, chunk_index);
            
            let chunk = DocumentChunk {
                chunk_id,
                document_id: document_id.to_string(),
                start_position: start,
                end_position: end,
                content: chunk_content.to_string(),
                word_count: chunk_content.split_whitespace().count(),
                line_count: chunk_content.lines().count(),
                loaded_at: chrono::Utc::now(),
                access_count: 0,
                last_accessed: chrono::Utc::now(),
            };
            
            chunks.push(chunk);
            start = end;
            chunk_index += 1;
        }
        
        Ok(chunks)
    }

    /// Load a specific chunk by position
    pub async fn load_chunk_at_position(&self, document_id: &str, position: usize) -> Result<DocumentChunk> {
        let metadata = self.get_document_metadata(document_id).await?;
        
        // Find the chunk containing this position
        let chunk_info = metadata.chunk_map.iter()
            .find(|chunk| position >= chunk.start_position && position < chunk.end_position)
            .ok_or_else(|| StoryWeaverError::validation("Position not found in document"))?;
        
        self.load_chunk(&chunk_info.chunk_id).await
    }

    /// Load a specific chunk by ID
    pub async fn load_chunk(&self, chunk_id: &str) -> Result<DocumentChunk> {
        let mut cache = self.chunk_cache.write().await;
        
        if let Some(chunk) = cache.get_mut(chunk_id) {
            // Update access statistics
            chunk.access_count += 1;
            chunk.last_accessed = chrono::Utc::now();
            
            // Update global stats
            self.update_access_stats(chunk_id, true).await;
            
            return Ok(chunk.clone());
        }
        
        // Cache miss - would need to load from database
        self.update_access_stats(chunk_id, false).await;
        Err(StoryWeaverError::not_found("Chunk", chunk_id))
    }

    /// Load chunks around a position (for preloading)
    pub async fn load_chunks_around_position(&self, document_id: &str, position: usize) -> Result<Vec<DocumentChunk>> {
        let metadata = self.get_document_metadata(document_id).await?;
        
        // Find the current chunk index
        let current_chunk_index = metadata.chunk_map.iter()
            .position(|chunk| position >= chunk.start_position && position < chunk.end_position)
            .ok_or_else(|| StoryWeaverError::validation("Position not found in document"))?;
        
        let preload_range = self.config.preload_chunks;
        let start_index = current_chunk_index.saturating_sub(preload_range);
        let end_index = std::cmp::min(current_chunk_index + preload_range + 1, metadata.chunk_map.len());
        
        let mut chunks = Vec::new();
        for i in start_index..end_index {
            if let Ok(chunk) = self.load_chunk(&metadata.chunk_map[i].chunk_id).await {
                chunks.push(chunk);
            }
        }
        
        Ok(chunks)
    }

    /// Get document metadata
    pub async fn get_document_metadata(&self, document_id: &str) -> Result<DocumentMetadata> {
        let metadata_cache = self.metadata_cache.read().await;
        metadata_cache.get(document_id)
            .cloned()
            .ok_or_else(|| StoryWeaverError::not_found("DocumentMetadata", document_id))
    }

    /// Update access statistics
    async fn update_access_stats(&self, chunk_id: &str, cache_hit: bool) {
        let document_id = chunk_id.split('_').next().unwrap_or(chunk_id);
        let mut stats = self.access_stats.write().await;
        
        let entry = stats.entry(document_id.to_string()).or_insert(AccessStats {
            total_accesses: 0,
            cache_hits: 0,
            cache_misses: 0,
            last_access: chrono::Utc::now(),
        });
        
        entry.total_accesses += 1;
        entry.last_access = chrono::Utc::now();
        
        if cache_hit {
            entry.cache_hits += 1;
        } else {
            entry.cache_misses += 1;
        }
    }

    /// Clean up expired chunks from cache
    pub async fn cleanup_expired_chunks(&self) {
        let mut cache = self.chunk_cache.write().await;
        let now = chrono::Utc::now();
        let ttl = chrono::Duration::seconds(self.config.cache_ttl_seconds as i64);
        
        cache.retain(|_, chunk| {
            now.signed_duration_since(chunk.last_accessed) < ttl
        });
    }

    /// Evict least recently used chunks if cache is full
    pub async fn evict_lru_chunks(&self) {
        // Build list of keys to remove under a read lock to avoid mutable/immutable borrow conflict
        let keys_to_remove: Vec<String> = {
            let cache_read = self.chunk_cache.read().await;
            if cache_read.len() <= self.config.max_chunks_in_memory {
                return;
            }
            let mut items: Vec<(String, chrono::DateTime<chrono::Utc>)> = cache_read.iter()
                .map(|(k, v)| (k.clone(), v.last_accessed))
                .collect();
            // Oldest first
            items.sort_by(|a, b| a.1.cmp(&b.1));
            let chunks_to_remove = cache_read.len() - self.config.max_chunks_in_memory;
            items.into_iter().take(chunks_to_remove).map(|(k, _)| k).collect()
        };

        if keys_to_remove.is_empty() {
            return;
        }

        let mut cache = self.chunk_cache.write().await;
        for key in keys_to_remove {
            cache.remove(&key);
        }
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let cache = self.chunk_cache.read().await;
        let stats = self.access_stats.read().await;
        
        let total_chunks = cache.len();
        let total_memory_kb = cache.values()
            .map(|chunk| chunk.content.len())
            .sum::<usize>() / 1024;
        
        let (total_accesses, total_hits, total_misses) = stats.values()
            .fold((0, 0, 0), |(acc, hits, misses), stat| {
                (acc + stat.total_accesses, hits + stat.cache_hits, misses + stat.cache_misses)
            });
        
        let hit_rate = if total_accesses > 0 {
            total_hits as f64 / total_accesses as f64
        } else {
            0.0
        };
        
        CacheStats {
            total_chunks,
            max_chunks: self.config.max_chunks_in_memory,
            total_memory_kb,
            total_accesses,
            cache_hits: total_hits,
            cache_misses: total_misses,
            hit_rate,
        }
    }

    /// Clear all cached data for a document
    pub async fn clear_document_cache(&self, document_id: &str) {
        let mut cache = self.chunk_cache.write().await;
        cache.retain(|_, chunk| chunk.document_id != document_id);
        
        let mut metadata_cache = self.metadata_cache.write().await;
        metadata_cache.remove(document_id);
        
        let mut stats = self.access_stats.write().await;
        stats.remove(document_id);
    }
}

/// Cache statistics for monitoring
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_chunks: usize,
    pub max_chunks: usize,
    pub total_memory_kb: usize,
    pub total_accesses: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub hit_rate: f64,
}

/// Global lazy loader instance
static LAZY_LOADER: OnceLock<Arc<LazyDocumentLoader>> = OnceLock::new();

/// Initialize the global lazy document loader
pub fn init_lazy_loader(config: LazyLoadingConfig) -> Result<()> {
    LAZY_LOADER.set(Arc::new(LazyDocumentLoader::new(config)))
        .map_err(|_| StoryWeaverError::validation("Lazy document loader already initialized"))
}

/// Get the global lazy document loader
pub fn get_lazy_loader() -> Result<Arc<LazyDocumentLoader>> {
    LAZY_LOADER.get().cloned().ok_or_else(|| {
        StoryWeaverError::validation("Lazy document loader not initialized")
    })
}

/// Background task to periodically clean up expired chunks
pub async fn start_lazy_loading_cleanup_task() {
    let loader = match get_lazy_loader() {
        Ok(loader) => loader,
        Err(_) => return,
    };
    
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60)); // 1 minute
    
    loop {
        interval.tick().await;
        loader.cleanup_expired_chunks().await;
        loader.evict_lru_chunks().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lazy_loading_initialization() {
        let config = LazyLoadingConfig::default();
        let loader = LazyDocumentLoader::new(config);
        
        let large_content = "word ".repeat(20000); // 100KB content
        let metadata = loader.initialize_document("test_doc", &large_content).await.unwrap();
        
        assert_eq!(metadata.document_id, "test_doc");
        assert!(metadata.total_chunks > 1);
        assert_eq!(metadata.total_size, large_content.len());
    }

    #[tokio::test]
    async fn test_chunk_loading() {
        let config = LazyLoadingConfig::default();
        let loader = LazyDocumentLoader::new(config);
        
        let content = "word ".repeat(5000); // 25KB content
        let _metadata = loader.initialize_document("test_doc", &content).await.unwrap();
        
        // Load chunk at position 0
        let chunk = loader.load_chunk_at_position("test_doc", 0).await.unwrap();
        assert_eq!(chunk.start_position, 0);
        assert!(chunk.content.len() > 0);
    }

    #[tokio::test]
    async fn test_preloading() {
        let config = LazyLoadingConfig::default();
        let loader = LazyDocumentLoader::new(config);
        
        let content = "word ".repeat(10000); // 50KB content
        let _metadata = loader.initialize_document("test_doc", &content).await.unwrap();
        
        // Load chunks around position 1000
        let chunks = loader.load_chunks_around_position("test_doc", 1000).await.unwrap();
        assert!(chunks.len() > 1); // Should load multiple chunks
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let config = LazyLoadingConfig::default();
        let loader = LazyDocumentLoader::new(config);
        
        let content = "word ".repeat(5000);
        let _metadata = loader.initialize_document("test_doc", &content).await.unwrap();
        
        // Access some chunks
        let _chunk = loader.load_chunk_at_position("test_doc", 0).await.unwrap();
        let _chunk = loader.load_chunk_at_position("test_doc", 0).await.unwrap(); // Second access for cache hit
        
        let stats = loader.get_cache_stats().await;
        assert!(stats.total_chunks > 0);
        assert!(stats.cache_hits > 0);
        assert!(stats.hit_rate > 0.0);
    }
}
