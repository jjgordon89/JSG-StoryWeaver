#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]

// StoryWeaver - AI-Powered Creative Writing Assistant
// Phase 1: Foundation Setup

use tauri::Manager;
use std::sync::Arc;

// Module declarations for Phase 1 foundation
pub mod commands;
pub mod database;
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
            
            // Project preview commands
            commands::project_preview_commands::get_project_preview,
            
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
            
            // Character commands
            commands::characters::create_character,
            commands::characters::get_characters,
            commands::characters::get_character,
            commands::characters::update_character,
            commands::characters::delete_character,
            commands::characters::get_characters_by_series,
            commands::characters::get_visible_characters,
            commands::characters::share_character_to_series,
            commands::characters::unshare_character_from_series,
            commands::characters::get_character_stats,
            
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
            
            // Series consistency commands
            commands::series_consistency_commands::generate_series_consistency_report,
            commands::series_consistency_commands::get_series_consistency_status,
            commands::series_consistency_commands::get_series_conflicts_by_severity,
            commands::series_consistency_commands::batch_check_series_consistency,
            
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
            commands::ai_writing::brainstorm_ideas,
            commands::ai_writing::visualize_scene,
            commands::ai_writing::quick_edit,
            commands::ai_writing::quick_chat,
            commands::ai_writing::tone_shift_write,
            commands::ai_writing::get_related_words,
            
            // AI Card commands
            commands::ai_cards::create_ai_response_card,
            commands::ai_cards::get_ai_response_card,
            commands::ai_cards::get_ai_response_cards_by_project,
            commands::ai_cards::update_ai_response_card,
            commands::ai_cards::delete_ai_response_card,
            commands::ai_cards::get_ai_response_cards_by_document,
            commands::ai_cards::get_ai_response_cards_by_type,
            commands::ai_cards::get_ai_response_cards_by_status,
            commands::ai_cards::get_ai_response_cards_by_date_range,
            commands::ai_cards::get_ai_response_cards_by_provider,
            commands::ai_cards::get_ai_response_cards_by_model,
            commands::ai_cards::get_ai_response_cards_by_cost_range,
            
            // Story Bible commands
            commands::story_bible::create_or_update_story_bible,
            commands::story_bible::get_story_bible,
            commands::story_bible::create_character_trait,
            commands::story_bible::get_character_traits,
            commands::story_bible::update_character_trait,
            commands::story_bible::delete_character_trait,
            commands::story_bible::create_world_element,
            commands::story_bible::get_world_elements,
            commands::story_bible::get_world_element,
            commands::story_bible::update_world_element,
            commands::story_bible::delete_world_element,
            commands::story_bible::search_world_elements,
            commands::story_bible::create_outline,
            commands::story_bible::get_outlines,
            commands::story_bible::get_outline,
            commands::story_bible::get_outline_by_chapter,
            commands::story_bible::update_outline,
            commands::story_bible::delete_outline,
            commands::story_bible::search_outlines,
            commands::story_bible::create_scene,
            commands::story_bible::get_scenes,
            commands::story_bible::get_scene,
            commands::story_bible::update_scene,
            commands::story_bible::delete_scene,
            commands::story_bible::validate_scene,
            commands::story_bible::search_scenes,
            
            // Story Bible AI Generation commands
            commands::story_bible_ai::generate_synopsis,
            commands::story_bible_ai::generate_character_traits,
            commands::story_bible_ai::generate_world_element,
            commands::story_bible_ai::generate_outline_from_story_bible,
            commands::story_bible_ai::generate_scene_content,
            commands::story_bible_ai::analyze_style_example,
            commands::story_bible_ai::generate_outline_from_text,
            
            // Style Example commands
            commands::style_examples::create_style_example,
            commands::style_examples::get_style_examples_by_project,
            commands::style_examples::get_analyzed_style_examples,
            commands::style_examples::get_style_example_by_id,
            commands::style_examples::update_style_example,
            commands::style_examples::delete_style_example,
            commands::style_examples::delete_style_examples_by_project,
            
            // Template commands
            commands::templates::get_character_templates,
            commands::templates::get_character_templates_by_archetype,
            commands::templates::get_character_archetypes,
            commands::templates::apply_character_template,
            commands::templates::get_worldbuilding_templates,
            commands::templates::get_worldbuilding_templates_by_type,
            commands::templates::get_worldbuilding_element_types,
            commands::templates::apply_worldbuilding_template,
            
            // Advanced AI commands (Phase 4)
            commands::advanced_ai_commands::generate_with_prose_mode,
            commands::advanced_ai_commands::generate_image,
            commands::advanced_ai_commands::create_brainstorm_session,
            commands::advanced_ai_commands::get_brainstorm_session,
            commands::advanced_ai_commands::rate_brainstorm_idea,
            commands::advanced_ai_commands::mark_idea_as_keeper,
            commands::advanced_ai_commands::add_style_example,
            commands::advanced_ai_commands::analyze_text_style,
            commands::advanced_ai_commands::get_available_prose_modes,
            commands::advanced_ai_commands::get_prose_mode_details,
            commands::advanced_ai_commands::get_credit_usage,
            commands::advanced_ai_commands::get_project_images,
            commands::advanced_ai_commands::delete_generated_image,
            commands::advanced_ai_commands::build_saliency_context,
            commands::advanced_ai_commands::smart_import_content,
            commands::advanced_ai_commands::start_streaming_generation,
            commands::advanced_ai_commands::get_stream_status,
            commands::advanced_ai_commands::save_generated_content,
            commands::advanced_ai_commands::cancel_streaming_generation,
            
            // Phase 5 Collaboration commands
            commands::collaboration::create_shared_document_link,
            commands::collaboration::get_shared_document,
            commands::collaboration::add_comment,
            commands::collaboration::get_comments,
            commands::collaboration::resolve_comment,
            commands::collaboration::delete_comment,
            commands::collaboration::create_collaboration_session,
            commands::collaboration::join_collaboration_session,
            commands::collaboration::leave_collaboration_session,
            commands::collaboration::duplicate_document_for_sharing,
            commands::collaboration::unpublish_shared_document,
            commands::collaboration::republish_shared_document,
            commands::collaboration::get_project_shared_documents,
            commands::collaboration::create_notification,
            commands::collaboration::get_notifications_for_user,
            commands::collaboration::mark_notification_read,
            commands::collaboration::mark_all_notifications_read,
            commands::collaboration::get_unread_notification_count,
            commands::collaboration::delete_old_notifications,
            
            // Phase 5 Plugin commands
            commands::plugin::create_plugin,
            commands::plugin::get_plugin,
            commands::plugin::get_plugins,
            commands::plugin::search_plugins,
            commands::plugin::update_plugin,
            commands::plugin::delete_plugin,
            commands::plugin::execute_plugin_command,
            commands::plugin::rate_plugin,
            commands::plugin::get_plugin_ratings,
            commands::plugin::record_plugin_execution,
            commands::plugin::get_plugin_execution_history,
            commands::plugin::get_plugin_daily_stats,
            commands::plugin::create_plugin_template,
            commands::plugin::get_plugin_templates,
            commands::plugin::apply_plugin_template,
            
            // Phase 5 Canvas commands
            commands::canvas::create_canvas,
            commands::canvas::get_canvas,
            commands::canvas::get_project_canvases,
            commands::canvas::update_canvas,
            commands::canvas::delete_canvas,
            commands::canvas::create_canvas_element,
            commands::canvas::get_canvas_elements,
            commands::canvas::update_canvas_element,
            commands::canvas::delete_canvas_element,
            commands::canvas::get_outline_templates,
            commands::canvas::create_outline_template,
            commands::canvas::create_canvas_snapshot,
            commands::canvas::get_canvas_snapshots,
            commands::canvas::restore_canvas_snapshot,
            commands::canvas::export_canvas,
            commands::canvas::create_canvas_collaboration_session,
            commands::canvas::get_canvas_collaboration_session,
            commands::canvas::join_canvas_collaboration,
            commands::canvas::leave_canvas_collaboration,
            commands::canvas::record_canvas_operation,
            
            // Phase 6 Optimization commands
            commands::optimization_commands::get_optimization_stats,
            commands::optimization_commands::run_database_optimization,
            commands::optimization_commands::get_index_recommendations,
            commands::optimization_commands::create_index,
            commands::optimization_commands::drop_unused_indexes,
            commands::optimization_commands::clear_ai_cache,
            commands::optimization_commands::optimize_memory_usage,
            commands::optimization_commands::get_cache_statistics,
            commands::optimization_commands::run_performance_analysis,
            commands::optimization_commands::schedule_maintenance
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
            
            // Initialize Advanced AI Manager (Phase 4)
            let advanced_ai_manager = ai::AdvancedAIManager::new();
            app.manage(commands::advanced_ai_commands::AdvancedAIState::new(advanced_ai_manager));
            
            // Initialize background task manager
            let background_task_manager = Arc::new(background::BackgroundTaskManager::new(3, 100));
            
            // Register AI task processor
            let app_handle_clone = app.handle().clone();
            let ai_task_processor = Arc::new(background::ai_processor::AITaskProcessor::new(app_handle_clone));
            
            let background_task_manager_clone = background_task_manager.clone();
            tauri::async_runtime::spawn(async move {
                background_task_manager_clone.register_processor(ai_task_processor).await;
                if let Err(e) = background_task_manager_clone.start().await {
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
        .unwrap_or_else(|e| {
            eprintln!("error while running tauri application: {}", e);
        });
}
