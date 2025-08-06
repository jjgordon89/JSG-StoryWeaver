//! Command registration for StoryWeaver
//! This module provides functions to register commands with Tauri

use tauri::plugin::PluginHandle;

// Define a simplified Invoke trait for command registration
pub trait Invoke {
    fn register_handler<F>(&mut self, cmd: &'static str, handler: F) -> &mut Self
    where
        F: Fn(tauri::Invoke) -> Result<(), String> + Send + Sync + 'static;
}

// Implement the trait for PluginHandle
impl<R: tauri::Runtime> Invoke for PluginHandle<R> {
    fn register_handler<F>(&mut self, cmd: &'static str, handler: F) -> &mut Self
    where
        F: Fn(tauri::Invoke) -> Result<(), String> + Send + Sync + 'static,
    {
        self.register_invoke_handler(cmd, handler);
        self
    }
}

/// Register core commands
pub fn register_core_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("greet", crate::commands::greet);
    invoke.register_handler("init_database", crate::commands::init_database);
    invoke.register_handler("health_check", crate::commands::health_check);
    invoke.register_handler("get_database_stats", crate::commands::get_database_stats);
}

/// Register project commands
pub fn register_project_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_project", crate::commands::projects::create_project);
    invoke.register_handler("get_projects", crate::commands::projects::get_projects);
    invoke.register_handler("get_project", crate::commands::projects::get_project);
    invoke.register_handler("update_project", crate::commands::projects::update_project);
    invoke.register_handler("delete_project", crate::commands::projects::delete_project);
    invoke.register_handler("update_project_word_count", crate::commands::projects::update_project_word_count);
    invoke.register_handler("get_project_summary", crate::commands::projects::get_project_summary);
    invoke.register_handler("get_project_preview", crate::commands::project_preview_commands::get_project_preview);
}

/// Register document commands
pub fn register_document_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_document", crate::commands::documents::create_document);
    invoke.register_handler("get_documents", crate::commands::documents::get_documents);
    invoke.register_handler("get_document", crate::commands::documents::get_document);
    invoke.register_handler("update_document", crate::commands::documents::update_document);
    invoke.register_handler("save_document", crate::commands::documents::save_document);
    invoke.register_handler("delete_document", crate::commands::documents::delete_document);
    invoke.register_handler("search_documents", crate::commands::documents::search_documents);
    invoke.register_handler("get_document_tree", crate::commands::documents::get_document_tree);
    invoke.register_handler("get_document_stats", crate::commands::documents::get_document_stats);
}

/// Register folder commands
pub fn register_folder_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_folder", crate::commands::folder_commands::create_folder);
    invoke.register_handler("get_folder", crate::commands::folder_commands::get_folder);
    invoke.register_handler("get_root_folders", crate::commands::folder_commands::get_root_folders);
    invoke.register_handler("get_child_folders", crate::commands::folder_commands::get_child_folders);
    invoke.register_handler("get_all_folders", crate::commands::folder_commands::get_all_folders);
    invoke.register_handler("update_folder", crate::commands::folder_commands::update_folder);
    invoke.register_handler("delete_folder", crate::commands::folder_commands::delete_folder);
    invoke.register_handler("move_items_to_folder", crate::commands::folder_commands::move_items_to_folder);
    invoke.register_handler("get_folder_tree", crate::commands::folder_commands::get_folder_tree);
}

/// Register series commands
pub fn register_series_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_series", crate::commands::series_commands::create_series);
    invoke.register_handler("get_series", crate::commands::series_commands::get_series);
    invoke.register_handler("get_all_series", crate::commands::series_commands::get_all_series);
    invoke.register_handler("get_series_with_counts", crate::commands::series_commands::get_series_with_counts);
    invoke.register_handler("update_series", crate::commands::series_commands::update_series);
    invoke.register_handler("delete_series", crate::commands::series_commands::delete_series);
    invoke.register_handler("get_series_projects", crate::commands::series_commands::get_series_projects);
    invoke.register_handler("add_project_to_series", crate::commands::series_commands::add_project_to_series);
    invoke.register_handler("remove_project_from_series", crate::commands::series_commands::remove_project_from_series);
}

/// Register document link commands
pub fn register_document_link_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_document_link", crate::commands::document_link_commands::create_document_link);
    invoke.register_handler("get_document_link", crate::commands::document_link_commands::get_document_link);
    invoke.register_handler("get_outgoing_links", crate::commands::document_link_commands::get_outgoing_links);
    invoke.register_handler("get_incoming_links", crate::commands::document_link_commands::get_incoming_links);
    invoke.register_handler("get_all_links_for_document", crate::commands::document_link_commands::get_all_links_for_document);
    invoke.register_handler("update_document_link", crate::commands::document_link_commands::update_document_link);
    invoke.register_handler("delete_document_link", crate::commands::document_link_commands::delete_document_link);
    invoke.register_handler("delete_all_links_for_document", crate::commands::document_link_commands::delete_all_links_for_document);
    invoke.register_handler("get_linked_documents", crate::commands::document_link_commands::get_linked_documents);
}

/// Register backup commands
pub fn register_backup_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_backup", crate::commands::backup_commands::create_backup);
    invoke.register_handler("restore_from_backup", crate::commands::backup_commands::restore_from_backup);
    invoke.register_handler("get_backups", crate::commands::backup_commands::get_backups);
    invoke.register_handler("delete_backup", crate::commands::backup_commands::delete_backup);
    invoke.register_handler("create_auto_backup", crate::commands::backup_commands::create_auto_backup);
    invoke.register_handler("cleanup_old_backups", crate::commands::backup_commands::cleanup_old_backups);
}

