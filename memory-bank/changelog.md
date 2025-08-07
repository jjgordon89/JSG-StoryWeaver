# Changelog: StoryWeaver

## [Unreleased]

### Added
- __Phase 2 Core Writing Features (Initial Implementation)__
  - Created updated Phase 2 Core Writing Features plan to reflect current progress and remaining tasks
  - Implemented AI provider abstraction layer with trait-based interface for various writing features
  - Created basic integrations for OpenAI, Claude, and Gemini providers
  - Implemented basic WriteProcessor with support for auto_write, guided_write, and tone_shift_write modes
  - Implemented basic token counting and credit calculation
  - Created Card System UI for displaying AI responses with stacking, filtering, and sorting capabilities

- __Performance Monitoring Foundation__
  - Implemented comprehensive performance metrics collection system
  - Created database schema and models for metrics, bottlenecks, memory snapshots, and query performance
  - Built performance monitoring utilities with timers and trackers
  - Implemented bottleneck detection with configurable thresholds
  - Added memory usage monitoring with component breakdown
  - Created database query performance tracking with slow query detection
  - Implemented frontend store and React hook for component-level monitoring
  - Built performance settings UI with toggles and configuration options
  - Created visual performance dashboard with charts and real-time monitoring
  - Added documentation for performance monitoring usage and best practices
  - Implemented automatic cleanup of old metrics data

- __Background Processing Foundation__
  - Created managed task queue for long-running AI operations
  - Implemented task prioritization (user-initiated vs. background)
  - Set up task status tracking (queued, running, completed, failed)
  - Added database models and operations for background tasks
  - Implemented background task commands for task management
  - Created AI processor for handling AI-related background tasks
  - Added error handling for background task failures

- __State Synchronization System for Real-Time Updates__
  - Created Rust backend commands for emitting events to frontend
  - Implemented event types and payload interfaces for various data types
  - Built custom React hook for subscribing to state change events
  - Created Zustand middleware for automatic event emission
  - Added StateSynchronizer component for application-wide event handling
  - Implemented document, settings, and card synchronization
  - Added error handling for event emission failures
  - Integrated with existing components for real-time updates

- __State Persistence Layer for Application Settings__
  - Created backend commands for settings management
  - Implemented database models and operations for app settings and user preferences
  - Enhanced Zustand store with backend synchronization
  - Added comprehensive settings UI with theme, editor, accessibility, and app settings
  - Implemented automatic synchronization between local storage and database
  - Added support for different data types (string, integer, boolean, JSON)

- __Focus Mode for Distraction-Free Writing__
  - Implemented toggle functionality with keyboard shortcut (Ctrl+Shift+F)
  - Created customizable settings for UI elements visibility
  - Added visual enhancements for better focus (dimming UI, hiding panels)
  - Implemented temporary keyboard shortcut hints
  - Added accessibility features (respecting reduced motion preferences)
  - Created persistent settings storage using Zustand

- __Database Backup and Recovery System__
  - Implemented manual and automatic backup creation
  - Added backup restoration functionality
  - Created backup management (listing, deleting)
  - Implemented automatic cleanup of old backups
  - Added proper handling of SQLite WAL and SHM files

- __Trash Management System__
  - Implemented moving projects and documents to trash
  - Added restoring items from trash
  - Created permanently deleting items functionality
  - Implemented listing trash items by type or parent

- __Document Version History__
  - Implemented creating document versions
  - Added retrieving version history
  - Created restoring to previous versions functionality
  - Implemented version management (deleting, etc.)

- __Backend Integration for UI Components__
  - Connected folder hierarchy drag-and-drop to backend commands
  - Integrated series management with backend persistence
  - Connected document linking system to backend commands

### Changed
- Updated Phase 1 documentation to reflect accurate completion status (~99% complete)
- Revised `Plans/Phase1-Foundation.md` to correctly mark completed items and identify the final three remaining tasks
- Updated `memory-bank/progress.md` to reorganize completed tasks and focus on the final integration tasks
- Revised `memory-bank/activeContext.md` to reflect current project status and next steps
- Updated folder hierarchy, series management, and document linking components to use backend data instead of placeholder data
- Marked background processing foundation as completed with all subtasks implemented
- Marked performance monitoring foundation as completed with dashboard visualization added
- Enhanced AI provider implementations with proper streaming support:
  - Replaced placeholder streaming implementations in OpenAI provider with full streaming functionality
  - Implemented proper streaming for all writing features in OpenAI provider (rewrite, expand, describe, quick chat)
  - Enhanced Claude provider with full streaming support for all writing features
  - Added complete streaming implementation for Gemini provider with proper format handling
  - Ensured consistent streaming implementation across all providers
  - Implemented provider-specific streaming response parsing for each AI service

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
