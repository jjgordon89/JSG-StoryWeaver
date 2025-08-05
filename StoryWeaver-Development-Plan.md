# StoryWeaver AI Toolkit - Comprehensive Development Plan

## Project Overview

StoryWeaver is a comprehensive AI-powered desktop application for novelists that provides planning, writing, editing, and organizational tools. This single-user Windows desktop application will be built using Tauri with Rust backend, modern web frontend, SQLite for local data storage, and LanceDB for AI vectorization capabilities.

## Technology Stack

### Core Framework
- **Tauri 2.0** - Cross-platform desktop app framework with Rust backend
  - Required plugins: `tauri-plugin-fs`, `tauri-plugin-dialog`, `tauri-plugin-notification`, `tauri-plugin-window-state`
  - Security features: CSP configuration, allowlist restrictions, secure IPC communication
  - Build targets: Windows MSI installer, portable executable
- **Rust 1.70+** - Backend logic, database operations, AI integrations
  - Required for async/await support, advanced type system, memory safety
  - Cargo workspace configuration for modular development
- **TypeScript 5.0+** - Frontend user interface with strict type checking
  - Required for advanced type inference, template literal types, const assertions
- **Vite 4.5+** - Frontend build tool and development server
  - Hot module replacement, optimized bundling, TypeScript support
  - Required plugins: `@vitejs/plugin-react`, `vite-plugin-tauri`

### Database & Storage
- **SQLite 3.40+** - Primary database for structured data
  - Required features: JSON support, FTS5 for full-text search, WAL mode for performance
  - Connection pooling, prepared statements, transaction management
  - Database migrations and schema versioning
- **LanceDB 0.4+** - Vector database for AI embeddings and semantic search
  - Arrow-based columnar storage, SIMD-optimized vector operations
  - Integration with embedding models, similarity search algorithms
  - Automatic indexing and query optimization
- **Local File System** - Document storage and exports
  - Structured directory hierarchy, atomic file operations
  - File watching for external changes, backup management
  - Support for multiple file formats (.docx, .txt, .rtf, .odt, .csv)

### AI Integration
- **OpenAI API Integration**
  - Models: GPT-4o, GPT-4o-mini, GPT-4.1, GPT-3.5-turbo
  - Features: Chat completions, embeddings (text-embedding-3-small/large)
  - Rate limiting, retry logic, error handling, token counting
  - Streaming responses for real-time generation
- **OpenAI-Compatible API Integration**
  - Support for custom base URLs and API endpoints
  - Compatible with providers like Together AI, Groq, Perplexity, OpenRouter, etc.
  - Custom model names and parameter configurations
  - Flexible authentication with custom API keys
  - Rate limiting and error handling per provider
  - Model-specific context window and token limits
- **Google Gemini API Integration**
  - Models: Gemini 1.5 Pro, Gemini 1.5 Flash, Gemini 2.0 Flash
  - Google AI Studio API with safety settings
  - Multi-modal capabilities (text and image input)
  - Function calling and tool use support
  - Content filtering and safety controls
  - Streaming responses and batch processing
- **Anthropic Claude API Integration**
  - Models: Claude 3.5 Sonnet, Claude 3 Opus, Claude 3 Haiku
  - Message API with system prompts, tool use capabilities
  - Content filtering, safety measures, context window management
- **Local AI Models (Optional)**
  - Ollama integration for offline capabilities
  - Model management: download, update, delete local models
  - GGML/GGUF format support, quantization options
  - Hardware acceleration (CUDA, Metal, OpenCL)
- **Embedding Models**
  - Text-to-vector conversion for semantic search
  - OpenAI embeddings (text-embedding-3-small/large)
  - Google Gemini embeddings (text-embedding-004)
  - Local embedding models via Ollama
  - Batch processing for large document collections
  - Caching and persistence of embeddings
  - Similarity metrics and ranking algorithms
- **Image Generation**
  - DALL-E 3 integration for Visualize feature
  - Google Imagen integration (via Gemini API)
  - Image processing and optimization
  - Content policy compliance, safety filtering

### Core Rust Dependencies
```toml
[dependencies]
# Core Framework
tauri = { version = "2.0", features = ["api-all", "macos-private-api"] }
tauri-plugin-fs = "2.0"
tauri-plugin-dialog = "2.0"
tauri-plugin-notification = "2.0"
tauri-plugin-window-state = "2.0"
tauri-plugin-clipboard-manager = "2.0"  # For copy/paste operations
tauri-plugin-shell = "2.0"  # For external command execution

# Async Runtime & Utilities
tokio = { version = "1.35", features = ["full", "tracing"] }
futures = "0.3"
async-trait = "0.1"
tokio-util = "0.7"  # Additional async utilities

# Serialization & Data Handling
serde = { version = "1.0", features = ["derive", "rc"] }
serde_json = "1.0"
serde_yaml = "0.9"
bincode = "1.3"
toml = "0.8"  # For configuration files

# Database Operations
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "json", "chrono", "uuid", "migrate"] }
lancedb = "0.4"
rusqlite = { version = "0.30", features = ["bundled", "json", "backup"] }
sea-query = "0.30"  # SQL query builder for complex queries

# HTTP Client & API Integration
reqwest = { version = "0.11", features = ["json", "stream", "multipart", "cookies"] }
url = "2.4"
base64 = "0.21"
mime = "0.3"  # MIME type handling

# AI & Vector Operations
candle-core = { version = "0.3", features = ["cuda", "metal"] }
candle-nn = "0.3"
candle-transformers = "0.3"
tokenizers = "0.15"
hf-hub = "0.3"
tiktoken-rs = "0.5"  # Token counting for OpenAI models

# File Operations & Processing
csv = "1.3"
zip = { version = "0.6", features = ["deflate", "time"] }
walkdir = "2.4"
notify = "6.1"
tempfile = "3.8"
fs_extra = "1.3"  # Extended file system operations

# Document Processing
mammoth-rs = "0.2"  # .docx reading
docx-rs = "0.4"     # .docx writing
rtf-parser = "0.3"  # .rtf support
odt-rs = "0.1"      # .odt support
pdf-writer = "0.9"  # PDF generation

# Text Processing & Analysis
regex = "1.10"
unicode-segmentation = "1.10"
similar = "2.3"     # Text diffing
pulldown-cmark = "0.9"  # Markdown processing
syntect = "5.1"     # Syntax highlighting
tree-sitter = "0.20"  # Text parsing

# Image Processing (for Visualize feature)
image = { version = "0.24", features = ["png", "jpeg", "webp"] }
imageproc = "0.23"  # Image processing operations
photon-rs = "0.3"   # Additional image effects

# Utilities
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
config = "0.13"
dirs = "5.0"
once_cell = "1.19"  # Lazy static initialization
dashmap = "5.5"     # Concurrent HashMap

# Caching & Performance
lru = "0.12"        # LRU cache implementation
moka = { version = "0.12", features = ["future"] }  # High-performance cache
rayon = "1.8"       # Data parallelism

# Security & Encryption
ring = "0.17"       # Cryptographic operations
argon2 = "0.5"      # Password hashing
aes-gcm = "0.10"    # Encryption for sensitive data
keyring = "2.0"     # OS keychain integration

# Plugin System
wasmtime = "15.0"   # WASM runtime for plugin sandboxing
wit-bindgen = "0.16"
wasmtime-wasi = "15.0"  # WASI support for plugins

# Rate Limiting & Throttling
governor = "0.6"    # Rate limiting
leaky-bucket = "1.0"  # Token bucket rate limiting

# Backup & Recovery
tar = "0.4"         # Archive creation
flate2 = "1.0"      # Compression
```

