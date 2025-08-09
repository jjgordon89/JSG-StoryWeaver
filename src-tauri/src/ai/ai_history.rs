//! AI History module for tracking AI interactions and usage patterns

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Represents a single AI interaction in the history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInteraction {
    pub id: String,
    pub session_id: Option<String>,
    pub project_id: Option<String>,
    pub feature_type: String,
    pub prompt: String,
    pub response: String,
    pub model_used: String,
    pub tokens_used: Option<u32>,
    pub credits_consumed: Option<f64>,
    pub duration_ms: Option<u64>,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// AI History manager for tracking and retrieving interactions
#[derive(Debug, Default)]
pub struct AIHistoryManager {
    interactions: Vec<AIInteraction>,
}

impl AIHistoryManager {
    pub fn new() -> Self {
        Self {
            interactions: Vec::new(),
        }
    }

    /// Add a new interaction to the history
    pub fn add_interaction(&mut self, interaction: AIInteraction) {
        self.interactions.push(interaction);
    }

    /// Get all interactions for a specific project
    pub fn get_project_interactions(&self, project_id: &str) -> Vec<&AIInteraction> {
        self.interactions
            .iter()
            .filter(|interaction| {
                interaction.project_id.as_ref().map_or(false, |id| id == project_id)
            })
            .collect()
    }

    /// Get interactions by feature type
    pub fn get_interactions_by_feature(&self, feature_type: &str) -> Vec<&AIInteraction> {
        self.interactions
            .iter()
            .filter(|interaction| interaction.feature_type == feature_type)
            .collect()
    }

    /// Get recent interactions (last N)
    pub fn get_recent_interactions(&self, limit: usize) -> Vec<&AIInteraction> {
        let mut interactions = self.interactions.iter().collect::<Vec<_>>();
        interactions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        interactions.into_iter().take(limit).collect()
    }

    /// Clear all interactions
    pub fn clear(&mut self) {
        self.interactions.clear();
    }

    /// Get total interactions count
    pub fn count(&self) -> usize {
        self.interactions.len()
    }

    /// Get total tokens used
    pub fn total_tokens_used(&self) -> u32 {
        self.interactions
            .iter()
            .filter_map(|interaction| interaction.tokens_used)
            .sum()
    }

    /// Get total credits consumed
    pub fn total_credits_consumed(&self) -> f64 {
        self.interactions
            .iter()
            .filter_map(|interaction| interaction.credits_consumed)
            .sum()
    }
}

/// Builder for creating AI interactions
#[derive(Debug, Default)]
pub struct AIInteractionBuilder {
    id: Option<String>,
    session_id: Option<String>,
    project_id: Option<String>,
    feature_type: Option<String>,
    prompt: Option<String>,
    response: Option<String>,
    model_used: Option<String>,
    tokens_used: Option<u32>,
    credits_consumed: Option<f64>,
    duration_ms: Option<u64>,
    metadata: HashMap<String, String>,
}

impl AIInteractionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn project_id(mut self, project_id: String) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn feature_type(mut self, feature_type: String) -> Self {
        self.feature_type = Some(feature_type);
        self
    }

    pub fn prompt(mut self, prompt: String) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn response(mut self, response: String) -> Self {
        self.response = Some(response);
        self
    }

    pub fn model_used(mut self, model_used: String) -> Self {
        self.model_used = Some(model_used);
        self
    }

    pub fn tokens_used(mut self, tokens_used: u32) -> Self {
        self.tokens_used = Some(tokens_used);
        self
    }

    pub fn credits_consumed(mut self, credits_consumed: f64) -> Self {
        self.credits_consumed = Some(credits_consumed);
        self
    }

    pub fn duration_ms(mut self, duration_ms: u64) -> Self {
        self.duration_ms = Some(duration_ms);
        self
    }

    pub fn metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn build(self) -> Result<AIInteraction, String> {
        let id = self.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let feature_type = self.feature_type.ok_or("feature_type is required")?;
        let prompt = self.prompt.ok_or("prompt is required")?;
        let response = self.response.ok_or("response is required")?;
        let model_used = self.model_used.ok_or("model_used is required")?;

        Ok(AIInteraction {
            id,
            session_id: self.session_id,
            project_id: self.project_id,
            feature_type,
            prompt,
            response,
            model_used,
            tokens_used: self.tokens_used,
            credits_consumed: self.credits_consumed,
            duration_ms: self.duration_ms,
            metadata: self.metadata,
            created_at: Utc::now(),
        })
    }
}