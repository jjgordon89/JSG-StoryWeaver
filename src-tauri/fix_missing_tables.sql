-- Fix missing tables and schema issues for StoryWeaver
-- This script addresses compilation errors by creating missing tables and fixing schema mismatches

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- Create missing background_tasks table
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

-- Create indexes for background_tasks
CREATE INDEX IF NOT EXISTS idx_background_tasks_status ON background_tasks(status);
CREATE INDEX IF NOT EXISTS idx_background_tasks_priority ON background_tasks(priority);
CREATE INDEX IF NOT EXISTS idx_background_tasks_project_id ON background_tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_background_tasks_document_id ON background_tasks(document_id);
CREATE INDEX IF NOT EXISTS idx_background_tasks_created_at ON background_tasks(created_at);
CREATE INDEX IF NOT EXISTS idx_background_tasks_completed_at ON background_tasks(completed_at);

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

-- Create missing audit_logs table
CREATE TABLE IF NOT EXISTS audit_logs (
    id INTEGER PRIMARY KEY,
    event_type TEXT NOT NULL,
    category TEXT NOT NULL,
    severity TEXT NOT NULL,
    description TEXT NOT NULL,
    context_data TEXT,
    project_id INTEGER,
    document_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Create index for audit_logs
CREATE INDEX IF NOT EXISTS idx_audit_logs_created_at ON audit_logs(created_at);

-- Add missing series_id column to projects table if it doesn't exist
-- First check if the column exists
PRAGMA table_info(projects);

-- Add series_id column to projects table (this will fail silently if column already exists)
-- SQLite doesn't have IF NOT EXISTS for ALTER TABLE, so we'll handle this in the application
-- For now, we'll add it and ignore errors
ALTER TABLE projects ADD COLUMN series_id TEXT;

-- Create foreign key index for series_id
CREATE INDEX IF NOT EXISTS idx_projects_series_id ON projects(series_id);

-- Fix generated_images table to add prompt column as alias to image_prompt
-- Since SQLite doesn't support column renaming easily, we'll add a view or trigger
-- For now, let's add the prompt column and copy data
ALTER TABLE generated_images ADD COLUMN prompt TEXT;

-- Copy image_prompt data to prompt column
UPDATE generated_images SET prompt = image_prompt WHERE prompt IS NULL;

-- Create series table if it doesn't exist (referenced by projects.series_id)
CREATE TABLE IF NOT EXISTS series (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Apply Phase 5 collaboration tables from the migration file
-- These might be missing from the current database

-- Shared documents for collaboration
CREATE TABLE IF NOT EXISTS shared_documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    project_id TEXT NOT NULL,
    share_token TEXT NOT NULL UNIQUE,
    share_type TEXT NOT NULL DEFAULT 'view', -- 'view', 'comment', 'edit'
    password_hash TEXT,
    expires_at DATETIME,
    max_uses INTEGER,
    current_uses INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    created_by TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Collaboration sessions for real-time editing
CREATE TABLE IF NOT EXISTS collaboration_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    session_token TEXT NOT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    max_participants INTEGER NOT NULL DEFAULT 5,
    current_participants INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Document comments for collaboration
CREATE TABLE IF NOT EXISTS document_comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    parent_comment_id INTEGER,
    author_name TEXT NOT NULL,
    author_email TEXT,
    content TEXT NOT NULL,
    position_start INTEGER,
    position_end INTEGER,
    is_resolved BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id) ON DELETE CASCADE
);

PRAGMA foreign_key_check;