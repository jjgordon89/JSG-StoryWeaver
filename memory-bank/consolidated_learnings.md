## General Patterns

### Database Operations and File Management
- **SQLite File Handling**: When working with SQLite databases, always handle related files (WAL and SHM) during backup and restore operations to maintain database integrity.
- **Connection Management**: Close database connections before file operations that might cause locking issues, especially during restore operations.
- **Soft Delete Implementation**: Implement soft delete functionality with metadata preservation for recoverable operations, allowing users to restore accidentally deleted items.
- **Transaction Handling**: Use transactions for operations that affect multiple database tables to ensure data consistency.
- **Error Handling for File Operations**: Implement comprehensive error handling for file operations, as they can fail for various reasons (permissions, disk space, etc.).

### React Component Development

#### UI/UX Patterns
- **Global Styling with Body Classes**: Use document body classes for global styling changes that affect multiple components. This approach allows for coordinated styling changes across the entire application.
- **Keyboard Shortcuts**: Implement keyboard shortcuts for frequently used features to improve user experience and productivity. Document these shortcuts in tooltips and UI elements.
- **Accessibility Considerations**: Respect user accessibility preferences like reduced motion by checking for prefers-reduced-motion media query and adjusting animations accordingly.
- **Progressive Disclosure**: Use progressive disclosure techniques to hide complexity until needed, such as settings panels that appear only when relevant.
- **Temporary UI Hints**: Display temporary hints (like keyboard shortcut reminders) that fade out after a few seconds to guide users without cluttering the interface permanently.
- **Placeholder-First Development**: When implementing a new UI component, start with a simple placeholder and then replace it with functional components in subsequent tasks. This approach allows for rapid UI prototyping and iterative refinement.

#### Third-Party Library Integration
- **Proper Initialization and Cleanup**: Use React's useRef and useEffect hooks for proper initialization and cleanup of third-party libraries like Monaco Editor. This ensures resources are properly allocated and released.
- **Event Handling**: Set up event listeners in useEffect and clean them up in the return function to prevent memory leaks.
- **Component Lifecycle Management**: Be mindful of component lifecycle when integrating external libraries, especially those with their own lifecycle.

#### Component Architecture
- **Recursive Component Rendering**: Implement recursive component rendering for hierarchical data structures like folder trees. This creates a clean, maintainable approach to displaying nested data.
- **Component Communication**: Use a combination of props, callbacks, and context for component communication based on the scope of data sharing needed.
- **State Management Strategy**: Use local state for component-specific concerns and global state (Zustand, Redux) for application-wide data.
- **Component Organization**: Organize components by feature area (editor, project, etc.) for better code organization and maintainability.
- **Loading and Error States**: Implement loading and error states for all asynchronous operations to improve user experience. This provides immediate feedback to users and handles edge cases gracefully.
- **Data Refresh Strategy**: Refresh related data after state-changing operations to maintain UI consistency. This ensures that all components reflect the current state of the application.

#### TypeScript Best Practices
- **Interface-First Development**: Define clear interfaces for component props before implementation to ensure type safety.
- **Consistent Prop Typing**: Use TypeScript interfaces for consistent prop typing across components to catch errors at compile time.
- **Union Types for State**: Use union types to represent different states of a component (e.g., 'saved' | 'saving' | 'error').
- **Generic API Calls**: Use TypeScript generics with invoke<T> to ensure type safety when handling backend responses. This provides compile-time type checking for API responses.
- **Data Transformation Helpers**: Create helper functions for data transformation between backend and frontend formats to maintain clean component code.

### Troubleshooting Strategies
- **Cascading Errors**: When troubleshooting a series of cascading errors, strip the configuration down to a minimal working state and then re-introduce components one by one.
- **Incremental Development**: Build features incrementally, testing each step before moving to the next to isolate issues early.
- **Error Handling with Context**: Provide rich context in error messages from external API calls to aid in debugging and user feedback.
- **Comprehensive Error Handling**: Implement user-friendly error messages and recovery options for failed operations to improve the overall user experience.

### UI Implementation Approach
- **Placeholder-First Development**: When implementing a new UI component, start with a simple placeholder and then replace it with functional components in subsequent tasks.
- **Progressive Enhancement**: Add features to components incrementally, starting with core functionality and adding enhancements later.

## Project-Specific Patterns

