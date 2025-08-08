use crate::database::models::{DeletedItem, DeletedItemType};
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// DeletedItem operations
impl super::DeletedItemOps {
    /// Create a new deleted item record
    pub async fn create(pool: &Pool<Sqlite>, mut deleted_item: DeletedItem) -> Result<DeletedItem> {
        deleted_item.id = Uuid::new_v4().to_string();
        deleted_item.deleted_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO deleted_items (
                id, item_type, item_id, item_data, parent_id, 
                deletion_reason, deleted_at, can_restore
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&deleted_item.id)
        .bind(&deleted_item.item_type)
        .bind(&deleted_item.item_id)
        .bind(&deleted_item.item_data)
        .bind(&deleted_item.parent_id)
        .bind(&deleted_item.deletion_reason)
        .bind(deleted_item.deleted_at)
        .bind(deleted_item.can_restore)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create deleted item: {}", e)))?;
        
        Ok(deleted_item)
    }
    
    /// Get a deleted item by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<DeletedItem>> {
        let deleted_item = sqlx::query_as::<_, DeletedItem>("SELECT * FROM deleted_items WHERE id = ?")
            .bind(id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get deleted item: {}", e)))?;
        
        Ok(deleted_item)
    }
    
    /// Get all deleted items
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<DeletedItem>> {
        let deleted_items = sqlx::query_as::<_, DeletedItem>(
            "SELECT * FROM deleted_items ORDER BY deleted_at DESC"
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get deleted items: {}", e)))?;
        
        Ok(deleted_items)
    }
    
    /// Get deleted items by type
    pub async fn get_by_type(pool: &Pool<Sqlite>, item_type: DeletedItemType) -> Result<Vec<DeletedItem>> {
        let deleted_items = sqlx::query_as::<_, DeletedItem>(
            "SELECT * FROM deleted_items WHERE item_type = ? ORDER BY deleted_at DESC"
        )
        .bind(item_type)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get deleted items by type: {}", e)))?;
        
        Ok(deleted_items)
    }
    
    /// Get deleted items by parent ID
    pub async fn get_by_parent(pool: &Pool<Sqlite>, parent_id: &str) -> Result<Vec<DeletedItem>> {
        let deleted_items = sqlx::query_as::<_, DeletedItem>(
            "SELECT * FROM deleted_items WHERE parent_id = ? ORDER BY deleted_at DESC"
        )
        .bind(parent_id)
        .fetch_all(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get deleted items by parent: {}", e)))?;
        
        Ok(deleted_items)
    }
    
    /// Update a deleted item
    pub async fn update(pool: &Pool<Sqlite>, deleted_item: &DeletedItem) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE deleted_items 
            SET item_data = ?, parent_id = ?, deletion_reason = ?, can_restore = ?
            WHERE id = ?
            "#,
        )
        .bind(&deleted_item.item_data)
        .bind(&deleted_item.parent_id)
        .bind(&deleted_item.deletion_reason)
        .bind(deleted_item.can_restore)
        .bind(&deleted_item.id)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update deleted item: {}", e)))?;
        
        Ok(())
    }
    
    /// Permanently delete a deleted item
    pub async fn permanently_delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM deleted_items WHERE id = ?")
            .bind(id)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to permanently delete item: {}", e)))?;
        
