-- Fix collaboration tables schema
-- Add missing columns to shared_documents and document_comments tables

BEGIN TRANSACTION;

-- Fix shared_documents table - add missing share_type column
ALTER TABLE shared_documents ADD COLUMN share_type TEXT DEFAULT 'public';
ALTER TABLE shared_documents ADD COLUMN password_hash TEXT;
ALTER TABLE shared_documents ADD COLUMN max_uses INTEGER;

-- Update existing records with default values
UPDATE shared_documents SET share_type = 'public' WHERE share_type IS NULL;
UPDATE shared_documents SET max_uses = 100 WHERE max_uses IS NULL;

-- Fix document_comments table - add missing columns
ALTER TABLE document_comments ADD COLUMN position_start INTEGER;
ALTER TABLE document_comments ADD COLUMN position_end INTEGER;
ALTER TABLE document_comments ADD COLUMN selected_text TEXT;
ALTER TABLE document_comments ADD COLUMN comment_type TEXT DEFAULT 'general';

-- Update existing records with default values
UPDATE document_comments SET comment_type = 'general' WHERE comment_type IS NULL;

-- Parse position_data JSON and populate new columns for existing records
UPDATE document_comments 
SET 
    position_start = CAST(json_extract(position_data, '$.start') AS INTEGER),
    position_end = CAST(json_extract(position_data, '$.end') AS INTEGER)
WHERE position_data IS NOT NULL AND position_data != '';

COMMIT;

-- Verify the changes
.schema shared_documents
.schema document_comments