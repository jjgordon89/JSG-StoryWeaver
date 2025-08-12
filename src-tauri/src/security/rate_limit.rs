//! Rate limiting and request size utilities
//!
//! This module provides lightweight, in-process rate limiting helpers and
//! request size validation utilities to harden API endpoints against abuse
//! and denial-of-service scenarios.
//!
//! Usage examples:
//! - Per-endpoint rate limit (60 req/min):
//!     check_rate_limit_default("create_document")?;
//! - Per-entity rate limit (e.g., endpoint + project id):
//!     check_rate_limit("create_document:project=42", 30, Duration::from_secs(60))?;
//! - Request body size limit (1 MB default):
//!     validate_request_body_size_default(body_str)?;
//!
//! Integrate these checks at the start of command handlers.

use crate::error::StoryWeaverError;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Default limits (can be overridden per-call)
pub const DEFAULT_MAX_REQUESTS_PER_MINUTE: usize = 60;
pub const DEFAULT_REQUEST_MAX_BYTES: usize = 1_000_000; // 1 MB

/// Central data structure for rate limiting buckets
///
/// Each key maps to a deque of Instants representing the timestamps of recent requests.
/// We keep only events within the active time window for the given check.
#[derive(Debug)]
pub struct RateLimiter {
    buckets: DashMap<String, VecDeque<Instant>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: DashMap::new(),
        }
    }

    /// Check and record a request for the given key.
    ///
    /// - `key`: a stable identifier, e.g., "endpoint_name", "endpoint:project=123", etc.
    /// - `max_requests`: maximum allowed within `per` window
    /// - `per`: size of the rate limit window (e.g., 60 seconds)
    pub fn check(&self, key: &str, max_requests: usize, per: Duration) -> Result<(), StoryWeaverError> {
        let now = Instant::now();
        let mut deque = self
            .buckets
            .entry(key.to_string())
            .or_insert_with(|| VecDeque::with_capacity(max_requests + 4));

        // Purge timestamps older than window
        while let Some(ts) = deque.front().copied() {
            if now.duration_since(ts) > per {
                deque.pop_front();
            } else {
                break;
            }
        }

        if deque.len() >= max_requests {
            // Too many requests within the window
            return Err(StoryWeaverError::security_error(format!(
                "Rate limit exceeded for '{}': max {} requests per {:?}",
                key, max_requests, per
            )));
        }

        // Record this request
        deque.push_back(now);
        Ok(())
    }
}

static RATE_LIMITER: Lazy<RateLimiter> = Lazy::new(|| RateLimiter::new());

//// Check rate limit with custom parameters
pub fn check_rate_limit(key: &str, max_requests: usize, per: Duration) -> Result<(), StoryWeaverError> {
    RATE_LIMITER.check(key, max_requests, per)
}

/// Convenience: 60 requests per minute per key
pub fn check_rate_limit_default(key: &str) -> Result<(), StoryWeaverError> {
    check_rate_limit(key, DEFAULT_MAX_REQUESTS_PER_MINUTE, Duration::from_secs(60))
}

/// Standardized rate limit configuration (60-second window)
pub const RL_WINDOW_SECS: u64 = 60;
pub const RL_CREATE_RPM: usize = 60;
pub const RL_UPDATE_RPM: usize = 120;
pub const RL_DELETE_RPM: usize = 60;
pub const RL_SEARCH_RPM: usize = 120;
pub const RL_LIST_RPM: usize = 180;
pub const RL_SAVE_RPM: usize = 300;

/// Build a standardized key using the schema:
/// "{action}:{entity}" or "{action}:{entity}:{scope}"
/// If env var RL_NAMESPACE is set, append ":ns={value}" to isolate tests/runs.
fn build_rate_key(action: &str, entity: &str, scope: Option<&str>) -> String {
    let mut key = if let Some(scope_val) = scope {
        format!("{}:{}:{}", action, entity, scope_val)
    } else {
        format!("{}:{}", action, entity)
    };
    if let Ok(ns) = std::env::var("RL_NAMESPACE") {
        if !ns.is_empty() {
            key.push(':');
            key.push_str(&format!("ns={}", ns));
        }
    }
    key
}

fn env_override_usize(var: &str) -> Option<usize> {
    std::env::var(var).ok().and_then(|v| v.parse::<usize>().ok())
}

