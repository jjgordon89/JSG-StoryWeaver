//! AI response caching system for StoryWeaver
//! Implements intelligent caching of AI responses to reduce API calls and improve performance

use crate::error::{Result, StoryWeaverError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use sha2::{Digest, Sha256};

/// AI response cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub response: String,
    pub model: String,
    pub provider: String,
    pub token_count: u32,
    pub cost_estimate: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub ttl_seconds: u64,
}

/// Cache key components for generating consistent cache keys
#[derive(Debug, Clone, Serialize)]
pub struct CacheKey {
    pub prompt: String,
    pub model: String,
    pub provider: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub context_hash: Option<String>, // Hash of relevant context (story bible, etc.)
}

impl CacheKey {
    /// Generate a consistent hash for this cache key
    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(self).unwrap_or_default().as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

/// AI response cache with intelligent eviction and persistence
pub struct AIResponseCache {
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    max_size: usize,
    default_ttl_seconds: u64,
    hit_count: Arc<RwLock<u64>>,
    miss_count: Arc<RwLock<u64>>,
}

impl AIResponseCache {
    pub fn new(max_size: usize, default_ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
            default_ttl_seconds,
            hit_count: Arc::new(RwLock::new(0)),
            miss_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Get a cached response if available and not expired
    pub async fn get(&self, key: &CacheKey) -> Option<CacheEntry> {
        let cache_key = key.hash();
        let mut cache = self.cache.write().await;
        
        if let Some(entry) = cache.get_mut(&cache_key) {
            // Check if entry is still valid
            let now = chrono::Utc::now();
            let age_seconds = (now - entry.created_at).num_seconds() as u64;
            
            if age_seconds < entry.ttl_seconds {
                // Update access statistics
                entry.access_count += 1;
                entry.last_accessed = now;
                
                // Increment hit count
                let mut hit_count = self.hit_count.write().await;
                *hit_count += 1;
                
                return Some(entry.clone());
            } else {
                // Remove expired entry
                cache.remove(&cache_key);
            }
        }
        
        // Increment miss count
        let mut miss_count = self.miss_count.write().await;
        *miss_count += 1;
        
        None
    }

    /// Store a response in the cache
    pub async fn set(&self, key: &CacheKey, response: String, model: String, provider: String, 
                     token_count: u32, cost_estimate: f64, custom_ttl: Option<u64>) {
        let cache_key = key.hash();
        let mut cache = self.cache.write().await;
        
        // Remove oldest entries if cache is full
        if cache.len() >= self.max_size {
            self.evict_oldest(&mut cache).await;
        }
        
        let now = chrono::Utc::now();
        let entry = CacheEntry {
            response,
            model,
            provider,
            token_count,
            cost_estimate,
            created_at: now,
            access_count: 1,
            last_accessed: now,
            ttl_seconds: custom_ttl.unwrap_or(self.default_ttl_seconds),
        };
        
        cache.insert(cache_key, entry);
    }

    /// Evict the oldest entry based on last access time and access count
    async fn evict_oldest(&self, cache: &mut HashMap<String, CacheEntry>) {
        if cache.is_empty() {
            return;
        }
        
        // Find the entry with the lowest score (oldest + least accessed)
        let mut lowest_score = f64::MAX;
        let mut key_to_remove = None;
        let now = chrono::Utc::now();
        
        for (key, entry) in cache.iter() {
            let age_hours = (now - entry.last_accessed).num_hours() as f64;
            let access_weight = 1.0 / (entry.access_count as f64 + 1.0);
            let score = age_hours * access_weight;
            
            if score < lowest_score {
                lowest_score = score;
                key_to_remove = Some(key.clone());
            }
        }
        
        if let Some(key) = key_to_remove {
            cache.remove(&key);
        }
    }

    /// Clear expired entries
    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        let now = chrono::Utc::now();
        
        cache.retain(|_, entry| {
            let age_seconds = (now - entry.created_at).num_seconds() as u64;
            age_seconds < entry.ttl_seconds
        });
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let hit_count = *self.hit_count.read().await;
        let miss_count = *self.miss_count.read().await;
        
        let total_requests = hit_count + miss_count;
        let hit_rate = if total_requests > 0 {
            hit_count as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let mut total_cost_saved = 0.0;
        let mut total_tokens_saved = 0;
        
        for entry in cache.values() {
            if entry.access_count > 1 {
                let saves = entry.access_count - 1;
                total_cost_saved += entry.cost_estimate * saves as f64;
                total_tokens_saved += entry.token_count * saves as u32;
            }
        }
        
        CacheStats {
            size: cache.len(),
            max_size: self.max_size,
            hit_count,
            miss_count,
            hit_rate,
            total_cost_saved,
            total_tokens_saved,
        }
    }

    /// Clear all cache entries
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        
        let mut hit_count = self.hit_count.write().await;
        let mut miss_count = self.miss_count.write().await;
        *hit_count = 0;
        *miss_count = 0;
    }

