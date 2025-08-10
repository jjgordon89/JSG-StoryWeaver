//! Plugin system models for custom AI tools and marketplace

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

/// Plugin model for custom AI tools
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub prompt_template: String,
    pub variables: String, // JSON string of PluginVariable array
    pub ai_model: String,
    pub temperature: f32,
    pub max_tokens: Option<i32>,
    pub stop_sequences: Option<String>, // JSON array of strings
    pub category: PluginCategory,
    pub tags: String, // JSON array of strings
    pub is_multi_stage: bool,
    pub stage_count: i32,
    pub creator_id: Option<String>,
    pub is_public: bool,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plugin variable for dynamic input handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVariable {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub variable_type: PluginVariableType,
    pub required: bool,
    pub default_value: Option<String>,
    pub options: Option<Vec<String>>, // For select/dropdown types
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
}

/// Plugin variable type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginVariableType {
    Text,
    TextArea,
    Number,
    Boolean,
    Select,
    StoryBibleCharacter,
    StoryBibleLocation,
    StoryBibleEvent,
    DocumentContent,
    SelectedText,
}

/// Plugin category enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum PluginCategory {
    #[sqlx(rename = "writing")]
    Writing,
    #[sqlx(rename = "editing")]
    Editing,
    #[sqlx(rename = "analysis")]
    Analysis,
    #[sqlx(rename = "brainstorming")]
    Brainstorming,
    #[sqlx(rename = "research")]
    Research,
    #[sqlx(rename = "formatting")]
    Formatting,
    #[sqlx(rename = "text_processing")]
    TextProcessing,
    #[sqlx(rename = "ai_integration")]
    AIIntegration,
    #[sqlx(rename = "export")]
    Export,
    #[sqlx(rename = "import")]
    Import,
    #[sqlx(rename = "theme")]
    Theme,
    #[sqlx(rename = "workflow")]
    Workflow,
    #[sqlx(rename = "collaboration")]
    Collaboration,
    #[sqlx(rename = "other")]
    Other,
}

impl std::str::FromStr for PluginCategory {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "writing" => Ok(PluginCategory::Writing),
            "editing" => Ok(PluginCategory::Editing),
            "analysis" => Ok(PluginCategory::Analysis),
            "brainstorming" => Ok(PluginCategory::Brainstorming),
            "research" => Ok(PluginCategory::Research),
            "formatting" => Ok(PluginCategory::Formatting),
            "text_processing" => Ok(PluginCategory::TextProcessing),
            "ai_integration" => Ok(PluginCategory::AIIntegration),
            "export" => Ok(PluginCategory::Export),
            "import" => Ok(PluginCategory::Import),
            "theme" => Ok(PluginCategory::Theme),
            "workflow" => Ok(PluginCategory::Workflow),
            "collaboration" => Ok(PluginCategory::Collaboration),
            "other" => Ok(PluginCategory::Other),
            _ => Err(format!("Invalid plugin category: {}", s)),
        }
    }
}

/// Plugin execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginExecutionRequest {
    pub plugin_id: i32,
    pub variables: HashMap<String, String>,
    pub document_id: Option<String>,
    pub selected_text: Option<String>,
    pub cursor_position: Option<i32>,
}

/// Plugin execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginExecutionResult {
    pub success: bool,
    pub result_text: Option<String>,
    pub error_message: Option<String>,
    pub credits_used: i32,
    pub execution_time_ms: i64,
    pub stage_results: Option<Vec<String>>, // For multi-stage plugins
}

/// Plugin marketplace entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PluginMarketplaceEntry {
    pub id: i32,
    pub plugin_id: i32,
    pub creator_name: String,
    pub visibility: PluginVisibility,
    pub download_count: i32,
    pub rating_average: f32,
    pub rating_count: i32,
    pub featured: bool,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plugin visibility enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "text")]
pub enum PluginVisibility {
    #[sqlx(rename = "published")]
    Published,
    #[sqlx(rename = "public")]
    Public,
    #[sqlx(rename = "unlisted")]
    Unlisted,
    #[sqlx(rename = "private")]
    Private,
}

impl std::str::FromStr for PluginVisibility {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "published" => Ok(PluginVisibility::Published),
            "public" => Ok(PluginVisibility::Public),
            "unlisted" => Ok(PluginVisibility::Unlisted),
            "private" => Ok(PluginVisibility::Private),
            _ => Err(format!("Invalid plugin visibility: {}", s)),
        }
    }
}

/// Plugin rating model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PluginRating {
    pub id: i32,
    pub plugin_id: i32,
    pub user_identifier: String, // Anonymous identifier
    pub rating: i32, // 1-5 stars
    pub review_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Plugin usage statistics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PluginUsageStats {
    pub id: i32,
    pub plugin_id: i32,
    pub user_identifier: String,
    pub execution_count: i32,
    pub total_credits_used: i32,
    pub last_used: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Plugin search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSearchResult {
    pub plugin: Plugin,
    pub marketplace_entry: PluginMarketplaceEntry,
    pub relevance_score: f32,
}

/// Plugin sort order enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginSortOrder {
    Relevance,
    Rating,
    Downloads,
    Recent,
    Name,
}

/// Plugin template for common writing tasks
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PluginTemplate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: PluginCategory,
    pub template_data: String, // JSON string of plugin configuration
    pub is_official: bool,
    pub created_at: DateTime<Utc>,
}

/// Plugin execution history for debugging and analytics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PluginExecutionHistory {
    pub id: i32,
    pub plugin_id: i32,
    pub user_identifier: String,
    pub execution_request: String, // JSON string of request
    pub execution_result: String, // JSON string of result
    pub credits_used: i32,
    pub execution_time_ms: i64,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Display trait implementations
impl std::fmt::Display for PluginCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginCategory::Writing => write!(f, "writing"),
            PluginCategory::Editing => write!(f, "editing"),
            PluginCategory::Analysis => write!(f, "analysis"),
            PluginCategory::Brainstorming => write!(f, "brainstorming"),
            PluginCategory::Research => write!(f, "research"),
            PluginCategory::Formatting => write!(f, "formatting"),
            PluginCategory::Other => write!(f, "other"),
        }
    }
}

impl std::fmt::Display for PluginVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginVisibility::Published => write!(f, "published"),
            PluginVisibility::Unlisted => write!(f, "unlisted"),
            PluginVisibility::Private => write!(f, "private"),
        }
    }
}