use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProseMode {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub model_configuration_id: i32,
    pub creativity_level: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub special_instructions: Option<String>,
    pub is_experimental: bool,
    pub max_context_words: i32,
    pub max_generation_words: i32,
    pub supports_streaming: bool,
    pub supports_unfiltered: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelConfiguration {
    pub id: Option<i32>,
    pub provider_id: i32,
    pub model_name: String,
    pub display_name: String,
    pub context_window: i32,
    pub max_output_tokens: i32,
    pub supports_streaming: bool,
    pub supports_images: bool,
    pub cost_per_input_token: Option<f64>,
    pub cost_per_output_token: Option<f64>,
    pub cost_per_image: Option<f64>,
    pub quality_tier: String,
    pub specializations: Option<String>, // JSON
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub id: Option<i32>,
    pub name: String,
    pub display_name: String,
    pub api_endpoint: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationSettings {
    pub prose_mode: String,
    pub context_words: i32,
    pub max_words: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub special_instructions: Option<String>,
    pub ultra_creative: bool,
    pub unfiltered: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClicheDetectionResult {
    pub detected_cliches: Vec<String>,
    pub severity_score: f32,
    pub suggestions: Vec<String>,
}

pub struct ProseModelManager {
    prose_modes: HashMap<String, ProseMode>,
    model_configurations: HashMap<i32, AIModelConfiguration>,
    providers: HashMap<i32, AIProvider>,
}

impl ProseModelManager {
    pub fn new() -> Self {
        Self {
            prose_modes: HashMap::new(),
            model_configurations: HashMap::new(),
            providers: HashMap::new(),
        }
    }

    pub fn add_prose_mode(&mut self, mode: ProseMode) {
        self.prose_modes.insert(mode.name.clone(), mode);
    }

    pub fn get_prose_mode(&self, name: &str) -> Option<&ProseMode> {
        self.prose_modes.get(name)
    }

    pub fn list_prose_modes(&self) -> Vec<&ProseMode> {
        self.prose_modes.values().collect()
    }

    pub fn add_model_configuration(&mut self, config: AIModelConfiguration) {
        if let Some(id) = config.id {
            self.model_configurations.insert(id, config);
        }
    }

    pub fn add_provider(&mut self, provider: AIProvider) {
        if let Some(id) = provider.id {
            self.providers.insert(id, provider);
        }
    }

    pub fn create_generation_settings(&self, prose_mode_name: &str, ultra_creative: bool) -> Option<GenerationSettings> {
        let mode = self.prose_modes.get(prose_mode_name)?;
        
        let mut settings = GenerationSettings {
            prose_mode: prose_mode_name.to_string(),
            context_words: mode.max_context_words,
            max_words: mode.max_generation_words,
            temperature: mode.temperature,
            top_p: mode.top_p,
            frequency_penalty: mode.frequency_penalty,
            presence_penalty: mode.presence_penalty,
            special_instructions: mode.special_instructions.clone(),
            ultra_creative,
            unfiltered: mode.supports_unfiltered,
        };

        // Apply ultra-creative enhancements
        if ultra_creative {
            settings.temperature = (settings.temperature * 1.3).min(1.0);
            settings.top_p = (settings.top_p * 1.1).min(1.0);
            settings.frequency_penalty = (settings.frequency_penalty + 0.2).min(2.0);
            settings.presence_penalty = (settings.presence_penalty + 0.1).min(2.0);
            
            // Add ultra-creative instructions
            let ultra_instructions = "\n\nULTRA-CREATIVE MODE: Push creative boundaries, explore unconventional narrative techniques, experiment with unique perspectives and innovative storytelling approaches. Avoid clichés and predictable patterns.";
            settings.special_instructions = match settings.special_instructions {
                Some(existing) => Some(format!("{}{}", existing, ultra_instructions)),
                None => Some(ultra_instructions.to_string()),
            };
        }

        Some(settings)
    }

    pub fn detect_cliches(&self, text: &str) -> ClicheDetectionResult {
        // Common clichés in creative writing
        let cliches = vec![
            "it was a dark and stormy night",
            "suddenly",
            "all of a sudden",
            "little did they know",
            "unbeknownst to them",
            "meanwhile",
            "without warning",
            "out of nowhere",
            "against all odds",
            "in the nick of time",
            "love at first sight",
            "happily ever after",
            "plot twist",
            "the chosen one",
            "destiny calls",
        ];

        let text_lower = text.to_lowercase();
        let mut detected = Vec::new();
        let mut severity = 0.0;

        for cliche in &cliches {
            if text_lower.contains(cliche) {
                detected.push(cliche.to_string());
                severity += 1.0;
            }
        }

        // Normalize severity score (0.0 to 1.0)
        severity = (severity / text.split_whitespace().count() as f32 * 100.0).min(1.0);

        let suggestions = if !detected.is_empty() {
            vec![
                "Consider using more specific, original descriptions".to_string(),
                "Try showing rather than telling".to_string(),
                "Explore unique metaphors and imagery".to_string(),
                "Focus on character-specific voice and perspective".to_string(),
            ]
        } else {
            vec![]
        };

        ClicheDetectionResult {
            detected_cliches: detected,
            severity_score: severity,
            suggestions,
        }
    }

    pub fn initialize_default_modes(&mut self) {
        // These would typically be loaded from the database
        // For now, we'll create some default modes
        
        let muse_mode = ProseMode {
            id: Some(1),
            name: "Muse".to_string(),
            description: Some("Premium creative writing with advanced AI models".to_string()),
            model_configuration_id: 1, // GPT-4 or Claude-3 Opus
            creativity_level: 8,
            temperature: 0.8,
            top_p: 0.9,
            frequency_penalty: 0.1,
            presence_penalty: 0.1,
            special_instructions: Some("Focus on literary quality, rich descriptions, and sophisticated prose. Maintain consistent character voice and narrative style.".to_string()),
            is_experimental: false,
            max_context_words: 6000,
            max_generation_words: 3000,
            supports_streaming: true,
            supports_unfiltered: true,
            is_active: true,
        };

        let excellent_mode = ProseMode {
            id: Some(2),
            name: "Excellent".to_string(),
            description: Some("High-quality writing with balanced creativity and coherence".to_string()),
            model_configuration_id: 2, // GPT-4 or Claude-3 Sonnet
            creativity_level: 6,
            temperature: 0.7,
            top_p: 0.85,
            frequency_penalty: 0.05,
            presence_penalty: 0.05,
            special_instructions: Some("Produce well-structured, engaging prose with good pacing and character development.".to_string()),
            is_experimental: false,
            max_context_words: 4000,
            max_generation_words: 2000,
            supports_streaming: true,
            supports_unfiltered: false,
            is_active: true,
        };

        let basic_mode = ProseMode {
            id: Some(3),
            name: "Basic".to_string(),
            description: Some("Reliable writing for drafts and quick content generation".to_string()),
            model_configuration_id: 3, // GPT-3.5 or Claude-3 Haiku
            creativity_level: 4,
            temperature: 0.6,
            top_p: 0.8,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            special_instructions: Some("Generate clear, coherent prose suitable for first drafts and content development.".to_string()),
            is_experimental: false,
            max_context_words: 3000,
            max_generation_words: 1500,
            supports_streaming: true,
            supports_unfiltered: false,
            is_active: true,
        };

        self.add_prose_mode(muse_mode);
        self.add_prose_mode(excellent_mode);
        self.add_prose_mode(basic_mode);
    }
}

impl Default for ProseModelManager {
    fn default() -> Self {
        let mut manager = Self::new();
        manager.initialize_default_modes();
        manager
    }
}