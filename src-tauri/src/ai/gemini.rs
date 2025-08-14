//! Google Gemini Provider implementation for StoryWeaver

use super::{AIProvider, AIContext, TextStream, RewriteStyle};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::error::{Result, StoryWeaverError};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

// Rate limiting constants for Gemini
const REQUESTS_PER_MINUTE: u32 = 60;
const TOKENS_PER_MINUTE: u32 = 100000; // Gemini has higher token limits

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
    role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
    #[serde(rename = "safetySettings", skip_serializing_if = "Option::is_none")]
    safety_settings: Option<Vec<SafetySetting>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    #[serde(rename = "topP")]
    top_p: f32,
    #[serde(rename = "topK")]
    top_k: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SafetySetting {
    category: String,
    threshold: String,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
    #[serde(rename = "finishReason")]
    #[allow(dead_code)]
    finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    #[allow(dead_code)]
    prompt_token_count: u32,
    #[serde(rename = "candidatesTokenCount")]
    #[allow(dead_code)]
    candidates_token_count: u32,
    #[serde(rename = "totalTokenCount")]
    total_token_count: u32,
}

pub struct GeminiProvider {
    pub api_key: String,
    pub model: String,
    pub client: reqwest::Client,
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

pub struct RateLimiter {
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
            let wait_time = 60000_u64.saturating_sub(elapsed);
            
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

    fn update_token_usage(&mut self, usage: &UsageMetadata) {
        self.token_count = self.token_count.saturating_add(usage.total_token_count);
    }
}

impl GeminiProvider {
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

