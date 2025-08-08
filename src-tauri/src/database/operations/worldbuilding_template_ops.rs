use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use uuid::Uuid;

/// Worldbuilding template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBuildingTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub element_type: String,
    pub default_properties: Vec<WorldBuildingTemplateProperty>,
    pub is_system: bool, // System templates vs user-created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Worldbuilding template property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBuildingTemplateProperty {
    pub property_name: String,
    pub default_value: Option<String>,
    pub is_required: bool,
    pub description: String,
    pub property_type: String, // "text", "number", "boolean", "list"
}

/// Worldbuilding template operations
pub struct WorldBuildingTemplateOps;

impl WorldBuildingTemplateOps {
    /// Get all system worldbuilding templates
    pub fn get_system_templates() -> Vec<WorldBuildingTemplate> {
        vec![
            // Location Templates
            WorldBuildingTemplate {
                id: "location-city".to_string(),
                name: "City/Town".to_string(),
                description: "A populated settlement with buildings, districts, and inhabitants".to_string(),
                element_type: "location".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "population".to_string(),
                        default_value: None,
                        is_required: false,
                        description: "Number of inhabitants".to_string(),
                        property_type: "number".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "government".to_string(),
                        default_value: Some("Local council or mayor".to_string()),
                        is_required: false,
                        description: "How the settlement is governed".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "notable_districts".to_string(),
                        default_value: Some("Market district, residential area, administrative center".to_string()),
                        is_required: false,
                        description: "Important areas within the settlement".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "economy".to_string(),
                        default_value: Some("Trade, crafts, agriculture".to_string()),
                        is_required: false,
                        description: "Primary economic activities".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "defenses".to_string(),
                        default_value: Some("City walls, guard posts".to_string()),
                        is_required: false,
                        description: "How the settlement protects itself".to_string(),
                        property_type: "text".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            WorldBuildingTemplate {
                id: "location-wilderness".to_string(),
                name: "Wilderness Area".to_string(),
                description: "Natural environments like forests, mountains, or deserts".to_string(),
                element_type: "location".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "terrain_type".to_string(),
                        default_value: Some("Forest".to_string()),
                        is_required: true,
                        description: "Type of natural environment".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "climate".to_string(),
                        default_value: Some("Temperate".to_string()),
                        is_required: false,
                        description: "Weather patterns and seasonal changes".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "wildlife".to_string(),
                        default_value: Some("Various forest creatures".to_string()),
                        is_required: false,
                        description: "Animals and creatures that inhabit the area".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "resources".to_string(),
                        default_value: Some("Timber, herbs, game".to_string()),
                        is_required: false,
                        description: "Natural resources available".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "dangers".to_string(),
                        default_value: Some("Wild animals, getting lost".to_string()),
                        is_required: false,
                        description: "Potential threats or hazards".to_string(),
                        property_type: "list".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Organization Templates
            WorldBuildingTemplate {
                id: "organization-guild".to_string(),
                name: "Guild/Organization".to_string(),
                description: "A formal group with shared interests, goals, or profession".to_string(),
                element_type: "organization".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "purpose".to_string(),
                        default_value: Some("Professional trade organization".to_string()),
                        is_required: true,
                        description: "Primary purpose and goals".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "membership".to_string(),
                        default_value: None,
                        is_required: false,
                        description: "Who can join and membership requirements".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "hierarchy".to_string(),
                        default_value: Some("Apprentice, Journeyman, Master, Guildmaster".to_string()),
                        is_required: false,
                        description: "Organizational structure and ranks".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "influence".to_string(),
                        default_value: Some("Local".to_string()),
                        is_required: false,
                        description: "Scope of power and influence".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "resources".to_string(),
                        default_value: Some("Guild hall, treasury, member skills".to_string()),
                        is_required: false,
                        description: "Assets and capabilities".to_string(),
                        property_type: "list".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Culture Templates
            WorldBuildingTemplate {
                id: "culture-society".to_string(),
                name: "Culture/Society".to_string(),
                description: "A distinct cultural group with shared values, traditions, and customs".to_string(),
                element_type: "culture".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "values".to_string(),
                        default_value: Some("Honor, family, tradition".to_string()),
                        is_required: true,
                        description: "Core cultural values and beliefs".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "traditions".to_string(),
                        default_value: Some("Coming of age ceremonies, seasonal festivals".to_string()),
                        is_required: false,
                        description: "Important cultural practices and rituals".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "social_structure".to_string(),
                        default_value: Some("Hierarchical with clear class distinctions".to_string()),
                        is_required: false,
                        description: "How society is organized".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "language".to_string(),
                        default_value: None,
                        is_required: false,
                        description: "Primary language and dialects".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "arts".to_string(),
                        default_value: Some("Music, storytelling, crafts".to_string()),
                        is_required: false,
                        description: "Cultural expressions and artistic traditions".to_string(),
                        property_type: "list".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Magic System Templates
            WorldBuildingTemplate {
                id: "magic-system".to_string(),
                name: "Magic System".to_string(),
                description: "A structured approach to supernatural abilities and their rules".to_string(),
                element_type: "magic".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "source".to_string(),
                        default_value: Some("Innate ability".to_string()),
                        is_required: true,
                        description: "Where magical power comes from".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "limitations".to_string(),
                        default_value: Some("Requires training, drains energy".to_string()),
                        is_required: true,
                        description: "What restricts or limits magical use".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "schools".to_string(),
                        default_value: Some("Elemental, Healing, Illusion, Divination".to_string()),
                        is_required: false,
                        description: "Different types or schools of magic".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "practitioners".to_string(),
                        default_value: Some("Wizards, sorcerers, clerics".to_string()),
                        is_required: false,
                        description: "Who can use magic and how they learn".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "social_impact".to_string(),
                        default_value: Some("Respected but feared".to_string()),
                        is_required: false,
                        description: "How society views and treats magic users".to_string(),
                        property_type: "text".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Technology Templates
            WorldBuildingTemplate {
                id: "technology-level".to_string(),
                name: "Technology Level".to_string(),
                description: "The general technological advancement of the world or region".to_string(),
                element_type: "technology".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "era".to_string(),
                        default_value: Some("Medieval".to_string()),
                        is_required: true,
                        description: "Technological era or period".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "transportation".to_string(),
                        default_value: Some("Horses, carts, ships".to_string()),
                        is_required: false,
                        description: "Available methods of travel".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "communication".to_string(),
                        default_value: Some("Messengers, letters, signal fires".to_string()),
                        is_required: false,
                        description: "How information is transmitted".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "weapons".to_string(),
                        default_value: Some("Swords, bows, siege engines".to_string()),
                        is_required: false,
                        description: "Military technology and weapons".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "medicine".to_string(),
                        default_value: Some("Herbal remedies, basic surgery".to_string()),
                        is_required: false,
                        description: "Medical knowledge and practices".to_string(),
                        property_type: "text".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Artifact Templates
            WorldBuildingTemplate {
                id: "artifact-magical".to_string(),
                name: "Magical Artifact".to_string(),
                description: "An item imbued with supernatural properties or significance".to_string(),
                element_type: "artifact".to_string(),
                default_properties: vec![
                    WorldBuildingTemplateProperty {
                        property_name: "powers".to_string(),
                        default_value: Some("Enhanced abilities or magical effects".to_string()),
                        is_required: true,
                        description: "What the artifact can do".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "origin".to_string(),
                        default_value: Some("Created by ancient civilization".to_string()),
                        is_required: false,
                        description: "How the artifact was created".to_string(),
                        property_type: "text".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "requirements".to_string(),
                        default_value: Some("Must be attuned to user".to_string()),
                        is_required: false,
                        description: "Conditions for using the artifact".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "drawbacks".to_string(),
                        default_value: Some("Drains user's energy".to_string()),
                        is_required: false,
                        description: "Negative effects or costs of use".to_string(),
                        property_type: "list".to_string(),
                    },
                    WorldBuildingTemplateProperty {
                        property_name: "appearance".to_string(),
                        default_value: None,
                        is_required: false,
                        description: "Physical description of the artifact".to_string(),
                        property_type: "text".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
        ]
    }

    /// Get templates by element type
    pub fn get_templates_by_type(element_type: &str) -> Vec<WorldBuildingTemplate> {
        Self::get_system_templates()
            .into_iter()
            .filter(|template| template.element_type == element_type)
            .collect()
    }

    /// Get template by ID
    pub fn get_template_by_id(template_id: &str) -> Option<WorldBuildingTemplate> {
        Self::get_system_templates()
            .into_iter()
            .find(|template| template.id == template_id)
    }

    /// Apply template to create world element with default properties
    pub async fn apply_template_to_world_element(
        pool: &Pool<Sqlite>,
        template_id: &str,
        project_id: &str,
        name: &str,
        description: Option<String>,
        overrides: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<WorldElement> {
        let template = Self::get_template_by_id(template_id)
            .ok_or_else(|| StoryWeaverError::Internal {
                message: format!("Template not found: {}", template_id),
            })?;

        let overrides = overrides.unwrap_or_default();
        let mut properties = serde_json::Map::new();

        // Apply template properties
        for template_property in template.default_properties {
            let value = overrides
                .get(&template_property.property_name)
                .cloned()
                .or_else(|| {
                    template_property.default_value.map(|v| {
                        match template_property.property_type.as_str() {
                            "list" => json!(v.split(", ").collect::<Vec<&str>>()),
                            "number" => v.parse::<f64>().map(json!).unwrap_or(json!(v)),
                            "boolean" => json!(v.to_lowercase() == "true"),
                            _ => json!(v),
                        }
                    })
                })
                .unwrap_or(json!(null));

            properties.insert(template_property.property_name, value);
        }

        // Create the world element
        let world_element = WorldElement {
            id: Uuid::new_v4().to_string(),
            project_id: Some(project_id.to_string()),
            series_id: None,
            name: name.to_string(),
            description,
            element_type: template.element_type,
            properties: serde_json::to_string(&properties)
                .map_err(|e| StoryWeaverError::Internal {
                    message: format!("Failed to serialize properties: {}", e),
                })?,
            is_visible: true,
            original_project_id: Some(project_id.to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Save to database
        sqlx::query(
            r#"
            INSERT INTO worldbuilding (id, project_id, series_id, name, description, element_type, 
                                     properties, is_visible, original_project_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&world_element.id)
        .bind(&world_element.project_id)
        .bind(&world_element.series_id)
        .bind(&world_element.name)
        .bind(&world_element.description)
        .bind(&world_element.element_type)
        .bind(&world_element.properties)
        .bind(world_element.is_visible)
        .bind(&world_element.original_project_id)
        .bind(world_element.created_at)
        .bind(world_element.updated_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create world element: {}", e)))?;

        Ok(world_element)
    }

    /// Get available element types
    pub fn get_element_types() -> Vec<String> {
        vec![
            "location".to_string(),
            "organization".to_string(),
            "culture".to_string(),
            "magic".to_string(),
            "technology".to_string(),
            "artifact".to_string(),
        ]
    }
}