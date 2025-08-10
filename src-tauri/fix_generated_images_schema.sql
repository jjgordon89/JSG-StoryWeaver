-- Fix generated_images table schema to match code expectations
-- Add missing columns that the Rust code is trying to access

PRAGMA foreign_keys = ON;

-- Add missing columns to generated_images table
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

-- Update existing records to have default values for new columns
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

PRAGMA foreign_key_check;