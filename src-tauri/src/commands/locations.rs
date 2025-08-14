//! Location command handlers

use crate::commands::CommandResponse;
use crate::database::{get_pool, models::*, operations::LocationOps};
use crate::error::Result;
use crate::security::validation::*;
use crate::security::rate_limit::{rl_create, rl_update, rl_delete, rl_list, validate_request_body_size};
use serde::{Deserialize, Serialize};

/// Create location request
#[derive(Debug, Deserialize)]
pub struct CreateLocationRequest {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location_type: Option<String>,
    pub geography: Option<String>,
    pub climate: Option<String>,
    pub culture: Option<String>,
    pub history: Option<String>,
    pub significance: Option<Importance>,
    pub visibility: Option<VisibilityLevel>,
}

/// Update location request
#[derive(Debug, Deserialize)]
pub struct UpdateLocationRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location_type: Option<String>,
    pub geography: Option<String>,
    pub climate: Option<String>,
    pub culture: Option<String>,
    pub history: Option<String>,
    pub significance: Option<Importance>,
    pub visibility: Option<VisibilityLevel>,
    pub metadata: Option<String>,
}

/// Create a new location
#[tauri::command]
pub async fn create_location(request: CreateLocationRequest) -> CommandResponse<Location> {
    async fn create(request: CreateLocationRequest) -> Result<Location> {
        // Rate limiting
        rl_create("location", Some(&request.project_id))?;
        // Input validation
        validate_security_input(&request.project_id)?;
        validate_safe_name(&request.name, "Location name")?;
        
        if let Some(ref description) = request.description {
            validate_request_body_size(description, 10_000)?;
            validate_content_length(description, 10000)?;
            validate_security_input(description)?;
        }
        
        if let Some(ref location_type) = request.location_type {
            validate_request_body_size(location_type, 100)?;
            validate_content_length(location_type, 100)?;
            validate_security_input(location_type)?;
        }
        
        if let Some(ref geography) = request.geography {
            validate_request_body_size(geography, 5_000)?;
            validate_content_length(geography, 5000)?;
            validate_security_input(geography)?;
        }
        
        if let Some(ref climate) = request.climate {
            validate_request_body_size(climate, 5_000)?;
            validate_content_length(climate, 5000)?;
            validate_security_input(climate)?;
        }
        
        if let Some(ref culture) = request.culture {
            validate_request_body_size(culture, 10_000)?;
            validate_content_length(culture, 10000)?;
            validate_security_input(culture)?;
        }
        
        if let Some(ref history) = request.history {
            validate_request_body_size(history, 10_000)?;
            validate_content_length(history, 10000)?;
            validate_security_input(history)?;
        }
        
        let pool = get_pool()?;
        
        let mut location = Location::new(
            request.project_id,
            request.name,
            LocationType::Fictional, // Default type, will be overridden if provided
        );
        
        // Set optional fields
        location.description = request.description;
        if let Some(location_type) = request.location_type {
            // Parse string to LocationType enum
            location.location_type = match location_type.to_lowercase().as_str() {
                "city" => LocationType::City,
                "building" => LocationType::Building,
                "room" => LocationType::Room,
                "landscape" => LocationType::Landscape,
                "historical" => LocationType::Historical,
                _ => LocationType::Fictional,
            };
        }
        location.geography = request.geography;
        location.climate = request.climate;
        location.culture = request.culture;
        location.history = request.history;
        if let Some(significance) = request.significance {
            location.significance = Some(format!("{:?}", significance));
        }
        if let Some(visibility) = request.visibility {
            location.visibility = visibility;
        }
        
        LocationOps::create(&pool, location).await
    }
    
    create(request).await.into()
}

/// Get locations by project ID
#[tauri::command]
pub async fn get_locations(project_id: String) -> CommandResponse<Vec<Location>> {
    async fn get_by_project(project_id: String) -> Result<Vec<Location>> {
        // Rate limiting
        rl_list("locations", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id.to_string())?;
        
        let pool = get_pool()?;
        LocationOps::get_by_project(&pool, &project_id).await
    }
    
    get_by_project(project_id).await.into()
}

