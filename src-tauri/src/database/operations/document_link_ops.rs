use crate::database::models::DocumentLink;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// DocumentLink operations
pub struct DocumentLinkOps;

impl DocumentLinkOps {
    /// Create a new document link
    pub async fn create(pool: &Pool<Sqlite>, mut link: DocumentLink) -> Result<DocumentLink> {
        link.id = Uuid::new_v4().to_string();
        link.created_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO document_links (id, from_document_id, to_document_id, link_order, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&link.id)
        .bind(&link.from_document_id)
        .bind(&link.to_document_id)
        .bind(link.link_order)
        .bind(link.created_at)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create document link: {}", e)))?;
        
        Ok(link)
    }
    
    /// Get a document link by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<DocumentLink>> {
        let link = sqlx::query_as::<_, DocumentLink>("SELECT * FROM document_links WHERE id = ?")
            .bind(id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get document link: {}", e)))?;
        
        Ok(link)
    }
    
    /// Get links from a document (outgoing links)
    pub async fn get_outgoing_links(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<DocumentLink>> {
        let links = sqlx::query_as::<_, DocumentLink>(
            "SELECT * FROM document_links WHERE from_document_id = ? ORDER BY link_order"
        )
        .bind(document_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get outgoing document links: {}", e)))?;
        
        Ok(links)
    }
    
    /// Get links to a document (incoming links)
    pub async fn get_incoming_links(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<DocumentLink>> {
        let links = sqlx::query_as::<_, DocumentLink>(
            "SELECT * FROM document_links WHERE to_document_id = ? ORDER BY link_order"
        )
        .bind(document_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get incoming document links: {}", e)))?;
        
        Ok(links)
    }
    
    /// Get all links for a document (both incoming and outgoing)
    pub async fn get_all_links_for_document(pool: &Pool<Sqlite>, document_id: &str) -> Result<Vec<DocumentLink>> {
        let links = sqlx::query_as::<_, DocumentLink>(
            r#"
            SELECT * FROM document_links 
            WHERE from_document_id = ? OR to_document_id = ?
            ORDER BY link_order
            "#
        )
        .bind(document_id)
        .bind(document_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get all document links: {}", e)))?;
        
        Ok(links)
    }
    
    /// Update a document link
    pub async fn update(pool: &Pool<Sqlite>, link: &DocumentLink) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE document_links SET from_document_id = ?, to_document_id = ?, link_order = ?
            WHERE id = ?
            "#,
        )
        .bind(&link.from_document_id)
        .bind(&link.to_document_id)
        .bind(link.link_order)
        .bind(&link.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update document link: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a document link
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM document_links WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete document link: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete all links for a document (both incoming and outgoing)
    pub async fn delete_all_links_for_document(pool: &Pool<Sqlite>, document_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM document_links 
            WHERE from_document_id = ? OR to_document_id = ?
            "#
        )
        .bind(document_id)
        .bind(document_id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to delete document links: {}", e)))?;
        
        Ok(())
    }
    
    /// Get linked documents with details
    pub async fn get_linked_documents(
        pool: &Pool<Sqlite>, 
        document_id: &str
    ) -> Result<LinkedDocuments> {
        // Get previous documents (documents that link to this one)
        let previous = sqlx::query_as!(
            LinkedDocument,
            r#"
            SELECT 
                d.id, 
                d.title, 
                d.document_type as "document_type: _", 
                dl.link_order
            FROM 
                documents d
            JOIN 
                document_links dl ON d.id = dl.from_document_id
            WHERE 
                dl.to_document_id = ?
            ORDER BY 
                dl.link_order
            "#,
            document_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get previous documents: {}", e)))?;
        
        // Get next documents (documents that this one links to)
        let next = sqlx::query_as!(
            LinkedDocument,
            r#"
            SELECT 
                d.id, 
                d.title, 
                d.document_type as "document_type: _", 
                dl.link_order
            FROM 
                documents d
            JOIN 
                document_links dl ON d.id = dl.to_document_id
            WHERE 
                dl.from_document_id = ?
            ORDER BY 
                dl.link_order
            "#,
            document_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get next documents: {}", e)))?;
        
        Ok(LinkedDocuments { previous, next })
    }
}

/// Linked document with details
#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct LinkedDocument {
    pub id: String,
    pub title: String,
    pub document_type: crate::database::models::DocumentType,
    pub link_order: i32,
}

/// Collection of linked documents
#[derive(Debug, Clone, serde::Serialize)]
pub struct LinkedDocuments {
    pub previous: Vec<LinkedDocument>,
    pub next: Vec<LinkedDocument>,
}
