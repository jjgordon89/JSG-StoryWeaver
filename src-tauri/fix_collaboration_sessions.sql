-- Fix collaboration_sessions table schema
-- Add missing columns to collaboration_sessions table

BEGIN TRANSACTION;

-- Add missing columns to collaboration_sessions table
ALTER TABLE collaboration_sessions ADD COLUMN max_participants INTEGER DEFAULT 10;
ALTER TABLE collaboration_sessions ADD COLUMN session_name TEXT;
ALTER TABLE collaboration_sessions ADD COLUMN allow_anonymous BOOLEAN DEFAULT 0;
ALTER TABLE collaboration_sessions ADD COLUMN expires_at DATETIME;

-- Update existing records with default values
UPDATE collaboration_sessions SET max_participants = 10 WHERE max_participants IS NULL;
UPDATE collaboration_sessions SET allow_anonymous = 0 WHERE allow_anonymous IS NULL;

COMMIT;

-- Verify the changes
.schema collaboration_sessions