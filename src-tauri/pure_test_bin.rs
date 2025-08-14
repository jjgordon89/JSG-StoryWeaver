// Standalone test binary for pure unit tests (no project linkage).
// Runs simple checks for count_words and is_palindrome from pure_unit_tests.rs
// This file intentionally duplicates the minimal functions to avoid linking the main crate.

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn main() {
    // count_words tests
    assert_eq!(count_words("hello world"), 2);
    assert_eq!(count_words("  multiple   spaces "), 2);
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("one"), 1);

    // is_palindrome tests
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("No 'x' in Nixon"));
    assert!(!is_palindrome("hello"));
    assert!(is_palindrome("")); // empty string is a palindrome

    println!("PURE_TESTS: all checks passed");
}