    fn get_api_url(&self) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        )
    }

    fn get_streaming_api_url(&self) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?key={}",
            self.model, self.api_key
        )
    }

    fn build_system_content(&self, context: &AIContext) -> GeminiContent {
        let mut system_text = "You are StoryWeaver, an AI writing assistant. Help the user write their story.".to_string();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            system_text.push_str(&format!(" The story is in the {} genre.", genre));
        }
        
        // Add writing style if available
        if let Some(style) = &context.writing_style {
            system_text.push_str(&format!(" Use a {} writing style.", style));
        }
        
        GeminiContent {
            role: "model".to_string(),
            parts: vec![GeminiPart { text: system_text }],
        }
    }

    fn create_generation_config(&self, max_tokens: u32, temperature: f32) -> GenerationConfig {
        GenerationConfig {
            temperature,
            max_output_tokens: max_tokens,
            top_p: 0.95,
            top_k: 40,
        }
    }

    fn create_safety_settings(&self) -> Vec<SafetySetting> {
        vec![
            SafetySetting {
                category: "HARM_CATEGORY_HARASSMENT".to_string(),
                threshold: "BLOCK_ONLY_HIGH".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_HATE_SPEECH".to_string(),
                threshold: "BLOCK_ONLY_HIGH".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_SEXUALLY_EXPLICIT".to_string(),
                threshold: "BLOCK_ONLY_HIGH".to_string(),
            },
            SafetySetting {
                category: "HARM_CATEGORY_DANGEROUS_CONTENT".to_string(),
                threshold: "BLOCK_ONLY_HIGH".to_string(),
            },
        ]
    }
    
    async fn rewrite_text_with_tone(&self, text: &str, tone: &str) -> Result<String> {
        let prompt = format!("Rewrite this text in a {} tone:\n\n{}", tone, text);
        let context = AIContext::default();
        self.generate_text(&prompt, &context).await
    }
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (prompt.len() / 4) as u32 + 500;
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build request
        let system_content = self.build_system_content(context);
        let user_content = GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart { text: prompt.to_string() }],
        };
        
        let request = GeminiRequest {
            contents: vec![system_content, user_content],
            generation_config: self.create_generation_config(1000, 0.7),
            safety_settings: Some(self.create_safety_settings()),
        };
        
        // Make API call
        let response = self.client.post(&self.get_api_url())
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "gemini".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Gemini API: {}", e),
            })?;
        
        // Check for errors first
        let status_code = response.status().as_u16();
        let is_success = response.status().is_success();
        
        // Get response text
        let response_text = response.text().await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "gemini".to_string(),
                status_code: 0,
                message: format!("Failed to read response: {}", e),
            })?;
        
        if !is_success {
            return Err(StoryWeaverError::AIRequest {
                provider: "gemini".to_string(),
                status_code,
                message: format!("Gemini API error: {}", response_text),
            });
        }
        
        // Parse response
        let gemini_response: GeminiResponse = serde_json::from_str(&response_text)
            .map_err(|e| StoryWeaverError::AIProvider {
                provider: "gemini".to_string(),
                message: format!("Failed to parse Gemini API response: {}", e),
            })?;
        
        // Update rate limiter with actual token usage
        if let Some(usage) = &gemini_response.usage_metadata {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.update_token_usage(usage);
        }
        
        // Extract generated text
        if let Some(candidate) = gemini_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                Ok(part.text.clone())
            } else {
                Err(StoryWeaverError::AIProvider {
                    provider: "gemini".to_string(),
                    message: "No text parts in response".to_string(),
                })
            }
        } else {
            Err(StoryWeaverError::AIProvider {
                provider: "gemini".to_string(),
                message: "No candidates returned".to_string(),
            })
        }
    }

    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> Result<TextStream> {
        // Estimate token usage for rate limiting
        let estimated_tokens = (prompt.len() / 4) as u32 + 500;
        
        // Wait if we need to respect rate limits
        {
            let mut rate_limiter = self.rate_limiter.lock().await;
            rate_limiter.wait_if_needed(estimated_tokens).await?;
        }
        
        // Build request
        let system_content = self.build_system_content(context);
        let user_content = GeminiContent {
            role: "user".to_string(),
            parts: vec![GeminiPart { text: prompt.to_string() }],
        };
        
        let request = GeminiRequest {
            contents: vec![system_content, user_content],
            generation_config: self.create_generation_config(1000, 0.7),
            safety_settings: Some(self.create_safety_settings()),
        };
        
        // Make API call with streaming
        let response = self.client.post(&self.get_streaming_api_url())
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| StoryWeaverError::AIRequest {
                provider: "gemini".to_string(),
                status_code: 0,
                message: format!("Failed to send request to Gemini API: {}", e),
            })?;
        
        // Check for errors first
        let status_code = response.status().as_u16();
        let is_success = response.status().is_success();
        
        if !is_success {
            // Only get response text if there's an error
            let response_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoryWeaverError::AIRequest {
                provider: "gemini".to_string(),
                status_code,
                message: format!("Gemini API error: {}", response_text),
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
            
            // Gemini streaming format is a series of JSON objects, each on its own line
            for line in chunk_str.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                // Parse the JSON data
                match serde_json::from_str::<serde_json::Value>(line) {
                    Ok(json) => {
                        // Check if this is a content chunk
                        if let Some(candidates) = json.get("candidates") {
                            if let Some(candidate) = candidates.get(0) {
                                if let Some(content) = candidate.get("content") {
                                    if let Some(parts) = content.get("parts") {
                                        if let Some(part) = parts.get(0) {
                                            if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                                                text_stream.append(text);
                                            }
                                        }
                                    }
                                }
                                
                                // Check if this is the final chunk
                                if let Some(finish_reason) = candidate.get("finishReason") {
                                    if finish_reason.is_string() {
                                        text_stream.complete();
                                    }
                                }
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
        
        // Ensure the stream is marked as complete
        if !text_stream.is_complete {
            text_stream.complete();
        }
        
        Ok(text_stream)
    }

    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> Result<String> {
        // Build prompt based on rewrite style
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
            RewriteStyle::ToneShift(tone) => return self.rewrite_text_with_tone(text, tone).await,
        };
        
        let prompt = format!("{}\n\n{}", style_instruction, text);
        let context = AIContext::default();
        self.generate_text(&prompt, &context).await
    }

    async fn generate_embedding(&self, _text: &str) -> Result<Vec<f32>> {
        // Gemini doesn't have a direct embedding API like OpenAI
        // We'll need to use a different approach or return a placeholder
        // In production, you might use Google's Universal Sentence Encoder or similar
        
        // For now, return a placeholder embedding
        let embedding_size = 768; // Standard size for many models
        Ok(vec![0.0; embedding_size])
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    fn get_context_window(&self) -> usize {
        match self.model.as_str() {
            "gemini-1.5-pro" => 1048576, // 1M tokens
            "gemini-1.5-flash" => 1048576, // 1M tokens
            "gemini-pro" => 32768,
            _ => 8192, // Default for unknown models
        }
    }

    fn get_model_name(&self) -> &str {
        &self.model
    }
    
    fn get_provider_name(&self) -> &str {
        "Gemini"
    }
    
    fn supports_image_generation(&self) -> bool {
        false // Gemini doesn't directly support image generation like DALL-E
    }
    
    async fn rewrite_text_stream(&self, text: &str, style: &RewriteStyle) -> Result<TextStream> {
        // Build prompt based on rewrite style
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
            RewriteStyle::ToneShift(tone) => {
                let prompt = format!("Rewrite this text in a {} tone:\n\n{}", tone, text);
                let context = AIContext::default();
                return self.generate_text_stream(&prompt, &context).await;
            },
        };
        
        let prompt = format!("{}\n\n{}", style_instruction, text);
        let context = AIContext::default();
        self.generate_text_stream(&prompt, &context).await
    }
    
    async fn expand_text(&self, text: &str, context: &AIContext) -> Result<String> {
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
        
        self.generate_text(&prompt, context).await
    }
    
    async fn expand_text_stream(&self, text: &str, context: &AIContext) -> Result<TextStream> {
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
        
        self.generate_text_stream(&prompt, context).await
    }
    
    async fn describe_scene(&self, description: &str, context: &AIContext) -> Result<String> {
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
        
        self.generate_text(&prompt, context).await
    }
    
    async fn describe_scene_stream(&self, description: &str, context: &AIContext) -> Result<TextStream> {
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
        
        self.generate_text_stream(&prompt, context).await
    }
    
    async fn brainstorm(&self, topic: &str, context: &AIContext) -> Result<Vec<String>> {
        let mut prompt = String::new();
        
        // Add genre context if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        prompt.push_str(&format!("Generate at least 10 creative ideas for the following topic:\n{}\n\n", topic));
        
        // Add any key details to consider
        if let Some(details) = &context.key_details {
            if !details.is_empty() {
                prompt.push_str("Consider these key details in your brainstorming:\n");
                for detail in details {
                    prompt.push_str(&format!("- {}\n", detail));
                }
                prompt.push_str("\n");
            }
        }
        
        prompt.push_str("Format each idea as a numbered list item (1. idea, 2. idea, etc.)");
        
        let response = self.generate_text(&prompt, context).await?;
        
        // Parse the numbered list into separate ideas
        let ideas: Vec<String> = response
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                
                // Check if line starts with a number
                if line.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                    // Remove the number prefix and return the idea
                    let parts: Vec<&str> = line.splitn(2, |c: char| c == '.' || c == ')' || c == ':').collect();
                    if parts.len() > 1 {
                        Some(parts[1].trim().to_string())
                    } else {
                        Some(line.to_string())
                    }
                } else {
                    Some(line.to_string())
                }
            })
            .collect();
        
        Ok(ideas)
    }
    
    async fn related_words(&self, word: &str, context: &AIContext) -> Result<Vec<String>> {
        let mut prompt = String::new();
        
        prompt.push_str(&format!("Provide synonyms and related words for: {}\n", word));
        
        // Add context from the document if available
        if let Some(preceding) = &context.preceding_text {
            if !preceding.is_empty() {
                let context_snippet = if preceding.len() > 200 {
                    &preceding[preceding.len() - 200..]
                } else {
                    preceding
                };
                prompt.push_str(&format!("\nContext: {}\n", context_snippet));
            }
        }
        
        prompt.push_str("\nProvide a comma-separated list of related words, synonyms, and contextually relevant alternatives.");
        
        let response = self.generate_text(&prompt, context).await?;
        
        // Parse the comma-separated list into individual words
        let words: Vec<String> = response
            .split(',')
            .map(|word| word.trim().to_string())
            .filter(|word| !word.is_empty())
            .collect();
        
        Ok(words)
    }
    
    async fn quick_edit(&self, text: &str, instruction: &str) -> Result<String> {
        let prompt = format!(
            "Edit the following text according to these instructions. Return only the edited text without explanations.\n\nText to edit:\n{}\n\nInstructions:\n{}",
            text, instruction
        );
        
        let context = AIContext::default();
        self.generate_text(&prompt, &context).await
    }
    
    async fn quick_chat(&self, message: &str, context: &AIContext) -> Result<String> {
        let mut prompt = String::new();
        
        // Add story context if available
        if let Some(story_context) = &context.story_context {
            prompt.push_str(&format!("Story context: {}\n\n", story_context));
        }
        
        // Add genre if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        prompt.push_str(&format!("User question: {}", message));
        
        self.generate_text(&prompt, context).await
    }
    
    async fn quick_chat_stream(&self, message: &str, context: &AIContext) -> Result<TextStream> {
        let mut prompt = String::new();
        
        // Add story context if available
        if let Some(story_context) = &context.story_context {
            prompt.push_str(&format!("Story context: {}\n\n", story_context));
        }
        
        // Add genre if available
        if let Some(genre) = &context.genre {
            prompt.push_str(&format!("Genre: {}\n\n", genre));
        }
        
        prompt.push_str(&format!("User question: {}", message));
        
        self.generate_text_stream(&prompt, context).await
    }
    
    async fn generate_image(&self, _prompt: &str) -> Result<String> {
        // Gemini doesn't directly support image generation
        // Return an error indicating this feature is not supported
        Err(StoryWeaverError::NotSupported {
            operation: "generate_image - Image generation is not supported by Gemini".to_string(),
        })
    }
}
