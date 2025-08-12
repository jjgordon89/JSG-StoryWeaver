use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormSession {
    pub id: String,
    pub project_id: String,
    pub category: BrainstormCategory,
    pub seed_prompt: Option<String>,
    pub ideas: Vec<BrainstormIdea>,
    pub keepers: Vec<String>, // IDs of ideas marked as keepers
    pub session_notes: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BrainstormCategory {
    #[default]
    Characters,
    Plot,
    Worldbuilding,
    Dialogue,
    Scenes,
    Themes,
    Conflicts,
    Settings,
    Relationships,
    Backstory,
    Custom(String),
}

impl BrainstormCategory {
    pub fn to_string(&self) -> String {
        match self {
            BrainstormCategory::Characters => "Characters".to_string(),
            BrainstormCategory::Plot => "Plot".to_string(),
            BrainstormCategory::Worldbuilding => "Worldbuilding".to_string(),
            BrainstormCategory::Dialogue => "Dialogue".to_string(),
            BrainstormCategory::Scenes => "Scenes".to_string(),
            BrainstormCategory::Themes => "Themes".to_string(),
            BrainstormCategory::Conflicts => "Conflicts".to_string(),
            BrainstormCategory::Settings => "Settings".to_string(),
            BrainstormCategory::Relationships => "Relationships".to_string(),
            BrainstormCategory::Backstory => "Backstory".to_string(),
            BrainstormCategory::Custom(name) => name.clone(),
        }
    }

}

impl FromStr for BrainstormCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Characters" => Ok(BrainstormCategory::Characters),
            "Plot" => Ok(BrainstormCategory::Plot),
            "Worldbuilding" => Ok(BrainstormCategory::Worldbuilding),
            "Dialogue" => Ok(BrainstormCategory::Dialogue),
            "Scenes" => Ok(BrainstormCategory::Scenes),
            "Themes" => Ok(BrainstormCategory::Themes),
            "Conflicts" => Ok(BrainstormCategory::Conflicts),
            "Settings" => Ok(BrainstormCategory::Settings),
            "Relationships" => Ok(BrainstormCategory::Relationships),
            "Backstory" => Ok(BrainstormCategory::Backstory),
            other => Ok(BrainstormCategory::Custom(other.to_string())),
        }
    }
}

