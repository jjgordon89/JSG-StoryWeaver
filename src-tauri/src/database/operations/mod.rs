mod folder_ops;
mod series_ops;
mod document_link_ops;
mod document_version_ops;
mod deleted_item_ops;
mod app_settings_ops;
mod project_ops;
mod document_ops;
mod character_ops;

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

// Define the original operations
pub struct ProjectOps;
pub struct DocumentOps;
pub struct CharacterOps;
pub struct LocationOps;
pub struct AIHistoryOps;