/// Get a location by ID
#[tauri::command]
pub async fn get_location(id: String) -> CommandResponse<Option<Location>> {
    async fn get(id: String) -> Result<Option<Location>> {
        // Rate limiting
        rl_list("location", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        
        let location = sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = ?")
            .bind(&id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get location: {}", e)))?;
        
        Ok(location)
    }
    
    get(id).await.into()
}

/// Update a location
#[tauri::command]
pub async fn update_location(request: UpdateLocationRequest) -> CommandResponse<()> {
    async fn update(request: UpdateLocationRequest) -> Result<()> {
        // Rate limiting
        rl_update("location", Some(&request.id))?;
        // Input validation
        validate_security_input(&request.id)?;
        
        if let Some(ref name) = request.name {
            validate_safe_name(name, "Location name")?;
        }
        
        if let Some(ref description) = request.description {
            validate_request_body_size(description, 10_000)?;
            validate_content_length(description, 10000)?;
            validate_security_input(description)?;
        }
        
        if let Some(ref location_type) = request.location_type {
            validate_request_body_size(location_type, 100)?;
            validate_content_length(location_type, 100)?;
            validate_security_input(location_type)?;
        }
        
        if let Some(ref geography) = request.geography {
            validate_request_body_size(geography, 5_000)?;
            validate_content_length(geography, 5000)?;
            validate_security_input(geography)?;
        }
        
        if let Some(ref climate) = request.climate {
            validate_request_body_size(climate, 5_000)?;
            validate_content_length(climate, 5000)?;
            validate_security_input(climate)?;
        }
        
        if let Some(ref culture) = request.culture {
            validate_request_body_size(culture, 10_000)?;
            validate_content_length(culture, 10000)?;
            validate_security_input(culture)?;
        }
        
        if let Some(ref history) = request.history {
            validate_request_body_size(history, 10_000)?;
            validate_content_length(history, 10000)?;
            validate_security_input(history)?;
        }
        
        if let Some(ref metadata) = request.metadata {
            validate_request_body_size(metadata, 5_000)?;
            validate_content_length(metadata, 5000)?;
            validate_security_input(metadata)?;
        }
        
        let pool = get_pool()?;
        
        // Get existing location
        let mut location = sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = ?")
            .bind(&request.id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get location: {}", e)))?
            .ok_or_else(|| crate::error::StoryWeaverError::Internal { message: "Location not found".to_string() })?;
        
        // Update fields if provided
        if let Some(name) = request.name {
            location.name = name;
        }
        if let Some(description) = request.description {
            location.description = Some(description);
        }
        if let Some(location_type) = request.location_type {
            // Parse string to LocationType enum
            location.location_type = match location_type.to_lowercase().as_str() {
                "city" => LocationType::City,
                "building" => LocationType::Building,
                "room" => LocationType::Room,
                "landscape" => LocationType::Landscape,
                "historical" => LocationType::Historical,
                _ => LocationType::Fictional,
            };
        }
        if let Some(geography) = request.geography {
            location.geography = Some(geography);
        }
        if let Some(climate) = request.climate {
            location.climate = Some(climate);
        }
        if let Some(culture) = request.culture {
            location.culture = Some(culture);
        }
        if let Some(history) = request.history {
            location.history = Some(history);
        }
        if let Some(significance) = request.significance {
            location.significance = Some(format!("{:?}", significance));
        }
        if let Some(visibility) = request.visibility {
            location.visibility = visibility;
        }
        if let Some(metadata) = request.metadata {
            location.metadata = metadata;
        }
        
        LocationOps::update(&pool, &location).await
    }
    
    update(request).await.into()
}

/// Delete a location
#[tauri::command]
pub async fn delete_location(id: String) -> CommandResponse<()> {
    async fn delete(id: String) -> Result<()> {
        // Rate limiting
        rl_delete("location", Some(&id))?;
        // Input validation
        validate_security_input(&id)?;
        
        let pool = get_pool()?;
        LocationOps::delete(&pool, &id).await
    }
    
    delete(id).await.into()
}

/// Location summary for quick reference
#[derive(Debug, Serialize)]
pub struct LocationSummary {
    pub id: String,
    pub name: String,
    pub location_type: Option<String>,
    pub description: Option<String>,
    pub significance: Option<Importance>,
    pub key_features: Vec<String>,
}

/// Get location summaries for a project (lightweight version)
#[tauri::command]
pub async fn get_location_summaries(project_id: String) -> CommandResponse<Vec<LocationSummary>> {
    async fn get_summaries(project_id: String) -> Result<Vec<LocationSummary>> {
        // Rate limiting
        rl_list("location_summaries", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id.to_string())?;
        
        let pool = get_pool()?;
        let locations = LocationOps::get_by_project(&pool, &project_id).await?;
        
        let summaries = locations
            .into_iter()
            .map(|location| {
                let mut key_features = Vec::new();
                
                // Extract key features from geography and culture
                if let Some(geography) = &location.geography {
                    if !geography.is_empty() {
                        let feature_text = geography
                            .split('.')
                            .next()
                            .unwrap_or(geography)
                            .chars()
                            .take(50)
                            .collect::<String>();
                        if !feature_text.trim().is_empty() {
                            key_features.push(format!("Geography: {}", feature_text.trim()));
                        }
                    }
                }
                
                if let Some(climate) = &location.climate {
                    if !climate.is_empty() && key_features.len() < 3 {
                        let feature_text = climate
                            .split('.')
                            .next()
                            .unwrap_or(climate)
                            .chars()
                            .take(50)
                            .collect::<String>();
                        if !feature_text.trim().is_empty() {
                            key_features.push(format!("Climate: {}", feature_text.trim()));
                        }
                    }
                }
                
                if let Some(culture) = &location.culture {
                    if !culture.is_empty() && key_features.len() < 3 {
                        let feature_text = culture
                            .split('.')
                            .next()
                            .unwrap_or(culture)
                            .chars()
                            .take(50)
                            .collect::<String>();
                        if !feature_text.trim().is_empty() {
                            key_features.push(format!("Culture: {}", feature_text.trim()));
                        }
                    }
                }
                
                LocationSummary {
                    id: location.id,
                    name: location.name,
                    location_type: Some(format!("{:?}", location.location_type)),
                    description: location.description,
                    significance: location.significance.as_ref().and_then(|s| {
                        match s.as_str() {
                            "Critical" => Some(Importance::Critical),
                            "High" => Some(Importance::High),
                            "Medium" => Some(Importance::Medium),
                            "Low" => Some(Importance::Low),
                            _ => None,
                        }
                    }),
                    key_features,
                }
            })
            .collect();
        
        Ok(summaries)
    }
    
    get_summaries(project_id).await.into()
}

/// Location hierarchy for world-building
#[derive(Debug, Serialize)]
pub struct LocationHierarchy {
    pub location: Location,
    pub parent_locations: Vec<LocationSummary>,
    pub child_locations: Vec<LocationSummary>,
    pub nearby_locations: Vec<LocationSummary>,
}

/// Get location hierarchy (relationships between locations)
#[tauri::command]
pub async fn get_location_hierarchy(location_id: String) -> CommandResponse<LocationHierarchy> {
    async fn get_hierarchy(location_id: String) -> Result<LocationHierarchy> {
        // Rate limiting
        rl_list("location_hierarchy", Some(&location_id))?;
        // Input validation
        validate_security_input(&location_id)?;
        
        let pool = get_pool()?;
        
        // Get the main location
        let location = sqlx::query_as::<_, Location>("SELECT * FROM locations WHERE id = ?")
            .bind(&location_id)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| crate::error::StoryWeaverError::database(format!("Failed to get location: {}", e)))?
            .ok_or_else(|| crate::error::StoryWeaverError::Internal { message: "Location not found".to_string() })?;
        
        // Get all locations in the same project
        let all_locations = LocationOps::get_by_project(&pool, &location.project_id).await?;
        
        // For now, we'll use simple heuristics to determine relationships
        // In a more advanced implementation, this would be based on explicit relationships
        let mut parent_locations = Vec::new();
        let mut child_locations = Vec::new();
        let mut nearby_locations = Vec::new();
        
        for other_location in all_locations {
            if other_location.id == location.id {
                continue;
            }
            
            let summary = LocationSummary {
                id: other_location.id.clone(),
                name: other_location.name.clone(),
                location_type: Some(format!("{:?}", other_location.location_type)),
                description: other_location.description.clone(),
                significance: other_location.significance.as_ref().and_then(|s| {
                    match s.as_str() {
                        "Critical" => Some(Importance::Critical),
                        "High" => Some(Importance::High),
                        "Medium" => Some(Importance::Medium),
                        "Low" => Some(Importance::Low),
                        _ => None,
                    }
                }),
                key_features: Vec::new(), // Simplified for hierarchy
            };
            
            // Simple heuristic: if location type suggests hierarchy
            if let (current_type, other_type) = (&location.location_type, &other_location.location_type) {
                let current_type_lower = format!("{:?}", current_type).to_lowercase();
                let other_type_lower = format!("{:?}", other_type).to_lowercase();
                
                // Parent relationships (larger contains smaller)
                if (current_type_lower.contains("city") && other_type_lower.contains("country")) ||
                   (current_type_lower.contains("building") && other_type_lower.contains("city")) ||
                   (current_type_lower.contains("room") && other_type_lower.contains("building")) {
                    parent_locations.push(summary);
                }
                // Child relationships (smaller contained in larger)
                else if (other_type_lower.contains("city") && current_type_lower.contains("country")) ||
                        (other_type_lower.contains("building") && current_type_lower.contains("city")) ||
                        (other_type_lower.contains("room") && current_type_lower.contains("building")) {
                    child_locations.push(summary);
                }
                // Nearby relationships (same level)
                else if current_type_lower == other_type_lower {
                    nearby_locations.push(summary);
                }
            } else {
                // If no type information, consider as nearby
                nearby_locations.push(summary);
            }
        }
        
        Ok(LocationHierarchy {
            location,
            parent_locations,
            child_locations,
            nearby_locations,
        })
    }
    
    get_hierarchy(location_id).await.into()
}

/// Location statistics
#[derive(Debug, Serialize)]
pub struct LocationStats {
    pub total_locations: i32,
    pub by_type: std::collections::HashMap<String, i32>,
    pub by_significance: std::collections::HashMap<String, i32>,
    pub by_visibility: std::collections::HashMap<String, i32>,
    pub major_locations: Vec<LocationSummary>,
}

/// Get location statistics for a project
#[tauri::command]
pub async fn get_location_stats(project_id: String) -> CommandResponse<LocationStats> {
    async fn get_stats(project_id: String) -> Result<LocationStats> {
        // Rate limiting
        rl_list("location_stats", Some(&project_id))?;
        // Input validation
        validate_security_input(&project_id.to_string())?;
        
        let pool = get_pool()?;
        let locations = LocationOps::get_by_project(&pool, &project_id).await?;
        
        let total_locations = locations.len() as i32;
        
        // Count by type
        let mut by_type = std::collections::HashMap::new();
        for location in &locations {
            let type_str = format!("{:?}", location.location_type);
            *by_type.entry(type_str).or_insert(0) += 1;
        }
        
        // Count by significance
        let mut by_significance = std::collections::HashMap::new();
        for location in &locations {
            let significance_str = location.significance
                .as_ref()
                .map(|s| format!("{:?}", s))
                .unwrap_or_else(|| "Unknown".to_string());
            *by_significance.entry(significance_str).or_insert(0) += 1;
        }
        
        // Count by visibility
        let mut by_visibility = std::collections::HashMap::new();
        for location in &locations {
            let visibility_str = format!("{:?}", location.visibility);
            *by_visibility.entry(visibility_str).or_insert(0) += 1;
        }
        
        // Get major locations (high significance)
        let major_locations = locations
            .into_iter()
            .filter(|l| {
                l.significance.as_ref().map_or(false, |s| s == "Critical" || s == "High")
            })
            .map(|location| LocationSummary {
                id: location.id,
                name: location.name,
                location_type: Some(format!("{:?}", location.location_type)),
                description: location.description,
                significance: location.significance.as_ref().and_then(|s| {
                    match s.as_str() {
                        "Critical" => Some(Importance::Critical),
                        "High" => Some(Importance::High),
                        "Medium" => Some(Importance::Medium),
                        "Low" => Some(Importance::Low),
                        _ => None,
                    }
                }),
                key_features: Vec::new(), // Simplified for stats
            })
            .collect();
        
        Ok(LocationStats {
            total_locations,
            by_type,
            by_significance,
            by_visibility,
            major_locations,
        })
    }
    
    get_stats(project_id).await.into()
}
