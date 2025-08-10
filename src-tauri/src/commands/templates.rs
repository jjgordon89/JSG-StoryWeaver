use crate::commands::CommandResponse;
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
pub async fn get_character_templates() -> CommandResponse<Vec<CharacterTemplate>> {
    async fn get() -> Result<Vec<CharacterTemplate>> {
        Ok(CharacterTemplateOps::get_system_templates())
    }
    
    get().await.into()
}

/// Get character templates by archetype
#[tauri::command]
pub async fn get_character_templates_by_archetype(archetype: String) -> CommandResponse<Vec<CharacterTemplate>> {
    async fn get(archetype: String) -> Result<Vec<CharacterTemplate>> {
        Ok(CharacterTemplateOps::get_templates_by_archetype(&archetype))
    }
    
    get(archetype).await.into()
}

/// Get available character archetypes
#[tauri::command]
pub async fn get_character_archetypes() -> CommandResponse<Vec<String>> {
    async fn get() -> Result<Vec<String>> {
        Ok(CharacterTemplateOps::get_archetypes())
    }
    
    get().await.into()
}

/// Apply character template to create character with default traits
#[tauri::command]
pub async fn apply_character_template(
    template_id: String,
    project_id: String,
    name: String,
    description: Option<String>,
    trait_overrides: Option<HashMap<String, String>>,
) -> CommandResponse<String> {
    async fn apply(
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
            &*pool,
            &template_id,
            &character_id,
            trait_overrides,
        ).await?;
        
        Ok(character_id)
    }
    
    apply(template_id, project_id, name, description, trait_overrides).await.into()
}

/// Get all worldbuilding templates
#[tauri::command]
pub async fn get_worldbuilding_templates() -> CommandResponse<Vec<WorldBuildingTemplate>> {
    async fn get() -> Result<Vec<WorldBuildingTemplate>> {
        Ok(WorldBuildingTemplateOps::get_system_templates())
    }
    
    get().await.into()
}

/// Get worldbuilding templates by type
#[tauri::command]
pub async fn get_worldbuilding_templates_by_type(element_type: String) -> CommandResponse<Vec<WorldBuildingTemplate>> {
    async fn get(element_type: String) -> Result<Vec<WorldBuildingTemplate>> {
        Ok(WorldBuildingTemplateOps::get_templates_by_type(&element_type))
    }
    
    get(element_type).await.into()
}

/// Get available worldbuilding element types
#[tauri::command]
pub async fn get_worldbuilding_element_types() -> CommandResponse<Vec<String>> {
    async fn get() -> Result<Vec<String>> {
        Ok(WorldBuildingTemplateOps::get_element_types())
    }
    
    get().await.into()
}

/// Apply worldbuilding template to create world element with default properties
#[tauri::command]
pub async fn apply_worldbuilding_template(
    template_id: String,
    project_id: String,
    name: String,
    description: Option<String>,
    property_overrides: Option<HashMap<String, serde_json::Value>>,
) -> CommandResponse<String> {
    async fn apply(
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
    
    apply(template_id, project_id, name, description, property_overrides).await.into()
}