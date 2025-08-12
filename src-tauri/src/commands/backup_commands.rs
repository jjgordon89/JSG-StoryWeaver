use crate::database::backup::{BackupManager, BackupInfo};
use crate::error::Result;
use tauri::AppHandle;

/// Create a backup of the database
#[tauri::command]
pub async fn create_backup(
    app_handle: AppHandle,
    backup_name: Option<String>,
) -> Result<String> {
    // Input validation
    if let Some(ref name) = backup_name {
        crate::security::validation::validate_content_length(name, 255)?;
        crate::security::validation::validate_security_input(name)?;
        
        if name.trim().is_empty() {
            return Err(crate::error::StoryWeaverError::ValidationError {
                message: "Backup name cannot be empty".to_string(),
            });
        }
    }
    let backup_path = BackupManager::create_backup(&app_handle, backup_name).await?;
    Ok(backup_path.to_string_lossy().into_owned())
}

/// Restore from a backup
#[tauri::command]
pub async fn restore_from_backup(
    app_handle: AppHandle,
    backup_filename: String,
) -> Result<()> {
    // Input validation
    crate::security::validation::validate_content_length(&backup_filename, 255)?;
    crate::security::validation::validate_security_input(&backup_filename)?;
    
    if backup_filename.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::ValidationError {
            message: "Backup filename cannot be empty".to_string(),
        });
    }
    BackupManager::restore_from_backup(&app_handle, &backup_filename).await
}

/// Get list of available backups
#[tauri::command]
pub async fn get_backups(app_handle: AppHandle) -> Result<Vec<BackupInfo>> {
    BackupManager::get_backups(&app_handle).await
}

/// Delete a backup
#[tauri::command]
pub async fn delete_backup(app_handle: AppHandle, backup_id: String) -> Result<()> {
    // Input validation
    crate::security::validation::validate_security_input(&backup_id)?;
    
    if backup_id.trim().is_empty() {
        return Err(crate::error::StoryWeaverError::ValidationError {
            message: "Backup ID cannot be empty".to_string(),
        });
    }
    BackupManager::delete_backup(&app_handle, &backup_id).await
}

/// Create an automatic backup
#[tauri::command]
pub async fn create_auto_backup(app_handle: AppHandle) -> Result<()> {
    BackupManager::create_auto_backup(&app_handle).await
}

/// Clean up old auto backups
#[tauri::command]
pub async fn cleanup_old_backups(app_handle: AppHandle) -> Result<()> {
    BackupManager::cleanup_old_backups(&app_handle).await
}
