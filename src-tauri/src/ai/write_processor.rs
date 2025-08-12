//! Write Processor for handling AI-powered writing features

use super::{AIProvider, AIContext};
use crate::database::DbPool;
use crate::database::operations::{document_ops, DocumentOps};
use crate::error::{Result, StoryWeaverError};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteSettings {
    pub mode: WriteMode,
    pub creativity_level: u8,  // 1-10
    pub word_count_target: Option<usize>,
    pub tone: Option<String>,
    pub include_key_details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WriteMode {
    Auto,       // Continue writing naturally
    Guided,     // Follow user's specific instructions
    ToneShift,  // Write with a specific tone
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    pub generated_text: String,
    pub credits_used: f32,
    pub word_count: usize,
    pub tokens_used: usize,
}

pub struct WriteProcessor {
    ai_provider: Arc<dyn AIProvider>,
    context_builder: ContextBuilder,
}

impl WriteProcessor {
    pub fn new(ai_provider: Arc<dyn AIProvider>) -> Self {
        Self {
            ai_provider,
            context_builder: ContextBuilder::new(),
        }
    }

    /// Auto write - continue the story naturally
    pub async fn auto_write(
        &self,
        document_id: i32,
        cursor_position: usize,
        settings: &WriteSettings,
        db_pool: &DbPool,
    ) -> Result<WriteResult> {
        // Build context from the document
        let context = self.context_builder
            .build_write_context(document_id, cursor_position, 1000, db_pool)
            .await?;
        
        // Create the prompt for continuation
        let prompt = self.build_auto_write_prompt(&context);
        
        // Generate text
        let generated_text = self.ai_provider
            .generate_text(&prompt, &context)
            .await?;
        
        // Calculate metrics
        let word_count = count_words(&generated_text);
        let tokens_used = estimate_tokens(&generated_text);
        let credits_used = calculate_credits(tokens_used);
        
        Ok(WriteResult {
            generated_text,
            credits_used,
            word_count,
            tokens_used,
        })
    }

    /// Guided write - follow user's specific instructions
    pub async fn guided_write(
        &self,
        document_id: i32,
        user_prompt: &str,
        settings: &WriteSettings,
        db_pool: &DbPool,
    ) -> Result<WriteResult> {
        // Build context from the document
        let mut context = self.context_builder
            .build_write_context(document_id, 0, 1000, db_pool)
            .await?;
        
        // Add user's instructions to context
        context.key_details = Some(settings.include_key_details.clone());
        context.creativity_level = Some(settings.creativity_level);
        
        // Create the prompt with user's guidance
        let prompt = format!(
            "Write the next part of this story based on this direction: '{}'\n\nStory context: {}",
            user_prompt,
            context.story_context.as_ref().unwrap_or(&String::new())
        );
        
        // Generate text
        let generated_text = self.ai_provider
            .generate_text(&prompt, &context)
            .await?;
        
        // Calculate metrics
        let word_count = count_words(&generated_text);
        let tokens_used = estimate_tokens(&generated_text);
        let credits_used = calculate_credits(tokens_used);
        
        Ok(WriteResult {
            generated_text,
            credits_used,
            word_count,
            tokens_used,
        })
    }

    /// Tone shift write - write with a specific tone
    pub async fn tone_shift_write(
        &self,
        document_id: i32,
        cursor_position: usize,
        tone: &str,
        settings: &WriteSettings,
        db_pool: &DbPool,
    ) -> Result<WriteResult> {
        // Build context from the document
        let mut context = self.context_builder
            .build_write_context(document_id, cursor_position, 1000, db_pool)
            .await?;
        
        // Set the tone in context
        context.tone = Some(tone.to_string());
        context.creativity_level = Some(settings.creativity_level);
        
        // Create the prompt with tone instruction
        let prompt = format!(
            "Continue this story in a {} tone. Context: {}",
            tone,
            context.preceding_text.as_ref().unwrap_or(&String::new())
        );
        
        // Generate text
        let generated_text = self.ai_provider
            .generate_text(&prompt, &context)
            .await?;
        
        // Calculate metrics
        let word_count = count_words(&generated_text);
        let tokens_used = estimate_tokens(&generated_text);
        let credits_used = calculate_credits(tokens_used);
        
        Ok(WriteResult {
            generated_text,
            credits_used,
            word_count,
            tokens_used,
        })
    }

    fn build_auto_write_prompt(&self, context: &AIContext) -> String {
        let mut prompt = String::new();
        
        // Add preceding text for context
        if let Some(preceding) = &context.preceding_text {
            prompt.push_str("Continue this story naturally from where it left off:\n\n");
            prompt.push_str(preceding);
            prompt.push_str("\n\nContinue writing:");
        } else {
            prompt.push_str("Begin writing a story:");
        }
        
        prompt
    }
}

/// Context builder for assembling relevant context for AI generation
pub struct ContextBuilder {
    // Could add caching and other optimizations here
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {}
    }

    /// Build context for writing features
    pub async fn build_write_context(
        &self,
        document_id: i32,
        cursor_position: usize,
        context_window: usize,
        db_pool: &DbPool,
    ) -> Result<AIContext> {
        let mut context = AIContext::default();
        
        // Get document content
        let document = DocumentOps::get_by_id(db_pool, &document_id.to_string())
            .await?
            .ok_or_else(|| StoryWeaverError::database(format!("Document with id {} not found", document_id)))?;
        
        // Extract text around cursor position
        let content = document.content;
        let content_chars: Vec<char> = content.chars().collect();
        let total_len = content_chars.len();
        
        // Get preceding text (up to context_window characters before cursor)
        let start = cursor_position.saturating_sub(context_window);
        let preceding: String = content_chars[start..cursor_position.min(total_len)]
            .iter()
            .collect();
        context.preceding_text = Some(preceding);
        
        // Get following text (up to context_window/2 characters after cursor)
        let end = (cursor_position + context_window / 2).min(total_len);
        if cursor_position < total_len {
            let following: String = content_chars[cursor_position..end]
                .iter()
                .collect();
            context.following_text = Some(following);
        }
        
        // Create a story summary (simplified for now)
        let summary = if content.len() > 500 {
            format!("{}...", &content[..500])
        } else {
            content.clone()
        };
        context.story_context = Some(summary);
        
        // Set document and project IDs
        context.document_id = Some(document_id.to_string());
        context.project_id = Some(document.project_id.to_string());
        
        // TODO: Add Story Bible elements (characters, locations, etc.)
        // This would involve querying the story bible tables
        
        Ok(context)
    }

    /// Build context for rewrite features
    pub async fn build_rewrite_context(
        &self,
        selected_text: &str,
        document_id: Option<i32>,
        db_pool: &DbPool,
    ) -> Result<AIContext> {
        let mut context = AIContext::default();
        
        context.selected_text = Some(selected_text.to_string());
        
        // If we have a document ID, get additional context
        if let Some(doc_id) = document_id {
            let document = DocumentOps::get_by_id(db_pool, &doc_id.to_string())
                .await?
                .ok_or_else(|| StoryWeaverError::database(format!("Document with id {} not found", doc_id)))?;
            
            context.document_id = Some(doc_id.to_string());
            context.project_id = Some(document.project_id.to_string());
            
            // Add story context if available
            let content = document.content;
            let summary = if content.len() > 500 {
                format!("{}...", &content[..500])
            } else {
                content
            };
            context.story_context = Some(summary);
        }
        
        Ok(context)
    }
}

/// Count words in text
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Estimate token count (rough approximation)
fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: 1 token ≈ 4 characters
    text.len() / 4
}

/// Calculate credits based on token usage
fn calculate_credits(tokens: usize) -> f32 {
    // Example pricing: $0.002 per 1K tokens
    (tokens as f32 / 1000.0) * 0.002
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("Hello world"), 2);
        assert_eq!(count_words("This is a test sentence."), 5);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("   "), 0);
    }

    #[test]
    fn test_estimate_tokens() {
        assert_eq!(estimate_tokens("Hello world"), 2); // 11 chars / 4 ≈ 2
        assert_eq!(estimate_tokens("This is a longer test sentence."), 7); // 31 chars / 4 ≈ 7
    }

    #[test]
    fn test_calculate_credits() {
        assert_eq!(calculate_credits(1000), 0.002);
        assert_eq!(calculate_credits(5000), 0.01);
        assert_eq!(calculate_credits(0), 0.0);
    }
}
