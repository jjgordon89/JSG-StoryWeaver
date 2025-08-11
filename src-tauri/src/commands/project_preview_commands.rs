//! Project preview command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::*};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Enhanced project summary for preview
#[derive(Debug, Serialize)]
pub struct EnhancedProjectSummary {
    pub project: Project,
    pub document_count: i32,
    pub character_count: i32,
    pub location_count: i32,
    pub recent_documents: Vec<DocumentSummary>,
    pub recent_activity: Vec<ActivityItem>,
    pub word_count_history: Vec<WordCountHistoryItem>,
}

/// Document summary for preview
#[derive(Debug, Serialize)]
pub struct DocumentSummary {
    pub id: String,
    pub title: String,
    pub document_type: String,
    pub word_count: i32,
    pub updated_at: DateTime<Utc>,
}

/// Activity item for preview
#[derive(Debug, Serialize)]
pub struct ActivityItem {
    pub activity_type: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub related_id: Option<String>,
}

/// Word count history item for preview
#[derive(Debug, Serialize)]
pub struct WordCountHistoryItem {
    pub date: String,
    pub count: i32,
}

/// Get enhanced project preview data
#[tauri::command]
pub async fn get_project_preview(project_id: String) -> CommandResponse<EnhancedProjectSummary> {
    async fn get_preview(project_id: String) -> Result<EnhancedProjectSummary> {
        let pool = get_pool()?;
        
        // Get project
        let project = ProjectOps::get_by_id(&pool, &project_id)
            .await?
            .ok_or_else(|| crate::error::StoryWeaverError::ProjectNotFound { id: project_id.clone() })?;
        
        // Get document count
        let document_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM documents WHERE project_id = ?"
        )
        .bind(&project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to count documents: {}", e)))? as i32;
        
        // Get character count
        let character_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM characters WHERE project_id = ?"
        )
        .bind(&project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to count characters: {}", e)))? as i32;
        
        // Get location count
        let location_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM locations WHERE project_id = ?"
        )
        .bind(&project_id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to count locations: {}", e)))? as i32;
        
        // Get recent documents (up to 5)
        let recent_documents: Vec<DocumentSummary> = sqlx::query!(
            r#"
            SELECT id, title, document_type as "document_type: String", word_count, updated_at
            FROM documents 
            WHERE project_id = ? 
            ORDER BY updated_at DESC 
            LIMIT 5
            "#,
            project_id
        )
        .fetch_all(&*pool)
        .await
        .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to fetch recent documents: {}", e)))?
        .into_iter()
        .map(|row| DocumentSummary {
            id: row.id.unwrap_or_default(),
            title: row.title,
            document_type: row.document_type,
            word_count: row.word_count as i32,
            updated_at: row.updated_at.map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)).unwrap_or_else(|| Utc::now()),
        })
        .collect();
        
        // Get recent activity (for now, we'll create placeholder data)
        // In a real implementation, this would come from an activity log table
        let recent_activity = vec![
            ActivityItem {
                activity_type: "document_edited".to_string(),
                description: "Document 'Chapter 1' was edited".to_string(),
                timestamp: Utc::now(),
                related_id: Some("doc1".to_string()),
            },
            ActivityItem {
                activity_type: "character_created".to_string(),
                description: "Character 'Protagonist' was created".to_string(),
                timestamp: Utc::now(),
                related_id: Some("char1".to_string()),
            },
            ActivityItem {
                activity_type: "project_created".to_string(),
                description: "Project was created".to_string(),
                timestamp: project.created_at,
                related_id: None,
            },
        ];
        
        // Get word count history (for now, we'll create placeholder data)
        // In a real implementation, this would come from a word count history table
        let word_count_history = vec![
            WordCountHistoryItem {
                date: "2025-08-01".to_string(),
                count: 1000,
            },
            WordCountHistoryItem {
                date: "2025-08-02".to_string(),
                count: 1500,
            },
            WordCountHistoryItem {
                date: "2025-08-03".to_string(),
                count: 2200,
            },
            WordCountHistoryItem {
                date: "2025-08-04".to_string(),
                count: 2500,
            },
            WordCountHistoryItem {
                date: "2025-08-05".to_string(),
                count: 3000,
            },
            WordCountHistoryItem {
                date: "2025-08-06".to_string(),
                count: project.current_word_count,
            },
        ];
        
        Ok(EnhancedProjectSummary {
            project,
            document_count,
            character_count,
            location_count,
            recent_documents,
            recent_activity,
            word_count_history,
        })
    }
    
    get_preview(project_id).await.into()
}
