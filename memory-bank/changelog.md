# Changelog: StoryWeaver

## [Unreleased]

### Added
- AI Provider abstraction layer
  - Created AIProvider trait for modular AI service integration
  - Implemented OpenAIProvider with full API functionality
  - Added ClaudeProvider for Anthropic's Claude API
  - Built rate limiting and token counting system
  - Implemented error handling for API failures
  - Added support for text generation, rewriting, and embeddings
- Document linking system for chapter continuity
  - Created DocumentLinking component with previous/next document navigation
  - Implemented link creation and removal functionality
  - Added document selection callbacks
- Series support for multi-project workflows
  - Created SeriesManager component for managing series
  - Implemented adding/removing projects to/from series
  - Added series creation functionality
- Folder hierarchy with drag-and-drop support
  - Created FolderHierarchy component with recursive folder rendering
  - Implemented drag-and-drop for reorganizing folders and documents
  - Added folder creation and navigation
- Enhanced document editor
  - Improved Monaco Editor integration with proper configuration
  - Added word count tracking with markdown-aware counting
  - Implemented auto-save with debouncing and status indicators
  - Added keyboard shortcuts (Ctrl+S/Cmd+S)
- Updated project management interface
  - Enhanced ProjectList to display documents
  - Updated ProjectCard to support selection state
  - Added document selection callbacks to ProjectView
  - Integrated components in MainLayout

## [0.1.0] - 2025-08-05
### Added
- Initial project setup with Tauri 2.0 and Vite + React + TypeScript
- SQLite database schema and migrations
- Core database models (Projects, Documents, Characters, Locations, Story Bible)
- Backend command structure for all core entities
- Error handling system with StoryWeaverError enum
- Database connection pooling and async operations
- AI response card data structure
- Monaco Editor integration (dependency installed)
- State management dependencies (Zustand, React Query)
- Three-column responsive UI layout
- Base UI components using Radix UI
- Project management interface with placeholder data
- Initialized the Memory Bank system
- Created core documentation files:
  - `projectBrief.md`
  - `productContext.md`
  - `systemPatterns.md`
  - `techContext.md`
  - `activeContext.md`
  - `progress.md`
  - `changelog.md`