### AI Provider Architecture
- **Trait-Based Abstraction**: Use trait-based abstractions for services with multiple potential implementations, enabling a clean plugin architecture.
- **Rate Limiting Strategy**: Implement rate limiting at the provider level to manage API usage efficiently and prevent quota exhaustion.
- **Shared State Management**: Use Arc<Mutex<T>> for shared state that needs to be modified across async contexts, particularly for tracking API usage.
- **Error Handling with Context**: Provide rich context in error messages from external API calls to aid in debugging and user feedback.
- **Feature Compatibility**: Consider adding fallback mechanisms for features not supported by all providers to maintain consistent application behavior.
- **Secure API Key Storage**: Implement secure API key storage as a priority for AI Provider systems to protect sensitive credentials.
- **Token Usage Tracking**: Implement token usage tracking to prevent exceeding API rate limits and manage costs effectively.

### Tauri 2.0 Configuration
- **Plugin Configuration**: Tauri 2.0 plugin configuration is very strict. When in doubt, start with a minimal configuration and add plugins back one by one.
- **State Management**: Store provider managers in Tauri state for global backend access across the application.
- **Plugin Scope Deprecation**: The `scope` field in plugins like `fs` is deprecated and has been replaced with `allow`. For many plugins, simply enabling them with `true` or an empty object `{}` is not sufficient. The correct way to enable a plugin with no configuration is to remove it from the `plugins` object entirely.

### Rust Backend Development
- **Async Trait Implementation**: Use async_trait for async trait object safety in Rust when designing plugin/provider abstractions.
- **Module Organization**: Use a dedicated module and trait for each AI provider, re-export via a central module for extensibility.
- **Import Placement**: Always place imports at the top-level to avoid attribute-related syntax errors in Rust.
- **API Integration Patterns**: When integrating with external APIs, create dedicated request/response structs that match the API's schema for reliable serialization/deserialization.
- **Thread Safety**: Use Arc for sharing immutable data across threads and Arc<Mutex<T>> for mutable data that needs synchronization in async contexts.
- **Provider Scaffolding**: Scaffold each new provider as a separate file, following the trait interface for consistency and testability. This modular approach enables easy extension for new providers.
- **Early Abstraction**: Scaffold AI provider abstractions early to enable modular, testable AI integrations for all future features. This provides a solid foundation for extensibility.

### StoryWeaver Architecture
- **Document Editor Integration**: Monaco Editor provides a powerful foundation for the document editor with features like syntax highlighting and keyboard shortcuts.
- **Folder Hierarchy**: A recursive component approach works well for displaying and managing nested folder structures.
- **Series Support**: Organizing related projects in series allows for shared story bible data and consistent worldbuilding.
- **Document Linking**: Bidirectional document linking enables continuity between chapters with intuitive navigation.
- **Database Backup System**: A comprehensive backup system should include both manual and automatic backups, with management features for listing, restoring, and cleaning up old backups.
- **Trash Management**: A robust trash system preserves metadata and relationships, allowing for easy restoration of deleted items.
- **Version History**: Document version history enables users to track changes and restore previous versions, enhancing the writing experience.
- **Focus Mode**: A distraction-free writing environment with customizable settings enhances concentration and productivity for writers.

### State Management Patterns
- **Persistent User Preferences**: Use Zustand's persist middleware for storing user preferences across sessions, providing a seamless experience when users return to the application.
- **Feature Toggles**: Implement toggleable features (like focus mode) with clear state indicators and consistent behavior across the application.
- **Settings Organization**: Group related settings together in the state store for better organization and easier management of feature configurations.
- **Default Values**: Provide sensible default values for all settings to ensure a good out-of-the-box experience while allowing customization.
- **Hybrid Persistence**: Use Zustand's persist middleware with onRehydrateStorage for hybrid local/remote persistence. This ensures settings are consistent across sessions and devices.
- **Data Type Flexibility**: Implement support for different data types (string, integer, boolean, JSON) in preference models to accommodate various setting needs.

