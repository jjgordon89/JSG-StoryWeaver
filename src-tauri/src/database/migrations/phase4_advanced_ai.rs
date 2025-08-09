//! Migration 015: Phase 4 Advanced AI Features
//! Creates tables for AI providers, model configurations, prose modes, style examples,
//! generated images, brainstorm sessions, credit usage, and streaming sessions

use crate::error::{Result, StoryWeaverError};
use sqlx::{Pool, Sqlite};

pub async fn up(pool: &Pool<Sqlite>) -> Result<()> {
    // AI Providers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ai_providers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            api_endpoint TEXT,
            is_active BOOLEAN DEFAULT TRUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create ai_providers table: {}", e)))?;

    // AI Model Configurations table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ai_model_configurations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider_id INTEGER NOT NULL,
            model_name TEXT NOT NULL,
            display_name TEXT NOT NULL,
            context_window INTEGER NOT NULL,
            max_output_tokens INTEGER NOT NULL,
            supports_streaming BOOLEAN DEFAULT TRUE,
            supports_images BOOLEAN DEFAULT FALSE,
            cost_per_input_token REAL,
            cost_per_output_token REAL,
            cost_per_image REAL,
            quality_tier TEXT DEFAULT 'standard',
            specializations TEXT, -- JSON
            is_active BOOLEAN DEFAULT TRUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (provider_id) REFERENCES ai_providers(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create ai_model_configurations table: {}", e)))?;

    // Prose Modes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS prose_modes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            model_configuration_id INTEGER NOT NULL,
            creativity_level INTEGER DEFAULT 5,
            temperature REAL DEFAULT 0.7,
            top_p REAL DEFAULT 0.9,
            frequency_penalty REAL DEFAULT 0.0,
            presence_penalty REAL DEFAULT 0.0,
            special_instructions TEXT,
            is_experimental BOOLEAN DEFAULT FALSE,
            max_context_words INTEGER DEFAULT 4000,
            max_generation_words INTEGER DEFAULT 2000,
            supports_streaming BOOLEAN DEFAULT TRUE,
            supports_unfiltered BOOLEAN DEFAULT FALSE,
            is_active BOOLEAN DEFAULT TRUE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (model_configuration_id) REFERENCES ai_model_configurations(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create prose_modes table: {}", e)))?;

    // Style Examples table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS style_examples (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER,
            user_id TEXT,
            example_text TEXT NOT NULL,
            analysis_result TEXT,
            generated_style_prompt TEXT,
            word_count INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create style_examples table: {}", e)))?;

    // Generated Images table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS generated_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            document_id INTEGER,
            source_text TEXT NOT NULL,
            image_prompt TEXT NOT NULL,
            image_data BLOB,
            image_url TEXT,
            credits_used INTEGER DEFAULT 2500,
            resolution TEXT DEFAULT '1024x1024',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (document_id) REFERENCES documents(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create generated_images table: {}", e)))?;

    // Brainstorm Sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS brainstorm_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            category TEXT NOT NULL,
            seed_prompt TEXT,
            session_data TEXT, -- JSON
            keepers_list TEXT, -- JSON
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create brainstorm_sessions table: {}", e)))?;

    // Credit Usage Tracking table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS credit_usage (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            feature_name TEXT NOT NULL,
            model_used TEXT,
            credits_used INTEGER,
            tokens_input INTEGER,
            tokens_output INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (project_id) REFERENCES projects(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create credit_usage table: {}", e)))?;

    // Streaming Sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS streaming_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            document_id INTEGER NOT NULL,
            feature_type TEXT NOT NULL,
            session_token TEXT UNIQUE NOT NULL,
            current_text TEXT,
            is_paused BOOLEAN DEFAULT FALSE,
            can_resume BOOLEAN DEFAULT TRUE,
            context_data TEXT, -- JSON
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (document_id) REFERENCES documents(id)
        )
        "#,
    )
    .execute(&*pool)
    .await
    .map_err(|e| StoryWeaverError::database(format!("Failed to create streaming_sessions table: {}", e)))?;

    // Insert default AI providers
    let default_providers = [
        ("openai", "OpenAI", "https://api.openai.com/v1"),
        ("anthropic", "Anthropic", "https://api.anthropic.com"),
        ("google", "Google AI", "https://generativelanguage.googleapis.com"),
        ("deepseek", "DeepSeek", "https://api.deepseek.com"),
    ];

    for (name, display_name, api_endpoint) in default_providers {
        sqlx::query(
            "INSERT OR IGNORE INTO ai_providers (name, display_name, api_endpoint) VALUES (?, ?, ?)"
        )
        .bind(name)
        .bind(display_name)
        .bind(api_endpoint)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to insert default AI provider: {}", e)))?;
    }

    // Insert default model configurations
    let default_models: Vec<(i32, &str, &str, i32, i32, bool, bool, Option<f64>, Option<f64>, Option<f64>)> = vec![
        // OpenAI models (assuming provider_id = 1)
        (1, "gpt-4o", "GPT-4o", 128000, 4096, true, false, Some(0.000005), Some(0.000015), None),
        (1, "gpt-4o-mini", "GPT-4o Mini", 128000, 16384, true, false, Some(0.00000015), Some(0.0000006), None),
        (1, "gpt-4-turbo", "GPT-4 Turbo", 128000, 4096, true, false, Some(0.00001), Some(0.00003), None),
        (1, "dall-e-3", "DALL-E 3", 0, 0, false, true, None, None, Some(0.04)),
        
        // Anthropic models (assuming provider_id = 2)
        (2, "claude-3-5-sonnet-20241022", "Claude 3.5 Sonnet", 200000, 8192, true, false, Some(0.000003), Some(0.000015), None),
        (2, "claude-3-5-haiku-20241022", "Claude 3.5 Haiku", 200000, 8192, true, false, Some(0.0000008), Some(0.000004), None),
        (2, "claude-3-opus-20240229", "Claude 3 Opus", 200000, 4096, true, false, Some(0.000015), Some(0.000075), None),
        
        // Google models (assuming provider_id = 3)
        (3, "gemini-1.5-pro", "Gemini 1.5 Pro", 2000000, 8192, true, false, Some(0.00000125), Some(0.000005), None),
        (3, "gemini-1.5-flash", "Gemini 1.5 Flash", 1000000, 8192, true, false, Some(0.000000075), Some(0.0000003), None),
        
        // DeepSeek models (assuming provider_id = 4)
        (4, "deepseek-chat", "DeepSeek Chat", 64000, 4096, true, false, Some(0.00000014), Some(0.00000028), None),
        (4, "deepseek-coder", "DeepSeek Coder", 64000, 4096, true, false, Some(0.00000014), Some(0.00000028), None),
    ];

    for (provider_id, model_name, display_name, context_window, max_output_tokens, supports_streaming, supports_images, cost_input, cost_output, cost_image) in default_models {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO ai_model_configurations 
            (provider_id, model_name, display_name, context_window, max_output_tokens, 
             supports_streaming, supports_images, cost_per_input_token, cost_per_output_token, cost_per_image) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(provider_id)
        .bind(model_name)
        .bind(display_name)
        .bind(context_window)
        .bind(max_output_tokens)
        .bind(supports_streaming)
        .bind(supports_images)
        .bind(cost_input)
        .bind(cost_output)
        .bind(cost_image)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to insert default model configuration: {}", e)))?;
    }

    // Insert default prose modes
    let default_prose_modes = [
        // Muse mode (premium creative writing)
        ("Muse", "Premium creative writing with cliché detection and ultra-creative mode", 1, 7, 0.8, 0.9, 0.1, 0.1, "Focus on creative, original prose. Avoid clichés and overused phrases. Embrace unconventional narrative techniques and vivid imagery.", false, 128000, 10000, true, true),
        
        // Excellent mode (high-quality writing)
        ("Excellent", "High-quality writing with balanced creativity and structure", 1, 6, 0.75, 0.85, 0.05, 0.05, "Produce well-crafted, engaging prose with strong narrative flow and character development.", false, 64000, 5000, true, false),
        
        // Basic mode (drafts and quick content)
        ("Basic", "Quick drafts and basic content generation", 2, 4, 0.6, 0.8, 0.0, 0.0, "Generate clear, straightforward prose suitable for drafts and basic content.", false, 32000, 2000, true, false),
        
        // Experimental mode
        ("Experimental", "Experimental features with cutting-edge models", 1, 8, 0.9, 0.95, 0.15, 0.15, "Experimental mode with maximum creativity and unconventional approaches. Use with caution.", true, 200000, 15000, true, true),
    ];

    for (name, description, model_config_id, creativity_level, temperature, top_p, freq_penalty, presence_penalty, instructions, is_experimental, max_context, max_generation, supports_streaming, supports_unfiltered) in default_prose_modes {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO prose_modes 
            (name, description, model_configuration_id, creativity_level, temperature, top_p, 
             frequency_penalty, presence_penalty, special_instructions, is_experimental, 
             max_context_words, max_generation_words, supports_streaming, supports_unfiltered) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(name)
        .bind(description)
        .bind(model_config_id)
        .bind(creativity_level)
        .bind(temperature)
        .bind(top_p)
        .bind(freq_penalty)
        .bind(presence_penalty)
        .bind(instructions)
        .bind(is_experimental)
        .bind(max_context)
        .bind(max_generation)
        .bind(supports_streaming)
        .bind(supports_unfiltered)
        .execute(&*pool)
        .await
        .map_err(|e| StoryWeaverError::database(format!("Failed to insert default prose mode: {}", e)))?;
    }

    // Create indexes for better performance
    let indexes = [
        "CREATE INDEX IF NOT EXISTS idx_ai_model_configs_provider ON ai_model_configurations(provider_id)",
        "CREATE INDEX IF NOT EXISTS idx_prose_modes_model_config ON prose_modes(model_configuration_id)",
        "CREATE INDEX IF NOT EXISTS idx_style_examples_project ON style_examples(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_generated_images_project ON generated_images(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_generated_images_document ON generated_images(document_id)",
        "CREATE INDEX IF NOT EXISTS idx_brainstorm_sessions_project ON brainstorm_sessions(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_credit_usage_project ON credit_usage(project_id)",
        "CREATE INDEX IF NOT EXISTS idx_credit_usage_created_at ON credit_usage(created_at)",
        "CREATE INDEX IF NOT EXISTS idx_streaming_sessions_document ON streaming_sessions(document_id)",
        "CREATE INDEX IF NOT EXISTS idx_streaming_sessions_token ON streaming_sessions(session_token)",
    ];

    for index_sql in indexes {
        sqlx::query(index_sql)
            .execute(&*pool)
            .await
            .map_err(|e| StoryWeaverError::database(format!("Failed to create index: {}", e)))?;
    }

    Ok(())
}