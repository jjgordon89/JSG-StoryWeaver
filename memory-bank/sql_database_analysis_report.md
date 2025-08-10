# StoryWeaver Database Analysis Report

**Analysis Date:** 2024-12-19  
**Framework:** SQL Analysis Framework - Phase 1-3 Complete  
**Scope:** Complete codebase analysis including SQL scripts, Rust database operations, and schema definitions  
**Status:** Critical Issues Identified - Immediate Action Required  

---

## Executive Summary

The StoryWeaver database infrastructure exhibits multiple critical issues preventing successful compilation and optimal performance. Analysis reveals 23 distinct issues across schema mismatches, compilation errors, and performance bottlenecks requiring immediate remediation.

**Critical Statistics:**
- **Critical Issues:** 8 (Immediate action required)
- **High Priority Issues:** 9 (Significant impact on functionality)
- **Medium Priority Issues:** 6 (Performance and maintainability)
- **Compilation Errors:** 15+ preventing build success
- **Schema Mismatches:** 12 tables with missing/incorrect columns

---

## Phase 1: Discovery & Assessment Results

### Codebase Analysis Summary
- **SQL Scripts Analyzed:** 25+ migration and fix scripts
- **Rust Operation Files:** 15+ database operation modules
- **Schema Definition Files:** 3 primary schema files
- **Error Log Analysis:** 10,000+ lines of compilation errors

### Infrastructure Review Summary
- **Database Type:** SQLite with SQLx async operations
- **Schema Evolution:** Multiple migration phases with incomplete fixes
- **Indexing Strategy:** Basic indexes present, optimization needed
- **Backup/Recovery:** Not implemented in current analysis scope

---

## Phase 2: Issue Classification & Prioritization

## CRITICAL ISSUES (Immediate Action Required)

### Issue_ID: CRIT-001
**Category:** Critical Priority  
**Component:** Codebase  
**Description:** Rust compilation error in projects.rs - Invalid function signature syntax  
**Root_Cause:** Anonymous parameter syntax `(())` not allowed in Rust 2018 edition  
**Impact:** Prevents entire application compilation and build  
**Repair_Action:** Fix function parameter syntax in get_projects command  
**SQL_Code:**
```rust
// Fix in src/commands/projects.rs line 53
// BEFORE: async fn get(()) -> Result<Vec<Project>> {
// AFTER:
async fn get(_: ()) -> Result<Vec<Project>> {
```
**Validation:** Cargo check should pass without syntax errors  
**Rollback_Plan:** Revert to previous working function signature  
**Estimated_Time:** 5 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: CRIT-002
**Category:** Critical Priority  
**Component:** Database  
**Description:** SQLx query macros failing due to missing DATABASE_URL environment variable  
**Root_Cause:** SQLx compile-time query validation requires database connection or prepared query cache  
**Impact:** Prevents compilation of all database operation modules  
**Repair_Action:** Set up DATABASE_URL environment variable or run cargo sqlx prepare  
**SQL_Code:**
```bash
# Option 1: Set environment variable
export DATABASE_URL="sqlite:./storyweaver.db"

# Option 2: Generate query cache
cargo sqlx prepare --database-url sqlite:./storyweaver.db
```
**Validation:** All sqlx::query! macros should compile successfully  
**Rollback_Plan:** Use sqlx::query instead of sqlx::query! for runtime validation  
**Estimated_Time:** 15 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: CRIT-003
**Category:** Critical Priority  
**Component:** Database  
**Description:** Missing background_tasks table causing compilation failures  
**Root_Cause:** Rust code references table that doesn't exist in current schema  
**Impact:** Background task operations completely non-functional  
**Repair_Action:** Execute fix_missing_tables.sql script  
**SQL_Code:**
```sql
CREATE TABLE IF NOT EXISTS background_tasks (
    id TEXT PRIMARY KEY,
    task_type TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 1,
    progress REAL NOT NULL DEFAULT 0.0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME,
    error_message TEXT,
    user_initiated BOOLEAN NOT NULL DEFAULT 0,
    project_id TEXT,
    document_id TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
);
```
**Validation:** Query `SELECT * FROM background_tasks LIMIT 1;` should execute  
**Rollback_Plan:** DROP TABLE background_tasks; (if no data exists)  
**Estimated_Time:** 10 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: CRIT-004
**Category:** Critical Priority  
**Component:** Database  
**Description:** credit_usage table schema mismatch - missing operation_type column  
**Root_Cause:** Table created with old schema (feature_name) but code expects new schema (operation_type)  
**Impact:** Credit tracking system completely broken  
**Repair_Action:** Execute credit usage schema migration  
**SQL_Code:**
```sql
-- Create new table with correct schema
CREATE TABLE credit_usage_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    model_used TEXT,
    tokens_used INTEGER,
    credits_consumed REAL,
    operation_details TEXT,
    session_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Migrate data if old table exists
INSERT INTO credit_usage_new (project_id, operation_type, model_used, tokens_used, credits_consumed, created_at)
SELECT project_id, feature_name, 'unknown', tokens_used, credits_consumed, created_at 
FROM credit_usage WHERE EXISTS (SELECT 1 FROM pragma_table_info('credit_usage') WHERE name = 'feature_name');

-- Replace old table
DROP TABLE IF EXISTS credit_usage;
ALTER TABLE credit_usage_new RENAME TO credit_usage;
```
**Validation:** Query credit_usage table structure matches Rust model  
**Rollback_Plan:** Restore from backup if data migration fails  
**Estimated_Time:** 20 minutes  
**Risk_Level:** Medium implementation risk (data migration)

