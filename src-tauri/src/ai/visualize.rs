use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub id: String,
    pub project_id: String,
    pub source_text: String,
    pub image_prompt: String,
    pub image_data: Option<Vec<u8>>, // Base64 encoded image data
    pub image_url: Option<String>,   // External URL if hosted
    pub resolution: String,
    pub credits_used: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizeRequest {
    pub project_id: String,
    pub source_text: String,
    pub style_preference: Option<String>,
    pub resolution: ImageResolution,
    pub enhance_prompt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageResolution {
    Square1024,    // 1024x1024
    Portrait1024,  // 1024x1792
    Landscape1792, // 1792x1024
}

impl ImageResolution {
    pub fn to_string(&self) -> String {
        match self {
            ImageResolution::Square1024 => "1024x1024".to_string(),
            ImageResolution::Portrait1024 => "1024x1792".to_string(),
            ImageResolution::Landscape1792 => "1792x1024".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "1024x1024" => Ok(ImageResolution::Square1024),
            "1024x1792" => Ok(ImageResolution::Portrait1024),
            "1792x1024" => Ok(ImageResolution::Landscape1792),
            _ => Err(format!("Invalid resolution: {}", s)),
        }
    }

    pub fn get_credits_cost(&self) -> i32 {
        match self {
            ImageResolution::Square1024 => 2500,
            ImageResolution::Portrait1024 => 3500,
            ImageResolution::Landscape1792 => 3500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptEnhancement {
    pub original_prompt: String,
    pub enhanced_prompt: String,
    pub style_elements: Vec<String>,
    pub composition_notes: Vec<String>,
    pub lighting_suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizeConfig {
    pub default_style: String,
    pub max_prompt_length: usize,
    pub enable_prompt_enhancement: bool,
    pub safety_filter: bool,
    pub quality_tier: String,
}

impl Default for VisualizeConfig {
    fn default() -> Self {
        Self {
            default_style: "cinematic, detailed, high quality".to_string(),
            max_prompt_length: 1000,
            enable_prompt_enhancement: true,
            safety_filter: true,
            quality_tier: "standard".to_string(),
        }
    }
}

pub struct VisualizeEngine {
    config: VisualizeConfig,
    generated_images: HashMap<String, GeneratedImage>,
}

impl VisualizeEngine {
    pub fn new(config: VisualizeConfig) -> Self {
        Self {
            config,
            generated_images: HashMap::new(),
        }
    }

    pub fn extract_visual_elements(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Extract visual elements from the text
        let mut visual_elements = Vec::new();
        
        // Look for character descriptions
        let character_keywords = ["tall", "short", "blonde", "brunette", "blue eyes", "green eyes", 
                                "wearing", "dressed in", "appearance", "looked like"];
        
        // Look for setting descriptions
        let setting_keywords = ["room", "forest", "castle", "city", "mountain", "ocean", "desert",
                              "dark", "bright", "moonlight", "sunlight", "shadows"];
        
        // Look for mood and atmosphere
        let mood_keywords = ["mysterious", "ominous", "peaceful", "chaotic", "serene", "tense",
                           "magical", "ethereal", "gritty", "romantic"];
        
        let _text_lower = text.to_lowercase();
        
        // Extract sentences containing visual keywords
        let sentences: Vec<&str> = text.split('.').collect();
        
        for sentence in sentences {
            let sentence_lower = sentence.to_lowercase();
            
            // Check for character descriptions
            if character_keywords.iter().any(|&keyword| sentence_lower.contains(keyword)) {
                visual_elements.push(format!("Character: {}", sentence.trim()));
            }
            
            // Check for setting descriptions
            if setting_keywords.iter().any(|&keyword| sentence_lower.contains(keyword)) {
                visual_elements.push(format!("Setting: {}", sentence.trim()));
            }
            
            // Check for mood descriptions
            if mood_keywords.iter().any(|&keyword| sentence_lower.contains(keyword)) {
                visual_elements.push(format!("Atmosphere: {}", sentence.trim()));
            }
        }
        
        if visual_elements.is_empty() {
            // Fallback: use the first few sentences
            let first_sentences: Vec<&str> = text.split('.').take(3).collect();
            visual_elements.push(first_sentences.join(". "));
        }
        
        Ok(visual_elements.join(". "))
    }

    pub fn enhance_prompt(&self, base_prompt: &str, style_preference: Option<&str>) -> PromptEnhancement {
        let mut enhanced_prompt = base_prompt.to_string();
        let mut style_elements = Vec::new();
        let mut composition_notes = Vec::new();
        let mut lighting_suggestions = Vec::new();

        // Add style preference if provided
        if let Some(style) = style_preference {
            enhanced_prompt.push_str(&format!(", {}", style));
            style_elements.push(style.to_string());
        } else {
            enhanced_prompt.push_str(&format!(", {}", self.config.default_style));
            style_elements.push(self.config.default_style.clone());
        }

        // Add composition suggestions based on content
        let prompt_lower = base_prompt.to_lowercase();
        
        if prompt_lower.contains("character") || prompt_lower.contains("person") {
            composition_notes.push("portrait composition".to_string());
            enhanced_prompt.push_str(", portrait composition, detailed facial features");
        }
        
        if prompt_lower.contains("landscape") || prompt_lower.contains("forest") || prompt_lower.contains("mountain") {
            composition_notes.push("wide landscape view".to_string());
            enhanced_prompt.push_str(", wide landscape view, depth of field");
        }
        
        if prompt_lower.contains("room") || prompt_lower.contains("interior") {
            composition_notes.push("interior scene".to_string());
            enhanced_prompt.push_str(", interior scene, atmospheric lighting");
        }

        // Add lighting suggestions
        if prompt_lower.contains("night") || prompt_lower.contains("dark") {
            lighting_suggestions.push("dramatic low lighting".to_string());
            enhanced_prompt.push_str(", dramatic low lighting, shadows");
        } else if prompt_lower.contains("day") || prompt_lower.contains("bright") {
            lighting_suggestions.push("natural daylight".to_string());
            enhanced_prompt.push_str(", natural daylight, soft shadows");
        } else {
            lighting_suggestions.push("balanced lighting".to_string());
            enhanced_prompt.push_str(", balanced lighting");
        }

        // Add quality enhancers
        enhanced_prompt.push_str(", highly detailed, professional photography, 8k resolution");
        style_elements.push("highly detailed".to_string());
        style_elements.push("professional quality".to_string());

        // Ensure prompt doesn't exceed max length
        if enhanced_prompt.len() > self.config.max_prompt_length {
            enhanced_prompt.truncate(self.config.max_prompt_length);
            // Find the last complete word
            if let Some(last_space) = enhanced_prompt.rfind(' ') {
                enhanced_prompt.truncate(last_space);
            }
        }

        PromptEnhancement {
            original_prompt: base_prompt.to_string(),
            enhanced_prompt,
            style_elements,
            composition_notes,
            lighting_suggestions,
        }
    }

    pub async fn generate_image(
        &mut self,
        request: VisualizeRequest,
    ) -> Result<GeneratedImage, Box<dyn std::error::Error>> {
        // Extract visual elements from source text
        let visual_elements = self.extract_visual_elements(&request.source_text)?;
        
        // Enhance the prompt if requested
        let final_prompt = if request.enhance_prompt {
            let enhancement = self.enhance_prompt(&visual_elements, request.style_preference.as_deref());
            enhancement.enhanced_prompt
        } else {
            visual_elements
        };

        // Create the generated image record
        let generated_image = GeneratedImage {
            id: Uuid::new_v4().to_string(),
            project_id: request.project_id.clone(),
            source_text: request.source_text.clone(),
            image_prompt: final_prompt.clone(),
            image_data: None, // Will be populated by the AI provider
            image_url: None,  // Will be populated by the AI provider
            resolution: request.resolution.to_string(),
            credits_used: request.resolution.get_credits_cost(),
            created_at: chrono::Utc::now(),
        };

        // Store the image record
        self.generated_images.insert(generated_image.id.clone(), generated_image.clone());

        Ok(generated_image)
    }

    pub fn get_generated_image(&self, image_id: &str) -> Option<&GeneratedImage> {
        self.generated_images.get(image_id)
    }

    pub fn list_project_images(&self, project_id: &str) -> Vec<&GeneratedImage> {
        self.generated_images
            .values()
            .filter(|img| img.project_id == project_id)
            .collect()
    }

    pub fn update_image_data(&mut self, image_id: &str, image_data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(image) = self.generated_images.get_mut(image_id) {
            image.image_data = Some(image_data);
            Ok(())
        } else {
            Err("Image not found".into())
        }
    }

    pub fn update_image_url(&mut self, image_id: &str, image_url: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(image) = self.generated_images.get_mut(image_id) {
            image.image_url = Some(image_url);
            Ok(())
        } else {
            Err("Image not found".into())
        }
    }

    pub fn delete_image(&mut self, image_id: &str) -> Result<GeneratedImage, Box<dyn std::error::Error>> {
        self.generated_images
            .remove(image_id)
            .ok_or_else(|| "Image not found".into())
    }

    pub fn get_total_credits_used(&self, project_id: &str) -> i32 {
        self.generated_images
            .values()
            .filter(|img| img.project_id == project_id)
            .map(|img| img.credits_used)
            .sum()
    }

    pub fn analyze_visual_content(&self, text: &str) -> VisualContentAnalysis {
        let mut analysis = VisualContentAnalysis {
            has_character_descriptions: false,
            has_setting_descriptions: false,
            has_action_scenes: false,
            has_mood_atmosphere: false,
            visual_density_score: 0.0,
            recommended_images: 0,
            key_visual_moments: Vec::new(),
        };

        let text_lower = text.to_lowercase();
        let word_count = text.split_whitespace().count();

        // Check for character descriptions
        let character_keywords = ["appearance", "looked", "wearing", "tall", "short", "hair", "eyes"];
        analysis.has_character_descriptions = character_keywords.iter()
            .any(|&keyword| text_lower.contains(keyword));

        // Check for setting descriptions
        let setting_keywords = ["room", "forest", "castle", "landscape", "building", "street"];
        analysis.has_setting_descriptions = setting_keywords.iter()
            .any(|&keyword| text_lower.contains(keyword));

        // Check for action scenes
        let action_keywords = ["ran", "jumped", "fought", "battle", "chase", "explosion"];
        analysis.has_action_scenes = action_keywords.iter()
            .any(|&keyword| text_lower.contains(keyword));

        // Check for mood/atmosphere
        let mood_keywords = ["dark", "bright", "mysterious", "peaceful", "tense", "magical"];
        analysis.has_mood_atmosphere = mood_keywords.iter()
            .any(|&keyword| text_lower.contains(keyword));

        // Calculate visual density score
        let visual_word_count = text_lower.split_whitespace()
            .filter(|word| {
                character_keywords.contains(word) ||
                setting_keywords.contains(word) ||
                action_keywords.contains(word) ||
                mood_keywords.contains(word)
            })
            .count();

        analysis.visual_density_score = if word_count > 0 {
            (visual_word_count as f32 / word_count as f32) * 100.0
        } else {
            0.0
        };

        // Recommend number of images based on content length and visual density
        analysis.recommended_images = match word_count {
            0..=500 => 1,
            501..=1500 => 2,
            1501..=3000 => 3,
            _ => 4,
        };

        if analysis.visual_density_score > 10.0 {
            analysis.recommended_images += 1;
        }

        // Extract key visual moments (simplified)
        let sentences: Vec<&str> = text.split('.').collect();
        for (i, sentence) in sentences.iter().enumerate() {
            let sentence_lower = sentence.to_lowercase();
            if character_keywords.iter().any(|&keyword| sentence_lower.contains(keyword)) ||
               setting_keywords.iter().any(|&keyword| sentence_lower.contains(keyword)) {
                analysis.key_visual_moments.push(KeyVisualMoment {
                    sentence_index: i,
                    content: sentence.trim().to_string(),
                    visual_type: if character_keywords.iter().any(|&keyword| sentence_lower.contains(keyword)) {
                        "character".to_string()
                    } else {
                        "setting".to_string()
                    },
                });
            }
        }

        analysis
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualContentAnalysis {
    pub has_character_descriptions: bool,
    pub has_setting_descriptions: bool,
    pub has_action_scenes: bool,
    pub has_mood_atmosphere: bool,
    pub visual_density_score: f32,
    pub recommended_images: usize,
    pub key_visual_moments: Vec<KeyVisualMoment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyVisualMoment {
    pub sentence_index: usize,
    pub content: String,
    pub visual_type: String,
}

impl Default for VisualizeEngine {
    fn default() -> Self {
        Self::new(VisualizeConfig::default())
    }
}
