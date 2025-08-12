//! Document command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::DocumentOps};
use crate::error::Result;
use crate::security::validation::{
    validate_document_name, validate_content_length, validate_security_input
};
use serde::{Deserialize, Serialize};

/// Create document request
#[derive(Debug, Deserialize)]
pub struct CreateDocumentRequest {
    pub project_id: String,
    pub title: String,
    pub content: Option<String>,
    pub document_type: DocumentType,
    pub order_index: Option<i32>,
    pub parent_id: Option<String>,
}

/// Update document request
#[derive(Debug, Deserialize)]
pub struct UpdateDocumentRequest {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub document_type: Option<DocumentType>,
    pub order_index: Option<i32>,
    pub parent_id: Option<String>,
    pub metadata: Option<String>,
}

/// Search documents request
#[derive(Debug, Deserialize)]
pub struct SearchDocumentsRequest {
    pub project_id: String,
    pub query: String,
}

/// Create a new document
#[tauri::command]
pub async fn create_document(request: CreateDocumentRequest) -> CommandResponse<Document> {
    async fn create(request: CreateDocumentRequest) -> Result<Document> {
        // Input validation
        validate_security_input(&request.project_id)?;
        validate_document_name(&request.title)?;
        
        if let Some(ref content) = request.content {
            validate_content_length(content, 1_000_000)?; // 1MB limit for document content
            validate_security_input(content)?;
        }
        
        if let Some(order_index) = request.order_index {
            if order_index < 0 || order_index > 10000 {
                return Err(crate::error::StoryWeaverError::ValidationError {
                    message: "Order index must be between 0 and 10,000".to_string()
                });
            }
        }
        
        if let Some(ref parent_id) = request.parent_id {
            validate_security_input(parent_id)?;
        }
        
        let pool = get_pool()?;
        
        let mut document = Document::new(
            request.project_id,
            request.title,
            request.document_type,
        );
        
        // Set optional fields
        if let Some(content) = request.content {
            document.content = content;
        }
        if let Some(order_index) = request.order_index {
            document.order_index = order_index;
        }
        document.parent_id = request.parent_id;
        
        DocumentOps::create(&pool, document).await
    }
    
    create(request).await.into()
}

