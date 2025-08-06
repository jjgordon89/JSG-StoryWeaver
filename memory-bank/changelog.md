# Changelog: StoryWeaver

## [Unreleased]

### Added

### Changed
- Updated Phase 1 documentation to reflect accurate completion status (~85% complete)
- Revised `Plans/Phase1-Foundation.md` to correctly mark completed, incomplete, and deferred tasks
- Updated `memory-bank/progress.md` to list remaining tasks before moving to Phase 2
- Revised `memory-bank/activeContext.md` to align with current project status
- Clarified that folder hierarchy, series management, and document linking have UI implemented but lack backend integration

## [0.2.0] - 2025-08-06

### Added

- __Card System UI__

  - Created card components for AI responses
  - Implemented card stacking and organization
  - Added interaction handlers (expand/collapse, star, delete)
  - Built card history and persistence with filtering and sorting

- __Theme Support & Accessibility__

  - Implemented dark/light/system theme switcher
  - Added keyboard navigation and accessibility features
  - Implemented reduced motion and high contrast modes
  - Added font size and line height adjustments

- __Windows MSI Build Configuration__

  - Created a script for building the Windows MSI installer
  - Added WiX template and configuration files
  - Set up custom installer UI with license and branding

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

- __AI Provider Abstraction Layer__

  - Created AIProvider trait for modular AI service integration
  - Implemented OpenAIProvider with full API functionality
  - Added ClaudeProvider for Anthropic's Claude API
  - Built rate limiting and token counting system
  - Implemented error handling for API failures

- __Document Linking System__

  - Created DocumentLinking component with previous/next document navigation
  - Implemented link creation and removal functionality
  - Added document selection callbacks

- __Series Support__

  - Created SeriesManager component for managing series
  - Implemented adding/removing projects to/from series
  - Added series creation functionality

- __Folder Hierarchy__

  - Created FolderHierarchy component with recursive folder rendering
  - Implemented drag-and-drop for reorganizing folders and documents
  - Added folder creation and navigation

- __Enhanced Document Editor__

  - Improved Monaco Editor integration with proper configuration
  - Added word count tracking with markdown-aware counting
  - Implemented auto-save with debouncing and status indicators
  - Added keyboard shortcuts (Ctrl+S/Cmd+S)

- __Updated Project Management Interface__

  - Enhanced ProjectList to display documents
  - Updated ProjectCard to support selection state
  - Added document selection callbacks to ProjectView
  - Integrated components in MainLayout
