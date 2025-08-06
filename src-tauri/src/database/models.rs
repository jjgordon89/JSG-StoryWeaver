//! Database models for StoryWeaver
//! Defines the core data structures and their database representations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Import our new models
mod folder;
mod series;
mod document_link;
mod document_version;
mod deleted_item;
mod app_settings;
mod performance_metric;

// Re-export all models
pub use folder::*;
pub use series::*;
pub use document_link::*;
pub use document_version::*;
pub use deleted_item::*;
pub use app_settings::*;
pub use performance_metric::*;

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

/// Character model - represents characters in the story bible
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Character {
    pub id: String,
    pub project_id: String,
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
            project_id,
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
        }
    }
}

impl Location {
    pub fn new(project_id: String, name: String, location_type: LocationType) -> Self {
        let now = Utc::now();
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
            created_at: now,
            updated_at: now,
            metadata: "{}".to_string(),
        }
    }
}