    /// Get entries that match a pattern (for debugging/admin)
    pub async fn get_entries_by_pattern(&self, pattern: &str) -> Vec<(String, CacheEntry)> {
        let cache = self.cache.read().await;
        cache.iter()
            .filter(|(key, _)| key.contains(pattern))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    /// Remove entries older than specified duration
    pub async fn remove_older_than(&self, hours: i64) {
        let mut cache = self.cache.write().await;
        let cutoff = chrono::Utc::now() - chrono::Duration::hours(hours);
        
        cache.retain(|_, entry| entry.created_at > cutoff);
    }

    /// Preload cache with common responses (for warming)
    pub async fn preload_common_responses(&self, responses: Vec<(CacheKey, CacheEntry)>) {
        let mut cache = self.cache.write().await;
        
        for (key, entry) in responses {
            let cache_key = key.hash();
            cache.insert(cache_key, entry);
        }
    }
}

/// Cache statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct CacheStats {
    pub size: usize,
    pub max_size: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub total_cost_saved: f64,
    pub total_tokens_saved: u32,
}

/// Global cache instance
static AI_CACHE: OnceLock<Arc<AIResponseCache>> = OnceLock::new();

/// Initialize the global AI cache
pub fn init_ai_cache(max_size: usize, default_ttl_seconds: u64) -> Result<()> {
    AI_CACHE.set(Arc::new(AIResponseCache::new(max_size, default_ttl_seconds)))
        .map_err(|_| StoryWeaverError::ai_request("Cache", 500, "AI cache already initialized"))
}

/// Get the global AI cache instance
pub fn get_ai_cache() -> Result<Arc<AIResponseCache>> {
    AI_CACHE.get().cloned().ok_or_else(|| {
        StoryWeaverError::ai_request("Cache", 500, "AI cache not initialized")
    })
}

/// Helper function to create cache key from common parameters
pub fn create_cache_key(
    prompt: &str,
    model: &str,
    provider: &str,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    context_data: Option<&str>,
) -> CacheKey {
    let context_hash = context_data.map(|data| {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    });
    
    CacheKey {
        prompt: prompt.to_string(),
        model: model.to_string(),
        provider: provider.to_string(),
        temperature,
        max_tokens,
        context_hash,
    }
}

/// Background task to periodically clean up expired cache entries
pub async fn start_cache_cleanup_task() {
    let cache = match get_ai_cache() {
        Ok(cache) => cache,
        Err(_) => return,
    };
    
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(300)); // 5 minutes
    
    loop {
        interval.tick().await;
        cache.cleanup_expired().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_basic_operations() {
        let cache = AIResponseCache::new(10, 3600);
        
        let key = CacheKey {
            prompt: "Test prompt".to_string(),
            model: "gpt-4".to_string(),
            provider: "openai".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(1000),
            context_hash: None,
        };
        
        // Test miss
        assert!(cache.get(&key).await.is_none());
        
        // Test set and hit
        cache.set(&key, "Test response".to_string(), "gpt-4".to_string(), 
                 "openai".to_string(), 100, 0.01, None).await;
        
        let entry = cache.get(&key).await;
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().response, "Test response");
        
        // Test stats
        let stats = cache.get_stats().await;
        assert_eq!(stats.hit_count, 1);
        assert_eq!(stats.miss_count, 1);
        assert_eq!(stats.size, 1);
    }

    #[tokio::test]
    async fn test_cache_key_hashing() {
        let key1 = CacheKey {
            prompt: "Test".to_string(),
            model: "gpt-4".to_string(),
            provider: "openai".to_string(),
            temperature: Some(0.7),
            max_tokens: Some(1000),
            context_hash: None,
        };
        
        let key2 = key1.clone();
        let mut key3 = key1.clone();
        key3.prompt = "Different".to_string();
        
        assert_eq!(key1.hash(), key2.hash());
        assert_ne!(key1.hash(), key3.hash());
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = AIResponseCache::new(10, 1); // 1 second TTL
        
        let key = CacheKey {
            prompt: "Test".to_string(),
            model: "gpt-4".to_string(),
            provider: "openai".to_string(),
            temperature: None,
            max_tokens: None,
            context_hash: None,
        };
        
        cache.set(&key, "Response".to_string(), "gpt-4".to_string(), 
                 "openai".to_string(), 100, 0.01, Some(1)).await;
        
        // Should be available immediately
        assert!(cache.get(&key).await.is_some());
        
        // Wait for expiration
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // Should be expired
        assert!(cache.get(&key).await.is_none());
    }
}