        Ok(())
    }
    
    /// Empty trash (delete all deleted items)
    pub async fn empty_trash(pool: &Pool<Sqlite>) -> Result<()> {
        sqlx::query("DELETE FROM deleted_items")
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to empty trash: {}", e)))?;
        
        Ok(())
    }
    
    /// Move a project to trash
    pub async fn trash_project(
        pool: &Pool<Sqlite>, 
        project_id: &str, 
        reason: Option<String>
    ) -> Result<DeletedItem> {
        // Start a transaction
        let mut tx = pool.begin().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Get the project data
        let project = sqlx::query!(
            "SELECT * FROM projects WHERE id = ?",
            project_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get project: {}", e)))?
        .ok_or_else(|| StoryWeaverError::ProjectNotFound { id: project_id.to_string() })?;
        
        // Serialize project data to JSON
        let project_data = serde_json::to_string(&project)
            .map_err(|e| StoryWeaverError::Serialization { message: e.to_string() })?;
        
        // Create deleted item record
        let deleted_item = DeletedItem {
            id: Uuid::new_v4().to_string(),
            item_type: DeletedItemType::Project,
            item_id: project_id.to_string(),
            item_data: project_data,
            parent_id: None,
            deletion_reason: reason,
            deleted_at: Utc::now(),
            can_restore: true,
        };
        
        // Insert deleted item record
        sqlx::query(
            r#"
            INSERT INTO deleted_items (
                id, item_type, item_id, item_data, parent_id, 
                deletion_reason, deleted_at, can_restore
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&deleted_item.id)
        .bind(&deleted_item.item_type)
        .bind(&deleted_item.item_id)
        .bind(&deleted_item.item_data)
        .bind(&deleted_item.parent_id)
        .bind(&deleted_item.deletion_reason)
        .bind(deleted_item.deleted_at)
        .bind(deleted_item.can_restore)
        .execute(&mut *tx)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create deleted item: {}", e)))?;
        
        // Delete the project
        sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(project_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete project: {}", e)))?;
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(deleted_item)
    }
    
    /// Move a document to trash
    pub async fn trash_document(
        pool: &Pool<Sqlite>, 
        document_id: &str, 
        reason: Option<String>
    ) -> Result<DeletedItem> {
        // Start a transaction
        let mut tx = pool.begin().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Get the document data
        let document = sqlx::query!(
            "SELECT * FROM documents WHERE id = ?",
            document_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get document: {}", e)))?
        .ok_or_else(|| StoryWeaverError::DocumentNotFound { id: document_id.to_string() })?;
        
        // Serialize document data to JSON
        let document_data = serde_json::to_string(&document)
            .map_err(|e| StoryWeaverError::Serialization { message: e.to_string() })?;
        
        // Create deleted item record
        let deleted_item = DeletedItem {
            id: Uuid::new_v4().to_string(),
            item_type: DeletedItemType::Document,
            item_id: document_id.to_string(),
            item_data: document_data,
            parent_id: Some(document.project_id),
            deletion_reason: reason,
            deleted_at: Utc::now(),
            can_restore: true,
        };
        
        // Insert deleted item record
        sqlx::query(
            r#"
            INSERT INTO deleted_items (
                id, item_type, item_id, item_data, parent_id, 
                deletion_reason, deleted_at, can_restore
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&deleted_item.id)
        .bind(&deleted_item.item_type)
        .bind(&deleted_item.item_id)
        .bind(&deleted_item.item_data)
        .bind(&deleted_item.parent_id)
        .bind(&deleted_item.deletion_reason)
        .bind(deleted_item.deleted_at)
        .bind(deleted_item.can_restore)
        .execute(&mut *tx)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create deleted item: {}", e)))?;
        
        // Delete the document
        sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(document_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete document: {}", e)))?;
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(deleted_item)
    }
    
    /// Restore a deleted item
    pub async fn restore_item(pool: &Pool<Sqlite>, deleted_item_id: &str) -> Result<()> {
        // Start a transaction
        let mut tx = pool.begin().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Get the deleted item
        let deleted_item = Self::get_by_id(&mut *tx, deleted_item_id).await?
            .ok_or_else(|| StoryWeaverError::DeletedItemNotFound { id: deleted_item_id.to_string() })?;
        
        // Restore based on item type
        match deleted_item.item_type {
            DeletedItemType::Project => {
                // Parse project data
                let project: crate::database::models::Project = serde_json::from_str(&deleted_item.item_data)
                    .map_err(|e| StoryWeaverError::Deserialization { message: e.to_string() })?;
                
                // Insert project back into database
                sqlx::query(
                    r#"
                    INSERT INTO projects (
                        id, name, description, genre, target_word_count, current_word_count,
                        status, created_at, updated_at, settings, series_id, folder_id
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
                .bind(&project.series_id)
                .bind(&project.folder_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to restore project: {}", e)))?;
            },
            DeletedItemType::Document => {
                // Parse document data
                let document: crate::database::models::Document = serde_json::from_str(&deleted_item.item_data)
                    .map_err(|e| StoryWeaverError::Deserialization { message: e.to_string() })?;
                
                // Insert document back into database
                sqlx::query(
                    r#"
                    INSERT INTO documents (
                        id, project_id, title, content, document_type, order_index,
                        word_count, parent_id, created_at, updated_at, metadata, folder_id
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
                .bind(&document.folder_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to restore document: {}", e)))?;
            },
            DeletedItemType::Folder => {
                // Parse folder data
                let folder: crate::database::models::Folder = serde_json::from_str(&deleted_item.item_data)
                    .map_err(|e| StoryWeaverError::Deserialization { message: e.to_string() })?;
                
                // Insert folder back into database
                sqlx::query(
                    r#"
                    INSERT INTO folders (
                        id, name, parent_folder_id, is_series, created_at
                    )
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                )
                .bind(&folder.id)
                .bind(&folder.name)
                .bind(&folder.parent_folder_id)
                .bind(folder.is_series)
                .bind(folder.created_at)
                .execute(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to restore folder: {}", e)))?;
            },
            DeletedItemType::Series => {
                // Parse series data
                let series: crate::database::models::Series = serde_json::from_str(&deleted_item.item_data)
                    .map_err(|e| StoryWeaverError::Deserialization { message: e.to_string() })?;
                
                // Insert series back into database
                sqlx::query(
                    r#"
                    INSERT INTO series (
                        id, name, description, folder_id, created_at
                    )
                    VALUES (?, ?, ?, ?, ?)
                    "#,
                )
                .bind(&series.id)
                .bind(&series.name)
                .bind(&series.description)
                .bind(&series.folder_id)
                .bind(series.created_at)
                .execute(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to restore series: {}", e)))?;
            },
            _ => {
                return Err(StoryWeaverError::UnsupportedOperation { 
                    message: format!("Restore not implemented for item type: {:?}", deleted_item.item_type) 
                });
            }
        }
        
        // Delete the deleted item record
        sqlx::query("DELETE FROM deleted_items WHERE id = ?")
            .bind(deleted_item_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete deleted item record: {}", e)))?;
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(())
    }
}