/// Register trash commands
pub fn register_trash_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("get_trash_items", crate::commands::trash_commands::get_trash_items);
    invoke.register_handler("get_trash_items_by_type", crate::commands::trash_commands::get_trash_items_by_type);
    invoke.register_handler("get_trash_items_by_parent", crate::commands::trash_commands::get_trash_items_by_parent);
    invoke.register_handler("trash_project", crate::commands::trash_commands::trash_project);
    invoke.register_handler("trash_document", crate::commands::trash_commands::trash_document);
    invoke.register_handler("restore_trash_item", crate::commands::trash_commands::restore_trash_item);
    invoke.register_handler("permanently_delete_trash_item", crate::commands::trash_commands::permanently_delete_trash_item);
    invoke.register_handler("empty_trash", crate::commands::trash_commands::empty_trash);
}

/// Register version commands
pub fn register_version_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("create_document_version", crate::commands::version_commands::create_document_version);
    invoke.register_handler("get_document_versions", crate::commands::version_commands::get_document_versions);
    invoke.register_handler("get_version_history", crate::commands::version_commands::get_version_history);
    invoke.register_handler("get_document_version", crate::commands::version_commands::get_document_version);
    invoke.register_handler("get_latest_document_version", crate::commands::version_commands::get_latest_document_version);
    invoke.register_handler("restore_document_version", crate::commands::version_commands::restore_document_version);
    invoke.register_handler("delete_document_version", crate::commands::version_commands::delete_document_version);
    invoke.register_handler("delete_all_document_versions", crate::commands::version_commands::delete_all_document_versions);
}

/// Register settings commands
pub fn register_settings_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("get_setting", crate::commands::settings_commands::get_setting);
    invoke.register_handler("get_all_settings", crate::commands::settings_commands::get_all_settings);
    invoke.register_handler("set_setting", crate::commands::settings_commands::set_setting);
    invoke.register_handler("delete_setting", crate::commands::settings_commands::delete_setting);
    
    invoke.register_handler("get_preference", crate::commands::settings_commands::get_preference);
    invoke.register_handler("get_preferences_by_category", crate::commands::settings_commands::get_preferences_by_category);
    invoke.register_handler("get_all_preferences", crate::commands::settings_commands::get_all_preferences);
    invoke.register_handler("set_preference", crate::commands::settings_commands::set_preference);
    invoke.register_handler("delete_preference", crate::commands::settings_commands::delete_preference);
    invoke.register_handler("delete_preference_category", crate::commands::settings_commands::delete_preference_category);
    
    invoke.register_handler("get_preferences_as_object", crate::commands::settings_commands::get_preferences_as_object);
    invoke.register_handler("set_preferences_from_object", crate::commands::settings_commands::set_preferences_from_object);
    invoke.register_handler("sync_settings", crate::commands::settings_commands::sync_settings);
}

/// Register sync commands
pub fn register_sync_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("emit_sync_event", crate::commands::sync_commands::emit_sync_event);
}

/// Register performance monitoring commands
pub fn register_performance_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("record_performance_metric", crate::commands::performance_commands::record_performance_metric);
    invoke.register_handler("get_metrics_by_name", crate::commands::performance_commands::get_metrics_by_name);
    invoke.register_handler("get_metrics_by_component", crate::commands::performance_commands::get_metrics_by_component);
    invoke.register_handler("get_metrics_in_timerange", crate::commands::performance_commands::get_metrics_in_timerange);
    invoke.register_handler("get_performance_summary", crate::commands::performance_commands::get_performance_summary);
    invoke.register_handler("record_performance_bottleneck", crate::commands::performance_commands::record_performance_bottleneck);
    invoke.register_handler("resolve_bottleneck", crate::commands::performance_commands::resolve_bottleneck);
    invoke.register_handler("record_memory_snapshot", crate::commands::performance_commands::record_memory_snapshot);
    invoke.register_handler("record_query_performance", crate::commands::performance_commands::record_query_performance);
    invoke.register_handler("cleanup_old_metrics", crate::commands::performance_commands::cleanup_old_metrics);
}

/// Register security commands
pub fn register_security_commands<R: Invoke>(invoke: &mut R) {
    invoke.register_handler("save_api_key", crate::commands::security_commands::save_api_key);
    invoke.register_handler("has_api_key", crate::commands::security_commands::has_api_key);
    invoke.register_handler("delete_api_key", crate::commands::security_commands::delete_api_key);
    invoke.register_handler("get_privacy_settings", crate::commands::security_commands::get_privacy_settings);
    invoke.register_handler("update_privacy_settings", crate::commands::security_commands::update_privacy_settings);
}

/// Register all commands
pub fn register_all_commands<R: Invoke>(invoke: &mut R) {
    register_core_commands(invoke);
    register_project_commands(invoke);
    register_document_commands(invoke);
    register_folder_commands(invoke);
    register_series_commands(invoke);
    register_document_link_commands(invoke);
    register_backup_commands(invoke);
    register_trash_commands(invoke);
    register_version_commands(invoke);
    register_settings_commands(invoke);
    register_sync_commands(invoke);
    register_performance_commands(invoke);
    register_security_commands(invoke);
}