### Frontend Dependencies
```json
{
  "dependencies": {
    // Core React Ecosystem
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@types/react": "^18.2.45",
    "@types/react-dom": "^18.2.18",
    
    // Tauri Integration
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/plugin-fs": "^2.0.0",
    "@tauri-apps/plugin-dialog": "^2.0.0",
    "@tauri-apps/plugin-notification": "^2.0.0",
    "@tauri-apps/plugin-clipboard-manager": "^2.0.0",
    "@tauri-apps/plugin-shell": "^2.0.0",
    
    // State Management & Data Fetching
    "@tanstack/react-query": "^5.17.0",
    "@tanstack/react-query-devtools": "^5.17.0",
    "zustand": "^4.4.7",
    "immer": "^10.0.3",
    "jotai": "^2.6.0",  // Additional state management for complex UI state
    
    // UI Framework & Components
    "@radix-ui/react-dialog": "^1.0.5",
    "@radix-ui/react-dropdown-menu": "^2.0.6",
    "@radix-ui/react-tabs": "^1.0.4",
    "@radix-ui/react-tooltip": "^1.0.7",
    "@radix-ui/react-popover": "^1.0.7",
    "@radix-ui/react-select": "^2.0.0",
    "@radix-ui/react-slider": "^1.1.2",
    "@radix-ui/react-switch": "^1.0.3",
    "@radix-ui/react-progress": "^1.0.3",
    "@radix-ui/react-separator": "^1.0.3",
    "@radix-ui/react-scroll-area": "^1.0.5",
    "@radix-ui/react-context-menu": "^2.1.5",
    "@radix-ui/react-hover-card": "^1.0.7",
    "@radix-ui/react-accordion": "^1.1.2",
    "@radix-ui/react-collapsible": "^1.0.3",
    "@radix-ui/react-toggle": "^1.0.3",
    "@radix-ui/react-toggle-group": "^1.0.4",
    
    // Text Editor & Rich Text
    "@monaco-editor/react": "^4.6.0",
    "monaco-editor": "^0.45.0",
    "@uiw/react-textarea-code-editor": "^2.1.9",
    "react-markdown": "^9.0.1",
    "remark-gfm": "^4.0.0",
    "rehype-highlight": "^7.0.0",
    "rehype-raw": "^7.0.0",
    "@lexical/react": "^0.12.5",  // Alternative rich text editor
    "lexical": "^0.12.5",
    "prosemirror-state": "^1.4.3",  // For advanced text editing
    "prosemirror-view": "^1.32.7",
    "prosemirror-model": "^1.19.4",
    
    // Drag & Drop, Canvas, Visualization
    "react-dnd": "^16.0.1",
    "react-dnd-html5-backend": "^16.0.1",
    "react-beautiful-dnd": "^13.1.1",  // For folder/project organization
    "reactflow": "^11.10.4",
    "@xyflow/react": "^12.0.0",
    "d3": "^7.8.5",
    "@types/d3": "^7.4.3",
    "konva": "^9.2.0",  // 2D canvas library for visual planning
    "react-konva": "^18.2.10",
    "fabric": "^5.3.0",  // Alternative canvas library
    
    // Animation & Transitions
    "framer-motion": "^10.16.16",
    "react-spring": "^9.7.3",
    "lottie-react": "^2.4.0",
    "react-transition-group": "^4.4.5",
    "auto-animate": "^0.8.0",  // Simple animations
    
    // Styling & Design System
    "tailwindcss": "^3.4.0",
    "@tailwindcss/typography": "^0.5.10",
    "@tailwindcss/forms": "^0.5.7",
    "@tailwindcss/container-queries": "^0.1.1",
    "class-variance-authority": "^0.7.0",
    "clsx": "^2.0.0",
    "tailwind-merge": "^2.2.0",
    "styled-components": "^6.1.8",  // For complex styling needs
    "@emotion/react": "^11.11.1",
    "@emotion/styled": "^11.11.0",
    
    // Icons & Assets
    "lucide-react": "^0.300.0",
    "@heroicons/react": "^2.0.18",
    "react-icons": "^4.12.0",
    "@tabler/icons-react": "^2.47.0",
    "phosphor-react": "^1.4.1",
    
    // Form Handling & Validation
    "react-hook-form": "^7.48.2",
    "@hookform/resolvers": "^3.3.2",
    "zod": "^3.22.4",
    "yup": "^1.4.0",  // Alternative validation
    "formik": "^2.4.5",  // Alternative form library
    
    // Utilities & Helpers
    "date-fns": "^3.0.6",
    "lodash-es": "^4.17.21",
    "@types/lodash-es": "^4.17.12",
    "nanoid": "^5.0.4",
    "fuse.js": "^7.0.0",
    "react-hotkeys-hook": "^4.4.1",
    "react-use": "^17.4.2",
    "use-debounce": "^10.0.0",
    "react-intersection-observer": "^9.5.3",
    "react-window": "^1.8.8",  // Virtualization for large lists
    "react-window-infinite-loader": "^1.0.9",
    "@types/react-window": "^1.8.8",
    
    // File Processing & Export
    "jspdf": "^2.5.1",
    "html2canvas": "^1.4.1",
    "file-saver": "^2.0.5",
    "@types/file-saver": "^2.0.7",
    "papaparse": "^5.4.1",
    "@types/papaparse": "^5.3.14",
    "docx": "^8.5.0",  // Document generation
    "mammoth": "^1.6.0",  // .docx to HTML conversion
    "jszip": "^3.10.1",  // ZIP file handling
    "pdfjs-dist": "^4.0.379",  // PDF processing
    
    // Image Processing (for Visualize feature)
    "canvas": "^2.11.2",
    "sharp": "^0.33.2",  // Image processing
    "react-image-crop": "^11.0.5",  // Image cropping
    "react-image-gallery": "^1.3.0",  // Image gallery
    
    // Search & Filtering
    "match-sorter": "^6.3.1",  // Fuzzy search
    "fast-fuzzy": "^1.12.0",  // Fast fuzzy search
    "flexsearch": "^0.7.43",  // Full-text search
    
    // Keyboard & Shortcuts
    "hotkeys-js": "^3.12.2",  // Keyboard shortcuts
    "mousetrap": "^1.6.5",  # Alternative keyboard library
    "@types/mousetrap": "^1.6.15",
    
    // Collaboration & Real-time
    "socket.io-client": "^4.7.4",  // For future real-time features
    "yjs": "^13.6.10",  // CRDT for collaboration
    "y-websocket": "^1.5.0",
    
    // Development & Testing
    "react-error-boundary": "^4.0.11",
    "@sentry/react": "^7.91.0",
    "react-helmet-async": "^2.0.4",  // Head management
    "react-router-dom": "^6.20.1",  // For potential routing needs
    
    // Performance & Monitoring
    "web-vitals": "^3.5.0",  // Performance monitoring
    "react-tracked": "^1.7.12",  // Performance optimization
    
    // Accessibility
    "@reach/skip-nav": "^0.18.0",  // Skip navigation
    "@reach/visually-hidden": "^0.18.0",  // Screen reader support
    "focus-trap-react": "^10.2.3",  // Focus management
    
    // Internationalization (for future)
    "react-i18next": "^13.5.0",
    "i18next": "^23.7.16"
  },
  "devDependencies": {
    // Build Tools
    "@vitejs/plugin-react": "^4.2.1",
    "vite": "^5.0.10",
    "vite-plugin-tauri": "^0.2.0",
    "vite-plugin-pwa": "^0.17.4",  // PWA support
    "rollup-plugin-visualizer": "^5.12.0",  // Bundle analysis
    
    // TypeScript
    "typescript": "^5.3.3",
    "@types/node": "^20.10.6",
    "@types/canvas": "^2.11.6",
    "@types/fabric": "^5.3.7",
    
    // Linting & Formatting
    "eslint": "^8.56.0",
    "@typescript-eslint/eslint-plugin": "^6.17.0",
    "@typescript-eslint/parser": "^6.17.0",
    "eslint-plugin-react": "^7.33.2",
    "eslint-plugin-react-hooks": "^4.6.0",
    "eslint-plugin-jsx-a11y": "^6.8.0",  // Accessibility linting
    "eslint-plugin-import": "^2.29.1",
    "prettier": "^3.1.1",
    "prettier-plugin-tailwindcss": "^0.5.10",
    
    // Testing
    "@testing-library/react": "^14.1.2",
    "@testing-library/jest-dom": "^6.2.0",
    "@testing-library/user-event": "^14.5.1",
    "vitest": "^1.1.0",
    "jsdom": "^23.0.1",
    "@vitest/ui": "^1.1.0",
    "happy-dom": "^12.10.3",  // Alternative to jsdom
    "playwright": "^1.40.1",  // E2E testing
    "@playwright/test": "^1.40.1",
    
    // Development Tools
    "@storybook/react": "^7.6.6",  // Component development
    "@storybook/addon-essentials": "^7.6.6",
    "@storybook/addon-interactions": "^7.6.6",
    "chromatic": "^10.2.0",  // Visual testing
    
    // Tauri CLI
    "@tauri-apps/cli": "^2.0.0"
  }
}
```

