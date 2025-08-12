//! Memory optimization for efficient document processing
//! Implements LRU caches, chunked processing, and memory monitoring

use crate::error::{Result, StoryWeaverError};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use tracing::{info, warn, debug};

/// Memory-optimized processor for handling large documents and embeddings
pub struct MemoryOptimizedProcessor {
    document_cache: Arc<RwLock<LruCache<String, CachedDocument>>>,
    embedding_cache: Arc<RwLock<LruCache<String, CachedEmbedding>>>,
    memory_monitor: Arc<MemoryMonitor>,
    config: MemoryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedDocument {
    pub id: String,
    pub content: String,
    pub word_count: usize,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub size_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedEmbedding {
    pub content_hash: String,
    pub embedding: Vec<f32>,
    pub model: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub max_document_cache_size: usize,
    pub max_embedding_cache_size: usize,
    pub chunk_size: usize,
    pub max_memory_usage_mb: usize,
    pub cleanup_threshold_mb: usize,
    pub enable_compression: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            max_document_cache_size: 100,
            max_embedding_cache_size: 500,
            chunk_size: 1000, // words per chunk
            max_memory_usage_mb: 512,
            cleanup_threshold_mb: 400,
            enable_compression: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentChunk {
    pub chunk_id: String,
    pub document_id: String,
    pub content: String,
    pub start_position: usize,
    pub end_position: usize,
    pub word_count: usize,
    pub chunk_index: usize,
    pub total_chunks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub document_id: String,
    pub chunks: Vec<DocumentChunk>,
    pub total_words: usize,
    pub processing_time_ms: u64,
    pub memory_used_mb: f64,
    pub cache_hits: usize,
    pub cache_misses: usize,
}

impl MemoryOptimizedProcessor {
    /// Create a new memory-optimized processor
    pub fn new(config: MemoryConfig) -> Self {
        let document_cache_size = NonZeroUsize::new(config.max_document_cache_size)
            .unwrap_or_else(|| NonZeroUsize::new(100).expect("Default cache size should be valid"));
        let embedding_cache_size = NonZeroUsize::new(config.max_embedding_cache_size)
            .unwrap_or_else(|| NonZeroUsize::new(500).expect("Default cache size should be valid"));

        Self {
            document_cache: Arc::new(RwLock::new(LruCache::new(document_cache_size))),
            embedding_cache: Arc::new(RwLock::new(LruCache::new(embedding_cache_size))),
            memory_monitor: Arc::new(MemoryMonitor::new()),
            config,
        }
    }

    /// Process a document with memory optimization
    pub async fn process_document(
        &self,
        document_id: &str,
        content: &str,
    ) -> Result<ProcessingResult> {
        let start_time = std::time::Instant::now();
        let initial_memory = self.memory_monitor.get_current_usage().await?;
        
        debug!("Processing document {} with {} characters", document_id, content.len());
        
        // Check if document is already cached
        let mut cache_hits = 0;
        let mut cache_misses = 0;
        
        if let Some(cached_doc) = self.get_cached_document(document_id).await {
            if cached_doc.content == content {
                cache_hits += 1;
                debug!("Document {} found in cache", document_id);
                
                // Return cached result if content hasn't changed
                let chunks = self.create_chunks_from_cached(&cached_doc).await?;
                return Ok(ProcessingResult {
                    document_id: document_id.to_string(),
                    chunks,
                    total_words: cached_doc.word_count,
                    processing_time_ms: start_time.elapsed().as_millis() as u64,
                    memory_used_mb: 0.0, // No additional memory used for cached content
                    cache_hits,
                    cache_misses,
                });
            }
        }
        
        cache_misses += 1;
        
        // Check memory usage before processing
        self.ensure_memory_availability().await?;
        
        // Create document chunks for efficient processing
        let chunks = self.create_document_chunks(document_id, content).await?;
        
        // Cache the processed document
        self.cache_document(document_id, content).await?;
        
        let final_memory = self.memory_monitor.get_current_usage().await?;
        let memory_used = final_memory - initial_memory;
        
        let result = ProcessingResult {
            document_id: document_id.to_string(),
            chunks,
            total_words: self.count_words(content),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            memory_used_mb: memory_used,
            cache_hits,
            cache_misses,
        };
        
        info!(
            "Processed document {} in {}ms, memory used: {:.2}MB",
            document_id, result.processing_time_ms, result.memory_used_mb
        );
        
        Ok(result)
    }
    
    /// Create document chunks for efficient processing
    async fn create_document_chunks(
        &self,
        document_id: &str,
        content: &str,
    ) -> Result<Vec<DocumentChunk>> {
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len();
        let chunk_size = self.config.chunk_size;
        
        if total_words <= chunk_size {
            // Document is small enough to process as a single chunk
            return Ok(vec![DocumentChunk {
                chunk_id: format!("{}_chunk_0", document_id),
                document_id: document_id.to_string(),
                content: content.to_string(),
                start_position: 0,
                end_position: content.len(),
                word_count: total_words,
                chunk_index: 0,
                total_chunks: 1,
            }]);
        }
        
        let mut chunks = Vec::new();
        let total_chunks = (total_words + chunk_size - 1) / chunk_size; // Ceiling division
        
        for (chunk_index, word_chunk) in words.chunks(chunk_size).enumerate() {
            let chunk_content = word_chunk.join(" ");
            let start_position = if chunk_index == 0 {
                0
            } else {
                // Find the actual character position in the original content
                let words_before: Vec<&str> = words.iter().take(chunk_index * chunk_size).copied().collect();
                words_before.join(" ").len() + 1 // +1 for the space
            };
            
            let end_position = start_position + chunk_content.len();
            
            chunks.push(DocumentChunk {
                chunk_id: format!("{}_chunk_{}", document_id, chunk_index),
                document_id: document_id.to_string(),
                content: chunk_content,
                start_position,
                end_position,
                word_count: word_chunk.len(),
                chunk_index,
                total_chunks,
            });
        }
        
        debug!("Created {} chunks for document {}", chunks.len(), document_id);
        Ok(chunks)
    }
    
    /// Create chunks from cached document
    async fn create_chunks_from_cached(
        &self,
        cached_doc: &CachedDocument,
    ) -> Result<Vec<DocumentChunk>> {
        self.create_document_chunks(&cached_doc.id, &cached_doc.content).await
    }
    
    /// Cache a document for future use
    async fn cache_document(&self, document_id: &str, content: &str) -> Result<()> {
        let cached_doc = CachedDocument {
            id: document_id.to_string(),
            content: content.to_string(),
            word_count: self.count_words(content),
            last_accessed: chrono::Utc::now(),
            access_count: 1,
            size_bytes: content.len(),
        };
        
        let mut cache = self.document_cache.write().await;
        cache.put(document_id.to_string(), cached_doc);
        
        debug!("Cached document {} ({} bytes)", document_id, content.len());
        Ok(())
    }
    
    /// Get a cached document
    async fn get_cached_document(&self, document_id: &str) -> Option<CachedDocument> {
        let mut cache = self.document_cache.write().await;
        if let Some(cached_doc) = cache.get_mut(document_id) {
            cached_doc.last_accessed = chrono::Utc::now();
            cached_doc.access_count += 1;
            Some(cached_doc.clone())
        } else {
            None
        }
    }
    
    /// Cache an embedding
    pub async fn cache_embedding(
        &self,
        content_hash: &str,
        embedding: Vec<f32>,
        model: &str,
    ) -> Result<()> {
        let cached_embedding = CachedEmbedding {
            content_hash: content_hash.to_string(),
            embedding,
            model: model.to_string(),
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            access_count: 1,
        };
        
        let mut cache = self.embedding_cache.write().await;
        cache.put(content_hash.to_string(), cached_embedding);
        
        debug!("Cached embedding for content hash {}", content_hash);
        Ok(())
    }
    
    /// Get a cached embedding
    pub async fn get_cached_embedding(&self, content_hash: &str) -> Option<CachedEmbedding> {
        let mut cache = self.embedding_cache.write().await;
        if let Some(cached_embedding) = cache.get_mut(content_hash) {
            cached_embedding.last_accessed = chrono::Utc::now();
            cached_embedding.access_count += 1;
            Some(cached_embedding.clone())
        } else {
            None
        }
    }
    
    /// Ensure sufficient memory is available for processing
    async fn ensure_memory_availability(&self) -> Result<()> {
        let current_usage = self.memory_monitor.get_current_usage().await?;
        
        if current_usage > self.config.cleanup_threshold_mb as f64 {
            warn!(
                "Memory usage ({:.2}MB) exceeds cleanup threshold ({}MB), performing cleanup",
                current_usage, self.config.cleanup_threshold_mb
            );
            
            self.cleanup_caches().await?;
            
            let post_cleanup_usage = self.memory_monitor.get_current_usage().await?;
            info!(
                "Memory cleanup completed: {:.2}MB -> {:.2}MB",
                current_usage, post_cleanup_usage
            );
        }
        
        Ok(())
    }
    
    /// Clean up caches to free memory
    async fn cleanup_caches(&self) -> Result<()> {
        // Clean up document cache - remove least recently used items
        {
            let mut doc_cache = self.document_cache.write().await;
            let initial_size = doc_cache.len();
            
            // Remove 25% of cached documents
            let items_to_remove = (initial_size / 4).max(1);
            for _ in 0..items_to_remove {
                if doc_cache.pop_lru().is_none() {
                    break;
                }
            }
            
            debug!(
                "Document cache cleanup: {} -> {} items",
                initial_size,
                doc_cache.len()
            );
        }
        
        // Clean up embedding cache
        {
            let mut emb_cache = self.embedding_cache.write().await;
            let initial_size = emb_cache.len();
            
            // Remove 25% of cached embeddings
            let items_to_remove = (initial_size / 4).max(1);
            for _ in 0..items_to_remove {
                if emb_cache.pop_lru().is_none() {
                    break;
                }
            }
            
            debug!(
                "Embedding cache cleanup: {} -> {} items",
                initial_size,
                emb_cache.len()
            );
        }
        
        Ok(())
    }
    
    /// Count words in content
    fn count_words(&self, content: &str) -> usize {
        content.split_whitespace().count()
    }
    
    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let doc_cache = self.document_cache.read().await;
        let emb_cache = self.embedding_cache.read().await;
        
        CacheStats {
            document_cache_size: doc_cache.len(),
            document_cache_capacity: doc_cache.cap().get(),
            embedding_cache_size: emb_cache.len(),
            embedding_cache_capacity: emb_cache.cap().get(),
            total_memory_usage_mb: self.memory_monitor.get_current_usage().await.unwrap_or(0.0),
        }
    }
    
    /// Clear all caches
    pub async fn clear_caches(&self) -> Result<()> {
        {
            let mut doc_cache = self.document_cache.write().await;
            doc_cache.clear();
        }
        
        {
            let mut emb_cache = self.embedding_cache.write().await;
            emb_cache.clear();
        }
        
        info!("All caches cleared");
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub document_cache_size: usize,
    pub document_cache_capacity: usize,
    pub embedding_cache_size: usize,
    pub embedding_cache_capacity: usize,
    pub total_memory_usage_mb: f64,
}

/// Memory monitor for tracking system memory usage
pub struct MemoryMonitor {
    component_usage: Arc<Mutex<HashMap<String, f64>>>,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        Self {
            component_usage: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Get current memory usage in MB
    pub async fn get_current_usage(&self) -> Result<f64> {
        #[cfg(target_os = "windows")]
        {
            self.get_windows_memory_usage().await
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            self.get_unix_memory_usage().await
        }
    }
    
    #[cfg(target_os = "windows")]
    async fn get_windows_memory_usage(&self) -> Result<f64> {
        use std::process::Command;
        
        let output = Command::new("powershell")
            .args([
                "-Command",
                "Get-Process -Id $PID | Select-Object -ExpandProperty WorkingSet64"
            ])
            .output()
            .map_err(|e| StoryWeaverError::internal(format!("Failed to get memory usage: {}", e)))?;
        
        if output.status.success() {
            let memory_bytes = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u64>()
                .map_err(|e| StoryWeaverError::internal(format!("Failed to parse memory usage: {}", e)))?;
            
            Ok(memory_bytes as f64 / 1024.0 / 1024.0) // Convert to MB
        } else {
            Err(StoryWeaverError::internal("Failed to execute memory usage command".to_string()))
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    async fn get_unix_memory_usage(&self) -> Result<f64> {
        use std::fs;
        
        let status = fs::read_to_string("/proc/self/status")
            .map_err(|e| StoryWeaverError::internal(format!("Failed to read /proc/self/status: {}", e)))?;
        
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let memory_kb = parts[1].parse::<f64>()
                        .map_err(|e| StoryWeaverError::internal(format!("Failed to parse memory usage: {}", e)))?;
                    return Ok(memory_kb / 1024.0); // Convert KB to MB
                }
            }
        }
        
        Err(StoryWeaverError::internal("Could not find memory usage in /proc/self/status".to_string()))
    }
    
    /// Record memory usage for a specific component
    pub fn record_component_usage(&self, component: &str, usage_mb: f64) {
        if let Ok(mut usage_map) = self.component_usage.lock() {
            usage_map.insert(component.to_string(), usage_mb);
        }
    }
    
    /// Get memory usage breakdown by component
    pub fn get_component_breakdown(&self) -> HashMap<String, f64> {
        self.component_usage.lock()
            .map(|usage_map| usage_map.clone())
            .unwrap_or_default()
    }
    
    /// Take a memory snapshot for performance analysis
    pub async fn take_snapshot(&self, context: &str) -> Result<MemorySnapshot> {
        let total_usage = self.get_current_usage().await?;
        let component_breakdown = self.get_component_breakdown();
        
        Ok(MemorySnapshot {
            timestamp: chrono::Utc::now(),
            context: context.to_string(),
            total_usage_mb: total_usage,
            component_breakdown,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: String,
    pub total_usage_mb: f64,
    pub component_breakdown: HashMap<String, f64>,
}

/// Initialize memory optimization system
pub fn initialize_memory_optimization(config: Option<MemoryConfig>) -> MemoryOptimizedProcessor {
    let config = config.unwrap_or_default();
    
    info!(
        "Initializing memory optimization with config: max_memory={}MB, chunk_size={} words",
        config.max_memory_usage_mb, config.chunk_size
    );
    
    MemoryOptimizedProcessor::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_document_chunking() {
        let processor = MemoryOptimizedProcessor::new(MemoryConfig {
            chunk_size: 5, // Small chunk size for testing
            ..Default::default()
        });
        
        let content = "This is a test document with more than five words to test chunking functionality properly.";
        let result = processor.process_document("test_doc", content).await
            .expect("Document processing should succeed in test");
        
        assert!(result.chunks.len() > 1);
        assert_eq!(result.document_id, "test_doc");
        assert!(result.total_words > 5);
    }
    
    #[tokio::test]
    async fn test_document_caching() {
        let processor = MemoryOptimizedProcessor::new(MemoryConfig::default());
        
        let content = "This is a test document for caching.";
        
        // First processing - should be a cache miss
        let result1 = processor.process_document("test_doc", content).await
            .expect("First document processing should succeed in test");
        assert_eq!(result1.cache_misses, 1);
        assert_eq!(result1.cache_hits, 0);
        
        // Second processing - should be a cache hit
        let result2 = processor.process_document("test_doc", content).await
            .expect("Second document processing should succeed in test");
        assert_eq!(result2.cache_hits, 1);
        assert_eq!(result2.cache_misses, 0);
    }
    
    #[tokio::test]
    async fn test_embedding_cache() {
        let processor = MemoryOptimizedProcessor::new(MemoryConfig::default());
        
        let embedding = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let content_hash = "test_hash";
        
        // Cache the embedding
        processor.cache_embedding(content_hash, embedding.clone(), "test_model").await
            .expect("Embedding caching should succeed in test");
        
        // Retrieve the embedding
        let cached = processor.get_cached_embedding(content_hash).await
            .expect("Embedding retrieval should succeed in test");
        assert_eq!(cached.embedding, embedding);
        assert_eq!(cached.model, "test_model");
        assert_eq!(cached.access_count, 2); // 1 for caching, 1 for retrieval
    }
}
