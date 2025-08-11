//! Database models for StoryWeaver
//! Defines the core data structures and their database representations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Import our new models
pub mod folder;
pub mod series;
pub mod document_link;
pub mod document_version;
pub mod deleted_item;
pub mod app_settings;
pub mod performance_metric;
pub mod collaboration;
pub mod plugin;
pub mod canvas;
pub mod ai;

// Re-export all models
pub use folder::*;
pub use series::*;
pub use document_link::*;
pub use document_version::*;
pub use deleted_item::*;
pub use app_settings::*;
pub use performance_metric::*;
pub use collaboration::*;
pub use plugin::*;
pub use canvas::*;
pub use ai::*;

/// Project model - represents a writing project
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub target_word_count: Option<i32>,
    pub current_word_count: i32,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: String, // JSON string for project-specific settings
    pub series_id: Option<String>, // Reference to series if part of one
    pub folder_id: Option<String>, // Reference to folder for organization
}

/// Project status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ProjectStatus {
    #[sqlx(rename = "planning")]
    Planning,
    #[sqlx(rename = "drafting")]
    Drafting,
    #[sqlx(rename = "revising")]
    Revising,
    #[sqlx(rename = "completed")]
    Completed,
    #[sqlx(rename = "archived")]
    Archived,
}

/// Document model - represents individual documents within a project
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub content: String,
    pub document_type: DocumentType,
    pub order_index: i32,
    pub word_count: i32,
    pub parent_id: Option<String>, // For hierarchical documents
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: String, // JSON string for document-specific metadata
    pub folder_id: Option<String>, // Reference to folder for organization
}

/// Document type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum DocumentType {
    #[sqlx(rename = "chapter")]
    Chapter,
    #[sqlx(rename = "scene")]
    Scene,
    #[sqlx(rename = "outline")]
    Outline,
    #[sqlx(rename = "notes")]
    Notes,
    #[sqlx(rename = "research")]
    Research,
    #[sqlx(rename = "synopsis")]
    Synopsis,
}

impl std::str::FromStr for DocumentType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "chapter" => Ok(DocumentType::Chapter),
            "scene" => Ok(DocumentType::Scene),
            "outline" => Ok(DocumentType::Outline),
            "notes" => Ok(DocumentType::Notes),
            "research" => Ok(DocumentType::Research),
            "synopsis" => Ok(DocumentType::Synopsis),
            _ => Err(format!("Invalid document type: {}", s)),
        }
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DocumentType::Chapter => "chapter",
            DocumentType::Scene => "scene",
            DocumentType::Outline => "outline",
            DocumentType::Notes => "notes",
            DocumentType::Research => "research",
            DocumentType::Synopsis => "synopsis",
        };
        write!(f, "{}", s)
    }
}

/// Character model - represents characters in the story bible
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: String,
    pub project_id: String,
    pub series_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub role: CharacterRole,
    pub age: Option<i32>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub goals: Option<String>,
    pub relationships: String, // JSON string for character relationships
    pub visibility: VisibilityLevel,
    pub original_project_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: String, // JSON string for additional character data
}

/// Character role enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum CharacterRole {
    #[sqlx(rename = "protagonist")]
    Protagonist,
    #[sqlx(rename = "antagonist")]
    Antagonist,
    #[sqlx(rename = "supporting")]
    Supporting,
    #[sqlx(rename = "minor")]
    Minor,
    #[sqlx(rename = "background")]
    Background,
}

/// Location model - represents locations in the story bible
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location_type: LocationType,
    pub geography: Option<String>,
    pub climate: Option<String>,
    pub culture: Option<String>,
    pub history: Option<String>,
    pub significance: Option<String>,
    pub visibility: VisibilityLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: String, // JSON string for additional location data
}

/// Location type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum LocationType {
    #[sqlx(rename = "city")]
    City,
    #[sqlx(rename = "building")]
    Building,
    #[sqlx(rename = "room")]
    Room,
    #[sqlx(rename = "landscape")]
    Landscape,
    #[sqlx(rename = "fictional")]
    Fictional,
    #[sqlx(rename = "historical")]
    Historical,
}

/// Visibility level for story bible elements
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum VisibilityLevel {
    #[sqlx(rename = "always")]
    Always,
    #[sqlx(rename = "relevant")]
    Relevant,
    #[sqlx(rename = "manual")]
    Manual,
    #[sqlx(rename = "hidden")]
    Hidden,
}

