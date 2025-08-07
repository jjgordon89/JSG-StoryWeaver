// StoryWeaver - AI-Powered Creative Writing Assistant
// Phase 1: Foundation Setup

use tauri::Manager;
use std::sync::Arc;

// Module declarations for Phase 1 foundation
mod commands;
mod database;
mod error;
mod models;
pub mod ai;
pub mod background;
mod utils;
pub mod security;

// Re-export utils for performance monitoring
pub use utils::performance_monitor;

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
            commands::health_check,
            commands::get_database_stats,
            
            // Project commands
            commands::projects::create_project,
            commands::projects::get_projects,
            commands::projects::get_project,
            commands::projects::update_project,
            commands::projects::delete_project,
            commands::projects::update_project_word_count,
            commands::projects::get_project_summary,
            
            // Document commands
            commands::documents::create_document,
            commands::documents::get_documents,
            commands::documents::get_document,
            commands::documents::update_document,
            commands::documents::save_document,
            commands::documents::delete_document,
            commands::documents::search_documents,
            commands::documents::get_document_tree,
            commands::documents::get_document_stats,
            
            // Folder commands
            commands::folder_commands::create_folder,
            commands::folder_commands::get_folder,
            commands::folder_commands::get_root_folders,
            commands::folder_commands::get_child_folders,
            commands::folder_commands::get_all_folders,
            commands::folder_commands::update_folder,
            commands::folder_commands::delete_folder,
            commands::folder_commands::move_items_to_folder,
            commands::folder_commands::get_folder_tree,
            
            // Series commands
            commands::series_commands::create_series,
            commands::series_commands::get_series,
            commands::series_commands::get_all_series,
            commands::series_commands::get_series_with_counts,
            commands::series_commands::update_series,
            commands::series_commands::delete_series,
            commands::series_commands::get_series_projects,
            commands::series_commands::add_project_to_series,
            commands::series_commands::remove_project_from_series,
            
            // Document link commands
            commands::document_link_commands::create_document_link,
            commands::document_link_commands::get_document_link,
            commands::document_link_commands::get_outgoing_links,
            commands::document_link_commands::get_incoming_links,
            commands::document_link_commands::get_all_links_for_document,
            commands::document_link_commands::update_document_link,
            commands::document_link_commands::delete_document_link,
            commands::document_link_commands::delete_all_links_for_document,
            commands::document_link_commands::get_linked_documents,
            
            // Backup commands
            commands::backup_commands::create_backup,
            commands::backup_commands::restore_from_backup,
            commands::backup_commands::get_backups,
            commands::backup_commands::delete_backup,
            commands::backup_commands::create_auto_backup,
            commands::backup_commands::cleanup_old_backups,
            
            // Trash commands
            commands::trash_commands::get_trash_items,
            commands::trash_commands::get_trash_items_by_type,
            commands::trash_commands::get_trash_items_by_parent,
            commands::trash_commands::trash_project,
            commands::trash_commands::trash_document,
            commands::trash_commands::restore_trash_item,
            commands::trash_commands::permanently_delete_trash_item,
            commands::trash_commands::empty_trash,
            
            // Version commands
            commands::version_commands::create_document_version,
            commands::version_commands::get_document_versions,
            commands::version_commands::get_version_history,
            commands::version_commands::get_document_version,
            commands::version_commands::get_latest_document_version,
            commands::version_commands::restore_document_version,
            commands::version_commands::delete_document_version,
            commands::version_commands::delete_all_document_versions,
            
            // Background processing commands
            commands::background_commands::create_background_task,
            commands::background_commands::get_background_task,
            commands::background_commands::get_all_background_tasks,
            commands::background_commands::get_background_tasks_by_status,
            commands::background_commands::get_background_tasks_by_project,
            commands::background_commands::get_background_tasks_by_document,
            commands::background_commands::cancel_background_task,
            commands::background_commands::cleanup_old_background_tasks,
            
            // Performance monitoring commands
            commands::performance_commands::record_performance_metric,
            commands::performance_commands::get_metrics_by_name,
            commands::performance_commands::get_metrics_by_component,
            commands::performance_commands::get_metrics_in_timerange,
            commands::performance_commands::get_performance_summary,
            commands::performance_commands::record_performance_bottleneck,
            commands::performance_commands::resolve_bottleneck,
            commands::performance_commands::record_memory_snapshot,
            commands::performance_commands::record_query_performance,
            commands::performance_commands::cleanup_old_metrics,
            
            // Security commands
            commands::security_commands::save_api_key,
            commands::security_commands::has_api_key,
            commands::security_commands::delete_api_key,
            commands::security_commands::get_privacy_settings,
            commands::security_commands::update_privacy_settings,
            
            // AI Writing commands
            commands::ai_writing::auto_write,
            commands::ai_writing::guided_write,
            commands::ai_writing::auto_write_stream,
            commands::ai_writing::guided_write_stream,
            commands::ai_writing::rewrite_text,
            commands::ai_writing::expand_text,
            commands::ai_writing::describe_scene,
            commands::ai_writing::brainstorm,
            commands::ai_writing::visualize_scene,
            commands::ai_writing::quick_edit,
            commands::ai_writing::quick_chat,
            commands::ai_writing::tone_shift_write,
            commands::ai_writing::get_related_words,
            
            // AI Card commands
            commands::ai_cards::create_ai_card,
            commands::ai_cards::get_ai_card,
            commands::ai_cards::get_ai_cards,
            commands::ai_cards::update_ai_card,
            commands::ai_cards::delete_ai_card,
            commands::ai_cards::get_ai_cards_by_project,
            commands::ai_cards::get_ai_cards_by_document,
            commands::ai_cards::get_stacked_ai_cards,
            commands::ai_cards::get_starred_ai_cards,
            commands::ai_cards::toggle_ai_card_stack,
            commands::ai_cards::toggle_ai_card_star,
            commands::ai_cards::toggle_ai_card_collapse
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
            
            // Initialize background task manager
            let background_task_manager = background::BackgroundTaskManager::new(3, 100);
            
            // Register AI task processor
            let app_handle_clone = app.handle().clone();
            let ai_task_processor = Arc::new(background::ai_processor::AITaskProcessor::new(app_handle_clone));
            
            tauri::async_runtime::spawn(async move {
                background_task_manager.register_processor(ai_task_processor).await;
                if let Err(e) = background_task_manager.start().await {
                    eprintln!("Failed to start background task manager: {}", e);
                }
            });
            
            app.manage(background_task_manager);
            
            // Initialize performance monitoring system
            tauri::async_runtime::spawn(async {
                if let Err(e) = utils::performance_monitor::initialize_performance_monitoring().await {
                    eprintln!("Failed to initialize performance monitoring: {}", e);
                } else {
                    println!("Performance monitoring system initialized successfully");
                }
            });
            
            // Initialize security module
            let app_handle_clone = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = security::init(&app_handle_clone).await {
                    eprintln!("Failed to initialize security module: {}", e);
                } else {
                    println!("Security module initialized successfully");
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