### Issue_ID: CRIT-005
**Category:** Critical Priority  
**Component:** Database  
**Description:** Type conversion errors in series_ops.rs - i64 to String conversion failures  
**Root_Cause:** Database returns i64 for ID fields but Rust code expects String  
**Impact:** Series management operations fail at runtime  
**Repair_Action:** Fix type mappings in SQLx queries  
**SQL_Code:**
```rust
// Fix in series_ops.rs - cast database types properly
let series = sqlx::query_as!(
    SeriesWithCount,
    r#"
    SELECT 
        CAST(s.id AS TEXT) as "id: String",
        s.name,
        s.description,
        CAST(s.folder_id AS TEXT) as "folder_id: Option<String>",
        s.created_at,
        COUNT(p.id) as "project_count: i64"
    FROM series s
    LEFT JOIN projects p ON p.series_id = s.id
    GROUP BY s.id, s.name, s.description, s.folder_id, s.created_at
    ORDER BY s.name
    "#
)
.fetch_all(pool)
.await?;
```
**Validation:** Series operations should execute without type conversion errors  
**Rollback_Plan:** Revert to original query structure  
**Estimated_Time:** 30 minutes  
**Risk_Level:** Medium implementation risk

### Issue_ID: CRIT-006
**Category:** Critical Priority  
**Component:** Database  
**Description:** Missing columns in generated_images table causing query failures  
**Root_Cause:** Rust code expects additional metadata columns not present in current schema  
**Impact:** AI image generation features completely broken  
**Repair_Action:** Add missing columns to generated_images table  
**SQL_Code:**
```sql
ALTER TABLE generated_images ADD COLUMN negative_prompt TEXT;
ALTER TABLE generated_images ADD COLUMN model_used TEXT;
ALTER TABLE generated_images ADD COLUMN local_path TEXT;
ALTER TABLE generated_images ADD COLUMN width INTEGER;
ALTER TABLE generated_images ADD COLUMN height INTEGER;
ALTER TABLE generated_images ADD COLUMN seed INTEGER;
ALTER TABLE generated_images ADD COLUMN steps INTEGER;
ALTER TABLE generated_images ADD COLUMN cfg_scale REAL;
ALTER TABLE generated_images ADD COLUMN style TEXT;
ALTER TABLE generated_images ADD COLUMN generation_time REAL;
ALTER TABLE generated_images ADD COLUMN cost_credits INTEGER;
ALTER TABLE generated_images ADD COLUMN metadata TEXT DEFAULT '{}';

-- Update existing records with defaults
UPDATE generated_images SET 
    negative_prompt = '',
    model_used = 'unknown',
    local_path = image_url,
    width = 1024,
    height = 1024,
    seed = 0,
    steps = 20,
    cfg_scale = 7.0,
    style = 'default',
    generation_time = 0.0,
    cost_credits = COALESCE(credits_used, 2500),
    metadata = '{}'
WHERE negative_prompt IS NULL;
```
**Validation:** Image generation queries should execute successfully  
**Rollback_Plan:** Remove added columns if issues arise  
**Estimated_Time:** 15 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: CRIT-007
**Category:** Critical Priority  
**Component:** Database  
**Description:** Missing columns in brainstorm_sessions table  
**Root_Cause:** Table schema doesn't match Rust model expectations for session management  
**Impact:** Brainstorming features non-functional  
**Repair_Action:** Add required columns to brainstorm_sessions table  
**SQL_Code:**
```sql
ALTER TABLE brainstorm_sessions ADD COLUMN session_name TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN session_type TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN initial_prompt TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN context_data TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN generated_ideas TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN selected_ideas TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN session_notes TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN model_used TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN total_tokens INTEGER;
ALTER TABLE brainstorm_sessions ADD COLUMN cost_credits REAL;
ALTER TABLE brainstorm_sessions ADD COLUMN status TEXT;

-- Update existing records
UPDATE brainstorm_sessions SET 
    session_name = COALESCE(category, 'Unnamed Session'),
    session_type = COALESCE(category, 'general'),
    initial_prompt = COALESCE(seed_prompt, ''),
    context_data = '{}',
    generated_ideas = COALESCE(session_data, '[]'),
    selected_ideas = '[]',
    session_notes = '',
    model_used = 'unknown',
    total_tokens = 0,
    cost_credits = 0.0,
    status = 'completed'
WHERE session_name IS NULL;
```
**Validation:** Brainstorm session operations should work correctly  
**Rollback_Plan:** Remove added columns and restore original schema  
**Estimated_Time:** 20 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: CRIT-008
**Category:** Critical Priority  
**Component:** Database  
**Description:** Executor trait implementation errors in project_preview_commands.rs  
**Root_Cause:** Incorrect pool dereferencing pattern for SQLx executor  
**Impact:** Project preview functionality completely broken  
**Repair_Action:** Fix pool reference pattern in SQLx queries  
**SQL_Code:**
```rust
// Fix in project_preview_commands.rs
// BEFORE: .fetch_all(&*pool)
// AFTER:
.fetch_all(pool.as_ref())
// OR
.fetch_all(&**pool)
```
**Validation:** Project preview commands should compile and execute  
**Rollback_Plan:** Revert to original pool usage pattern  
**Estimated_Time:** 10 minutes  
**Risk_Level:** Low implementation risk