### Feature-Specific Technical Requirements

#### AI Writing Tools
- **Token Management**: Precise token counting for all AI models, context window optimization
- **Streaming Support**: Real-time text generation with cancellation capabilities
- **Context Assembly**: Intelligent context building from Story Bible, document history, and user selections
- **Response Caching**: LRU cache for AI responses, deduplication of similar requests
- **Error Recovery**: Graceful handling of API failures, automatic retries with exponential backoff

#### Plugin System
- **WASM Runtime**: Secure sandboxed execution environment using Wasmtime
- **Variable Injection**: Dynamic template processing with Story Bible data
- **Multi-Stage Execution**: Sequential prompt processing with intermediate result handling
- **Resource Limits**: Memory and execution time constraints for plugin safety
- **Marketplace Integration**: Plugin discovery, installation, and update mechanisms

#### Document Management
- **Real-time Collaboration**: Operational Transform (OT) for conflict resolution
- **Version Control**: Git-like versioning with branching and merging capabilities
- **Auto-save**: Debounced saving with conflict detection and resolution
- **Import/Export**: Comprehensive file format support with metadata preservation
- **Search & Indexing**: Full-text search with fuzzy matching and relevance scoring

#### Canvas/Visual Planning
- **Rendering Engine**: Hardware-accelerated 2D graphics using Canvas API
- **Interaction System**: Multi-touch support, gesture recognition, keyboard shortcuts
- **Layout Algorithms**: Automatic node positioning, force-directed graphs
- **Export Capabilities**: High-resolution image export, vector format support
- **Performance Optimization**: Viewport culling, level-of-detail rendering

#### Story Bible System
- **Relationship Mapping**: Graph-based character and world element relationships
- **Visibility Engine**: Fine-grained access control with inheritance rules
- **Template System**: Extensible card templates with custom field types
- **Validation Rules**: Data consistency checks and constraint enforcement
- **Series Synchronization**: Cross-project data sharing with conflict resolution

### Security & Privacy
- **Local-First Architecture**: All data stored locally, no cloud dependencies
- **Encryption**: AES-256-GCM for sensitive data at rest
- **API Key Security**: Secure storage using OS keychain/credential manager
- **Plugin Sandboxing**: WASM-based isolation with capability-based security
- **Input Sanitization**: XSS prevention, SQL injection protection
- **Audit Logging**: Comprehensive activity logging for debugging and security

### Performance Optimization
- **Database Optimization**: Query optimization, connection pooling, prepared statements
- **Memory Management**: Efficient data structures, lazy loading, garbage collection tuning
- **Caching Strategy**: Multi-level caching (memory, disk, network)
- **Background Processing**: Worker threads for CPU-intensive operations
- **Resource Monitoring**: Memory usage tracking, performance metrics collection

### Development Tools & Infrastructure
- **Build System**: Cargo workspaces, conditional compilation, feature flags
- **Testing Framework**: Unit tests (Rust), integration tests, E2E tests (Playwright)
- **CI/CD Pipeline**: Automated testing, cross-platform builds, release automation
- **Documentation**: API documentation, user guides, developer documentation
- **Monitoring**: Error tracking (Sentry), performance monitoring, usage analytics

## Application Architecture

