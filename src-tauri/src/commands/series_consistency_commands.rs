//! Series consistency checking commands

use crate::database::{get_pool, operations::SeriesConsistencyOps};
use crate::database::operations::series_consistency_ops::*;
use crate::commands::CommandResponse;
use crate::error::StoryWeaverError;
use tauri::command;

/// Generate a comprehensive consistency report for a series
#[command]
pub async fn generate_series_consistency_report(
    series_id: String,
) -> Result<CommandResponse<SeriesConsistencyReport>, StoryWeaverError> {
    let pool = get_pool()?;
    
    match SeriesConsistencyOps::generate_consistency_report(&pool, &series_id).await {
        Ok(report) => Ok(CommandResponse::success(report)),
        Err(e) => Ok(CommandResponse::error(e.to_string())),
    }
}

/// Get quick consistency status (score and conflict count) for a series
#[command]
pub async fn get_series_consistency_status(
    series_id: String,
) -> Result<CommandResponse<(f64, usize)>, StoryWeaverError> {
    let pool = get_pool()?;
    
    match SeriesConsistencyOps::get_consistency_status(&pool, &series_id).await {
        Ok(status) => Ok(CommandResponse::success(status)),
        Err(e) => Ok(CommandResponse::error(e.to_string())),
    }
}

/// Get conflicts filtered by severity level
#[command]
pub async fn get_series_conflicts_by_severity(
    series_id: String,
    severity: String, // "Critical", "High", "Medium", "Low"
) -> Result<CommandResponse<Vec<ConsistencyConflict>>, StoryWeaverError> {
    let pool = get_pool()?;
    
    let severity_enum = match severity.as_str() {
        "Critical" => ConflictSeverity::Critical,
        "High" => ConflictSeverity::High,
        "Medium" => ConflictSeverity::Medium,
        "Low" => ConflictSeverity::Low,
        _ => return Ok(CommandResponse::error("Invalid severity level".to_string())),
    };
    
    match SeriesConsistencyOps::get_conflicts_by_severity(&pool, &series_id, severity_enum).await {
        Ok(conflicts) => Ok(CommandResponse::success(conflicts)),
        Err(e) => Ok(CommandResponse::error(e.to_string())),
    }
}

/// Batch check consistency for multiple series
#[command]
pub async fn batch_check_series_consistency(
    series_ids: Vec<String>,
) -> Result<CommandResponse<Vec<(String, f64, usize)>>, StoryWeaverError> {
    let pool = get_pool()?;
    let mut results = Vec::new();
    
    for series_id in series_ids {
        match SeriesConsistencyOps::get_consistency_status(&pool, &series_id).await {
            Ok((score, conflict_count)) => {
                results.push((series_id, score, conflict_count));
            }
            Err(_) => {
                // Skip series that can't be checked (e.g., don't exist)
                continue;
            }
        }
    }
    
    Ok(CommandResponse::success(results))
}
