//! Streaming memory optimization for AI operations in StoryWeaver
//! Implements memory-efficient streaming with backpressure and resource management

use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, OnceLock};
use tokio::sync::{RwLock, Semaphore};
use tokio::time::{Duration, Instant};

/// Configuration for streaming optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingConfig {
    pub max_concurrent_streams: usize,
    pub buffer_size: usize,
    pub chunk_size: usize,
    pub memory_limit_mb: usize,
    pub backpressure_threshold: f64,
    pub cleanup_interval_seconds: u64,
    pub max_stream_duration_seconds: u64,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            max_concurrent_streams: 10,
            buffer_size: 1024,
            chunk_size: 256,
            memory_limit_mb: 128,
            backpressure_threshold: 0.8,
            cleanup_interval_seconds: 30,
            max_stream_duration_seconds: 300, // 5 minutes
        }
    }
}

/// Stream buffer for managing memory-efficient streaming
#[derive(Debug)]
pub struct StreamBuffer {
    pub stream_id: String,
    pub buffer: VecDeque<String>,
    pub total_size: usize,
    pub created_at: Instant,
    pub last_activity: Instant,
    pub is_complete: bool,
    pub consumer_position: usize,
}

impl StreamBuffer {
    pub fn new(stream_id: String, buffer_size: usize) -> Self {
        let now = Instant::now();
        Self {
            stream_id,
            buffer: VecDeque::with_capacity(buffer_size),
            total_size: 0,
            created_at: now,
            last_activity: now,
            is_complete: false,
            consumer_position: 0,
        }
    }

    pub fn push(&mut self, chunk: String) -> Result<()> {
        self.total_size += chunk.len();
        self.buffer.push_back(chunk);
        self.last_activity = Instant::now();
        Ok(())
    }

