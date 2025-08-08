use crate::database::models::*;
use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Series consistency checking operations
pub struct SeriesConsistencyOps;

/// Consistency conflict types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    CharacterInconsistency,
    WorldElementInconsistency,
    TimelineConflict,
    StyleMismatch,
    GenreConflict,
}

/// Consistency conflict details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyConflict {
    pub conflict_type: ConflictType,
    pub severity: ConflictSeverity,
    pub description: String,
    pub affected_projects: Vec<String>,
    pub affected_elements: Vec<String>,
    pub suggestions: Vec<String>,
}

/// Conflict severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictSeverity {
    Critical,  // Major story inconsistencies
    High,      // Important character/world conflicts
    Medium,    // Style or minor element conflicts
    Low,       // Formatting or preference differences
}

/// Series consistency report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesConsistencyReport {
    pub series_id: String,
    pub series_name: String,
    pub total_projects: usize,
    pub conflicts: Vec<ConsistencyConflict>,
    pub consistency_score: f64, // 0.0 to 1.0
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Character consistency data for comparison
#[derive(Debug, Clone)]
struct CharacterConsistencyData {
    pub character_name: String,
    pub project_id: String,
    pub traits: Vec<CharacterTrait>,
    pub description: Option<String>,
}

/// World element consistency data for comparison
#[derive(Debug, Clone)]
struct WorldElementConsistencyData {
    pub element_name: String,
    pub project_id: String,
    pub element_type: String,
    pub description: String,
    pub details: Option<String>,
}

impl SeriesConsistencyOps {
    /// Generate comprehensive consistency report for a series
    pub async fn generate_consistency_report(
        pool: &Pool<Sqlite>,
        series_id: &str,
    ) -> Result<SeriesConsistencyReport> {
        // Get series information
        let series = crate::database::operations::series_ops::SeriesOps::get_by_id(pool, series_id)
            .await?
            .ok_or_else(|| StoryWeaverError::NotFound {
                resource: "Series".to_string(),
                id: series_id.to_string(),
            })?;

        // Get all projects in the series
        let projects = crate::database::operations::series_ops::SeriesOps::get_projects(pool, series_id).await?;
        
        let mut conflicts = Vec::new();
        
        // Check character consistency
        conflicts.extend(Self::check_character_consistency(pool, &projects).await?);
        
        // Check world element consistency
        conflicts.extend(Self::check_world_element_consistency(pool, &projects).await?);
        
        // Check story bible consistency
        conflicts.extend(Self::check_story_bible_consistency(pool, &projects).await?);
        
        // Calculate consistency score
        let consistency_score = Self::calculate_consistency_score(&conflicts, projects.len());
        
        Ok(SeriesConsistencyReport {
            series_id: series_id.to_string(),
            series_name: series.name,
            total_projects: projects.len(),
            conflicts,
            consistency_score,
            generated_at: chrono::Utc::now(),
        })
    }
    
