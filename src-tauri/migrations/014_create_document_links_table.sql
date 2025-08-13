-- Migration 014: Create document_links table
CREATE TABLE IF NOT EXISTS document_links (
    id TEXT PRIMARY KEY,
    from_document_id TEXT NOT NULL,
    to_document_id TEXT NOT NULL,
    link_order INTEGER NOT NULL DEFAULT 1,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (from_document_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (to_document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Indexes to support lookups
CREATE INDEX IF NOT EXISTS idx_document_links_from ON document_links(from_document_id);
CREATE INDEX IF NOT EXISTS idx_document_links_to ON document_links(to_document_id);

-- Record migration (if the migrations table exists already this will be ignored)
INSERT OR IGNORE INTO migrations (name) VALUES ('014_create_document_links_table');