/// Timeline event model - for tracking story chronology
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimelineEvent {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub description: Option<String>,
    pub event_date: Option<String>, // Story-internal date/time
    pub real_date: Option<DateTime<Utc>>, // Real-world date if applicable
    pub importance: EventImportance,
    pub characters_involved: String, // JSON array of character IDs
    pub locations_involved: String, // JSON array of location IDs
    pub visibility: VisibilityLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Event importance enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum EventImportance {
    #[sqlx(rename = "critical")]
    Critical,
    #[sqlx(rename = "major")]
    Major,
    #[sqlx(rename = "minor")]
    Minor,
    #[sqlx(rename = "background")]
    Background,
}

/// General importance enumeration (for locations, etc.)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum Importance {
    #[sqlx(rename = "critical")]
    Critical,
    #[sqlx(rename = "high")]
    High,
    #[sqlx(rename = "medium")]
    Medium,
    #[sqlx(rename = "low")]
    Low,
}

/// Plot thread model - for tracking multiple storylines
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlotThread {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: PlotThreadStatus,
    pub priority: ThreadPriority,
    pub characters_involved: String, // JSON array of character IDs
    pub documents_involved: String, // JSON array of document IDs
    pub visibility: VisibilityLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plot thread status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum PlotThreadStatus {
    #[sqlx(rename = "planned")]
    Planned,
    #[sqlx(rename = "active")]
    Active,
    #[sqlx(rename = "resolved")]
    Resolved,
    #[sqlx(rename = "abandoned")]
    Abandoned,
}

/// Thread priority enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ThreadPriority {
    #[sqlx(rename = "main")]
    Main,
    #[sqlx(rename = "subplot")]
    Subplot,
    #[sqlx(rename = "background")]
    Background,
}

/// AI generation history model - for tracking AI-generated content
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AIGenerationHistory {
    pub id: String,
    pub project_id: String,
    pub document_id: Option<String>,
    pub generation_type: AIGenerationType,
    pub provider: String,
    pub model: String,
    pub prompt: String,
    pub response: String,
    pub token_count: i32,
    pub cost_estimate: Option<f64>,
    pub context_used: String, // JSON string of context elements used
    pub created_at: DateTime<Utc>,
}

/// AI generation type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum AIGenerationType {
    #[sqlx(rename = "auto_write")]
    AutoWrite,
    #[sqlx(rename = "expand")]
    Expand,
    #[sqlx(rename = "rewrite")]
    Rewrite,
    #[sqlx(rename = "describe")]
    Describe,
    #[sqlx(rename = "brainstorm")]
    Brainstorm,
    #[sqlx(rename = "outline")]
    Outline,
    #[sqlx(rename = "character_development")]
    CharacterDevelopment,
    #[sqlx(rename = "world_building")]
    WorldBuilding,
}

/// User preferences model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserPreferences {
    pub id: String,
    pub ai_provider_preferences: String, // JSON string
    pub writing_preferences: String, // JSON string
    pub ui_preferences: String, // JSON string
    pub plugin_preferences: String, // JSON string
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Helper functions for model creation
impl Project {
    pub fn new(name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            genre: None,
            target_word_count: None,
            current_word_count: 0,
            status: ProjectStatus::Planning,
            created_at: now,
            updated_at: now,
            settings: "{}".to_string(),
            series_id: None,
            folder_id: None,
        }
    }
}

impl Document {
    pub fn new(project_id: String, title: String, document_type: DocumentType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            title,
            content: String::new(),
            document_type,
            order_index: 0,
            word_count: 0,
            parent_id: None,
            created_at: now,
            updated_at: now,
            metadata: "{}".to_string(),
            folder_id: None,
        }
    }
}

impl Character {
    pub fn new(project_id: String, name: String, role: CharacterRole) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.clone(),
            name,
            description: None,
            role,
            age: None,
            appearance: None,
            personality: None,
            background: None,
            goals: None,
            relationships: "{}".to_string(),
            visibility: VisibilityLevel::Relevant,
            created_at: now,
            updated_at: now,
            metadata: "{}".to_string(),
            series_id: None,
            original_project_id: Some(project_id),
        }
    }
}

impl Location {
    pub fn new(project_id: String, name: String, location_type: LocationType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            name,
            description: None,
            location_type,
            geography: None,
            climate: None,
            culture: None,
            history: None,
            significance: None,
            visibility: VisibilityLevel::Relevant,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: "{}".to_string(),
        }
    }
}