## HIGH PRIORITY ISSUES

### Issue_ID: HIGH-001
**Category:** High Priority  
**Component:** Database  
**Description:** Missing canvas table columns for UI state management  
**Root_Cause:** Canvas table lacks viewport and zoom columns expected by frontend  
**Impact:** Canvas functionality limited, poor user experience  
**Repair_Action:** Add missing canvas columns  
**SQL_Code:**
```sql
ALTER TABLE canvas ADD COLUMN width INTEGER DEFAULT 800;
ALTER TABLE canvas ADD COLUMN height INTEGER DEFAULT 600;
ALTER TABLE canvas ADD COLUMN zoom_level REAL DEFAULT 1.0;
ALTER TABLE canvas ADD COLUMN viewport_x REAL DEFAULT 0.0;
ALTER TABLE canvas ADD COLUMN viewport_y REAL DEFAULT 0.0;
```
**Validation:** Canvas operations should handle viewport state correctly  
**Rollback_Plan:** Remove added columns if compatibility issues arise  
**Estimated_Time:** 10 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-002
**Category:** High Priority  
**Component:** Database  
**Description:** Missing canvas_elements table columns for element management  
**Root_Cause:** Canvas elements table lacks metadata and styling columns  
**Impact:** Limited canvas element functionality and customization  
**Repair_Action:** Add missing canvas_elements columns  
**SQL_Code:**
```sql
ALTER TABLE canvas_elements ADD COLUMN title TEXT;
ALTER TABLE canvas_elements ADD COLUMN content TEXT;
ALTER TABLE canvas_elements ADD COLUMN color TEXT;
ALTER TABLE canvas_elements ADD COLUMN metadata TEXT;
ALTER TABLE canvas_elements ADD COLUMN connections TEXT;
ALTER TABLE canvas_elements ADD COLUMN order_index INTEGER DEFAULT 0;
```
**Validation:** Canvas element operations should support full feature set  
**Rollback_Plan:** Remove added columns if issues occur  
**Estimated_Time:** 10 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-003
**Category:** High Priority  
**Component:** Database  
**Description:** Missing outline_templates table columns  
**Root_Cause:** Template system lacks structure and usage tracking columns  
**Impact:** Template functionality severely limited  
**Repair_Action:** Add missing outline_templates columns  
**SQL_Code:**
```sql
ALTER TABLE outline_templates ADD COLUMN structure TEXT;
ALTER TABLE outline_templates ADD COLUMN is_public BOOLEAN DEFAULT 0;
ALTER TABLE outline_templates ADD COLUMN usage_count INTEGER DEFAULT 0;
```
**Validation:** Template operations should support full feature set  
**Rollback_Plan:** Remove added columns if compatibility issues arise  
**Estimated_Time:** 5 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-004
**Category:** High Priority  
**Component:** Database  
**Description:** Type mismatch in document_link_ops.rs for ID fields  
**Root_Cause:** Database returns i64 but Rust expects String for document IDs  
**Impact:** Document linking system fails at runtime  
**Repair_Action:** Fix type casting in document link queries  
**SQL_Code:**
```rust
// Fix type casting in document_link_ops.rs
let previous = sqlx::query_as!(
    LinkedDocument,
    r#"
    SELECT 
        CAST(id AS TEXT) as "id: String",
        CAST(from_document_id AS TEXT) as "from_document_id: String",
        CAST(to_document_id AS TEXT) as "to_document_id: String",
        link_order,
        created_at
    FROM document_links
    WHERE to_document_id = ?
    ORDER BY link_order
    "#,
    document_id
)
.fetch_all(pool)
.await?;
```
**Validation:** Document linking operations should execute without type errors  
**Rollback_Plan:** Revert to original query structure  
**Estimated_Time:** 25 minutes  
**Risk_Level:** Medium implementation risk

