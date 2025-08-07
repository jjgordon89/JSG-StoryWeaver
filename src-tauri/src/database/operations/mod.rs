pub mod folder_ops;
pub mod series_ops;
pub mod document_link_ops;
pub mod document_version_ops;
pub mod deleted_item_ops;
pub mod app_settings_ops;
pub mod project_ops;
pub mod document_ops;
pub mod character_ops;
pub mod background_task_ops;
pub mod performance_metric_ops;

// Re-export all operations
pub use folder_ops::*;
pub use series_ops::*;
pub use document_link_ops::*;
pub use document_version_ops::*;
pub use deleted_item_ops::*;
pub use app_settings_ops::*;
pub use project_ops::*;
pub use document_ops::*;
pub use character_ops::*;
pub use background_task_ops::*;
pub use performance_metric_ops::*;

// Define the original operations
pub struct ProjectOps;
pub struct DocumentOps;
pub struct CharacterOps;
pub struct LocationOps;
pub struct AIHistoryOps;
pub struct BackgroundTaskOps;
pub struct PerformanceMetricOps;
