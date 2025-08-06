//! Document link command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::*};
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Create document link request
#[derive(Debug, Deserialize)]
pub struct CreateDocumentLinkRequest {
    pub from_document_id: String,
    pub to_document_id: String,
    pub link_order: Option<i32>,
}

/// Update document link request
#[derive(Debug, Deserialize)]
pub struct UpdateDocumentLinkRequest {
    pub id: String,
    pub from_document_id: Option<String>,
    pub to_document_id: Option<String>,
    pub link_order: Option<i32>,
}

/// Create a new document link
#[tauri::command]
pub async fn create_document_link(request: CreateDocumentLinkRequest) -> CommandResponse<DocumentLink> {
    async fn create(request: CreateDocumentLinkRequest) -> Result<DocumentLink> {
        let pool = get_pool()?;
        
        let link = DocumentLink {
            id: String::new(), // Will be set by the create function
            from_document_id: request.from_document_id,
            to_document_id: request.to_document_id,
            link_order: request.link_order.unwrap_or(1),
            created_at: chrono::Utc::now(),
        };
        
        DocumentLinkOps::create(&pool, link).await
    }
    
    create(request).await.into()
}

/// Get a document link by ID
#[tauri::command]
pub async fn get_document_link(id: String) -> CommandResponse<Option<DocumentLink>> {
    async fn get(id: String) -> Result<Option<DocumentLink>> {
        let pool = get_pool()?;
        DocumentLinkOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Get outgoing links from a document
#[tauri::command]
pub async fn get_outgoing_links(document_id: String) -> CommandResponse<Vec<DocumentLink>> {
    async fn get_links(document_id: String) -> Result<Vec<DocumentLink>> {
        let pool = get_pool()?;
        DocumentLinkOps::get_outgoing_links(&pool, &document_id).await
    }
    
    get_links(document_id).await.into()
}

/// Get incoming links to a document
#[tauri::command]
pub async fn get_incoming_links(document_id: String) -> CommandResponse<Vec<DocumentLink>> {
    async fn get_links(document_id: String) -> Result<Vec<DocumentLink>> {
        let pool = get_pool()?;
        DocumentLinkOps::get_incoming_links(&pool, &document_id).await
    }
    
    get_links(document_id).await.into()
}

/// Get all links for a document (both incoming and outgoing)
#[tauri::command]
pub async fn get_all_links_for_document(document_id: String) -> CommandResponse<Vec<DocumentLink>> {
    async fn get_links(document_id: String) -> Result<Vec<DocumentLink>> {
        let pool = get_pool()?;
        DocumentLinkOps::get_all_links_for_document(&pool, &document_id).await
    }
    
    get_links(document_id).await.into()
}

/// Update a document link
#[tauri::command]
pub async fn update_document_link(request: UpdateDocumentLinkRequest) -> CommandResponse<()> {
    async fn update(request: UpdateDocumentLinkRequest) -> Result<()> {
        let pool = get_pool()?;
        
        // Get existing link
        let mut link = DocumentLinkOps::get_by_id(&pool, &request.id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::DocumentLinkNotFound { id: request.id.clone() })?;
        
        // Update fields if provided
        if let Some(from_document_id) = request.from_document_id {
            link.from_document_id = from_document_id;
        }
        if let Some(to_document_id) = request.to_document_id {
            link.to_document_id = to_document_id;
        }
        if let Some(link_order) = request.link_order {
            link.link_order = link_order;
        }
        
        DocumentLinkOps::update(&pool, &link).await
    }
    
    update(request).await.into()
}

/// Delete a document link
#[tauri::command]
pub async fn delete_document_link(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        DocumentLinkOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Delete all links for a document (both incoming and outgoing)
#[tauri::command]
pub async fn delete_all_links_for_document(document_id: String) -> CommandResponse<()> {
    async fn delete_links(document_id: String) -> Result<()> {
        let pool = get_pool()?;
        DocumentLinkOps::delete_all_links_for_document(&pool, &document_id).await
    }
    
    delete_links(document_id).await.into()
}

/// Get linked documents with details
#[tauri::command]
pub async fn get_linked_documents(document_id: String) -> CommandResponse<LinkedDocuments> {
    async fn get_documents(document_id: String) -> Result<LinkedDocuments> {
        let pool = get_pool()?;
        DocumentLinkOps::get_linked_documents(&pool, &document_id).await
    }
    
    get_documents(document_id).await.into()
}