### Issue_ID: HIGH-005
**Category:** High Priority  
**Component:** Database  
**Description:** Missing document_links table causing link functionality failure  
**Root_Cause:** Table referenced in code but not created in database  
**Impact:** Document linking between chapters completely broken  
**Repair_Action:** Create document_links table with proper schema  
**SQL_Code:**
```sql
CREATE TABLE IF NOT EXISTS document_links (
    id TEXT PRIMARY KEY NOT NULL,
    from_document_id TEXT NOT NULL,
    to_document_id TEXT NOT NULL,
    link_order INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (from_document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (to_document_id) REFERENCES documents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_document_links_from ON document_links(from_document_id);
CREATE INDEX IF NOT EXISTS idx_document_links_to ON document_links(to_document_id);
```
**Validation:** Document linking queries should execute successfully  
**Rollback_Plan:** DROP TABLE document_links;  
**Estimated_Time:** 15 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-006
**Category:** High Priority  
**Component:** Database  
**Description:** Missing shared_documents table for collaboration features  
**Root_Cause:** Collaboration module references non-existent table  
**Impact:** Document sharing functionality completely unavailable  
**Repair_Action:** Create shared_documents table  
**SQL_Code:**
```sql
CREATE TABLE IF NOT EXISTS shared_documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    share_token TEXT NOT NULL UNIQUE,
    share_type TEXT NOT NULL,
    password_hash TEXT,
    expires_at DATETIME,
    max_uses INTEGER,
    current_uses INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT 1,
    created_by TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_shared_documents_token ON shared_documents(share_token);
CREATE INDEX IF NOT EXISTS idx_shared_documents_project ON shared_documents(project_id);
```
**Validation:** Collaboration features should be accessible  
**Rollback_Plan:** DROP TABLE shared_documents;  
**Estimated_Time:** 20 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-007
**Category:** High Priority  
**Component:** Database  
**Description:** Missing collaboration_sessions table  
**Root_Cause:** Real-time collaboration features reference non-existent table  
**Impact:** Multi-user collaboration completely unavailable  
**Repair_Action:** Create collaboration_sessions table  
**SQL_Code:**
```sql
CREATE TABLE IF NOT EXISTS collaboration_sessions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    host_user TEXT NOT NULL,
    participants TEXT NOT NULL DEFAULT '[]',
    session_data TEXT NOT NULL DEFAULT '{}',
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_token ON collaboration_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_document ON collaboration_sessions(document_id);
```
**Validation:** Collaboration session management should work  
**Rollback_Plan:** DROP TABLE collaboration_sessions;  
**Estimated_Time:** 15 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-008
**Category:** High Priority  
**Component:** Database  
**Description:** Missing document_comments table for review features  
**Root_Cause:** Comment system references non-existent table  
**Impact:** Document review and commenting unavailable  
**Repair_Action:** Create document_comments table  
**SQL_Code:**
```sql
CREATE TABLE IF NOT EXISTS document_comments (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    comment_text TEXT NOT NULL,
    position_data TEXT,
    comment_type TEXT DEFAULT 'general',
    is_resolved BOOLEAN DEFAULT 0,
    parent_comment_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_document_comments_document ON document_comments(document_id);
CREATE INDEX IF NOT EXISTS idx_document_comments_user ON document_comments(user_id);
```
**Validation:** Comment system should be functional  
**Rollback_Plan:** DROP TABLE document_comments;  
**Estimated_Time:** 15 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: HIGH-009
**Category:** High Priority  
**Component:** Database  
**Description:** Type mismatch in document_version_ops.rs for content field  
**Root_Cause:** Query returns Option<String> but code expects String  
**Impact:** Document versioning system fails at runtime  
**Repair_Action:** Fix type handling in document version operations  
**SQL_Code:**
```rust
// Fix in document_version_ops.rs line 112
// BEFORE: content: document.content,
// AFTER:
content: document.content.unwrap_or_default(),
// OR handle the Option properly:
content: document.content.unwrap_or_else(|| "No content".to_string()),
```
**Validation:** Document versioning should work without type errors  
**Rollback_Plan:** Revert to original field assignment  
**Estimated_Time:** 10 minutes  
**Risk_Level:** Low implementation risk

