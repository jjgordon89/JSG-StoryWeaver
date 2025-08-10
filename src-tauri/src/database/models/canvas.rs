//! Canvas models for visual story planning and outline templates

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Canvas model for visual story planning
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Canvas {
    pub id: i32,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub canvas_data: String, // JSON string of canvas elements and layout
    pub template_type: Option<OutlineTemplateType>,
    pub width: i32,
    pub height: i32,
    pub zoom_level: f32,
    pub viewport_x: f32,
    pub viewport_y: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Canvas element for individual story planning components
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CanvasElement {
    pub id: i32,
    pub canvas_id: i32,
    pub element_type: CanvasElementType,
    pub title: String,
    pub content: String,
    pub position_x: f32,
    pub position_y: f32,
    pub width: f32,
    pub height: f32,
    pub color: String,
    pub metadata: String, // JSON string for element-specific data
    pub connections: String, // JSON array of connected element IDs
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Canvas type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum CanvasType {
    #[sqlx(rename = "story_outline")]
    StoryOutline,
    #[sqlx(rename = "character_map")]
    CharacterMap,
    #[sqlx(rename = "world_building")]
    WorldBuilding,
    #[sqlx(rename = "timeline")]
    Timeline,
    #[sqlx(rename = "plot_structure")]
    PlotStructure,
    #[sqlx(rename = "mind_map")]
    MindMap,
    #[sqlx(rename = "free_form")]
    FreeForm,
}

impl std::fmt::Display for CanvasType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CanvasType::StoryOutline => "story_outline",
            CanvasType::CharacterMap => "character_map",
            CanvasType::WorldBuilding => "world_building",
            CanvasType::Timeline => "timeline",
            CanvasType::PlotStructure => "plot_structure",
            CanvasType::MindMap => "mind_map",
            CanvasType::FreeForm => "free_form",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for CanvasType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "story_outline" => Ok(CanvasType::StoryOutline),
            "character_map" => Ok(CanvasType::CharacterMap),
            "world_building" => Ok(CanvasType::WorldBuilding),
            "timeline" => Ok(CanvasType::Timeline),
            "plot_structure" => Ok(CanvasType::PlotStructure),
            "mind_map" => Ok(CanvasType::MindMap),
            "free_form" => Ok(CanvasType::FreeForm),
            _ => Err(format!("Unknown canvas type: {}", s)),
        }
    }
}

/// Canvas element type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum CanvasElementType {
    #[sqlx(rename = "plot_point")]
    PlotPoint,
    #[sqlx(rename = "character_arc")]
    CharacterArc,
    #[sqlx(rename = "scene")]
    Scene,
    #[sqlx(rename = "chapter")]
    Chapter,
    #[sqlx(rename = "act")]
    Act,
    #[sqlx(rename = "note")]
    Note,
    #[sqlx(rename = "connection")]
    Connection,
    #[sqlx(rename = "timeline_event")]
    TimelineEvent,
    #[sqlx(rename = "theme")]
    Theme,
    #[sqlx(rename = "conflict")]
    Conflict,
    #[sqlx(rename = "text_box")]
    TextBox,
    #[sqlx(rename = "sticky_note")]
    StickyNote,
}

impl std::fmt::Display for CanvasElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CanvasElementType::PlotPoint => "plot_point",
            CanvasElementType::CharacterArc => "character_arc",
            CanvasElementType::Scene => "scene",
            CanvasElementType::Chapter => "chapter",
            CanvasElementType::Act => "act",
            CanvasElementType::Note => "note",
            CanvasElementType::Connection => "connection",
            CanvasElementType::TimelineEvent => "timeline_event",
            CanvasElementType::Theme => "theme",
            CanvasElementType::Conflict => "conflict",
            CanvasElementType::TextBox => "text_box",
            CanvasElementType::StickyNote => "sticky_note",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for CanvasElementType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plot_point" => Ok(CanvasElementType::PlotPoint),
            "character_arc" => Ok(CanvasElementType::CharacterArc),
            "scene" => Ok(CanvasElementType::Scene),
            "chapter" => Ok(CanvasElementType::Chapter),
            "act" => Ok(CanvasElementType::Act),
            "note" => Ok(CanvasElementType::Note),
            "connection" => Ok(CanvasElementType::Connection),
            "timeline_event" => Ok(CanvasElementType::TimelineEvent),
            "theme" => Ok(CanvasElementType::Theme),
            "conflict" => Ok(CanvasElementType::Conflict),
            "text_box" => Ok(CanvasElementType::TextBox),
            "sticky_note" => Ok(CanvasElementType::StickyNote),
            _ => Err(format!("Unknown canvas element type: {}", s)),
        }
    }
}

/// Outline template type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum OutlineTemplateType {
    #[sqlx(rename = "heros_journey")]
    HerosJourney,
    #[sqlx(rename = "hollywood_beats")]
    HollywoodBeats,
    #[sqlx(rename = "story_circle")]
    StoryCircle,
    #[sqlx(rename = "romance_outline")]
    RomanceOutline,
    #[sqlx(rename = "three_act")]
    ThreeAct,
    #[sqlx(rename = "save_the_cat")]
    SaveTheCat,
    #[sqlx(rename = "snowflake")]
    Snowflake,
    #[sqlx(rename = "seven_point")]
    SevenPoint,
    #[sqlx(rename = "custom")]
    Custom,
}