    pub fn pop(&mut self) -> Option<String> {
        if let Some(chunk) = self.buffer.pop_front() {
            self.total_size = self.total_size.saturating_sub(chunk.len());
            self.consumer_position += 1;
            self.last_activity = Instant::now();
            Some(chunk)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&String> {
        self.buffer.front()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn memory_usage(&self) -> usize {
        self.total_size
    }

    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }

    pub fn idle_time(&self) -> Duration {
        self.last_activity.elapsed()
    }
}

/// Memory-optimized streaming manager
pub struct StreamingOptimizer {
    config: StreamingConfig,
    streams: Arc<RwLock<std::collections::HashMap<String, StreamBuffer>>>,
    semaphore: Arc<Semaphore>,
    memory_usage: Arc<RwLock<usize>>,
    stats: Arc<RwLock<StreamingStats>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StreamingStats {
    pub active_streams: usize,
    pub total_streams_created: u64,
    pub total_streams_completed: u64,
    pub total_memory_usage: usize,
    pub peak_memory_usage: usize,
    pub backpressure_events: u64,
    pub cleanup_events: u64,
    pub average_stream_duration_ms: f64,
}

impl StreamingOptimizer {
    pub fn new(config: StreamingConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent_streams));
        
        Self {
            config,
            streams: Arc::new(RwLock::new(std::collections::HashMap::new())),
            semaphore,
            memory_usage: Arc::new(RwLock::new(0)),
            stats: Arc::new(RwLock::new(StreamingStats::default())),
        }
    }

    /// Create a new stream with memory management
    pub async fn create_stream(&self, stream_id: String) -> Result<()> {
        // Acquire semaphore permit for concurrency control
        let _permit = self.semaphore.acquire().await
            .map_err(|e| StoryWeaverError::system(format!("Failed to acquire stream permit: {}", e)))?;

        // Check memory limits
        let current_memory = *self.memory_usage.read().await;
        let memory_limit = self.config.memory_limit_mb * 1024 * 1024;
        
        if current_memory > (memory_limit as f64 * self.config.backpressure_threshold) as usize {
            self.trigger_backpressure().await?;
        }

        let mut streams = self.streams.write().await;
        let buffer = StreamBuffer::new(stream_id.clone(), self.config.buffer_size);
        streams.insert(stream_id, buffer);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.active_streams = streams.len();
        stats.total_streams_created += 1;

        Ok(())
    }

    /// Add data to a stream with memory optimization
    pub async fn push_to_stream(&self, stream_id: &str, data: String) -> Result<()> {
        let mut streams = self.streams.write().await;
        
        if let Some(stream) = streams.get_mut(stream_id) {
            // Check if buffer is full
            if stream.len() >= self.config.buffer_size {
                return Err(StoryWeaverError::system("Stream buffer full - backpressure applied"));
            }

            let data_size = data.len();
            stream.push(data)?;

            // Update memory usage
            let mut memory_usage = self.memory_usage.write().await;
            *memory_usage += data_size;

            // Update peak memory usage
            let mut stats = self.stats.write().await;
            if *memory_usage > stats.peak_memory_usage {
                stats.peak_memory_usage = *memory_usage;
            }
            stats.total_memory_usage = *memory_usage;

            Ok(())
        } else {
            Err(StoryWeaverError::not_found("Stream", stream_id))
        }
    }

    /// Consume data from a stream
    pub async fn consume_from_stream(&self, stream_id: &str) -> Result<Option<String>> {
        let mut streams = self.streams.write().await;
        
        if let Some(stream) = streams.get_mut(stream_id) {
            if let Some(data) = stream.pop() {
                // Update memory usage
                let mut memory_usage = self.memory_usage.write().await;
                *memory_usage = memory_usage.saturating_sub(data.len());

                Ok(Some(data))
            } else {
                Ok(None)
            }
        } else {
            Err(StoryWeaverError::not_found("Stream", stream_id))
        }
    }

    /// Mark a stream as complete
    pub async fn complete_stream(&self, stream_id: &str) -> Result<()> {
        let mut streams = self.streams.write().await;
        
        if let Some(stream) = streams.get_mut(stream_id) {
            stream.is_complete = true;
            
            // Update stats
            let mut stats = self.stats.write().await;
            stats.total_streams_completed += 1;
            
            let duration_ms = stream.age().as_millis() as f64;
            stats.average_stream_duration_ms = 
                (stats.average_stream_duration_ms * (stats.total_streams_completed - 1) as f64 + duration_ms) 
                / stats.total_streams_completed as f64;

            Ok(())
        } else {
            Err(StoryWeaverError::not_found("Stream", stream_id))
        }
    }

    /// Check if a stream is complete and empty
    pub async fn is_stream_finished(&self, stream_id: &str) -> Result<bool> {
        let streams = self.streams.read().await;
        
        if let Some(stream) = streams.get(stream_id) {
            Ok(stream.is_complete && stream.is_empty())
        } else {
            Err(StoryWeaverError::not_found("Stream", stream_id))
        }
    }

    /// Get stream information
    pub async fn get_stream_info(&self, stream_id: &str) -> Result<StreamInfo> {
        let streams = self.streams.read().await;
        
        if let Some(stream) = streams.get(stream_id) {
            Ok(StreamInfo {
                stream_id: stream.stream_id.clone(),
                buffer_size: stream.len(),
                memory_usage: stream.memory_usage(),
                is_complete: stream.is_complete,
                age_seconds: stream.age().as_secs(),
                idle_seconds: stream.idle_time().as_secs(),
            })
        } else {
            Err(StoryWeaverError::not_found("Stream", stream_id))
        }
    }

    /// Trigger backpressure by cleaning up old streams
    async fn trigger_backpressure(&self) -> Result<()> {
        let mut stats = self.stats.write().await;
        stats.backpressure_events += 1;
        drop(stats);

        self.cleanup_idle_streams().await?;
        Ok(())
    }

    /// Clean up idle or expired streams
    pub async fn cleanup_idle_streams(&self) -> Result<usize> {
        let mut streams = self.streams.write().await;
        let max_duration = Duration::from_secs(self.config.max_stream_duration_seconds);
        let max_idle = Duration::from_secs(self.config.cleanup_interval_seconds * 2);
        
        let mut removed_count = 0;
        let mut memory_freed = 0;

        streams.retain(|_, stream| {
            let should_remove = (stream.is_complete && stream.is_empty()) ||
                               stream.age() > max_duration ||
                               stream.idle_time() > max_idle;
            
            if should_remove {
                memory_freed += stream.memory_usage();
                removed_count += 1;
                false
            } else {
                true
            }
        });

        // Update memory usage
        let mut memory_usage = self.memory_usage.write().await;
        *memory_usage = memory_usage.saturating_sub(memory_freed);

        // Update stats
        let mut stats = self.stats.write().await;
        stats.active_streams = streams.len();
        stats.cleanup_events += 1;
        stats.total_memory_usage = *memory_usage;

        Ok(removed_count)
    }

    /// Get comprehensive streaming statistics
    pub async fn get_stats(&self) -> StreamingStats {
        let stats = self.stats.read().await;
        let mut current_stats = stats.clone();
        
        let streams = self.streams.read().await;
        current_stats.active_streams = streams.len();
        current_stats.total_memory_usage = *self.memory_usage.read().await;
        
        current_stats
    }

    /// Force cleanup of all streams
    pub async fn cleanup_all_streams(&self) -> Result<()> {
        let mut streams = self.streams.write().await;
        streams.clear();
        
        let mut memory_usage = self.memory_usage.write().await;
        *memory_usage = 0;
        
        let mut stats = self.stats.write().await;
        stats.active_streams = 0;
        stats.total_memory_usage = 0;
        
        Ok(())
    }

    /// Get memory pressure level (0.0 to 1.0)
    pub async fn get_memory_pressure(&self) -> f64 {
        let current_memory = *self.memory_usage.read().await;
        let memory_limit = self.config.memory_limit_mb * 1024 * 1024;
        
        if memory_limit == 0 {
            0.0
        } else {
            (current_memory as f64) / (memory_limit as f64)
        }
    }
}

/// Stream information for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub stream_id: String,
    pub buffer_size: usize,
    pub memory_usage: usize,
    pub is_complete: bool,
    pub age_seconds: u64,
    pub idle_seconds: u64,
}

