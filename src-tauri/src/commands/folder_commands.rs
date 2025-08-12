//! Folder command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::*};
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Create folder request
#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_folder_id: Option<String>,
    pub is_series: Option<bool>,
}

/// Update folder request
#[derive(Debug, Deserialize)]
pub struct UpdateFolderRequest {
    pub id: String,
    pub name: Option<String>,
    pub parent_folder_id: Option<String>,
    pub is_series: Option<bool>,
}

/// Move items to folder request
#[derive(Debug, Deserialize)]
pub struct MoveItemsToFolderRequest {
    pub folder_id: String,
    pub project_ids: Vec<String>,
    pub document_ids: Vec<String>,
}

/// Create a new folder
#[tauri::command]
pub async fn create_folder(request: CreateFolderRequest) -> CommandResponse<Folder> {
    async fn create(request: CreateFolderRequest) -> Result<Folder> {
        // Input validation
        if request.name.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::validation("Folder name cannot be empty".to_string()));
        }
        if request.name.len() > 255 {
            return Err(crate::error::StoryWeaverError::validation("Folder name too long (max 255 characters)".to_string()));
        }
        crate::security::validate_security_input(&request.name)?;
        if let Some(ref parent_id) = request.parent_folder_id {
            crate::security::validate_security_input(parent_id)?;
        }

        let pool = get_pool()?;
        
        let folder = Folder {
            id: String::new(), // Will be set by the create function
            name: request.name,
            parent_folder_id: request.parent_folder_id,
            is_series: request.is_series.unwrap_or(false),
            created_at: chrono::Utc::now(),
        };
        
        FolderOps::create(&pool, folder).await
    }
    
    create(request).await.into()
}

/// Get a folder by ID
#[tauri::command]
pub async fn get_folder(id: String) -> CommandResponse<Option<Folder>> {
    async fn get(id: String) -> Result<Option<Folder>> {
        // Input validation
        crate::security::validate_security_input(&id)?;

        let pool = get_pool()?;
        FolderOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Get all root folders
#[tauri::command]
pub async fn get_root_folders() -> CommandResponse<Vec<Folder>> {
    async fn get_roots() -> Result<Vec<Folder>> {
        let pool = get_pool()?;
        FolderOps::get_root_folders(&pool).await
    }
    
    get_roots().await.into()
}

/// Get child folders for a parent/// Get child folders
#[tauri::command]
pub async fn get_child_folders(parent_id: String) -> CommandResponse<Vec<Folder>> {
    async fn get_children(parent_id: String) -> Result<Vec<Folder>> {
        // Input validation
        crate::security::validate_security_input(&parent_id)?;

        let pool = get_pool()?;
        FolderOps::get_children(&pool, &parent_id).await
    }
    
    get_children(parent_id).await.into()
}

/// Get all folders
#[tauri::command]
pub async fn get_all_folders() -> CommandResponse<Vec<Folder>> {
    async fn get_all() -> Result<Vec<Folder>> {
        let pool = get_pool()?;
        FolderOps::get_all(&pool).await
    }
    
    get_all().await.into()
}

/// Update a folder
#[tauri::command]
pub async fn update_folder(request: UpdateFolderRequest) -> CommandResponse<()> {
    async fn update(request: UpdateFolderRequest) -> Result<()> {
        // Input validation
        crate::security::validate_security_input(&request.id)?;
        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(crate::error::StoryWeaverError::validation("Folder name cannot be empty".to_string()));
            }
            if name.len() > 255 {
                return Err(crate::error::StoryWeaverError::validation("Folder name too long (max 255 characters)".to_string()));
            }
            crate::security::validate_security_input(name)?;
        }
        if let Some(ref parent_id) = request.parent_folder_id {
            crate::security::validate_security_input(parent_id)?;
        }

        let pool = get_pool()?;
        
        // Get existing folder
        let mut folder = FolderOps::get_by_id(&pool, &request.id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::FolderNotFound { id: request.id.clone() })?;
        
        // Update fields if provided
        if let Some(name) = request.name {
            folder.name = name;
        }
        if let Some(parent_folder_id) = request.parent_folder_id {
            folder.parent_folder_id = Some(parent_folder_id);
        }
        if let Some(is_series) = request.is_series {
            folder.is_series = is_series;
        }
        
        FolderOps::update(&pool, &folder).await
    }
    
    update(request).await.into()
}

/// Delete a folder
#[tauri::command]
pub async fn delete_folder(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Input validation
        if !crate::security::is_safe_input(&id) {
            return Err(crate::error::StoryWeaverError::validation("Invalid folder id".to_string()));
        }

        let pool = get_pool()?;
        FolderOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Move items to folder
#[tauri::command]
pub async fn move_items_to_folder(request: MoveItemsToFolderRequest) -> CommandResponse<()> {
    async fn move_items(request: MoveItemsToFolderRequest) -> Result<()> {
        // Input validation
        crate::security::validate_security_input(&request.folder_id)?;
        for project_id in &request.project_ids {
            crate::security::validate_security_input(project_id)?;
        }
        for document_id in &request.document_ids {
            crate::security::validate_security_input(document_id)?;
        }

        let pool = get_pool()?;
        FolderOps::move_items_to_folder(
            &pool, 
            &request.folder_id, 
            &request.project_ids, 
            &request.document_ids
        ).await
    }
    
    move_items(request).await.into()
}

/// Get folder hierarchy as a tree
#[tauri::command]
pub async fn get_folder_tree() -> CommandResponse<Vec<FolderTreeNode>> {
    async fn get_tree() -> Result<Vec<FolderTreeNode>> {
        let pool = get_pool()?;
        FolderOps::get_folder_tree(&pool).await
    }
    
    get_tree().await.into()
}