### Backend (Rust/Tauri)
```
src-tauri/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── commands/          # Tauri command handlers
│   │   ├── mod.rs
│   │   ├── projects.rs
│   │   ├── documents.rs
│   │   ├── ai_features.rs
│   │   ├── story_bible.rs
│   │   └── export.rs
│   ├── database/          # Database operations
│   │   ├── mod.rs
│   │   ├── sqlite.rs
│   │   ├── models.rs
│   │   └── migrations.rs
│   ├── ai/               # AI integration
│   │   ├── mod.rs
│   │   ├── openai.rs
│   │   ├── openai_compatible.rs
│   │   ├── gemini.rs
│   │   ├── claude.rs
│   │   ├── embeddings.rs
│   │   └── local_models.rs
│   ├── vector_db/        # LanceDB operations
│   │   ├── mod.rs
│   │   ├── lance.rs
│   │   └── search.rs
│   ├── utils/            # Utility functions
│   │   ├── mod.rs
│   │   ├── file_ops.rs
│   │   └── export.rs
│   └── error.rs          # Error handling
```

### Frontend (React/TypeScript)
```
src/
├── components/           # Reusable UI components
│   ├── ui/              # Base UI components
│   ├── editor/          # Text editor components
│   ├── sidebar/         # Navigation components
│   └── modals/          # Dialog components
├── features/            # Feature-specific components
│   ├── projects/
│   ├── documents/
│   ├── story-bible/
│   ├── ai-tools/
│   ├── canvas/
│   └── plugins/
├── hooks/               # Custom React hooks
├── stores/              # State management
├── services/            # API and Tauri invoke functions
├── types/               # TypeScript type definitions
├── utils/               # Utility functions
└── styles/              # Global styles
```

## Core Features Implementation

### 1. Project Management
- **Hierarchical Organization**: Projects, folders, and series support with shared Story Bible data
- **Document Management**: Create, edit, organize multiple documents per project
- **Chapter Continuity**: Link documents to create continuous story flow for AI context
- **Import/Export**: Support for various file formats (.docx, .txt, .rtf, .odt, .csv)
- **Smart Import**: Novel import with auto-populated Story Bible (up to 120,000 words), character import from text/files (up to 60,000 words, 30 characters max), CSV import for unlimited characters
- **Document Export**: Individual document export as .docx with formatting preservation
- **Project Export**: Complete project export as .zip (excludes Story Bible)
- **Story Bible Export**: Separate CSV export for Outline (chapters/summaries) and Characters (all traits)
- **Version Control**: Track changes and maintain document history
- **Collaboration**: Document sharing with clean copy commenting system, reader display names/anonymous options, private comments between author and individual readers, document duplication for multiple share links, unpublish/republish functionality
- **Backup System**: Automatic local backups

### 2. Story Bible System
- **Centralized Story Management**: Single source of truth for all story elements
- **Braindump**: Free-form text area for core story ideas and vision (influences synopsis)
- **Genre & Style**: Define writing style and genre preferences with style examples
- **Synopsis**: Story summary that influences AI generation and outline creation
- **Characters**: Detailed character profiles with traits, relationships, and visibility controls
- **Worldbuilding**: Organized world details, settings, and lore with customizable cards
- **Outline**: Chapter-by-chapter story structure with unlimited length, document linking, Acts as dividers (Part, Book, Episode, Section), document creation from chapters with automatic linking, outline updating from written documents
- **Scenes & Draft**: Building blocks for chapters with AI-generated options, extra instructions (POV, tense, style notes), Story Bible detection with underlined elements, scene validation with quick fixes, word count and credit estimates
- **Series Support**: Share Characters, Worldbuilding, and Outlines across multiple projects
- **Visibility Controls**: Toggle AI access to specific cards or traits for spoiler management
- **Smart Generation**: AI-powered creation of all Story Bible elements with user refinement

### 3. AI Writing Tools

#### Write Features
- **Auto Write**: Context-aware text continuation (uses up to 1000 words of context, outputs 150-200 words)
- **Guided Write**: Directed writing with user prompts and AI-generated story ideas with suggestions mode
- **Tone Shift**: Style-specific writing variations with preset tones (Ominous, Fantastical, Fast-Paced, Upbeat, Authoritative, Conflicted, Romantic, Sensual)
- **First Draft**: Complete scene generation from prompts (up to 3000 words, only available in empty documents)
- **Write Settings**: Configurable creativity levels, card count (1-5), length settings (~50-500 words), and Key Details for project-level context

#### Editing Tools
- **Rewrite**: Multiple rewriting styles (Rephrase, Shorter, More descriptive, Show-Not-Tell, More Inner Conflict, More Intense) with custom options (max 6,000 words) and configurable card generation count
- **Describe**: Sensory detail generation with toggleable senses (sight, sound, touch, taste, smell) and metaphors (considers paragraph + 200 preceding words)
- **Expand**: Detailed expansion of brief passages (minimum 3 words, maximum 1,000 words) that reads both preceding and following text
- **Brainstorm**: Rapid-fire idea generation with categories (Dialogue, Characters, World building, Plot points, Names, Places, Objects, Descriptions, Article ideas, Tweets, Something else) and "Keepers List" for saving favorites with thumbs up/down voting
- **Quick Tools**: Quick Edit and Quick Chat for in-document AI assistance with High Quality mode toggle and keyboard shortcut (Ctrl/Cmd+K)
- **Selection Menu**: Context-sensitive tools that adapt based on selection length (Describe, Quick Edit, Related Words, Expand, Visualize)
- **Related Words**: Smart thesaurus for finding contextually appropriate alternatives with expandable word cloud view
- **Visualize**: Generate images from text descriptions (minimum 10 words, maximum 3000 words, 2500 credits, 1024x1024 resolution)

#### AI Model Selection & Control
- **Multiple Prose Modes**: Muse, Excellent, Basic and experimental models
- **Saliency Engine**: Intelligent exposure of relevant story information to AI
- **Visibility Settings**: Control AI access to specific Story Bible elements

#### Analysis Tools
- **Content Analysis**: Character consistency, plot holes
- **Style Analysis**: Writing pattern recognition
- **Pacing Analysis**: Scene rhythm and flow

### 4. Plugin System
- **Custom AI Functions**: User-created AI tools with access to Story Bible data for writing, editing, and analysis tasks
- **Plugin Builder**: Visual interface with Basic and Advanced editors for creating plugins with prompts and variables
- **Available Variables**: Access to `highlighted_text`, `preceding_text`, `user_text_input`, `previous_document_text`, `braindump`, `genre`, `style`, `synopsis`, `characters`, `characters_raw`, `outline`, `scene_summary`, `is_story_bible_active`, `chapter_scenes`, `chapter_scenes_extra_instructions`, `worldbuilding`, `worldbuilding_raw`
- **Multi-Stage Prompts**: Support for sequential prompts (up to 2 stages) with intermediate results
- **AI Model Selection**: Choose from various AI engines (GPT-4o-mini, GPT-4.1, Gemini-2.5-pro) with configurable parameters (temperature, frequency penalty, presence penalty, stop sequences, max tokens)
- **Plugin Testing**: Built-in testing environment with sample data and Story Bible context
- **Plugin Marketplace**: Share and discover community-created plugins with visibility controls (published/unlisted)
- **Template System**: Pre-built plugin templates for common writing tasks
- **Plugin Guidelines**: Best practices and validation for plugin creation with category organization
- **Credit System**: Plugins use standard credit system based on AI model usage with variable costs
- **Sandboxed Execution**: Secure plugin environment with controlled access
- **API Integration**: Third-party service connections
- **Profile Management**: Creator name display and plugin attribution system

