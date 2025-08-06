use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Document operations
impl super::DocumentOps {
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
        super::ProjectOps::update_word_count(pool, &document.project_id).await?;
        
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
        super::ProjectOps::update_word_count(pool, &document.project_id).await?;
        
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
            super::ProjectOps::update_word_count(pool, &project_id).await?;
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
