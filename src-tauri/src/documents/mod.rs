//! Document management module for StoryWeaver
//! Handles document processing, lazy loading, and performance optimization

pub mod lazy_loading;

pub use lazy_loading::{
    LazyDocumentLoader, DocumentChunk, DocumentMetadata, LazyLoadingConfig,
    CacheStats as DocumentCacheStats, init_lazy_loader, get_lazy_loader,
    start_lazy_loading_cleanup_task
};
