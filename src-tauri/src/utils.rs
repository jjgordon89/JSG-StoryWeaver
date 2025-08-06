//! Utility functions for StoryWeaver

// Module declarations
pub mod performance_monitor;

use crate::error::{Result, StoryWeaverError};
use std::path::Path;
use uuid::Uuid;

/// Generate a new UUID string
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

/// Validate that a string is a valid UUID
pub fn is_valid_uuid(id: &str) -> bool {
    Uuid::parse_str(id).is_ok()
}

/// Sanitize a string for use as a filename
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// Count words in a text string
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Count characters in a text string (excluding whitespace)
pub fn count_characters(text: &str) -> usize {
    text.chars().filter(|c| !c.is_whitespace()).count()
}

/// Count paragraphs in a text string
pub fn count_paragraphs(text: &str) -> usize {
    text.split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .count()
}

/// Truncate text to a specified length with ellipsis
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length.saturating_sub(3)])
    }
}

/// Extract a preview from text (first few sentences)
pub fn extract_preview(text: &str, max_sentences: usize) -> String {
    let sentences: Vec<&str> = text
        .split('.')
        .take(max_sentences)
        .collect();
    
    if sentences.len() < max_sentences || text.ends_with('.') {
        sentences.join(".")
    } else {
        format!("{}.", sentences.join("."))
    }
}

/// Validate file extension
pub fn is_supported_file_extension(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            matches!(ext_str.to_lowercase().as_str(), "txt" | "md" | "docx" | "rtf" | "odt" | "csv")
        } else {
            false
        }
    } else {
        false
    }
}

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Validate project name
pub fn validate_project_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Project name cannot be empty"));
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::validation("Project name cannot exceed 100 characters"));
    }
    
    // Check for invalid characters
    if name.chars().any(|c| c.is_control() && c != '\t') {
        return Err(StoryWeaverError::validation("Project name contains invalid characters"));
    }
    
    Ok(())
}

/// Validate document title
pub fn validate_document_title(title: &str) -> Result<()> {
    if title.trim().is_empty() {
        return Err(StoryWeaverError::validation("Document title cannot be empty"));
    }
    
    if title.len() > 200 {
        return Err(StoryWeaverError::validation("Document title cannot exceed 200 characters"));
    }
    
    Ok(())
}

/// Validate character name
pub fn validate_character_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Character name cannot be empty"));
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::validation("Character name cannot exceed 100 characters"));
    }
    
    Ok(())
}

/// Validate location name
pub fn validate_location_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(StoryWeaverError::validation("Location name cannot be empty"));
    }
    
    if name.len() > 100 {
        return Err(StoryWeaverError::validation("Location name cannot exceed 100 characters"));
    }
    
    Ok(())
}

/// Calculate reading time estimate (words per minute)
pub fn calculate_reading_time(word_count: usize, wpm: usize) -> String {
    let minutes = (word_count as f64 / wpm as f64).ceil() as usize;
    
    if minutes < 60 {
        format!("{} min", minutes)
    } else {
        let hours = minutes / 60;
        let remaining_minutes = minutes % 60;
        
        if remaining_minutes == 0 {
            format!("{} hr", hours)
        } else {
            format!("{} hr {} min", hours, remaining_minutes)
        }
    }
}

/// Extract hashtags from text
pub fn extract_hashtags(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|word| word.starts_with('#') && word.len() > 1)
        .map(|tag| tag[1..].to_string()) // Remove the # symbol
        .collect()
}

/// Extract mentions from text (@username)
pub fn extract_mentions(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|word| word.starts_with('@') && word.len() > 1)
        .map(|mention| mention[1..].to_string()) // Remove the @ symbol
        .collect()
}

/// Clean text for search (remove special characters, normalize whitespace)
pub fn clean_text_for_search(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c.to_lowercase().to_string()
            } else {
                " ".to_string()
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Generate search keywords from text
pub fn generate_search_keywords(text: &str, min_length: usize) -> Vec<String> {
    let cleaned = clean_text_for_search(text);
    let mut keywords: Vec<String> = cleaned
        .split_whitespace()
        .filter(|word| word.len() >= min_length)
        .map(|word| word.to_string())
        .collect();
    
    // Remove duplicates and sort
    keywords.sort();
    keywords.dedup();
    
    keywords
}

/// Escape special characters for SQL LIKE queries
pub fn escape_sql_like(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

/// Format duration in human-readable format
pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{} sec", seconds)
    } else if seconds < 3600 {
        let minutes = seconds / 60;
        let remaining_seconds = seconds % 60;
        
        if remaining_seconds == 0 {
            format!("{} min", minutes)
        } else {
            format!("{} min {} sec", minutes, remaining_seconds)
        }
    } else {
        let hours = seconds / 3600;
        let remaining_minutes = (seconds % 3600) / 60;
        
        if remaining_minutes == 0 {
            format!("{} hr", hours)
        } else {
            format!("{} hr {} min", hours, remaining_minutes)
        }
    }
}

/// Check if text contains profanity (basic implementation)
pub fn contains_profanity(text: &str) -> bool {
    // This is a basic implementation - in production, you'd want a more sophisticated approach
    let profanity_words = [
        "damn", "hell", "shit", "fuck", "bitch", "ass", "bastard", "crap"
    ];
    
    let lowercase_text = text.to_lowercase();
    profanity_words.iter().any(|&word| lowercase_text.contains(word))
}

/// Generate a slug from text (URL-friendly)
pub fn generate_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
        .trim_matches('-')
        .to_string()
}

/// Calculate text similarity using Jaccard index
pub fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
    let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();
    
    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();
    
    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Text statistics
#[derive(Debug, serde::Serialize)]
pub struct TextStats {
    pub word_count: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
    pub paragraph_count: usize,
    pub sentence_count: usize,
    pub reading_time_minutes: usize,
    pub hashtags: Vec<String>,
    pub mentions: Vec<String>,
}

/// Calculate comprehensive text statistics
pub fn calculate_text_stats(text: &str) -> TextStats {
    let word_count = count_words(text);
    let character_count = text.len();
    let character_count_no_spaces = count_characters(text);
    let paragraph_count = count_paragraphs(text);
    let sentence_count = text.matches('.').count() + text.matches('!').count() + text.matches('?').count();
    let reading_time_minutes = (word_count as f64 / 200.0).ceil() as usize; // 200 WPM average
    let hashtags = extract_hashtags(text);
    let mentions = extract_mentions(text);
    
    TextStats {
        word_count,
        character_count,
        character_count_no_spaces,
        paragraph_count,
        sentence_count,
        reading_time_minutes,
        hashtags,
        mentions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_id() {
        let id = generate_id();
        assert!(is_valid_uuid(&id));
    }
    
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("hello/world"), "hello_world");
        assert_eq!(sanitize_filename("test:file"), "test_file");
        assert_eq!(sanitize_filename("normal_file.txt"), "normal_file.txt");
    }
    
    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("  hello   world  "), 2);
        assert_eq!(count_words(""), 0);
    }
    
    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("hello world", 10), "hello w...");
        assert_eq!(truncate_text("hello", 10), "hello");
    }
    
    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World!"), "hello-world");
        assert_eq!(generate_slug("Test_File-Name"), "test-file-name");
    }
    
    #[test]
    fn test_calculate_text_similarity() {
        let similarity = calculate_text_similarity("hello world", "hello universe");
        assert!(similarity > 0.0 && similarity < 1.0);
        
        let identical = calculate_text_similarity("hello world", "hello world");
        assert_eq!(identical, 1.0);
    }
}
