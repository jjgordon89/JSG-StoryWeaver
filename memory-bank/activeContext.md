# Active Context: StoryWeaver

## Current Work Focus
We're currently working on Phase 1 of the StoryWeaver project, focusing on establishing the foundation for the application. We've made significant progress (~70% complete) on the core functionality and are now moving to implement the remaining features.

## Recent Events (Last 10)
- **2025-08-06:** Implemented AI Provider abstraction layer with OpenAI and Claude providers.
- **2025-08-06:** Updated the Phase 1 Foundation plan to reflect our progress (70% complete).
- **2025-08-06:** Implemented DocumentLinking component for chapter continuity.
- **2025-08-06:** Created SeriesManager component for multi-project workflows.
- **2025-08-06:** Implemented FolderHierarchy component with drag-and-drop support.
- **2025-08-06:** Enhanced DocumentEditor with improved auto-save and word count tracking.
- **2025-08-06:** Updated ProjectList to display documents and handle document selection.
- **2025-08-06:** Updated ProjectCard to support selection state.
- **2025-08-06:** Updated ProjectView to accept document selection callbacks.
- **2025-08-06:** Enhanced MainLayout to integrate document editor and navigation.
- **2025-08-05:** Initialized Memory Bank by creating the `memory-bank/` directory and core documentation files.

## Next Steps
1. ~~**Implement AI provider abstraction layer:**~~ ✅ Completed
2. **Create card system UI:** Develop the UI for AI responses and interactions.
3. **Add theme support:** Implement dark/light theme support for accessibility.
4. **Configure Windows MSI build:** Set up the build configuration for distribution.

## Active Decisions & Considerations
- We've implemented a modular AI Provider system with a trait-based abstraction layer, allowing easy integration of different AI services.
- The AI Provider system includes rate limiting and token counting to manage API usage efficiently.
- Both OpenAI and Claude providers have been implemented, demonstrating the modularity of the system.
- We're using a component-based architecture with React and TypeScript for the frontend.
- Monaco Editor is being used for the document editor with custom hooks for features.
- We're using Zustand for state management and React Query for server state.
- The folder hierarchy uses a recursive component approach for displaying nested folders.
- Series support allows projects to share story bible data for consistent worldbuilding.
- Document linking enables continuity between chapters with bidirectional navigation.
