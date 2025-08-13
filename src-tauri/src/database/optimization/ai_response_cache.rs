//! AI Response Cache for intelligent caching and retrieval of AI responses
//! Implements similarity-based matching and efficient storage using DashMap

use crate::error::{Result};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use sha2::{Sha256, Digest};

/// Intelligent cache for AI responses with similarity-based retrieval
pub struct AIResponseCache {
    cache: Arc<DashMap<String, CachedAIResponse>>,
    similarity_index: Arc<RwLock<SimilarityIndex>>,
    config: CacheConfig,
    stats: Arc<RwLock<CacheStatistics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedAIResponse {
    pub request_hash: String,
    pub request_text: String,
    pub response_text: String,
    pub provider: String,
    pub model: String,
    pub tokens_used: Option<u32>,
    pub cost_estimate: Option<f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u64,
    pub similarity_score: f64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_entries: usize,
    pub similarity_threshold: f64,
    pub ttl_hours: u64,
    pub enable_fuzzy_matching: bool,
    pub max_request_length: usize,
    pub compression_enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 10000,
            similarity_threshold: 0.85,
            ttl_hours: 24 * 7, // 1 week
            enable_fuzzy_matching: true,
            max_request_length: 10000,
            compression_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub text: String,
    pub provider: String,
    pub model: String,
    pub parameters: HashMap<String, String>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub text: String,
    pub tokens_used: Option<u32>,
    pub cost_estimate: Option<f64>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatistics {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub similarity_matches: u64,
    pub exact_matches: u64,
    pub total_tokens_saved: u64,
    pub total_cost_saved: f64,
    pub average_similarity_score: f64,
}

impl Default for CacheStatistics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
            similarity_matches: 0,
            exact_matches: 0,
            total_tokens_saved: 0,
            total_cost_saved: 0.0,
            average_similarity_score: 0.0,
        }
    }
}

/// Similarity index for efficient similarity-based retrieval
pub struct SimilarityIndex {
    entries: Vec<IndexEntry>,
    word_index: HashMap<String, Vec<usize>>,
}

#[derive(Debug, Clone)]
struct IndexEntry {
    request_hash: String,
    words: Vec<String>,
    word_count: usize,
}