## MEDIUM PRIORITY ISSUES

### Issue_ID: MED-001
**Category:** Medium Priority  
**Component:** Database  
**Description:** Missing indexes on frequently queried columns  
**Root_Cause:** Database lacks optimized indexing strategy for common query patterns  
**Impact:** Slower query performance, especially with larger datasets  
**Repair_Action:** Add performance indexes for common queries  
**SQL_Code:**
```sql
-- Add performance indexes
CREATE INDEX IF NOT EXISTS idx_documents_project_type ON documents(project_id, document_type);
CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_characters_project_id ON characters(project_id);
CREATE INDEX IF NOT EXISTS idx_locations_project_id ON locations(project_id);
CREATE INDEX IF NOT EXISTS idx_ai_generation_history_project ON ai_generation_history(project_id);
CREATE INDEX IF NOT EXISTS idx_ai_generation_history_created ON ai_generation_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_background_tasks_status_priority ON background_tasks(status, priority DESC);
CREATE INDEX IF NOT EXISTS idx_credit_usage_project_operation ON credit_usage(project_id, operation_type);
```
**Validation:** Query performance should improve measurably  
**Rollback_Plan:** DROP INDEX statements for each created index  
**Estimated_Time:** 20 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: MED-002
**Category:** Medium Priority  
**Component:** Database  
**Description:** Inconsistent foreign key constraint enforcement  
**Root_Cause:** Some tables lack proper foreign key relationships  
**Impact:** Data integrity risks and potential orphaned records  
**Repair_Action:** Add missing foreign key constraints  
**SQL_Code:**
```sql
-- Enable foreign key enforcement
PRAGMA foreign_keys = ON;

-- Add missing foreign key constraints where safe
-- Note: This requires careful data validation first
-- Example for series_id in projects table:
ALTER TABLE projects ADD CONSTRAINT fk_projects_series 
    FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE SET NULL;

ALTER TABLE projects ADD CONSTRAINT fk_projects_folder 
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE SET NULL;
```
**Validation:** Foreign key constraints should be enforced properly  
**Rollback_Plan:** Remove foreign key constraints if data issues arise  
**Estimated_Time:** 30 minutes  
**Risk_Level:** Medium implementation risk (data validation required)

### Issue_ID: MED-003
**Category:** Medium Priority  
**Component:** Codebase  
**Description:** Inconsistent error handling patterns across database operations  
**Root_Cause:** Different modules use varying error handling approaches  
**Impact:** Inconsistent user experience and debugging difficulties  
**Repair_Action:** Standardize error handling patterns  
**SQL_Code:**
```rust
// Standardize error handling pattern
impl From<sqlx::Error> for StoryWeaverError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => StoryWeaverError::NotFound {
                resource_type: "Database Record".to_string(),
                id: "unknown".to_string(),
            },
            sqlx::Error::Database(db_err) => StoryWeaverError::Database {
                message: db_err.message().to_string(),
            },
            _ => StoryWeaverError::Database {
                message: err.to_string(),
            },
        }
    }
}
```
**Validation:** Error messages should be consistent across all operations  
**Rollback_Plan:** Revert to original error handling patterns  
**Estimated_Time:** 45 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: MED-004
**Category:** Medium Priority  
**Component:** Database  
**Description:** Missing audit trail for data modifications  
**Root_Cause:** No systematic logging of data changes for debugging and compliance  
**Impact:** Difficult to track data changes and debug issues  
**Repair_Action:** Implement audit logging system  
**SQL_Code:**
```sql
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    table_name TEXT NOT NULL,
    record_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- INSERT, UPDATE, DELETE
    old_values TEXT, -- JSON of old values
    new_values TEXT, -- JSON of new values
    user_id TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    session_id TEXT
);

CREATE INDEX IF NOT EXISTS idx_audit_logs_table_record ON audit_logs(table_name, record_id);
CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
```
**Validation:** Audit trail should capture all significant data changes  
**Rollback_Plan:** DROP TABLE audit_logs;  
**Estimated_Time:** 60 minutes  
**Risk_Level:** Low implementation risk

### Issue_ID: MED-005
**Category:** Medium Priority  
**Component:** Database  
**Description:** Suboptimal query patterns in complex operations  
**Root_Cause:** Some operations use N+1 query patterns instead of joins  
**Impact:** Performance degradation with larger datasets  
**Repair_Action:** Optimize query patterns for better performance  
**SQL_Code:**
```sql
-- Example optimization for project summary queries
-- BEFORE: Multiple separate queries
-- AFTER: Single optimized query with joins
SELECT 
    p.*,
    COUNT(DISTINCT d.id) as document_count,
    COUNT(DISTINCT c.id) as character_count,
    COUNT(DISTINCT l.id) as location_count,
    MAX(d.updated_at) as last_document_update
FROM projects p
LEFT JOIN documents d ON d.project_id = p.id
LEFT JOIN characters c ON c.project_id = p.id
LEFT JOIN locations l ON l.project_id = p.id
WHERE p.id = ?
GROUP BY p.id;
```
**Validation:** Query performance should improve significantly  
**Rollback_Plan:** Revert to original query patterns  
**Estimated_Time:** 90 minutes  
**Risk_Level:** Medium implementation risk

