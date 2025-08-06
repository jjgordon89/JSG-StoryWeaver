use crate::database::get_pool;
use crate::database::models::DocumentVersion;
use crate::database::operations::{DocumentVersionOps, VersionHistoryItem};
use crate::error::Result;

/// Create a new document version
#[tauri::command]
pub async fn create_document_version(
    document_id: String,
    created_by: Option<String>,
    comment: Option<String>,
) -> Result<DocumentVersion> {
    let pool = get_pool()?;
    DocumentVersionOps::create_from_document(&pool, &document_id, created_by, comment).await
}

/// Get versions for a document
#[tauri::command]
pub async fn get_document_versions(document_id: String) -> Result<Vec<DocumentVersion>> {
    let pool = get_pool()?;
    DocumentVersionOps::get_versions(&pool, &document_id).await
}

/// Get version history with metadata
#[tauri::command]
pub async fn get_version_history(document_id: String) -> Result<Vec<VersionHistoryItem>> {
    let pool = get_pool()?;
    DocumentVersionOps::get_version_history(&pool, &document_id).await
}

/// Get a specific document version
#[tauri::command]
pub async fn get_document_version(version_id: String) -> Result<Option<DocumentVersion>> {
    let pool = get_pool()?;
    DocumentVersionOps::get_by_id(&pool, &version_id).await
}

/// Get latest version for a document
#[tauri::command]
pub async fn get_latest_document_version(document_id: String) -> Result<Option<DocumentVersion>> {
    let pool = get_pool()?;
    DocumentVersionOps::get_latest_version(&pool, &document_id).await
}

/// Restore a document to a specific version
#[tauri::command]
pub async fn restore_document_version(version_id: String) -> Result<()> {
    let pool = get_pool()?;
    DocumentVersionOps::restore_version(&pool, &version_id).await
}

/// Delete a document version
#[tauri::command]
pub async fn delete_document_version(version_id: String) -> Result<()> {
    let pool = get_pool()?;
    DocumentVersionOps::delete(&pool, &version_id).await
}

/// Delete all versions for a document
#[tauri::command]
pub async fn delete_all_document_versions(document_id: String) -> Result<()> {
    let pool = get_pool()?;
    DocumentVersionOps::delete_all_versions(&pool, &document_id).await
}