impl std::fmt::Display for OutlineTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            OutlineTemplateType::HerosJourney => "heros_journey",
            OutlineTemplateType::HollywoodBeats => "hollywood_beats",
            OutlineTemplateType::StoryCircle => "story_circle",
            OutlineTemplateType::RomanceOutline => "romance_outline",
            OutlineTemplateType::ThreeAct => "three_act",
            OutlineTemplateType::SaveTheCat => "save_the_cat",
            OutlineTemplateType::Snowflake => "snowflake",
            OutlineTemplateType::SevenPoint => "seven_point",
            OutlineTemplateType::Custom => "custom",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for OutlineTemplateType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "heros_journey" => Ok(OutlineTemplateType::HerosJourney),
            "hollywood_beats" => Ok(OutlineTemplateType::HollywoodBeats),
            "story_circle" => Ok(OutlineTemplateType::StoryCircle),
            "romance_outline" => Ok(OutlineTemplateType::RomanceOutline),
            "three_act" => Ok(OutlineTemplateType::ThreeAct),
            "custom" => Ok(OutlineTemplateType::Custom),
            _ => Err(format!("Invalid outline template type: {}", s)),
        }
    }
}

/// Outline template model for predefined story structures
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OutlineTemplate {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub template_type: OutlineTemplateType,
    pub template_data: String, // JSON string of template structure
    pub is_official: bool,
    pub created_at: DateTime<Utc>,
}

/// Canvas connection for linking story elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasConnection {
    pub from_element_id: i32,
    pub to_element_id: i32,
    pub connection_type: ConnectionType,
    pub label: Option<String>,
    pub style: ConnectionStyle,
}

/// Connection type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Sequence,
    Cause,
    Conflict,
    Resolution,
    Character,
    Theme,
    Custom,
}

/// Connection style for visual representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStyle {
    pub line_type: LineType,
    pub color: String,
    pub thickness: f32,
    pub arrow_type: ArrowType,
}

/// Line type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineType {
    Solid,
    Dashed,
    Dotted,
    Curved,
}

/// Arrow type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowType {
    None,
    Single,
    Double,
    Diamond,
    Circle,
}

/// Canvas export request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasExportRequest {
    pub canvas_id: i32,
    pub export_format: ExportFormat,
    pub include_connections: bool,
    pub include_metadata: bool,
}

/// Export format enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    StoryBible,
    Outline,
    Json,
    Markdown,
    Image,
    PNG,
    SVG,
    PDF,
    JSON,
}

impl std::fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportFormat::StoryBible => write!(f, "story_bible"),
            ExportFormat::Outline => write!(f, "outline"),
            ExportFormat::Json => write!(f, "json"),
            ExportFormat::Markdown => write!(f, "markdown"),
            ExportFormat::Image => write!(f, "image"),
            ExportFormat::PNG => write!(f, "png"),
            ExportFormat::SVG => write!(f, "svg"),
            ExportFormat::PDF => write!(f, "pdf"),
            ExportFormat::JSON => write!(f, "json"),
        }
    }
}

/// Canvas export result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasExportResult {
    pub canvas_id: String,
    pub format: ExportFormat,
    pub data: serde_json::Value,
    pub file_size: i64,
    pub exported_at: DateTime<Utc>,
}

/// Canvas snapshot for version control
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CanvasSnapshot {
    pub id: i32,
    pub canvas_id: i32,
    pub snapshot_name: String,
    pub canvas_data: String, // JSON string of canvas state
    pub created_at: DateTime<Utc>,
}

/// Canvas collaboration session for real-time editing
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CanvasCollaborationSession {
    pub id: i64,
    pub canvas_id: i64,
    pub session_token: String,
    pub host_user: String,
    pub participants: String, // JSON array of participants
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Canvas operation for real-time synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasOperation {
    pub canvas_id: String,
    pub operation_type: CanvasOperationType,
    pub element_id: Option<i32>,
    pub data: String, // JSON string of operation data
    pub user_token: String,
    pub timestamp: DateTime<Utc>,
}

/// Canvas operation type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanvasOperationType {
    CreateElement,
    UpdateElement,
    DeleteElement,
    MoveElement,
    ResizeElement,
    CreateConnection,
    DeleteConnection,
    UpdateCanvas,
}

impl std::fmt::Display for CanvasOperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanvasOperationType::CreateElement => write!(f, "create_element"),
            CanvasOperationType::UpdateElement => write!(f, "update_element"),
            CanvasOperationType::DeleteElement => write!(f, "delete_element"),
            CanvasOperationType::MoveElement => write!(f, "move_element"),
            CanvasOperationType::ResizeElement => write!(f, "resize_element"),
            CanvasOperationType::CreateConnection => write!(f, "create_connection"),
            CanvasOperationType::DeleteConnection => write!(f, "delete_connection"),
            CanvasOperationType::UpdateCanvas => write!(f, "update_canvas"),
        }
    }
}