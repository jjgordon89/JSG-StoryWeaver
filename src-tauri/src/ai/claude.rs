//! Claude (Anthropic) Provider implementation for StoryWeaver

use super::{AIProvider, AIContext, TextStream, RewriteStyle};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::{Result, StoryWeaverError};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors first
        let status_code = response.status().as_u16();
        let is_success = response.status().is_success();
        
        // Get response text
        let response_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        
        if !is_success {
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code,
                message: format!("Claude API error: {}", response_text),
            });
        }
        
        // Parse response
        let completion: ClaudeCompletionResponse = serde_json::from_str(&response_text)
            .map_err(|e| StoryWeaverError::AIProvider {
                provider: "claude".to_string(),
                message: format!("Failed to parse Claude API response: {}", e),
            })?;
        
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
            Err(StoryWeaverError::AIProvider {
                provider: "claude".to_string(),
                message: "No text content returned".to_string(),
            })
        } else {
            Ok(result)
        }
    }

    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> Result<TextStream> {
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
        
        // Make API call with streaming
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .header("anthropic-beta", "messages-2023-12-15-streaming") // Enable streaming
            .query(&[("stream", "true")]) // Add stream parameter
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors first
        let status_code = response.status().as_u16();
        let is_success = response.status().is_success();
        
        if !is_success {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code,
                message: format!("Claude API error: {}", error_text),
            });
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| StoryWeaverError::AIProvider {
                provider: "claude".to_string(),
                message: format!("Error reading stream chunk: {}", e),
            })?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // Claude sends "data: " prefixed SSE events
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Skip "data: " prefix
                    
                    // Skip empty lines and [DONE] marker
                    if data.is_empty() || data == "[DONE]" {
                        continue;
                    }
                    
                    // Parse the JSON data
                    match serde_json::from_str::<serde_json::Value>(data) {
                        Ok(json) => {
                            // Extract the content from the JSON
                            if let Some(delta) = json.get("delta") {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    text_stream.append(text);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing JSON from stream: {}", e);
                            continue;
                        }
                    }
                }
            }
        }
        
        // Mark the stream as complete
        text_stream.complete();
        
        Ok(text_stream)
    }

    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> Result<String> {
        let estimated_tokens = (text.len() / 4) as u32 + 500;
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }

        let style_instruction = match style {
            RewriteStyle::Rephrase => "Rephrase this text while keeping the same meaning:",
            RewriteStyle::Shorter => "Rewrite this text to be more concise:",
            RewriteStyle::MoreDescriptive => "Rewrite this text to be more descriptive and vivid:",
            RewriteStyle::Longer => "Expand this text with more details and elaboration:",
            RewriteStyle::MoreFormal => "Rewrite this text in a more formal, professional tone:",
            RewriteStyle::MoreCasual => "Rewrite this text in a more casual, conversational tone:",
            RewriteStyle::MoreVivid => "Rewrite this text with more vivid imagery and sensory details:",
            RewriteStyle::MoreDirect => "Rewrite this text to be more direct and straightforward:",
            RewriteStyle::MorePoetic => "Rewrite this text in a more poetic, lyrical style:",
            RewriteStyle::ToneShift(tone) => &format!("Rewrite this text in a {} tone:", tone),
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
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors
        let status = response.status();
        let is_success = status.is_success();
        
        if !is_success {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: status.as_u16(),
                message: format!("Claude API error: {}", error_text),
            });
        }
        
        // Parse response
        let completion: ClaudeCompletionResponse = response.json().await
            .map_err(|e| StoryWeaverError::AIProvider {
                provider: "claude".to_string(),
                message: format!("Failed to parse Claude API response: {}", e),
            })?;
        
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
            Err(StoryWeaverError::AIProvider {
                provider: "claude".to_string(),
                message: "No text content returned".to_string(),
            })
        } else {
            Ok(result)
        }
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Claude doesn't have a native embedding API, so we'll return an error
        Err(StoryWeaverError::AIProvider {
            provider: "claude".to_string(),
            message: "Claude does not support embeddings".to_string(),
        })
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

    fn get_provider_name(&self) -> &str {
        "Claude"
    }

    fn supports_image_generation(&self) -> bool {
        false
    }

    async fn rewrite_text_stream(&self, text: &str, style: &RewriteStyle) -> Result<TextStream> {
        let estimated_tokens = (text.len() / 4) as u32 + 500;
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }

        let style_instruction = match style {
            RewriteStyle::Rephrase => "Rephrase this text while keeping the same meaning:",
            RewriteStyle::Shorter => "Rewrite this text to be more concise:",
            RewriteStyle::MoreDescriptive => "Rewrite this text to be more descriptive and vivid:",
            RewriteStyle::Longer => "Expand this text with more details and elaboration:",
            RewriteStyle::MoreFormal => "Rewrite this text in a more formal, professional tone:",
            RewriteStyle::MoreCasual => "Rewrite this text in a more casual, conversational tone:",
            RewriteStyle::MoreVivid => "Rewrite this text with more vivid imagery and sensory details:",
            RewriteStyle::MoreDirect => "Rewrite this text to be more direct and straightforward:",
            RewriteStyle::MorePoetic => "Rewrite this text in a more poetic, lyrical style:",
            RewriteStyle::ToneShift(tone) => &format!("Rewrite this text in a {} tone:", tone),
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
            max_tokens: (text.len() as u32 * 2), // Allow for expansion
            temperature: 0.7,
            system: Some(system),
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .header("anthropic-beta", "messages-2023-12-15-streaming") // Enable streaming
            .query(&[("stream", "true")]) // Add stream parameter
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors
        let status = response.status();
        let is_success = status.is_success();
        
        if !is_success {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: status.as_u16(),
                message: format!("Claude API error: {}", error_text),
            });
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| StoryWeaverError::Network {
                message: format!("Error reading stream chunk: {}", e),
            })?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // Claude sends "data: " prefixed SSE events
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Skip "data: " prefix
                    
                    // Skip empty lines and [DONE] marker
                    if data.is_empty() || data == "[DONE]" {
                        continue;
                    }
                    
                    // Parse the JSON data
                    match serde_json::from_str::<serde_json::Value>(data) {
                        Ok(json) => {
                            // Extract the content from the JSON
                            if let Some(delta) = json.get("delta") {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    text_stream.append(text);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing JSON from stream: {}", e);
                            continue;
                        }
                    }
                }
            }
        }
        
        // Mark the stream as complete
        text_stream.complete();
        
        Ok(text_stream)
    }

    async fn expand_text(&self, text: &str, context: &AIContext) -> Result<String> {
        let prompt = format!("Expand the following text, using the provided context to guide the expansion:\n\nText: {}\n\nContext: {:?}", text, context);
        self.generate_text(&prompt, context).await
    }

    async fn expand_text_stream(&self, text: &str, context: &AIContext) -> Result<TextStream> {
        let estimated_tokens = (text.len() / 4) as u32 + 500;
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            prompt.push_str(&format!("Writing style: {}\n\n", style));
        }
        
        prompt.push_str(&format!("Expand the following text with more details, descriptions, and depth while maintaining the original style and intent:\n\n{}", text));
        
        // Add any key details to include
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("\n\nPlease include these key details in the expansion:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
            }
        }
        
        let system = "You are a skilled writing assistant. Expand the provided text with more details, descriptions, and depth while maintaining the original style and intent.".to_string();
        
        let user_message = ClaudeMessage {
            role: "user".to_string(),
            content: vec![ClaudeContent {
                content_type: "text".to_string(),
                text: prompt,
            }],
        };
        
        let request = ClaudeCompletionRequest {
            model: self.model.clone(),
            messages: vec![user_message],
            max_tokens: 2000, // Allow for significant expansion
            temperature: 0.7,
            system: Some(system),
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .header("anthropic-beta", "messages-2023-12-15-streaming") // Enable streaming
            .query(&[("stream", "true")]) // Add stream parameter
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors
        let status = response.status();
        let is_success = status.is_success();
        
        if !is_success {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: status.as_u16(),
                message: format!("Claude API error: {}", error_text),
            });
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| StoryWeaverError::Network {
                message: format!("Error reading stream chunk: {}", e),
            })?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // Claude sends "data: " prefixed SSE events
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Skip "data: " prefix
                    
                    // Skip empty lines and [DONE] marker
                    if data.is_empty() || data == "[DONE]" {
                        continue;
                    }
                    
                    // Parse the JSON data
                    match serde_json::from_str::<serde_json::Value>(data) {
                        Ok(json) => {
                            // Extract the content from the JSON
                            if let Some(delta) = json.get("delta") {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    text_stream.append(text);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing JSON from stream: {}", e);
                            continue;
                        }
                    }
                }
            }
        }
        
        // Mark the stream as complete
        text_stream.complete();
        
        Ok(text_stream)
    }

    async fn describe_scene(&self, description: &str, context: &AIContext) -> Result<String> {
        let prompt = format!("Describe the following scene in a vivid and engaging way, using the provided context:\n\nScene: {}\n\nContext: {:?}", description, context);
        self.generate_text(&prompt, context).await
    }

    async fn describe_scene_stream(&self, description: &str, context: &AIContext) -> Result<TextStream> {
        let estimated_tokens = (description.len() / 4) as u32 + 500;
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        let mut prompt = String::new();
        
        prompt.push_str("Create a detailed, vivid scene description based on the following information:\n\n");
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            prompt.push_str(&format!("Writing style: {}\n", style));
        }
        
        prompt.push_str(&format!("\nScene to describe:\n{}\n", description));
        
        // Add character context if available
        if let Some(characters) = &context.characters {
            if !characters.is_empty() {
                prompt.push_str("\nCharacters present in the scene:\n");
                for character in characters {
                    prompt.push_str(&format!("- {}", character.name));
                    if let Some(desc) = &character.description {
                        prompt.push_str(&format!(": {}", desc));
                    }
                    prompt.push_str("\n");
                }
            }
        }
        
        // Add location context if available
        if let Some(locations) = &context.locations {
            if !locations.is_empty() {
                prompt.push_str("\nLocation details:\n");
                for location in locations {
                    prompt.push_str(&format!("- {}", location.name));
                    if let Some(desc) = &location.description {
                        prompt.push_str(&format!(": {}", desc));
                    }
                    prompt.push_str("\n");
                }
            }
        }
        
        prompt.push_str("\nCreate a rich, sensory description that brings this scene to life.");
        
        let system = "You are a skilled writing assistant specializing in vivid, sensory descriptions. Create a detailed scene description based on the provided information.".to_string();
        
        let user_message = ClaudeMessage {
            role: "user".to_string(),
            content: vec![ClaudeContent {
                content_type: "text".to_string(),
                text: prompt,
            }],
        };
        
        let request = ClaudeCompletionRequest {
            model: self.model.clone(),
            messages: vec![user_message],
            max_tokens: 2000,
            temperature: 0.7,
            system: Some(system),
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .header("anthropic-beta", "messages-2023-12-15-streaming") // Enable streaming
            .query(&[("stream", "true")]) // Add stream parameter
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors
        let status = response.status();
        let is_success = status.is_success();
        
        if !is_success {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: status.as_u16(),
                message: format!("Claude API error: {}", error_text),
            });
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| StoryWeaverError::Network {
                message: format!("Error reading stream chunk: {}", e),
            })?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // Claude sends "data: " prefixed SSE events
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Skip "data: " prefix
                    
                    // Skip empty lines and [DONE] marker
                    if data.is_empty() || data == "[DONE]" {
                        continue;
                    }
                    
                    // Parse the JSON data
                    match serde_json::from_str::<serde_json::Value>(data) {
                        Ok(json) => {
                            // Extract the content from the JSON
                            if let Some(delta) = json.get("delta") {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    text_stream.append(text);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing JSON from stream: {}", e);
                            continue;
                        }
                    }
                }
            }
        }
        
        // Mark the stream as complete
        text_stream.complete();
        
        Ok(text_stream)
    }

    async fn brainstorm(&self, topic: &str, context: &AIContext) -> Result<Vec<String>> {
        let prompt = format!("Brainstorm a list of ideas for the following topic, taking into account the provided context:\n\nTopic: {}\n\nContext: {:?}", topic, context);
        let response = self.generate_text(&prompt, context).await?;
        Ok(response.lines().map(String::from).collect())
    }

    async fn related_words(&self, word: &str, context: &AIContext) -> Result<Vec<String>> {
        let prompt = format!("Provide a list of related words for the word \"{}\", considering the following context:\n\nContext: {:?}", word, context);
        let response = self.generate_text(&prompt, context).await?;
        Ok(response.split(',').map(|s| s.trim().to_string()).collect())
    }

    async fn quick_edit(&self, text: &str, instruction: &str) -> Result<String> {
        let prompt = format!("Apply the following instruction to the text below:\n\nInstruction: {}\n\nText: {}", instruction, text);
        self.generate_text(&prompt, &AIContext::default()).await
    }

    async fn quick_chat(&self, message: &str, context: &AIContext) -> Result<String> {
        self.generate_text(message, context).await
    }

    async fn quick_chat_stream(&self, message: &str, context: &AIContext) -> Result<TextStream> {
        let estimated_tokens = (message.len() / 4) as u32 + 300;
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        let mut system = "You are StoryWeaver, an AI writing assistant. You help the user with their writing project by answering questions and providing guidance.".to_string();
        
        // Add story context if available
        if let Some(story_context) = &context.story_context {
            system.push_str(&format!("\n\nStory context: {}", story_context));
        }
        
        // Add genre if available
        if let Some(genre) = &context.genre {
            system.push_str(&format!("\n\nGenre: {}", genre));
        }
        
        let user_message = ClaudeMessage {
            role: "user".to_string(),
            content: vec![ClaudeContent {
                content_type: "text".to_string(),
                text: message.to_string(),
            }],
        };
        
        let request = ClaudeCompletionRequest {
            model: self.model.clone(),
            messages: vec![user_message],
            max_tokens: 1000,
            temperature: 0.7,
            system: Some(system),
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .header("anthropic-beta", "messages-2023-12-15-streaming") // Enable streaming
            .query(&[("stream", "true")]) // Add stream parameter
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Claude API: {}", e),
            })?;
        
        // Check for errors
        let status = response.status();
        let is_success = status.is_success();
        
        if !is_success {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "claude".to_string(),
                status_code: status.as_u16(),
                message: format!("Claude API error: {}", error_text),
            });
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| StoryWeaverError::Network {
                message: format!("Error reading stream chunk: {}", e),
            })?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // Claude sends "data: " prefixed SSE events
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..]; // Skip "data: " prefix
                    
                    // Skip empty lines and [DONE] marker
                    if data.is_empty() || data == "[DONE]" {
                        continue;
                    }
                    
                    // Parse the JSON data
                    match serde_json::from_str::<serde_json::Value>(data) {
                        Ok(json) => {
                            // Extract the content from the JSON
                            if let Some(delta) = json.get("delta") {
                                if let Some(text) = delta.get("text").and_then(|t| t.as_str()) {
                                    text_stream.append(text);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("Error parsing JSON from stream: {}", e);
                            continue;
                        }
                    }
                }
            }
        }
        
        // Mark the stream as complete
        text_stream.complete();
        
        Ok(text_stream)
    }

    async fn generate_image(&self, _prompt: &str) -> Result<String> {
        Err(StoryWeaverError::AIProvider {
            provider: "claude".to_string(),
            message: "Claude does not support image generation".to_string(),
        })
    }
}