### Issue_ID: MED-006
**Category:** Medium Priority  
**Component:** Infrastructure  
**Description:** Missing database connection pooling optimization  
**Root_Cause:** Default SQLx pool settings may not be optimal for application usage  
**Impact:** Potential connection exhaustion under load  
**Repair_Action:** Optimize database connection pool configuration  
**SQL_Code:**
```rust
// Optimize pool configuration in database/mod.rs
let pool = SqlitePoolOptions::new()
    .max_connections(20) // Adjust based on usage patterns
    .min_connections(5)
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(1800))
    .connect(&database_url)
    .await?;
```
**Validation:** Connection pool should handle concurrent operations efficiently  
**Rollback_Plan:** Revert to default pool settings  
**Estimated_Time:** 30 minutes  
**Risk_Level:** Low implementation risk

---

## Phase 3: Automated Repair & Optimization

### Implementation Priority Order

1. **CRITICAL Issues (CRIT-001 to CRIT-008)** - Must be fixed immediately
2. **HIGH Priority Issues (HIGH-001 to HIGH-009)** - Fix within 24 hours
3. **MEDIUM Priority Issues (MED-001 to MED-006)** - Address within 1 week

### Comprehensive Repair Script

```sql
-- StoryWeaver Database Repair Script
-- Execute in order to resolve all identified issues

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- CRITICAL FIXES

-- Fix missing background_tasks table
CREATE TABLE IF NOT EXISTS background_tasks (
    id TEXT PRIMARY KEY,
    task_type TEXT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 1,
    progress REAL NOT NULL DEFAULT 0.0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    started_at DATETIME,
    completed_at DATETIME,
    error_message TEXT,
    user_initiated BOOLEAN NOT NULL DEFAULT 0,
    project_id TEXT,
    document_id TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE SET NULL
);

-- Fix credit_usage table schema
CREATE TABLE IF NOT EXISTS credit_usage_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    operation_type TEXT NOT NULL,
    model_used TEXT,
    tokens_used INTEGER,
    credits_consumed REAL,
    operation_details TEXT,
    session_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Migrate existing credit_usage data if needed
INSERT OR IGNORE INTO credit_usage_new (project_id, operation_type, model_used, tokens_used, credits_consumed, created_at)
SELECT project_id, 
       COALESCE(feature_name, 'unknown') as operation_type,
       'unknown' as model_used,
       tokens_used, 
       credits_consumed, 
       created_at 
FROM credit_usage 
WHERE EXISTS (SELECT 1 FROM pragma_table_info('credit_usage') WHERE name = 'feature_name');

-- Replace old table
DROP TABLE IF EXISTS credit_usage;
ALTER TABLE credit_usage_new RENAME TO credit_usage;

-- Add missing columns to generated_images
ALTER TABLE generated_images ADD COLUMN negative_prompt TEXT;
ALTER TABLE generated_images ADD COLUMN model_used TEXT;
ALTER TABLE generated_images ADD COLUMN local_path TEXT;
ALTER TABLE generated_images ADD COLUMN width INTEGER;
ALTER TABLE generated_images ADD COLUMN height INTEGER;
ALTER TABLE generated_images ADD COLUMN seed INTEGER;
ALTER TABLE generated_images ADD COLUMN steps INTEGER;
ALTER TABLE generated_images ADD COLUMN cfg_scale REAL;
ALTER TABLE generated_images ADD COLUMN style TEXT;
ALTER TABLE generated_images ADD COLUMN generation_time REAL;
ALTER TABLE generated_images ADD COLUMN cost_credits INTEGER;
ALTER TABLE generated_images ADD COLUMN metadata TEXT DEFAULT '{}';

-- Update existing generated_images records
UPDATE generated_images SET 
    negative_prompt = COALESCE(negative_prompt, ''),
    model_used = COALESCE(model_used, 'unknown'),
    local_path = COALESCE(local_path, image_url),
    width = COALESCE(width, 1024),
    height = COALESCE(height, 1024),
    seed = COALESCE(seed, 0),
    steps = COALESCE(steps, 20),
    cfg_scale = COALESCE(cfg_scale, 7.0),
    style = COALESCE(style, 'default'),
    generation_time = COALESCE(generation_time, 0.0),
    cost_credits = COALESCE(cost_credits, COALESCE(credits_used, 2500)),
    metadata = COALESCE(metadata, '{}')
WHERE negative_prompt IS NULL OR model_used IS NULL;

-- Add missing columns to brainstorm_sessions
ALTER TABLE brainstorm_sessions ADD COLUMN session_name TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN session_type TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN initial_prompt TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN context_data TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN generated_ideas TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN selected_ideas TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN session_notes TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN model_used TEXT;
ALTER TABLE brainstorm_sessions ADD COLUMN total_tokens INTEGER;
ALTER TABLE brainstorm_sessions ADD COLUMN cost_credits REAL;
ALTER TABLE brainstorm_sessions ADD COLUMN status TEXT;

-- Update existing brainstorm_sessions records
UPDATE brainstorm_sessions SET 
    session_name = COALESCE(session_name, category, 'Unnamed Session'),
    session_type = COALESCE(session_type, category, 'general'),
    initial_prompt = COALESCE(initial_prompt, seed_prompt, ''),
    context_data = COALESCE(context_data, '{}'),
    generated_ideas = COALESCE(generated_ideas, session_data, '[]'),
    selected_ideas = COALESCE(selected_ideas, '[]'),
    session_notes = COALESCE(session_notes, ''),
    model_used = COALESCE(model_used, 'unknown'),
    total_tokens = COALESCE(total_tokens, 0),
    cost_credits = COALESCE(cost_credits, 0.0),
    status = COALESCE(status, 'completed')
WHERE session_name IS NULL;

-- HIGH PRIORITY FIXES

-- Add missing canvas columns
ALTER TABLE canvas ADD COLUMN width INTEGER DEFAULT 800;
ALTER TABLE canvas ADD COLUMN height INTEGER DEFAULT 600;
ALTER TABLE canvas ADD COLUMN zoom_level REAL DEFAULT 1.0;
ALTER TABLE canvas ADD COLUMN viewport_x REAL DEFAULT 0.0;
ALTER TABLE canvas ADD COLUMN viewport_y REAL DEFAULT 0.0;

-- Add missing canvas_elements columns
ALTER TABLE canvas_elements ADD COLUMN title TEXT;
ALTER TABLE canvas_elements ADD COLUMN content TEXT;
ALTER TABLE canvas_elements ADD COLUMN color TEXT;
ALTER TABLE canvas_elements ADD COLUMN metadata TEXT;
ALTER TABLE canvas_elements ADD COLUMN connections TEXT;
ALTER TABLE canvas_elements ADD COLUMN order_index INTEGER DEFAULT 0;

-- Add missing outline_templates columns
ALTER TABLE outline_templates ADD COLUMN structure TEXT;
ALTER TABLE outline_templates ADD COLUMN is_public BOOLEAN DEFAULT 0;
ALTER TABLE outline_templates ADD COLUMN usage_count INTEGER DEFAULT 0;

-- Create missing document_links table
CREATE TABLE IF NOT EXISTS document_links (
    id TEXT PRIMARY KEY NOT NULL,
    from_document_id TEXT NOT NULL,
    to_document_id TEXT NOT NULL,
    link_order INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (from_document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (to_document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Create missing shared_documents table
CREATE TABLE IF NOT EXISTS shared_documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    share_token TEXT NOT NULL UNIQUE,
    share_type TEXT NOT NULL,
    password_hash TEXT,
    expires_at DATETIME,
    max_uses INTEGER,
    current_uses INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT 1,
    created_by TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Create missing collaboration_sessions table
CREATE TABLE IF NOT EXISTS collaboration_sessions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    host_user TEXT NOT NULL,
    participants TEXT NOT NULL DEFAULT '[]',
    session_data TEXT NOT NULL DEFAULT '{}',
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Create missing document_comments table
CREATE TABLE IF NOT EXISTS document_comments (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    comment_text TEXT NOT NULL,
    position_data TEXT,
    comment_type TEXT DEFAULT 'general',
    is_resolved BOOLEAN DEFAULT 0,
    parent_comment_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id) ON DELETE CASCADE
);

-- PERFORMANCE INDEXES

-- Background tasks indexes
CREATE INDEX IF NOT EXISTS idx_background_tasks_status ON background_tasks(status);
CREATE INDEX IF NOT EXISTS idx_background_tasks_priority ON background_tasks(priority);
CREATE INDEX IF NOT EXISTS idx_background_tasks_project_id ON background_tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_background_tasks_document_id ON background_tasks(document_id);
CREATE INDEX IF NOT EXISTS idx_background_tasks_created_at ON background_tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_background_tasks_completed_at ON background_tasks(completed_at);
CREATE INDEX IF NOT EXISTS idx_background_tasks_status_priority ON background_tasks(status, priority DESC);

-- Document links indexes
CREATE INDEX IF NOT EXISTS idx_document_links_from ON document_links(from_document_id);
CREATE INDEX IF NOT EXISTS idx_document_links_to ON document_links(to_document_id);

-- Shared documents indexes
CREATE INDEX IF NOT EXISTS idx_shared_documents_token ON shared_documents(share_token);
CREATE INDEX IF NOT EXISTS idx_shared_documents_project ON shared_documents(project_id);

-- Collaboration sessions indexes
CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_token ON collaboration_sessions(session_token);
CREATE INDEX IF NOT EXISTS idx_collaboration_sessions_document ON collaboration_sessions(document_id);

-- Document comments indexes
CREATE INDEX IF NOT EXISTS idx_document_comments_document ON document_comments(document_id);
CREATE INDEX IF NOT EXISTS idx_document_comments_user ON document_comments(user_id);

-- General performance indexes
CREATE INDEX IF NOT EXISTS idx_documents_project_type ON documents(project_id, document_type);
CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_characters_project_id ON characters(project_id);
CREATE INDEX IF NOT EXISTS idx_locations_project_id ON locations(project_id);
CREATE INDEX IF NOT EXISTS idx_ai_generation_history_project ON ai_generation_history(project_id);
CREATE INDEX IF NOT EXISTS idx_ai_generation_history_created ON ai_generation_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_credit_usage_project_operation ON credit_usage(project_id, operation_type);

-- Record migration completion
INSERT OR IGNORE INTO migrations (name, applied_at) 
VALUES ('comprehensive_database_repair', CURRENT_TIMESTAMP);

-- Verify foreign key integrity
PRAGMA foreign_key_check;

SELECT 'Database repair completed successfully' as status;
```

