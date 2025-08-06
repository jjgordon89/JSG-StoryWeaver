# PROJECT RULES

## Development Phase Awareness

- Current implementation should align with the 6-phase development plan (24 weeks total)
- Reference the specific phase requirements when implementing features
- Follow the dependency chain: Foundation → Writing Features → Story Bible → Advanced AI → Collaboration → Polish
- Implement features incrementally according to the established success criteria

## Technology Stack Compliance

- **Backend:** Use Rust 1.70+ with Tauri 2.0 framework and required plugins
  - Core plugins: `tauri-plugin-fs`, `tauri-plugin-dialog`, `tauri-plugin-notification`, `tauri-plugin-window-state`
  - Additional plugins: `tauri-plugin-clipboard-manager`, `tauri-plugin-shell`, `tauri-plugin-updater`, `tauri-plugin-global-shortcut`, `tauri-plugin-store`
- **Frontend:** Use React 18+ with TypeScript 5.0+, Vite 4.5+, and Tailwind CSS
  - UI Components: Radix UI primitives for accessibility and consistency
  - State Management: Zustand for global state, React Query for server state
- **Database:** SQLite 3.40+ for structured data, LanceDB 0.4+ for vector operations
  - Features: JSON support, FTS5 for full-text search, WAL mode for performance
  - Use SQLx with prepared statements and connection pooling
- **AI:** Support OpenAI, Claude, Gemini, and OpenAI-compatible providers
  - OpenAI-compatible: Together AI, Groq, OpenRouter, Perplexity, custom endpoints
  - Local AI: Ollama integration for offline capabilities
  - Embeddings: text-embedding-3-small/large, Gemini embeddings, local models
- **Testing:** Use Vitest for unit tests, Playwright for E2E tests
- **Plugin System:** WASM runtime (wasmtime) for secure plugin sandboxing
- **Vector Operations:** candle-core for ML operations, tiktoken-rs for token counting
- **Document Processing:** Support .docx, .txt, .rtf, .odt, .csv formats

## Security & Privacy Standards

- **API Key Storage:** Use OS keychain integration for secure storage of API keys
- **Data Privacy:** Never log sensitive user data (API keys, document content, personal information)
- **Input Validation:** Implement proper sanitization and validation for all user inputs
- **Plugin Security:** Use WASM sandboxing with proper resource limits for plugin execution
- **Local-First Architecture:** No cloud dependencies for core functionality
- **Encryption:** Use ring/aes-gcm for sensitive data encryption when needed

## AI Integration Standards

- **Provider Abstraction:** Always use the AIProvider trait rather than direct API calls
- **Token Management:** Implement proper token counting and cost estimation before requests
- **Context Selection:** Use the Saliency Engine for intelligent Story Bible context selection
- **Rate Limiting:** Respect API limits and implement exponential backoff for failures
- **Response Caching:** Cache AI responses using similarity-based caching system
- **Streaming Support:** Implement streaming responses for real-time generation
- **Error Handling:** Graceful degradation for network and AI provider failures

## Memory Management & Performance

- **Large Documents:** Implement proper cleanup for 100k+ word documents
- **AI Response Caching:** Manage memory usage for cached responses
- **Vector Operations:** Optimize embedding storage and retrieval
- **Plugin Resources:** Monitor and limit plugin memory usage
- **Database Connections:** Use connection pooling and proper indexing
- **Async Operations:** Use async/await patterns consistently

## Feature Implementation Guidelines

- **Story Bible System:** Implement visibility controls, series support, and intelligent context selection
- **Writing Tools:** Follow the established credit costs and word limits (Auto Write: 150-200 words, Expand: max 1000 words, etc.)
- **Plugin System:** Use WASM sandboxing, support multi-stage prompts, and implement the established variable system
- **Collaboration:** Implement Clean Copy commenting with proper permission management
- **Canvas:** Use drag-and-drop for visual planning with established outline templates
- **Three-Column Layout:** Implement responsive design with proper breakpoint handling

## Performance Requirements

- Application startup time: < 3 seconds on average hardware
- Large document processing: No degradation for 100k+ word documents
- Memory usage: Stable during extended sessions with proper cleanup
- Database queries: Use proper indexing and connection pooling
- AI response times: Display progress indicators for operations > 2 seconds

## Error Handling Strategy

- Use the comprehensive `StoryWeaverError` enum for all error types
- Implement graceful degradation for network and AI provider failures
- Provide user-friendly error messages with recovery suggestions
- Log errors with proper context for debugging without exposing sensitive data
- Support automatic retry for recoverable errors (network issues, temporary AI failures)

## File Structure Standards

src-tauri/
├── src/
│ ├── commands/ # Tauri command handlers
│ ├── database/ # Database operations and models
│ ├── ai/ # AI provider integrations
│ │ ├── providers/ # Individual AI provider implementations
│ │ ├── context_selection/ # Saliency Engine implementation
│ │ └── caching/ # AI response caching system
│ ├── vector_db/ # LanceDB operations
│ │ ├── embeddings/ # Embedding generation and management
│ │ └── similarity/ # Similarity search algorithms
│ ├── plugins/ # Plugin system core
│ │ ├── wasm/ # WASM runtime and sandboxing
│ │ └── registry/ # Plugin registration and management
│ ├── security/ # Security utilities (keychain, encryption)
│ └── utils/ # Utility functions

src/
├── components/ # Reusable UI components
│ ├── ui/ # Base UI components (Radix UI wrappers)
│ ├── layout/ # Layout components (three-column, responsive)
│ └── common/ # Common reusable components
├── features/ # Feature-specific components
│ ├── editor/ # Main editor functionality
│ ├── storybible/ # Story Bible system
│ ├── canvas/ # Visual planning canvas
│ ├── plugins/ # Plugin management UI
│ └── ai/ # AI tool interfaces
├── hooks/ # Custom React hooks
├── stores/ # State management (Zustand stores)
├── services/ # Tauri invoke functions
├── types/ # TypeScript definitions
└── utils/ # Frontend utility functions

## Documentation Standards

- Include JSDoc comments for all public functions and complex logic
- Document API contracts between frontend and backend
- Maintain inline comments explaining business logic and AI integration points
- Update relevant phase documentation when implementing features
- Include examples in code comments for complex AI prompt templates

## Testing Requirements

- **Unit Tests:** All utility functions, data processing logic, and AI integration components
- **Integration Tests:** AI provider interactions, database operations, and plugin system
- **E2E Tests:** Complete user workflows (project creation → writing → AI assistance)
- **Performance Tests:** Large document handling, memory usage, and vector operations
- **Security Tests:** Plugin sandboxing, input validation, and API key handling
- **Accessibility Tests:** Use jest-axe for all UI components and keyboard navigation
- **Plugin Tests:** WASM plugin execution and resource limit enforcement
- **AI Tests:** Token counting accuracy, cost estimation, and response caching
