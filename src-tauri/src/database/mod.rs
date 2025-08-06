//! Database module for StoryWeaver
//! Handles SQLite database initialization, migrations, and core operations

use crate::error::{Result, StoryWeaverError};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub mod models;
pub mod migrations;
pub mod operations;

/// Database connection pool
static mut DB_POOL: Option<Pool<Sqlite>> = None;

/// Initialize the database
pub async fn init(app_handle: &AppHandle) -> Result<()> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| StoryWeaverError::database(format!("Failed to get app data dir: {}", e)))?;
    
    // Ensure the directory exists
    tokio::fs::create_dir_all(&app_data_dir)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to create app data dir: {}", e)))?;
    
    let db_path = app_data_dir.join("storyweaver.db");
    let database_url = format!("sqlite:{}", db_path.display());
    
    // Create connection pool with optimized settings
    let pool = SqlitePool::connect_with(
        sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .foreign_keys(true)
            .pragma("cache_size", "-64000") // 64MB cache
            .pragma("temp_store", "memory")
            .pragma("mmap_size", "268435456") // 256MB mmap
    )
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to connect to database: {}", e)))?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    // Store the pool globally
    unsafe {
        DB_POOL = Some(pool);
    }
    
    Ok(())
}

/// Get the database pool
pub fn get_pool() -> Result<&'static Pool<Sqlite>> {
    unsafe {
        DB_POOL.as_ref().ok_or_else(|| {
            StoryWeaverError::database("Database not initialized")
        })
    }
}

/// Health check for the database
pub async fn health_check() -> Result<()> {
    let pool = get_pool()?;
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Health check failed: {}", e)))?;
    Ok(())
}

/// Get database statistics
pub async fn get_stats() -> Result<DatabaseStats> {
    let pool = get_pool()?;
    
    let projects_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects")
        .fetch_one(pool)
        .await
        .unwrap_or(0);
    
    let documents_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM documents")
        .fetch_one(pool)
        .await
        .unwrap_or(0);
    
    let characters_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM characters")
        .fetch_one(pool)
        .await
        .unwrap_or(0);
    
    let locations_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM locations")
        .fetch_one(pool)
        .await
        .unwrap_or(0);
    
    Ok(DatabaseStats {
        projects_count: projects_count as u32,
        documents_count: documents_count as u32,
        characters_count: characters_count as u32,
        locations_count: locations_count as u32,
    })
}

/// Database statistics
#[derive(Debug, serde::Serialize)]
pub struct DatabaseStats {
    pub projects_count: u32,
    pub documents_count: u32,
    pub characters_count: u32,
    pub locations_count: u32,
}