### 5. Canvas/Visual Planning
- **Digital Whiteboard**: Drag-and-drop story planning with cards, text, and outlines
- **Outline Templates**: Hero's Journey, Hollywood Beats, Story Circle, Romance Outline with distinct visuals
- **Visual Story Mapping**: Character arcs and plot threads
- **Mind Mapping**: Brainstorming visualization
- **Keyboard Shortcuts**: Full shortcut support (Select all, Delete, Pan, Zoom, Undo/Redo, Reset)
- **Outline Generation**: Generate complete outlines from single sentences or paragraphs
- **Copy to Clipboard**: Export canvas outlines to documents or Story Bible

### 6. Advanced Editor
- **Rich Text Editing**: Full-featured writing environment with purple highlighting for AI content
- **Selection Menu**: Context-sensitive tools that appear when text is highlighted
- **Focus Mode**: Distraction-free writing environment
- **Word Count Tracking**: Real-time statistics and goals
- **Auto-save**: Continuous document preservation
- **Document Linking**: Connect documents for chapter continuity and AI context
- **Comment System**: In-document commenting for collaboration and self-notes
- **Version History**: Document revision tracking
- **Point of View (POV) & Tense Settings**: Global and per-chapter control over narrative perspective (1st, 2nd, 3rd person) and tense (past, present, future), with the ability to assign a specific character's POV
- **Story Bible Detection**: Visual highlighting of detected Story Bible elements in text
- **Document Duplication**: Easy document copying for versioning and experimentation

### 7. User Interface
- **Modern Design**: Clean, distraction-free writing environment
- **Customizable Layout**: Adjustable panels and workspace
- **Dark/Light Themes**: Multiple theme options
- **Responsive Design**: Adaptive to different screen sizes
- **Accessibility**: Screen reader support and keyboard navigation
- **History Panel**: Track and review AI generation history with undo/redo capabilities
- **Keyboard Shortcuts**: Comprehensive shortcut system for all major functions

### 8. Advanced AI Features
- **Chapter Continuity**: AI awareness of linked documents for seamless story flow
- **Saliency Engine**: Intelligent selection of relevant Story Bible information for AI
- **Multiple AI Models**: Support for various models (Muse, Claude, GPT) with different capabilities
- **Prose Modes**: Curated model suites optimized for fiction writing
- **Style Examples**: Train AI on user's writing style for personalized prose
- **Credit Management**: Transparent credit usage tracking across all AI features with cost estimates before generation and credit balance display
- **Content Filtering**: Adjustable content filters based on selected AI model
- **Model Switching**: Dynamic model selection based on task with dropdown selection
- **Custom Prompts**: User-defined AI instructions and prompt templates
- **Related Words**: Contextual word suggestions and alternatives with semantic similarity
- **Visualize**: AI-generated images from text descriptions (2500 credits per 1024x1024 image, requires 10-3000 words)
- **Advanced Brainstorming**: Multi-step idea development with context awareness and category-specific prompts
- **Prose Mode Selection**: Dropdown interface for selecting generation modes before AI operations

## Database Schema

### Additional Implementation Notes
- **Template System**: Predefined templates for Story Bible cards (character archetypes, worldbuilding categories)
- **Outline Acts**: Support for named dividers in outlines (Part, Book, Episode, Section)
- **Document Linking**: Automatic bidirectional linking between outline chapters and documents
- **Reverse Sync**: Update outline summaries from written chapter content
- **Word Limits**: Enforce specific word limits across all AI features for optimal performance

### SQLite Tables
```sql
-- Projects and Organization
CREATE TABLE projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    folder_id INTEGER,
    series_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (folder_id) REFERENCES folders(id),
    FOREIGN KEY (series_id) REFERENCES series(id)
);

CREATE TABLE folders (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    parent_folder_id INTEGER,
    is_series BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_folder_id) REFERENCES folders(id)
);

CREATE TABLE series (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    folder_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (folder_id) REFERENCES folders(id)
);

-- Documents
CREATE TABLE documents (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    name TEXT NOT NULL,
    content TEXT,
    word_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Story Bible
CREATE TABLE story_bible (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    braindump TEXT,
    synopsis TEXT,
    genre TEXT,
    style TEXT,
    style_examples TEXT,
    pov_mode TEXT DEFAULT 'global',
    global_pov TEXT DEFAULT '3rd Person Limited',
    global_tense TEXT DEFAULT 'Past',
    global_character_pov_ids JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Document Continuity
CREATE TABLE document_links (
    id INTEGER PRIMARY KEY,
    from_document_id INTEGER NOT NULL,
    to_document_id INTEGER NOT NULL,
    link_order INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (from_document_id) REFERENCES documents(id),
    FOREIGN KEY (to_document_id) REFERENCES documents(id)
);

-- Characters
CREATE TABLE characters (
    id INTEGER PRIMARY KEY,
    project_id INTEGER,
    series_id INTEGER,
    name TEXT NOT NULL,
    description TEXT,
    character_type TEXT,
    traits JSON,
    is_visible BOOLEAN DEFAULT TRUE,
    original_project_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (series_id) REFERENCES series(id),
    FOREIGN KEY (original_project_id) REFERENCES projects(id)
);

CREATE TABLE character_traits (
    id INTEGER PRIMARY KEY,
    character_id INTEGER NOT NULL,
    trait_name TEXT NOT NULL,
    trait_value TEXT,
    is_visible BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id)
);

-- World Building
CREATE TABLE worldbuilding (
    id INTEGER PRIMARY KEY,
    project_id INTEGER,
    series_id INTEGER,
    name TEXT NOT NULL,
    type TEXT,
    description TEXT,
    properties JSON,
    is_visible BOOLEAN DEFAULT TRUE,
    original_project_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (series_id) REFERENCES series(id),
    FOREIGN KEY (original_project_id) REFERENCES projects(id)
);

CREATE TABLE worldbuilding_traits (
    id INTEGER PRIMARY KEY,
    worldbuilding_id INTEGER NOT NULL,
    trait_name TEXT NOT NULL,
    trait_value TEXT,
    is_visible BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (worldbuilding_id) REFERENCES worldbuilding(id)
);

-- Outlines
CREATE TABLE outlines (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    chapter_number INTEGER,
    title TEXT,
    summary TEXT,
    pov TEXT,
    tense TEXT,
    character_pov_ids JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- AI History
CREATE TABLE ai_history (
    id INTEGER PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id),
    document_id INTEGER REFERENCES documents(id),
    feature_type TEXT,
    prompt TEXT,
    response TEXT,
    starred BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Plugins
CREATE TABLE plugins (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    prompt_template TEXT,
    variables JSON,
    is_public BOOLEAN DEFAULT FALSE,
    created_by TEXT,
    test_data JSON,
    guidelines TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Comments and Collaboration
CREATE TABLE document_comments (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    user_name TEXT,
    comment_text TEXT NOT NULL,
    start_position INTEGER,
    end_position INTEGER,
    is_author_comment BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

CREATE TABLE shared_documents (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    share_token TEXT UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    allow_comments BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- AI Model Settings
CREATE TABLE ai_model_settings (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    prose_mode TEXT DEFAULT 'Muse',
    experimental_model TEXT,
    creativity_level INTEGER DEFAULT 5,
    content_filter_level TEXT DEFAULT 'Standard',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Credit Usage Tracking
CREATE TABLE credit_usage (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    feature_name TEXT NOT NULL,
    model_used TEXT,
    credits_used INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- AI Provider Configurations
CREATE TABLE ai_providers (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL, -- 'openai', 'openai_compatible', 'gemini', 'claude', 'ollama'
    base_url TEXT,
    api_key_encrypted TEXT,
    model_name TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    context_window INTEGER,
    max_tokens INTEGER,
    supports_streaming BOOLEAN DEFAULT TRUE,
    supports_images BOOLEAN DEFAULT FALSE,
    rate_limit_rpm INTEGER,
    rate_limit_tpm INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Settings
CREATE TABLE settings (
    id INTEGER PRIMARY KEY,
    key TEXT UNIQUE NOT NULL,
    value TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### LanceDB Schema
```rust
// Document embeddings for semantic search
struct DocumentEmbedding {
    id: String,
    project_id: i32,
    document_id: i32,
    content_chunk: String,
    embedding: Vec<f32>,
    metadata: HashMap<String, String>,
}

