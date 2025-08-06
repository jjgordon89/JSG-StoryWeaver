//! Claude (Anthropic) Provider implementation for StoryWeaver

use super::{AIProvider, AIContext, TextStream, RewriteStyle};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

// Rate limiting constants
const REQUESTS_PER_MINUTE: u32 = 50; // Anthropic's rate limits may differ
const TOKENS_PER_MINUTE: u32 = 80000;

#[derive(Debug, Clone, Serialize)]
struct ClaudeMessage {
    role: String,
    content: Vec<ClaudeContent>,
}

#[derive(Debug, Clone, Serialize)]
struct ClaudeContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Clone, Serialize)]
struct ClaudeCompletionRequest {
    model: String,
    messages: Vec<ClaudeMessage>,
    max_tokens: u32,
    temperature: f32,
    system: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ClaudeCompletionResponse {
    id: String,
    content: Vec<ClaudeContent>,
    usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenUsage {
    input_tokens: u32,
    output_tokens: u32,
}

pub struct ClaudeProvider {
    pub api_key: String,
    pub model: String,
    pub client: reqwest::Client,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

struct RateLimiter {
    request_count: u32,
    token_count: u32,
    last_reset: std::time::Instant,
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            request_count: 0,
            token_count: 0,
            last_reset: std::time::Instant::now(),
        }
    }

    async fn wait_if_needed(&mut self, estimated_tokens: u32) -> Result<()> {
        // Reset counters if a minute has passed
        let now = std::time::Instant::now();
        if now.duration_since(self.last_reset).as_secs() >= 60 {
            self.request_count = 0;
            self.token_count = 0;
            self.last_reset = now;
        }

        // Check if we're about to exceed limits
        if self.request_count >= REQUESTS_PER_MINUTE || 
           self.token_count + estimated_tokens >= TOKENS_PER_MINUTE {
            
            // Calculate time to wait until next minute
            let elapsed = now.duration_since(self.last_reset).as_millis() as u64;
            let wait_time = if elapsed < 60000 { 60000 - elapsed } else { 0 };
            
            // Wait until rate limit resets
            if wait_time > 0 {
                sleep(Duration::from_millis(wait_time)).await;
                self.request_count = 0;
                self.token_count = 0;
                self.last_reset = std::time::Instant::now();
            }
        }

        // Update counters
        self.request_count += 1;
        self.token_count += estimated_tokens;
        
        Ok(())
    }

    fn update_token_usage(&mut self, usage: &TokenUsage) {
        self.token_count = self.token_count.saturating_add(usage.input_tokens + usage.output_tokens);
    }
}

impl ClaudeProvider {
    pub fn new(api_key: String, model: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_default();
            
        Self {
            api_key,
            model,
            client,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
        }
    }

    fn build_system_message(&self, context: &AIContext) -> String {
        // Build a system message based on the context
        // This could include information about the document, user preferences, etc.
        "You are StoryWeaver, an AI writing assistant. Help the user write their story.".to_string()
    }
}

#[async_trait]
impl AIProvider for ClaudeProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (prompt.len() / 4) as u32 + 500; // Rough estimate
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build request
        let system = self.build_system_message(context);
        let user_message = ClaudeMessage {
            role: "user".to_string(),
            content: vec![ClaudeContent {
                content_type: "text".to_string(),
                text: prompt.to_string(),
            }],
        };
        
        let request = ClaudeCompletionRequest {
            model: self.model.clone(),
            messages: vec![user_message],
            max_tokens: 1000,
            temperature: 0.7,
            system: Some(system),
        };
        
        // Make API call
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("Claude API error: {}", error_text));
        }
        
        // Parse response
        let completion: ClaudeCompletionResponse = response.json().await
            .context("Failed to parse Claude API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        let mut result = String::new();
        for content in completion.content {
            if content.content_type == "text" {
                result.push_str(&content.text);
            }
        }
        
        if result.is_empty() {
            Err(anyhow::anyhow!("No text content returned"))
        } else {
            Ok(result)
        }
    }

    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> Result<TextStream> {
        // For now, we'll just return a placeholder
        // In a real implementation, this would use the streaming API
        Ok(TextStream)
    }

    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32 + 500; // Rough estimate
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build prompt based on rewrite style
        let style_instruction = match style {
            RewriteStyle::Rephrase => "Rephrase this text while keeping the same meaning:",
            RewriteStyle::Shorter => "Rewrite this text to be more concise:",
            RewriteStyle::MoreDescriptive => "Rewrite this text to be more descriptive and vivid:",
        };
        
        let system = format!("You are a helpful writing assistant. {}", style_instruction);
        
        let user_message = ClaudeMessage {
            role: "user".to_string(),
            content: vec![ClaudeContent {
                content_type: "text".to_string(),
                text: text.to_string(),
            }],
        };
        
        let request = ClaudeCompletionRequest {
            model: self.model.clone(),
            messages: vec![user_message],
            max_tokens: (text.len() as u32 / 2), // Limit token usage based on input
            temperature: 0.7,
            system: Some(system),
        };
        
        // Make API call
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("Claude API error: {}", error_text));
        }
        
        // Parse response
        let completion: ClaudeCompletionResponse = response.json().await
            .context("Failed to parse Claude API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        let mut result = String::new();
        for content in completion.content {
            if content.content_type == "text" {
                result.push_str(&content.text);
            }
        }
        
        if result.is_empty() {
            Err(anyhow::anyhow!("No text content returned"))
        } else {
            Ok(result)
        }
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Claude doesn't have a native embedding API, so we'll return an error
        Err(anyhow::anyhow!("Claude does not support embeddings"))
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn get_context_window(&self) -> usize {
        match self.model.as_str() {
            "claude-3-opus" => 200000,
            "claude-3-sonnet" => 180000,
            "claude-3-haiku" => 150000,
            _ => 100000, // Default for unknown models
        }
    }

    fn get_model_name(&self) -> &str {
        &self.model
    }
}
