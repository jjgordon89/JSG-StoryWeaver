-- Migration 012: Add composite indexes for AI card filtering performance
-- This migration adds indexes to optimize AI card filtering queries

-- Index for project-based queries with date filtering
CREATE INDEX idx_ai_cards_project_date 
ON ai_response_cards(project_id, created_at DESC);

-- Index for project-based queries with provider/model filtering
CREATE INDEX idx_ai_cards_project_provider_model 
ON ai_response_cards(project_id, model_used);

-- Index for cost-based filtering
CREATE INDEX idx_ai_cards_project_cost 
ON ai_response_cards(project_id, cost_estimate);

-- Index for feature type filtering
CREATE INDEX idx_ai_cards_project_feature 
ON ai_response_cards(project_id, feature_type);

-- Index for document-based queries
CREATE INDEX idx_ai_cards_document_date 
ON ai_response_cards(document_id, created_at DESC);

-- Record migration
INSERT INTO migrations (name) 
SELECT '012_ai_card_indexes' 
WHERE NOT EXISTS (SELECT 1 FROM migrations WHERE name = '012_ai_card_indexes');