### Project Documentation and Planning
- **Documentation Consistency**: Maintain consistency across all documentation files (plans, progress, active context) to ensure a single source of truth about project status.
- **Regular Documentation Audits**: Periodically audit documentation against actual implementation to catch discrepancies early and maintain accurate project status.
- **Feature Deferral Documentation**: When deferring features to a later phase, explicitly document this decision in all relevant files to maintain clarity about project scope.
- **UI/Backend Integration Tracking**: Clearly distinguish between UI-only implementations and fully integrated features with backend support in project documentation.
- **Critical Path Identification**: Identify and prioritize critical path items that must be completed before moving to the next phase of development.
- **Completion Criteria**: Define clear, measurable criteria for considering a phase complete to avoid premature advancement to the next phase.
- **Implementation-Documentation Sync**: Update documentation immediately after implementing new features to maintain an accurate project status.
- **UI-Backend Integration Planning**: Plan for both backend implementation and UI integration when designing new features to ensure a complete user experience.
- **Backend Integration Prioritization**: Prioritize backend integration for existing UI components before adding new features to ensure a solid foundation.
- **Documentation Process**: Implement a more rigorous process for updating documentation when features are completed or deferred to maintain accuracy.
- **Documentation Synchronization Pattern**: When updating project plans, ensure all related documentation files (progress.md, activeContext.md, changelog.md) are updated simultaneously to maintain consistency across the project documentation ecosystem.
- **Progress Tracking Pattern**: Use consistent symbols (✅, ⏳) across all documentation to clearly indicate completion status, making it easier to quickly assess project progress at a glance.
- **Project Phase Transition Checklist**: Create a standardized checklist for transitioning between project phases, including documentation updates, final testing, and preparation for the next phase to ensure smooth transitions and prevent premature advancement.
- **Parallel Development Pattern**: Effectively manage the transition between project phases by allowing initial work on the next phase to begin while completing the final tasks of the current phase. This creates a smoother transition and prevents delays.
- **Component Status Tracking**: Use a consistent system for tracking the status of individual components within a larger feature (e.g., ✅ for completed, ⏳ for in progress) to provide a more granular view of progress.
- **Documentation-Code Synchronization**: Ensure that documentation is updated whenever significant code changes are made, especially when implementing features ahead of schedule, to maintain an accurate representation of the codebase.

## Project-Specific Implementations

### Focus Mode Implementation
- **Component Structure**: Separate the focus mode into distinct components (toggle, settings, hint) for better maintainability and separation of concerns.
- **Visual Transitions**: Implement smooth transitions when entering/exiting focus mode to avoid jarring visual changes.
- **Customization Options**: Provide users with options to customize their focus mode experience (hiding panels, dimming UI) to accommodate different preferences.
- **Keyboard Control**: Make focus mode fully accessible via keyboard shortcuts for writers who prefer keyboard-centric workflows.
- **State Persistence**: Remember user's focus mode settings across sessions to provide a consistent experience.
- **Accessibility**: Ensure focus mode respects accessibility preferences like reduced motion and maintains proper contrast ratios.
- **Typewriter Mode**: Consider adding a "typewriter mode" that keeps the current line centered in the editor for improved focus during writing.
- **Font Size Adjustments**: Add options for adjusting font size and line width in focus mode to enhance readability and reduce eye strain.

### Database Backup System
- **Automatic Backup Strategy**: Implement configurable automatic backups (hourly, daily, weekly) with rotation to balance data protection and storage usage.
- **Pre-Restore Safety Measures**: Create a backup of the current state before performing a restore operation to prevent data loss in case of restore failures.
- **Backup Metadata**: Store metadata about backups (creation time, type, comment) to help users identify and manage their backups.
- **Cleanup Policies**: Implement configurable policies for automatic cleanup of old backups to prevent excessive disk usage.
- **Backup Compression**: Consider adding compression for backups to reduce storage requirements while maintaining data integrity.
- **Backup Rotation Strategy**: Implement a more sophisticated backup rotation strategy to balance storage usage with data protection needs.

### Trash Management System
- **Metadata Preservation**: Preserve metadata (original location, deletion time, reason) when moving items to trash to enable proper restoration.
- **Hierarchical Trash**: Support trashing of hierarchical structures (projects with nested documents) while maintaining relationships.
- **Restoration Strategy**: Implement a restoration process that handles conflicts and ensures data integrity when restoring items.
- **Permanent Deletion**: Provide clear separation between moving to trash and permanent deletion to prevent accidental data loss.

### Document Version History
- **Version Creation Triggers**: Create versions at meaningful points (manual saves, significant changes) rather than for every small edit.
- **Metadata Enrichment**: Store metadata with versions (author, comment, timestamp) to provide context for changes.
- **Efficient Storage**: Balance storage efficiency with functionality by storing only the differences between versions when appropriate.
- **Version Comparison**: Provide tools for comparing different versions to help users understand changes over time.

