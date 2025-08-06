# User Rules

## Development Phase Awareness

- Always align implementation with the 6-phase development plan (24 weeks total)
- Reference specific phase requirements when implementing features
- Follow the dependency chain: Foundation → Writing Features → Story Bible → Advanced AI → Collaboration → Polish
- Implement features incrementally according to established success criteria

## Code Quality & Architecture

- **Technology Stack Compliance:**
  - Backend: Rust 1.70+ with Tauri 2.0 framework and required plugins
  - Frontend: React 18+ with TypeScript 5.0+, Vite 4.5+, and Tailwind CSS
  - Database: SQLite 3.40+ with JSON support, FTS5, WAL mode; LanceDB 0.4+ for vectors
  - Testing: Vitest for unit tests, Playwright for E2E tests

- **Required Tauri Plugins:**
  - Core: `tauri-plugin-fs`, `tauri-plugin-dialog`, `tauri-plugin-notification`, `tauri-plugin-window-state`
  - Additional: `tauri-plugin-clipboard-manager`, `tauri-plugin-shell`, `tauri-plugin-updater`, `tauri-plugin-global-shortcut`, `tauri-plugin-store`

- Use async/await patterns consistently and implement proper error handling with the comprehensive `StoryWeaverError` enum

- Follow the established database schema and use SQLx with prepared statements and connection pooling

- **Memory Management & Performance:**
  - Implement proper cleanup for 100k+ word documents
  - Manage AI response caching memory usage
  - Optimize vector operations and embedding storage
  - Monitor and limit plugin memory usage
  - Use connection pooling and proper database indexing

- **State Management Pattern:**
  - Zustand for global state
  - React Query for server state
  - Local state for UI components only

## AI Integration Standards

- **Provider Support:** OpenAI, Claude, Gemini, and OpenAI-compatible providers (Together AI, Groq, OpenRouter, Perplexity, custom endpoints)
- **Local AI:** Ollama integration for offline capabilities
- **Embeddings:** text-embedding-3-small/large, Gemini embeddings, local models

- **Implementation Requirements:**
  - Always use the AIProvider trait abstraction rather than direct API calls
  - Implement proper token counting using tiktoken-rs and cost estimation before requests
  - Use the Saliency Engine for intelligent Story Bible context selection
  - Respect rate limiting and implement exponential backoff for API failures
  - Cache AI responses using similarity-based caching system
  - Implement streaming responses for real-time generation
  - Graceful degradation for network and AI provider failures

## Security & Privacy Standards

- **Data Privacy:** Never log sensitive user data (API keys, document content, personal information)
- **API Key Storage:** Use OS keychain integration for secure storage of API keys
- **Input Validation:** Implement proper sanitization and validation for all user inputs
- **Plugin Security:** Use WASM sandboxing (wasmtime) with proper resource limits for plugin execution
- **Local-First Architecture:** No cloud dependencies for core functionality
- **Encryption:** Use ring/aes-gcm for sensitive data encryption when needed
- **Document Processing:** Support .docx, .txt, .rtf, .odt, .csv formats securely

## User Experience & Interface

- **Performance Requirements:**
  - Application startup time: < 3 seconds on average hardware
  - Large document processing: No degradation for 100k+ word documents
  - Memory usage: Stable during extended sessions with proper cleanup
  - AI response times: Display progress indicators for operations > 2 seconds

- **UI/UX Standards:**
  - Implement loading states and progress indicators for all async operations
  - Provide clear error messages with actionable recovery steps
  - Use Radix UI primitives for accessibility and consistency
  - Implement the three-column responsive layout with proper breakpoint handling

- **Accessibility Requirements:**
  - ARIA labels for all interactive elements
  - Semantic HTML structure
  - Full keyboard navigation support
  - Use jest-axe for accessibility testing

## Feature Implementation Guidelines

- **Story Bible System:** Implement visibility controls, series support, and intelligent context selection
- **Writing Tools:** Follow established credit costs and word limits (Auto Write: 150-200 words, Expand: max 1000 words)
- **Plugin System:** Use WASM sandboxing, support multi-stage prompts, implement established variable system
- **Collaboration:** Implement Clean Copy commenting with proper permission management
- **Canvas:** Use drag-and-drop for visual planning with established outline templates

## Testing & Documentation Standards

- **Required Tests:**
  - Unit tests for all utility functions and AI integration components
  - Integration tests for AI providers, database operations, and plugin system
  - E2E tests for complete user workflows
  - Performance tests for large document handling and vector operations
  - Security tests for plugin sandboxing and input validation

- **Documentation Requirements:**
  - JSDoc comments for all public functions and complex logic
  - Document API contracts between frontend and backend
  - Inline comments explaining business logic and AI integration points
  - Examples in code comments for complex AI prompt templates
