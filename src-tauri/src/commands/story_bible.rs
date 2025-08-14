//! Story Bible command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::*};
use crate::error::{Result, StoryWeaverError};
use crate::security::validation::*;
use crate::security::rate_limit::{validate_request_body_size, rl_create, rl_update, rl_delete, rl_list, rl_search};
use serde::Deserialize;
use std::collections::HashMap;

// ===== STORY BIBLE COMMANDS =====

/// Create or update story bible request
#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateStoryBibleRequest {
    pub project_id: String,
    pub braindump: Option<String>,
    pub synopsis: Option<String>,
    pub genre: Option<String>,
    pub style: Option<String>,
    pub style_examples: Option<String>,
    pub pov_mode: Option<String>,
    pub global_pov: Option<String>,
    pub global_tense: Option<String>,
    pub global_character_pov_ids: Option<Vec<String>>,
}

/// Create or update story bible
#[tauri::command]
pub async fn create_or_update_story_bible(request: CreateOrUpdateStoryBibleRequest) -> CommandResponse<StoryBible> {
    async fn create_or_update(request: CreateOrUpdateStoryBibleRequest) -> Result<StoryBible> {
        // Rate limiting
        rl_update("story_bible", Some(&request.project_id))?;
        // Input validation
        validate_security_input(&request.project_id)?;
        
        if let Some(ref braindump) = request.braindump {
            validate_request_body_size(braindump, 50_000)?;
            validate_content_length(braindump, 50000)?;
            validate_security_input(braindump)?;
        }
        
        if let Some(ref synopsis) = request.synopsis {
            validate_request_body_size(synopsis, 10_000)?;
            validate_content_length(synopsis, 10000)?;
            validate_security_input(synopsis)?;
        }
        
        if let Some(ref genre) = request.genre {
            validate_request_body_size(genre, 500)?;
            validate_content_length(genre, 500)?;
            validate_security_input(genre)?;
        }
        
        if let Some(ref style) = request.style {
            validate_request_body_size(style, 5_000)?;
            validate_content_length(style, 5000)?;
            validate_security_input(style)?;
        }
        
        if let Some(ref style_examples) = request.style_examples {
            validate_request_body_size(style_examples, 20_000)?;
            validate_content_length(style_examples, 20000)?;
            validate_security_input(style_examples)?;
        }
        
        if let Some(ref pov_mode) = request.pov_mode {
            validate_request_body_size(pov_mode, 100)?;
            validate_content_length(pov_mode, 100)?;
            validate_security_input(pov_mode)?;
        }
        
        if let Some(ref global_pov) = request.global_pov {
            validate_request_body_size(global_pov, 100)?;
            validate_content_length(global_pov, 100)?;
            validate_security_input(global_pov)?;
        }
        
        if let Some(ref global_tense) = request.global_tense {
            validate_request_body_size(global_tense, 100)?;
            validate_content_length(global_tense, 100)?;
            validate_security_input(global_tense)?;
        }
        
        if let Some(ref character_pov_ids) = request.global_character_pov_ids {
            for id in character_pov_ids {
                validate_security_input(id)?;
            }
        }
        
        let pool = get_pool()?;
        
        let story_bible = StoryBible {
            id: String::new(), // Will be set by the operation
            project_id: request.project_id,
            braindump: request.braindump,
            synopsis: request.synopsis,
            genre: request.genre,
            style: request.style,
            style_examples: request.style_examples,
            pov_mode: request.pov_mode.unwrap_or_else(|| "global".to_string()),
            global_pov: request.global_pov,
            global_tense: request.global_tense,
            global_character_pov_ids: serde_json::to_string(&request.global_character_pov_ids.unwrap_or_default()).unwrap_or_else(|_| "[]".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        StoryBibleOps::create_or_update(&pool, story_bible).await
    }
    
    create_or_update(request).await.into()
}

/// Get story bible by project ID
#[tauri::command]
pub async fn get_story_bible(project_id: String) -> CommandResponse<Option<StoryBible>> {
    async fn get(project_id: String) -> Result<Option<StoryBible>> {
        // Rate limiting
        rl_list("story_bible", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id.to_string())?;
        
        let pool = get_pool()?;
        match StoryBibleOps::get_by_project(&pool, &project_id).await {
            Ok(story_bible) => Ok(Some(story_bible)),
            Err(_) => Ok(None), // Return None if not found instead of error
        }
    }
    
    get(project_id).await.into()
}

// ===== CHARACTER TRAIT COMMANDS =====

/// Visibility enum for character traits
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TraitVisibility {
    Always,
    Chapter,
    Never,
    Public,
    Private,
}

impl TraitVisibility {
    /// Convert visibility enum to boolean for database storage
    pub fn to_boolean(&self) -> bool {
        match self {
            TraitVisibility::Always | TraitVisibility::Chapter | TraitVisibility::Public => true,
            TraitVisibility::Never | TraitVisibility::Private => false,
        }
    }
}

/// Create character trait request
#[derive(Debug, Deserialize)]
pub struct CreateCharacterTraitRequest {
    pub character_id: String,
    pub trait_name: String,
    pub trait_value: String,
    pub visibility: Option<TraitVisibility>,
}

/// Update character trait request
#[derive(Debug, Deserialize)]
pub struct UpdateCharacterTraitRequest {
    pub id: String,
    pub trait_name: Option<String>,
    pub trait_value: Option<String>,
    pub visibility: Option<TraitVisibility>,
}

/// Create a new character trait
#[tauri::command]
pub async fn create_character_trait(request: CreateCharacterTraitRequest) -> CommandResponse<CharacterTrait> {
    async fn create(request: CreateCharacterTraitRequest) -> Result<CharacterTrait> {
        // Rate limiting
        rl_create("character_trait", Some(&request.character_id))?;
        // Input validation
        validate_security_input(&request.character_id)?;
        validate_safe_name(&request.trait_name, "Name")?;
        validate_request_body_size(&request.trait_value, 5_000)?;
        validate_content_length(&request.trait_value, 5000)?;
        validate_security_input(&request.trait_value)?;
        
        let pool = get_pool()?;
        
        let is_visible = request.visibility
            .map(|v| v.to_boolean())
            .unwrap_or(true);
        
        let trait_data = CharacterTrait {
            id: String::new(), // Will be set by the operation
            character_id: request.character_id,
            trait_name: request.trait_name,
            trait_value: Some(request.trait_value),
            is_visible,
            created_at: chrono::Utc::now(),
        };
        
        CharacterTraitOps::create(&pool, trait_data).await
    }
    
    create(request).await.into()
}

/// Get character traits by character ID
#[tauri::command]
pub async fn get_character_traits(character_id: String) -> CommandResponse<Vec<CharacterTrait>> {
    async fn get_by_character(character_id: String) -> Result<Vec<CharacterTrait>> {
        // Rate limiting
        rl_list("character_trait", Some(&character_id))?;
        // Input validation
        validate_security_input(&character_id)?;
        
        let pool = get_pool()?;
        CharacterTraitOps::get_by_character(&pool, &character_id).await
    }
    
    get_by_character(character_id).await.into()
}

/// Update character trait
#[tauri::command]
pub async fn update_character_trait(request: UpdateCharacterTraitRequest) -> CommandResponse<()> {
    async fn update(request: UpdateCharacterTraitRequest) -> Result<()> {
        // Rate limiting
        rl_update("character_trait", Some(&request.id))?;
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref trait_name) = request.trait_name {
            validate_safe_name(trait_name, "Name")?;
        }
        
        if let Some(ref trait_value) = request.trait_value {
            validate_request_body_size(trait_value, 5_000)?;
            validate_content_length(trait_value, 5000)?;
            validate_security_input(trait_value)?;
        }
        
        let pool = get_pool()?;
        
        // Get the existing trait
        let mut character_trait = CharacterTraitOps::get_by_id(&pool, &request.id).await?;
        
        // Update fields if provided
        if let Some(trait_name) = request.trait_name {
            character_trait.trait_name = trait_name;
        }
        if let Some(trait_value) = request.trait_value {
            character_trait.trait_value = Some(trait_value);
        }
        if let Some(visibility) = request.visibility {
            character_trait.is_visible = visibility.to_boolean();
        }
        
        CharacterTraitOps::update(&pool, character_trait).await?;
        Ok(())
    }
    
    update(request).await.into()
}

/// Delete character trait
#[tauri::command]
pub async fn delete_character_trait(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("character_trait", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        CharacterTraitOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

// ===== WORLD ELEMENT COMMANDS =====

/// Create world element request
#[derive(Debug, Deserialize)]
pub struct CreateWorldElementRequest {
    pub project_id: String,
    pub series_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub element_type: String,
    pub properties: Option<HashMap<String, String>>,
    pub is_visible: Option<bool>,
}

/// Update world element request
#[derive(Debug, Deserialize)]
pub struct UpdateWorldElementRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub element_type: Option<String>,
    pub properties: Option<HashMap<String, String>>,
    pub is_visible: Option<bool>,
}

/// Create a new world element
#[tauri::command]
pub async fn create_world_element(request: CreateWorldElementRequest) -> CommandResponse<WorldElement> {
    async fn create(request: CreateWorldElementRequest) -> Result<WorldElement> {
        // Rate limiting
        rl_create("world_element", Some(&request.project_id))?;
        // Input validation
        validate_security_input(&request.project_id)?;
        validate_safe_name(&request.name, "Name")?;
        validate_safe_name(&request.element_type, "Name")?;
        
        if let Some(ref series_id) = request.series_id {
            validate_security_input(series_id)?;
        }
        
        if let Some(ref description) = request.description {
            validate_request_body_size(description, 10_000)?;
            validate_content_length(description, 10000)?;
            validate_security_input(description)?;
        }
        
        if let Some(ref properties) = request.properties {
            for (key, value) in properties {
                validate_safe_name(key, "Name")?;
                validate_request_body_size(value, 5_000)?;
                validate_content_length(value, 5000)?;
                validate_security_input(value)?;
            }
        }
        
        let pool = get_pool()?;
        
        let element = WorldElement {
            id: String::new(), // Will be set by the operation
            project_id: Some(request.project_id.clone()),
            series_id: request.series_id,
            name: request.name,
            description: request.description,
            element_type: request.element_type,
            properties: serde_json::to_string(&request.properties.unwrap_or_default()).unwrap_or_else(|_| "{}".to_string()),
            is_visible: request.is_visible.unwrap_or(true),
            original_project_id: Some(request.project_id),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        WorldElementOps::create(&pool, element).await
    }
    
    create(request).await.into()
}

/// Get world elements by project ID
#[tauri::command]
pub async fn get_world_elements(project_id: String) -> CommandResponse<Vec<WorldElement>> {
    async fn get_by_project(project_id: String) -> Result<Vec<WorldElement>> {
        // Rate limiting
        rl_list("world_element", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        WorldElementOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get world element by ID
#[tauri::command]
pub async fn get_world_element(id: String) -> CommandResponse<Option<WorldElement>> {
    async fn get(id: String) -> Result<Option<WorldElement>> {
        // Rate limiting
        rl_list("world_element", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        WorldElementOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Update world element
#[tauri::command]
pub async fn update_world_element(request: UpdateWorldElementRequest) -> CommandResponse<()> {
    async fn update(request: UpdateWorldElementRequest) -> Result<()> {
        // Rate limiting
        rl_update("world_element", Some(&request.id))?;
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref name) = request.name {
            validate_safe_name(name, "Name")?;
        }
        
        if let Some(ref description) = request.description {
            validate_request_body_size(description, 10_000)?;
            validate_content_length(description, 10000)?;
            validate_security_input(description)?;
        }
        
        if let Some(ref element_type) = request.element_type {
            validate_safe_name(element_type, "Name")?;
        }
        
        if let Some(ref properties) = request.properties {
            for (key, value) in properties {
                validate_safe_name(key, "Name")?;
                validate_request_body_size(value, 5_000)?;
                validate_content_length(value, 5000)?;
                validate_security_input(value)?;
            }
        }
        
        let pool = get_pool()?;
        
        // Get the existing world element
        let mut element = WorldElementOps::get_by_id(&pool, &request.id).await?
            .ok_or_else(|| StoryWeaverError::Internal { 
                message: format!("WorldElement with id {} not found", request.id)
            })?;
        
        // Update fields
        if let Some(name) = request.name {
            element.name = name;
        }
        if let Some(description) = request.description {
            element.description = Some(description);
        }
        if let Some(element_type) = request.element_type {
            element.element_type = element_type;
        }
        if let Some(properties) = request.properties {
            element.properties = serde_json::to_string(&properties).unwrap_or_else(|_| "{}".to_string());
        }
        if let Some(is_visible) = request.is_visible {
            element.is_visible = is_visible;
        }
        
        WorldElementOps::update(&pool, element).await?;
        Ok(())
    }
    
    update(request).await.into()
}

/// Delete world element
#[tauri::command]
pub async fn delete_world_element(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("world_element", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        WorldElementOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Search world elements
#[tauri::command]
pub async fn search_world_elements(project_id: String, query: String) -> CommandResponse<Vec<WorldElement>> {
    async fn search(project_id: String, query: String) -> Result<Vec<WorldElement>> {
        // Rate limiting
        rl_search("world_element", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id)?;
        validate_request_body_size(&query, 4_000)?;
        validate_content_length(&query, 1000)?;
        validate_security_input(&query)?;
        
        if query.trim().is_empty() {
            return Err(StoryWeaverError::validation("Search query cannot be empty"));
        }
        
        let pool = get_pool()?;
        WorldElementOps::search(&pool, &project_id, &query).await
    }
    
    search(project_id, query).await.into()
}

// ===== OUTLINE COMMANDS =====

/// Create outline request
#[derive(Debug, Deserialize)]
pub struct CreateOutlineRequest {
    pub project_id: String,
    pub chapter_number: i32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub pov: Option<String>,
    pub tense: Option<String>,
    pub character_pov_ids: Option<Vec<String>>,
}

/// Update outline request
#[derive(Debug, Deserialize)]
pub struct UpdateOutlineRequest {
    pub id: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub pov: Option<String>,
    pub tense: Option<String>,
    pub character_pov_ids: Option<Vec<String>>,
}

/// Create a new outline
#[tauri::command]
pub async fn create_outline(request: CreateOutlineRequest) -> CommandResponse<Outline> {
    async fn create(request: CreateOutlineRequest) -> Result<Outline> {
        // Rate limiting
        rl_create("outline", Some(&request.project_id))?;
        // Input validation
        validate_security_input(&request.project_id)?;
        
        if request.chapter_number < 1 || request.chapter_number > 10000 {
            return Err(StoryWeaverError::validation("Chapter number must be between 1 and 10000"));
        }
        
        if let Some(ref title) = request.title {
            validate_request_body_size(title, 500)?;
            validate_content_length(title, 500)?;
            validate_security_input(title)?;
        }
        
        if let Some(ref summary) = request.summary {
            validate_request_body_size(summary, 10_000)?;
            validate_content_length(summary, 10000)?;
            validate_security_input(summary)?;
        }
        
        if let Some(ref pov) = request.pov {
            validate_request_body_size(pov, 100)?;
            validate_content_length(pov, 100)?;
            validate_security_input(pov)?;
        }
        
        if let Some(ref tense) = request.tense {
            validate_request_body_size(tense, 100)?;
            validate_content_length(tense, 100)?;
            validate_security_input(tense)?;
        }
        
        if let Some(ref character_pov_ids) = request.character_pov_ids {
            for id in character_pov_ids {
                validate_security_input(id)?;
            }
        }
        
        let pool = get_pool()?;
        
        let outline = Outline {
            id: String::new(), // Will be set by the operation
            project_id: request.project_id,
            chapter_number: Some(request.chapter_number),
            title: request.title,
            summary: request.summary,
            pov: request.pov,
            tense: request.tense,
            character_pov_ids: serde_json::to_string(&request.character_pov_ids.unwrap_or_default())?,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        OutlineOps::create(&pool, outline).await
    }
    
    create(request).await.into()
}

/// Get outlines by project ID
#[tauri::command]
pub async fn get_outlines(project_id: String) -> CommandResponse<Vec<Outline>> {
    async fn get_by_project(project_id: String) -> Result<Vec<Outline>> {
        // Rate limiting
        rl_list("outline", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        OutlineOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get outline by ID
#[tauri::command]
pub async fn get_outline(id: String) -> CommandResponse<Outline> {
    async fn get(id: String) -> Result<Outline> {
        // Rate limiting
        rl_list("outline", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        OutlineOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Get outline by chapter number
#[tauri::command]
pub async fn get_outline_by_chapter(project_id: String, chapter_number: i32) -> CommandResponse<Option<Outline>> {
    async fn get_by_chapter(project_id: String, chapter_number: i32) -> Result<Option<Outline>> {
        // Rate limiting
        rl_list("outline", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id)?;
        
        if chapter_number < 1 || chapter_number > 10000 {
            return Err(StoryWeaverError::validation("Chapter number must be between 1 and 10000"));
        }
        
        let pool = get_pool()?;
        OutlineOps::get_by_chapter(&pool, &project_id, chapter_number).await
    }
    
    get_by_chapter(project_id, chapter_number).await.into()
}

/// Update outline
#[tauri::command]
pub async fn update_outline(request: UpdateOutlineRequest) -> CommandResponse<()> {
    async fn update(request: UpdateOutlineRequest) -> Result<()> {
        // Rate limiting
        rl_update("outline", Some(&request.id))?;
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref title) = request.title {
            validate_content_length(title, 500)?;
            validate_security_input(title)?;
        }
        
        if let Some(ref summary) = request.summary {
            validate_content_length(summary, 10000)?;
            validate_security_input(summary)?;
        }
        
        if let Some(ref pov) = request.pov {
            validate_content_length(pov, 100)?;
            validate_security_input(pov)?;
        }
        
        if let Some(ref tense) = request.tense {
            validate_content_length(tense, 100)?;
            validate_security_input(tense)?;
        }
        
        if let Some(ref character_pov_ids) = request.character_pov_ids {
            for id in character_pov_ids {
                validate_security_input(id)?;
            }
        }
        
        let pool = get_pool()?;
        
        // Get the existing outline
        let mut outline = OutlineOps::get_by_id(&pool, &request.id).await?;
        
        // Update fields from request
        if let Some(title) = request.title {
            outline.title = Some(title);
        }
        if let Some(summary) = request.summary {
            outline.summary = Some(summary);
        }
        if let Some(pov) = request.pov {
            outline.pov = Some(pov);
        }
        if let Some(tense) = request.tense {
            outline.tense = Some(tense);
        }
        if let Some(character_pov_ids) = request.character_pov_ids {
            outline.character_pov_ids = serde_json::to_string(&character_pov_ids)?;
        }
        
        let _ = OutlineOps::update(&pool, outline).await?;
        Ok(())
    }
    
    update(request).await.into()
}

/// Delete outline
#[tauri::command]
pub async fn delete_outline(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("outline", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        OutlineOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Search outlines
#[tauri::command]
pub async fn search_outlines(project_id: String, query: String) -> CommandResponse<Vec<Outline>> {
    async fn search(project_id: String, query: String) -> Result<Vec<Outline>> {
        // Rate limiting
        rl_search("outline", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id)?;
        validate_request_body_size(&query, 4_000)?;
        validate_content_length(&query, 1000)?;
        validate_security_input(&query)?;
        
        if query.trim().is_empty() {
            return Err(StoryWeaverError::validation("Search query cannot be empty"));
        }
        
        let pool = get_pool()?;
        OutlineOps::search(&pool, &project_id, &query).await
    }
    
    search(project_id, query).await.into()
}

// ===== SCENE COMMANDS =====

/// Create scene request
#[derive(Debug, Deserialize)]
pub struct CreateSceneRequest {
    pub outline_id: String,
    pub scene_number: i32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub extra_instructions: Option<String>,
    pub pov: Option<String>,
    pub tense: Option<String>,
    pub character_pov_ids: Option<Vec<String>>,
    pub word_count_estimate: Option<i32>,
    pub credit_estimate: Option<f64>,
}

/// Update scene request
#[derive(Debug, Deserialize)]
pub struct UpdateSceneRequest {
    pub id: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub extra_instructions: Option<String>,
    pub pov: Option<String>,
    pub tense: Option<String>,
    pub character_pov_ids: Option<Vec<String>>,
    pub word_count_estimate: Option<i32>,
    pub credit_estimate: Option<f64>,
}

/// Create a new scene
#[tauri::command]
pub async fn create_scene(request: CreateSceneRequest) -> CommandResponse<Scene> {
    async fn create(request: CreateSceneRequest) -> Result<Scene> {
        // Rate limiting
        rl_create("scene", Some(&request.outline_id))?;
        // Input validation
        validate_security_input(&request.outline_id)?;
        
        if request.scene_number < 1 || request.scene_number > 10000 {
            return Err(StoryWeaverError::validation("Scene number must be between 1 and 10000"));
        }
        
        if let Some(ref title) = request.title {
            validate_request_body_size(title, 500)?;
            validate_content_length(title, 500)?;
            validate_security_input(title)?;
        }
        
        if let Some(ref summary) = request.summary {
            validate_request_body_size(summary, 10_000)?;
            validate_content_length(summary, 10000)?;
            validate_security_input(summary)?;
        }
        
        if let Some(ref extra_instructions) = request.extra_instructions {
            validate_request_body_size(extra_instructions, 5_000)?;
            validate_content_length(extra_instructions, 5000)?;
            validate_security_input(extra_instructions)?;
        }
        
        if let Some(ref pov) = request.pov {
            validate_request_body_size(pov, 100)?;
            validate_content_length(pov, 100)?;
            validate_security_input(pov)?;
        }
        
        if let Some(ref tense) = request.tense {
            validate_request_body_size(tense, 100)?;
            validate_content_length(tense, 100)?;
            validate_security_input(tense)?;
        }
        
        if let Some(ref character_pov_ids) = request.character_pov_ids {
            for id in character_pov_ids {
                validate_security_input(id)?;
            }
        }
        
        if let Some(word_count) = request.word_count_estimate {
            if word_count < 0 || word_count > 1000000 {
                return Err(StoryWeaverError::validation("Word count estimate must be between 0 and 1,000,000"));
            }
        }
        
        if let Some(credit_estimate) = request.credit_estimate {
            if credit_estimate < 0.0 || credit_estimate > 1000000.0 {
                return Err(StoryWeaverError::validation("Credit estimate must be between 0 and 1,000,000"));
            }
        }
        
        let pool = get_pool()?;
        
        let scene = Scene {
            id: String::new(), // Will be set by the operation
            outline_id: request.outline_id,
            scene_number: request.scene_number,
            title: request.title,
            summary: request.summary,
            extra_instructions: request.extra_instructions,
            pov: request.pov,
            tense: request.tense,
            character_pov_ids: serde_json::to_string(&request.character_pov_ids.unwrap_or_default())?,
            word_count_estimate: request.word_count_estimate,
            credit_estimate: request.credit_estimate,
            is_validated: false,
            validation_issues: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        SceneOps::create(&pool, scene).await
    }
    
    create(request).await.into()
}

/// Get scenes by outline ID
#[tauri::command]
pub async fn get_scenes(outline_id: String) -> CommandResponse<Vec<Scene>> {
    async fn get_by_outline(outline_id: String) -> Result<Vec<Scene>> {
        // Rate limiting
        rl_list("scene", Some(&outline_id))?;
        // Input validation
        validate_security_input(&outline_id)?;
        
        let pool = get_pool()?;
        SceneOps::get_by_outline(&pool, &outline_id).await
    }
    
    get_by_outline(outline_id).await.into()
}

/// Get scene by ID
#[tauri::command]
pub async fn get_scene(id: String) -> CommandResponse<Scene> {
    async fn get(id: String) -> Result<Scene> {
        // Rate limiting
        rl_list("scene", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        SceneOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Update scene
#[tauri::command]
pub async fn update_scene(request: UpdateSceneRequest) -> CommandResponse<Scene> {
    async fn update(request: UpdateSceneRequest) -> Result<Scene> {
        // Rate limiting
        rl_update("scene", Some(&request.id))?;
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref title) = request.title {
            validate_content_length(title, 500)?;
            validate_security_input(title)?;
        }
        
        if let Some(ref summary) = request.summary {
            validate_content_length(summary, 10000)?;
            validate_security_input(summary)?;
        }
        
        if let Some(ref extra_instructions) = request.extra_instructions {
            validate_content_length(extra_instructions, 5000)?;
            validate_security_input(extra_instructions)?;
        }
        
        if let Some(ref pov) = request.pov {
            validate_content_length(pov, 100)?;
            validate_security_input(pov)?;
        }
        
        if let Some(ref tense) = request.tense {
            validate_content_length(tense, 100)?;
            validate_security_input(tense)?;
        }
        
        if let Some(ref character_pov_ids) = request.character_pov_ids {
            for id in character_pov_ids {
                validate_security_input(id)?;
            }
        }
        
        if let Some(word_count) = request.word_count_estimate {
            if word_count < 0 || word_count > 1000000 {
                return Err(StoryWeaverError::validation("Word count estimate must be between 0 and 1,000,000"));
            }
        }
        
        if let Some(credit_estimate) = request.credit_estimate {
            if credit_estimate < 0.0 || credit_estimate > 1000000.0 {
                return Err(StoryWeaverError::validation("Credit estimate must be between 0 and 1,000,000"));
            }
        }
        
        let pool = get_pool()?;
        
        // Get the existing scene first
        let mut scene = SceneOps::get_by_id(&pool, &request.id).await?;
        
        // Update the fields from the request
        scene.title = request.title;
        scene.summary = request.summary;
        scene.extra_instructions = request.extra_instructions;
        scene.pov = request.pov;
        scene.tense = request.tense;
        scene.character_pov_ids = serde_json::to_string(&request.character_pov_ids.unwrap_or_default())?;
        scene.word_count_estimate = request.word_count_estimate;
        scene.credit_estimate = request.credit_estimate;
        
        SceneOps::update(&pool, scene).await
    }
    
    update(request).await.into()
}

/// Delete scene
#[tauri::command]
pub async fn delete_scene(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("scene", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        SceneOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Validate scene
#[tauri::command]
pub async fn validate_scene(id: String) -> CommandResponse<()> {
    async fn validate(id: String) -> Result<()> {
        // Rate limiting
        rl_update("scene", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        SceneOps::get_validated(&pool, &id).await?;
        Ok(())
    }
    
    validate(id).await.into()
}

/// Search scenes
#[tauri::command]
pub async fn search_scenes(outline_id: String, query: String) -> CommandResponse<Vec<Scene>> {
    async fn search(outline_id: String, query: String) -> Result<Vec<Scene>> {
        // Rate limiting
        rl_search("scene", Some(&outline_id))?;
        // Input validation
        validate_security_input(&outline_id)?;
        validate_request_body_size(&query, 4_000)?;
        validate_content_length(&query, 1000)?;
        validate_security_input(&query)?;
        
        if query.trim().is_empty() {
            return Err(StoryWeaverError::validation("Search query cannot be empty"));
        }
        
        let pool = get_pool()?;
        SceneOps::search(&pool, &outline_id, &query).await
    }
    
    search(outline_id, query).await.into()
}

// ===== OUTLINE-TO-DOCUMENT LINKING COMMANDS =====

/// Link outline to document request
#[derive(Debug, Deserialize)]
pub struct LinkOutlineToDocumentRequest {
    pub outline_id: String,
    pub document_id: String,
    pub link_type: String, // "chapter", "scene", "reference"
}

/// Unlink outline from document request
#[derive(Debug, Deserialize)]
pub struct UnlinkOutlineFromDocumentRequest {
    pub outline_id: String,
    pub document_id: String,
}

/// Link an outline to a document
#[tauri::command]
pub async fn link_outline_to_document(request: LinkOutlineToDocumentRequest) -> CommandResponse<()> {
    async fn link(request: LinkOutlineToDocumentRequest) -> Result<()> {
        // Rate limiting
        rl_create("outline_document_link", Some(&request.outline_id))?;
        // Input validation
        validate_security_input(&request.outline_id)?;
        validate_security_input(&request.document_id)?;
        validate_safe_name(&request.link_type, "Link type")?;
        
        let pool = get_pool()?;
        
        // Check if outline exists
        let _outline = OutlineOps::get_by_id(&pool, &request.outline_id).await?;
        
        // Check if document exists
        let _document = DocumentOps::get_by_id(&pool, &request.document_id).await?;
        
        // Create the link using document_links table
        let link = DocumentLink {
            id: String::new(), // Will be set by the operation
            from_document_id: request.outline_id, // Using outline_id as from_document_id
            to_document_id: request.document_id,
            link_order: 1, // Default order
            created_at: chrono::Utc::now(),
        };
        
        DocumentLinkOps::create(&pool, link).await?;
        Ok(())
    }
    
    link(request).await.into()
}

/// Unlink an outline from a document
#[tauri::command]
pub async fn unlink_outline_from_document(request: UnlinkOutlineFromDocumentRequest) -> CommandResponse<()> {
    async fn unlink(request: UnlinkOutlineFromDocumentRequest) -> Result<()> {
        // Rate limiting
        rl_delete("outline_document_link", Some(&request.outline_id))?;
        // Input validation
        validate_security_input(&request.outline_id)?;
        validate_security_input(&request.document_id)?;
        
        let pool = get_pool()?;
        
        // Find and delete the link
        let links = DocumentLinkOps::get_outgoing_links(&pool, &request.outline_id).await?;
        for link in links {
            if link.to_document_id == request.document_id {
                DocumentLinkOps::delete(&pool, &link.id).await?;
                break;
            }
        }
        
        Ok(())
    }
    
    unlink(request).await.into()
}

/// Get documents linked to an outline
#[tauri::command]
pub async fn get_outline_linked_documents(outline_id: String) -> CommandResponse<Vec<Document>> {
    async fn get_linked(outline_id: String) -> Result<Vec<Document>> {
        // Rate limiting
        rl_list("outline_document_link", Some(&outline_id))?;
        // Input validation
        validate_security_input(&outline_id)?;
        
        let pool = get_pool()?;
        
        // Get outgoing links from the outline
        let links = DocumentLinkOps::get_outgoing_links(&pool, &outline_id).await?;
        let mut documents = Vec::new();
        
        for link in links {
            if let Ok(Some(document)) = DocumentOps::get_by_id(&pool, &link.to_document_id).await {
                documents.push(document);
            }
        }
        
        Ok(documents)
    }
    
    get_linked(outline_id).await.into()
}

/// Get outlines linked to a document
#[tauri::command]
pub async fn get_document_linked_outlines(document_id: String) -> CommandResponse<Vec<Outline>> {
    async fn get_linked(document_id: String) -> Result<Vec<Outline>> {
        // Rate limiting
        rl_list("outline_document_link", Some(&document_id))?;
        // Input validation
        validate_security_input(&document_id)?;
        
        let pool = get_pool()?;
        
        // Get incoming links to the document
        let links = DocumentLinkOps::get_incoming_links(&pool, &document_id).await?;
        let mut outlines = Vec::new();
        
        for link in links {
            if let Ok(outline) = OutlineOps::get_by_id(&pool, &link.from_document_id).await {
                outlines.push(outline);
            }
        }
        
        Ok(outlines)
    }
    
    get_linked(document_id).await.into()
}

// ===== SERIES-LEVEL SHARING COMMANDS =====

/// Share world element to series
#[tauri::command]
pub async fn share_world_element_to_series(element_id: String, series_id: String) -> CommandResponse<()> {
    async fn share(element_id: String, series_id: String) -> Result<()> {
        // Rate limiting
        rl_update("world_element", Some(&element_id))?;
        // Input validation
        validate_security_input(&element_id)?;
        validate_security_input(&series_id)?;
        
        let pool = get_pool()?;
        WorldElementOps::share_to_series(&pool, &element_id, &series_id).await
    }
    
    share(element_id, series_id).await.into()
}

/// Unshare world element from series
#[tauri::command]
pub async fn unshare_world_element_from_series(element_id: String) -> CommandResponse<()> {
    async fn unshare(element_id: String) -> Result<()> {
        // Rate limiting
        rl_update("world_element", Some(&element_id))?;
        // Input validation
        validate_security_input(&element_id)?;
        
        let pool = get_pool()?;
        WorldElementOps::unshare_from_series(&pool, &element_id).await
    }
    
    unshare(element_id).await.into()
}

/// Get world elements shared to a series
#[tauri::command]
pub async fn get_series_world_elements(series_id: String) -> CommandResponse<Vec<WorldElement>> {
    async fn get_series_elements(series_id: String) -> Result<Vec<WorldElement>> {
        // Rate limiting
        rl_list("world_element", Some(&series_id))?;
        // Input validation
        validate_security_input(&series_id)?;
        
        let pool = get_pool()?;
        WorldElementOps::get_by_series(&pool, &series_id).await
    }
    
    get_series_elements(series_id).await.into()
}

// ===== STORY BIBLE DETECTION COMMANDS =====

/// Story Bible detection request
#[derive(Debug, Deserialize)]
pub struct DetectStoryBibleRequest {
    pub project_id: String,
    pub text: String,
    pub detection_types: Vec<String>, // ["characters", "locations", "world_elements"]
}

/// Story Bible detection result
#[derive(Debug, serde::Serialize)]
pub struct StoryBibleDetection {
    pub detection_type: String,
    pub entity_id: String,
    pub entity_name: String,
    pub start_position: usize,
    pub end_position: usize,
    pub confidence: f64,
}

/// Detect Story Bible elements in text
#[tauri::command]
pub async fn detect_story_bible_in_text(request: DetectStoryBibleRequest) -> CommandResponse<Vec<StoryBibleDetection>> {
    async fn detect(request: DetectStoryBibleRequest) -> Result<Vec<StoryBibleDetection>> {
        // Rate limiting
        rl_search("story_bible_detection", Some(&request.project_id))?;
        // Input validation
        validate_security_input(&request.project_id)?;
        validate_request_body_size(&request.text, 100_000)?;
        validate_content_length(&request.text, 100000)?;
        validate_security_input(&request.text)?;
        
        for detection_type in &request.detection_types {
            validate_safe_name(detection_type, "Detection type")?;
        }
        
        let pool = get_pool()?;
        let mut detections = Vec::new();
        
        // Detect characters
        if request.detection_types.contains(&"characters".to_string()) {
            let characters = CharacterOps::get_by_project(&pool, &request.project_id).await?;
            for character in characters {
                if let Some(positions) = find_text_occurrences(&request.text, &character.name) {
                    for (start, end) in positions {
                        detections.push(StoryBibleDetection {
                            detection_type: "character".to_string(),
                            entity_id: character.id.clone(),
                            entity_name: character.name.clone(),
                            start_position: start,
                            end_position: end,
                            confidence: 0.9, // High confidence for exact name matches
                        });
                    }
                }
            }
        }
        
        // Detect locations
        if request.detection_types.contains(&"locations".to_string()) {
            let locations = LocationOps::get_by_project(&pool, &request.project_id).await?;
            for location in locations {
                if let Some(positions) = find_text_occurrences(&request.text, &location.name) {
                    for (start, end) in positions {
                        detections.push(StoryBibleDetection {
                            detection_type: "location".to_string(),
                            entity_id: location.id.clone(),
                            entity_name: location.name.clone(),
                            start_position: start,
                            end_position: end,
                            confidence: 0.9, // High confidence for exact name matches
                        });
                    }
                }
            }
        }
        
        // Detect world elements
        if request.detection_types.contains(&"world_elements".to_string()) {
            let world_elements = WorldElementOps::get_by_project(&pool, &request.project_id).await?;
            for element in world_elements {
                if let Some(positions) = find_text_occurrences(&request.text, &element.name) {
                    for (start, end) in positions {
                        detections.push(StoryBibleDetection {
                            detection_type: "world_element".to_string(),
                            entity_id: element.id.clone(),
                            entity_name: element.name.clone(),
                            start_position: start,
                            end_position: end,
                            confidence: 0.9, // High confidence for exact name matches
                        });
                    }
                }
            }
        }
        
        // Sort by position in text
        detections.sort_by_key(|d| d.start_position);
        
        Ok(detections)
    }
    
    detect(request).await.into()
}

/// Helper function to find text occurrences
fn find_text_occurrences(text: &str, search_term: &str) -> Option<Vec<(usize, usize)>> {
    if search_term.is_empty() {
        return None;
    }
    
    let mut positions = Vec::new();
    let text_lower = text.to_lowercase();
    let search_lower = search_term.to_lowercase();
    
    let mut start = 0;
    while let Some(pos) = text_lower[start..].find(&search_lower) {
        let actual_pos = start + pos;
        let end_pos = actual_pos + search_term.len();
        
        // Check if it's a whole word (not part of another word)
        let is_word_boundary_start = actual_pos == 0 || 
            !text.chars().nth(actual_pos - 1).unwrap_or(' ').is_alphanumeric();
        let is_word_boundary_end = end_pos >= text.len() || 
            !text.chars().nth(end_pos).unwrap_or(' ').is_alphanumeric();
        
        if is_word_boundary_start && is_word_boundary_end {
            positions.push((actual_pos, end_pos));
        }
        
        start = actual_pos + 1;
    }
    
    if positions.is_empty() {
        None
    } else {
        Some(positions)
    }
}