impl BrainstormCategory {
    pub fn get_prompt_template(&self) -> String {
        match self {
            BrainstormCategory::Characters => {
                "Generate creative character ideas including their personality traits, backgrounds, motivations, and unique characteristics. Consider diverse perspectives and compelling flaws.".to_string()
            },
            BrainstormCategory::Plot => {
                "Brainstorm plot ideas, story arcs, plot twists, and narrative structures. Focus on compelling conflicts and satisfying resolutions.".to_string()
            },
            BrainstormCategory::Worldbuilding => {
                "Create worldbuilding elements including cultures, societies, magic systems, technology, geography, and unique world features.".to_string()
            },
            BrainstormCategory::Dialogue => {
                "Generate dialogue ideas, character voice patterns, memorable quotes, and conversation scenarios that reveal character and advance plot.".to_string()
            },
            BrainstormCategory::Scenes => {
                "Brainstorm scene ideas, dramatic moments, action sequences, and emotional beats that would be compelling to read.".to_string()
            },
            BrainstormCategory::Themes => {
                "Explore thematic elements, deeper meanings, philosophical questions, and universal truths that could be woven throughout the story.".to_string()
            },
            BrainstormCategory::Conflicts => {
                "Generate conflict ideas including internal struggles, interpersonal tensions, societal issues, and external obstacles.".to_string()
            },
            BrainstormCategory::Settings => {
                "Create setting ideas including locations, environments, atmospheres, and places that would enhance the story's mood and themes.".to_string()
            },
            BrainstormCategory::Relationships => {
                "Brainstorm relationship dynamics, character interactions, romantic elements, friendships, rivalries, and family connections.".to_string()
            },
            BrainstormCategory::Backstory => {
                "Generate backstory elements, character histories, past events, formative experiences, and hidden secrets that shape the present narrative.".to_string()
            },
            BrainstormCategory::Custom(_) => {
                "Generate creative ideas related to the specified topic. Think outside the box and explore unique angles and perspectives.".to_string()
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormIdea {
    pub id: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub rating: Option<i32>, // 1-5 stars
    pub notes: String,
    pub is_keeper: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormRequest {
    pub project_id: String,
    pub category: BrainstormCategory,
    pub seed_prompt: Option<String>,
    pub context: Option<String>,
    pub num_ideas: usize,
    pub creativity_level: i32, // 1-10
    pub focus_areas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormConfig {
    pub default_ideas_per_session: usize,
    pub max_ideas_per_session: usize,
    pub enable_auto_tagging: bool,
    pub enable_idea_scoring: bool,
    pub creativity_boost_threshold: i32,
}

impl Default for BrainstormConfig {
    fn default() -> Self {
        Self {
            default_ideas_per_session: 10,
            max_ideas_per_session: 50,
            enable_auto_tagging: true,
            enable_idea_scoring: true,
            creativity_boost_threshold: 7,
        }
    }
}

pub struct BrainstormEngine {
    config: BrainstormConfig,
    sessions: HashMap<String, BrainstormSession>,
    idea_templates: HashMap<String, Vec<String>>,
}

impl BrainstormEngine {
    pub fn new(config: BrainstormConfig) -> Self {
        let mut engine = Self {
            config,
            sessions: HashMap::new(),
            idea_templates: HashMap::new(),
        };
        engine.initialize_templates();
        engine
    }

    fn initialize_templates(&mut self) {
        // Character templates
        self.idea_templates.insert("Characters".to_string(), vec![
            "A character who {trait} but secretly {hidden_trait}".to_string(),
            "Someone whose greatest strength is also their greatest weakness".to_string(),
            "A character who must choose between {value1} and {value2}".to_string(),
            "An unlikely hero with {unusual_background}".to_string(),
            "A character haunted by {past_event}".to_string(),
        ]);

        // Plot templates
        self.idea_templates.insert("Plot".to_string(), vec![
            "What if {normal_situation} but {twist}?".to_string(),
            "A story where the protagonist discovers {revelation}".to_string(),
            "The consequences of {action} lead to {unexpected_outcome}".to_string(),
            "A race against time to {goal} before {deadline}".to_string(),
            "Two characters must {collaborate} despite {conflict}".to_string(),
        ]);

        // Worldbuilding templates
        self.idea_templates.insert("Worldbuilding".to_string(), vec![
            "A world where {natural_law} works differently".to_string(),
            "A society built around {central_concept}".to_string(),
            "Technology that {capability} but {limitation}".to_string(),
            "A culture where {value} is the highest virtue".to_string(),
            "A place where {phenomenon} occurs regularly".to_string(),
        ]);
    }

    pub fn create_session(&mut self, request: BrainstormRequest) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = Uuid::new_v4().to_string();
        
        let session = BrainstormSession {
            id: session_id.clone(),
            project_id: request.project_id,
            category: request.category,
            seed_prompt: request.seed_prompt,
            ideas: Vec::new(),
            keepers: Vec::new(),
            session_notes: String::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.sessions.insert(session_id.clone(), session);
        Ok(session_id)
    }

    pub fn generate_ideas(
        &mut self,
        session_id: &str,
        request: &BrainstormRequest,
    ) -> Result<Vec<BrainstormIdea>, Box<dyn std::error::Error>> {
        let mut ideas = Vec::new();
        let num_ideas = request.num_ideas.min(self.config.max_ideas_per_session);

        // Generate base prompt
let base_prompt = self.build_generation_prompt(request)?;
// TODO: Use base_prompt with AI provider when implemented
        
        // For now, we'll generate template-based ideas
        // In a real implementation, this would call the AI provider
        let templates = self.idea_templates.get(&request.category.to_string())
            .cloned()
            .unwrap_or_else(|| vec!["Generate a creative idea for {topic}".to_string()]);

        for i in 0..num_ideas {
            let template = &templates[i % templates.len()];
            let idea_content = self.generate_idea_from_template(template, request)?;
            
            // Pre-compute values that require immutable borrows
            let tags = if self.config.enable_auto_tagging {
                self.auto_generate_tags(&idea_content)
            } else {
                Vec::new()
            };
            
            let rating = if self.config.enable_idea_scoring {
                Some(self.score_idea(&idea_content))
            } else {
                None
            };
            
            let idea = BrainstormIdea {
                id: Uuid::new_v4().to_string(),
                content: idea_content,
                category: request.category.to_string(),
                tags,
                rating,
                notes: String::new(),
                is_keeper: false,
                created_at: chrono::Utc::now(),
            };

            ideas.push(idea.clone());
        }

        // Now get mutable reference to session and add all ideas
        let session = self.sessions.get_mut(session_id)
            .ok_or("Session not found")?;
        
        for idea in &ideas {
            session.ideas.push(idea.clone());
        }
        session.updated_at = chrono::Utc::now();
        
        Ok(ideas)
    }

    fn build_generation_prompt(&self, request: &BrainstormRequest) -> Result<String, Box<dyn std::error::Error>> {
        let mut prompt = request.category.get_prompt_template();
        
        if let Some(seed) = &request.seed_prompt {
            prompt.push_str(&format!("\n\nSeed idea: {}", seed));
        }
        
        if let Some(context) = &request.context {
            prompt.push_str(&format!("\n\nContext: {}", context));
        }
        
        if !request.focus_areas.is_empty() {
            prompt.push_str(&format!("\n\nFocus on: {}", request.focus_areas.join(", ")));
        }
        
        // Adjust creativity based on level
        if request.creativity_level >= self.config.creativity_boost_threshold {
            prompt.push_str("\n\nBe especially creative and think outside the box. Explore unconventional ideas and unique perspectives.");
        }
        
        prompt.push_str(&format!("\n\nGenerate {} distinct ideas.", request.num_ideas));
        
        Ok(prompt)
    }

    fn generate_idea_from_template(
        &self,
        template: &str,
        request: &BrainstormRequest,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Simple template filling for demonstration
        // In a real implementation, this would use AI generation
        let mut idea = template.to_string();
        
        // Replace common placeholders with category-appropriate content
        match request.category {
            BrainstormCategory::Characters => {
                idea = idea.replace("{trait}", "appears confident");
                idea = idea.replace("{hidden_trait}", "struggles with deep insecurity");
                idea = idea.replace("{value1}", "loyalty to family");
                idea = idea.replace("{value2}", "personal freedom");
                idea = idea.replace("{unusual_background}", "a background in quantum physics");
                idea = idea.replace("{past_event}", "a childhood accident they caused");
            },
            BrainstormCategory::Plot => {
                idea = idea.replace("{normal_situation}", "a routine job interview");
                idea = idea.replace("{twist}", "the interviewer is from the future");
                idea = idea.replace("{revelation}", "they're living in a simulation");
                idea = idea.replace("{action}", "telling a small lie");
                idea = idea.replace("{unexpected_outcome}", "saving the world");
                idea = idea.replace("{goal}", "deliver a message");
                idea = idea.replace("{deadline}", "the sun sets");
                idea = idea.replace("{collaborate}", "work together");
                idea = idea.replace("{conflict}", "being from rival families");
            },
            BrainstormCategory::Worldbuilding => {
                idea = idea.replace("{natural_law}", "gravity");
                idea = idea.replace("{central_concept}", "the power of music");
                idea = idea.replace("{capability}", "reads minds");
                idea = idea.replace("{limitation}", "only works on Tuesdays");
                idea = idea.replace("{value}", "honesty");
                idea = idea.replace("{phenomenon}", "time reversal");
            },
            _ => {
                idea = idea.replace("{topic}", &request.category.to_string().to_lowercase());
            }
        }
        
        Ok(idea)
    }

    fn auto_generate_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let content_lower = content.to_lowercase();
        
        // Character-related tags
        if content_lower.contains("character") || content_lower.contains("person") {
            tags.push("character".to_string());
        }
        
        // Emotion tags
        let emotions = ["love", "fear", "anger", "joy", "sadness", "hope", "despair"];
        for emotion in &emotions {
            if content_lower.contains(emotion) {
                tags.push(emotion.to_string());
            }
        }
        
        // Conflict tags
        if content_lower.contains("conflict") || content_lower.contains("struggle") {
            tags.push("conflict".to_string());
        }
        
        // Mystery tags
        if content_lower.contains("secret") || content_lower.contains("hidden") {
            tags.push("mystery".to_string());
        }
        
        // Relationship tags
        if content_lower.contains("family") || content_lower.contains("friend") || content_lower.contains("love") {
            tags.push("relationship".to_string());
        }
        
        tags
    }

    fn score_idea(&self, content: &str) -> i32 {
        let mut score = 3; // Base score
        
        // Length bonus
        if content.len() > 100 {
            score += 1;
        }
        
        // Complexity bonus
        if content.contains("but") || content.contains("however") || content.contains("despite") {
            score += 1;
        }
        
        // Emotional content bonus
        let emotions = ["love", "fear", "anger", "joy", "sadness", "hope", "despair"];
        if emotions.iter().any(|&emotion| content.to_lowercase().contains(emotion)) {
            score += 1;
        }
        
        score.min(5).max(1)
    }

    pub fn mark_as_keeper(&mut self, session_id: &str, idea_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.sessions.get_mut(session_id)
            .ok_or("Session not found")?;
        
        // Find and mark the idea
        if let Some(idea) = session.ideas.iter_mut().find(|i| i.id == idea_id) {
            idea.is_keeper = true;
            if !session.keepers.contains(&idea_id.to_string()) {
                session.keepers.push(idea_id.to_string());
            }
            session.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Idea not found".into())
        }
    }

    pub fn remove_keeper(&mut self, session_id: &str, idea_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.sessions.get_mut(session_id)
            .ok_or("Session not found")?;
        
        // Find and unmark the idea
        if let Some(idea) = session.ideas.iter_mut().find(|i| i.id == idea_id) {
            idea.is_keeper = false;
            session.keepers.retain(|id| id != idea_id);
            session.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Idea not found".into())
        }
    }

    pub fn rate_idea(&mut self, session_id: &str, idea_id: &str, rating: i32) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.sessions.get_mut(session_id)
            .ok_or("Session not found")?;
        
        if let Some(idea) = session.ideas.iter_mut().find(|i| i.id == idea_id) {
            idea.rating = Some(rating.min(5).max(1));
            session.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Idea not found".into())
        }
    }

    pub fn add_idea_notes(&mut self, session_id: &str, idea_id: &str, notes: String) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.sessions.get_mut(session_id)
            .ok_or("Session not found")?;
        
        if let Some(idea) = session.ideas.iter_mut().find(|i| i.id == idea_id) {
            idea.notes = notes;
            session.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err("Idea not found".into())
        }
    }

    pub fn get_session(&self, session_id: &str) -> Option<&BrainstormSession> {
        self.sessions.get(session_id)
    }

    pub fn list_project_sessions(&self, project_id: &str) -> Vec<&BrainstormSession> {
        self.sessions
            .values()
            .filter(|session| session.project_id == project_id)
            .collect()
    }

    pub fn get_keepers(&self, session_id: &str) -> Result<Vec<&BrainstormIdea>, Box<dyn std::error::Error>> {
        let session = self.sessions.get(session_id)
            .ok_or("Session not found")?;
        
        Ok(session.ideas.iter().filter(|idea| idea.is_keeper).collect())
    }

    pub fn export_keepers_to_story_bible(&self, session_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let keepers = self.get_keepers(session_id)?;
        let session = self.sessions.get(session_id).ok_or("Session not found")?;
        
        let mut export = format!("# {} Ideas - Keepers\n\n", session.category.to_string());
        
        for idea in keepers {
            export.push_str(&format!("## {}\n", idea.content));
            if !idea.notes.is_empty() {
                export.push_str(&format!("**Notes:** {}\n", idea.notes));
            }
            if !idea.tags.is_empty() {
                export.push_str(&format!("**Tags:** {}\n", idea.tags.join(", ")));
            }
            if let Some(rating) = idea.rating {
                export.push_str(&format!("**Rating:** {}/5\n", rating));
            }
            export.push_str("\n");
        }
        
        Ok(export)
    }
}

impl Default for BrainstormEngine {
    fn default() -> Self {
        Self::new(BrainstormConfig::default())
    }
}