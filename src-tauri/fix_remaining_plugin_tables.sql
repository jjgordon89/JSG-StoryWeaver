-- Fix remaining plugin table schema issues
BEGIN TRANSACTION;

-- Fix plugin_execution_history table
CREATE TABLE plugin_execution_history_new (
    id TEXT PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL,
    input_variables TEXT, -- JSON
    output_result TEXT, -- JSON
    execution_time_ms INTEGER,
    success BOOLEAN,
    error_message TEXT,
    executed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE
);

-- Copy existing data, mapping columns appropriately
INSERT INTO plugin_execution_history_new (
    id, plugin_id, user_identifier, input_variables, output_result, 
    execution_time_ms, success, error_message, executed_at
)
SELECT 
    CAST(id AS TEXT), plugin_id, 'system', input_data, output_data,
    execution_time, CASE WHEN status = 'success' THEN 1 ELSE 0 END,
    error_message, executed_at
FROM plugin_execution_history;

-- Replace old table
DROP TABLE plugin_execution_history;
ALTER TABLE plugin_execution_history_new RENAME TO plugin_execution_history;

-- Recreate indexes
CREATE INDEX idx_plugin_execution_plugin ON plugin_execution_history(plugin_id);
CREATE INDEX idx_plugin_execution_user ON plugin_execution_history(user_identifier);
CREATE INDEX idx_plugin_execution_date ON plugin_execution_history(executed_at);

-- Fix plugin_usage_stats table with correct column names
CREATE TABLE plugin_usage_stats_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    plugin_id INTEGER NOT NULL,
    date DATE NOT NULL,
    total_executions INTEGER DEFAULT 0,
    successful_executions INTEGER DEFAULT 0,
    failed_executions INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id) ON DELETE CASCADE,
    UNIQUE(plugin_id, date)
);

-- Copy existing data, mapping columns appropriately
INSERT INTO plugin_usage_stats_new (
    plugin_id, date, total_executions, successful_executions, 
    failed_executions, created_at, updated_at
)
SELECT 
    plugin_id, 
    DATE('now') as date,
    execution_count as total_executions,
    success_count as successful_executions,
    error_count as failed_executions,
    created_at,
    updated_at
FROM plugin_usage_stats;

-- Replace old table
DROP TABLE plugin_usage_stats;
ALTER TABLE plugin_usage_stats_new RENAME TO plugin_usage_stats;

-- Recreate indexes
CREATE INDEX idx_plugin_usage_plugin ON plugin_usage_stats(plugin_id);
CREATE INDEX idx_plugin_usage_date ON plugin_usage_stats(date);

COMMIT;