//! Common validator helpers for command handlers
//!
//! Goal: keep command files DRY and consistent by centralizing frequently
//! used validation patterns (ids, text fields, numeric ranges, lists, etc.).
//!
//! These helpers wrap the lower-level primitives from `security::validation`
//! and `security::rate_limit` so that command code can stay concise and clear.

use crate::error::StoryWeaverError;
use crate::security::rate_limit::validate_request_body_size;
use crate::security::validation::{validate_content_length, validate_security_input};

/// Validate an identifier-like field (e.g., project_id, document_id).
///
/// Rules:
/// - Not empty after trim
/// - Max length limited (default commonly <= 128)
/// - Must pass `validate_security_input` (guards for SQLi/XSS patterns)
pub fn validate_id(field_name: &str, id: &str, max_len: usize) -> Result<(), StoryWeaverError> {
    if id.trim().is_empty() {
        return Err(StoryWeaverError::input_validation(
            field_name,
            "cannot be empty",
        ));
    }
    // Check both character count and raw byte size against same limit
    validate_content_length(id, max_len)?;
    validate_request_body_size(id, max_len)?;
    validate_security_input(id)?;
    Ok(())
}

/// Validate a non-empty text field with length and security checks.
///
/// Rules:
/// - Not empty after trim
/// - Max length limited
/// - Must pass `validate_security_input`
pub fn validate_non_empty_str(field_name: &str, value: &str, max_len: usize) -> Result<(), StoryWeaverError> {
    if value.trim().is_empty() {
        return Err(StoryWeaverError::input_validation(
            field_name,
            "cannot be empty",
        ));
    }
    validate_content_length(value, max_len)?;
    validate_request_body_size(value, max_len)?;
    validate_security_input(value)?;
    Ok(())
}

/// Validate an optional text field. When present:
/// - If allow_empty = false, must be non-empty after trim
/// - Enforce length and security checks
pub fn validate_optional_str(
    field_name: &str,
    value: &Option<String>,
    max_len: usize,
    allow_empty: bool,
) -> Result<(), StoryWeaverError> {
    if let Some(ref v) = value {
        if !allow_empty && v.trim().is_empty() {
            return Err(StoryWeaverError::input_validation(
                field_name,
                "cannot be empty",
            ));
        }
        validate_content_length(v, max_len)?;
        validate_request_body_size(v, max_len)?;
        validate_security_input(v)?;
    }
    Ok(())
}

/// Validate a numeric range (inclusive) for i32.
///
/// Errors with a clear range message if outside bounds.
pub fn validate_numeric_range_i32(
    field_name: &str,
    value: i32,
    min_inclusive: i32,
    max_inclusive: i32,
) -> Result<(), StoryWeaverError> {
    if value < min_inclusive || value > max_inclusive {
        return Err(StoryWeaverError::validation(format!(
            "{} must be between {} and {}",
            field_name, min_inclusive, max_inclusive
        )));
    }
    Ok(())
}

/// Validate a numeric range (inclusive) for u32.
///
/// Errors with a clear range message if outside bounds.
pub fn validate_numeric_range_u32(
    field_name: &str,
    value: u32,
    min_inclusive: u32,
    max_inclusive: u32,
) -> Result<(), StoryWeaverError> {
    if value < min_inclusive || value > max_inclusive {
        return Err(StoryWeaverError::validation(format!(
            "{} must be between {} and {}",
            field_name, min_inclusive, max_inclusive
        )));
    }
    Ok(())
}

/// Validate raw body limits with both bytes and character-length constraints,
/// and run security checks.
/// Useful for content/metadata/payload validation where both limits apply.
pub fn validate_body_limits(
    field_name: &str,
    body: &str,
    max_bytes: usize,
    max_chars: usize,
) -> Result<(), StoryWeaverError> {
    validate_request_body_size(body, max_bytes)?;
    validate_content_length(body, max_chars)?;
    validate_security_input(body)?;
    Ok(())
}

/// Validate a list of identifiers.
///
/// Ensures each element is non-empty and security-validated.
pub fn validate_id_list(
    field_name: &str,
    ids: &[String],
    element_max_len: usize,
) -> Result<(), StoryWeaverError> {
    for (idx, id) in ids.iter().enumerate() {
        validate_id(&format!("{}[{}]", field_name, idx), id, element_max_len)?;
    }
    Ok(())
}

