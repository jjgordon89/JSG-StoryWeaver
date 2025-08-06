use crate::database::models::Folder;
use crate::error::{Result, StoryWeaverError};
use chrono::Utc;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Folder operations
pub struct FolderOps;

impl FolderOps {
    /// Create a new folder
    pub async fn create(pool: &Pool<Sqlite>, mut folder: Folder) -> Result<Folder> {
        folder.id = Uuid::new_v4().to_string();
        folder.created_at = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO folders (id, name, parent_folder_id, is_series, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&folder.id)
        .bind(&folder.name)
        .bind(&folder.parent_folder_id)
        .bind(folder.is_series)
        .bind(folder.created_at)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create folder: {}", e)))?;
        
        Ok(folder)
    }
    
    /// Get a folder by ID
    pub async fn get_by_id(pool: &Pool<Sqlite>, id: &str) -> Result<Option<Folder>> {
        let folder = sqlx::query_as::<_, Folder>("SELECT * FROM folders WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get folder: {}", e)))?;
        
        Ok(folder)
    }
    
    /// Get all root folders (no parent)
    pub async fn get_root_folders(pool: &Pool<Sqlite>) -> Result<Vec<Folder>> {
        let folders = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE parent_folder_id IS NULL ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get root folders: {}", e)))?;
        
        Ok(folders)
    }
    
    /// Get child folders for a parent folder
    pub async fn get_children(pool: &Pool<Sqlite>, parent_id: &str) -> Result<Vec<Folder>> {
        let folders = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders WHERE parent_folder_id = ? ORDER BY name"
        )
        .bind(parent_id)
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get child folders: {}", e)))?;
        
        Ok(folders)
    }
    
    /// Get all folders
    pub async fn get_all(pool: &Pool<Sqlite>) -> Result<Vec<Folder>> {
        let folders = sqlx::query_as::<_, Folder>(
            "SELECT * FROM folders ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get folders: {}", e)))?;
        
        Ok(folders)
    }
    
    /// Update a folder
    pub async fn update(pool: &Pool<Sqlite>, folder: &Folder) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE folders SET name = ?, parent_folder_id = ?, is_series = ?
            WHERE id = ?
            "#,
        )
        .bind(&folder.name)
        .bind(&folder.parent_folder_id)
        .bind(folder.is_series)
        .bind(&folder.id)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to update folder: {}", e)))?;
        
        Ok(())
    }
    
    /// Delete a folder
    pub async fn delete(pool: &Pool<Sqlite>, id: &str) -> Result<()> {
        // First, check if there are any child folders
        let child_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM folders WHERE parent_folder_id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check child folders: {}", e)))?;
        
        if child_count > 0 {
            return Err(StoryWeaverError::FolderNotEmpty { id: id.to_string() });
        }
        
        // Check if there are any projects in this folder
        let project_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM projects WHERE folder_id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check folder projects: {}", e)))?;
        
        if project_count > 0 {
            return Err(StoryWeaverError::FolderNotEmpty { id: id.to_string() });
        }
        
        // Check if there are any documents in this folder
        let document_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM documents WHERE folder_id = ?"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to check folder documents: {}", e)))?;
        
        if document_count > 0 {
            return Err(StoryWeaverError::FolderNotEmpty { id: id.to_string() });
        }
        
        // If no children, delete the folder
        sqlx::query("DELETE FROM folders WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete folder: {}", e)))?;
        
        Ok(())
    }
    
    /// Move items to a folder
    pub async fn move_items_to_folder(
        pool: &Pool<Sqlite>, 
        folder_id: &str, 
        project_ids: &[String], 
        document_ids: &[String]
    ) -> Result<()> {
        // Start a transaction
        let mut tx = pool.begin().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to start transaction: {}", e)))?;
        
        // Update projects
        for project_id in project_ids {
            sqlx::query("UPDATE projects SET folder_id = ? WHERE id = ?")
                .bind(folder_id)
                .bind(project_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to move project to folder: {}", e)))?;
        }
        
        // Update documents
        for document_id in document_ids {
            sqlx::query("UPDATE documents SET folder_id = ? WHERE id = ?")
                .bind(folder_id)
                .bind(document_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to move document to folder: {}", e)))?;
        }
        
        // Commit the transaction
        tx.commit().await
            .map_err(|e| StoryWeaverError::database(format!("Failed to commit transaction: {}", e)))?;
        
        Ok(())
    }
    
    /// Get folder hierarchy as a tree
    pub async fn get_folder_tree(pool: &Pool<Sqlite>) -> Result<Vec<FolderTreeNode>> {
        // Get all folders
        let folders = Self::get_all(pool).await?;
        
        // Build tree structure
        let mut tree: Vec<FolderTreeNode> = Vec::new();
        let mut folder_map = std::collections::HashMap::new();
        
        // First pass: create map of all folders
        for folder in &folders {
            folder_map.insert(folder.id.clone(), FolderTreeNode {
                folder: folder.clone(),
                children: Vec::new(),
            });
        }
        
        // Second pass: build hierarchy
        let mut root_folders = Vec::new();
        let folder_ids: Vec<String> = folder_map.keys().cloned().collect();
        
        for folder_id in folder_ids {
            if let Some(folder_tree) = folder_map.remove(&folder_id) {
                if let Some(parent_id) = &folder_tree.folder.parent_folder_id {
                    // This is a child folder
                    if let Some(parent) = folder_map.get_mut(parent_id) {
                        parent.children.push(folder_tree);
                    } else {
                        // Parent not found, treat as root
                        root_folders.push(folder_tree);
                    }
                } else {
                    // This is a root folder
                    root_folders.push(folder_tree);
                }
            }
        }
        
        // Sort folders by name
        root_folders.sort_by(|a, b| a.folder.name.cmp(&b.folder.name));
        
        // Recursively sort children
        fn sort_children(tree: &mut FolderTreeNode) {
            tree.children.sort_by(|a, b| a.folder.name.cmp(&b.folder.name));
            
            for child in &mut tree.children {
                sort_children(child);
            }
        }
        
        for tree in &mut root_folders {
            sort_children(tree);
        }
        
        Ok(root_folders)
    }
}

/// Folder tree node for hierarchical display
#[derive(Debug, Clone, serde::Serialize)]
pub struct FolderTreeNode {
    pub folder: Folder,
    pub children: Vec<FolderTreeNode>,
}