/// Get documents by project ID
#[tauri::command]
pub async fn get_documents(project_id: String) -> CommandResponse<Vec<Document>> {
    async fn get_by_project(project_id: String) -> Result<Vec<Document>> {
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        DocumentOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get a document by ID
#[tauri::command]
pub async fn get_document(id: String) -> CommandResponse<Option<Document>> {
    async fn get(id: String) -> Result<Option<Document>> {
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        DocumentOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Update a document
#[tauri::command]
pub async fn update_document(request: UpdateDocumentRequest) -> CommandResponse<()> {
    async fn update(request: UpdateDocumentRequest) -> Result<()> {
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref title) = request.title {
            validate_document_name(title)?;
        }
        
        if let Some(ref content) = request.content {
            validate_content_length(content, 1_000_000)?; // 1MB limit
            validate_security_input(content)?;
        }
        
        if let Some(order_index) = request.order_index {
            if order_index < 0 || order_index > 10000 {
                return Err(crate::error::StoryWeaverError::ValidationError {
                    message: "Order index must be between 0 and 10,000".to_string()
                });
            }
        }
        
        if let Some(ref parent_id) = request.parent_id {
            validate_security_input(parent_id)?;
        }
        
        if let Some(ref metadata) = request.metadata {
            validate_content_length(metadata, 50000)?; // 50KB limit for metadata
            validate_security_input(metadata)?;
        }
        
        let pool = get_pool()?;
        
        // Get existing document
        let mut document = DocumentOps::get_by_id(&pool, &request.id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::DocumentNotFound { id: request.id.to_string() })?;
        
        // Update fields if provided
        if let Some(title) = request.title {
            document.title = title;
        }
        if let Some(content) = request.content {
            document.content = content;
        }
        if let Some(document_type) = request.document_type {
            document.document_type = document_type;
        }
        if let Some(order_index) = request.order_index {
            document.order_index = order_index;
        }
        if let Some(parent_id) = request.parent_id {
            document.parent_id = Some(parent_id);
        }
        if let Some(metadata) = request.metadata {
            document.metadata = metadata;
        }
        
        DocumentOps::update(&pool, &document).await
    }
    
    update(request).await.into()
}

/// Save document content (simplified update for frequent saves)
#[tauri::command]
pub async fn save_document(id: String, content: String) -> CommandResponse<()> {
    async fn save(id: String, content: String) -> Result<()> {
        // Input validation
        validate_security_input(&id)?;
        validate_content_length(&content, 1_000_000)?; // 1MB limit
        validate_security_input(&content)?;
        
        let pool = get_pool()?;
        
        // Get existing document
        let mut document = DocumentOps::get_by_id(&pool, &id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::DocumentNotFound { id: id.to_string() })?;
        
        // Update content
        document.content = content;
        
        DocumentOps::update(&pool, &document).await
    }
    
    save(id, content).await.into()
}

/// Delete a document
#[tauri::command]
pub async fn delete_document(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        DocumentOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Search documents using full-text search
#[tauri::command]
pub async fn search_documents(request: SearchDocumentsRequest) -> CommandResponse<Vec<Document>> {
    async fn search(request: SearchDocumentsRequest) -> Result<Vec<Document>> {
        // Input validation
        validate_security_input(&request.project_id)?;
        validate_content_length(&request.query, 1000)?; // Limit search query length
        validate_security_input(&request.query)?;
        
        if request.query.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Search query cannot be empty".to_string()
            });
        }
        
        let pool = get_pool()?;
        DocumentOps::search(&pool, &request.project_id, &request.query).await
    }
    
    search(request).await.into()
}

/// Document tree structure for hierarchical display
#[derive(Debug, Serialize)]
pub struct DocumentTree {
    pub document: Document,
    pub children: Vec<DocumentTree>,
}

/// Get documents as a tree structure
#[tauri::command]
pub async fn get_document_tree(project_id: String) -> CommandResponse<Vec<DocumentTree>> {
    async fn get_tree(project_id: String) -> Result<Vec<DocumentTree>> {
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        let documents = DocumentOps::get_by_project(&pool, &project_id).await?;
        
        // Build tree structure
        let mut tree: Vec<DocumentTree> = Vec::new();
        let mut document_map = std::collections::HashMap::new();
        
        // First pass: create map of all documents
        for doc in documents {
            document_map.insert(doc.id.clone(), DocumentTree {
                document: doc,
                children: Vec::new(),
            });
        }
        
        // Second pass: build tree structure
        let mut root_docs = Vec::new();
        let doc_ids: Vec<String> = document_map.keys().cloned().collect();
        
        for doc_id in doc_ids {
            if let Some(doc_tree) = document_map.remove(&doc_id) {
                if let Some(parent_id) = &doc_tree.document.parent_id {
                    // This is a child document
                    if let Some(parent) = document_map.get_mut(parent_id) {
                        parent.children.push(doc_tree);
                    } else {
                        // Parent not found, treat as root
                        root_docs.push(doc_tree);
                    }
                } else {
                    // This is a root document
                    root_docs.push(doc_tree);
                }
            }
        }
        
        // Sort by order_index and title
        root_docs.sort_by(|a, b| {
            a.document.order_index.cmp(&b.document.order_index)
                .then_with(|| a.document.title.cmp(&b.document.title))
        });
        
        // Recursively sort children
        fn sort_children(tree: &mut DocumentTree) {
            tree.children.sort_by(|a, b| {
                a.document.order_index.cmp(&b.document.order_index)
                    .then_with(|| a.document.title.cmp(&b.document.title))
            });
            
            for child in &mut tree.children {
                sort_children(child);
            }
        }
        
        for tree in &mut root_docs {
            sort_children(tree);
        }
        
        Ok(root_docs)
    }
    
    get_tree(project_id).await.into()
}

/// Document statistics
#[derive(Debug, Serialize)]
pub struct DocumentStats {
    pub total_documents: i32,
    pub total_word_count: i32,
    pub by_type: std::collections::HashMap<String, i32>,
    pub recent_documents: Vec<Document>,
}

/// Get document statistics for a project
#[tauri::command]
pub async fn get_document_stats(project_id: String) -> CommandResponse<DocumentStats> {
    async fn get_stats(project_id: String) -> Result<DocumentStats> {
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        
        // Get all documents for the project
        let documents = DocumentOps::get_by_project(&pool, &project_id).await?;
        
        let total_documents = documents.len() as i32;
        let total_word_count = documents.iter().map(|d| d.word_count).sum();
        
        // Count by type
        let mut by_type = std::collections::HashMap::new();
        for doc in &documents {
            let type_str = format!("{:?}", doc.document_type);
            *by_type.entry(type_str).or_insert(0) += 1;
        }
        
        // Get recent documents (last 5)
        let mut recent_documents = documents;
        recent_documents.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        recent_documents.truncate(5);
        
        Ok(DocumentStats {
            total_documents,
            total_word_count,
            by_type,
            recent_documents,
        })
    }
    
    get_stats(project_id).await.into()
}