### Background Processing System
- **Task Queue Management**: Implement a managed task queue for long-running operations to prevent UI blocking and improve user experience.
- **Task Prioritization**: Use a prioritization system (e.g., user-initiated vs. background) to ensure important tasks are processed first.
- **Status Tracking**: Implement comprehensive status tracking (queued, running, completed, failed) to provide feedback on task progress.
- **Database Persistence**: Store tasks in the database to ensure they survive application restarts and can be resumed.
- **Error Handling**: Implement robust error handling for background tasks with proper status updates and recovery options.
- **Task Processor Specialization**: Create specialized processors for different types of tasks (e.g., AI processor for AI-related tasks) to modularize the system.
- **Command Interface**: Provide a clean command interface for creating, canceling, and checking the status of background tasks.
- **Task Cancellation**: Consider adding task cancellation for user-initiated tasks to provide more control over long-running operations.
- **Progress Reporting**: Implement task progress reporting for better user feedback during long-running operations.
- **Task Dependencies**: Add task dependencies for complex workflows that require sequential execution of multiple tasks.

### Performance Monitoring System
- **Metric Sampling Strategy**: Use sampling for high-frequency metrics to reduce overhead while still providing valuable insights.
- **Configurable Thresholds**: Implement configurable thresholds for bottleneck detection to adapt to different environments and use cases.
- **Database Schema Design**: Create a flexible database schema that can store different types of performance metrics with proper indexing for efficient querying.
- **Component-Level Monitoring**: Implement React hooks for component-level performance monitoring to identify UI bottlenecks.
- **Memory Usage Tracking**: Track memory usage with component breakdown to identify memory leaks and optimize resource usage.
- **Query Performance Analysis**: Monitor database query performance to identify slow queries and optimize database operations.
- **Visual Dashboard**: Create a comprehensive dashboard for visualizing performance metrics with real-time updates and filtering.
- **Settings Integration**: Provide configuration options for enabling/disabling different monitoring features to balance performance and overhead.
- **Documentation**: Create detailed documentation for using the performance monitoring system effectively, including best practices and troubleshooting.
- **Automatic Cleanup**: Implement automatic cleanup of old metrics data to prevent database bloat while maintaining historical trends.
- **Bottleneck Detection**: Use a combination of thresholds and trend analysis to automatically detect performance bottlenecks.
- **Performance Visualization**: Use appropriate visualizations (charts, tables, indicators) to make performance data easily understandable.
- **Advanced Visualizations**: Consider adding more sophisticated visualizations like heatmaps for query performance to provide deeper insights.
- **Optimization Recommendations**: Implement automatic performance optimization recommendations based on collected metrics to guide developers.
- **Regression Testing**: Add performance regression testing for critical paths in the application to catch performance degradation early.

### AI Writing Features Implementation
- **AI Provider Abstraction**: Use a trait-based approach for AI service integrations that defines a common interface for all providers, allowing for easy addition of new providers and features.
- **Writing Mode Separation**: Implement different writing modes (Auto, Guided, Tone Shift) as separate methods with clear responsibilities to improve maintainability and testability.
- **Context Building Strategy**: Create a dedicated context builder that assembles relevant information from various sources (document content, story bible, user preferences) to provide rich context for AI generation.
- **Token Management**: Implement precise token counting and credit calculation to manage API usage efficiently and provide accurate cost estimates to users.
- **Card System Organization**: Organize AI responses into a card system with stacking, filtering, and sorting capabilities to help users manage and reference generated content.
- **Streaming Text Generation**: Implement streaming support for real-time text generation to provide immediate feedback and a more interactive writing experience.
- **Feature-Specific Context**: Tailor the context provided to AI based on the specific feature being used (Write, Rewrite, Expand, etc.) to optimize for relevant and high-quality responses.
- **Error Recovery Strategies**: Implement specific error recovery strategies for different types of AI-related failures (network issues, rate limits, content filtering) to provide a robust user experience.
- **Creativity Control**: Provide users with configurable creativity levels (1-10) to control the balance between predictable and creative AI-generated content.
- **Purple Text Highlighting**: Use visual indicators (purple text) to distinguish AI-generated content from user-written content, with automatic removal on edit to maintain a clean interface.
- **Quick Tools Integration**: Implement Quick Edit and Quick Chat as lightweight, context-aware tools that can be accessed quickly without disrupting the writing flow.
- **High Quality Mode**: Provide a High Quality mode option that uses more tokens and more careful processing for important writing tasks, with appropriate credit system warnings.
