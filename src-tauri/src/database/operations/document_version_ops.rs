use crate::database::models::DocumentVersion;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// DocumentVersion operations
impl super::DocumentVersionOps {
    /// Create a new document version
    pub async fn create(pool: &Pool<Sqlite>, mut version: DocumentVersion) -> Result<DocumentVersion> {
        version.id = Uuid::new_v4().to_string();
        version.created_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO document_versions (
                id, document_id, content, word_count, version_number, 
                created_at, created_by, comment
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&version.id)
        .bind(&version.document_id)
        .bind(&version.content)
        .bind(version.word_count)
        .bind(version.version_number)
        .bind(version.created_at)
        .bind(&version.created_by)
        .bind(&version.comment)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create document version: {}", e)))?;
        
        Ok(version)
    }
    
    /// Get a document version by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<DocumentVersion>> {
        let version = sqlx::query_as::<_, DocumentVersion>("SELECT * FROM document_versions WHERE id = ?")
            .bind(id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get document version: {}", e)))?;
        
        Ok(version)
    }
    
    /// Get versions for a document
    pub async fn get_versions(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<DocumentVersion>> {
        let versions = sqlx::query_as::<_, DocumentVersion>(
            "SELECT * FROM document_versions WHERE document_id = ? ORDER BY version_number DESC"
        )
        .bind(document_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get document versions: {}", e)))?;
        
        Ok(versions)
    }
    
    /// Get latest version for a document
    pub async fn get_latest_version(pool: &Pool<Sqlite>, document_id: &str) -> Result<Option<DocumentVersion>> {
        let version = sqlx::query_as::<_, DocumentVersion>(
            "SELECT * FROM document_versions WHERE document_id = ? ORDER BY version_number DESC LIMIT 1"
        )
        .bind(document_id)
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get latest document version: {}", e)))?;
        
        Ok(version)
    }
    
    /// Get next version number for a document
    pub async fn get_next_version_number(pool: &Pool<Sqlite>, document_id: &str) -> Result<i32> {
        let max_version = sqlx::query_scalar::<_, Option<i32>>(
            "SELECT MAX(version_number) FROM document_versions WHERE document_id = ?"
        )
        .bind(document_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get max version number: {}", e)))?;
        
        Ok(max_version.unwrap_or(0) + 1)
    }
    
    /// Create a new version from current document content
    pub async fn create_from_document(
        pool: &Pool<Sqlite>, 
        document_id: &str, 
        created_by: Option<String>,
        comment: Option<String>
    ) -> Result<DocumentVersion> {
        // Get the document content
        let document = sqlx::query!(
            "SELECT content, word_count FROM documents WHERE id = ?",
            document_id
        )
        .fetch_optional(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get document: {}", e)))?
        .ok_or_else(|| StoryWeaverError::DocumentNotFound { id: document_id.to_string() })?;
        
        // Get the next version number
        let version_number = Self::get_next_version_number(pool, document_id).await?;
        
        // Create the version
        let version = DocumentVersion {
            id: Uuid::new_v4().to_string(),
            document_id: document_id.to_string(),
            content: document.content.unwrap_or_default(),
            word_count: document.word_count.unwrap_or(0) as i32,
            version_number,
            created_at: Utc::now(),
            created_by,
            comment,
        };
        
        Self::create(pool, version).await
    }
    
    /// Delete a document version
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM document_versions WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete document version: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete all versions for a document
    pub async fn delete_all_versions(pool: &Pool<Sqlite>, document_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM document_versions WHERE document_id = ?")
            .bind(document_id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete document versions: {}", e)))?;
        
        Ok(())
    }
    
    /// Restore a document to a specific version
    pub async fn restore_version(pool: &Pool<Sqlite>, version_id: &str) -> Result<()> {
        // Start a transaction
        let mut tx = pool.begin().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Get the version
        let version = Self::get_by_id(pool, version_id).await?
            .ok_or_else(|| StoryWeaverError::VersionNotFound { id: version_id.to_string() })?;
        
        // Update the document with the version content
        sqlx::query(
            r#"
            UPDATE documents 
            SET content = ?, word_count = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&version.content)
        .bind(version.word_count)
        .bind(Utc::now())
        .bind(&version.document_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to restore document version: {}", e)))?;
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(())
    }
    
    /// Get version history with metadata
    pub async fn get_version_history(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<VersionHistoryItem>> {
        let versions = sqlx::query_as!(
            VersionHistoryItem,
            r#"
            SELECT 
                v.id,
                v.version_number as "version_number: i32",
                v.word_count as "word_count: i32",
                v.created_at as "created_at: chrono::NaiveDateTime",
                v.created_by,
                v.comment,
                (v.word_count - COALESCE(prev.word_count, 0)) as "word_count_change: i32"
            FROM 
                document_versions v
            LEFT JOIN 
                document_versions prev ON v.document_id = prev.document_id 
                AND prev.version_number = v.version_number - 1
            WHERE 
                v.document_id = ?
            ORDER BY 
                v.version_number DESC
            "#,
            document_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get version history: {}", e)))?;
        
        Ok(versions)
    }
}

/// Version history item with metadata
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct VersionHistoryItem {
    pub id: Option<String>,
    pub version_number: i32,
    pub word_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<String>,
    pub comment: Option<String>,
    pub word_count_change: i32,
}