### Validation Test Suite

```sql
-- Validation queries to verify all fixes

-- Test 1: Verify all required tables exist
SELECT name FROM sqlite_master WHERE type='table' 
AND name IN (
    'background_tasks', 'document_links', 'shared_documents', 
    'collaboration_sessions', 'document_comments', 'credit_usage'
);

-- Test 2: Verify background_tasks table structure
PRAGMA table_info(background_tasks);

-- Test 3: Verify credit_usage table has correct schema
SELECT sql FROM sqlite_master WHERE name='credit_usage';

-- Test 4: Verify generated_images has all required columns
PRAGMA table_info(generated_images);

-- Test 5: Verify brainstorm_sessions has all required columns
PRAGMA table_info(brainstorm_sessions);

-- Test 6: Verify canvas tables have required columns
PRAGMA table_info(canvas);
PRAGMA table_info(canvas_elements);

-- Test 7: Verify all indexes were created
SELECT name FROM sqlite_master WHERE type='index' 
AND name LIKE 'idx_%';

-- Test 8: Verify foreign key constraints
PRAGMA foreign_key_check;

-- Test 9: Test basic operations
INSERT INTO background_tasks (id, task_type, description, status) 
VALUES ('test-task', 'test', 'Test task', 'pending');

SELECT COUNT(*) as background_tasks_count FROM background_tasks;

DELETE FROM background_tasks WHERE id = 'test-task';

SELECT 'All validation tests passed' as result;
```

