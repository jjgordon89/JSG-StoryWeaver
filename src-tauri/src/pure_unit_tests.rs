//! Pure Rust unit tests (no heavy native deps)
//! These helpers are intentionally simple and independent from Tauri/sqlx so they
//! can be executed while we diagnose native-loader issues in the larger test harness.

/// Count words by splitting on whitespace
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Check if a string is a palindrome (alphanumeric, case-insensitive)
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_basic() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("  multiple   spaces "), 2);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("one"), 1);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("No 'x' in Nixon"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome("")); // empty string is a palindrome
    }
}
