use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};
use std::path::{Path, PathBuf};
use chrono::{Utc, DateTime};
use tokio::fs;
use tauri::AppHandle;

/// Database backup manager
pub struct BackupManager;

impl BackupManager {
    /// Create a backup of the database
    pub async fn create_backup(
        app_handle: &AppHandle,
        backup_name: Option<String>,
    ) -> Result<PathBuf> {
        // Get app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| StoryWeaverError::database(format!("Failed to get app data dir: {}", e)))?;
        
        // Create backups directory if it doesn't exist
        let backups_dir = app_data_dir.join("backups");
        fs::create_dir_all(&backups_dir)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create backups directory: {}", e)))?;
        
        // Source database path
        let db_path = app_data_dir.join("storyweaver.db");
        
        // Generate backup filename
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = match backup_name {
            Some(name) => format!("{}_{}.db", name, timestamp),
            None => format!("backup_{}.db", timestamp),
        };
        
        let backup_path = backups_dir.join(&backup_filename);
        
        // Copy the database file
        fs::copy(&db_path, &backup_path)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create backup: {}", e)))?;
        
        // Also copy the WAL and SHM files if they exist
        let wal_path = db_path.with_extension("db-wal");
        let shm_path = db_path.with_extension("db-shm");
        
        if wal_path.exists() {
            let backup_wal_path = backup_path.with_extension("db-wal");
            fs::copy(&wal_path, &backup_wal_path)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to backup WAL file: {}", e)))?;
        }
        
        if shm_path.exists() {
            let backup_shm_path = backup_path.with_extension("db-shm");
            fs::copy(&shm_path, &backup_shm_path)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to backup SHM file: {}", e)))?;
        }
        
        // Record backup in the database
        let pool = crate::database::get_pool()?;
        sqlx::query(
            r#"
            INSERT INTO backups (
                id, filename, created_at, is_auto, comment
            )
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&backup_filename)
        .bind(Utc::now())
        .bind(backup_name.is_none()) // If no name provided, it's an auto backup
        .bind(backup_name)
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to record backup: {}", e)))?;
        
        Ok(backup_path)
    }
    
    /// Restore from a backup
    pub async fn restore_from_backup(
        app_handle: &AppHandle,
        backup_filename: &str,
    ) -> Result<()> {
        // Get app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| StoryWeaverError::database(format!("Failed to get app data dir: {}", e)))?;
        
        // Backup source path
        let backups_dir = app_data_dir.join("backups");
        let backup_path = backups_dir.join(backup_filename);
        
        if !backup_path.exists() {
            return Err(StoryWeaverError::database(format!("Backup file not found: {}", backup_filename)));
        }
        
        // Target database path
        let db_path = app_data_dir.join("storyweaver.db");
        
        // Create a backup of the current database before restoring
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let pre_restore_backup = backups_dir.join(format!("pre_restore_{}.db", timestamp));
        
        fs::copy(&db_path, &pre_restore_backup)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create pre-restore backup: {}", e)))?;
        
        // Close the database connection
        // This is important to ensure the database file is not locked
        unsafe {
            if let Some(pool) = crate::database::DB_POOL.take() {
                pool.close().await;
            }
        }
        
        // Copy the backup file to the database location
        fs::copy(&backup_path, &db_path)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to restore backup: {}", e)))?;
        
        // Also copy the WAL and SHM files if they exist
        let backup_wal_path = backup_path.with_extension("db-wal");
        let backup_shm_path = backup_path.with_extension("db-shm");
        let db_wal_path = db_path.with_extension("db-wal");
        let db_shm_path = db_path.with_extension("db-shm");
        
        if backup_wal_path.exists() {
            fs::copy(&backup_wal_path, &db_wal_path)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to restore WAL file: {}", e)))?;
        }
        
        if backup_shm_path.exists() {
            fs::copy(&backup_shm_path, &db_shm_path)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to restore SHM file: {}", e)))?;
        }
        
        // Reinitialize the database connection
        crate::database::init(app_handle).await?;
        
        Ok(())
    }
    