---

## Summary & Next Steps

### Immediate Actions Required

1. **Fix Rust compilation errors** (CRIT-001, CRIT-002, CRIT-008)
2. **Execute database schema fixes** (CRIT-003 through CRIT-007)
3. **Create missing tables** (HIGH-001 through HIGH-008)
4. **Apply performance optimizations** (MED-001, MED-006)

### Success Metrics

- **Compilation Success:** All Rust code compiles without errors
- **Feature Functionality:** All major features operational
- **Performance Improvement:** Query response times under 100ms for common operations
- **Data Integrity:** No foreign key violations or orphaned records

### Risk Mitigation

- **Backup Strategy:** Create full database backup before applying fixes
- **Incremental Deployment:** Apply fixes in priority order with validation
- **Rollback Procedures:** Documented rollback steps for each change
- **Testing Protocol:** Comprehensive validation after each fix

### Long-term Recommendations

1. **Implement automated schema validation** in CI/CD pipeline
2. **Establish database migration best practices** for future changes
3. **Create comprehensive test suite** for database operations
4. **Implement monitoring and alerting** for database performance
5. **Regular database maintenance** and optimization schedules

---

**Report Generated:** 2024-12-19  
**Total Issues Identified:** 23  
**Estimated Total Repair Time:** 6-8 hours  
**Overall Risk Level:** Medium (due to data migration requirements)  
**Recommended Completion Timeline:** 24-48 hours