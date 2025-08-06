//! Database operations for StoryWeaver
//! CRUD operations for all models

use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Project operations
pub struct ProjectOps;

impl ProjectOps {
    /// Create a new project
    pub async fn create(pool: &Pool<Sqlite>, mut project: Project) -> Result<Project> {
        project.id = Uuid::new_v4().to_string();
        project.created_at = Utc::now();
        project.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, description, genre, target_word_count, 
                                current_word_count, status, created_at, updated_at, settings)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&project.id)
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.genre)
        .bind(project.target_word_count)
        .bind(project.current_word_count)
        .bind(&project.status)
        .bind(project.created_at)
        .bind(project.updated_at)
        .bind(&project.settings)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create project: {}", e)))?;
        
        Ok(project)
    }
    
    /// Get a project by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<Project>> {
        let project = sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get project: {}", e)))?;
        
        Ok(project)
    }
    
    /// Get all projects
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Project>> {
        let projects = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects ORDER BY updated_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get projects: {}", e)))?;
        
        Ok(projects)
    }
    
    /// Update a project
    pub async fn update(pool: &Pool<Sqlite>, project: &Project) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE projects SET name = ?, description = ?, genre = ?, target_word_count = ?,
                              current_word_count = ?, status = ?, updated_at = ?, settings = ?
            WHERE id = ?
            "#,
        )
        .bind(&project.name)
        .bind(&project.description)
        .bind(&project.genre)
        .bind(project.target_word_count)
        .bind(project.current_word_count)
        .bind(&project.status)
        .bind(Utc::now())
        .bind(&project.settings)
        .bind(&project.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update project: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a project
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete project: {}", e)))?;
        
        Ok(())
    }
    
    /// Update word count for a project
    pub async fn update_word_count(pool: &Pool<Sqlite>, project_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE projects SET current_word_count = (
                SELECT COALESCE(SUM(word_count), 0) FROM documents WHERE project_id = ?
            ), updated_at = ? WHERE id = ?
            "#,
        )
        .bind(project_id)
        .bind(Utc::now())
        .bind(project_id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update word count: {}", e)))?;
        
        Ok(())
    }
}

/// Document operations
pub struct DocumentOps;

impl DocumentOps {
    /// Create a new document
    pub async fn create(pool: &Pool<Sqlite>, mut document: Document) -> Result<Document> {
        document.id = Uuid::new_v4().to_string();
        document.created_at = Utc::now();
        document.updated_at = Utc::now();
        document.word_count = Self::count_words(&document.content);
        
        sqlx::query(
            r#"
            INSERT INTO documents (id, project_id, title, content, document_type, 
                                 order_index, word_count, parent_id, created_at, updated_at, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&document.id)
        .bind(&document.project_id)
        .bind(&document.title)
        .bind(&document.content)
        .bind(&document.document_type)
        .bind(document.order_index)
        .bind(document.word_count)
        .bind(&document.parent_id)
        .bind(document.created_at)
        .bind(document.updated_at)
        .bind(&document.metadata)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create document: {}", e)))?;
        
        // Update project word count
        ProjectOps::update_word_count(pool, &document.project_id).await?;
        
        Ok(document)
    }
    
    /// Get a document by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<Document>> {
        let document = sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get document: {}", e)))?;
        
        Ok(document)
    }
    
    /// Get documents by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Document>> {
        let documents = sqlx::query_as::<_, Document>(
            "SELECT * FROM documents WHERE project_id = ? ORDER BY order_index, created_at"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get documents: {}", e)))?;
        
        Ok(documents)
    }
    
    /// Update a document
    pub async fn update(pool: &Pool<Sqlite>, document: &Document) -> Result<()> {
        let word_count = Self::count_words(&document.content);
        
        sqlx::query(
            r#"
            UPDATE documents SET title = ?, content = ?, document_type = ?, order_index = ?,
                               word_count = ?, parent_id = ?, updated_at = ?, metadata = ?
            WHERE id = ?
            "#,
        )
        .bind(&document.title)
        .bind(&document.content)
        .bind(&document.document_type)
        .bind(document.order_index)
        .bind(word_count)
        .bind(&document.parent_id)
        .bind(Utc::now())
        .bind(&document.metadata)
        .bind(&document.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update document: {}", e)))?;
        
        // Update project word count
        ProjectOps::update_word_count(pool, &document.project_id).await?;
        
        Ok(())
    }
    
    /// Delete a document
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        // Get project_id before deletion for word count update
        let project_id = sqlx::query_scalar::<_, String>(
            "SELECT project_id FROM documents WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get project_id: {}", e)))?;
        
        sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete document: {}", e)))?;
        
        // Update project word count if we found the project
        if let Some(project_id) = project_id {
            ProjectOps::update_word_count(pool, &project_id).await?;
        }
        
        Ok(())
    }
    
    /// Search documents using full-text search
    pub async fn search(pool: &Pool<Sqlite>, project_id: &str, query: &str) -> Result<Vec<Document>> {
        let documents = sqlx::query_as::<_, Document>(
            r#"
            SELECT d.* FROM documents d
            JOIN documents_fts fts ON d.rowid = fts.rowid
            WHERE d.project_id = ? AND documents_fts MATCH ?
            ORDER BY rank
            "#,
        )
        .bind(project_id)
        .bind(query)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to search documents: {}", e)))?;
        
        Ok(documents)
    }
    
    /// Count words in text
    fn count_words(text: &str) -> i32 {
        text.split_whitespace().count() as i32
    }
}