// Character embeddings for consistency checking
struct CharacterEmbedding {
    id: String,
    project_id: i32,
    character_id: i32,
    trait_type: String,
    content: String,
    embedding: Vec<f32>,
}
```

## Development Phases

### Phase 1: Foundation (Weeks 1-5)
- Set up Tauri project structure
- Implement basic SQLite database operations
- Create core UI components and layout
- Basic project and document management
- Simple text editor integration
- Series support and folder hierarchy
- Document linking for chapter continuity

### Phase 2: Core Writing Features (Weeks 6-10)
- Implement AI API integrations (OpenAI/Claude)
- Document editor with selection menu
- Core writing tools (Write modes, Rewrite, Expand, Describe)
- Quick tools and Quick Chat
- Related Words feature
- Basic commenting system

### Phase 3: Story Bible System (Weeks 11-15)
- Story Bible foundation (Braindump, Synopsis, Genre, Style)
- Characters and Worldbuilding with visibility controls
- Outline system with document linking
- Scenes & Draft functionality
- Series-level sharing of Story Bible elements

### Phase 4: Advanced AI Features (Weeks 16-19)
- Multiple AI models and prose modes
- Saliency Engine implementation
- Chapter continuity and style examples
- Credit tracking system
- Visualize feature
- Advanced brainstorming with Keepers List
- Smart import with character extraction

### Phase 5: Collaboration & Plugins (Weeks 20-22)
- Document sharing with Clean Copy commenting
- Plugin system with builder interface
- Plugin testing environment and marketplace
- Canvas implementation with drag-and-drop
- Visual story planning tools

### Phase 6: Polish & Optimization (Weeks 23-24)
- Performance optimization
- UI/UX refinements
- Comprehensive testing
- Documentation and help system
- Packaging and distribution setup

## Technical Implementation Details

### AI Integration Strategy
```rust
// AI service abstraction
pub trait AIProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String>;
    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> Result<String>;
    async fn analyze_text(&self, text: &str, analysis_type: &AnalysisType) -> Result<String>;
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>>;
    async fn generate_image(&self, prompt: &str) -> Result<Vec<u8>>;
    fn supports_streaming(&self) -> bool;
    fn supports_images(&self) -> bool;
    fn get_context_window(&self) -> usize;
}

// Context for AI operations
pub struct AIContext {
    pub story_bible: Option<StoryBible>,
    pub characters: Vec<Character>,
    pub worldbuilding: Vec<WorldElement>,
    pub previous_text: String,
    pub user_preferences: UserPreferences,
    pub document_history: Vec<String>,
    pub chapter_continuity: Option<ChapterContext>,
}

// AI Provider Configuration
#[derive(Debug, Clone)]
pub struct AIProviderConfig {
    pub name: String,
    pub provider_type: ProviderType,
    pub base_url: Option<String>,
    pub api_key: String,
    pub model_name: String,
    pub context_window: usize,
    pub max_tokens: usize,
    pub supports_streaming: bool,
    pub supports_images: bool,
    pub rate_limit_rpm: Option<u32>,
    pub rate_limit_tpm: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum ProviderType {
    OpenAI,
    OpenAICompatible,
    Gemini,
    Claude,
    Ollama,
}

// OpenAI-Compatible Provider Implementation
pub struct OpenAICompatibleProvider {
    config: AIProviderConfig,
    client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
}

impl OpenAICompatibleProvider {
    pub fn new(config: AIProviderConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");
        
        let rate_limiter = Arc::new(RateLimiter::new(
            config.rate_limit_rpm.unwrap_or(60),
            config.rate_limit_tpm.unwrap_or(10000),
        ));

        Self {
            config,
            client,
            rate_limiter,
        }
    }
}

#[async_trait]
impl AIProvider for OpenAICompatibleProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String> {
        self.rate_limiter.wait_if_needed().await;
        
        let messages = self.build_messages(prompt, context)?;
        let request_body = serde_json::json!({
            "model": self.config.model_name,
            "messages": messages,
            "max_tokens": self.config.max_tokens,
            "temperature": 0.7,
            "stream": false
        });

        let base_url = self.config.base_url.as_ref()
            .unwrap_or(&"https://api.openai.com".to_string());
        
