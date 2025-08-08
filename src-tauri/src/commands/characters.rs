//! Character command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::CharacterOps};
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Create character request
#[derive(Debug, Deserialize)]
pub struct CreateCharacterRequest {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub role: Option<CharacterRole>,
    pub age: Option<i32>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub goals: Option<String>,
    pub relationships: Option<String>,
    pub visibility: Option<VisibilityLevel>,
}

/// Update character request
#[derive(Debug, Deserialize)]
pub struct UpdateCharacterRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub role: Option<CharacterRole>,
    pub age: Option<i32>,
    pub appearance: Option<String>,
    pub personality: Option<String>,
    pub background: Option<String>,
    pub goals: Option<String>,
    pub relationships: Option<String>,
    pub visibility: Option<VisibilityLevel>,
    pub metadata: Option<String>,
}

/// Create a new character
#[tauri::command]
pub async fn create_character(request: CreateCharacterRequest) -> CommandResponse<Character> {
    async fn create(request: CreateCharacterRequest) -> Result<Character> {
        let pool = get_pool()?;
        
        let mut character = Character::new(
            request.project_id,
            request.name,
            request.role.unwrap_or(CharacterRole::Supporting),
        );
        
        // Set optional fields
        character.description = request.description;
        character.age = request.age;
        character.appearance = request.appearance;
        character.personality = request.personality;
        character.background = request.background;
        character.goals = request.goals;
        if let Some(relationships) = request.relationships {
            character.relationships = relationships;
        }
        if let Some(visibility) = request.visibility {
            character.visibility = visibility;
        }
        
        CharacterOps::create(&pool, character).await
    }
    
    create(request).await.into()
}

/// Get characters by project ID
#[tauri::command]
pub async fn get_characters(project_id: String) -> CommandResponse<Vec<Character>> {
    async fn get_by_project(project_id: String) -> Result<Vec<Character>> {
        let pool = get_pool()?;
        CharacterOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get a character by ID
#[tauri::command]
pub async fn get_character(id: String) -> CommandResponse<Option<Character>> {
    async fn get(id: String) -> Result<Option<Character>> {
        let pool = get_pool()?;
        
        let character = sqlx::query_as::<_, Character>("SELECT * FROM characters WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get character: {}", e)))?;
        
        Ok(character)
    }
    
    get(id).await.into()
}

