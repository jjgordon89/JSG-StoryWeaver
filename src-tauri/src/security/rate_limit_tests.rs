//! Unit tests for rate_limit utilities

use crate::security::rate_limit::{check_rate_limit, validate_request_body_size_default, validate_request_body_size};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn rate_limit_blocks_after_threshold_within_window() {
    // Allow 3 requests per 200ms window
    let key = "rl:test:block";
    assert!(check_rate_limit(key, 3, Duration::from_millis(200)).is_ok());
    assert!(check_rate_limit(key, 3, Duration::from_millis(200)).is_ok());
    assert!(check_rate_limit(key, 3, Duration::from_millis(200)).is_ok());
    // 4th within the same window should fail
    assert!(check_rate_limit(key, 3, Duration::from_millis(200)).is_err());
}

#[test]
fn rate_limit_allows_after_window_eviction() {
    let key = "rl:test:evict";
    // Hit the limit (2 per 150ms)
    assert!(check_rate_limit(key, 2, Duration::from_millis(150)).is_ok());
    assert!(check_rate_limit(key, 2, Duration::from_millis(150)).is_ok());
    // Next call should fail within the window
    assert!(check_rate_limit(key, 2, Duration::from_millis(150)).is_err());

    // Wait for the window to pass, then it should allow again
    sleep(Duration::from_millis(180));
    assert!(check_rate_limit(key, 2, Duration::from_millis(150)).is_ok());
}

#[test]
fn request_size_validators_work_as_expected() {
    // Default 1MB limit passes for small content
    let small = "hello world";
    assert!(validate_request_body_size_default(small).is_ok());

    // Custom small limit rejects content larger than limit
    let content = "abcdefghij"; // 10 bytes
    assert!(validate_request_body_size(content, 9).is_err());
    assert!(validate_request_body_size(content, 10).is_ok());
}
