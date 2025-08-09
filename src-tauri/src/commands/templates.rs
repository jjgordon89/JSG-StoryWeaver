use crate::database::operations::{
    CharacterTemplateOps, WorldBuildingTemplateOps,
    character_template_ops::{CharacterTemplate, CharacterTemplateTrait},
    worldbuilding_template_ops::{WorldBuildingTemplate, WorldBuildingTemplateProperty}
};
use crate::database;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use uuid;

/// Get all character templates
#[tauri::command]
pub async fn get_character_templates() -> Result<Vec<CharacterTemplate>> {
    Ok(CharacterTemplateOps::get_system_templates())
}

/// Get character templates by archetype
#[tauri::command]
pub async fn get_character_templates_by_archetype(archetype: String) -> Result<Vec<CharacterTemplate>> {
    Ok(CharacterTemplateOps::get_templates_by_archetype(&archetype))
}

/// Get available character archetypes
#[tauri::command]
pub async fn get_character_archetypes() -> Result<Vec<String>> {
    Ok(CharacterTemplateOps::get_archetypes())
}

/// Apply character template to create character with default traits
#[tauri::command]
pub async fn apply_character_template(
    template_id: String,
    project_id: String,
    name: String,
    description: Option<String>,
    trait_overrides: Option<HashMap<String, String>>,
) -> Result<String> {
    let pool = database::get_pool()?;
    // First, we need to create a character and get its ID
    // For now, we'll generate a UUID as character_id
    let character_id = uuid::Uuid::new_v4().to_string();
    
    let _traits = CharacterTemplateOps::apply_template_to_character(
        &pool,
        &template_id,
        &character_id,
        trait_overrides,
    ).await?;
    
    Ok(character_id)
}

/// Get all worldbuilding templates
#[tauri::command]
pub async fn get_worldbuilding_templates() -> Result<Vec<WorldBuildingTemplate>> {
    Ok(WorldBuildingTemplateOps::get_system_templates())
}

/// Get worldbuilding templates by element type
#[tauri::command]
pub async fn get_worldbuilding_templates_by_type(element_type: String) -> Result<Vec<WorldBuildingTemplate>> {
    Ok(WorldBuildingTemplateOps::get_templates_by_type(&element_type))
}

/// Get available worldbuilding element types
#[tauri::command]
pub async fn get_worldbuilding_element_types() -> Result<Vec<String>> {
    Ok(WorldBuildingTemplateOps::get_element_types())
}

/// Apply worldbuilding template to create world element with default properties
#[tauri::command]
pub async fn apply_worldbuilding_template(
    template_id: String,
    project_id: String,
    name: String,
    description: Option<String>,
    property_overrides: Option<HashMap<String, serde_json::Value>>,
) -> Result<String> {
    let pool = database::get_pool()?;
    let world_element = WorldBuildingTemplateOps::apply_template_to_world_element(
        &pool,
        &template_id,
        &project_id,
        &name,
        description,
        property_overrides,
    ).await?;
    Ok(world_element.id)
}