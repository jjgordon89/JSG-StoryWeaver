// StoryWeaver - AI-Powered Creative Writing Assistant
// Phase 1: Foundation Setup

use tauri::Manager;
use std::sync::Arc;

// Module declarations for Phase 1 foundation
mod commands;
mod database;
mod error;
pub mod ai;
mod utils;

// Re-export error types
pub use error::StoryWeaverError;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Initialize all required plugins for Phase 1
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        // Setup handler for commands
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::init_database,
            commands::projects::create_project,
            commands::projects::get_projects,
            commands::documents::create_document,
            commands::documents::save_document
        ])
        .setup(|app| {
            // Initialize database on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = database::init(&app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });

            // Initialize AIProviderManager and register OpenAIProvider
            let mut ai_manager = ai::AIProviderManager::new();
            let openai_provider = Arc::new(ai::OpenAIProvider::new(
                std::env::var("OPENAI_API_KEY").unwrap_or_default(),
                "gpt-4-turbo".to_string(),
            ));
            ai_manager.register_provider("openai".to_string(), openai_provider);
            ai_manager.set_default_provider("openai".to_string());
            app.manage(ai_manager);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
