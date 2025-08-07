//! Story Bible command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::*};
use crate::error::Result;
use serde::{Deserialize, Serialize};
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
            global_character_pov_ids: request.global_character_pov_ids.unwrap_or_default(),
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
    async fn get_by_project(project_id: String) -> Result<Option<StoryBible>> {
        let pool = get_pool()?;
        match StoryBibleOps::get_by_project(&pool, &project_id).await {
            Ok(story_bible) => Ok(Some(story_bible)),
            Err(_) => Ok(None), // Return None if not found instead of error
        }
    }
    
    get_by_project(project_id).await.into()
}

// ===== CHARACTER TRAIT COMMANDS =====

/// Create character trait request
#[derive(Debug, Deserialize)]
pub struct CreateCharacterTraitRequest {
    pub character_id: String,
    pub trait_name: String,
    pub trait_value: String,
    pub is_visible: Option<bool>,
}

/// Update character trait request
#[derive(Debug, Deserialize)]
pub struct UpdateCharacterTraitRequest {
    pub id: String,
    pub trait_name: Option<String>,
    pub trait_value: Option<String>,
    pub is_visible: Option<bool>,
}

/// Create a new character trait
#[tauri::command]
pub async fn create_character_trait(request: CreateCharacterTraitRequest) -> CommandResponse<CharacterTrait> {
    async fn create(request: CreateCharacterTraitRequest) -> Result<CharacterTrait> {
        let pool = get_pool()?;
        
        let trait_data = CharacterTrait {
            id: String::new(), // Will be set by the operation
            character_id: request.character_id,
            trait_name: request.trait_name,
            trait_value: request.trait_value,
            is_visible: request.is_visible.unwrap_or(true),
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
        let pool = get_pool()?;
        CharacterTraitOps::get_by_character(&pool, &character_id).await
    }
    
    get_by_character(character_id).await.into()
}

/// Update character trait
#[tauri::command]
pub async fn update_character_trait(request: UpdateCharacterTraitRequest) -> CommandResponse<()> {
    async fn update(request: UpdateCharacterTraitRequest) -> Result<()> {
        let pool = get_pool()?;
        CharacterTraitOps::update(&pool, &request.id, request.trait_name, request.trait_value, request.is_visible).await
    }
    
    update(request).await.into()
}

/// Delete character trait
#[tauri::command]
pub async fn delete_character_trait(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
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
        let pool = get_pool()?;
        
        let element = WorldElement {
            id: String::new(), // Will be set by the operation
            project_id: request.project_id.clone(),
            series_id: request.series_id,
            name: request.name,
            description: request.description,
            element_type: request.element_type,
            properties: request.properties.unwrap_or_default(),
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
        let pool = get_pool()?;
        WorldElementOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get world element by ID
#[tauri::command]
pub async fn get_world_element(id: String) -> CommandResponse<Option<WorldElement>> {
    async fn get(id: String) -> Result<Option<WorldElement>> {
        let pool = get_pool()?;
        WorldElementOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Update world element
#[tauri::command]
pub async fn update_world_element(request: UpdateWorldElementRequest) -> CommandResponse<()> {
    async fn update(request: UpdateWorldElementRequest) -> Result<()> {
        let pool = get_pool()?;
        WorldElementOps::update(&pool, &request.id, request.name, request.description, request.element_type, request.properties, request.is_visible).await
    }
    
    update(request).await.into()
}

/// Delete world element
#[tauri::command]
pub async fn delete_world_element(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        WorldElementOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Search world elements
#[tauri::command]
pub async fn search_world_elements(project_id: String, query: String) -> CommandResponse<Vec<WorldElement>> {
    async fn search(project_id: String, query: String) -> Result<Vec<WorldElement>> {
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
        let pool = get_pool()?;
        
        let outline = Outline {
            id: String::new(), // Will be set by the operation
            project_id: request.project_id,
            chapter_number: request.chapter_number,
            title: request.title,
            summary: request.summary,
            pov: request.pov,
            tense: request.tense,
            character_pov_ids: request.character_pov_ids.unwrap_or_default(),
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
        let pool = get_pool()?;
        OutlineOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get outline by ID
#[tauri::command]
pub async fn get_outline(id: String) -> CommandResponse<Option<Outline>> {
    async fn get(id: String) -> Result<Option<Outline>> {
        let pool = get_pool()?;
        OutlineOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Get outline by chapter number
#[tauri::command]
pub async fn get_outline_by_chapter(project_id: String, chapter_number: i32) -> CommandResponse<Option<Outline>> {
    async fn get_by_chapter(project_id: String, chapter_number: i32) -> Result<Option<Outline>> {
        let pool = get_pool()?;
        OutlineOps::get_by_chapter(&pool, &project_id, chapter_number).await
    }
    
    get_by_chapter(project_id, chapter_number).await.into()
}

/// Update outline
#[tauri::command]
pub async fn update_outline(request: UpdateOutlineRequest) -> CommandResponse<()> {
    async fn update(request: UpdateOutlineRequest) -> Result<()> {
        let pool = get_pool()?;
        OutlineOps::update(&pool, &request.id, request.title, request.summary, request.pov, request.tense, request.character_pov_ids).await
    }
    
    update(request).await.into()
}

/// Delete outline
#[tauri::command]
pub async fn delete_outline(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        OutlineOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Search outlines
#[tauri::command]
pub async fn search_outlines(project_id: String, query: String) -> CommandResponse<Vec<Outline>> {
    async fn search(project_id: String, query: String) -> Result<Vec<Outline>> {
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
            character_pov_ids: request.character_pov_ids.unwrap_or_default(),
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
        let pool = get_pool()?;
        SceneOps::get_by_outline(&pool, &outline_id).await
    }
    
    get_by_outline(outline_id).await.into()
}

/// Get scene by ID
#[tauri::command]
pub async fn get_scene(id: String) -> CommandResponse<Option<Scene>> {
    async fn get(id: String) -> Result<Option<Scene>> {
        let pool = get_pool()?;
        SceneOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Update scene
#[tauri::command]
pub async fn update_scene(request: UpdateSceneRequest) -> CommandResponse<()> {
    async fn update(request: UpdateSceneRequest) -> Result<()> {
        let pool = get_pool()?;
        SceneOps::update(&pool, &request.id, request.title, request.summary, request.extra_instructions, request.pov, request.tense, request.character_pov_ids, request.word_count_estimate, request.credit_estimate).await
    }
    
    update(request).await.into()
}

/// Delete scene
#[tauri::command]
pub async fn delete_scene(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        SceneOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Validate scene
#[tauri::command]
pub async fn validate_scene(id: String) -> CommandResponse<()> {
    async fn validate(id: String) -> Result<()> {
        let pool = get_pool()?;
        SceneOps::validate(&pool, &id).await
    }
    
    validate(id).await.into()
}

/// Search scenes
#[tauri::command]
pub async fn search_scenes(outline_id: String, query: String) -> CommandResponse<Vec<Scene>> {
    async fn search(outline_id: String, query: String) -> Result<Vec<Scene>> {
        let pool = get_pool()?;
        SceneOps::search(&pool, &outline_id, &query).await
    }
    
    search(outline_id, query).await.into()
}