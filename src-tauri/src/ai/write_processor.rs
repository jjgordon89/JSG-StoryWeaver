//! Write Processor for handling AI-powered writing features

use super::{AIProvider, AIContext};
use crate::database::DbPool;
use crate::database::operations::{DocumentOps, CharacterOps, LocationOps, WorldElementOps};
use crate::database::models::{Character, Location, WorldElement, CharacterRole};
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
    pub include_story_bible: Option<bool>,
    pub max_story_bible_tokens: Option<usize>,
    pub story_bible_priority: Option<StoryBiblePriority>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoryBiblePriority {
    Characters,
    Locations,
    Lore,
    Balanced,
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
        // Build context from the document with settings
        let context = self.context_builder
            .build_write_context_with_settings(document_id, cursor_position, 1000, db_pool, Some(settings))
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
        // Build context from the document with settings
        let mut context = self.context_builder
            .build_write_context_with_settings(document_id, 0, 1000, db_pool, Some(settings))
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
        // Build context from the document with settings
        let mut context = self.context_builder
            .build_write_context_with_settings(document_id, cursor_position, 1000, db_pool, Some(settings))
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

/// Story Bible budget allocation for token management
#[derive(Debug, Clone)]
struct StoryBibleBudget {
    #[allow(dead_code)]
    total_tokens: usize,
    characters_tokens: usize,
    locations_tokens: usize,
    lore_tokens: usize,
}

impl StoryBibleBudget {
    fn new(max_tokens: usize, priority: &StoryBiblePriority) -> Self {
        match priority {
            StoryBiblePriority::Characters => Self {
                total_tokens: max_tokens,
                characters_tokens: (max_tokens as f32 * 0.7) as usize,
                locations_tokens: (max_tokens as f32 * 0.2) as usize,
                lore_tokens: (max_tokens as f32 * 0.1) as usize,
            },
            StoryBiblePriority::Locations => Self {
                total_tokens: max_tokens,
                characters_tokens: (max_tokens as f32 * 0.2) as usize,
                locations_tokens: (max_tokens as f32 * 0.7) as usize,
                lore_tokens: (max_tokens as f32 * 0.1) as usize,
            },
            StoryBiblePriority::Lore => Self {
                total_tokens: max_tokens,
                characters_tokens: (max_tokens as f32 * 0.2) as usize,
                locations_tokens: (max_tokens as f32 * 0.1) as usize,
                lore_tokens: (max_tokens as f32 * 0.7) as usize,
            },
            StoryBiblePriority::Balanced => Self {
                total_tokens: max_tokens,
                characters_tokens: (max_tokens as f32 * 0.5) as usize,
                locations_tokens: (max_tokens as f32 * 0.3) as usize,
                lore_tokens: (max_tokens as f32 * 0.2) as usize,
            },
        }
    }
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
        self.build_write_context_with_settings(
            document_id,
            cursor_position,
            context_window,
            db_pool,
            None,
        ).await
    }

    /// Build context for writing features with settings
    pub async fn build_write_context_with_settings(
        &self,
        document_id: i32,
        cursor_position: usize,
        context_window: usize,
        db_pool: &DbPool,
        settings: Option<&WriteSettings>,
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
        
        // Add Story Bible elements if enabled
        if let Some(settings) = settings {
            if settings.include_story_bible.unwrap_or(true) {
                self.enrich_with_story_bible(&mut context, &document.project_id, settings, db_pool).await?;
            }
        } else {
            // Default: include Story Bible with balanced priority
            let default_settings = WriteSettings {
                mode: WriteMode::Auto,
                creativity_level: 5,
                word_count_target: None,
                tone: None,
                include_key_details: Vec::new(),
                include_story_bible: Some(true),
                max_story_bible_tokens: Some(1000),
                story_bible_priority: Some(StoryBiblePriority::Balanced),
            };
            self.enrich_with_story_bible(&mut context, &document.project_id, &default_settings, db_pool).await?;
        }
        
        Ok(context)
    }

    /// Enrich context with Story Bible elements
    async fn enrich_with_story_bible(
        &self,
        context: &mut AIContext,
        project_id: &str,
        settings: &WriteSettings,
        db_pool: &DbPool,
    ) -> Result<()> {
        let max_tokens = settings.max_story_bible_tokens.unwrap_or(1000);
        let priority = settings.story_bible_priority.as_ref().unwrap_or(&StoryBiblePriority::Balanced);
        let budget = StoryBibleBudget::new(max_tokens, priority);

        // Get relevant characters
        if budget.characters_tokens > 0 {
            match self.get_relevant_characters(project_id, budget.characters_tokens, db_pool).await {
                Ok(characters) => {
                    if !characters.is_empty() {
                        context.characters = Some(characters);
                    }
                }
                Err(e) => {
                    // Log error but don't fail the entire operation
                    eprintln!("Warning: Failed to get characters for Story Bible context: {}", e);
                }
            }
        }

        // Get relevant locations
        if budget.locations_tokens > 0 {
            match self.get_relevant_locations(project_id, budget.locations_tokens, db_pool).await {
                Ok(locations) => {
                    if !locations.is_empty() {
                        context.locations = Some(locations);
                    }
                }
                Err(e) => {
                    // Log error but don't fail the entire operation
                    eprintln!("Warning: Failed to get locations for Story Bible context: {}", e);
                }
            }
        }

        // Get relevant lore (world elements)
        if budget.lore_tokens > 0 {
            match self.get_relevant_lore(project_id, budget.lore_tokens, db_pool).await {
                Ok(lore_elements) => {
                    if !lore_elements.is_empty() {
                        // Store world elements in plot_threads field for now
                        // In a future update, we could add a dedicated lore field to AIContext
                        context.plot_threads = Some(lore_elements.into_iter().map(|we| {
                            // Convert WorldElement to PlotThread for compatibility
                            // This is a temporary solution until AIContext is updated
                            crate::database::models::PlotThread {
                                id: we.id,
                                project_id: we.project_id.unwrap_or_default(),
                                name: we.name,
                                description: we.description,
                                status: crate::database::models::PlotThreadStatus::Active,
                                priority: crate::database::models::ThreadPriority::Background,
                                characters_involved: "[]".to_string(),
                                documents_involved: "[]".to_string(),
                                visibility: crate::database::models::VisibilityLevel::Relevant,
                                created_at: we.created_at,
                                updated_at: we.updated_at,
                            }
                        }).collect());
                    }
                }
                Err(e) => {
                    // Log error but don't fail the entire operation
                    eprintln!("Warning: Failed to get lore for Story Bible context: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Get relevant characters for the current context
    async fn get_relevant_characters(
        &self,
        project_id: &str,
        token_budget: usize,
        db_pool: &DbPool,
    ) -> Result<Vec<Character>> {
        // Get all characters for the project
        let mut characters = CharacterOps::get_by_project(db_pool, project_id).await?;

        // Sort by priority: Protagonist > Antagonist > Supporting > Minor > Background
        characters.sort_by(|a, b| {
            let a_priority = match a.role {
                CharacterRole::Protagonist => 0,
                CharacterRole::Antagonist => 1,
                CharacterRole::Supporting => 2,
                CharacterRole::Minor => 3,
                CharacterRole::Background => 4,
            };
            let b_priority = match b.role {
                CharacterRole::Protagonist => 0,
                CharacterRole::Antagonist => 1,
                CharacterRole::Supporting => 2,
                CharacterRole::Minor => 3,
                CharacterRole::Background => 4,
            };
            a_priority.cmp(&b_priority)
        });

        // Apply token budget by selecting characters until we exceed the budget
        let mut selected_characters = Vec::new();
        let mut current_tokens = 0;

        for character in characters {
            let character_tokens = self.estimate_character_tokens(&character);
            if current_tokens + character_tokens <= token_budget {
                current_tokens += character_tokens;
                selected_characters.push(character);
            } else {
                break;
            }
        }

        Ok(selected_characters)
    }

    /// Get relevant locations for the current context
    async fn get_relevant_locations(
        &self,
        project_id: &str,
        token_budget: usize,
        db_pool: &DbPool,
    ) -> Result<Vec<Location>> {
        // Get all locations for the project
        let mut locations = LocationOps::get_by_project(db_pool, project_id).await?;

        // Sort by significance (if available) and then by name
        locations.sort_by(|a, b| {
            // Parse significance strings to compare
            let a_sig = a.significance.as_ref().map(|s| match s.as_str() {
                "Critical" => 0,
                "High" => 1,
                "Medium" => 2,
                "Low" => 3,
                _ => 4,
            }).unwrap_or(4);
            
            let b_sig = b.significance.as_ref().map(|s| match s.as_str() {
                "Critical" => 0,
                "High" => 1,
                "Medium" => 2,
                "Low" => 3,
                _ => 4,
            }).unwrap_or(4);
            
            a_sig.cmp(&b_sig).then_with(|| a.name.cmp(&b.name))
        });

        // Apply token budget
        let mut selected_locations = Vec::new();
        let mut current_tokens = 0;

        for location in locations {
            let location_tokens = self.estimate_location_tokens(&location);
            if current_tokens + location_tokens <= token_budget {
                current_tokens += location_tokens;
                selected_locations.push(location);
            } else {
                break;
            }
        }

        Ok(selected_locations)
    }

    /// Get relevant lore (world elements) for the current context
    async fn get_relevant_lore(
        &self,
        project_id: &str,
        token_budget: usize,
        db_pool: &DbPool,
    ) -> Result<Vec<WorldElement>> {
        // Get visible world elements for the project
        let mut world_elements = WorldElementOps::get_visible_by_project(db_pool, project_id).await?;

        // Sort by update time (most recent first) to prioritize actively developed lore
        world_elements.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        // Apply token budget
        let mut selected_elements = Vec::new();
        let mut current_tokens = 0;

        for element in world_elements {
            let element_tokens = self.estimate_world_element_tokens(&element);
            if current_tokens + element_tokens <= token_budget {
                current_tokens += element_tokens;
                selected_elements.push(element);
            } else {
                break;
            }
        }

        Ok(selected_elements)
    }

    /// Estimate token count for a character
    fn estimate_character_tokens(&self, character: &Character) -> usize {
        let mut content = character.name.clone();
        
        if let Some(desc) = &character.description {
            content.push_str(" ");
            content.push_str(desc);
        }
        
        if let Some(personality) = &character.personality {
            content.push_str(" ");
            content.push_str(personality);
        }
        
        if let Some(background) = &character.background {
            // Limit background to first 200 characters to avoid token explosion
            content.push_str(" ");
            content.push_str(&background.chars().take(200).collect::<String>());
        }
        
        estimate_tokens(&content)
    }

    /// Estimate token count for a location
    fn estimate_location_tokens(&self, location: &Location) -> usize {
        let mut content = location.name.clone();
        
        if let Some(desc) = &location.description {
            content.push_str(" ");
            content.push_str(desc);
        }
        
        if let Some(geography) = &location.geography {
            content.push_str(" ");
            content.push_str(&geography.chars().take(100).collect::<String>());
        }
        
        if let Some(culture) = &location.culture {
            content.push_str(" ");
            content.push_str(&culture.chars().take(100).collect::<String>());
        }
        
        estimate_tokens(&content)
    }

    /// Estimate token count for a world element
    fn estimate_world_element_tokens(&self, element: &WorldElement) -> usize {
        let mut content = element.name.clone();
        
        if let Some(desc) = &element.description {
            content.push_str(" ");
            content.push_str(&desc.chars().take(200).collect::<String>());
        }
        
        estimate_tokens(&content)
    }

    /// Build context for rewrite features
    pub async fn build_rewrite_context(
        &self,
        selected_text: &str,
        document_id: Option<i32>,
        db_pool: &DbPool,
    ) -> Result<AIContext> {
        self.build_rewrite_context_with_settings(selected_text, document_id, db_pool, None).await
    }

    /// Build context for rewrite features with settings
    pub async fn build_rewrite_context_with_settings(
        &self,
        selected_text: &str,
        document_id: Option<i32>,
        db_pool: &DbPool,
        settings: Option<&WriteSettings>,
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

            // Add Story Bible elements if enabled
            if let Some(settings) = settings {
                if settings.include_story_bible.unwrap_or(true) {
                    self.enrich_with_story_bible(&mut context, &document.project_id, settings, db_pool).await?;
                }
            } else {
                // Default: include Story Bible with balanced priority
                let default_settings = WriteSettings {
                    mode: WriteMode::Auto,
                    creativity_level: 5,
                    word_count_target: None,
                    tone: None,
                    include_key_details: Vec::new(),
                    include_story_bible: Some(true),
                    max_story_bible_tokens: Some(800), // Slightly less for rewrite context
                    story_bible_priority: Some(StoryBiblePriority::Balanced),
                };
                self.enrich_with_story_bible(&mut context, &document.project_id, &default_settings, db_pool).await?;
            }
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
    use crate::database::models::{Character, Location, WorldElement, CharacterRole, VisibilityLevel, LocationType};
    use chrono::Utc;

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

    #[test]
    fn test_story_bible_budget_balanced() {
        let budget = StoryBibleBudget::new(1000, &StoryBiblePriority::Balanced);
        assert_eq!(budget.total_tokens, 1000);
        assert_eq!(budget.characters_tokens, 500);
        assert_eq!(budget.locations_tokens, 300);
        assert_eq!(budget.lore_tokens, 200);
    }

    #[test]
    fn test_story_bible_budget_characters() {
        let budget = StoryBibleBudget::new(1000, &StoryBiblePriority::Characters);
        assert_eq!(budget.total_tokens, 1000);
        assert_eq!(budget.characters_tokens, 700);
        assert_eq!(budget.locations_tokens, 200);
        assert_eq!(budget.lore_tokens, 100);
    }

    #[test]
    fn test_estimate_character_tokens() {
        let context_builder = ContextBuilder::new();
        
        let character = Character {
            id: "test-id".to_string(),
            project_id: "test-project".to_string(),
            series_id: None,
            name: "John Doe".to_string(),
            description: Some("A brave hero".to_string()),
            role: CharacterRole::Protagonist,
            age: Some(25),
            appearance: Some("Tall and strong".to_string()),
            personality: Some("Courageous and kind".to_string()),
            background: Some("Born in a small village, trained as a knight".to_string()),
            goals: Some("Save the kingdom".to_string()),
            relationships: "{}".to_string(),
            visibility: VisibilityLevel::Always,
            original_project_id: Some("test-project".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: "{}".to_string(),
        };

        let tokens = context_builder.estimate_character_tokens(&character);
        assert!(tokens > 0);
        // Should include name, description, personality, and truncated background
        assert!(tokens > 10); // Should be more than just the name
    }

    #[test]
    fn test_estimate_location_tokens() {
        let context_builder = ContextBuilder::new();
        
        let location = Location {
            id: "test-id".to_string(),
            project_id: "test-project".to_string(),
            name: "Castle Blackstone".to_string(),
            description: Some("An ancient fortress".to_string()),
            location_type: LocationType::Building,
            geography: Some("Built on a rocky hill overlooking the valley".to_string()),
            climate: Some("Cold and windy".to_string()),
            culture: Some("Military stronghold".to_string()),
            history: Some("Built 500 years ago".to_string()),
            significance: Some("High".to_string()),
            visibility: VisibilityLevel::Always,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: "{}".to_string(),
        };

        let tokens = context_builder.estimate_location_tokens(&location);
        assert!(tokens > 0);
        // Should include name, description, geography, and culture (truncated)
        assert!(tokens > 5);
    }

    #[test]
    fn test_estimate_world_element_tokens() {
        let context_builder = ContextBuilder::new();
        
        let element = WorldElement {
            id: "test-id".to_string(),
            project_id: Some("test-project".to_string()),
            series_id: None,
            name: "Magic System".to_string(),
            description: Some("A complex system of elemental magic based on natural forces".to_string()),
            element_type: "magic".to_string(),
            properties: "{}".to_string(),
            is_visible: true,
            original_project_id: Some("test-project".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let tokens = context_builder.estimate_world_element_tokens(&element);
        assert!(tokens > 0);
        // Should include name and description (truncated)
        assert!(tokens > 3);
    }
}