/// Global streaming optimizer instance
static STREAMING_OPTIMIZER: OnceLock<Arc<StreamingOptimizer>> = OnceLock::new();

/// Initialize the global streaming optimizer
pub fn init_streaming_optimizer(config: StreamingConfig) -> Result<()> {
    STREAMING_OPTIMIZER.set(Arc::new(StreamingOptimizer::new(config)))
        .map_err(|_| StoryWeaverError::system("Streaming optimizer already initialized"))
}

/// Get the global streaming optimizer
pub fn get_streaming_optimizer() -> Result<Arc<StreamingOptimizer>> {
    STREAMING_OPTIMIZER.get().cloned().ok_or_else(|| {
        StoryWeaverError::system("Streaming optimizer not initialized")
    })
}

/// Background task to periodically clean up streams
pub async fn start_streaming_cleanup_task() {
    let optimizer = match get_streaming_optimizer() {
        Ok(optimizer) => optimizer,
        Err(_) => return,
    };
    
    let cleanup_interval = Duration::from_secs(optimizer.config.cleanup_interval_seconds);
    let mut interval = tokio::time::interval(cleanup_interval);
    
    loop {
        interval.tick().await;
        
        if let Err(e) = optimizer.cleanup_idle_streams().await {
            tracing::warn!("Failed to cleanup streams: {}", e);
        }
        
        // Log memory pressure
        let pressure = optimizer.get_memory_pressure().await;
        if pressure > 0.8 {
            tracing::warn!("High memory pressure: {:.1}%", pressure * 100.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stream_creation() {
        let config = StreamingConfig::default();
        let optimizer = StreamingOptimizer::new(config);
        
        let stream_id = "test_stream".to_string();
        optimizer.create_stream(stream_id.clone()).await.unwrap();
        
        let info = optimizer.get_stream_info(&stream_id).await.unwrap();
        assert_eq!(info.stream_id, stream_id);
        assert_eq!(info.buffer_size, 0);
        assert!(!info.is_complete);
    }

    #[tokio::test]
    async fn test_stream_data_flow() {
        let config = StreamingConfig::default();
        let optimizer = StreamingOptimizer::new(config);
        
        let stream_id = "test_stream".to_string();
        optimizer.create_stream(stream_id.clone()).await.unwrap();
        
        // Push data
        optimizer.push_to_stream(&stream_id, "chunk1".to_string()).await.unwrap();
        optimizer.push_to_stream(&stream_id, "chunk2".to_string()).await.unwrap();
        
        // Consume data
        let chunk1 = optimizer.consume_from_stream(&stream_id).await.unwrap();
        assert_eq!(chunk1, Some("chunk1".to_string()));
        
        let chunk2 = optimizer.consume_from_stream(&stream_id).await.unwrap();
        assert_eq!(chunk2, Some("chunk2".to_string()));
        
        // Should be empty now
        let empty = optimizer.consume_from_stream(&stream_id).await.unwrap();
        assert_eq!(empty, None);
    }

    #[tokio::test]
    async fn test_stream_completion() {
        let config = StreamingConfig::default();
        let optimizer = StreamingOptimizer::new(config);
        
        let stream_id = "test_stream".to_string();
        optimizer.create_stream(stream_id.clone()).await.unwrap();
        
        // Stream should not be finished initially
        assert!(!optimizer.is_stream_finished(&stream_id).await.unwrap());
        
        // Complete the stream
        optimizer.complete_stream(&stream_id).await.unwrap();
        
        // Should be finished now (complete and empty)
        assert!(optimizer.is_stream_finished(&stream_id).await.unwrap());
    }

    #[tokio::test]
    async fn test_memory_tracking() {
        let config = StreamingConfig::default();
        let optimizer = StreamingOptimizer::new(config);
        
        let stream_id = "test_stream".to_string();
        optimizer.create_stream(stream_id.clone()).await.unwrap();
        
        let initial_stats = optimizer.get_stats().await;
        assert_eq!(initial_stats.total_memory_usage, 0);
        
        // Push some data
        let test_data = "x".repeat(1000); // 1KB
        optimizer.push_to_stream(&stream_id, test_data).await.unwrap();
        
        let stats_after_push = optimizer.get_stats().await;
        assert_eq!(stats_after_push.total_memory_usage, 1000);
        
        // Consume the data
        optimizer.consume_from_stream(&stream_id).await.unwrap();
        
        let stats_after_consume = optimizer.get_stats().await;
        assert_eq!(stats_after_consume.total_memory_usage, 0);
    }
}
