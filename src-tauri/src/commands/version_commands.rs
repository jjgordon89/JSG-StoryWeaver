use crate::commands::CommandResponse;
use crate::database::get_pool;
use crate::database::models::DocumentVersion;
use crate::database::operations::{DocumentVersionOps, VersionHistoryItem};
use crate::error::Result;
use crate::security::validation::*;
use crate::security::rate_limit::{rl_create, rl_update, rl_delete, rl_list};

/// Create a new document version
#[tauri::command]
pub async fn create_document_version(
    document_id: String,
    created_by: Option<String>,
    comment: Option<String>,
) -> CommandResponse<DocumentVersion> {
    async fn create(
        document_id: String,
        created_by: Option<String>,
        comment: Option<String>,
    ) -> Result<DocumentVersion> {
        // Rate limiting
        rl_create("document_version", Some(&document_id))?;
        // Input validation
        validate_security_input(&document_id)?;
        
        if let Some(ref created_by_val) = created_by {
            validate_security_input(created_by_val)?;
        }
        
        if let Some(ref comment_val) = comment {
            validate_content_length(comment_val, 1000)?;
            validate_security_input(comment_val)?;
        }
        
        let pool = get_pool()?;
        DocumentVersionOps::create_from_document(&pool, &document_id, created_by, comment).await
    }
    
    create(document_id, created_by, comment).await.into()
}

/// Get versions for a document
#[tauri::command]
pub async fn get_document_versions(document_id: String) -> CommandResponse<Vec<DocumentVersion>> {
    async fn get(document_id: String) -> Result<Vec<DocumentVersion>> {
        // Rate limiting
        rl_list("document_version", Some(&document_id))?;
        // Input validation
        validate_security_input(&document_id)?;
        
        let pool = get_pool()?;
        DocumentVersionOps::get_versions(&pool, &document_id).await
    }
    
    get(document_id).await.into()
}

/// Get version history with metadata
#[tauri::command]
pub async fn get_version_history(document_id: String) -> CommandResponse<Vec<VersionHistoryItem>> {
    async fn get(document_id: String) -> Result<Vec<VersionHistoryItem>> {
        // Rate limiting
        rl_list("version_history", Some(&document_id))?;
        // Input validation
        validate_security_input(&document_id)?;
        
        let pool = get_pool()?;
        DocumentVersionOps::get_version_history(&pool, &document_id).await
    }
    
    get(document_id).await.into()
}

/// Get a specific document version
#[tauri::command]
pub async fn get_document_version(version_id: String) -> CommandResponse<Option<DocumentVersion>> {
    async fn get(version_id: String) -> Result<Option<DocumentVersion>> {
        // Rate limiting
        rl_list("document_version", Some(&version_id))?;
        // Input validation
        validate_security_input(&version_id)?;
        let pool = get_pool()?;
        DocumentVersionOps::get_by_id(&pool, &version_id).await
    }
    
    get(version_id).await.into()
}

/// Get latest version for a document
#[tauri::command]
pub async fn get_latest_document_version(document_id: String) -> CommandResponse<Option<DocumentVersion>> {
    async fn get(document_id: String) -> Result<Option<DocumentVersion>> {
        // Rate limiting
        rl_list("document_version", Some(&document_id))?;
        // Input validation
        validate_security_input(&document_id)?;
        let pool = get_pool()?;
        DocumentVersionOps::get_latest_version(&pool, &document_id).await
    }
    
    get(document_id).await.into()
}

/// Restore a document to a specific version
#[tauri::command]
pub async fn restore_document_version(version_id: String) -> CommandResponse<()> {
    async fn restore(version_id: String) -> Result<()> {
        // Rate limiting
        rl_update("document_version", Some(&version_id))?;
        // Input validation
        validate_security_input(&version_id)?;
        let pool = get_pool()?;
        DocumentVersionOps::restore_version(&pool, &version_id).await
    }
    
    restore(version_id).await.into()
}

/// Delete a specific document version
#[tauri::command]
pub async fn delete_document_version(version_id: String) -> CommandResponse<()> {
    async fn delete(version_id: String) -> Result<()> {
        // Rate limiting
        rl_delete("document_version", Some(&version_id))?;
        // Input validation
        validate_security_input(&version_id)?;
        let pool = get_pool()?;
        DocumentVersionOps::delete(&pool, &version_id).await
    }
    
    delete(version_id).await.into()
}

/// Delete all versions for a document
#[tauri::command]
pub async fn delete_all_document_versions(document_id: String) -> CommandResponse<()> {
    async fn delete_all(document_id: String) -> Result<()> {
        // Rate limiting
        rl_delete("document_version", Some(&document_id))?;
        // Input validation
        validate_security_input(&document_id)?;
        let pool = get_pool()?;
        DocumentVersionOps::delete_all_versions(&pool, &document_id).await
    }
    
    delete_all(document_id).await.into()
}
