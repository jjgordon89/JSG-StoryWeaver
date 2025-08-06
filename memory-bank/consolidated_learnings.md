## General Patterns

### React Component Development

#### Third-Party Library Integration
- **Proper Initialization and Cleanup**: Use React's useRef and useEffect hooks for proper initialization and cleanup of third-party libraries like Monaco Editor. This ensures resources are properly allocated and released.
- **Event Handling**: Set up event listeners in useEffect and clean them up in the return function to prevent memory leaks.
- **Component Lifecycle Management**: Be mindful of component lifecycle when integrating external libraries, especially those with their own lifecycle.

#### Component Architecture
- **Recursive Component Rendering**: Implement recursive component rendering for hierarchical data structures like folder trees. This creates a clean, maintainable approach to displaying nested data.
- **Component Communication**: Use a combination of props, callbacks, and context for component communication based on the scope of data sharing needed.
- **State Management Strategy**: Use local state for component-specific concerns and global state (Zustand, Redux) for application-wide data.
- **Component Organization**: Organize components by feature area (editor, project, etc.) for better code organization and maintainability.

#### TypeScript Best Practices
- **Interface-First Development**: Define clear interfaces for component props before implementation to ensure type safety.
- **Consistent Prop Typing**: Use TypeScript interfaces for consistent prop typing across components to catch errors at compile time.
- **Union Types for State**: Use union types to represent different states of a component (e.g., 'saved' | 'saving' | 'error').

### Troubleshooting Strategies
- **Cascading Errors**: When troubleshooting a series of cascading errors, strip the configuration down to a minimal working state and then re-introduce components one by one.
- **Incremental Development**: Build features incrementally, testing each step before moving to the next to isolate issues early.

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

### Tauri 2.0 Configuration
- **Plugin Configuration**: Tauri 2.0 plugin configuration is very strict. When in doubt, start with a minimal configuration and add plugins back one by one.
- **State Management**: Store provider managers in Tauri state for global backend access across the application.

### Rust Backend Development
- **Async Trait Implementation**: Use async_trait for async trait object safety in Rust when designing plugin/provider abstractions.
- **Module Organization**: Use a dedicated module and trait for each AI provider, re-export via a central module for extensibility.
- **Import Placement**: Always place imports at the top-level to avoid attribute-related syntax errors in Rust.
- **API Integration Patterns**: When integrating with external APIs, create dedicated request/response structs that match the API's schema for reliable serialization/deserialization.
- **Thread Safety**: Use Arc for sharing immutable data across threads and Arc<Mutex<T>> for mutable data that needs synchronization in async contexts.

### StoryWeaver Architecture
- **Document Editor Integration**: Monaco Editor provides a powerful foundation for the document editor with features like syntax highlighting and keyboard shortcuts.
- **Folder Hierarchy**: A recursive component approach works well for displaying and managing nested folder structures.
- **Series Support**: Organizing related projects in series allows for shared story bible data and consistent worldbuilding.
- **Document Linking**: Bidirectional document linking enables continuity between chapters with intuitive navigation.