    /// Check character consistency across projects
    async fn check_character_consistency(
        pool: &Pool<Sqlite>,
        projects: &[Project],
    ) -> Result<Vec<ConsistencyConflict>> {
        let mut conflicts = Vec::new();
        let mut character_data: HashMap<String, Vec<CharacterConsistencyData>> = HashMap::new();
        
        // Collect character data from all projects
        for project in projects {
            // Get character traits for this project
            let traits = sqlx::query_as::<_, CharacterTrait>(
                "SELECT * FROM character_traits ct 
                 JOIN characters c ON ct.character_id = c.id 
                 WHERE c.project_id = ? AND ct.series_shared = true"
            )
            .bind(&project.id)
            .fetch_all(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get character traits: {}", e)))?;
            
            // Group traits by character name
            let mut project_characters: HashMap<String, Vec<CharacterTrait>> = HashMap::new();
            for trait in traits {
                // Extract character name from trait content or use a placeholder
                let character_name = Self::extract_character_name_from_trait(&trait);
                project_characters.entry(character_name).or_default().push(trait);
            }
            
            // Add to global character data
            for (character_name, traits) in project_characters {
                character_data.entry(character_name.clone()).or_default().push(
                    CharacterConsistencyData {
                        character_name: character_name.clone(),
                        project_id: project.id.clone(),
                        traits,
                        description: None, // Could be enhanced to include character descriptions
                    }
                );
            }
        }
        
        // Check for conflicts between character versions
        for (character_name, versions) in character_data {
            if versions.len() > 1 {
                let character_conflicts = Self::analyze_character_conflicts(&character_name, &versions);
                conflicts.extend(character_conflicts);
            }
        }
        
        Ok(conflicts)
    }
    
    /// Check world element consistency across projects
    async fn check_world_element_consistency(
        pool: &Pool<Sqlite>,
        projects: &[Project],
    ) -> Result<Vec<ConsistencyConflict>> {
        let mut conflicts = Vec::new();
        let mut world_element_data: HashMap<String, Vec<WorldElementConsistencyData>> = HashMap::new();
        
        // Collect world element data from all projects
        for project in projects {
            let world_elements = sqlx::query_as::<_, WorldElement>(
                "SELECT * FROM world_elements WHERE project_id = ? AND series_shared = true"
            )
            .bind(&project.id)
            .fetch_all(pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to get world elements: {}", e)))?;
            
            for element in world_elements {
                world_element_data.entry(element.name.clone()).or_default().push(
                    WorldElementConsistencyData {
                        element_name: element.name.clone(),
                        project_id: project.id.clone(),
                        element_type: element.element_type.clone(),
                        description: element.description.clone(),
                        details: element.details.clone(),
                    }
                );
            }
        }
        
        // Check for conflicts between world element versions
        for (element_name, versions) in world_element_data {
            if versions.len() > 1 {
                let element_conflicts = Self::analyze_world_element_conflicts(&element_name, &versions);
                conflicts.extend(element_conflicts);
            }
        }
        
        Ok(conflicts)
    }
    
    /// Check story bible consistency (genre, style, etc.)
    async fn check_story_bible_consistency(
        pool: &Pool<Sqlite>,
        projects: &[Project],
    ) -> Result<Vec<ConsistencyConflict>> {
        let mut conflicts = Vec::new();
        let mut story_bibles = Vec::new();
        
        // Collect story bible data from all projects
        for project in projects {
            if let Ok(Some(story_bible)) = crate::database::operations::story_bible_ops::StoryBibleOps::get_by_project(pool, &project.id).await {
                story_bibles.push((project.clone(), story_bible));
            }
        }
        
        if story_bibles.len() > 1 {
            // Check genre consistency
            let genres: Vec<_> = story_bibles.iter()
                .filter_map(|(_, sb)| sb.genre.as_ref())
                .collect();
            
            if Self::has_conflicts(&genres) {
                conflicts.push(ConsistencyConflict {
                    conflict_type: ConflictType::GenreConflict,
                    severity: ConflictSeverity::Medium,
                    description: "Different genres detected across series projects".to_string(),
                    affected_projects: story_bibles.iter().map(|(p, _)| p.id.clone()).collect(),
                    affected_elements: genres.into_iter().map(|g| g.clone()).collect(),
                    suggestions: vec![
                        "Consider standardizing the genre across all projects in the series".to_string(),
                        "If different genres are intentional, ensure they complement each other".to_string(),
                    ],
                });
            }
            
            // Check style consistency
            let styles: Vec<_> = story_bibles.iter()
                .filter_map(|(_, sb)| sb.style.as_ref())
                .collect();
            
            if Self::has_conflicts(&styles) {
                conflicts.push(ConsistencyConflict {
                    conflict_type: ConflictType::StyleMismatch,
                    severity: ConflictSeverity::Medium,
                    description: "Different writing styles detected across series projects".to_string(),
                    affected_projects: story_bibles.iter().map(|(p, _)| p.id.clone()).collect(),
                    affected_elements: styles.into_iter().map(|s| s.clone()).collect(),
                    suggestions: vec![
                        "Consider maintaining consistent writing style across the series".to_string(),
                        "If style evolution is intentional, ensure smooth transitions".to_string(),
                    ],
                });
            }
        }
        
        Ok(conflicts)
    }
    
    /// Analyze conflicts between character versions
    fn analyze_character_conflicts(
        character_name: &str,
        versions: &[CharacterConsistencyData],
    ) -> Vec<ConsistencyConflict> {
        let mut conflicts = Vec::new();
        
        // Check for conflicting trait types
        let mut trait_conflicts = HashMap::new();
        
        for version in versions {
            for trait in &version.traits {
                trait_conflicts.entry(&trait.trait_type)
                    .or_insert_with(Vec::new)
                    .push((version.project_id.clone(), trait.content.clone()));
            }
        }
        
        for (trait_type, trait_versions) in trait_conflicts {
            if trait_versions.len() > 1 {
                // Check if the content is significantly different
                let contents: Vec<_> = trait_versions.iter().map(|(_, content)| content).collect();
                if Self::has_significant_content_differences(&contents) {
                    conflicts.push(ConsistencyConflict {
                        conflict_type: ConflictType::CharacterInconsistency,
                        severity: ConflictSeverity::High,
                        description: format!(
                            "Character '{}' has conflicting '{}' traits across projects",
                            character_name, trait_type
                        ),
                        affected_projects: trait_versions.iter().map(|(pid, _)| pid.clone()).collect(),
                        affected_elements: vec![character_name.to_string(), trait_type.clone()],
                        suggestions: vec![
                            "Review and reconcile the conflicting character traits".to_string(),
                            "Consider if the differences represent character development over time".to_string(),
                            "Update traits to maintain consistency or mark as intentional variations".to_string(),
                        ],
                    });
                }
            }
        }
        
        conflicts
    }
    
    /// Analyze conflicts between world element versions
    fn analyze_world_element_conflicts(
        element_name: &str,
        versions: &[WorldElementConsistencyData],
    ) -> Vec<ConsistencyConflict> {
        let mut conflicts = Vec::new();
        
        // Check for conflicting descriptions
        let descriptions: Vec<_> = versions.iter().map(|v| &v.description).collect();
        if Self::has_significant_content_differences(&descriptions) {
            conflicts.push(ConsistencyConflict {
                conflict_type: ConflictType::WorldElementInconsistency,
                severity: ConflictSeverity::High,
                description: format!(
                    "World element '{}' has conflicting descriptions across projects",
                    element_name
                ),
                affected_projects: versions.iter().map(|v| v.project_id.clone()).collect(),
                affected_elements: vec![element_name.to_string()],
                suggestions: vec![
                    "Review and reconcile the conflicting world element descriptions".to_string(),
                    "Ensure world-building consistency across the series".to_string(),
                    "Consider if differences represent world evolution over time".to_string(),
                ],
            });
        }
        
        // Check for conflicting element types
        let element_types: Vec<_> = versions.iter().map(|v| &v.element_type).collect();
        if Self::has_conflicts(&element_types) {
            conflicts.push(ConsistencyConflict {
                conflict_type: ConflictType::WorldElementInconsistency,
                severity: ConflictSeverity::Medium,
                description: format!(
                    "World element '{}' has different types across projects",
                    element_name
                ),
                affected_projects: versions.iter().map(|v| v.project_id.clone()).collect(),
                affected_elements: vec![element_name.to_string()],
                suggestions: vec![
                    "Standardize the element type across projects".to_string(),
                    "Ensure consistent categorization of world elements".to_string(),
                ],
            });
        }
        
        conflicts
    }
    
    /// Calculate overall consistency score
    fn calculate_consistency_score(conflicts: &[ConsistencyConflict], project_count: usize) -> f64 {
        if conflicts.is_empty() {
            return 1.0;
        }
        
        let total_severity_points: f64 = conflicts.iter().map(|c| {
            match c.severity {
                ConflictSeverity::Critical => 4.0,
                ConflictSeverity::High => 3.0,
                ConflictSeverity::Medium => 2.0,
                ConflictSeverity::Low => 1.0,
            }
        }).sum();
        
        // Normalize based on project count and conflict severity
        let max_possible_points = (project_count as f64) * 4.0 * 10.0; // Assume max 10 conflicts per project
        let score = 1.0 - (total_severity_points / max_possible_points).min(1.0);
        
        score.max(0.0)
    }
    
    /// Extract character name from trait (simplified implementation)
    fn extract_character_name_from_trait(trait: &CharacterTrait) -> String {
        // This is a simplified implementation
        // In a real scenario, you might want to maintain a character name mapping
        // or extract names from trait content using NLP
        format!("Character_{}", trait.character_id)
    }
    
    /// Check if a collection of strings has conflicts (different values)
    fn has_conflicts<T: PartialEq>(items: &[T]) -> bool {
        if items.len() <= 1 {
            return false;
        }
        
        let first = &items[0];
        items.iter().any(|item| item != first)
    }
    
    /// Check if content has significant differences (simplified implementation)
    fn has_significant_content_differences(contents: &[&String]) -> bool {
        if contents.len() <= 1 {
            return false;
        }
        
        // Simple implementation: check if any content is significantly different
        // In a real scenario, you might use text similarity algorithms
        let first = contents[0];
        contents.iter().any(|content| {
            let similarity = Self::calculate_text_similarity(first, content);
            similarity < 0.7 // Threshold for significant difference
        })
    }
    
    /// Calculate text similarity (simplified implementation)
    fn calculate_text_similarity(text1: &str, text2: &str) -> f64 {
        if text1 == text2 {
            return 1.0;
        }
        
        // Simple Jaccard similarity based on words
        let words1: std::collections::HashSet<&str> = text1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = text2.split_whitespace().collect();
        
        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();
        
        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }
    
    /// Get quick consistency status for a series
    pub async fn get_consistency_status(
        pool: &Pool<Sqlite>,
        series_id: &str,
    ) -> Result<(f64, usize)> {
        let report = Self::generate_consistency_report(pool, series_id).await?;
        Ok((report.consistency_score, report.conflicts.len()))
    }
    
    /// Get conflicts by severity
    pub async fn get_conflicts_by_severity(
        pool: &Pool<Sqlite>,
        series_id: &str,
        severity: ConflictSeverity,
    ) -> Result<Vec<ConsistencyConflict>> {
        let report = Self::generate_consistency_report(pool, series_id).await?;
        Ok(report.conflicts.into_iter()
            .filter(|c| matches!(c.severity, severity))
            .collect())
    }
}