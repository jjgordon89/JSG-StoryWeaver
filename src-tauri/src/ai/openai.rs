//! OpenAI Provider implementation for StoryWeaver

use super::{AIProvider, AIContext, TextStream, RewriteStyle};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

// Rate limiting constants
const REQUESTS_PER_MINUTE: u32 = 60;
const TOKENS_PER_MINUTE: u32 = 90000;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: Option<u32>,
    stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatCompletionChoice {
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatCompletionResponse {
    id: String,
    choices: Vec<ChatCompletionChoice>,
    usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmbeddingRequest {
    model: String,
    input: String,
}

#[derive(Debug, Clone, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[derive(Debug, Clone, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
    usage: Option<TokenUsage>,
}

pub struct OpenAIProvider {
    pub api_key: String,
    pub model: String,
    pub embedding_model: String,
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
        self.token_count = self.token_count.saturating_add(usage.total_tokens);
    }
}

impl OpenAIProvider {
    pub fn new(api_key: String, model: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .unwrap_or_default();
            
        Self {
            api_key,
            model,
            embedding_model: "text-embedding-ada-002".to_string(),
            client,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
        }
    }

    fn build_system_message(&self, context: &AIContext) -> ChatMessage {
        // Build a system message based on the context
        // This could include information about the document, user preferences, etc.
        ChatMessage {
            role: "system".to_string(),
            content: "You are StoryWeaver, an AI writing assistant. Help the user write their story.".to_string(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (prompt.len() / 4) as u32 + 500; // Rough estimate
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build request
        let system_message = self.build_system_message(context);
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(1000),
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
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
        let system_message = self.build_system_message(context);
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(1000),
            stream: true, // Enable streaming
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error reading stream chunk")?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // OpenAI sends "data: " prefixed SSE events
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
                            if let Some(content) = json
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                                .and_then(|content| content.as_str())
                            {
                                text_stream.append(content);
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
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32 + 500; // Rough estimate
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build prompt based on rewrite style
        let style_instruction = match style {
            RewriteStyle::Rephrase => "Rephrase this text while keeping the same meaning:".to_string(),
            RewriteStyle::Shorter => "Rewrite this text to be more concise:".to_string(),
            RewriteStyle::MoreDescriptive => "Rewrite this text to be more descriptive and vivid:".to_string(),
            RewriteStyle::Longer => "Expand this text with more details and elaboration:".to_string(),
            RewriteStyle::MoreFormal => "Rewrite this text in a more formal, professional tone:".to_string(),
            RewriteStyle::MoreCasual => "Rewrite this text in a more casual, conversational tone:".to_string(),
            RewriteStyle::MoreVivid => "Rewrite this text with more vivid imagery and sensory details:".to_string(),
            RewriteStyle::MoreDirect => "Rewrite this text to be more direct and straightforward:".to_string(),
            RewriteStyle::MorePoetic => "Rewrite this text in a more poetic, lyrical style:".to_string(),
            RewriteStyle::ToneShift(tone) => format!("Rewrite this text in a {} tone:", tone),
        };
        
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: format!("You are a helpful writing assistant. {}", style_instruction),
        };
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: text.to_string(),
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(text.len() as u32 / 2), // Limit token usage based on input
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32; // Rough estimate
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build request
        let request = EmbeddingRequest {
            model: self.embedding_model.clone(),
            input: text.to_string(),
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let embedding_response: EmbeddingResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &embedding_response.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract embedding
        if let Some(data) = embedding_response.data.first() {
            Ok(data.embedding.clone())
        } else {
            Err(anyhow::anyhow!("No embedding data returned"))
        }
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn get_context_window(&self) -> usize {
        match self.model.as_str() {
            "gpt-4-turbo" => 128000,
            "gpt-4" => 8192,
            "gpt-3.5-turbo" => 16385,
            _ => 4096, // Default for unknown models
        }
    }

    fn get_model_name(&self) -> &str {
        &self.model
    }
    
    fn get_provider_name(&self) -> &str {
        "OpenAI"
    }
    
    fn supports_image_generation(&self) -> bool {
        true // OpenAI supports DALL-E for image generation
    }
    
    // Implement the new methods required by the AIProvider trait
    
    async fn rewrite_text_stream(&self, text: &str, style: &RewriteStyle) -> Result<TextStream> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32 + 500;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build prompt based on rewrite style
        let style_instruction = match style {
            RewriteStyle::Rephrase => "Rephrase this text while keeping the same meaning:".to_string(),
            RewriteStyle::Shorter => "Rewrite this text to be more concise:".to_string(),
            RewriteStyle::MoreDescriptive => "Rewrite this text to be more descriptive and vivid:".to_string(),
            RewriteStyle::Longer => "Expand this text with more details and elaboration:".to_string(),
            RewriteStyle::MoreFormal => "Rewrite this text in a more formal, professional tone:".to_string(),
            RewriteStyle::MoreCasual => "Rewrite this text in a more casual, conversational tone:".to_string(),
            RewriteStyle::MoreVivid => "Rewrite this text with more vivid imagery and sensory details:".to_string(),
            RewriteStyle::MoreDirect => "Rewrite this text to be more direct and straightforward:".to_string(),
            RewriteStyle::MorePoetic => "Rewrite this text in a more poetic, lyrical style:".to_string(),
            RewriteStyle::ToneShift(tone) => format!("Rewrite this text in a {} tone:", tone),
        };
        
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: format!("You are a helpful writing assistant. {}", style_instruction),
        };
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: text.to_string(),
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(text.len() as u32 * 2), // Allow for expansion
            stream: true, // Enable streaming
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error reading stream chunk")?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // OpenAI sends "data: " prefixed SSE events
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
                            if let Some(content) = json
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                                .and_then(|content| content.as_str())
                            {
                                text_stream.append(content);
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
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32 + 500;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a skilled writing assistant. Expand the following text with more details, descriptions, and depth while maintaining the original style and intent.".to_string(),
        };
        
        // Build user message with context
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            prompt.push_str(&format!("Writing style: {}\n\n", style));
        }
        
        // Add the text to expand
        prompt.push_str(&format!("Text to expand:\n{}\n\n", text));
        
        // Add any key details to include
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("Please include these key details in the expansion:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
                prompt.push_str("\n");
            }
        }
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(2000), // Allow for significant expansion
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }
    
