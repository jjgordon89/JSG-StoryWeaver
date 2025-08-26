# System Patterns: StoryWeaver

## Architecture
StoryWeaver uses a hybrid desktop application architecture powered by **Tauri 2.0**.

- **Backend:** A monolithic Rust binary that handles all core logic, including database operations, file system access, and AI provider communication. This approach is chosen for performance and security.
- **Frontend:** A modern web-based UI built with **React, TypeScript, and Vite**. The UI communicates with the Rust backend via Tauri's API bridge.

## Core Design Patterns
- **Command-Based Backend:** The backend exposes a set of commands (e.g., `create_project`, `get_document`) that the frontend can invoke. This follows the Command pattern and creates a clear, well-defined API boundary between the frontend and backend.
- **State Management:**
    - **Zustand:** Used for global UI state management (e.g., theme, active project). It's chosen for its simplicity and minimal boilerplate.
    - **React Query:** Used for managing server state, caching data fetched from the backend, and handling asynchronous operations like fetching documents. This helps to keep the UI responsive and reduce redundant backend calls.
- **Error Handling:** A centralized `StoryWeaverError` enum is used throughout the Rust backend to provide consistent and descriptive error types. These errors are propagated to the frontend, where they can be handled gracefully.
- **Database:**
    - **SQLite:** A single-file, embedded database is used for all data storage. This simplifies deployment and keeps all project data self-contained.
    - **SQLx:** Provides compile-time checked queries and an async interface to the database.
    - **Connection Pooling:** A database connection pool is used to manage concurrent database access efficiently.
- **AI Abstraction:** An AI provider abstraction layer (trait/interface) is planned. This will allow for swapping out different AI models or services (e.g., OpenAI, Anthropic) with minimal changes to the core application logic.
- **Unified Streaming Envelope**: A standardized streaming envelope will be implemented in `src-tauri/src/ai/streaming.rs` to ensure a consistent data format for real-time events across all AI providers.
- **Internationalization (i18n)**: The application will use `react-i18next` for internationalization. Strings will be externalized to JSON files in `public/locales`, and the React application will be wrapped in an `I18nextProvider`.
- **Project Management:** The project management interface is implemented with a three-column layout, allowing writers to organize their projects with ease.

## AdvancedAI Style Manager Patterns
- **Optimistic UI + Local Persistence (Style Examples):**
  - The `advancedAIStore` manages `styleExamples` with optimistic updates for update/delete/bulk-delete operations.
  - Local fallback persistence via `localStorage` is used to hydrate and persist `styleExamples` across sessions under key `sw_style_examples_v1`.
  - On initialization, the store hydrates from local storage before loading other data to avoid UI flicker.

- **Backend Layering and Compatibility:**
  - Adding and analyzing style examples uses Tauri commands in `advanced_ai_commands.rs`:
    - `add_style_example` (returns `StyleExample` with string `id`, analysis computed in Rust `AdvancedAIManager`)
    - `analyze_text_style` (computes `StyleAnalysis` on-demand)
  - There is a separate DB-backed style_examples subsystem (`src-tauri/src/commands/style_examples.rs`) with numeric IDs and different shapes. The UI currently standardizes on the AdvancedAIManager path to avoid ID/type mismatches. A future migration can unify on one subsystem.

- **Generation Constraints Pipeline (Generate-from-style):**
  - The Style Manager's "Generate" action constructs a `ProseGenerationRequest` and calls `generate_with_prose_mode`.
  - The request sets `style_examples` to an array of selected style example IDs. The backend (`AdvancedAIManager`) inlines matched examples into the enhanced AI context to constrain tone, sentence structure, and vocabulary.
  - Additional request fields (e.g., `ultra_creative`, `max_words`, `use_saliency_engine`) are sourced from store settings to keep behavior consistent across tools.

## Testing Patterns
- **E2E Testing with Playwright:**
  - Cross-browser testing across Chromium, Firefox, and WebKit ensures compatibility
  - Selector patterns use semantic text matching (e.g., `h1:has-text("Projects")`) for robust UI testing
  - Test files follow naming convention: `*.spec.ts` in the `e2e/` directory
  - All tests wait for UI elements to be visible before interaction to prevent flaky tests
  - Consistent test structure: setup → action → assertion → cleanup
