//! AI Provider Abstraction Layer for StoryWeaver
//! Defines the AIProvider trait and AIProviderManager for modular AI integrations.

pub mod openai;
pub mod claude;
pub mod gemini;
pub mod write_processor;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// AI Context with more detailed information for better generation
#[derive(Debug, Default, Clone)]
pub struct AIContext {
    // Project and document identifiers
    pub project_id: Option<String>,
    pub document_id: Option<String>,
    
    // Content context
    pub preceding_text: Option<String>,  // Text before cursor for continuation
    pub following_text: Option<String>,  // Text after cursor for context
    pub selected_text: Option<String>,   // Currently selected text (for rewrite, etc.)
    pub story_context: Option<String>,   // Summary of the story/document
    
    // Story Bible elements relevant to current context
    pub characters: Option<Vec<Character>>,
    pub locations: Option<Vec<Location>>,
    pub plot_threads: Option<Vec<PlotThread>>,
    
    // User preferences and settings
    pub user_preferences: Option<HashMap<String, String>>,
    pub writing_style: Option<String>,   // User's preferred writing style
    pub tone: Option<String>,            // Desired tone for generation
    pub creativity_level: Option<u8>,    // 1-10 scale for generation creativity
    
    // Feature-specific context
    pub feature_type: Option<WritingFeature>,
    pub feature_options: Option<HashMap<String, String>>,
    
    // Additional metadata
    pub word_count_target: Option<usize>,
    pub genre: Option<String>,
    pub key_details: Option<Vec<String>>, // Important details to include
}

/// Character information for context
#[derive(Debug, Clone)]
pub struct Character {
    pub name: String,
    pub description: Option<String>,
    pub role: Option<String>,
    pub relevance: Option<u8>, // 1-10 scale of relevance to current context
}

/// Location information for context
#[derive(Debug, Clone)]
pub struct Location {
    pub name: String,
    pub description: Option<String>,
    pub relevance: Option<u8>, // 1-10 scale of relevance to current context
}

/// Plot thread information for context
#[derive(Debug, Clone)]
pub struct PlotThread {
    pub name: String,
    pub description: Option<String>,
    pub relevance: Option<u8>, // 1-10 scale of relevance to current context
}

/// Streaming text response from AI providers
#[derive(Debug, Default, Clone)]
pub struct TextStream {
    pub content: String,
    pub is_complete: bool,
    pub token_count: usize,
}

impl TextStream {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
        // Rough estimate of tokens (characters / 4)
        self.token_count = self.content.len() / 4;
    }

    pub fn complete(&mut self) {
        self.is_complete = true;
    }
}

#[derive(Debug, Clone)]
pub enum RewriteStyle {
    Rephrase,
    Shorter,
    MoreDescriptive,
    Longer,
    MoreFormal,
    MoreCasual,
    MoreVivid,
    MoreDirect,
    MorePoetic,
    ToneShift(String), // Custom tone with description
}

/// Writing feature types
#[derive(Debug, Clone)]
pub enum WritingFeature {
    Write,
    Rewrite(RewriteStyle),
    Expand,
    Describe,
    Brainstorm,
    Visualize,
    RelatedWords,
    QuickEdit,
    QuickChat,
}

#[async_trait]
pub trait AIProvider: Send + Sync {
    // Basic text generation
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> anyhow::Result<String>;
    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> anyhow::Result<TextStream>;
    
    // Rewrite functionality
    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> anyhow::Result<String>;
    async fn rewrite_text_stream(&self, text: &str, style: &RewriteStyle) -> anyhow::Result<TextStream>;
    
    // Expand functionality - add more detail to text
    async fn expand_text(&self, text: &str, context: &AIContext) -> anyhow::Result<String>;
    async fn expand_text_stream(&self, text: &str, context: &AIContext) -> anyhow::Result<TextStream>;
    
    // Describe functionality - generate vivid descriptions
    async fn describe_scene(&self, description: &str, context: &AIContext) -> anyhow::Result<String>;
    async fn describe_scene_stream(&self, description: &str, context: &AIContext) -> anyhow::Result<TextStream>;
    
    // Brainstorm functionality - generate ideas
    async fn brainstorm(&self, topic: &str, context: &AIContext) -> anyhow::Result<Vec<String>>;
    
    // Related words functionality - thesaurus and contextual alternatives
    async fn related_words(&self, word: &str, context: &AIContext) -> anyhow::Result<Vec<String>>;
    
    // Quick tools
    async fn quick_edit(&self, text: &str, instruction: &str) -> anyhow::Result<String>;
    async fn quick_chat(&self, message: &str, context: &AIContext) -> anyhow::Result<String>;
    async fn quick_chat_stream(&self, message: &str, context: &AIContext) -> anyhow::Result<TextStream>;
    
    // Image generation for Visualize feature
    async fn generate_image(&self, prompt: &str) -> anyhow::Result<String>; // Returns URL or base64 image
    
    // Embeddings for semantic search and context relevance
    async fn generate_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>>;
    
    // Provider information
    fn supports_streaming(&self) -> bool;
    fn supports_image_generation(&self) -> bool;
    fn get_context_window(&self) -> usize;
    fn get_model_name(&self) -> &str;
    fn get_provider_name(&self) -> &str;
}

pub use openai::OpenAIProvider;
pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;

pub struct AIProviderManager {
    providers: HashMap<String, Arc<dyn AIProvider>>,
    default_provider: Option<String>,
    // Add rate limiter and other fields as needed
}

impl AIProviderManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: None,
        }
    }

    pub fn register_provider(&mut self, name: String, provider: Arc<dyn AIProvider>) {
        self.providers.insert(name, provider);
    }

    pub fn set_default_provider(&mut self, name: String) {
        self.default_provider = Some(name);
    }

    pub fn get_provider(&self, name: &str) -> Option<&Arc<dyn AIProvider>> {
        self.providers.get(name)
    }

    pub fn get_default_provider(&self) -> Option<&Arc<dyn AIProvider>> {
        match &self.default_provider {
            Some(name) => self.providers.get(name),
            None => None,
        }
    }
    
    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }
    
    pub fn get_default_provider_name(&self) -> Option<String> {
        self.default_provider.clone()
    }
}
