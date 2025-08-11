//! Series command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::*};
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// Create series request
#[derive(Debug, Deserialize)]
pub struct CreateSeriesRequest {
    pub name: String,
    pub description: Option<String>,
    pub folder_id: Option<String>,
}

/// Update series request
#[derive(Debug, Deserialize)]
pub struct UpdateSeriesRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub folder_id: Option<String>,
}

/// Create a new series
#[tauri::command]
pub async fn create_series(request: CreateSeriesRequest) -> CommandResponse<Series> {
    async fn create(request: CreateSeriesRequest) -> Result<Series> {
        let pool = get_pool()?;
        
        let series = Series::new(
            request.name,
            request.description,
            request.folder_id,
        );
        
        SeriesOps::create(&pool, series).await
    }
    
    create(request).await.into()
}

/// Get a series by ID
#[tauri::command]
pub async fn get_series(id: String) -> CommandResponse<Option<Series>> {
    async fn get(id: String) -> Result<Option<Series>> {
        let pool = get_pool()?;
        SeriesOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Get all series
#[tauri::command]
pub async fn get_all_series() -> CommandResponse<Vec<Series>> {
    async fn get_all() -> Result<Vec<Series>> {
        let pool = get_pool()?;
        SeriesOps::get_all(&pool).await
    }
    
    get_all().await.into()
}

/// Get series with project counts
#[tauri::command]
pub async fn get_series_with_counts() -> CommandResponse<Vec<SeriesWithCount>> {
    async fn get_with_counts() -> Result<Vec<SeriesWithCount>> {
        let pool = get_pool()?;
        SeriesOps::get_series_with_counts(&pool).await
    }
    
    get_with_counts().await.into()
}

/// Update a series
#[tauri::command]
pub async fn update_series(request: UpdateSeriesRequest) -> CommandResponse<()> {
    async fn update(request: UpdateSeriesRequest) -> Result<()> {
        let pool = get_pool()?;
        
        // Get existing series
        let mut series = SeriesOps::get_by_id(&pool, &request.id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::series_not_found(request.id.clone()))?;
        
        // Update fields if provided
        if let Some(name) = request.name {
            series.name = name;
        }
        if let Some(description) = request.description {
            series.description = Some(description);
        }
        if let Some(folder_id) = request.folder_id {
            series.folder_id = Some(folder_id);
        }
        
        SeriesOps::update(&pool, &series).await
    }
    
    update(request).await.into()
}

/// Delete a series
#[tauri::command]
pub async fn delete_series(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        let pool = get_pool()?;
        SeriesOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Get projects in a series
#[tauri::command]
pub async fn get_series_projects(series_id: String) -> CommandResponse<Vec<Project>> {
    async fn get_projects(series_id: String) -> Result<Vec<Project>> {
        let pool = get_pool()?;
        SeriesOps::get_projects(&pool, &series_id).await
    }
    
    get_projects(series_id).await.into()
}

/// Add project to series
#[tauri::command]
pub async fn add_project_to_series(series_id: String, project_id: String) -> CommandResponse<()> {
    async fn add_project(series_id: String, project_id: String) -> Result<()> {
        let pool = get_pool()?;
        SeriesOps::add_project(&pool, &series_id, &project_id).await
    }
    
    add_project(series_id, project_id).await.into()
}

/// Remove project from series
#[tauri::command]
pub async fn remove_project_from_series(project_id: String) -> CommandResponse<()> {
    async fn remove_project(project_id: String) -> Result<()> {
        let pool = get_pool()?;
        SeriesOps::remove_project(&pool, &project_id).await
    }
    
    remove_project(project_id).await.into()
}