/// Validate an optional id string.
///
/// When present: not empty, length and security checks applied.
pub fn validate_optional_id(
    field_name: &str,
    id: &Option<String>,
    max_len: usize,
) -> Result<(), StoryWeaverError> {
    if let Some(ref val) = id {
        validate_id(field_name, val, max_len)?;
    }
    Ok(())
}

/// Shortcut for common "order_index" validation typically used in lists.
/// Defaults: [0, 10_000]
pub fn validate_order_index_default(order_index: i32) -> Result<(), StoryWeaverError> {
    validate_numeric_range_i32("Order index", order_index, 0, 10_000)
}

/// Guard for "rating 0..=10"-style validations.
pub fn validate_rating_0_to_10(rating: u32) -> Result<(), StoryWeaverError> {
    validate_numeric_range_u32("rating", rating, 0, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_id_rejects_empty() {
        let err = validate_id("project_id", "   ", 64).err().unwrap();
        let s = err.to_string().to_lowercase();
        assert!(s.contains("project_id") || s.contains("cannot be empty"), "unexpected: {}", s);
    }

    #[test]
    fn validate_id_accepts_basic() {
        let ok = validate_id("project_id", "abc123-XYZ_", 64);
        assert!(ok.is_ok(), "expected valid id to pass");
    }

    #[test]
    fn validate_non_empty_rejects_empty() {
        let err = validate_non_empty_str("title", "   ", 255).err().unwrap();
        let s = err.to_string().to_lowercase();
        assert!(s.contains("title") || s.contains("cannot be empty"), "unexpected: {}", s);
    }

    #[test]
    fn validate_non_empty_accepts_ok() {
        let ok = validate_non_empty_str("title", "Chapter 1", 255);
        assert!(ok.is_ok(), "expected non-empty title to pass");
    }

    #[test]
    fn validate_optional_str_rules() {
        // None is OK
        assert!(validate_optional_str("meta", &None, 500, false).is_ok());
        // Some empty not allowed when allow_empty = false
        let v = Some("   ".to_string());
        let e = validate_optional_str("meta", &v, 500, false).err().unwrap();
        assert!(e.to_string().to_lowercase().contains("cannot be empty"));
        // Some empty allowed when allow_empty = true (still passes security/length)
        let e2 = validate_optional_str("meta", &Some("".into()), 500, true);
        assert!(e2.is_ok());
    }

    #[test]
    fn numeric_range_i32_bounds() {
        assert!(validate_numeric_range_i32("order_index", 0, 0, 10_000).is_ok());
        assert!(validate_numeric_range_i32("order_index", 10_000, 0, 10_000).is_ok());
        let err = validate_numeric_range_i32("order_index", -1, 0, 10_000).err().unwrap();
        assert!(err.to_string().to_lowercase().contains("between"));
    }

    #[test]
    fn numeric_range_u32_bounds() {
        assert!(validate_numeric_range_u32("rating", 0, 0, 10).is_ok());
        assert!(validate_numeric_range_u32("rating", 10, 0, 10).is_ok());
        let err = validate_numeric_range_u32("rating", 11, 0, 10).err().unwrap();
        assert!(err.to_string().to_lowercase().contains("between"));
    }

    #[test]
    fn body_limits_rejects_oversize() {
        // bytes limit smaller than content size
        let content = "x".repeat(1024);
        let err = validate_body_limits("content", &content, 512, 10_000).err().unwrap();
        let s = err.to_string().to_lowercase();
        assert!(s.contains("exceeds") || s.contains("limit"), "unexpected: {}", s);
    }

    #[test]
    fn validate_id_list_mixed() {
        // All good
        let ids_ok = vec!["a".into(), "b".into(), "c".into()];
        assert!(validate_id_list("ids", &ids_ok, 64).is_ok());
        // One bad (empty)
        let ids_bad = vec!["good".into(), "   ".into()];
        let e = validate_id_list("ids", &ids_bad, 64).err().unwrap();
        let s = e.to_string().to_lowercase();
        assert!(s.contains("ids[1]") || s.contains("cannot be empty"), "unexpected: {}", s);
    }

    #[test]
    fn order_index_default_and_rating_shortcuts() {
        assert!(validate_order_index_default(0).is_ok());
        assert!(validate_order_index_default(10_000).is_ok());
        assert!(validate_order_index_default(-1).is_err());

        assert!(validate_rating_0_to_10(0).is_ok());
        assert!(validate_rating_0_to_10(10).is_ok());
        assert!(validate_rating_0_to_10(11).is_err());
    }
}