/// Update a character
#[tauri::command]
pub async fn update_character(request: UpdateCharacterRequest) -> CommandResponse<()> {
    async fn update(request: UpdateCharacterRequest) -> Result<()> {
        let pool = get_pool()?;
        
        // Get existing character
        let mut character = sqlx::query_as::<_, Character>("SELECT * FROM characters WHERE id = ?")
            .bind(&request.id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get character: {}", e)))?
            .ok_or_else(|| crate::error::StoryWeaverError::Internal { message: format!("Character not found: {}", request.id) })?;
        
        // Update fields if provided
        if let Some(name) = request.name {
            character.name = name;
        }
        if let Some(description) = request.description {
            character.description = Some(description);
        }
        if let Some(role) = request.role {
            character.role = role;
        }
        if let Some(age) = request.age {
            character.age = Some(age);
        }
        if let Some(appearance) = request.appearance {
            character.appearance = Some(appearance);
        }
        if let Some(personality) = request.personality {
            character.personality = Some(personality);
        }
        if let Some(background) = request.background {
            character.background = Some(background);
        }
        if let Some(goals) = request.goals {
            character.goals = Some(goals);
        }
        if let Some(relationships) = request.relationships {
            character.relationships = relationships;
        }
        if let Some(visibility) = request.visibility {
            character.visibility = visibility;
        }
        if let Some(metadata) = request.metadata {
            character.metadata = metadata;
        }
        
        CharacterOps::update(&pool, &character).await
    }
    
    update(request).await.into()
}

/// Delete a character
#[tauri::command]
pub async fn delete_character(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        CharacterOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Character summary for quick reference
#[derive(Debug, Serialize)]
pub struct CharacterSummary {
    pub id: String,
    pub name: String,
    pub role: Option<CharacterRole>,
    pub description: Option<String>,
    pub key_traits: Vec<String>,
}

/// Get character summaries for a project (lightweight version)
#[tauri::command]
pub async fn get_character_summaries(project_id: String) -> CommandResponse<Vec<CharacterSummary>> {
    async fn get_summaries(project_id: String) -> Result<Vec<CharacterSummary>> {
        let pool = get_pool()?;
        let characters = CharacterOps::get_by_project(&pool, &project_id).await?;
        
        let summaries = characters
            .into_iter()
            .map(|character| {
                let mut key_traits = Vec::new();
                
                // Extract key traits from personality and appearance
                if let Some(personality) = &character.personality {
                    if !personality.is_empty() {
                        // Simple extraction - take first sentence or up to 50 chars
                        let trait_text = personality
                            .split('.')
                            .next()
                            .unwrap_or(personality)
                            .chars()
                            .take(50)
                            .collect::<String>();
                        if !trait_text.trim().is_empty() {
                            key_traits.push(trait_text.trim().to_string());
                        }
                    }
                }
                
                if let Some(appearance) = &character.appearance {
                    if !appearance.is_empty() && key_traits.len() < 3 {
                        let trait_text = appearance
                            .split('.')
                            .next()
                            .unwrap_or(appearance)
                            .chars()
                            .take(50)
                            .collect::<String>();
                        if !trait_text.trim().is_empty() {
                            key_traits.push(trait_text.trim().to_string());
                        }
                    }
                }
                
                CharacterSummary {
                    id: character.id,
                    name: character.name,
                    role: Some(character.role),
                    description: character.description,
                    key_traits,
                }
            })
            .collect();
        
        Ok(summaries)
    }
    
    get_summaries(project_id).await.into()
}

/// Character relationship mapping
#[derive(Debug, Serialize)]
pub struct CharacterRelationship {
    pub from_character_id: String,
    pub from_character_name: String,
    pub to_character_id: String,
    pub to_character_name: String,
    pub relationship_type: String,
    pub description: Option<String>,
}

/// Get character relationships for a project
#[tauri::command]
pub async fn get_character_relationships(project_id: String) -> CommandResponse<Vec<CharacterRelationship>> {
    async fn get_relationships(project_id: String) -> Result<Vec<CharacterRelationship>> {
        let pool = get_pool()?;
        let characters = CharacterOps::get_by_project(&pool, &project_id).await?;
        
        let mut relationships = Vec::new();
        
        // Simple relationship extraction from relationships field
        // In a more advanced implementation, this would be a separate table
        for character in &characters {
            let relationships_text = &character.relationships;
            if !relationships_text.is_empty() {
                // Parse relationships (simplified - assumes format like "friend of John, enemy of Jane")
                for other_character in &characters {
                    if other_character.id != character.id {
                        let other_name = &other_character.name;
                        if relationships_text.to_lowercase().contains(&other_name.to_lowercase()) {
                            // Extract relationship type (very basic)
                            let relationship_type = if relationships_text.to_lowercase().contains("friend") {
                                "friend".to_string()
                            } else if relationships_text.to_lowercase().contains("enemy") {
                                "enemy".to_string()
                            } else if relationships_text.to_lowercase().contains("family") {
                                "family".to_string()
                            } else if relationships_text.to_lowercase().contains("romantic") {
                                "romantic".to_string()
                            } else {
                                "acquaintance".to_string()
                            };
                            
                            relationships.push(CharacterRelationship {
                                from_character_id: character.id.clone(),
                                from_character_name: character.name.clone(),
                                to_character_id: other_character.id.clone(),
                                to_character_name: other_character.name.clone(),
                                relationship_type,
                                description: Some(relationships_text.clone()),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(relationships)
    }
    
    get_relationships(project_id).await.into()
}

/// Character statistics
#[derive(Debug, Serialize)]
pub struct CharacterStats {
    pub total_characters: i32,
    pub by_role: std::collections::HashMap<String, i32>,
    pub by_visibility: std::collections::HashMap<String, i32>,
    pub main_characters: Vec<CharacterSummary>,
}

/// Get character statistics for a project
#[tauri::command]
pub async fn get_character_stats(project_id: String) -> CommandResponse<CharacterStats> {
    async fn get_stats(project_id: String) -> Result<CharacterStats> {
        let pool = get_pool()?;
        let characters = CharacterOps::get_by_project(&pool, &project_id).await?;
        
        let total_characters = characters.len() as i32;
        
        // Count by role
        let mut by_role = std::collections::HashMap::new();
        for character in &characters {
            let role_str = format!("{:?}", character.role);
            *by_role.entry(role_str).or_insert(0) += 1;
        }
        
        // Count by visibility
        let mut by_visibility = std::collections::HashMap::new();
        for character in &characters {
            let visibility_str = format!("{:?}", character.visibility);
            *by_visibility.entry(visibility_str).or_insert(0) += 1;
        }
        
        // Get main characters (protagonists and major characters)
        let main_characters = characters
            .into_iter()
            .filter(|c| {
                matches!(c.role, CharacterRole::Protagonist | CharacterRole::Antagonist | CharacterRole::Supporting)
            })
            .map(|character| CharacterSummary {
                id: character.id,
                name: character.name,
                role: Some(character.role),
                description: character.description,
                key_traits: Vec::new(), // Simplified for stats
            })
            .collect();
        
        Ok(CharacterStats {
            total_characters,
            by_role,
            by_visibility,
            main_characters,
        })
    }
    
    get_stats(project_id).await.into()
}

/// Get characters by series ID
#[tauri::command]
pub async fn get_characters_by_series(series_id: String) -> CommandResponse<Vec<Character>> {
    async fn get_by_series(series_id: String) -> Result<Vec<Character>> {
        let pool = get_pool()?;
        CharacterOps::get_by_series(&pool, &series_id).await
    }
    
    get_by_series(series_id).await.into()
}

/// Get visible characters for a project (includes project-specific and series-shared)
#[tauri::command]
pub async fn get_visible_characters(project_id: String, series_id: Option<String>) -> CommandResponse<Vec<Character>> {
    async fn get_visible(project_id: String, series_id: Option<String>) -> Result<Vec<Character>> {
        let pool = get_pool()?;
        CharacterOps::get_visible_by_project(&pool, &project_id, series_id.as_deref()).await
    }
    
    get_visible(project_id, series_id).await.into()
}

/// Share a character to series
#[tauri::command]
pub async fn share_character_to_series(character_id: String, series_id: String) -> CommandResponse<()> {
    async fn share_to_series(character_id: String, series_id: String) -> Result<()> {
        let pool = get_pool()?;
        CharacterOps::share_to_series(&pool, &character_id, &series_id).await
    }
    
    share_to_series(character_id, series_id).await.into()
}

/// Unshare a character from series
#[tauri::command]
pub async fn unshare_character_from_series(character_id: String) -> CommandResponse<()> {
    async fn unshare_from_series(character_id: String) -> Result<()> {
        let pool = get_pool()?;
        CharacterOps::unshare_from_series(&pool, &character_id).await
    }
    
    unshare_from_series(character_id).await.into()
}
