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

/// Check rate limit with custom parameters
pub fn check_rate_limit(key: &str, max_requests: usize, per: Duration) -> Result<(), StoryWeaverError> {
    RATE_LIMITER.check(key, max_requests, per)
}

/// Convenience: 60 requests per minute per key
pub fn check_rate_limit_default(key: &str) -> Result<(), StoryWeaverError> {
    check_rate_limit(key, DEFAULT_MAX_REQUESTS_PER_MINUTE, Duration::from_secs(60))
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