    async fn expand_text_stream(&self, text: &str, context: &AIContext) -> Result<TextStream> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32 + 500;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a skilled writing assistant. Expand the following text with more details, descriptions, and depth while maintaining the original style and intent.".to_string(),
        };
        
        // Build user message with context
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            prompt.push_str(&format!("Writing style: {}\n\n", style));
        }
        
        // Add the text to expand
        prompt.push_str(&format!("Text to expand:\n{}\n\n", text));
        
        // Add any key details to include
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("Please include these key details in the expansion:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
                prompt.push_str("\n");
            }
        }
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(2000), // Allow for significant expansion
            stream: true, // Enable streaming
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error reading stream chunk")?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // OpenAI sends "data: " prefixed SSE events
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
                            if let Some(content) = json
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                                .and_then(|content| content.as_str())
                            {
                                text_stream.append(content);
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
        // Estimate token usage for rate limiting
        let estimated_tokens = (description.len() / 4) as u32 + 500;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a skilled writing assistant specializing in vivid, sensory descriptions. Create a detailed scene description based on the provided information.".to_string(),
        };
        
        // Build user message with context
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            prompt.push_str(&format!("Writing style: {}\n\n", style));
        }
        
        // Add the scene to describe
        prompt.push_str(&format!("Scene to describe:\n{}\n\n", description));
        
        // Add any key details to include
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("Please include these key details in the description:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
                prompt.push_str("\n");
            }
        }
        
        // Add character context if available
        if let Some(characters) = &context.characters {
            if !characters.is_empty() {
                prompt.push_str("Characters present in the scene:\n");
                for character in characters {
                    prompt.push_str(&format!("- {}", character.name));
                    if let Some(desc) = &character.description {
                        prompt.push_str(&format!(": {}", desc));
                    }
                    prompt.push_str("\n");
                }
                prompt.push_str("\n");
            }
        }
        
        // Add location context if available
        if let Some(locations) = &context.locations {
            if !locations.is_empty() {
                prompt.push_str("Location details:\n");
                for location in locations {
                    prompt.push_str(&format!("- {}", location.name));
                    if let Some(desc) = &location.description {
                        prompt.push_str(&format!(": {}", desc));
                    }
                    prompt.push_str("\n");
                }
                prompt.push_str("\n");
            }
        }
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(2000),
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }
    
    async fn describe_scene_stream(&self, description: &str, context: &AIContext) -> Result<TextStream> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (description.len() / 4) as u32 + 500;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a skilled writing assistant specializing in vivid, sensory descriptions. Create a detailed scene description based on the provided information.".to_string(),
        };
        
        // Build user message with context
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            prompt.push_str(&format!("Writing style: {}\n\n", style));
        }
        
        // Add the scene to describe
        prompt.push_str(&format!("Scene to describe:\n{}\n\n", description));
        
        // Add any key details to include
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("Please include these key details in the description:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
                prompt.push_str("\n");
            }
        }
        
        // Add character context if available
        if let Some(characters) = &context.characters {
            if !characters.is_empty() {
                prompt.push_str("Characters present in the scene:\n");
                for character in characters {
                    prompt.push_str(&format!("- {}", character.name));
                    if let Some(desc) = &character.description {
                        prompt.push_str(&format!(": {}", desc));
                    }
                    prompt.push_str("\n");
                }
                prompt.push_str("\n");
            }
        }
        
        // Add location context if available
        if let Some(locations) = &context.locations {
            if !locations.is_empty() {
                prompt.push_str("Location details:\n");
                for location in locations {
                    prompt.push_str(&format!("- {}", location.name));
                    if let Some(desc) = &location.description {
                        prompt.push_str(&format!(": {}", desc));
                    }
                    prompt.push_str("\n");
                }
                prompt.push_str("\n");
            }
        }
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(2000),
            stream: true, // Enable streaming
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error reading stream chunk")?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // OpenAI sends "data: " prefixed SSE events
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
                            if let Some(content) = json
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                                .and_then(|content| content.as_str())
                            {
                                text_stream.append(content);
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
        // Estimate token usage for rate limiting
        let estimated_tokens = (topic.len() / 4) as u32 + 500;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a creative writing assistant. Generate a list of ideas based on the provided topic. Each idea should be distinct and presented as a separate item in a numbered list.".to_string(),
        };
        
        // Build user message with context
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        // Add the topic to brainstorm
        prompt.push_str(&format!("Topic to brainstorm ideas for:\n{}\n\n", topic));
        
        // Add any key details to consider
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("Please consider these key details in your brainstorming:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
                prompt.push_str("\n");
            }
        }
        
        prompt.push_str("Generate at least 10 creative ideas. Format each idea as a numbered list item.");
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.8, // Higher temperature for more creative ideas
            max_tokens: Some(2000),
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text and parse into a list of ideas
        if let Some(choice) = completion.choices.first() {
            let content = &choice.message.content;
            
            // Parse the numbered list into separate ideas
            let ideas: Vec<String> = content
                .lines()
                .filter_map(|line| {
                    // Look for lines that start with a number followed by a period or parenthesis
                    if line.trim().is_empty() {
                        return None;
                    }
                    
                    let line = line.trim();
                    
                    // Check if line starts with a number (with or without period/parenthesis)
                    if line.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                        // Remove the number prefix and return the idea
                        let parts: Vec<&str> = line.splitn(2, |c: char| c == '.' || c == ')' || c == ':').collect();
                        if parts.len() > 1 {
                            Some(parts[1].trim().to_string())
                        } else {
                            Some(line.to_string())
                        }
                    } else {
                        // Include lines that don't match the pattern as well
                        Some(line.to_string())
                    }
                })
                .collect();
            
            Ok(ideas)
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }
    
    async fn related_words(&self, word: &str, context: &AIContext) -> Result<Vec<String>> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (word.len() / 4) as u32 + 200;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a helpful writing assistant with expertise in vocabulary and language. Provide related words, synonyms, and contextually relevant alternatives for the given word.".to_string(),
        };
        
        // Build user message with context
        let mut prompt = String::new();
        
        // Add the word to find related words for
        prompt.push_str(&format!("Word: {}\n\n", word));
        
        // Add context from the document if available
        if let Some(preceding) = &context.preceding_text {
            if !preceding.is_empty() {
                let context_snippet = if preceding.len() > 200 {
                    // Take the last 200 characters for context
                    &preceding[preceding.len() - 200..]
                } else {
                    preceding
                };
                prompt.push_str(&format!("Context: {}\n\n", context_snippet));
            }
        }
        
        prompt.push_str("Please provide a list of related words, synonyms, and contextually relevant alternatives. Format the response as a simple comma-separated list without explanations.");
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.5,
            max_tokens: Some(500),
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text and parse into a list of words
        if let Some(choice) = completion.choices.first() {
            let content = &choice.message.content;
            
            // Parse the comma-separated list into individual words
            let words: Vec<String> = content
                .split(',')
                .map(|word| word.trim().to_string())
                .filter(|word| !word.is_empty())
                .collect();
            
            Ok(words)
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }
    
    async fn quick_edit(&self, text: &str, instruction: &str) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (text.len() / 4) as u32 + (instruction.len() / 4) as u32 + 300;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: "You are a helpful writing assistant. Edit the provided text according to the user's instructions. Return only the edited text without explanations or comments.".to_string(),
        };
        
        // Build user message
        let prompt = format!("Text to edit:\n{}\n\nInstructions:\n{}", text, instruction);
        
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: prompt,
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.5,
            max_tokens: Some((text.len() as u32 * 2).max(500)), // Allow for expansion
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }
    
    async fn quick_chat(&self, message: &str, context: &AIContext) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (message.len() / 4) as u32 + 300;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let mut system_content = "You are StoryWeaver, an AI writing assistant. You help the user with their writing project by answering questions and providing guidance.".to_string();
        
        // Add story context if available
        if let Some(story_context) = &context.story_context {
            system_content.push_str(&format!("\n\nStory context: {}", story_context));
        }
        
        // Add genre if available
        if let Some(genre) = &context.genre {
            system_content.push_str(&format!("\n\nGenre: {}", genre));
        }
        
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: system_content,
        };
        
        // Build user message
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(1000),
            stream: false,
        };
        
        // Make API call
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Parse response
        let completion: ChatCompletionResponse = response.json().await
            .context("Failed to parse OpenAI API response")?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &completion.usage {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(choice) = completion.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(anyhow::anyhow!("No completion choices returned"))
        }
    }
    
    async fn quick_chat_stream(&self, message: &str, context: &AIContext) -> Result<TextStream> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (message.len() / 4) as u32 + 300;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build system message
        let mut system_content = "You are StoryWeaver, an AI writing assistant. You help the user with their writing project by answering questions and providing guidance.".to_string();
        
        // Add story context if available
        if let Some(story_context) = &context.story_context {
            system_content.push_str(&format!("\n\nStory context: {}", story_context));
        }
        
        // Add genre if available
        if let Some(genre) = &context.genre {
            system_content.push_str(&format!("\n\nGenre: {}", genre));
        }
        
        let system_message = ChatMessage {
            role: "system".to_string(),
            content: system_content,
        };
        
        // Build user message
        let user_message = ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        };
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![system_message, user_message],
            temperature: 0.7,
            max_tokens: Some(1000),
            stream: true, // Enable streaming
        };
        
        // Make API call with streaming
        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to OpenAI API")?;
        
        // Check for errors
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow::anyhow!("OpenAI API error: {}", error_text));
        }
        
        // Create a new TextStream
        let mut text_stream = TextStream::new();
        
        // Process the streaming response
        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;
        
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error reading stream chunk")?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            
            // OpenAI sends "data: " prefixed SSE events
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
                            if let Some(content) = json
                                .get("choices")
                                .and_then(|choices| choices.get(0))
                                .and_then(|choice| choice.get("delta"))
                                .and_then(|delta| delta.get("content"))
                                .and_then(|content| content.as_str())
                            {
                                text_stream.append(content);
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
    
    async fn generate_image(&self, prompt: &str) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (prompt.len() / 4) as u32 + 100;
        
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // In a real implementation, this would call the DALL-E API
        // For now, return a placeholder
        
        // Simulate API delay
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        
        Ok("https://example.com/generated-image.png".to_string())
    }
}