fn env_override_u64(var: &str) -> Option<u64> {
    std::env::var(var).ok().and_then(|v| v.parse::<u64>().ok())
}

/// Standardized helpers for consistent limits across endpoints

/// Create operations (e.g., create_project, create_document, create_*).
pub fn rl_create(entity: &str, scope: Option<&str>) -> Result<(), StoryWeaverError> {
    let key = build_rate_key("create", entity, scope);
    let limit = env_override_usize("RL_CREATE_RPM").unwrap_or(RL_CREATE_RPM);
    let window = env_override_u64("RL_WINDOW_SECS").unwrap_or(RL_WINDOW_SECS);
    check_rate_limit(&key, limit, Duration::from_secs(window))
}

/// Update operations (e.g., update_project, update_document, update_*).
pub fn rl_update(entity: &str, scope: Option<&str>) -> Result<(), StoryWeaverError> {
    let key = build_rate_key("update", entity, scope);
    let limit = env_override_usize("RL_UPDATE_RPM").unwrap_or(RL_UPDATE_RPM);
    let window = env_override_u64("RL_WINDOW_SECS").unwrap_or(RL_WINDOW_SECS);
    check_rate_limit(&key, limit, Duration::from_secs(window))
}

/// Delete operations.
pub fn rl_delete(entity: &str, scope: Option<&str>) -> Result<(), StoryWeaverError> {
    let key = build_rate_key("delete", entity, scope);
    let limit = env_override_usize("RL_DELETE_RPM").unwrap_or(RL_DELETE_RPM);
    let window = env_override_u64("RL_WINDOW_SECS").unwrap_or(RL_WINDOW_SECS);
    check_rate_limit(&key, limit, Duration::from_secs(window))
}

/// Search/query operations (free-text or param-based "search").
pub fn rl_search(entity: &str, scope: Option<&str>) -> Result<(), StoryWeaverError> {
    let key = build_rate_key("search", entity, scope);
    let limit = env_override_usize("RL_SEARCH_RPM").unwrap_or(RL_SEARCH_RPM);
    let window = env_override_u64("RL_WINDOW_SECS").unwrap_or(RL_WINDOW_SECS);
    check_rate_limit(&key, limit, Duration::from_secs(window))
}

/// List/browse operations (e.g., get_* collections).
pub fn rl_list(entity: &str, scope: Option<&str>) -> Result<(), StoryWeaverError> {
    let key = build_rate_key("list", entity, scope);
    let limit = env_override_usize("RL_LIST_RPM").unwrap_or(RL_LIST_RPM);
    let window = env_override_u64("RL_WINDOW_SECS").unwrap_or(RL_WINDOW_SECS);
    check_rate_limit(&key, limit, Duration::from_secs(window))
}

/// High-frequency save operations.
pub fn rl_save(entity: &str, scope: Option<&str>) -> Result<(), StoryWeaverError> {
    let key = build_rate_key("save", entity, scope);
    let limit = env_override_usize("RL_SAVE_RPM").unwrap_or(RL_SAVE_RPM);
    let window = env_override_u64("RL_WINDOW_SECS").unwrap_or(RL_WINDOW_SECS);
    check_rate_limit(&key, limit, Duration::from_secs(window))
}

#[cfg(test)]
pub fn reset_rate_limits_for_test() {
    RATE_LIMITER.buckets.clear();
}

/// Validate raw request size against a specified byte limit
pub fn validate_request_size(size_bytes: usize, max_bytes: usize) -> Result<(), StoryWeaverError> {
    if size_bytes > max_bytes {
        return Err(StoryWeaverError::invalid_input(format!(
            "Request size exceeds limit: {} > {} bytes",
            size_bytes, max_bytes
        )));
    }
    Ok(())
}

/// Validate a string body against the default max size (1 MB)
pub fn validate_request_body_size_default(body: &str) -> Result<(), StoryWeaverError> {
    validate_request_size(body.as_bytes().len(), DEFAULT_REQUEST_MAX_BYTES)
}

/// Validate a string body against a custom byte limit
pub fn validate_request_body_size(body: &str, max_bytes: usize) -> Result<(), StoryWeaverError> {
    validate_request_size(body.as_bytes().len(), max_bytes)
}
