//! Token counting and cost estimation for AI providers
//!
//! This module provides functionality to count tokens and estimate costs
//! for different AI providers and models.

use crate::error::{Result, StoryWeaverError};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub input_cost: f64,
    pub output_cost: f64,
    pub total_cost: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenCountResult {
    pub usage: TokenUsage,
    pub cost: CostEstimate,
    pub provider: String,
    pub model: String,
}

/// Pricing information for different AI models
#[derive(Debug, Clone)]
struct ModelPricing {
    pub input_cost_per_1k: f64,  // Cost per 1000 input tokens
    pub output_cost_per_1k: f64, // Cost per 1000 output tokens
}

pub struct TokenCounter {
    pricing: HashMap<String, ModelPricing>,
}

impl TokenCounter {
    pub fn new() -> Self {
        let mut pricing = HashMap::new();
        
        // OpenAI pricing (as of 2024)
        pricing.insert("gpt-4".to_string(), ModelPricing {
            input_cost_per_1k: 0.03,
            output_cost_per_1k: 0.06,
        });
        pricing.insert("gpt-4-turbo".to_string(), ModelPricing {
            input_cost_per_1k: 0.01,
            output_cost_per_1k: 0.03,
        });
        pricing.insert("gpt-3.5-turbo".to_string(), ModelPricing {
            input_cost_per_1k: 0.0015,
            output_cost_per_1k: 0.002,
        });
        
        // Claude pricing (as of 2024)
        pricing.insert("claude-3-opus".to_string(), ModelPricing {
            input_cost_per_1k: 0.015,
            output_cost_per_1k: 0.075,
        });
        pricing.insert("claude-3-sonnet".to_string(), ModelPricing {
            input_cost_per_1k: 0.003,
            output_cost_per_1k: 0.015,
        });
        pricing.insert("claude-3-haiku".to_string(), ModelPricing {
            input_cost_per_1k: 0.00025,
            output_cost_per_1k: 0.00125,
        });
        
        Self { pricing }
    }
    
    /// Count tokens in text using a simple approximation
    /// This is a basic implementation - in production, you might want to use
    /// provider-specific tokenizers like tiktoken for OpenAI
    pub fn count_tokens(&self, text: &str) -> u32 {
        // Simple approximation: ~4 characters per token for English text
        // This is a rough estimate and should be replaced with proper tokenization
        let char_count = text.chars().count() as f64;
        (char_count / 4.0).ceil() as u32
    }
    
    /// Estimate cost for a given token usage and model
    pub fn estimate_cost(
        &self,
        provider: &str,
        model: &str,
        input_tokens: u32,
        output_tokens: u32,
    ) -> f64 {
        let pricing = self.pricing.get(model)
            .unwrap_or(&ModelPricing {
                input_cost_per_1k: 0.01,  // Default fallback pricing
                output_cost_per_1k: 0.03,
            });
        
        let input_cost = (input_tokens as f64 / 1000.0) * pricing.input_cost_per_1k;
        let output_cost = (output_tokens as f64 / 1000.0) * pricing.output_cost_per_1k;
        input_cost + output_cost
    }
    
    /// Count tokens and estimate cost for a request/response pair
    pub fn analyze_usage(
        &self,
        input_text: &str,
        output_text: &str,
        provider: &str,
        model: &str,
    ) -> Result<TokenCountResult> {
        let input_tokens = self.count_tokens(input_text);
        let output_tokens = self.count_tokens(output_text);
        let total_tokens = input_tokens + output_tokens;
        
        let usage = TokenUsage {
            input_tokens,
            output_tokens,
            total_tokens,
        };
        
        let total_cost = self.estimate_cost(provider, model, input_tokens, output_tokens);
        let input_cost = (input_tokens as f64 / 1000.0) * 0.0015; // Approximate input cost
        let output_cost = total_cost - input_cost;
        
        let cost = CostEstimate {
            input_cost,
            output_cost,
            total_cost,
            currency: "USD".to_string(),
        };
        
        Ok(TokenCountResult {
            usage,
            cost,
            provider: provider.to_string(),
            model: model.to_string(),
        })
    }
    
    /// Get available models and their pricing
    pub fn get_model_pricing(&self) -> &HashMap<String, ModelPricing> {
        &self.pricing
    }
    
    /// Add or update pricing for a model
    pub fn update_model_pricing(
        &mut self,
        model: String,
        input_cost_per_1k: f64,
        output_cost_per_1k: f64,
    ) {
        self.pricing.insert(model, ModelPricing {
            input_cost_per_1k,
            output_cost_per_1k,
        });
    }
}

impl Default for TokenCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_token_counting() {
        let counter = TokenCounter::new();
        let text = "Hello, world! This is a test.";
        let token_count = counter.count_tokens(text);
        
        // Should be approximately 7 tokens (28 chars / 4)
        assert!(token_count >= 6 && token_count <= 8);
    }
    
    #[test]
    fn test_cost_estimation() {
        let counter = TokenCounter::new();
        let cost = counter.estimate_cost("openai", "gpt-3.5-turbo", 1000, 500);
        
        // 1000 input tokens * $0.0015 + 500 output tokens * $0.002
        assert!((cost - 0.0025).abs() < 0.0001);
    }
    
    #[test]
    fn test_usage_analysis() {
        let counter = TokenCounter::new();
        let input = "Generate a story about a dragon.";
        let output = "Once upon a time, there was a mighty dragon who lived in a cave.";
        
        let result = counter.analyze_usage(input, output, "openai", "gpt-3.5-turbo").unwrap();
        
        assert!(result.usage.input_tokens > 0);
        assert!(result.usage.output_tokens > 0);
        assert_eq!(result.usage.total_tokens, result.usage.input_tokens + result.usage.output_tokens);
        assert!(result.cost.total_cost > 0.0);
    }
    
    #[test]
    fn test_unknown_model() {
        let counter = TokenCounter::new();
        let result = counter.estimate_cost(100, 50, "unknown-model");
        
        assert!(result.is_err());
    }
}