/// Story Bible model - core story bible structure
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StoryBible {
    pub id: String,
    pub project_id: String,
    pub braindump: Option<String>,
    pub synopsis: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub style_examples: Option<String>,
    pub pov_mode: String, // 'global', 'per_chapter', 'mixed'
    pub global_pov: Option<String>,
    pub global_tense: Option<String>,
    pub global_character_pov_ids: String, // JSON array of character IDs
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Character trait model - for character trait details
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CharacterTrait {
    pub id: String,
    pub character_id: String,
    pub trait_name: String,
    pub trait_value: Option<String>,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
}

/// Worldbuilding element model - for worldbuilding cards
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WorldElement {
    pub id: String,
    pub project_id: Option<String>,
    pub series_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub element_type: String,
    pub properties: String, // JSON string for custom properties
    pub is_visible: bool,
    pub original_project_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Outline model - for chapter outlines
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Outline {
    pub id: String,
    pub project_id: String,
    pub chapter_number: Option<i32>,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub pov: Option<String>,
    pub tense: Option<String>,
    pub character_pov_ids: String, // JSON array of character IDs
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Outline act model - for acts/dividers in outlines
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OutlineAct {
    pub id: String,
    pub outline_id: String,
    pub act_type: ActType,
    pub act_number: i32,
    pub title: String,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Act type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ActType {
    #[sqlx(rename = "part")]
    Part,
    #[sqlx(rename = "book")]
    Book,
    #[sqlx(rename = "episode")]
    Episode,
    #[sqlx(rename = "section")]
    Section,
}

/// Scene model - for scene building blocks
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scene {
    pub id: String,
    pub outline_id: String,
    pub scene_number: i32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub extra_instructions: Option<String>,
    pub pov: Option<String>,
    pub tense: Option<String>,
    pub character_pov_ids: String, // JSON array of character IDs
    pub word_count_estimate: Option<i32>,
    pub credit_estimate: Option<f64>,
    pub is_validated: bool,
    pub validation_issues: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Implementation for StoryBible
impl StoryBible {
    pub fn new(project_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            braindump: None,
            synopsis: None,
            genre: None,
            style: None,
            style_examples: None,
            pov_mode: "global".to_string(),
            global_pov: Some("3rd Person Limited".to_string()),
            global_tense: Some("Past".to_string()),
            global_character_pov_ids: "[]".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Implementation for CharacterTrait
impl CharacterTrait {
    pub fn new(character_id: String, trait_name: String, trait_value: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            character_id,
            trait_name,
            trait_value,
            is_visible: true,
            created_at: Utc::now(),
        }
    }
}

/// Implementation for WorldElement
impl WorldElement {
    pub fn new(project_id: Option<String>, name: String, element_type: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.clone(),
            series_id: None,
            name,
            description: None,
            element_type,
            properties: "{}".to_string(),
            is_visible: true,
            original_project_id: project_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Implementation for Outline
impl Outline {
    pub fn new(project_id: String, chapter_number: Option<i32>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            project_id,
            chapter_number,
            title: None,
            summary: None,
            pov: None,
            tense: None,
            character_pov_ids: "[]".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Implementation for OutlineAct
impl OutlineAct {
    pub fn new(outline_id: String, act_type: ActType, act_number: i32, title: String, position: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            outline_id,
            act_type,
            act_number,
            title,
            position,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Implementation for Scene
impl Scene {
    pub fn new(outline_id: String, scene_number: i32) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            outline_id,
            scene_number,
            title: None,
            summary: None,
            extra_instructions: None,
            pov: None,
            tense: None,
            character_pov_ids: "[]".to_string(),
            word_count_estimate: None,
            credit_estimate: None,
            is_validated: false,
            validation_issues: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

/// Style example model - for storing user writing style examples and AI analysis
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StyleExample {
    pub id: i64,
    pub project_id: String,
    pub user_id: Option<String>,
    pub example_text: String,
    pub analysis_result: Option<String>, // JSON string containing AI analysis
    pub generated_style_prompt: Option<String>, // AI-generated style prompt based on analysis
    pub word_count: Option<i32>,
    pub created_at: DateTime<Utc>,
}

impl StyleExample {
    pub fn new(project_id: String, user_id: Option<String>, example_text: String) -> Self {
        let word_count = example_text.split_whitespace().count() as i32;
        Self {
            id: 0, // Will be auto-generated by database
            project_id,
            user_id,
            example_text,
            analysis_result: None,
            generated_style_prompt: None,
            word_count: Some(word_count),
            created_at: Utc::now(),
        }
    }
}
