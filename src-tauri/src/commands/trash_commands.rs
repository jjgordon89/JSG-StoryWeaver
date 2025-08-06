use crate::database::get_pool;
use crate::database::models::{DeletedItem, DeletedItemType};
use crate::database::operations::DeletedItemOps;
use crate::error::Result;

/// Get all deleted items (trash)
#[tauri::command]
pub async fn get_trash_items() -> Result<Vec<DeletedItem>> {
    let pool = get_pool()?;
    DeletedItemOps::get_all(&pool).await
}

/// Get deleted items by type
#[tauri::command]
pub async fn get_trash_items_by_type(item_type: DeletedItemType) -> Result<Vec<DeletedItem>> {
    let pool = get_pool()?;
    DeletedItemOps::get_by_type(&pool, item_type).await
}

/// Get deleted items by parent ID
#[tauri::command]
pub async fn get_trash_items_by_parent(parent_id: String) -> Result<Vec<DeletedItem>> {
    let pool = get_pool()?;
    DeletedItemOps::get_by_parent(&pool, &parent_id).await
}

/// Move a project to trash
#[tauri::command]
pub async fn trash_project(project_id: String, reason: Option<String>) -> Result<DeletedItem> {
    let pool = get_pool()?;
    DeletedItemOps::trash_project(&pool, &project_id, reason).await
}

/// Move a document to trash
#[tauri::command]
pub async fn trash_document(document_id: String, reason: Option<String>) -> Result<DeletedItem> {
    let pool = get_pool()?;
    DeletedItemOps::trash_document(&pool, &document_id, reason).await
}

/// Restore a deleted item
#[tauri::command]
pub async fn restore_trash_item(deleted_item_id: String) -> Result<()> {
    let pool = get_pool()?;
    DeletedItemOps::restore_item(&pool, &deleted_item_id).await
}

/// Permanently delete a deleted item
#[tauri::command]
pub async fn permanently_delete_trash_item(deleted_item_id: String) -> Result<()> {
    let pool = get_pool()?;
    DeletedItemOps::permanently_delete(&pool, &deleted_item_id).await
}

/// Empty trash (delete all deleted items)
#[tauri::command]
pub async fn empty_trash() -> Result<()> {
    let pool = get_pool()?;
    DeletedItemOps::empty_trash(&pool).await
}
