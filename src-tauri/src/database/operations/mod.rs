pub mod folder_ops;
pub mod series_ops;
pub mod document_link_ops;
pub mod document_version_ops;
pub mod deleted_item_ops;
pub mod app_settings_ops;
pub mod project_ops;
pub mod document_ops;
pub mod character_ops;
pub mod location_ops;
pub mod background_task_ops;
pub mod performance_metric_ops;
pub mod story_bible_ops;
pub mod timeline_ops;
pub mod plot_thread_ops;
pub mod character_trait_ops;
pub mod world_element_ops;
pub mod outline_ops;
pub mod outline_act_ops;
pub mod scene_ops;
pub mod series_consistency_ops;
pub mod style_example_ops;
pub mod ai_history_ops;
pub mod ai_card_ops;
pub mod character_template_ops;
pub mod worldbuilding_template_ops;

// Phase 4 Advanced AI Features
pub mod ai_provider_ops;
pub mod ai_model_configuration_ops;
pub mod prose_mode_ops;
pub mod generated_image_ops;
pub mod brainstorm_session_ops;
pub mod credit_usage_ops;
pub mod streaming_session_ops;

// Phase 5 Collaboration & Plugins
pub mod collaboration;
pub mod plugin;
pub mod canvas;
pub mod ai;

// Re-export for convenience - only actively used modules
pub use folder_ops::*;
pub use series_ops::*;
pub use document_link_ops::*;
pub use document_version_ops::*;
pub use app_settings_ops::*;
pub use series_consistency_ops::*;
pub use ai_card_ops::*;

// Phase 4 Advanced AI Features - only actively used
pub use ai_provider_ops::*;
pub use ai_model_configuration_ops::*;
pub use prose_mode_ops::*;
pub use generated_image_ops::*;
pub use brainstorm_session_ops::*;
pub use credit_usage_ops::*;
pub use streaming_session_ops::*;

// Phase 5 Collaboration & Plugins - only actively used
pub use collaboration::*;
pub use plugin::*;
pub use canvas::*;

// Struct definitions for operations
pub struct FolderOps;
pub struct SeriesOps;
pub struct DocumentLinkOps;
pub struct DocumentVersionOps;
pub struct DeletedItemOps;
pub struct AppSettingsOps;
pub struct ProjectOps;
pub struct DocumentOps;
pub struct CharacterOps;
pub struct LocationOps;
pub struct BackgroundTaskOps;
pub struct PerformanceMetricOps;
pub struct StoryBibleOps;
pub struct TimelineOps;
pub struct PlotThreadOps;
pub struct CharacterTraitOps;
pub struct WorldElementOps;
pub struct OutlineOps;
pub struct OutlineActOps;
pub struct SceneOps;
pub struct SeriesConsistencyOps;
pub struct StyleExampleOps;
pub struct AIHistoryOps;
pub struct CharacterTemplateOps;
pub struct WorldBuildingTemplateOps;

// Phase 4 Advanced AI Features
pub struct AIProviderOps;
pub struct AIModelConfigurationOps;
pub struct ProseModeOps;
pub struct GeneratedImageOps;
pub struct BrainstormSessionOps;
pub struct CreditUsageOps;
pub struct StreamingSessionOps;

// Phase 5 Collaboration & Plugins
pub struct CollaborationOps;
pub struct PluginOps;
pub struct CanvasOps;
