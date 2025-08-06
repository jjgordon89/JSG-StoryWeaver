# Changelog: StoryWeaver

## [Unreleased]

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
- Monaco Editor configured for writing
- Auto-save system implemented with debouncing
- Folder hierarchy and series support implemented with a drag-and-drop interface
- Document linking system implemented to ensure continuity across documents
- AI provider abstraction layer implemented to integrate AI providers into the application
- Card system implemented to display AI responses in a card-based interface
- Theme support implemented to support dark and light themes
- Keyboard navigation and accessibility features implemented to support keyboard navigation and accessibility
- Build configuration for Windows MSI implemented to support building the application for Windows
- Backup and recovery system implemented to support backup and recovery of user data

## [0.1.0] - 2025-08-05
### Added
- Initialized the Memory Bank system.
- Created core documentation files:
    - `projectBrief.md`
    - `productContext.md`
    - `systemPatterns.md`
    - `techContext.md`
    - `activeContext.md`
    - `progress.md`
    - `changelog.md`
