 //! AI Provider Abstraction Layer for StoryWeaver
//! Defines the AIProvider trait and AIProviderManager for modular AI integrations.

pub mod openai;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AIContext {
    // Add fields as needed for context (e.g., user, document, settings)
}

pub struct TextStream; // Placeholder for streaming responses

pub enum RewriteStyle {
    Rephrase,
    Shorter,
    MoreDescriptive,
    // Add more styles as needed
}

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> anyhow::Result<String>;
    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> anyhow::Result<TextStream>;
    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> anyhow::Result<String>;
    async fn generate_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>>;
    fn supports_streaming(&self) -> bool;
    fn get_context_window(&self) -> usize;
    fn get_model_name(&self) -> &str;
}

pub use openai::OpenAIProvider;

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
}
