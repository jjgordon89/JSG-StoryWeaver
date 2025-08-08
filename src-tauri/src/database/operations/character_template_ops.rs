use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::collections::HashMap;
use uuid::Uuid;

/// Character template definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub archetype: String,
    pub default_traits: Vec<CharacterTemplateTrait>,
    pub is_system: bool, // System templates vs user-created
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Character template trait definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTemplateTrait {
    pub trait_name: String,
    pub default_value: Option<String>,
    pub is_required: bool,
    pub description: String,
}

/// Character template operations
pub struct CharacterTemplateOps;

impl CharacterTemplateOps {
    /// Get all system character templates
    pub fn get_system_templates() -> Vec<CharacterTemplate> {
        vec![
            // Protagonist Templates
            CharacterTemplate {
                id: "hero-classic".to_string(),
                name: "Classic Hero".to_string(),
                description: "A traditional heroic protagonist with noble qualities and clear motivations".to_string(),
                archetype: "protagonist".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Brave, determined, and morally upright with a strong sense of justice".to_string()),
                        is_required: true,
                        description: "Core personality traits that define the hero's character".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "motivation".to_string(),
                        default_value: Some("To protect others and uphold what is right".to_string()),
                        is_required: true,
                        description: "What drives the hero to action".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "strength".to_string(),
                        default_value: Some("Unwavering moral compass and physical/mental resilience".to_string()),
                        is_required: false,
                        description: "The hero's greatest strengths".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "weakness".to_string(),
                        default_value: Some("Sometimes too trusting or rigid in moral thinking".to_string()),
                        is_required: false,
                        description: "Character flaws that create conflict".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "background".to_string(),
                        default_value: None,
                        is_required: false,
                        description: "The hero's origin story and formative experiences".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            CharacterTemplate {
                id: "antihero".to_string(),
                name: "Antihero".to_string(),
                description: "A morally complex protagonist with questionable methods but relatable goals".to_string(),
                archetype: "protagonist".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Cynical, pragmatic, and morally ambiguous with hidden depths".to_string()),
                        is_required: true,
                        description: "Complex personality with both dark and redeeming qualities".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "motivation".to_string(),
                        default_value: Some("Personal gain or revenge, but with underlying noble intentions".to_string()),
                        is_required: true,
                        description: "What drives the antihero, often conflicted".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "strength".to_string(),
                        default_value: Some("Resourcefulness and willingness to do what others won't".to_string()),
                        is_required: false,
                        description: "Unconventional strengths".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "weakness".to_string(),
                        default_value: Some("Trust issues and tendency toward self-destructive behavior".to_string()),
                        is_required: false,
                        description: "Character flaws that complicate their journey".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Antagonist Templates
            CharacterTemplate {
                id: "villain-classic".to_string(),
                name: "Classic Villain".to_string(),
                description: "A traditional antagonist with clear evil motivations and methods".to_string(),
                archetype: "antagonist".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Ruthless, cunning, and power-hungry with little regard for others".to_string()),
                        is_required: true,
                        description: "Core villainous traits".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "motivation".to_string(),
                        default_value: Some("To gain power, control, or destroy what the hero represents".to_string()),
                        is_required: true,
                        description: "What drives the villain's actions".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "strength".to_string(),
                        default_value: Some("Strategic thinking and willingness to use any means necessary".to_string()),
                        is_required: false,
                        description: "The villain's advantages".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "weakness".to_string(),
                        default_value: Some("Arrogance and underestimation of others".to_string()),
                        is_required: false,
                        description: "Fatal flaws that lead to downfall".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            CharacterTemplate {
                id: "sympathetic-villain".to_string(),
                name: "Sympathetic Villain".to_string(),
                description: "An antagonist with understandable motivations and tragic circumstances".to_string(),
                archetype: "antagonist".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Driven by pain or loss, with genuine beliefs about their cause".to_string()),
                        is_required: true,
                        description: "Complex personality shaped by tragedy".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "motivation".to_string(),
                        default_value: Some("To right a perceived wrong or prevent others from suffering as they did".to_string()),
                        is_required: true,
                        description: "Understandable, even noble motivations".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "background".to_string(),
                        default_value: Some("Tragic past that shaped their worldview and methods".to_string()),
                        is_required: true,
                        description: "The tragedy that created the villain".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            // Supporting Character Templates
            CharacterTemplate {
                id: "mentor".to_string(),
                name: "Mentor".to_string(),
                description: "A wise guide who helps the protagonist grow and learn".to_string(),
                archetype: "supporting".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Wise, patient, and experienced with a deep understanding of the world".to_string()),
                        is_required: true,
                        description: "Mentoring qualities and wisdom".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "background".to_string(),
                        default_value: Some("Extensive experience in the protagonist's field or similar challenges".to_string()),
                        is_required: true,
                        description: "What qualifies them to be a mentor".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "relationship".to_string(),
                        default_value: Some("Protective and nurturing toward the protagonist".to_string()),
                        is_required: false,
                        description: "How they relate to the protagonist".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            CharacterTemplate {
                id: "sidekick".to_string(),
                name: "Loyal Sidekick".to_string(),
                description: "A faithful companion who supports the protagonist's journey".to_string(),
                archetype: "supporting".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Loyal, brave, and supportive with unwavering faith in the protagonist".to_string()),
                        is_required: true,
                        description: "Supportive personality traits".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "relationship".to_string(),
                        default_value: Some("Deep friendship and loyalty to the protagonist".to_string()),
                        is_required: true,
                        description: "Bond with the main character".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "strength".to_string(),
                        default_value: Some("Complementary skills that support the protagonist's weaknesses".to_string()),
                        is_required: false,
                        description: "How they help the protagonist".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
            CharacterTemplate {
                id: "love-interest".to_string(),
                name: "Love Interest".to_string(),
                description: "A romantic partner who adds emotional depth to the story".to_string(),
                archetype: "supporting".to_string(),
                default_traits: vec![
                    CharacterTemplateTrait {
                        trait_name: "personality".to_string(),
                        default_value: Some("Attractive, independent, and emotionally intelligent".to_string()),
                        is_required: true,
                        description: "Qualities that make them appealing".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "relationship".to_string(),
                        default_value: Some("Romantic tension and emotional connection with the protagonist".to_string()),
                        is_required: true,
                        description: "Nature of the romantic relationship".to_string(),
                    },
                    CharacterTemplateTrait {
                        trait_name: "goal".to_string(),
                        default_value: Some("Personal goals that may complement or conflict with the protagonist's".to_string()),
                        is_required: false,
                        description: "Their own motivations and desires".to_string(),
                    },
                ],
                is_system: true,
                created_at: Utc::now(),
            },
        ]
    }

    /// Get templates by archetype
    pub fn get_templates_by_archetype(archetype: &str) -> Vec<CharacterTemplate> {
        Self::get_system_templates()
            .into_iter()
            .filter(|template| template.archetype == archetype)
            .collect()
    }

    /// Get template by ID
    pub fn get_template_by_id(template_id: &str) -> Option<CharacterTemplate> {
        Self::get_system_templates()
            .into_iter()
            .find(|template| template.id == template_id)
    }

    /// Apply template to create character with default traits
    pub async fn apply_template_to_character(
        pool: &Pool<Sqlite>,
        template_id: &str,
        character_id: &str,
        overrides: Option<HashMap<String, String>>,
    ) -> Result<Vec<CharacterTrait>> {
        let template = Self::get_template_by_id(template_id)
            .ok_or_else(|| StoryWeaverError::Internal {
                message: format!("Template not found: {}", template_id),
            })?;

        let mut created_traits = Vec::new();
        let overrides = overrides.unwrap_or_default();

        for template_trait in template.default_traits {
            let trait_value = overrides
                .get(&template_trait.trait_name)
                .cloned()
                .or(template_trait.default_value);

            let character_trait = CharacterTrait::new(
                character_id.to_string(),
                template_trait.trait_name.clone(),
                trait_value,
            );

            // Create the trait in the database
            sqlx::query(
                r#"
                INSERT INTO character_traits (id, character_id, trait_name, trait_value, is_visible, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&character_trait.id)
            .bind(&character_trait.character_id)
            .bind(&character_trait.trait_name)
            .bind(&character_trait.trait_value)
            .bind(character_trait.is_visible)
            .bind(character_trait.created_at)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create character trait: {}", e)))?;

            created_traits.push(character_trait);
        }

        Ok(created_traits)
    }

    /// Get available archetypes
    pub fn get_archetypes() -> Vec<String> {
        vec![
            "protagonist".to_string(),
            "antagonist".to_string(),
            "supporting".to_string(),
        ]
    }
}