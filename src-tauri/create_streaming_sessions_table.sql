-- Create the missing streaming_sessions table
-- Based on the StreamingSession struct and SQL queries in streaming_session_ops.rs

CREATE TABLE IF NOT EXISTS streaming_sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL UNIQUE,
    project_id INTEGER NOT NULL,
    model_used TEXT NOT NULL,
    prompt TEXT NOT NULL,
    status TEXT NOT NULL, -- "active", "completed", "error", "cancelled"
    generated_content TEXT,
    tokens_generated INTEGER,
    credits_consumed REAL,
    error_message TEXT,
    metadata TEXT, -- JSON
    started_at DATETIME,
    completed_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_streaming_sessions_session_id ON streaming_sessions(session_id);
CREATE INDEX IF NOT EXISTS idx_streaming_sessions_project_id ON streaming_sessions(project_id);
CREATE INDEX IF NOT EXISTS idx_streaming_sessions_status ON streaming_sessions(status);
CREATE INDEX IF NOT EXISTS idx_streaming_sessions_started_at ON streaming_sessions(started_at);