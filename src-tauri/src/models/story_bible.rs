//! Story Bible data models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBible {
    pub id: Option<i32>,
    pub project_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBibleEntry {
    pub id: Option<i32>,
    pub story_bible_id: i32,
    pub entry_type: String,
    pub title: String,
    pub content: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryBibleElements {
    pub characters: Vec<HashMap<String, String>>,
    pub locations: Vec<HashMap<String, String>>,
    pub items: Vec<HashMap<String, String>>,
    pub lore: Vec<HashMap<String, String>>,
}
