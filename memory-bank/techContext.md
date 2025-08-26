# Tech Context: StoryWeaver

## Backend (Rust)
- **Framework:** Tauri 2.0
- **Async Runtime:** Tokio
- **Database:** SQLite
- **Database ORM/Driver:** SQLx
- **Serialization:** Serde
- **UUIDs:** uuid

## Frontend (TypeScript)
- **Framework:** React 18
- **Build Tool:** Vite
- **State Management:**
    - Zustand (Global UI State)
    - TanStack React Query (Server State & Caching)
- **UI Components:**
    - Radix UI (Headless components for accessibility)
    - Tailwind CSS (Styling)
- **Editor:** Monaco Editor
- **Internationalization**: `react-i18next`, `i18next`

## Development Environment
- **Package Manager:** Cargo (Rust), pnpm/npm/yarn (Node.js)
- **Build System:** Vite for frontend, Cargo for backend. Tauri CLI orchestrates the final build.
- **Hot Reloading:** Supported for both frontend and backend changes.
- **Testing:** Playwright for e2e testing across Chromium, Firefox, and WebKit browsers
- **E2E Test Infrastructure:** Stable and reliable; all tests pass consistently across browsers

## Key Dependencies & Justification
- **Tauri 2.0:** Chosen for its performance (Rust backend), security, and ability to create cross-platform desktop apps from a web-based frontend.
- **SQLx:** Preferred over Diesel for its async-first approach and compile-time query validation, which fits well with the Tokio runtime.
- **React Query:** Essential for managing the complexity of server state (data from the Rust backend), handling caching, and preventing unnecessary data fetching.
- **Radix UI:** Provides a solid foundation of accessible, unstyled UI components, allowing for full design control with Tailwind CSS.
- **Project Management:** The project management interface is implemented with a three-column layout, allowing writers to organize their projects with ease.
