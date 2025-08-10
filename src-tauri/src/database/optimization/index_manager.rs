//! Database index management for optimal query performance
//! Implements intelligent indexing strategies based on query patterns

use crate::database::{get_pool, DbPool};
use crate::error::{Result, StoryWeaverError};
use sqlx::Row;
use std::collections::HashMap;
use tracing::{info, warn, error};

/// Manages database indexes for optimal performance
pub struct IndexManager {
    pool: std::sync::Arc<DbPool>,
    index_usage_stats: HashMap<String, IndexUsageStats>,
}

#[derive(Debug, Clone)]
pub struct IndexUsageStats {
    pub index_name: String,
    pub table_name: String,
    pub usage_count: u64,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub effectiveness_score: f64,
}

#[derive(Debug, Clone)]
pub struct IndexRecommendation {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub index_type: IndexType,
    pub priority: IndexPriority,
    pub estimated_benefit: f64,
}

#[derive(Debug, Clone)]
pub enum IndexType {
    BTree,
    Unique,
    Composite,
    PartialIndex,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndexPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl IndexManager {
    /// Create a new IndexManager instance
    pub async fn new() -> Result<Self> {
        let pool = get_pool()?;
        let mut manager = Self {
            pool,
            index_usage_stats: HashMap::new(),
        };
        
        manager.load_existing_indexes().await?;
        Ok(manager)
    }
    
    /// Create all essential indexes for optimal performance
    pub async fn create_essential_indexes(&self) -> Result<()> {
        info!("Creating essential database indexes for optimal performance");
        
        let essential_indexes = vec![
            // Document-related indexes
            ("idx_documents_project_id", "documents", "project_id"),
            ("idx_documents_type", "documents", "document_type"),
            ("idx_documents_parent_id", "documents", "parent_id"),
            ("idx_documents_updated_at", "documents", "updated_at"),
            ("idx_documents_word_count", "documents", "word_count"),
            
            // Project-related indexes
            ("idx_projects_status", "projects", "status"),
            ("idx_projects_updated_at", "projects", "updated_at"),
            ("idx_projects_genre", "projects", "genre"),
            
            // Character-related indexes
            ("idx_characters_project_id", "characters", "project_id"),
            ("idx_characters_role", "characters", "role"),
            ("idx_characters_series_id", "characters", "series_id"),
            ("idx_characters_original_project_id", "characters", "original_project_id"),
            
            // Location-related indexes
            ("idx_locations_project_id", "locations", "project_id"),
            ("idx_locations_type", "locations", "location_type"),
            
            // AI History indexes
            ("idx_ai_history_project_id", "ai_generation_history", "project_id"),
            ("idx_ai_history_document_id", "ai_generation_history", "document_id"),
            ("idx_ai_history_type", "ai_generation_history", "generation_type"),
            ("idx_ai_history_provider", "ai_generation_history", "provider"),
            ("idx_ai_history_created_at", "ai_generation_history", "created_at"),
            
            // Timeline and plot thread indexes
            ("idx_timeline_events_project_id", "timeline_events", "project_id"),
            ("idx_plot_threads_project_id", "plot_threads", "project_id"),
            
            // Performance monitoring indexes
            ("idx_performance_metrics_component", "performance_metrics", "component"),
            ("idx_performance_metrics_recorded_at", "performance_metrics", "recorded_at"),
            ("idx_memory_snapshots_recorded_at", "memory_snapshots", "recorded_at"),
            ("idx_query_performance_table_name", "query_performance", "table_name"),
            ("idx_query_performance_is_slow", "query_performance", "is_slow"),
        ];
        
        for (index_name, table_name, column_name) in essential_indexes {
            self.create_index_if_not_exists(index_name, table_name, column_name).await?;
        }
        
        // Create composite indexes for common query patterns
        self.create_composite_indexes().await?;
        
        info!("Essential database indexes created successfully");
        Ok(())
    }
    
    /// Create composite indexes for complex queries
    async fn create_composite_indexes(&self) -> Result<()> {
        let composite_indexes = vec![
            // Document project + type composite
            (
                "idx_documents_project_type",
                "documents",
                "(project_id, document_type)"
            ),
            // AI history project + document composite
            (
                "idx_ai_history_project_document",
                "ai_generation_history",
                "(project_id, document_id)"
            ),
            // Characters project + role composite
            (
                "idx_characters_project_role",
                "characters",
                "(project_id, role)"
            ),
            // Performance metrics component + recorded_at composite
            (
                "idx_perf_metrics_component_time",
                "performance_metrics",
                "(component, recorded_at)"
            ),
        ];
        
        for (index_name, table_name, columns) in composite_indexes {
            let sql = format!(
                "CREATE INDEX IF NOT EXISTS {} ON {} {}",
                index_name, table_name, columns
            );
            
            sqlx::query(&sql)
                .execute(&*self.pool)
                .await
                .map_err(|e| {
                    StoryWeaverError::database(format!(
                        "Failed to create composite index {}: {}",
                        index_name, e
                    ))
                })?;
                
            info!("Created composite index: {}", index_name);
        }
        
        Ok(())
    }
    
    /// Create full-text search indexes for content search
    pub async fn create_fulltext_indexes(&self) -> Result<()> {
        info!("Creating full-text search indexes");
        
        // Create FTS virtual tables for content search
        let fts_tables = vec![
            (
                "documents_fts",
                "documents",
                "id, title, content",
                "CREATE VIRTUAL TABLE IF NOT EXISTS documents_fts USING fts5(id, title, content, content='documents', content_rowid='rowid')"
            ),
            (
                "characters_fts",
                "characters",
                "id, name, description, personality, background",
                "CREATE VIRTUAL TABLE IF NOT EXISTS characters_fts USING fts5(id, name, description, personality, background, content='characters', content_rowid='rowid')"
            ),
            (
                "locations_fts",
                "locations",
                "id, name, description, geography, culture, history",
                "CREATE VIRTUAL TABLE IF NOT EXISTS locations_fts USING fts5(id, name, description, geography, culture, history, content='locations', content_rowid='rowid')"
            ),
        ];
        
        for (fts_name, source_table, _columns, create_sql) in fts_tables {
            // Create the FTS table
            sqlx::query(create_sql)
                .execute(&*self.pool)
                .await
                .map_err(|e| {
                    StoryWeaverError::database(format!(
                        "Failed to create FTS table {}: {}",
                        fts_name, e
                    ))
                })?;
            
            // Create triggers to keep FTS table in sync
            self.create_fts_triggers(fts_name, source_table).await?;
            
            info!("Created full-text search index: {}", fts_name);
        }
        
        Ok(())
    }
    
    /// Create triggers to maintain FTS indexes
    async fn create_fts_triggers(&self, fts_table: &str, source_table: &str) -> Result<()> {
        let triggers = vec![
            format!(
                "CREATE TRIGGER IF NOT EXISTS {}_ai AFTER INSERT ON {} BEGIN
                    INSERT INTO {} (rowid, id, name, description) VALUES (new.rowid, new.id, new.name, new.description);
                END",
                fts_table, source_table, fts_table
            ),
            format!(
                "CREATE TRIGGER IF NOT EXISTS {}_ad AFTER DELETE ON {} BEGIN
                    INSERT INTO {} ({}, rowid, id, name, description) VALUES ('delete', old.rowid, old.id, old.name, old.description);
                END",
                fts_table, source_table, fts_table, fts_table
            ),
            format!(
                "CREATE TRIGGER IF NOT EXISTS {}_au AFTER UPDATE ON {} BEGIN
                    INSERT INTO {} ({}, rowid, id, name, description) VALUES ('delete', old.rowid, old.id, old.name, old.description);
                    INSERT INTO {} (rowid, id, name, description) VALUES (new.rowid, new.id, new.name, new.description);
                END",
                fts_table, source_table, fts_table, fts_table
            ),
        ];
        
        for trigger_sql in triggers {
            sqlx::query(&trigger_sql)
                .execute(&*self.pool)
                .await
                .map_err(|e| {
                    StoryWeaverError::database(format!(
                        "Failed to create FTS trigger: {}",
                        e
                    ))
                })?;
        }
        
        Ok(())
    }
    
    /// Create a single index if it doesn't exist
    async fn create_index_if_not_exists(
        &self,
        index_name: &str,
        table_name: &str,
        column_name: &str,
    ) -> Result<()> {
        let sql = format!(
            "CREATE INDEX IF NOT EXISTS {} ON {}({})",
            index_name, table_name, column_name
        );
        
        sqlx::query(&sql)
            .execute(&*self.pool)
            .await
            .map_err(|e| {
                StoryWeaverError::database(format!(
                    "Failed to create index {}: {}",
                    index_name, e
                ))
            })?;
            
        info!("Created index: {} on {}.{}", index_name, table_name, column_name);
        Ok(())
    }
    
    /// Load existing indexes from the database
    async fn load_existing_indexes(&mut self) -> Result<()> {
        let rows = sqlx::query(
            "SELECT name, tbl_name FROM sqlite_master WHERE type = 'index' AND name NOT LIKE 'sqlite_%'"
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to load existing indexes: {}", e)))?;
        
        for row in rows {
            let index_name: String = row.get("name");
            let table_name: String = row.get("tbl_name");
            
            self.index_usage_stats.insert(
                index_name.clone(),
                IndexUsageStats {
                    index_name,
                    table_name,
                    usage_count: 0,
                    last_used: chrono::Utc::now(),
                    effectiveness_score: 0.0,
                },
            );
        }
        
        info!("Loaded {} existing indexes", self.index_usage_stats.len());
        Ok(())
    }
    
    /// Analyze query patterns and recommend new indexes
    pub async fn analyze_and_recommend_indexes(&self) -> Result<Vec<IndexRecommendation>> {
        info!("Analyzing query patterns for index recommendations");
        
        let mut recommendations = Vec::new();
        
        // Analyze slow queries from performance metrics
        let slow_queries = sqlx::query(
            r#"
            SELECT table_name, COUNT(*) as slow_count, AVG(execution_time_ms) as avg_time
            FROM query_performance
            WHERE is_slow = 1
            GROUP BY table_name
            ORDER BY slow_count DESC, avg_time DESC
            LIMIT 10
            "#
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to analyze slow queries: {}", e)))?;
        
        for row in slow_queries {
            let table_name: String = row.get("table_name");
            let slow_count: i64 = row.get("slow_count");
            let avg_time: f64 = row.get("avg_time");
            
            // Calculate priority based on frequency and execution time
            let priority = if slow_count > 10 && avg_time > 100.0 {
                IndexPriority::Critical
            } else if slow_count > 5 && avg_time > 50.0 {
                IndexPriority::High
            } else if slow_count > 2 {
                IndexPriority::Medium
            } else {
                IndexPriority::Low
            };
            
            recommendations.push(IndexRecommendation {
                table_name: table_name.clone(),
                column_names: self.suggest_columns_for_table(&table_name).await?,
                index_type: IndexType::BTree,
                priority,
                estimated_benefit: (slow_count as f64 * avg_time) / 100.0,
            });
        }
        
        info!("Generated {} index recommendations", recommendations.len());
        Ok(recommendations)
    }
    
    /// Suggest optimal columns for indexing based on table structure
    async fn suggest_columns_for_table(&self, table_name: &str) -> Result<Vec<String>> {
        let common_index_columns = match table_name {
            "documents" => vec!["project_id", "document_type", "parent_id", "updated_at"],
            "characters" => vec!["project_id", "role", "series_id"],
            "locations" => vec!["project_id", "location_type"],
            "ai_generation_history" => vec!["project_id", "document_id", "generation_type", "provider"],
            "performance_metrics" => vec!["component", "recorded_at"],
            _ => vec!["id", "created_at", "updated_at"],
        };
        
        Ok(common_index_columns.into_iter().map(String::from).collect())
    }
    
    /// Get index usage statistics
    pub fn get_index_usage_stats(&self) -> &HashMap<String, IndexUsageStats> {
        &self.index_usage_stats
    }
    
    /// Update index usage statistics
    pub fn update_index_usage(&mut self, index_name: &str) {
        if let Some(stats) = self.index_usage_stats.get_mut(index_name) {
            stats.usage_count += 1;
            stats.last_used = chrono::Utc::now();
            stats.effectiveness_score = self.calculate_effectiveness_score(stats);
        }
    }
    
    /// Calculate effectiveness score for an index
    fn calculate_effectiveness_score(&self, stats: &IndexUsageStats) -> f64 {
        let days_since_last_use = (chrono::Utc::now() - stats.last_used).num_days() as f64;
        let usage_frequency = stats.usage_count as f64;
        
        // Higher score for frequently used, recently accessed indexes
        (usage_frequency * 10.0) / (1.0 + days_since_last_use)
    }
    
    /// Remove unused indexes to improve write performance
    pub async fn cleanup_unused_indexes(&self, min_effectiveness_score: f64) -> Result<Vec<String>> {
        let mut removed_indexes = Vec::new();
        
        for (index_name, stats) in &self.index_usage_stats {
            if stats.effectiveness_score < min_effectiveness_score {
                // Don't remove essential indexes
                if !self.is_essential_index(index_name) {
                    let sql = format!("DROP INDEX IF EXISTS {}", index_name);
                    
                    match sqlx::query(&sql).execute(&*self.pool).await {
                        Ok(_) => {
                            info!("Removed unused index: {}", index_name);
                            removed_indexes.push(index_name.clone());
                        }
                        Err(e) => {
                            warn!("Failed to remove index {}: {}", index_name, e);
                        }
                    }
                }
            }
        }
        
        Ok(removed_indexes)
    }
    
    /// Check if an index is essential and should not be removed
    fn is_essential_index(&self, index_name: &str) -> bool {
        let essential_patterns = [
            "idx_documents_project_id",
            "idx_characters_project_id",
            "idx_locations_project_id",
            "idx_ai_history_project_id",
            "idx_projects_status",
            "sqlite_autoindex",
        ];
        
        essential_patterns.iter().any(|pattern| index_name.contains(pattern))
    }
    
    /// Get database statistics for optimization analysis
    pub async fn get_database_stats(&self) -> Result<DatabaseOptimizationStats> {
        let table_stats = sqlx::query(
            r#"
            SELECT 
                name as table_name,
                (SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND tbl_name=m.name) as index_count
            FROM sqlite_master m
            WHERE type='table' AND name NOT LIKE 'sqlite_%'
            "#
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to get table stats: {}", e)))?;
        
        let total_indexes = self.index_usage_stats.len();
        let active_indexes = self.index_usage_stats.values()
            .filter(|stats| stats.usage_count > 0)
            .count();
        
        Ok(DatabaseOptimizationStats {
            total_tables: table_stats.len(),
            total_indexes,
            active_indexes,
            unused_indexes: total_indexes - active_indexes,
            average_effectiveness_score: self.calculate_average_effectiveness_score(),
        })
    }
    
    /// Calculate average effectiveness score across all indexes
    fn calculate_average_effectiveness_score(&self) -> f64 {
        if self.index_usage_stats.is_empty() {
            return 0.0;
        }
        
        let total_score: f64 = self.index_usage_stats.values()
            .map(|stats| stats.effectiveness_score)
            .sum();
            
        total_score / self.index_usage_stats.len() as f64
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseOptimizationStats {
    pub total_tables: usize,
    pub total_indexes: usize,
    pub active_indexes: usize,
    pub unused_indexes: usize,
    pub average_effectiveness_score: f64,
}

/// Initialize database optimization on startup
pub async fn initialize_database_optimization() -> Result<IndexManager> {
    info!("Initializing database optimization");
    
    let index_manager = IndexManager::new().await?;
    
    // Create essential indexes
    index_manager.create_essential_indexes().await?;
    
    // Create full-text search indexes
    index_manager.create_fulltext_indexes().await?;
    
    info!("Database optimization initialized successfully");
    Ok(index_manager)
}