impl SimilarityIndex {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            word_index: HashMap::new(),
        }
    }
    
    /// Add a request to the similarity index
    pub fn add_request(&mut self, request_hash: &str, request_text: &str) {
        let words = self.tokenize_and_normalize(request_text);
        let word_count = words.len();
        
        let entry = IndexEntry {
            request_hash: request_hash.to_string(),
            words: words.clone(),
            word_count,
        };
        
        let entry_index = self.entries.len();
        self.entries.push(entry);
        
        // Update word index
        for word in words {
            self.word_index
                .entry(word)
                .or_insert_with(Vec::new)
                .push(entry_index);
        }
    }
    
    /// Find similar requests based on text similarity
    pub fn find_similar(&self, request_text: &str, threshold: f64) -> Vec<(String, f64)> {
        let query_words = self.tokenize_and_normalize(request_text);
        let mut candidates = HashMap::new();
        
        // Find candidate entries that share words with the query
        for word in &query_words {
            if let Some(entry_indices) = self.word_index.get(word) {
                for &entry_index in entry_indices {
                    *candidates.entry(entry_index).or_insert(0) += 1;
                }
            }
        }
        
        let mut results = Vec::new();
        
        // Calculate similarity scores for candidates
        for (entry_index, shared_words) in candidates {
            if let Some(entry) = self.entries.get(entry_index) {
                let similarity = self.calculate_jaccard_similarity(
                    &query_words,
                    &entry.words,
                    shared_words,
                );
                
                if similarity >= threshold {
                    results.push((entry.request_hash.clone(), similarity));
                }
            }
        }
        
        // Sort by similarity score (descending)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        results
    }
    
    /// Tokenize and normalize text for similarity comparison
    fn tokenize_and_normalize(&self, text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .filter(|word| word.len() > 2) // Filter out very short words
            .map(|word| {
                // Remove punctuation
                word.chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .filter(|word| !word.is_empty())
            .collect()
    }
    
    /// Calculate Jaccard similarity between two sets of words
    fn calculate_jaccard_similarity(
        &self,
        words1: &[String],
        words2: &[String],
        intersection_size: usize,
    ) -> f64 {
        let union_size = words1.len() + words2.len() - intersection_size;
        if union_size == 0 {
            return 0.0;
        }
        intersection_size as f64 / union_size as f64
    }
    
    /// Remove an entry from the index
    pub fn remove_entry(&mut self, request_hash: &str) {
        if let Some(pos) = self.entries.iter().position(|e| e.request_hash == request_hash) {
            let entry = self.entries.remove(pos);
            
            // Update word index
            for word in &entry.words {
                if let Some(indices) = self.word_index.get_mut(word) {
                    indices.retain(|&i| i != pos);
                    if indices.is_empty() {
                        self.word_index.remove(word);
                    } else {
                        // Update indices after removal
                        for index in indices.iter_mut() {
                            if *index > pos {
                                *index -= 1;
                            }
                        }
                    }
                }
            }
            
            // Update all indices in word_index
            for indices in self.word_index.values_mut() {
                for index in indices.iter_mut() {
                    if *index > pos {
                        *index -= 1;
                    }
                }
            }
        }
    }
    
    /// Get the number of entries in the index
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl AIResponseCache {
    /// Create a new AI response cache
    pub fn new(config: CacheConfig) -> Self {
        let cache = Self {
            cache: Arc::new(DashMap::new()),
            similarity_index: Arc::new(RwLock::new(SimilarityIndex::new())),
            config,
            stats: Arc::new(RwLock::new(CacheStatistics::default())),
        };
        
        // Start background sweeper task
        cache.start_background_sweeper();
        
        cache
    }
    
    /// Start background sweeper for time-based cleanup
    fn start_background_sweeper(&self) {
        let cache_ref = Arc::new(DashMap::clone(&self.cache));
        let similarity_index_ref = Arc::clone(&self.similarity_index);
        let ttl_hours = self.config.ttl_hours;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Run every hour
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::sweep_expired_entries(
                    &cache_ref,
                    &similarity_index_ref,
                    ttl_hours,
                ).await {
                    tracing::warn!("Background cache sweep failed: {}", e);
                }
            }
        });
    }
    
    /// Sweep expired entries from cache
    async fn sweep_expired_entries(
        cache: &Arc<DashMap<String, CachedAIResponse>>,
        similarity_index: &Arc<RwLock<SimilarityIndex>>,
        ttl_hours: u64,
    ) -> Result<usize> {
        let now = chrono::Utc::now();
        let ttl_duration = chrono::Duration::hours(ttl_hours as i64);
        let mut expired_keys = Vec::new();
        
        // Collect expired entries
        for entry in cache.iter() {
            let (key, cached_response) = (entry.key(), entry.value());
            
            // Check if entry has expired based on last access time
            let age = now.signed_duration_since(cached_response.last_accessed);
            if age > ttl_duration {
                expired_keys.push(key.clone());
            }
        }
        
        // Remove expired entries
        let mut removed_count = 0;
        for key in &expired_keys {
            if cache.remove(key).is_some() {
                removed_count += 1;
                
                // Remove from similarity index
                let mut index = similarity_index.write().await;
                index.remove_entry(key);
            }
        }
        
        if removed_count > 0 {
            tracing::info!("Background sweep removed {} expired cache entries", removed_count);
        }
        
        Ok(removed_count)
    }
    
    /// Generate a cache key for an AI request
    pub fn generate_cache_key(&self, request: &AIRequest) -> String {
        let mut hasher = Sha256::new();
        hasher.update(request.text.as_bytes());
        hasher.update(request.provider.as_bytes());
        hasher.update(request.model.as_bytes());
        
        // Include parameters in hash
        let mut params: Vec<_> = request.parameters.iter().collect();
        params.sort_by_key(|&(k, _)| k);
        for (key, value) in params {
            hasher.update(key.as_bytes());
            hasher.update(value.as_bytes());
        }
        
        if let Some(context) = &request.context {
            hasher.update(context.as_bytes());
        }
        
        format!("{:x}", hasher.finalize())
    }
    
    /// Cache an AI response
    pub async fn cache_response(
        &self,
        request: &AIRequest,
        response: &AIResponse,
    ) -> Result<()> {
        // Check if request is too long
        if request.text.len() > self.config.max_request_length {
            debug!("Request too long to cache: {} characters", request.text.len());
            return Ok(());
        }
        
        let request_hash = self.generate_cache_key(request);
        
        let cached_response = CachedAIResponse {
            request_hash: request_hash.clone(),
            request_text: request.text.clone(),
            response_text: response.text.clone(),
            provider: request.provider.clone(),
            model: request.model.clone(),
            tokens_used: response.tokens_used,
            cost_estimate: response.cost_estimate,
            created_at: chrono::Utc::now(),
            last_accessed: chrono::Utc::now(),
            access_count: 1,
            similarity_score: 1.0, // Exact match
            metadata: response.metadata.clone(),
        };
        
        // Add to cache
        self.cache.insert(request_hash.clone(), cached_response);
        
        // Add to similarity index
        {
            let mut index = self.similarity_index.write().await;
            index.add_request(&request_hash, &request.text);
        }
        
        // Cleanup if cache is too large
        if self.cache.len() > self.config.max_entries {
            self.cleanup_cache().await?;
        }
        
        debug!("Cached AI response for request hash: {}", request_hash);
        Ok(())
    }
    
    /// Retrieve a cached response or find similar ones
    pub async fn get_response(&self, request: &AIRequest) -> Result<Option<CachedAIResponse>> {
        let request_hash = self.generate_cache_key(request);
        
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }
        
        // Try exact match first
        if let Some(mut cached_response) = self.cache.get_mut(&request_hash) {
            cached_response.last_accessed = chrono::Utc::now();
            cached_response.access_count += 1;
            
            // Update statistics
            {
                let mut stats = self.stats.write().await;
                stats.cache_hits += 1;
                stats.exact_matches += 1;
                
                if let Some(tokens) = cached_response.tokens_used {
                    stats.total_tokens_saved += tokens as u64;
                }
                
                if let Some(cost) = cached_response.cost_estimate {
                    stats.total_cost_saved += cost;
                }
            }
            
            debug!("Found exact cache match for request hash: {}", request_hash);
            return Ok(Some(cached_response.clone()));
        }
        
        // Try similarity-based matching if enabled
        if self.config.enable_fuzzy_matching {
            if let Some(similar_response) = self.find_similar_response(request).await? {
                // Update statistics
                {
                    let mut stats = self.stats.write().await;
                    stats.cache_hits += 1;
                    stats.similarity_matches += 1;
                    
                    if let Some(tokens) = similar_response.tokens_used {
                        stats.total_tokens_saved += tokens as u64;
                    }
                    
                    if let Some(cost) = similar_response.cost_estimate {
                        stats.total_cost_saved += cost;
                    }
                }
                
                debug!(
                    "Found similar cache match with similarity: {:.2}",
                    similar_response.similarity_score
                );
                return Ok(Some(similar_response));
            }
        }
        
        // No match found
        {
            let mut stats = self.stats.write().await;
            stats.cache_misses += 1;
        }
        
        debug!("No cache match found for request");
        Ok(None)
    }
    
    /// Find similar cached responses
    async fn find_similar_response(&self, request: &AIRequest) -> Result<Option<CachedAIResponse>> {
        let index = self.similarity_index.read().await;
        let similar_requests = index.find_similar(&request.text, self.config.similarity_threshold);
        
        for (request_hash, similarity_score) in similar_requests {
            if let Some(mut cached_response) = self.cache.get_mut(&request_hash) {
                // Check if provider and model match (optional strict matching)
                if cached_response.provider == request.provider && cached_response.model == request.model {
                    cached_response.last_accessed = chrono::Utc::now();
                    cached_response.access_count += 1;
                    cached_response.similarity_score = similarity_score;
                    
                    return Ok(Some(cached_response.clone()));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Clean up expired and least recently used cache entries
    async fn cleanup_cache(&self) -> Result<()> {
        let now = chrono::Utc::now();
        let ttl_duration = chrono::Duration::hours(self.config.ttl_hours as i64);
        
        let mut expired_keys = Vec::new();
        let mut lru_candidates = Vec::new();
        
        // Collect expired entries and LRU candidates
        for entry in self.cache.iter() {
            let (key, cached_response) = (entry.key(), entry.value());
            
            // Check if expired
            if now.signed_duration_since(cached_response.created_at) > ttl_duration {
                expired_keys.push(key.clone());
            } else {
                lru_candidates.push((key.clone(), cached_response.last_accessed, cached_response.access_count));
            }
        }
        
        // Remove expired entries
        for key in &expired_keys {
            self.cache.remove(key);
            
            let mut index = self.similarity_index.write().await;
            index.remove_entry(key);
        }
        
        info!("Removed {} expired cache entries", expired_keys.len());
        
        // If still over capacity, remove least recently used entries
        if self.cache.len() > self.config.max_entries {
            // Sort by last accessed time and access count
            lru_candidates.sort_by(|a, b| {
                a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2))
            });
            
            let entries_to_remove = self.cache.len() - self.config.max_entries;
            let mut removed_count = 0;
            
            for (key, _, _) in lru_candidates.iter().take(entries_to_remove) {
                self.cache.remove(key);
                
                let mut index = self.similarity_index.write().await;
                index.remove_entry(key);
                
                removed_count += 1;
            }
            
            info!("Removed {} LRU cache entries", removed_count);
        }
        
        Ok(())
    }
    
    /// Get cache statistics
    pub async fn get_statistics(&self) -> CacheStatistics {
        let stats = self.stats.read().await;
        let mut stats_clone = stats.clone();
        
        // Calculate hit rate and average similarity
        if stats_clone.total_requests > 0 {
            stats_clone.average_similarity_score = if stats_clone.similarity_matches > 0 {
                // This is a simplified calculation - in practice, you'd track actual similarity scores
                self.config.similarity_threshold
            } else {
                1.0 // Exact matches
            };
        }
        
        stats_clone
    }
    
    /// Get cache size and capacity information
    pub async fn get_cache_info(&self) -> CacheInfo {
        let index = self.similarity_index.read().await;
        
        CacheInfo {
            current_size: self.cache.len(),
            max_capacity: self.config.max_entries,
            similarity_index_size: index.len(),
            hit_rate: self.calculate_hit_rate().await,
            memory_usage_estimate_mb: self.estimate_memory_usage(),
        }
    }
    
    /// Calculate cache hit rate
    async fn calculate_hit_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        if stats.total_requests > 0 {
            stats.cache_hits as f64 / stats.total_requests as f64
        } else {
            0.0
        }
    }
    
    /// Estimate memory usage of the cache
    fn estimate_memory_usage(&self) -> f64 {
        let entry_count = self.cache.len();
        let avg_entry_size = 1024; // Rough estimate in bytes
        (entry_count * avg_entry_size) as f64 / 1024.0 / 1024.0 // Convert to MB
    }
    
    /// Clear all cached responses
    pub async fn clear_cache(&self) -> Result<()> {
        self.cache.clear();
        
        {
            let mut index = self.similarity_index.write().await;
            *index = SimilarityIndex::new();
        }
        
        {
            let mut stats = self.stats.write().await;
            *stats = CacheStatistics::default();
        }
        
        info!("AI response cache cleared");
        Ok(())
    }
    
    /// Clear expired entries based on TTL
    pub async fn clear_expired_entries(&self, older_than_hours: u64) -> Result<usize> {
        let now = chrono::Utc::now();
        let ttl_duration = chrono::Duration::hours(older_than_hours as i64);
        let mut expired_keys = Vec::new();
        
        // Collect expired entries
        for entry in self.cache.iter() {
            let (key, cached_response) = (entry.key(), entry.value());
            
            // Check if entry has expired based on last access time
            let age = now.signed_duration_since(cached_response.last_accessed);
            if age > ttl_duration {
                expired_keys.push(key.clone());
            }
        }
        
        // Remove expired entries
        let mut removed_count = 0;
        for key in &expired_keys {
            if self.cache.remove(key).is_some() {
                removed_count += 1;
                
                // Remove from similarity index
                let mut index = self.similarity_index.write().await;
                index.remove_entry(key);
            }
        }
        
        if removed_count > 0 {
            info!("Manually cleared {} expired cache entries (older than {} hours)", removed_count, older_than_hours);
        }
        
        Ok(removed_count)
    }
    
    /// Export cache statistics for analysis
    pub async fn export_analytics(&self) -> Result<CacheAnalytics> {
        let stats = self.get_statistics().await;
        let cache_info = self.get_cache_info().await;
        
        Ok(CacheAnalytics {
            statistics: stats,
            cache_info,
            top_providers: self.get_top_providers().await,
            top_models: self.get_top_models().await,
            cost_savings_summary: self.calculate_cost_savings().await,
        })
    }
    
    /// Get most used AI providers
    async fn get_top_providers(&self) -> Vec<(String, u64)> {
        let mut provider_counts = HashMap::new();
        
        for entry in self.cache.iter() {
            let cached_response = entry.value();
            *provider_counts.entry(cached_response.provider.clone()).or_insert(0) += cached_response.access_count;
        }
        
        let mut sorted_providers: Vec<_> = provider_counts.into_iter().collect();
        sorted_providers.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_providers.into_iter().take(10).collect()
    }
    
    /// Get most used AI models
    async fn get_top_models(&self) -> Vec<(String, u64)> {
        let mut model_counts = HashMap::new();
        
        for entry in self.cache.iter() {
            let cached_response = entry.value();
            *model_counts.entry(cached_response.model.clone()).or_insert(0) += cached_response.access_count;
        }
        
        let mut sorted_models: Vec<_> = model_counts.into_iter().collect();
        sorted_models.sort_by(|a, b| b.1.cmp(&a.1));
        sorted_models.into_iter().take(10).collect()
    }
    
    /// Calculate cost savings from caching
    async fn calculate_cost_savings(&self) -> CostSavingsSummary {
        let stats = self.stats.read().await;
        
        CostSavingsSummary {
            total_cost_saved: stats.total_cost_saved,
            total_tokens_saved: stats.total_tokens_saved,
            estimated_monthly_savings: stats.total_cost_saved * 30.0 / 7.0, // Rough estimate
            cache_efficiency: if stats.total_requests > 0 {
                stats.cache_hits as f64 / stats.total_requests as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub current_size: usize,
    pub max_capacity: usize,
    pub similarity_index_size: usize,
    pub hit_rate: f64,
    pub memory_usage_estimate_mb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheAnalytics {
    pub statistics: CacheStatistics,
    pub cache_info: CacheInfo,
    pub top_providers: Vec<(String, u64)>,
    pub top_models: Vec<(String, u64)>,
    pub cost_savings_summary: CostSavingsSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSavingsSummary {
    pub total_cost_saved: f64,
    pub total_tokens_saved: u64,
    pub estimated_monthly_savings: f64,
    pub cache_efficiency: f64,
}

/// Initialize AI response cache with default configuration
pub fn initialize_ai_response_cache(config: Option<CacheConfig>) -> AIResponseCache {
    let config = config.unwrap_or_default();
    
    info!(
        "Initializing AI response cache with {} max entries, {:.2} similarity threshold",
        config.max_entries, config.similarity_threshold
    );
    
    AIResponseCache::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cache_exact_match() {
        let cache = AIResponseCache::new(CacheConfig::default());
        
        let request = AIRequest {
            text: "Generate a character description".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            parameters: HashMap::new(),
            context: None,
        };
        
        let response = AIResponse {
            text: "A brave knight with golden armor".to_string(),
            tokens_used: Some(100),
            cost_estimate: Some(0.01),
            metadata: HashMap::new(),
        };
        
        // Cache the response
        cache.cache_response(&request, &response).await
            .expect("Failed to cache response in test");
        
        // Retrieve the response
        let cached = cache.get_response(&request).await
            .expect("Failed to get response from cache")
            .expect("Expected cached response to exist");
        assert_eq!(cached.response_text, response.text);
        assert_eq!(cached.access_count, 2); // 1 for caching, 1 for retrieval
    }
    
    #[tokio::test]
    async fn test_similarity_matching() {
        let mut config = CacheConfig::default();
        config.similarity_threshold = 0.5; // Lower threshold for testing
        
        let cache = AIResponseCache::new(config);
        
        let original_request = AIRequest {
            text: "Create a character description for a brave knight".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            parameters: HashMap::new(),
            context: None,
        };
        
        let response = AIResponse {
            text: "A valiant knight in shining armor".to_string(),
            tokens_used: Some(100),
            cost_estimate: Some(0.01),
            metadata: HashMap::new(),
        };
        
        // Cache the original response
        cache.cache_response(&original_request, &response).await
            .expect("Failed to cache original response in similarity test");
        
        // Try a similar request
        let similar_request = AIRequest {
            text: "Generate a character description for a brave knight".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            parameters: HashMap::new(),
            context: None,
        };
        
        // Should find the similar cached response
        let cached = cache.get_response(&similar_request).await
            .expect("Failed to get similar response from cache");
        assert!(cached.is_some());
        
        let cached_response = cached.expect("Expected similar cached response to exist");
        assert_eq!(cached_response.response_text, response.text);
        assert!(cached_response.similarity_score < 1.0); // Not an exact match
    }
    
    #[tokio::test]
    async fn test_cache_statistics() {
        let cache = AIResponseCache::new(CacheConfig::default());
        
        let request = AIRequest {
            text: "Test request".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            parameters: HashMap::new(),
            context: None,
        };
        
        // Miss
        let _ = cache.get_response(&request).await
            .expect("Failed to get response for cache miss test");
        
        // Cache and hit
        let response = AIResponse {
            text: "Test response".to_string(),
            tokens_used: Some(50),
            cost_estimate: Some(0.005),
            metadata: HashMap::new(),
        };
        
        cache.cache_response(&request, &response).await
            .expect("Failed to cache response in statistics test");
        let _ = cache.get_response(&request).await
            .expect("Failed to get cached response for hit test");
        
        let stats = cache.get_statistics().await;
        assert_eq!(stats.total_requests, 2);
        assert_eq!(stats.cache_hits, 1);
        assert_eq!(stats.cache_misses, 1);
        assert_eq!(stats.exact_matches, 1);
    }
    
    #[tokio::test]
    async fn test_time_based_clearing() {
        let mut config = CacheConfig::default();
        config.ttl_hours = 1; // 1 hour TTL for testing
        
        let cache = AIResponseCache::new(config);
        
        let request = AIRequest {
            text: "Test request for TTL".to_string(),
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            parameters: HashMap::new(),
            context: None,
        };
        
        let response = AIResponse {
            text: "Test response for TTL".to_string(),
            tokens_used: Some(50),
            cost_estimate: Some(0.005),
            metadata: HashMap::new(),
        };
        
        // Cache the response
        cache.cache_response(&request, &response).await
            .expect("Failed to cache response for TTL test");
        
        // Verify it's cached
        let cached = cache.get_response(&request).await
            .expect("Failed to get response from cache");
        assert!(cached.is_some());
        
        // Clear entries older than 0 hours (should clear everything)
        let cleared_count = cache.clear_expired_entries(0).await
            .expect("Failed to clear expired entries");
        assert_eq!(cleared_count, 1);
        
        // Verify cache is now empty
        let cached_after_clear = cache.get_response(&request).await
            .expect("Failed to get response from cache after clear");
        assert!(cached_after_clear.is_none());
        
        let cache_info = cache.get_cache_info().await;
        assert_eq!(cache_info.current_size, 0);
    }
}
