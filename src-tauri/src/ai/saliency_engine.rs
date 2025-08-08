use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaliencyContext {
    pub id: String,
    pub project_id: String,
    pub context_hash: String,
    pub selected_elements: SelectedElements,
    pub relevance_scores: HashMap<String, f32>,
    pub total_tokens: i32,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedElements {
    pub characters: Vec<CharacterElement>,
    pub locations: Vec<LocationElement>,
    pub plot_threads: Vec<PlotThreadElement>,
    pub worldbuilding: Vec<WorldbuildingElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterElement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub relevance_score: f32,
    pub token_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationElement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub relevance_score: f32,
    pub token_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotThreadElement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub relevance_score: f32,
    pub token_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldbuildingElement {
    pub id: String,
    pub category: String,
    pub title: String,
    pub content: String,
    pub relevance_score: f32,
    pub token_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaliencyConfig {
    pub max_total_tokens: i32,
    pub character_weight: f32,
    pub location_weight: f32,
    pub plot_weight: f32,
    pub worldbuilding_weight: f32,
    pub recency_weight: f32,
    pub similarity_threshold: f32,
    pub cache_duration_hours: i32,
}

impl Default for SaliencyConfig {
    fn default() -> Self {
        Self {
            max_total_tokens: 4000,
            character_weight: 1.2,
            location_weight: 1.0,
            plot_weight: 1.1,
            worldbuilding_weight: 0.9,
            recency_weight: 0.1,
            similarity_threshold: 0.3,
            cache_duration_hours: 24,
        }
    }
}

pub struct SaliencyEngine {
    config: SaliencyConfig,
    context_cache: HashMap<String, SaliencyContext>,
}

impl SaliencyEngine {
    pub fn new(config: SaliencyConfig) -> Self {
        Self {
            config,
            context_cache: HashMap::new(),
        }
    }

    pub fn build_context(
        &mut self,
        project_id: &str,
        current_text: &str,
        story_bible_elements: &StoryBibleElements,
    ) -> Result<SaliencyContext, Box<dyn std::error::Error>> {
        // Generate context hash for caching
        let context_hash = self.generate_context_hash(current_text, story_bible_elements);
        
        // Check cache first
        if let Some(cached_context) = self.get_cached_context(&context_hash) {
            if !self.is_cache_expired(&cached_context) {
                return Ok(cached_context);
            }
        }

        // Build new context
        let selected_elements = self.select_relevant_elements(current_text, story_bible_elements)?;
        let relevance_scores = self.calculate_relevance_scores(&selected_elements);
        let total_tokens = self.calculate_total_tokens(&selected_elements);

        let context = SaliencyContext {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            context_hash: context_hash.clone(),
            selected_elements,
            relevance_scores,
            total_tokens,
            expires_at: Some(
                chrono::Utc::now() + chrono::Duration::hours(self.config.cache_duration_hours as i64)
            ),
        };

        // Cache the context
        self.context_cache.insert(context_hash, context.clone());

        Ok(context)
    }

    fn select_relevant_elements(
        &self,
        current_text: &str,
        story_bible: &StoryBibleElements,
    ) -> Result<SelectedElements, Box<dyn std::error::Error>> {
        let mut selected = SelectedElements {
            characters: Vec::new(),
            locations: Vec::new(),
            plot_threads: Vec::new(),
            worldbuilding: Vec::new(),
        };

        let mut remaining_tokens = self.config.max_total_tokens;

        // Score and select characters
        let mut character_scores: Vec<(CharacterElement, f32)> = story_bible.characters
            .iter()
            .map(|char| {
                let element = CharacterElement {
                    id: char.id.clone(),
                    name: char.name.clone(),
                    description: char.description.clone(),
                    relevance_score: 0.0,
                    token_count: self.estimate_tokens(&format!("{}: {}", char.name, char.description)),
                };
                let score = self.calculate_similarity_score(current_text, &char.description) * self.config.character_weight;
                (element, score)
            })
            .collect();

        character_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (mut element, score) in character_scores {
            if score >= self.config.similarity_threshold && remaining_tokens >= element.token_count {
                element.relevance_score = score;
                remaining_tokens -= element.token_count;
                selected.characters.push(element);
            }
        }

        // Score and select locations
        let mut location_scores: Vec<(LocationElement, f32)> = story_bible.locations
            .iter()
            .map(|loc| {
                let element = LocationElement {
                    id: loc.id.clone(),
                    name: loc.name.clone(),
                    description: loc.description.clone(),
                    relevance_score: 0.0,
                    token_count: self.estimate_tokens(&format!("{}: {}", loc.name, loc.description)),
                };
                let score = self.calculate_similarity_score(current_text, &loc.description) * self.config.location_weight;
                (element, score)
            })
            .collect();

        location_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (mut element, score) in location_scores {
            if score >= self.config.similarity_threshold && remaining_tokens >= element.token_count {
                element.relevance_score = score;
                remaining_tokens -= element.token_count;
                selected.locations.push(element);
            }
        }

        // Score and select plot threads
        let mut plot_scores: Vec<(PlotThreadElement, f32)> = story_bible.plot_threads
            .iter()
            .map(|plot| {
                let element = PlotThreadElement {
                    id: plot.id.clone(),
                    title: plot.title.clone(),
                    description: plot.description.clone(),
                    relevance_score: 0.0,
                    token_count: self.estimate_tokens(&format!("{}: {}", plot.title, plot.description)),
                };
                let score = self.calculate_similarity_score(current_text, &plot.description) * self.config.plot_weight;
                (element, score)
            })
            .collect();

        plot_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (mut element, score) in plot_scores {
            if score >= self.config.similarity_threshold && remaining_tokens >= element.token_count {
                element.relevance_score = score;
                remaining_tokens -= element.token_count;
                selected.plot_threads.push(element);
            }
        }

        // Score and select worldbuilding elements
        let mut worldbuilding_scores: Vec<(WorldbuildingElement, f32)> = story_bible.worldbuilding
            .iter()
            .map(|wb| {
                let element = WorldbuildingElement {
                    id: wb.id.clone(),
                    category: wb.category.clone(),
                    title: wb.title.clone(),
                    content: wb.content.clone(),
                    relevance_score: 0.0,
                    token_count: self.estimate_tokens(&format!("{}: {}", wb.title, wb.content)),
                };
                let score = self.calculate_similarity_score(current_text, &wb.content) * self.config.worldbuilding_weight;
                (element, score)
            })
            .collect();

        worldbuilding_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        for (mut element, score) in worldbuilding_scores {
            if score >= self.config.similarity_threshold && remaining_tokens >= element.token_count {
                element.relevance_score = score;
                remaining_tokens -= element.token_count;
                selected.worldbuilding.push(element);
            }
        }

        Ok(selected)
    }

    fn calculate_similarity_score(&self, text1: &str, text2: &str) -> f32 {
        // Simple keyword-based similarity for now
        // In a real implementation, this would use embeddings
        let words1: std::collections::HashSet<String> = text1
            .to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3) // Filter out short words
            .map(|w| w.to_string())
            .collect();

        let words2: std::collections::HashSet<String> = text2
            .to_lowercase()
            .split_whitespace()
            .filter(|w| w.len() > 3)
            .map(|w| w.to_string())
            .collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f32 / union as f32
        }
    }

    fn estimate_tokens(&self, text: &str) -> i32 {
        // Rough estimation: 1 token ≈ 0.75 words
        (text.split_whitespace().count() as f32 * 1.33) as i32
    }

    fn calculate_total_tokens(&self, elements: &SelectedElements) -> i32 {
        elements.characters.iter().map(|c| c.token_count).sum::<i32>()
            + elements.locations.iter().map(|l| l.token_count).sum::<i32>()
            + elements.plot_threads.iter().map(|p| p.token_count).sum::<i32>()
            + elements.worldbuilding.iter().map(|w| w.token_count).sum::<i32>()
    }

    fn calculate_relevance_scores(&self, elements: &SelectedElements) -> HashMap<String, f32> {
        let mut scores = HashMap::new();

        for char in &elements.characters {
            scores.insert(char.id.clone(), char.relevance_score);
        }
        for loc in &elements.locations {
            scores.insert(loc.id.clone(), loc.relevance_score);
        }
        for plot in &elements.plot_threads {
            scores.insert(plot.id.clone(), plot.relevance_score);
        }
        for wb in &elements.worldbuilding {
            scores.insert(wb.id.clone(), wb.relevance_score);
        }

        scores
    }

    fn generate_context_hash(&self, text: &str, story_bible: &StoryBibleElements) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        
        // Hash story bible elements for cache invalidation
        for char in &story_bible.characters {
            char.id.hash(&mut hasher);
            char.name.hash(&mut hasher);
        }
        for loc in &story_bible.locations {
            loc.id.hash(&mut hasher);
            loc.name.hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }

    fn get_cached_context(&self, hash: &str) -> Option<SaliencyContext> {
        self.context_cache.get(hash).cloned()
    }

    fn is_cache_expired(&self, context: &SaliencyContext) -> bool {
        if let Some(expires_at) = context.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            true
        }
    }

    pub fn format_context_for_ai(&self, context: &SaliencyContext) -> String {
        let mut formatted = String::new();

        if !context.selected_elements.characters.is_empty() {
            formatted.push_str("\n## Relevant Characters:\n");
            for char in &context.selected_elements.characters {
                formatted.push_str(&format!("**{}**: {}\n", char.name, char.description));
            }
        }

        if !context.selected_elements.locations.is_empty() {
            formatted.push_str("\n## Relevant Locations:\n");
            for loc in &context.selected_elements.locations {
                formatted.push_str(&format!("**{}**: {}\n", loc.name, loc.description));
            }
        }

        if !context.selected_elements.plot_threads.is_empty() {
            formatted.push_str("\n## Relevant Plot Threads:\n");
            for plot in &context.selected_elements.plot_threads {
                formatted.push_str(&format!("**{}**: {}\n", plot.title, plot.description));
            }
        }

        if !context.selected_elements.worldbuilding.is_empty() {
            formatted.push_str("\n## Relevant Worldbuilding:\n");
            for wb in &context.selected_elements.worldbuilding {
                formatted.push_str(&format!("**{} ({})**: {}\n", wb.title, wb.category, wb.content));
            }
        }

        formatted
    }
}

// Placeholder structs for Story Bible elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBibleElements {
    pub characters: Vec<StoryBibleCharacter>,
    pub locations: Vec<StoryBibleLocation>,
    pub plot_threads: Vec<StoryBiblePlotThread>,
    pub worldbuilding: Vec<StoryBibleWorldbuilding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBibleCharacter {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBibleLocation {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBiblePlotThread {
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBibleWorldbuilding {
    pub id: String,
    pub category: String,
    pub title: String,
    pub content: String,
}

impl Default for SaliencyEngine {
    fn default() -> Self {
        Self::new(SaliencyConfig::default())
    }
}