    /// Get list of available backups
    pub async fn get_backups(app_handle: &AppHandle) -> Result<Vec<BackupInfo>> {
        let pool = crate::database::get_pool()?;
        
        let backups = sqlx::query_as!(
            BackupRecord,
            r#"
            SELECT id, filename, created_at, is_auto, comment
            FROM backups
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get backups: {}", e)))?;
        
        // Get app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| StoryWeaverError::database(format!("Failed to get app data dir: {}", e)))?;
        
        let backups_dir = app_data_dir.join("backups");
        
        // Convert to BackupInfo and check if file exists
        let mut backup_infos = Vec::new();
        for backup in backups {
            let backup_path = backups_dir.join(&backup.filename);
            let file_exists = backup_path.exists();
            let file_size = if file_exists {
                match fs::metadata(&backup_path).await {
                    Ok(metadata) => Some(metadata.len()),
                    Err(_) => None,
                }
            } else {
                None
            };
            
            backup_infos.push(BackupInfo {
                id: backup.id,
                filename: backup.filename,
                created_at: backup.created_at,
                is_auto: backup.is_auto,
                comment: backup.comment,
                file_exists,
                file_size,
            });
        }
        
        Ok(backup_infos)
    }
    
    /// Delete a backup
    pub async fn delete_backup(app_handle: &AppHandle, backup_id: &str) -> Result<()> {
        let pool = crate::database::get_pool()?;
        
        // Get the backup filename
        let backup = sqlx::query!(
            "SELECT filename FROM backups WHERE id = ?",
            backup_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get backup: {}", e)))?
        .ok_or_else(|| StoryWeaverError::database(format!("Backup not found: {}", backup_id)))?;
        
        // Get app data directory
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| StoryWeaverError::database(format!("Failed to get app data dir: {}", e)))?;
        
        let backups_dir = app_data_dir.join("backups");
        let backup_path = backups_dir.join(&backup.filename);
        
        // Delete the backup file if it exists
        if backup_path.exists() {
            fs::remove_file(&backup_path)
                .await
                .map_err(|e| StoryWeaverError::database(format!("Failed to delete backup file: {}", e)))?;
            
            // Also delete WAL and SHM files if they exist
            let wal_path = backup_path.with_extension("db-wal");
            let shm_path = backup_path.with_extension("db-shm");
            
            if wal_path.exists() {
                fs::remove_file(&wal_path)
                    .await
                    .map_err(|e| StoryWeaverError::database(format!("Failed to delete WAL file: {}", e)))?;
            }
            
            if shm_path.exists() {
                fs::remove_file(&shm_path)
                    .await
                    .map_err(|e| StoryWeaverError::database(format!("Failed to delete SHM file: {}", e)))?;
            }
        }
        
        // Delete the backup record
        sqlx::query("DELETE FROM backups WHERE id = ?")
            .bind(backup_id)
            .execute(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to delete backup record: {}", e)))?;
        
        Ok(())
    }
    
    /// Create an automatic backup
    pub async fn create_auto_backup(app_handle: &AppHandle) -> Result<()> {
        // Check if we need to create an auto backup
        let pool = crate::database::get_pool()?;
        
        // Get the last auto backup time
        let last_auto_backup = sqlx::query_scalar::<_, Option<DateTime<Utc>>>(
            "SELECT MAX(created_at) FROM backups WHERE is_auto = 1"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get last auto backup: {}", e)))?;
        
        // Get the auto backup interval from settings
        let auto_backup_interval = sqlx::query_scalar::<_, Option<String>>(
            "SELECT value FROM settings WHERE key = 'auto_backup_interval'"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get auto backup interval: {}", e)))?
        .unwrap_or_else(|| "daily".to_string());
        
        let now = Utc::now();
        let should_backup = match last_auto_backup {
            Some(last_time) => {
                match auto_backup_interval.as_str() {
                    "hourly" => (now - last_time).num_hours() >= 1,
                    "daily" => (now - last_time).num_days() >= 1,
                    "weekly" => (now - last_time).num_weeks() >= 1,
                    "monthly" => (now - last_time).num_days() >= 30,
                    _ => false, // If setting is invalid, don't auto backup
                }
            },
            None => true, // No previous auto backup, so create one
        };
        
        if should_backup {
            // Create the backup
            Self::create_backup(app_handle, None).await?;
        }
        
        Ok(())
    }
    
    /// Clean up old auto backups
    pub async fn cleanup_old_backups(app_handle: &AppHandle) -> Result<()> {
        let pool = crate::database::get_pool()?;
        
        // Get the max number of auto backups to keep
        let max_auto_backups = sqlx::query_scalar::<_, Option<String>>(
            "SELECT value FROM settings WHERE key = 'max_auto_backups'"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get max auto backups: {}", e)))?
        .unwrap_or_else(|| "10".to_string())
        .parse::<i64>()
        .unwrap_or(10);
        
        // Get auto backups to delete
        let backups_to_delete = sqlx::query!(
            r#"
            SELECT id, filename
            FROM backups
            WHERE is_auto = 1
            ORDER BY created_at DESC
            LIMIT -1 OFFSET ?
            "#,
            max_auto_backups
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get old backups: {}", e)))?;
        
        // Delete old backups
        for backup in backups_to_delete {
            Self::delete_backup(app_handle, &backup.id).await?;
        }
        
        Ok(())
    }
}

/// Backup record from database
#[derive(Debug, sqlx::FromRow)]
struct BackupRecord {
    pub id: String,
    pub filename: String,
    pub created_at: DateTime<Utc>,
    pub is_auto: bool,
    pub comment: Option<String>,
}

/// Backup information
#[derive(Debug, serde::Serialize)]
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub created_at: DateTime<Utc>,
    pub is_auto: bool,
    pub comment: Option<String>,
    pub file_exists: bool,
    pub file_size: Option<u64>,
}
