-- Fix documents table and create collaboration_notifications table
-- Add missing folder_id column to documents table and create collaboration_notifications table

BEGIN TRANSACTION;

-- Add missing folder_id column to documents table
ALTER TABLE documents ADD COLUMN folder_id TEXT;

-- Create collaboration_notifications table
CREATE TABLE collaboration_notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    message TEXT NOT NULL,
    recipient_token TEXT,
    is_read BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Create indexes for collaboration_notifications
CREATE INDEX idx_collaboration_notifications_document ON collaboration_notifications(document_id);
CREATE INDEX idx_collaboration_notifications_recipient ON collaboration_notifications(recipient_token);
CREATE INDEX idx_collaboration_notifications_created_at ON collaboration_notifications(created_at);
CREATE INDEX idx_collaboration_notifications_is_read ON collaboration_notifications(is_read);

COMMIT;

-- Verify the changes
.schema documents
.schema collaboration_notifications