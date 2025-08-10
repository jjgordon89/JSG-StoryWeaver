use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    prose_modes::{ProseModelManager, GenerationSettings, ProseMode},
    saliency_engine::{SaliencyEngine, SaliencyContext, StoryBibleElements},
    visualize::{VisualizeEngine, VisualizeRequest, GeneratedImage},
    brainstorm::{BrainstormEngine, BrainstormRequest, BrainstormSession},
    AIProvider, AIContext, TextStream,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedGenerationRequest {
    pub project_id: String,
    pub document_id: Option<String>,
    pub prose_mode: String,
    pub text_context: String,
    pub generation_type: String,
    pub max_words: Option<i32>,
    pub ultra_creative: bool,
    pub use_saliency_engine: bool,
    pub style_examples: Vec<String>,
    pub special_instructions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedGenerationResult {
    pub generated_text: String,
    pub prose_mode_used: String,
    pub saliency_context: Option<SaliencyContext>,
    pub cliche_detection: Option<super::prose_modes::ClicheDetectionResult>,
    pub token_count: i32,
    pub credits_used: i32,
    pub generation_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleExample {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub content: String,
    pub word_count: i32,
    pub analysis_result: Option<StyleAnalysis>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleAnalysis {
    pub sentence_length_avg: f32,
    pub vocabulary_complexity: f32,
    pub dialogue_ratio: f32,
    pub description_ratio: f32,
    pub action_ratio: f32,
    pub tone_indicators: Vec<String>,
    pub common_phrases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditUsage {
    pub operation_type: String,
    pub credits_used: i32,
    pub cost_estimate: Option<f64>,
    pub provider: String,
    pub model: String,
    pub details: HashMap<String, serde_json::Value>,
}

pub struct AdvancedAIManager {
    prose_manager: ProseModelManager,
    saliency_engine: SaliencyEngine,
    visualize_engine: VisualizeEngine,
    brainstorm_engine: BrainstormEngine,
    ai_providers: HashMap<String, Box<dyn AIProvider>>,
    style_examples: HashMap<String, StyleExample>,
    credit_tracker: CreditTracker,
}

pub struct CreditTracker {
    project_usage: HashMap<String, i32>,
    daily_usage: HashMap<String, i32>, // date -> credits
    monthly_limits: HashMap<String, i32>, // project_id -> limit
}

impl CreditTracker {
    pub fn new() -> Self {
        Self {
            project_usage: HashMap::new(),
            daily_usage: HashMap::new(),
            monthly_limits: HashMap::new(),
        }
    }

    pub fn track_usage(&mut self, project_id: &str, credits: i32) {
        *self.project_usage.entry(project_id.to_string()).or_insert(0) += credits;
        
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        *self.daily_usage.entry(today).or_insert(0) += credits;
    }

    pub fn get_project_usage(&self, project_id: &str) -> i32 {
        self.project_usage.get(project_id).copied().unwrap_or(0)
    }

    pub fn get_daily_usage(&self, date: &str) -> i32 {
        self.daily_usage.get(date).copied().unwrap_or(0)
    }

    pub fn check_limit(&self, project_id: &str, additional_credits: i32) -> bool {
        if let Some(limit) = self.monthly_limits.get(project_id) {
            let current_usage = self.get_project_usage(project_id);
            current_usage + additional_credits <= *limit
        } else {
            true // No limit set
        }
    }
}

impl AdvancedAIManager {
    pub fn new() -> Self {
        Self {
            prose_manager: ProseModelManager::default(),
            saliency_engine: SaliencyEngine::default(),
            visualize_engine: VisualizeEngine::default(),
            brainstorm_engine: BrainstormEngine::default(),
            ai_providers: HashMap::new(),
            style_examples: HashMap::new(),
            credit_tracker: CreditTracker::new(),
        }
    }

    pub fn add_ai_provider(&mut self, name: String, provider: Box<dyn AIProvider>) {
        self.ai_providers.insert(name, provider);
    }

    pub async fn generate_with_advanced_features(
        &mut self,
        request: AdvancedGenerationRequest,
        story_bible: Option<StoryBibleElements>,
    ) -> Result<AdvancedGenerationResult, Box<dyn std::error::Error>> {
        // Get prose mode settings
        let generation_settings = self.prose_manager
            .create_generation_settings(&request.prose_mode, request.ultra_creative)
            .ok_or("Invalid prose mode")?;

        // Build saliency context if requested
        let saliency_context = if request.use_saliency_engine && story_bible.is_some() {
            Some(self.saliency_engine.build_context(
                &request.project_id,
                &request.text_context,
                &story_bible.unwrap(),
            )?)
        } else {
            None
        };

        // Build enhanced context
        let mut enhanced_context = request.text_context.clone();
        
        // Add saliency context
        if let Some(ref context) = saliency_context {
            enhanced_context.push_str(&self.saliency_engine.format_context_for_ai(context));
        }

        // Add style examples
        if !request.style_examples.is_empty() {
            enhanced_context.push_str("\n\n## Style Examples:\n");
            for example_id in &request.style_examples {
                if let Some(example) = self.style_examples.get(example_id) {
                    enhanced_context.push_str(&format!("**{}:**\n{}\n\n", example.name, example.content));
                }
            }
        }

        // Create AI context
        let ai_context = AIContext {
            project_id: Some(request.project_id.clone()),
            document_id: request.document_id.clone(),
            preceding_text: Some(enhanced_context),
            following_text: None,
            selected_text: None,
            story_context: None,
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: None,
            feature_type: None,
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };

        // Get appropriate AI provider based on prose mode
        let provider_name = self.get_provider_for_prose_mode(&request.prose_mode)?;
        let provider = self.ai_providers.get_mut(&provider_name)
            .ok_or("AI provider not available")?;

        // Generate text
        let prompt = generation_settings.special_instructions.unwrap_or_default();
        let generated_text = provider.generate_text(&prompt, &ai_context).await?;

        // Perform cliché detection if ultra-creative mode
        let cliche_detection = if request.ultra_creative {
            Some(self.prose_manager.detect_cliches(&generated_text))
        } else {
            None
        };

        // Calculate token count and credits
        let token_count = self.estimate_tokens(&generated_text);
        let credits_used = self.calculate_credits(&request.prose_mode, token_count);

        // Track credit usage
        self.credit_tracker.track_usage(&request.project_id, credits_used);

        let generation_id = Uuid::new_v4().to_string();

        Ok(AdvancedGenerationResult {
            generated_text,
            prose_mode_used: request.prose_mode,
            saliency_context,
            cliche_detection,
            token_count,
            credits_used,
            generation_id,
        })
    }

    pub async fn generate_image(
        &mut self,
        request: VisualizeRequest,
    ) -> Result<GeneratedImage, Box<dyn std::error::Error>> {
        // Check credit limits
        let credits_needed = request.resolution.get_credits_cost();
        if !self.credit_tracker.check_limit(&request.project_id, credits_needed) {
            return Err("Credit limit exceeded".into());
        }

        // Generate image using visualize engine
        let mut generated_image = self.visualize_engine.generate_image(request).await?;

        // Get appropriate AI provider for image generation
        if let Some(provider) = self.ai_providers.get_mut("openai") {
            // Generate the actual image
            let image_data = provider.generate_image(&generated_image.image_prompt).await?;

            // Update the generated image with actual data
            let image_bytes = image_data.into_bytes();
            self.visualize_engine.update_image_data(&generated_image.id, image_bytes)?;
        }

        // Track credit usage
        self.credit_tracker.track_usage(&generated_image.project_id, generated_image.credits_used);

        Ok(generated_image)
    }

    pub async fn create_brainstorm_session(
        &mut self,
        request: BrainstormRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = self.brainstorm_engine.create_session(request.clone())?;
        
        // Generate initial ideas
        self.brainstorm_engine.generate_ideas(&session_id, &request)?;
        
        // Track credit usage for brainstorming
        let credits_used = self.calculate_brainstorm_credits(&request);
        self.credit_tracker.track_usage(&request.project_id, credits_used);
        
        Ok(session_id)
    }

    pub fn add_style_example(&mut self, example: StyleExample) {
        self.style_examples.insert(example.id.clone(), example);
    }

    pub fn analyze_style(&self, content: &str) -> StyleAnalysis {
        let sentences: Vec<&str> = content.split('.').filter(|s| !s.trim().is_empty()).collect();
        let words: Vec<&str> = content.split_whitespace().collect();
        
        let sentence_length_avg = if !sentences.is_empty() {
            words.len() as f32 / sentences.len() as f32
        } else {
            0.0
        };

        // Simple vocabulary complexity (average word length)
        let vocabulary_complexity = if !words.is_empty() {
            words.iter().map(|w| w.len()).sum::<usize>() as f32 / words.len() as f32
        } else {
            0.0
        };

        // Estimate dialogue ratio (text in quotes)
        let dialogue_chars = content.matches('"').count() / 2; // Pairs of quotes
        let dialogue_ratio = if content.len() > 0 {
            (dialogue_chars as f32 * 50.0) / content.len() as f32 // Rough estimate
        } else {
            0.0
        };

        // Simple ratios for description and action
        let description_keywords = ["looked", "appeared", "seemed", "was", "were"];
        let action_keywords = ["ran", "walked", "jumped", "moved", "went"];
        
        let description_count = description_keywords.iter()
            .map(|&keyword| content.to_lowercase().matches(keyword).count())
            .sum::<usize>();
        
        let action_count = action_keywords.iter()
            .map(|&keyword| content.to_lowercase().matches(keyword).count())
            .sum::<usize>();
        
        let total_words = words.len();
        let description_ratio = if total_words > 0 {
            description_count as f32 / total_words as f32
        } else {
            0.0
        };
        
        let action_ratio = if total_words > 0 {
            action_count as f32 / total_words as f32
        } else {
            0.0
        };

        StyleAnalysis {
            sentence_length_avg,
            vocabulary_complexity,
            dialogue_ratio,
            description_ratio,
            action_ratio,
            tone_indicators: self.extract_tone_indicators(content),
            common_phrases: self.extract_common_phrases(content),
        }
    }

    fn extract_tone_indicators(&self, content: &str) -> Vec<String> {
        let tone_words = [
            "dark", "bright", "mysterious", "cheerful", "somber", "playful",
            "serious", "humorous", "tense", "relaxed", "formal", "casual"
        ];
        
        let content_lower = content.to_lowercase();
        tone_words.iter()
            .filter(|&&word| content_lower.contains(word))
            .map(|&word| word.to_string())
            .collect()
    }

    fn create_import_analysis_prompt(&self, content: &str, content_type: &str) -> String {
        let word_count = content.split_whitespace().count();
        let content_preview = if word_count > 120000 {
            // Truncate to 120,000 words if content is too long
            let words: Vec<&str> = content.split_whitespace().take(120000).collect();
            words.join(" ")
        } else {
            content.to_string()
        };
        
        format!(
            r#"Analyze the following {} content for story elements that can be imported into a writing project. The content contains approximately {} words.

Content to analyze:
{}

Please provide a structured analysis in the following format:

## SUGGESTIONS
[List 3-5 actionable suggestions for how this content could be used in the project]

## CHARACTERS
[Extract up to 30 character information entries in format: Name | Description | Traits]
[Focus on main characters, supporting characters, and memorable minor characters]
[Include personality traits, motivations, and key characteristics]

## LOCATIONS
[Extract location information in format: Name | Description | Atmosphere]
[Include both major settings and notable minor locations]

## PLOT_POINTS
[Extract key plot elements, conflicts, story beats, and narrative turning points]
[Include major events, conflicts, resolutions, and character developments]

## THEMES
[Identify major themes, motifs, symbolic elements, and underlying messages]
[Include both explicit and implicit thematic content]

Focus on extracting concrete, usable story elements that would be valuable for a writer to reference or build upon. Prioritize the most significant and well-developed elements."#,
            content_type, word_count, content_preview
        )
    }

    fn parse_import_analysis(&self, analysis_text: &str) -> Result<crate::commands::advanced_ai_commands::SmartImportAnalysisResult, Box<dyn std::error::Error>> {
        use crate::commands::advanced_ai_commands::*;
        
        let mut suggestions = Vec::new();
        let mut characters = Vec::new();
        let mut locations = Vec::new();
        let mut plot_points = Vec::new();
        let mut themes = Vec::new();
        
        let sections: Vec<&str> = analysis_text.split("##").collect();
        
        for section in sections {
            let lines: Vec<&str> = section.lines().collect();
            if lines.is_empty() { continue; }
            
            let header = lines[0].trim().to_uppercase();
            let content_lines: Vec<&str> = lines[1..].iter().filter(|line| !line.trim().is_empty()).map(|&line| line).collect();
            
            match header.as_str() {
                "SUGGESTIONS" => {
                    for line in content_lines {
                        let clean_line = line.trim().trim_start_matches('-').trim_start_matches('*').trim();
                        if !clean_line.is_empty() {
                            suggestions.push(ImportSuggestion {
                                suggestion_type: "general".to_string(),
                                name: "General Suggestion".to_string(),
                                description: clean_line.to_string(),
                                confidence: 0.8,
                                auto_apply: false,
                                additional_data: None,
                            });
                        }
                    }
                },
                "CHARACTERS" => {
                    for line in content_lines {
                        let clean_line = line.trim().trim_start_matches('-').trim_start_matches('*').trim();
                        if !clean_line.is_empty() && characters.len() < 30 {
                            let parts: Vec<&str> = clean_line.split('|').collect();
                            if parts.len() >= 2 {
                                let name = parts[0].trim().to_string();
                                // Skip if character already exists
                                if !characters.iter().any(|c: &ExtractedCharacter| c.name.eq_ignore_ascii_case(&name)) {
                                    characters.push(ExtractedCharacter {
                                        name,
                                        description: parts.get(1).unwrap_or(&"").trim().to_string(),
                                        traits: if parts.len() > 2 {
                                            parts[2].trim().split(',').map(|s| s.trim().to_string())
                                                .filter(|s| !s.is_empty())
                                                .collect()
                                        } else {
                                            Vec::new()
                                        },
                                        relationships: Vec::new(),
                                        confidence: 0.8,
                                    });
                                }
                            }
                        }
                    }
                },
                "LOCATIONS" => {
                    for line in content_lines {
                        let clean_line = line.trim().trim_start_matches('-').trim_start_matches('*').trim();
                        if !clean_line.is_empty() {
                            let parts: Vec<&str> = clean_line.split('|').collect();
                            if parts.len() >= 2 {
                                locations.push(ExtractedLocation {
                                    name: parts[0].trim().to_string(),
                                    description: parts.get(1).unwrap_or(&"").trim().to_string(),
                                    atmosphere: parts.get(2).unwrap_or(&"").trim().to_string(),
                                    significance: "Extracted from content".to_string(),
                                    confidence: 0.8,
                                });
                            }
                        }
                    }
                },
                "PLOT_POINTS" => {
                    for line in content_lines {
                        let clean_line = line.trim().trim_start_matches('-').trim_start_matches('*').trim();
                        if !clean_line.is_empty() {
                            plot_points.push(clean_line.to_string());
                        }
                    }
                },
                "THEMES" => {
                    for line in content_lines {
                        let clean_line = line.trim().trim_start_matches('-').trim_start_matches('*').trim();
                        if !clean_line.is_empty() {
                            themes.push(clean_line.to_string());
                        }
                    }
                },
                _ => {} // Ignore unknown sections
            }
        }
        
        Ok(SmartImportAnalysisResult {
            suggestions,
            extracted_elements: ExtractedElements {
                characters,
                locations,
                plot_points,
                themes,
            },
        })
    }

    fn extract_common_phrases(&self, content: &str) -> Vec<String> {
        // Simple implementation - find repeated 2-3 word phrases
        let words: Vec<&str> = content.split_whitespace().collect();
        let mut phrase_counts: HashMap<String, usize> = HashMap::new();
        
        // Check 2-word phrases
        for window in words.windows(2) {
            let phrase = window.join(" ").to_lowercase();
            *phrase_counts.entry(phrase).or_insert(0) += 1;
        }
        
        // Return phrases that appear more than once
        phrase_counts.into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(phrase, _)| phrase)
            .take(5) // Limit to top 5
            .collect()
    }

    fn get_provider_for_prose_mode(&self, prose_mode: &str) -> Result<String, Box<dyn std::error::Error>> {
        match prose_mode {
            "Muse" => Ok("openai".to_string()), // Use GPT-4 for premium
            "Excellent" => Ok("claude".to_string()), // Use Claude for balanced
            "Basic" => Ok("openai".to_string()), // Use GPT-3.5 for basic
            "Experimental" => Ok("gemini".to_string()), // Use Gemini for experimental
            _ => Ok("openai".to_string()), // Default fallback
        }
    }

    fn estimate_tokens(&self, text: &str) -> i32 {
        // Rough estimation: 1 token ≈ 0.75 words
        (text.split_whitespace().count() as f32 * 1.33) as i32
    }

    fn calculate_credits(&self, prose_mode: &str, token_count: i32) -> i32 {
        let base_credits = match prose_mode {
            "Muse" => token_count * 3,      // Premium pricing
            "Excellent" => token_count * 2, // Standard pricing
            "Basic" => token_count,         // Basic pricing
            "Experimental" => token_count / 2, // Discounted experimental
            _ => token_count,
        };
        base_credits.max(1) // Minimum 1 credit
    }

    fn calculate_brainstorm_credits(&self, request: &BrainstormRequest) -> i32 {
        let base_credits = 100; // Base cost for brainstorming
        let idea_credits = request.num_ideas as i32 * 10; // 10 credits per idea
        let creativity_multiplier = if request.creativity_level > 7 { 2 } else { 1 };
        
        (base_credits + idea_credits) * creativity_multiplier
    }

    pub fn get_credit_usage(&self, project_id: &str) -> i32 {
        self.credit_tracker.get_project_usage(project_id)
    }

    pub fn get_prose_modes(&self) -> Vec<&ProseMode> {
        self.prose_manager.list_prose_modes()
    }

    pub fn get_brainstorm_session(&self, session_id: &str) -> Option<&BrainstormSession> {
        self.brainstorm_engine.get_session(session_id)
    }

    pub fn get_generated_images(&self, project_id: &str) -> Vec<&GeneratedImage> {
        self.visualize_engine.list_project_images(project_id)
    }

    pub async fn analyze_content_for_import(
        &self,
        project_id: &str,
        content: &str,
        content_type: &str,
    ) -> Result<crate::commands::advanced_ai_commands::SmartImportAnalysisResult, Box<dyn std::error::Error>> {
        // Get AI provider for analysis
        let provider = self.ai_providers.get("openai")
            .or_else(|| self.ai_providers.get("claude"))
            .or_else(|| self.ai_providers.values().next())
            .ok_or("No AI provider available")?;

        // Create analysis prompt
        let analysis_prompt = self.create_import_analysis_prompt(content, content_type);
        
        // Create AI context
        let ai_context = AIContext {
            project_id: Some(project_id.to_string()),
            document_id: None,
            preceding_text: Some(content.to_string()),
            following_text: None,
            selected_text: None,
            story_context: None,
            characters: None,
            locations: None,
            plot_threads: None,
            user_preferences: Some(HashMap::new()),
            writing_style: None,
            tone: None,
            creativity_level: None,
            feature_type: None,
            feature_options: None,
            word_count_target: None,
            genre: None,
            key_details: None,
        };

        // Get AI analysis
        let analysis_text = provider.generate_text(&analysis_prompt, &ai_context).await?;
        
        // Parse the AI response into structured data
        self.parse_import_analysis(&analysis_text)
    }
}

impl Default for AdvancedAIManager {
    fn default() -> Self {
        Self::new()
    }
}