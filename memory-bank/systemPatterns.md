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
- **Project Management:** The project management interface is implemented with a three-column layout, allowing writers to organize their projects with ease.
