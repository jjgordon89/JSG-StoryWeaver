use crate::commands::CommandResponse;
use crate::database::get_pool;
use crate::database::models::{DeletedItem, DeletedItemType};
use crate::database::operations::DeletedItemOps;
use crate::error::Result;
use crate::security::rate_limit::{rl_update, rl_delete, rl_list};

/// Get all deleted items (trash)
#[tauri::command]
pub async fn get_trash_items() -> CommandResponse<Vec<DeletedItem>> {
    async fn get() -> Result<Vec<DeletedItem>> {
        // Rate limiting
        rl_list("trash_items", None)?;
        let pool = get_pool()?;
        DeletedItemOps::get_all(&pool).await
    }
    
    get().await.into()
}

/// Get deleted items by type
#[tauri::command]
pub async fn get_trash_items_by_type(item_type: DeletedItemType) -> CommandResponse<Vec<DeletedItem>> {
    async fn get(item_type: DeletedItemType) -> Result<Vec<DeletedItem>> {
        // Rate limiting
        rl_list("trash_items", None)?;
        let pool = get_pool()?;
        DeletedItemOps::get_by_type(&pool, item_type).await
    }
    
    get(item_type).await.into()
}

/// Get deleted items by parent ID
#[tauri::command]
pub async fn get_trash_items_by_parent(parent_id: String) -> CommandResponse<Vec<DeletedItem>> {
    async fn get(parent_id: String) -> Result<Vec<DeletedItem>> {
        // Rate limiting
        rl_list("trash_items", Some(&parent_id))?;
        let pool = get_pool()?;
        DeletedItemOps::get_by_parent(&pool, &parent_id).await
    }
    
    get(parent_id).await.into()
}

/// Move a project to trash
#[tauri::command]
pub async fn trash_project(project_id: String, reason: Option<String>) -> CommandResponse<DeletedItem> {
    async fn trash(project_id: String, reason: Option<String>) -> Result<DeletedItem> {
        // Rate limiting
        rl_delete("project", Some(&project_id))?;
        let pool = get_pool()?;
        DeletedItemOps::trash_project(&pool, &project_id, reason).await
    }
    
    trash(project_id, reason).await.into()
}

/// Move a document to trash
#[tauri::command]
pub async fn trash_document(document_id: String, reason: Option<String>) -> CommandResponse<DeletedItem> {
    async fn trash(document_id: String, reason: Option<String>) -> Result<DeletedItem> {
        // Rate limiting
        rl_delete("document", Some(&document_id))?;
        let pool = get_pool()?;
        DeletedItemOps::trash_document(&pool, &document_id, reason).await
    }
    
    trash(document_id, reason).await.into()
}

/// Restore a deleted item
#[tauri::command]
pub async fn restore_trash_item(deleted_item_id: String) -> CommandResponse<()> {
    async fn restore(deleted_item_id: String) -> Result<()> {
        // Rate limiting
        rl_update("trash_item", Some(&deleted_item_id))?;
        let pool = get_pool()?;
        DeletedItemOps::restore_item(&pool, &deleted_item_id).await
    }
    
    restore(deleted_item_id).await.into()
}

/// Permanently delete a deleted item
#[tauri::command]
pub async fn permanently_delete_trash_item(deleted_item_id: String) -> CommandResponse<()> {
    async fn delete(deleted_item_id: String) -> Result<()> {
        // Rate limiting
        rl_delete("trash_item", Some(&deleted_item_id))?;
        let pool = get_pool()?;
        DeletedItemOps::permanently_delete(&pool, &deleted_item_id).await
    }
    
    delete(deleted_item_id).await.into()
}

/// Empty the trash (permanently delete all items)
#[tauri::command]
pub async fn empty_trash() -> CommandResponse<()> {
    async fn empty() -> Result<()> {
        // Rate limiting
        rl_delete("trash_items", None)?;
        let pool = get_pool()?;
        DeletedItemOps::empty_trash(&pool).await
    }
    
    empty().await.into()
}