        let response = self.client
            .post(&format!("{}/v1/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

        Ok(content.to_string())
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Implementation for embedding generation
        todo!("Implement embedding generation for OpenAI-compatible providers")
    }

    async fn generate_image(&self, prompt: &str) -> Result<Vec<u8>> {
        // Implementation for image generation if supported
        todo!("Implement image generation for compatible providers")
    }

    fn supports_streaming(&self) -> bool {
        self.config.supports_streaming
    }

    fn supports_images(&self) -> bool {
        self.config.supports_images
    }

    fn get_context_window(&self) -> usize {
        self.config.context_window
    }
}

// Gemini Provider Implementation
pub struct GeminiProvider {
    config: AIProviderConfig,
    client: reqwest::Client,
    rate_limiter: Arc<RateLimiter>,
}

#[async_trait]
impl AIProvider for GeminiProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String> {
        self.rate_limiter.wait_if_needed().await;
        
        let request_body = serde_json::json!({
            "contents": [{
                "parts": [{
                    "text": self.build_gemini_prompt(prompt, context)?
                }]
            }],
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": self.config.max_tokens,
                "topP": 0.8,
                "topK": 40
            },
            "safetySettings": [
                {
                    "category": "HARM_CATEGORY_HARASSMENT",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                },
                {
                    "category": "HARM_CATEGORY_HATE_SPEECH",
                    "threshold": "BLOCK_MEDIUM_AND_ABOVE"
                }
            ]
        });

        let response = self.client
            .post(&format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
                self.config.model_name, self.config.api_key
            ))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;
        let content = response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid Gemini response format"))?;

        Ok(content.to_string())
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let request_body = serde_json::json!({
            "model": "models/text-embedding-004",
            "content": {
                "parts": [{
                    "text": text
                }]
            }
        });

        let response = self.client
            .post(&format!(
                "https://generativelanguage.googleapis.com/v1beta/models/text-embedding-004:embedContent?key={}",
                self.config.api_key
            ))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_json: serde_json::Value = response.json().await?;
        let embedding: Vec<f32> = response_json["embedding"]["values"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid embedding response"))?
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();

        Ok(embedding)
    }

    // Additional Gemini-specific methods...
}

// AI Provider Manager
pub struct AIProviderManager {
    providers: HashMap<String, Box<dyn AIProvider + Send + Sync>>,
    default_provider: Option<String>,
}

impl AIProviderManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default_provider: None,
        }
    }

    pub fn add_provider(&mut self, name: String, provider: Box<dyn AIProvider + Send + Sync>) {
        self.providers.insert(name, provider);
    }

    pub fn set_default_provider(&mut self, name: String) {
        self.default_provider = Some(name);
    }

    pub async fn generate_text(&self, provider_name: Option<&str>, prompt: &str, context: &AIContext) -> Result<String> {
        let provider_name = provider_name
            .or(self.default_provider.as_deref())
            .ok_or_else(|| anyhow::anyhow!("No provider specified or default set"))?;

        let provider = self.providers.get(provider_name)
            .ok_or_else(|| anyhow::anyhow!("Provider not found: {}", provider_name))?;

        provider.generate_text(prompt, context).await
    }
}