/// Character operations
pub struct CharacterOps;

impl CharacterOps {
    /// Create a new character
    pub async fn create(pool: &Pool<Sqlite>, mut character: Character) -> Result<Character> {
        character.id = Uuid::new_v4().to_string();
        character.created_at = Utc::now();
        character.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO characters (id, project_id, name, description, role, age, appearance,
                                  personality, background, goals, relationships, visibility,
                                  created_at, updated_at, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&character.id)
        .bind(&character.project_id)
        .bind(&character.name)
        .bind(&character.description)
        .bind(&character.role)
        .bind(character.age)
        .bind(&character.appearance)
        .bind(&character.personality)
        .bind(&character.background)
        .bind(&character.goals)
        .bind(&character.relationships)
        .bind(&character.visibility)
        .bind(character.created_at)
        .bind(character.updated_at)
        .bind(&character.metadata)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create character: {}", e)))?;
        
        Ok(character)
    }
    
    /// Get characters by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Character>> {
        let characters = sqlx::query_as::<_, Character>(
            "SELECT * FROM characters WHERE project_id = ? ORDER BY name"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get characters: {}", e)))?;
        
        Ok(characters)
    }
    
    /// Update a character
    pub async fn update(pool: &Pool<Sqlite>, character: &Character) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE characters SET name = ?, description = ?, role = ?, age = ?, appearance = ?,
                                personality = ?, background = ?, goals = ?, relationships = ?,
                                visibility = ?, updated_at = ?, metadata = ?
            WHERE id = ?
            "#,
        )
        .bind(&character.name)
        .bind(&character.description)
        .bind(&character.role)
        .bind(character.age)
        .bind(&character.appearance)
        .bind(&character.personality)
        .bind(&character.background)
        .bind(&character.goals)
        .bind(&character.relationships)
        .bind(&character.visibility)
        .bind(Utc::now())
        .bind(&character.metadata)
        .bind(&character.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update character: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a character
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM characters WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete character: {}", e)))?;
        
        Ok(())
    }
}

/// Location operations
pub struct LocationOps;

impl LocationOps {
    /// Create a new location
    pub async fn create(pool: &Pool<Sqlite>, mut location: Location) -> Result<Location> {
        location.id = Uuid::new_v4().to_string();
        location.created_at = Utc::now();
        location.updated_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO locations (id, project_id, name, description, location_type, geography,
                                 climate, culture, history, significance, visibility,
                                 created_at, updated_at, metadata)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&location.id)
        .bind(&location.project_id)
        .bind(&location.name)
        .bind(&location.description)
        .bind(&location.location_type)
        .bind(&location.geography)
        .bind(&location.climate)
        .bind(&location.culture)
        .bind(&location.history)
        .bind(&location.significance)
        .bind(&location.visibility)
        .bind(location.created_at)
        .bind(location.updated_at)
        .bind(&location.metadata)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create location: {}", e)))?;
        
        Ok(location)
    }
    
    /// Get locations by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str) -> Result<Vec<Location>> {
        let locations = sqlx::query_as::<_, Location>(
            "SELECT * FROM locations WHERE project_id = ? ORDER BY name"
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get locations: {}", e)))?;
        
        Ok(locations)
    }
    
    /// Update a location
    pub async fn update(pool: &Pool<Sqlite>, location: &Location) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE locations SET name = ?, description = ?, location_type = ?, geography = ?,
                               climate = ?, culture = ?, history = ?, significance = ?,
                               visibility = ?, updated_at = ?, metadata = ?
            WHERE id = ?
            "#,
        )
        .bind(&location.name)
        .bind(&location.description)
        .bind(&location.location_type)
        .bind(&location.geography)
        .bind(&location.climate)
        .bind(&location.culture)
        .bind(&location.history)
        .bind(&location.significance)
        .bind(&location.visibility)
        .bind(Utc::now())
        .bind(&location.metadata)
        .bind(&location.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update location: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a location
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM locations WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete location: {}", e)))?;
        
        Ok(())
    }
}

/// AI generation history operations
pub struct AIHistoryOps;

impl AIHistoryOps {
    /// Create a new AI generation record
    pub async fn create(pool: &Pool<Sqlite>, mut record: AIGenerationHistory) -> Result<AIGenerationHistory> {
        record.id = Uuid::new_v4().to_string();
        record.created_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO ai_generation_history (id, project_id, document_id, generation_type,
                                             provider, model, prompt, response, token_count,
                                             cost_estimate, context_used, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&record.id)
        .bind(&record.project_id)
        .bind(&record.document_id)
        .bind(&record.generation_type)
        .bind(&record.provider)
        .bind(&record.model)
        .bind(&record.prompt)
        .bind(&record.response)
        .bind(record.token_count)
        .bind(record.cost_estimate)
        .bind(&record.context_used)
        .bind(record.created_at)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create AI history record: {}", e)))?;
        
        Ok(record)
    }
    
    /// Get AI history by project ID
    pub async fn get_by_project(pool: &Pool<Sqlite>, project_id: &str, limit: Option<i32>) -> Result<Vec<AIGenerationHistory>> {
        let limit = limit.unwrap_or(100);
        
        let records = sqlx::query_as::<_, AIGenerationHistory>(
            "SELECT * FROM ai_generation_history WHERE project_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(project_id)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get AI history: {}", e)))?;
        
        Ok(records)
    }
}