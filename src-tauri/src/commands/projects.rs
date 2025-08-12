//! Project command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::ProjectOps};
use crate::error::Result;
use crate::security::validation::{
    validate_project_name, validate_content_length, validate_security_input
};
use serde::{Deserialize, Serialize};
use crate::security::rate_limit::{check_rate_limit, check_rate_limit_default};
use std::time::Duration;

/// Create project request
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub target_word_count: Option<i32>,
}

/// Update project request
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub genre: Option<String>,
    pub target_word_count: Option<i32>,
    pub status: Option<ProjectStatus>,
    pub settings: Option<String>,
}

/// Create a new project
#[tauri::command]
pub async fn create_project(request: CreateProjectRequest) -> CommandResponse<Project> {
    async fn create(request: CreateProjectRequest) -> Result<Project> {
        // Rate limiting
        check_rate_limit_default("create_project")?;
        // Input validation
        validate_project_name(&request.name)?;
        
        if let Some(ref description) = request.description {
            validate_content_length(description, 5000)?;
            validate_security_input(description)?;
        }
        
        if let Some(ref genre) = request.genre {
            validate_security_input(genre)?;
            validate_content_length(genre, 100)?;
        }
        
        // Validate target word count is reasonable
        if let Some(word_count) = request.target_word_count {
            if word_count < 0 || word_count > 10_000_000 {
                return Err(crate::error::StoryWeaverError::ValidationError {
                    message: "Target word count must be between 0 and 10,000,000".to_string()
                });
            }
        }
        
        let pool = get_pool()?;
        
        let mut project = Project::new(
            request.name,
            request.description,
        );
        
        // Set optional fields
        project.genre = request.genre;
        project.target_word_count = request.target_word_count;
        
        ProjectOps::create(&pool, project).await
    }
    
    create(request).await.into()
}

/// Get all projects
#[tauri::command]
pub async fn get_projects() -> CommandResponse<Vec<Project>> {
    async fn get(_: ()) -> Result<Vec<Project>> {
        let pool = get_pool()?;
        ProjectOps::get_all(&pool).await
    }
    
    get(()).await.into()
}

/// Get a project by ID
#[tauri::command]
pub async fn get_project(id: String) -> CommandResponse<Option<Project>> {
    async fn get(id: String) -> Result<Option<Project>> {
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        ProjectOps::get_by_id(&pool, &id).await
    }
    
    get(id).await.into()
}

/// Update a project
#[tauri::command]
pub async fn update_project(request: UpdateProjectRequest) -> CommandResponse<()> {
    async fn update(request: UpdateProjectRequest) -> Result<()> {
        // Rate limiting
        check_rate_limit(&format!("update_project:{}", &request.id), 120, Duration::from_secs(60))?;
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref name) = request.name {
            validate_project_name(name)?;
        }
        
        if let Some(ref description) = request.description {
            validate_content_length(description, 5000)?;
            validate_security_input(description)?;
        }
        
        if let Some(ref genre) = request.genre {
            validate_security_input(genre)?;
            validate_content_length(genre, 100)?;
        }
        
        if let Some(word_count) = request.target_word_count {
            if word_count < 0 || word_count > 10_000_000 {
                return Err(crate::error::StoryWeaverError::ValidationError {
                    message: "Target word count must be between 0 and 10,000,000".to_string()
                });
            }
        }
        
        if let Some(ref settings) = request.settings {
            validate_content_length(settings, 10000)?;
            validate_security_input(settings)?;
        }
        
        let pool = get_pool()?;
        
        // Get existing project
        let mut project = ProjectOps::get_by_id(&pool, &request.id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::project_not_found(request.id.to_string()))?;
        
        // Update fields if provided
        if let Some(name) = request.name {
            project.name = name;
        }
        if let Some(description) = request.description {
            project.description = Some(description);
        }
        if let Some(genre) = request.genre {
            project.genre = Some(genre);
        }
        if let Some(target_word_count) = request.target_word_count {
            project.target_word_count = Some(target_word_count);
        }
        if let Some(status) = request.status {
            project.status = status;
        }
        if let Some(settings) = request.settings {
            project.settings = settings;
        }
        
        ProjectOps::update(&pool, &project).await
    }
    
    update(request).await.into()
}

/// Delete a project
#[tauri::command]
pub async fn delete_project(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        check_rate_limit(&format!("delete_project:{}", &id), 30, Duration::from_secs(60))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        ProjectOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Update project word count
#[tauri::command]
pub async fn update_project_word_count(project_id: String) -> CommandResponse<()> {
    async fn update_count(project_id: String) -> Result<()> {
        // Rate limiting
        check_rate_limit(&format!("update_project_word_count:{}", &project_id), 120, Duration::from_secs(60))?;
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        ProjectOps::update_word_count(&pool, &project_id).await
    }
    
    update_count(project_id).await.into()
}

/// Project summary for dashboard
#[derive(Debug, Serialize)]
pub struct ProjectSummary {
    pub project: Project,
    pub document_count: i32,
    pub character_count: i32,
    pub location_count: i32,
    pub recent_activity: Vec<RecentActivity>,
}

/// Recent activity item
#[derive(Debug, Serialize)]
pub struct RecentActivity {
    pub activity_type: String,
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Get project summary with statistics
#[tauri::command]
pub async fn get_project_summary(project_id: String) -> CommandResponse<ProjectSummary> {
    async fn get_summary(project_id: String) -> Result<ProjectSummary> {
        // Input validation
        validate_security_input(&project_id)?;
        
        let pool = get_pool()?;
        
        // Get project
        let project = ProjectOps::get_by_id(&pool, &project_id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::project_not_found(project_id.to_string()))?;
        
        // Get counts
        let document_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM documents WHERE project_id = ?"
        )
        .bind(&project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to count documents: {}", e)))? as i32;
        
        let character_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE project_id = ?"
        )
        .bind(&project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to count characters: {}", e)))? as i32;
        
        let location_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM locations WHERE project_id = ?"
        )
        .bind(&project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to count locations: {}", e)))? as i32;
        
        // Get recent activity (simplified for now)
        let recent_activity = vec![
            RecentActivity {
                activity_type: "project_created".to_string(),
                description: "Project was created".to_string(),
                timestamp: project.created_at,
            },
        ];
        
        Ok(ProjectSummary {
            project,
            document_count,
            character_count,
            location_count,
            recent_activity,
        })
    }
    
    get_summary(project_id).await.into()
}