// Rate Limiter for API calls
pub struct RateLimiter {
    requests_per_minute: u32,
    tokens_per_minute: u32,
    request_timestamps: Arc<Mutex<VecDeque<Instant>>>,
    token_usage: Arc<Mutex<VecDeque<(Instant, u32)>>>,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32, tokens_per_minute: u32) -> Self {
        Self {
            requests_per_minute,
            tokens_per_minute,
            request_timestamps: Arc::new(Mutex::new(VecDeque::new())),
            token_usage: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub async fn wait_if_needed(&self) {
        // Implementation for rate limiting logic
        let now = Instant::now();
        let minute_ago = now - Duration::from_secs(60);

        // Clean old entries and check limits
        let mut requests = self.request_timestamps.lock().await;
        while let Some(&front) = requests.front() {
            if front < minute_ago {
                requests.pop_front();
            } else {
                break;
            }
        }

        if requests.len() >= self.requests_per_minute as usize {
            let sleep_duration = requests.front().unwrap().duration_since(minute_ago);
            tokio::time::sleep(sleep_duration).await;
        }

        requests.push_back(now);
    }
}
```

### Vector Database Operations
```rust
// Semantic search implementation
pub async fn semantic_search(
    query: &str,
    project_id: i32,
    limit: usize,
) -> Result<Vec<SearchResult>> {
    let embedding = generate_embedding(query).await?;
    let results = lance_db
        .search(embedding)
        .filter(format!("project_id = {}", project_id))
        .limit(limit)
        .execute()
        .await?;
    Ok(results)
}
```

### Plugin System Architecture
```rust
// Plugin execution environment
pub struct PluginEngine {
    runtime: Runtime,
    ai_provider: Box<dyn AIProvider>,
}

impl PluginEngine {
    pub async fn execute_plugin(
        &self,
        plugin_code: &str,
        input: &str,
        context: &PluginContext,
    ) -> Result<String> {
        // Safe plugin execution with sandboxing
    }
}
```

## Security Considerations

- **API Key Management**: Secure storage of AI provider API keys
- **Plugin Sandboxing**: Safe execution of user-created plugins
- **Data Privacy**: All data stored locally, no cloud dependencies
- **Input Validation**: Sanitization of all user inputs
- **File System Access**: Controlled file operations through Tauri

## Performance Optimization

- **Lazy Loading**: Load documents and features on demand
- **Caching Strategy**: Cache AI responses and embeddings
- **Background Processing**: Async operations for AI calls
- **Database Indexing**: Optimize queries with proper indexes
- **Memory Management**: Efficient handling of large documents

## Testing Strategy

- **Unit Tests**: Core business logic and utilities
- **Integration Tests**: Database operations and AI integrations
- **E2E Tests**: Complete user workflows
- **Performance Tests**: Large document handling
- **Security Tests**: Plugin sandboxing and input validation

## Deployment & Distribution

- **Windows Installer**: MSI package for easy installation
- **Auto-Updates**: Built-in update mechanism
- **Portable Version**: Standalone executable option
- **Documentation**: Comprehensive user guide and API docs
- **Support System**: Error reporting and diagnostics

## Future Enhancements

- **Collaboration Features**: Real-time document sharing
- **Mobile Companion**: Read-only mobile app
- **Advanced Analytics**: Writing pattern analysis
- **Integration APIs**: Connect with other writing tools
- **Cloud Sync**: Optional cloud backup and sync
- **Voice Integration**: Speech-to-text capabilities
- **Advanced AI Models**: Integration with latest AI developments

## Estimated Timeline: 24 Weeks

This comprehensive plan provides a roadmap for building a feature-rich AI toolkit for novelists that rivals commercial solutions while maintaining the benefits of a local, privacy-focused desktop application. The modular architecture allows for iterative development and future enhancements based on user feedback.

---

## Recent Updates to Development Plan

After reviewing all reference files, the following features have been added to ensure comprehensive coverage:

### New Core Features Added:
- **Point of View (POV) & Tense Settings**: Global and per-chapter control with character-specific POV assignment
- **Series Support**: Share Characters, Worldbuilding, and Outlines across multiple projects with series timeline management
- **Chapter Continuity**: Link documents for seamless AI context across chapters (up to 25 documents, 20,000 words)
- **Braindump**: Free-form text area in Story Bible for core story vision that influences synopsis generation
- **Visibility Settings**: Granular control over AI access to specific Story Bible elements and individual traits
- **Saliency Engine**: Intelligent selection of relevant story information for AI with raw data alternatives
- **Multiple AI Models**: Support for Muse, Excellent, Basic prose modes plus experimental models (GPT-4.1, Claude 3.5, DeepSeek, Gemini, etc.)
- **Style Examples**: Train AI on user's writing style for personalized prose generation (up to 1,000 words)
- **Document Sharing**: Clean Copy system with private commenting, reader anonymity options, and unpublish functionality
- **Smart Import**: Novel import with auto-populated Story Bible (120K words), character import (60K words, 30 chars), CSV support
- **Selection Menu**: Context-sensitive tools that adapt based on selection length and content type
- **Quick Tools**: Quick Edit and Quick Chat with High Quality mode toggle and story-aware context
- **Related Words**: Smart thesaurus with contextual suggestions and expandable word cloud interface
- **Visualize**: Generate images from text descriptions with content filtering and credit cost transparency
- **Advanced Brainstorming**: Category-specific prompts with Keepers List, thumbs up/down voting, and refresh options
- **Plugin Testing**: Built-in environment with sample Story Bible data and variable testing
- **Credit Tracking**: Transparent usage monitoring with pre-generation cost estimates and balance display
- **Comment System**: In-document commenting for collaboration with author-reader privacy controls
- **Write Settings**: Comprehensive configuration including creativity levels, card count, length settings, and Key Details
- **Trait Customization**: Individual and default trait management for characters and worldbuilding elements
- **Story Bible Detection**: Visual highlighting of detected elements in text with underlined references
- **Multi-Stage Plugin Prompts**: Sequential prompt execution with intermediate result access
- **Canvas Keyboard Shortcuts**: Full shortcut support for visual planning with zoom, pan, and selection controls
- **Automatic Document Linking**: Smart linking from outline chapters with bidirectional relationships
- **Series Timeline**: Project sequencing for consistent story context across multiple books
- **Match My Style**: AI analysis of user's writing to generate personalized style prompts
- **Creativity Level 11**: Special ultra-creative mode that works differently from standard creativity settings
- **Hover Menu**: Context-sensitive menu that appears when selecting text for quick tool access
- **History Panel**: Comprehensive tracking of all AI generations with starring and organization features
- **Project Folders**: Hierarchical organization with drag-and-drop, nested folders, and preview functionality
- **Deleted Projects Recovery**: Trash system with restoration capabilities for projects and folders
- **Card System**: Collapsible, stackable AI response cards with prompt context display
- **Tone Shift**: Write continuation in specific tones (Ominous, Fantastical, Fast-Paced, Upbeat, Authoritative, Conflicted, Romantic, Sensual)
- **Kitbashing Support**: Tools for combining and merging different AI generations and drafts
- **Focus Mode**: Distraction-free writing environment
- **Purple Text Highlighting**: Visual indication of AI-generated content until user edits
- **Beats to Scenes Migration**: Automatic conversion system from deprecated Beats to new Scenes format
- **Extra Instructions**: Per-scene guidance for tone, pacing, POV, and style in Draft tool
- **Story Bible Boxes**: Organized text fields for different story elements with generation capabilities
- **Guardrails System**: Boundary setting for AI generations to maintain story consistency
- **Drivers Identification**: Recognition and tracking of story-moving elements
- **Three-Column Interface**: Left navigation, center editor, right history/cards layout
- **Toolbar Integration**: Centralized access to all AI tools and features
- **Document Preview**: Quick preview of recent projects in folders without opening
- **File Path Navigation**: Breadcrumb navigation for nested folder structures

### Enhanced Database Schema:
- Added POV and Tense settings to `story_bible` and `outlines` tables.
- Added series support with shared Story Bible data
- Document linking for chapter continuity
- Visibility controls for characters and worldbuilding traits
- Comment and collaboration tables
- AI model settings and credit usage tracking
- Enhanced plugin system with testing capabilities

### Extended Timeline:
- Increased from 20 to 24 weeks to accommodate additional features
- Reorganized phases to better reflect feature complexity
- Added dedicated phase for collaboration and plugin features

### Dependency Updates:
- Added `csv`, `zip`, `diffy`, `mammoth-rs`, and `html-to-docx-rs` to Rust dependencies for file operations and version control.
- Added `reactflow`, `jspdf`, and `html2canvas` to frontend dependencies for visual planning and PDF export.

The plan now comprehensively covers all features found in the reference documentation, ensuring StoryWeaver will be a complete AI writing toolkit for novelists.

## Dependencies Summary

### Rust Dependencies (Cargo.toml)
```toml
[dependencies]
tauri = { version = "2.0", features = ["api-all"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "json"] }
reqwest = { version = "0.11", features = ["json"] }
lancedb = "0.4"
candle-core = "0.3"
candle-nn = "0.3"
candle-transformers = "0.3"
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1.0"
anyhow = "1.0"
log = "0.4"
env_logger = "0.10"
csv = "1.1"
zip = "0.6"
diffy = "0.3"
mammoth-rs = "0.2"
html-to-docx-rs = "0.1"
```

### Frontend Dependencies (package.json)
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2.0.0",
    "@tanstack/react-query": "^5.0.0",
    "zustand": "^4.4.0",
    "@radix-ui/react-dialog": "^1.0.0",
    "@radix-ui/react-dropdown-menu": "^2.0.0",
    "@radix-ui/react-tabs": "^1.0.0",
    "@monaco-editor/react": "^4.6.0",
    "react-dnd": "^16.0.0",
    "react-dnd-html5-backend": "^16.0.0",
    "framer-motion": "^10.16.0",
    "tailwindcss": "^3.3.0",
    "lucide-react": "^0.290.0",
    "react-hook-form": "^7.47.0",
    "zod": "^3.22.0",
    "reactflow": "^11.10.1",
    "jspdf": "^2.5.1",
    "html2canvas": "^1.4.1"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.1.0",
    "typescript": "^5.2.0",
    "vite": "^4.5.0",
    "@tauri-apps/cli": "^2.0.0"
  }
}
```

This plan provides a comprehensive roadmap for building StoryWeaver as a professional-grade AI toolkit for novelists, incorporating all the features identified in the reference materials while leveraging modern technologies for optimal performance and user experience.
