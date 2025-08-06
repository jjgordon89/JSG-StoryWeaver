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
tauri-plugin-updater = "2.0"  # For auto-updates
tauri-plugin-global-shortcut = "2.0"  # For global keyboard shortcuts
tauri-plugin-store = "2.0"  # For persistent settings storage

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

# Additional Dependencies for Reference Features
serde_derive = "1.0"  # Derive macros for serialization
lazy_static = "1.4"   # Static initialization
parking_lot = "0.12"  # High-performance synchronization primitives
crossbeam = "0.8"     # Lock-free data structures
crossbeam-channel = "0.5"  # Multi-producer multi-consumer channels
bytes = "1.5"         # Byte buffer utilities
futures-util = "0.3" # Additional futures utilities
pin-project = "1.1"   # Safe pin projection
async-stream = "0.3"  # Async stream utilities
stream-cancel = "0.8" # Stream cancellation
tokio-stream = "0.1"  # Tokio stream utilities
tokio-tungstenite = "0.20"  # WebSocket support for future features
tungstenite = "0.20"  # WebSocket protocol
rustls = "0.21"       # TLS implementation
rustls-pemfile = "1.0"  # PEM file parsing
webpki-roots = "0.25" # Root certificates
native-tls = "0.2"    # Native TLS support
openssl = { version = "0.10", optional = true }  # OpenSSL bindings
sha2 = "0.10"         # SHA-2 hash functions
hmac = "0.12"         # HMAC implementation
rand = "0.8"          # Random number generation
rand_chacha = "0.3"   # ChaCha random number generator
getrandom = "0.2"     # OS random number interface
zeroize = "1.6"       # Secure memory clearing
secrecy = "0.8"       # Secret management
subtle = "2.5"        # Constant-time operations
ed25519-dalek = "2.0" # Ed25519 signatures
x25519-dalek = "2.0"  # X25519 key exchange
curve25519-dalek = "4.1"  # Curve25519 operations
blake3 = "1.5"        # BLAKE3 hash function
argon2id = "0.2"      # Argon2id password hashing
scrypt = "0.11"       # Scrypt password hashing
pbkdf2 = "0.12"       # PBKDF2 key derivation
hkdf = "0.12"         # HKDF key derivation
chacha20poly1305 = "0.10"  # ChaCha20-Poly1305 AEAD
aes = "0.8"           # AES block cipher
ctr = "0.9"           # CTR mode
ccm = "0.5"           # CCM mode
eax = "0.5"           # EAX mode
ocb3 = "0.1"          # OCB3 mode
siv = "0.4"           # SIV mode

# Real-time Features & WebRTC
webrtc = "0.7"        # WebRTC for real-time collaboration
datachannel = "0.1"   # Data channels for peer-to-peer communication
ice = "0.9"           # ICE protocol for NAT traversal
stun = "0.4"          # STUN protocol for network discovery

# Advanced Text Processing
natural = "0.5"       # Natural language processing
stemmer = "0.1"       # Text stemming algorithms
stopwords = "0.1"     # Stop words filtering
lingua = "1.4"        # Language detection
whatlang = "0.16"     # Language identification

# Machine Learning & AI
ort = "1.16"          # ONNX Runtime for local ML models
tch = "0.13"          # PyTorch bindings for Rust
candle-onnx = "0.3"   # ONNX support for Candle
linfa = "0.7"         # Machine learning toolkit
smartcore = "0.3"    # ML algorithms and data structures

# Advanced File Processing
calamine = "0.22"     # Excel file reading
xlsxwriter = "0.6"    # Excel file writing
epub-builder = "0.7"  # EPUB generation
mobi = "0.4"          # MOBI format support
pandoc = "0.8"        # Document conversion via Pandoc

# Performance & Monitoring
criterion = "0.5"     # Benchmarking framework
pprof = "0.12"        # CPU profiling
memory-stats = "1.1"  # Memory usage monitoring
sysinfo = "0.29"      # System information
psutil = "3.2"        # Process and system utilities

# Advanced Networking
hyper = { version = "0.14", features = ["full"] }  # HTTP client/server
tower = "0.4"         # Service abstraction layer
tower-http = "0.4"    # HTTP middleware
axum = "0.7"          # Web framework for potential local server

# Additional Dependencies for New Features
diffy = "0.3"         # Text diffing for version control and change tracking
html-to-docx-rs = "0.1"  # HTML to DOCX conversion for document export
serde_with = "3.4"    # Additional serde utilities for complex serialization
indexmap = "2.1"      # Ordered hash maps for maintaining insertion order
smallvec = "1.11"     # Stack-allocated vectors for performance
ahash = "0.8"         # Fast hash algorithm for HashMap performance
fnv = "1.0"           # FNV hash for small keys
bitflags = "2.4"      # Bit flag types for feature toggles
enum-iterator = "1.4" # Enum iteration for UI dropdowns
strum = { version = "0.25", features = ["derive"] }  # String enum conversions
derive_more = "0.99"  # Additional derive macros
tap = "1.0"           # Method chaining utilities
itertools = "0.12"    # Additional iterator methods
either = "1.9"        # Either type for error handling
thiserror-impl = "1.0"  # Implementation details for thiserror
anyhow-std = "1.0"    # Standard library integration for anyhow
log4rs = "1.2"        # Advanced logging configuration
tracing-appender = "0.2"  # Log file rotation
tracing-bunyan-formatter = "0.3"  # Structured logging format
metrics = "0.21"      # Application metrics collection
metrics-exporter-prometheus = "0.12"  # Prometheus metrics export
sysinfo = "0.29"      # System information for performance monitoring
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
    "@tanstack/react-virtual": "^3.0.0",  // Virtual scrolling for large lists
    "react-window": "^1.8.8",  // Alternative virtualization
    
    // Real-time Features
    "socket.io-client": "^4.7.4",  // WebSocket client
    "simple-peer": "^9.11.1",  // WebRTC peer connections
    "peerjs": "^1.5.0",  // Simplified WebRTC
    "y-webrtc": "^10.2.5",  // WebRTC provider for Yjs
    "y-indexeddb": "^9.0.12",  // IndexedDB persistence for Yjs
    
    // Advanced Text Processing
    "natural": "^6.5.0",  // Natural language processing
    "compromise": "^14.10.0",  // Text analysis and NLP
    "franc": "^6.1.0",  // Language detection
    "sentiment": "^5.0.2",  // Sentiment analysis
    "keyword-extractor": "^0.0.25",  // Keyword extraction
    
    // Advanced Editor Features
    "@codemirror/state": "^6.4.0",  // CodeMirror state management
    "@codemirror/view": "^6.23.0",  // CodeMirror view layer
    "@codemirror/lang-markdown": "^6.2.4",  // Markdown support
    "@codemirror/autocomplete": "^6.12.0",  // Autocompletion
    "@codemirror/search": "^6.5.5",  // Search functionality
    "prosemirror-commands": "^1.5.2",  // ProseMirror commands
    "prosemirror-keymap": "^1.2.2",  // Keyboard shortcuts
    "prosemirror-history": "^1.3.2",  // Undo/redo functionality
    "prosemirror-schema-basic": "^1.2.2",  // Basic schema
    "prosemirror-schema-list": "^1.3.0",  // List support
    "prosemirror-transform": "^1.8.0",  // Document transformations
    
    // Machine Learning & AI
    "@tensorflow/tfjs": "^4.15.0",  // TensorFlow.js for client-side ML
    "@tensorflow/tfjs-node": "^4.15.0",  // Node.js backend
    "ml-matrix": "^6.10.7",  // Matrix operations
    "ml-distance": "^4.0.1",  // Distance calculations
    "compromise-plugin": "^0.0.3",  // NLP plugin system
    
    // Advanced File Processing
    "xlsx": "^0.18.5",  // Excel file processing
    "csv-parser": "^3.0.0",  // CSV parsing
    "xml2js": "^0.6.2",  // XML parsing
    "epub-gen": "^0.1.0",  // EPUB generation
    "html-pdf": "^3.0.1",  // HTML to PDF conversion
    "puppeteer": "^21.6.1",  // Browser automation for PDF generation
    
    // Accessibility
    "@reach/skip-nav": "^0.18.0",  // Skip navigation
    "@reach/visually-hidden": "^0.18.0",  // Screen reader support
    "focus-trap-react": "^10.2.3",  // Focus management
    "react-aria": "^3.32.1",  // Accessibility primitives
    "react-aria-live": "^3.0.1",  // Live regions for screen readers
    
    // Internationalization (for future)
    "react-i18next": "^13.5.0",
    "i18next": "^23.7.16",
    "i18next-browser-languagedetector": "^7.2.0",  // Language detection
    "i18next-http-backend": "^2.4.2",  // HTTP backend for translations
    
    // Advanced State Management
    "valtio": "^1.12.1",  // Proxy-based state management
    "jotai": "^2.6.0",  // Atomic state management
    "recoil": "^0.7.7",  // Facebook's state management
    "redux-toolkit": "^2.0.1",  // Modern Redux
    
    // Testing & Quality Assurance
    "axe-core": "^4.8.3",  // Accessibility testing
    "jest-axe": "^8.0.0",  // Jest accessibility matcher
    "react-testing-library": "^8.0.1",  // Component testing utilities
    
    // Performance Optimization
    "workbox-webpack-plugin": "^7.0.0",  // Service worker generation
    "comlink": "^4.4.1",  // Web worker communication
    "web-worker": "^1.2.0",  // Web worker utilities
    "react-loadable": "^5.5.0",  // Code splitting
    
    // Advanced UI Components
    "react-grid-layout": "^1.4.4",  // Draggable grid layout
    "react-resizable": "^3.0.5",  // Resizable components
    "react-split-pane": "^0.1.92",  // Split pane layout
    "react-hotkeys": "^2.0.0",  // Keyboard shortcuts
    "react-contextmenu": "^2.14.0",  // Context menus
    "react-tooltip": "^5.25.2",  // Advanced tooltips
    
    // Data Visualization
    "recharts": "^2.8.0",  // Charts and graphs
    "victory": "^36.7.0",  // Data visualization
    "react-vis": "^1.12.1",  // Uber's visualization library
    
    // Utility Libraries
    "ramda": "^0.29.1",  // Functional programming utilities
    "immutable": "^4.3.4",  // Immutable data structures
    "validator": "^13.11.0",  // String validation
    "sanitize-html": "^2.11.0",  // HTML sanitization
    "dompurify": "^3.0.7",  // DOM sanitization
    "he": "^1.2.0",  // HTML entity encoding/decoding
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
    "webpack-bundle-analyzer": "^4.10.1",  // Bundle analysis
    "source-map-explorer": "^2.5.3",  // Source map analysis
    
    // Advanced Testing
    "msw": "^2.0.11",  // Mock Service Worker for API mocking
    "@testing-library/jest-dom": "^6.2.0",  // Jest DOM matchers
    "@testing-library/user-event": "^14.5.1",  // User interaction testing
    "jest-environment-jsdom": "^29.7.0",  // JSDOM test environment
    "cypress": "^13.6.2",  // E2E testing alternative
    "@cypress/react": "^8.0.0",  // Cypress React component testing
    
    // Code Quality
    "husky": "^8.0.3",  // Git hooks
    "lint-staged": "^15.2.0",  // Run linters on staged files
    "commitizen": "^4.3.0",  // Conventional commits
    "@commitlint/cli": "^18.4.4",  // Commit message linting
    "@commitlint/config-conventional": "^18.4.4",  // Conventional commit config
    
    // Build Optimization
    "vite-plugin-windicss": "^1.9.3",  // WindiCSS integration
    "vite-plugin-eslint": "^1.8.1",  // ESLint integration
    "vite-plugin-checker": "^0.6.2",  // TypeScript checker
    "rollup-plugin-analyzer": "^4.0.0",  // Bundle analyzer
    "vite-bundle-analyzer": "^0.7.0",  // Vite-specific bundle analysis
    
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

-- Quick Tools and Interface State
CREATE TABLE quick_tools_sessions (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    session_type TEXT NOT NULL, -- 'quick_edit', 'quick_chat'
    original_text TEXT,
    modified_text TEXT,
    user_input TEXT,
    high_quality_mode BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Brainstorm Sessions and Keepers List
CREATE TABLE brainstorm_sessions (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    category TEXT NOT NULL,
    seed_prompt TEXT,
    session_data JSON,
    keepers_list JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Card System for AI Responses
CREATE TABLE ai_response_cards (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    document_id INTEGER,
    feature_type TEXT NOT NULL,
    prompt_context TEXT,
    response_text TEXT,
    is_stacked BOOLEAN DEFAULT FALSE,
    stack_order INTEGER,
    is_starred BOOLEAN DEFAULT FALSE,
    is_collapsed BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Style Examples and Match My Style
CREATE TABLE style_examples (
    id INTEGER PRIMARY KEY,
    project_id INTEGER,
    user_id TEXT, -- For global style examples
    example_text TEXT NOT NULL,
    analysis_result TEXT,
    generated_style_prompt TEXT,
    word_count INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Deleted Items Recovery System
CREATE TABLE deleted_items (
    id INTEGER PRIMARY KEY,
    item_type TEXT NOT NULL, -- 'project', 'folder', 'document'
    item_id INTEGER NOT NULL,
    item_data JSON NOT NULL,
    parent_id INTEGER,
    deletion_reason TEXT,
    deleted_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    can_restore BOOLEAN DEFAULT TRUE
);

-- Canvas Data Storage
CREATE TABLE canvas_elements (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    element_type TEXT NOT NULL, -- 'card', 'text', 'outline', 'connection'
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    width REAL,
    height REAL,
    content TEXT,
    style_data JSON,
    connections JSON, -- For linked elements
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Hover Menu and Selection Context
CREATE TABLE selection_contexts (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    selection_text TEXT NOT NULL,
    selection_start INTEGER NOT NULL,
    selection_end INTEGER NOT NULL,
    context_type TEXT, -- 'single_word', 'paragraph', 'long_text'
    available_tools JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Related Words Cache
CREATE TABLE related_words_cache (
    id INTEGER PRIMARY KEY,
    word TEXT NOT NULL,
    context_hash TEXT NOT NULL,
    related_words JSON NOT NULL,
    word_cloud_data JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME
);

-- Visualize Image Generation
CREATE TABLE generated_images (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    document_id INTEGER,
    source_text TEXT NOT NULL,
    image_prompt TEXT NOT NULL,
    image_data BLOB,
    image_url TEXT,
    credits_used INTEGER DEFAULT 2500,
    resolution TEXT DEFAULT '1024x1024',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Streaming Generation State
CREATE TABLE streaming_sessions (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    feature_type TEXT NOT NULL,
    session_token TEXT UNIQUE NOT NULL,
    current_text TEXT,
    is_paused BOOLEAN DEFAULT FALSE,
    can_resume BOOLEAN DEFAULT TRUE,
    context_data JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Purple Text Highlighting Tracking
CREATE TABLE ai_generated_ranges (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    start_position INTEGER NOT NULL,
    end_position INTEGER NOT NULL,
    feature_type TEXT NOT NULL,
    is_edited BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Settings
CREATE TABLE settings (
    id INTEGER PRIMARY KEY,
    key TEXT UNIQUE NOT NULL,
    value TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Additional Database Tables for New Features

-- Scenes and Draft System
CREATE TABLE scenes (
    id INTEGER PRIMARY KEY,
    outline_id INTEGER NOT NULL,
    scene_number INTEGER NOT NULL,
    title TEXT,
    summary TEXT,
    extra_instructions TEXT,
    pov TEXT,
    tense TEXT,
    character_pov_ids JSON,
    word_count_estimate INTEGER,
    credit_estimate INTEGER,
    is_validated BOOLEAN DEFAULT FALSE,
    validation_issues JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outline_id) REFERENCES outlines(id)
);

-- Acts/Dividers in Outlines
CREATE TABLE outline_acts (
    id INTEGER PRIMARY KEY,
    outline_id INTEGER NOT NULL,
    act_type TEXT NOT NULL, -- 'Part', 'Book', 'Episode', 'Section'
    act_number INTEGER NOT NULL,
    title TEXT NOT NULL,
    position INTEGER NOT NULL, -- Position in outline
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outline_id) REFERENCES outlines(id)
);

-- Document Versions and History
CREATE TABLE document_versions (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    version_number INTEGER NOT NULL,
    content TEXT NOT NULL,
    word_count INTEGER DEFAULT 0,
    change_summary TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Backup System
CREATE TABLE backups (
    id INTEGER PRIMARY KEY,
    backup_type TEXT NOT NULL, -- 'auto', 'manual', 'export'
    project_id INTEGER,
    backup_path TEXT NOT NULL,
    backup_size INTEGER,
    includes_story_bible BOOLEAN DEFAULT TRUE,
    backup_metadata JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Import/Export Operations
CREATE TABLE import_export_operations (
    id INTEGER PRIMARY KEY,
    operation_type TEXT NOT NULL, -- 'import', 'export'
    project_id INTEGER,
    file_type TEXT NOT NULL, -- 'docx', 'txt', 'rtf', 'odt', 'csv', 'zip'
    file_path TEXT,
    operation_status TEXT DEFAULT 'pending', -- 'pending', 'processing', 'completed', 'failed'
    progress_percentage INTEGER DEFAULT 0,
    result_data JSON,
    error_message TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Smart Import Results
CREATE TABLE smart_import_results (
    id INTEGER PRIMARY KEY,
    import_operation_id INTEGER NOT NULL,
    extracted_characters JSON,
    extracted_worldbuilding JSON,
    extracted_outline JSON,
    story_bible_data JSON,
    word_count INTEGER,
    character_count INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (import_operation_id) REFERENCES import_export_operations(id)
);

-- Plugin Marketplace and Sharing
CREATE TABLE plugin_marketplace (
    id INTEGER PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    creator_name TEXT NOT NULL,
    visibility TEXT DEFAULT 'published', -- 'published', 'unlisted', 'private'
    download_count INTEGER DEFAULT 0,
    rating_average REAL DEFAULT 0.0,
    rating_count INTEGER DEFAULT 0,
    category TEXT,
    tags JSON,
    featured BOOLEAN DEFAULT FALSE,
    published_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id)
);

CREATE TABLE plugin_ratings (
    id INTEGER PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id)
);

-- Plugin Usage Analytics
CREATE TABLE plugin_usage (
    id INTEGER PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    execution_time_ms INTEGER,
    credits_used INTEGER,
    success BOOLEAN DEFAULT TRUE,
    error_message TEXT,
    used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Advanced AI Model Configurations
CREATE TABLE ai_model_configurations (
    id INTEGER PRIMARY KEY,
    provider_id INTEGER NOT NULL,
    model_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    context_window INTEGER NOT NULL,
    max_output_tokens INTEGER NOT NULL,
    supports_streaming BOOLEAN DEFAULT TRUE,
    supports_images BOOLEAN DEFAULT FALSE,
    supports_function_calling BOOLEAN DEFAULT FALSE,
    cost_per_input_token REAL,
    cost_per_output_token REAL,
    cost_per_image REAL,
    quality_tier TEXT DEFAULT 'standard', -- 'basic', 'standard', 'premium'
    specializations JSON, -- ['creative_writing', 'technical_writing', 'dialogue', 'description']
    content_filter_levels JSON,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (provider_id) REFERENCES ai_providers(id)
);

-- Prose Mode Definitions
CREATE TABLE prose_modes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    model_configuration_id INTEGER NOT NULL,
    creativity_level INTEGER DEFAULT 5,
    temperature REAL DEFAULT 0.7,
    top_p REAL DEFAULT 0.9,
    frequency_penalty REAL DEFAULT 0.0,
    presence_penalty REAL DEFAULT 0.0,
    special_instructions TEXT,
    is_experimental BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (model_configuration_id) REFERENCES ai_model_configurations(id)
);

-- User Preferences and Settings
CREATE TABLE user_preferences (
    id INTEGER PRIMARY KEY,
    preference_category TEXT NOT NULL,
    preference_key TEXT NOT NULL,
    preference_value TEXT,
    data_type TEXT DEFAULT 'string', -- 'string', 'integer', 'boolean', 'json'
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(preference_category, preference_key)
);

-- Keyboard Shortcuts and Hotkeys
CREATE TABLE keyboard_shortcuts (
    id INTEGER PRIMARY KEY,
    action_name TEXT NOT NULL,
    shortcut_combination TEXT NOT NULL,
    context TEXT DEFAULT 'global', -- 'global', 'editor', 'canvas', 'story_bible'
    is_custom BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Feature Usage Analytics
CREATE TABLE feature_usage_analytics (
    id INTEGER PRIMARY KEY,
    feature_name TEXT NOT NULL,
    project_id INTEGER,
    usage_count INTEGER DEFAULT 1,
    total_credits_used INTEGER DEFAULT 0,
    average_execution_time_ms INTEGER,
    last_used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    first_used_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Error Logs and Diagnostics
CREATE TABLE error_logs (
    id INTEGER PRIMARY KEY,
    error_type TEXT NOT NULL,
    error_message TEXT NOT NULL,
    stack_trace TEXT,
    context_data JSON,
    project_id INTEGER,
    document_id INTEGER,
    user_action TEXT,
    severity TEXT DEFAULT 'error', -- 'info', 'warning', 'error', 'critical'
    is_resolved BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Performance Metrics
CREATE TABLE performance_metrics (
    id INTEGER PRIMARY KEY,
    metric_name TEXT NOT NULL,
    metric_value REAL NOT NULL,
    metric_unit TEXT, -- 'ms', 'mb', 'count', 'percentage'
    context_data JSON,
    recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Notification System
CREATE TABLE notifications (
    id INTEGER PRIMARY KEY,
    notification_type TEXT NOT NULL, -- 'info', 'warning', 'error', 'success'
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    action_data JSON,
    is_read BOOLEAN DEFAULT FALSE,
    is_dismissed BOOLEAN DEFAULT FALSE,
    expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Template System
CREATE TABLE templates (
    id INTEGER PRIMARY KEY,
    template_type TEXT NOT NULL, -- 'character', 'worldbuilding', 'outline', 'document'
    name TEXT NOT NULL,
    description TEXT,
    template_data JSON NOT NULL,
    is_system_template BOOLEAN DEFAULT FALSE,
    is_public BOOLEAN DEFAULT FALSE,
    usage_count INTEGER DEFAULT 0,
    created_by TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Workflow Automation
CREATE TABLE workflow_automations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    trigger_type TEXT NOT NULL, -- 'document_save', 'word_count_milestone', 'time_based', 'manual'
    trigger_conditions JSON,
    actions JSON, -- Array of actions to perform
    is_active BOOLEAN DEFAULT TRUE,
    project_id INTEGER,
    last_executed_at DATETIME,
    execution_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Content Analysis Results
CREATE TABLE content_analysis (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    analysis_type TEXT NOT NULL, -- 'character_consistency', 'plot_holes', 'pacing', 'style'
    analysis_results JSON NOT NULL,
    suggestions JSON,
    confidence_score REAL,
    analyzed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

-- Collaboration Sessions
CREATE TABLE collaboration_sessions (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    session_token TEXT UNIQUE NOT NULL,
    session_name TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    allow_anonymous BOOLEAN DEFAULT TRUE,
    max_participants INTEGER DEFAULT 10,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME,
    FOREIGN KEY (document_id) REFERENCES documents(id)
);

CREATE TABLE collaboration_participants (
    id INTEGER PRIMARY KEY,
    session_id INTEGER NOT NULL,
    participant_name TEXT,
    participant_token TEXT UNIQUE NOT NULL,
    is_anonymous BOOLEAN DEFAULT TRUE,
    joined_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_active_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (session_id) REFERENCES collaboration_sessions(id)
);

-- Advanced Search Index
CREATE TABLE search_index (
    id INTEGER PRIMARY KEY,
    content_type TEXT NOT NULL, -- 'document', 'character', 'worldbuilding', 'outline'
    content_id INTEGER NOT NULL,
    searchable_text TEXT NOT NULL,
    keywords JSON,
    last_indexed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Full-Text Search Virtual Table
CREATE VIRTUAL TABLE search_fts USING fts5(
    content_type,
    content_id,
    title,
    content,
    keywords,
    content='search_index',
    content_rowid='id'
);

-- Triggers for FTS updates
CREATE TRIGGER search_index_ai AFTER INSERT ON search_index BEGIN
    INSERT INTO search_fts(rowid, content_type, content_id, title, content, keywords)
    VALUES (new.id, new.content_type, new.content_id, '', new.searchable_text, new.keywords);
END;

CREATE TRIGGER search_index_ad AFTER DELETE ON search_index BEGIN
    INSERT INTO search_fts(search_fts, rowid, content_type, content_id, title, content, keywords)
    VALUES ('delete', old.id, old.content_type, old.content_id, '', old.searchable_text, old.keywords);
END;

CREATE TRIGGER search_index_au AFTER UPDATE ON search_index BEGIN
    INSERT INTO search_fts(search_fts, rowid, content_type, content_id, title, content, keywords)
    VALUES ('delete', old.id, old.content_type, old.content_id, '', old.searchable_text, old.keywords);
    INSERT INTO search_fts(rowid, content_type, content_id, title, content, keywords)
    VALUES (new.id, new.content_type, new.content_id, '', new.searchable_text, new.keywords);
END;
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
- **Embedding Generation**: Implement the generation and storage of vector embeddings in LanceDB for all new and updated Story Bible elements (characters, worldbuilding, etc.) to support the Phase 4 Saliency Engine.

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

## Technical Workflow Details

### Document Processing Pipeline
```rust
// Document processing workflow
pub struct DocumentProcessor {
    content_analyzer: ContentAnalyzer,
    story_bible_detector: StoryBibleDetector,
    version_manager: VersionManager,
    backup_manager: BackupManager,
}

impl DocumentProcessor {
    pub async fn process_document_save(&self, document: &Document) -> Result<ProcessingResult> {
        // 1. Content analysis and validation
        let analysis = self.content_analyzer.analyze(&document.content).await?;
        
        // 2. Story Bible element detection
        let detected_elements = self.story_bible_detector.detect(&document.content).await?;
        
        // 3. Version management
        let version = self.version_manager.create_version(document).await?;
        
        // 4. Automatic backup if needed
        if self.should_backup(document) {
            self.backup_manager.create_backup(document.project_id).await?;
        }
        
        // 5. Update search index
        self.update_search_index(document).await?;
        
        Ok(ProcessingResult {
            analysis,
            detected_elements,
            version,
        })
    }
}

// Smart Import Processing
pub struct SmartImportProcessor {
    character_extractor: CharacterExtractor,
    worldbuilding_extractor: WorldbuildingExtractor,
    outline_generator: OutlineGenerator,
    story_bible_populator: StoryBiblePopulator,
}

impl SmartImportProcessor {
    pub async fn process_novel_import(&self, content: &str, project_id: i32) -> Result<ImportResult> {
        // Validate content length (max 120,000 words)
        if self.word_count(content) > 120_000 {
            return Err(ImportError::ContentTooLarge);
        }
        
        // Extract characters (max 30 characters)
        let characters = self.character_extractor
            .extract_characters(content, 30)
            .await?;
        
        // Extract worldbuilding elements
        let worldbuilding = self.worldbuilding_extractor
            .extract_worldbuilding(content)
            .await?;
        
        // Generate outline from content
        let outline = self.outline_generator
            .generate_outline(content)
            .await?;
        
        // Populate Story Bible
        let story_bible = self.story_bible_populator
            .populate_from_content(content, &characters, &worldbuilding)
            .await?;
        
        Ok(ImportResult {
            characters,
            worldbuilding,
            outline,
            story_bible,
        })
    }
    
    pub async fn process_character_import(&self, content: &str, max_chars: usize) -> Result<Vec<Character>> {
        // Validate content length (max 60,000 words)
        if self.word_count(content) > 60_000 {
            return Err(ImportError::ContentTooLarge);
        }
        
        // Extract characters with limit
        let characters = self.character_extractor
            .extract_characters(content, max_chars.min(30))
            .await?;
        
        Ok(characters)
    }
    
    pub async fn process_csv_import(&self, csv_content: &str) -> Result<Vec<Character>> {
        // Parse CSV and create character objects
        let mut reader = csv::Reader::from_reader(csv_content.as_bytes());
        let mut characters = Vec::new();
        
        for result in reader.records() {
            let record = result?;
            let character = self.parse_character_from_csv_record(&record)?;
            characters.push(character);
        }
        
        Ok(characters)
    }
}

// Quick Tools Processing
pub struct QuickToolsProcessor {
    ai_provider: Arc<dyn AIProvider>,
    context_builder: ContextBuilder,
}

impl QuickToolsProcessor {
    pub async fn process_quick_edit(&self, 
        document_id: i32, 
        selected_text: &str, 
        user_input: &str,
        high_quality: bool
    ) -> Result<QuickEditResult> {
        // Build context from document and Story Bible
        let context = self.context_builder
            .build_quick_tools_context(document_id)
            .await?;
        
        // Determine model based on quality setting
        let model = if high_quality { "gpt-4o" } else { "gpt-4o-mini" };
        
        // Create prompt for quick edit
        let prompt = format!(
            "Edit the following text based on the user's instruction: '{}'\n\nText to edit: {}\n\nContext: {}",
            user_input, selected_text, context.summary
        );
        
        // Generate edit
        let edited_text = self.ai_provider
            .generate_text(&prompt, &context.ai_context)
            .await?;
        
        // Save session for undo/redo
        self.save_quick_tools_session(document_id, "quick_edit", selected_text, &edited_text, user_input, high_quality).await?;
        
        Ok(QuickEditResult {
            original_text: selected_text.to_string(),
            edited_text,
            credits_used: if high_quality { 10 } else { 0 },
        })
    }
    
    pub async fn process_quick_chat(&self,
        document_id: i32,
        user_message: &str,
        high_quality: bool
    ) -> Result<String> {
        // Build context
        let context = self.context_builder
            .build_quick_tools_context(document_id)
            .await?;
        
        // Create chat prompt
        let prompt = format!(
            "User question: {}\n\nDocument context: {}\n\nStory Bible context: {}",
            user_message, context.document_excerpt, context.story_bible_summary
        );
        
        // Generate response
        let response = self.ai_provider
            .generate_text(&prompt, &context.ai_context)
            .await?;
        
        Ok(response)
    }
}

// Brainstorm Session Management
pub struct BrainstormManager {
    ai_provider: Arc<dyn AIProvider>,
    session_store: Arc<dyn SessionStore>,
}

impl BrainstormManager {
    pub async fn start_brainstorm_session(&self,
        project_id: i32,
        category: &str,
        seed_prompt: Option<&str>
    ) -> Result<BrainstormSession> {
        let session_id = Uuid::new_v4().to_string();
        
        // Generate initial ideas
        let ideas = self.generate_category_ideas(project_id, category, seed_prompt).await?;
        
        let session = BrainstormSession {
            id: session_id,
            project_id,
            category: category.to_string(),
            seed_prompt: seed_prompt.map(String::from),
            ideas,
            keepers_list: Vec::new(),
            created_at: Utc::now(),
        };
        
        self.session_store.save_session(&session).await?;
        Ok(session)
    }
    
    pub async fn vote_on_idea(&self,
        session_id: &str,
        idea_id: &str,
        vote: Vote
    ) -> Result<()> {
        let mut session = self.session_store.get_session(session_id).await?;
        
        if let Some(idea) = session.ideas.iter_mut().find(|i| i.id == idea_id) {
            match vote {
                Vote::ThumbsUp => {
                    idea.thumbs_up += 1;
                    if !session.keepers_list.iter().any(|k| k.id == idea_id) {
                        session.keepers_list.push(idea.clone());
                    }
                },
                Vote::ThumbsDown => {
                    idea.thumbs_down += 1;
                    session.keepers_list.retain(|k| k.id != idea_id);
                }
            }
        }
        
        self.session_store.save_session(&session).await?;
        Ok(())
    }
    
    pub async fn refresh_ideas(&self, session_id: &str) -> Result<Vec<BrainstormIdea>> {
        let session = self.session_store.get_session(session_id).await?;
        
        // Generate new ideas while preserving keepers
        let new_ideas = self.generate_category_ideas(
            session.project_id, 
            &session.category, 
            session.seed_prompt.as_deref()
        ).await?;
        
        // Update session with new ideas
        let mut updated_session = session;
        updated_session.ideas = new_ideas;
        self.session_store.save_session(&updated_session).await?;
        
        Ok(updated_session.ideas)
    }
}

// Plugin Execution Engine
pub struct PluginExecutionEngine {
    wasm_runtime: Arc<WasmRuntime>,
    ai_provider_manager: Arc<AIProviderManager>,
    variable_injector: VariableInjector,
}

impl PluginExecutionEngine {
    pub async fn execute_plugin(&self,
        plugin: &Plugin,
        context: &PluginExecutionContext
    ) -> Result<PluginExecutionResult> {
        // Validate plugin safety
        self.validate_plugin_safety(plugin)?;
        
        // Inject variables into plugin template
        let processed_prompts = self.variable_injector
            .inject_variables(&plugin.prompt_template, context)
            .await?;
        
        let mut results = Vec::new();
        
        // Execute multi-stage prompts
        for (stage, prompt) in processed_prompts.iter().enumerate() {
            let ai_context = self.build_ai_context_for_plugin(context, stage).await?;
            
            let result = self.ai_provider_manager
                .generate_text(
                    Some(&plugin.ai_model), 
                    prompt, 
                    &ai_context
                ).await?;
            
            results.push(PluginStageResult {
                stage,
                prompt: prompt.clone(),
                result: result.clone(),
            });
            
            // Update context with intermediate result for next stage
            if stage < processed_prompts.len() - 1 {
                context.intermediate_results.push(result);
            }
        }
        
        // Track usage analytics
        self.track_plugin_usage(plugin, context, &results).await?;
        
        Ok(PluginExecutionResult {
            plugin_id: plugin.id,
            execution_id: Uuid::new_v4().to_string(),
            stage_results: results,
            credits_used: self.calculate_credits_used(&results),
            execution_time_ms: context.start_time.elapsed().as_millis() as u32,
        })
    }
    
    pub async fn test_plugin(&self,
        plugin: &Plugin,
        test_data: &PluginTestData
    ) -> Result<PluginTestResult> {
        // Create test execution context
        let test_context = PluginExecutionContext {
            project_id: test_data.project_id,
            document_id: test_data.document_id,
            highlighted_text: test_data.highlighted_text.clone(),
            user_input: test_data.user_input.clone(),
            story_bible_data: test_data.story_bible_data.clone(),
            intermediate_results: Vec::new(),
            start_time: Instant::now(),
        };
        
        // Execute plugin in test mode
        let result = self.execute_plugin(plugin, &test_context).await?;
        
        Ok(PluginTestResult {
            success: true,
            execution_result: result,
            test_warnings: self.analyze_test_results(&result),
        })
    }
}

// Canvas Processing System
pub struct CanvasProcessor {
    element_manager: ElementManager,
    layout_engine: LayoutEngine,
    export_manager: ExportManager,
}

impl CanvasProcessor {
    pub async fn create_outline_from_canvas(&self,
        project_id: i32,
        canvas_elements: &[CanvasElement]
    ) -> Result<Outline> {
        // Extract outline structure from canvas elements
        let outline_cards = canvas_elements
            .iter()
            .filter(|e| e.element_type == "outline_card")
            .collect::<Vec<_>>();
        
        // Sort by position and connections
        let sorted_cards = self.layout_engine
            .sort_elements_by_flow(&outline_cards)?;
        
        // Convert to outline chapters
        let mut chapters = Vec::new();
        for (index, card) in sorted_cards.iter().enumerate() {
            let chapter = OutlineChapter {
                chapter_number: index + 1,
                title: card.content.get("title").unwrap_or(&"Untitled".to_string()).clone(),
                summary: card.content.get("summary").unwrap_or(&"".to_string()).clone(),
                pov: card.content.get("pov").cloned(),
                tense: card.content.get("tense").cloned(),
                character_pov_ids: card.content.get("character_pov_ids")
                    .and_then(|v| serde_json::from_str(v).ok())
                    .unwrap_or_default(),
            };
            chapters.push(chapter);
        }
        
        Ok(Outline {
            project_id,
            chapters,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    pub async fn generate_visual_outline(&self,
        project_id: i32,
        outline_type: OutlineType,
        seed_text: &str
    ) -> Result<Vec<CanvasElement>> {
        // Generate outline structure based on type
        let template = self.get_outline_template(outline_type);
        let generated_outline = self.generate_outline_from_seed(seed_text, &template).await?;
        
        // Convert to canvas elements with appropriate positioning
        let mut elements = Vec::new();
        let positions = self.layout_engine.calculate_positions(&generated_outline, outline_type);
        
        for (chapter, position) in generated_outline.chapters.iter().zip(positions.iter()) {
            let element = CanvasElement {
                id: Uuid::new_v4().to_string(),
                project_id,
                element_type: "outline_card".to_string(),
                position_x: position.x,
                position_y: position.y,
                width: 200.0,
                height: 150.0,
                content: serde_json::json!({
                    "title": chapter.title,
                    "summary": chapter.summary,
                    "chapter_number": chapter.chapter_number
                }),
                style_data: self.get_default_card_style(outline_type),
                connections: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            elements.push(element);
        }
        
        Ok(elements)
    }
}

// Collaboration System
pub struct CollaborationManager {
    session_manager: SessionManager,
    comment_processor: CommentProcessor,
    notification_service: NotificationService,
}

impl CollaborationManager {
    pub async fn create_share_link(&self,
        document_id: i32,
        settings: ShareSettings
    ) -> Result<ShareLink> {
        let share_token = self.generate_secure_token();
        
        let shared_document = SharedDocument {
            document_id,
            share_token: share_token.clone(),
            is_active: true,
            allow_comments: settings.allow_comments,
            allow_anonymous: settings.allow_anonymous,
            max_participants: settings.max_participants,
            expires_at: settings.expires_at,
            created_at: Utc::now(),
        };
        
        self.session_manager.create_shared_document(shared_document).await?;
        
        Ok(ShareLink {
            token: share_token,
            url: format!("https://storyweaver.app/shared/{}", share_token),
            settings,
        })
    }
    
    pub async fn add_comment(&self,
        document_id: i32,
        comment: CommentRequest
    ) -> Result<Comment> {
        // Validate comment permissions
        self.validate_comment_permissions(document_id, &comment.user_token).await?;
        
        let new_comment = Comment {
            id: Uuid::new_v4().to_string(),
            document_id,
            user_name: comment.user_name,
            comment_text: comment.text,
            start_position: comment.start_position,
            end_position: comment.end_position,
            is_author_comment: comment.is_author,
            created_at: Utc::now(),
        };
        
        // Save comment
        self.comment_processor.save_comment(&new_comment).await?;
        
        // Notify document owner if not author comment
        if !comment.is_author {
            self.notification_service
                .notify_new_comment(document_id, &new_comment)
                .await?;
        }
        
        Ok(new_comment)
    }
}

// Credit Management System
pub struct CreditManager {
    usage_tracker: UsageTracker,
    cost_calculator: CostCalculator,
    balance_manager: BalanceManager,
}

impl CreditManager {
    pub async fn estimate_cost(&self,
        feature: &str,
        model: &str,
        input_tokens: usize,
        estimated_output_tokens: usize
    ) -> Result<CreditEstimate> {
        let model_config = self.cost_calculator.get_model_config(model).await?;
        
        let input_cost = (input_tokens as f64 * model_config.cost_per_input_token) as i32;
        let output_cost = (estimated_output_tokens as f64 * model_config.cost_per_output_token) as i32;
        
        Ok(CreditEstimate {
            feature: feature.to_string(),
            model: model.to_string(),
            input_tokens,
            estimated_output_tokens,
            estimated_credits: input_cost + output_cost,
            breakdown: CostBreakdown {
                input_cost,
                output_cost,
                base_feature_cost: self.get_base_feature_cost(feature),
            },
        })
    }
    
    pub async fn consume_credits(&self,
        project_id: i32,
        feature: &str,
        model: &str,
        actual_tokens_used: TokenUsage
    ) -> Result<CreditTransaction> {
        let cost = self.calculate_actual_cost(model, &actual_tokens_used).await?;
        
        // Check balance
        let current_balance = self.balance_manager.get_balance().await?;
        if current_balance < cost {
            return Err(CreditError::InsufficientBalance);
        }
        
        // Deduct credits
        let transaction = CreditTransaction {
            id: Uuid::new_v4().to_string(),
            project_id,
            feature: feature.to_string(),
            model: model.to_string(),
            credits_used: cost,
            tokens_used: actual_tokens_used,
            timestamp: Utc::now(),
        };
        
        self.balance_manager.deduct_credits(cost).await?;
        self.usage_tracker.record_usage(&transaction).await?;
        
        Ok(transaction)
    }
}

// Streaming Generation Manager
pub struct StreamingManager {
    active_streams: Arc<DashMap<String, StreamingSession>>,
    ai_provider: Arc<dyn AIProvider>,
}

impl StreamingManager {
    pub async fn start_streaming_generation(&self,
        document_id: i32,
        feature_type: &str,
        prompt: &str,
        context: &AIContext
    ) -> Result<StreamingSession> {
        let session_token = Uuid::new_v4().to_string();
        
        let session = StreamingSession {
            token: session_token.clone(),
            document_id,
            feature_type: feature_type.to_string(),
            current_text: String::new(),
            is_paused: false,
            can_resume: true,
            context_data: serde_json::to_value(context)?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.active_streams.insert(session_token.clone(), session.clone());
        
        // Start streaming in background
        let provider = Arc::clone(&self.ai_provider);
        let streams = Arc::clone(&self.active_streams);
        let token = session_token.clone();
        let prompt = prompt.to_string();
        let context = context.clone();
        
        tokio::spawn(async move {
            if let Ok(mut stream) = provider.generate_text_stream(&prompt, &context).await {
                while let Some(chunk) = stream.next().await {
                    if let Ok(text) = chunk {
                        if let Some(mut session) = streams.get_mut(&token) {
                            if session.is_paused {
                                break;
                            }
                            session.current_text.push_str(&text);
                            session.updated_at = Utc::now();
                        }
                    }
                }
            }
        });
        
        Ok(session)
    }
    
    pub async fn pause_stream(&self, session_token: &str) -> Result<()> {
        if let Some(mut session) = self.active_streams.get_mut(session_token) {
            session.is_paused = true;
            session.updated_at = Utc::now();
        }
        Ok(())
    }
    
    pub async fn resume_stream(&self, session_token: &str) -> Result<()> {
        if let Some(mut session) = self.active_streams.get_mut(session_token) {
            if session.can_resume {
                session.is_paused = false;
                session.updated_at = Utc::now();
                // Resume streaming logic here
            }
        }
        Ok(())
    }
}
```

### Advanced Feature Workflows

#### Style Examples Processing
```rust
pub struct StyleAnalyzer {
    ai_provider: Arc<dyn AIProvider>,
    style_cache: Arc<LruCache<String, StyleAnalysis>>,
}

impl StyleAnalyzer {
    pub async fn analyze_user_style(&self, text: &str) -> Result<StyleAnalysis> {
        // Validate text length (max 1,000 words)
        if self.word_count(text) > 1000 {
            return Err(StyleError::TextTooLong);
        }
        
        let analysis_prompt = format!(
            "Analyze the writing style of this text and create a detailed style profile:\n\n{}",
            text
        );
        
        let analysis = self.ai_provider
            .generate_text(&analysis_prompt, &AIContext::default())
            .await?;
        
        // Generate style prompt for future use
        let style_prompt = self.generate_style_prompt(&analysis).await?;
        
        Ok(StyleAnalysis {
            original_text: text.to_string(),
            analysis_result: analysis,
            generated_style_prompt: style_prompt,
            word_count: self.word_count(text),
            created_at: Utc::now(),
        })
    }
}
```

#### Related Words Processing
```rust
pub struct RelatedWordsProcessor {
    ai_provider: Arc<dyn AIProvider>,
    word_cache: Arc<LruCache<String, RelatedWordsResult>>,
}

impl RelatedWordsProcessor {
    pub async fn get_related_words(&self, 
        word: &str, 
        context: &str
    ) -> Result<RelatedWordsResult> {
        let context_hash = self.hash_context(context);
        let cache_key = format!("{}:{}", word, context_hash);
        
        // Check cache first
        if let Some(cached) = self.word_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        
        let prompt = format!(
            "Provide contextually appropriate alternatives for the word '{}' in this context:\n\n{}\n\nReturn synonyms, related words, and variations that would fit naturally.",
            word, context
        );
        
        let response = self.ai_provider
            .generate_text(&prompt, &AIContext::default())
            .await?;
        
        let related_words = self.parse_related_words_response(&response)?;
        let word_cloud_data = self.generate_word_cloud_data(&related_words);
        
        let result = RelatedWordsResult {
            original_word: word.to_string(),
            context_hash,
            related_words,
            word_cloud_data,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(24),
        };
        
        // Cache result
        self.word_cache.put(cache_key, result.clone());
        
        Ok(result)
    }
}
```

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

### Critical Missing Features Identified from Reference Review:

#### Interface & User Experience
- **Three-Column Layout**: Left project navigation, center editor, right history/cards with responsive design and toolbar spanning all columns
- **Hover Menu System**: Context-aware tool access when selecting text with adaptive options based on selection length and content type
- **Selection Menu**: Smart menu that appears when highlighting text, changing options based on what's selected (single word vs paragraph vs longer text)
- **Card Stacking Interface**: Visual organization of AI responses with collapsible cards, prompt context display, and feature attribution
- **Project Card System**: Visual project representation on homepage with preview capabilities, recent document access, and drag-and-drop organization
- **File Path Navigation**: Breadcrumb system for nested folder navigation with drag-and-drop support and hover previews
- **Purple Text Highlighting**: Visual indication of AI-generated content until user edits, with automatic removal after modification
- **Focus Mode**: Distraction-free writing environment with minimal UI elements
- **Document Preview**: Quick preview of recent projects in folders without opening full project

#### Quick Tools System
- **Quick Edit & Quick Chat**: Instant access via Ctrl+K (PC) / ⌘+K (Mac) with inline editing capabilities
- **High Quality Mode**: Toggle for Quick Tools that uses credits for more complex tasks vs free default mode
- **Story-Aware Context**: Quick Tools have full access to Story Bible and document context without briefing
- **Inline Editing**: Quick Edit shows struck-through original text with green replacement text for comparison
- **Tab Toggle**: Switch between Quick Edit and Quick Chat with Tab key within same interface
- **Free Quick Tools**: Default free usage without credit consumption for basic operations

#### Advanced AI Features
- **Muse-Specific Advanced Features**: 
  - Creativity Level 11 (special ultra-creative mode with different algorithms from standard 1-10 scale)
  - Up to 10,000 words generation in Draft tool (significantly higher than other models)
  - 128,000 words context reading capability with pause/resume functionality
  - Cliché detection and removal system during training for unique prose generation
  - Unfiltered content generation without safety restrictions for mature themes
  - Style Examples integration (up to 1,000 words of user writing samples for personalized AI training)
- **Match My Style**: AI analysis of user's writing to generate personalized style prompts automatically
- **Tone Shift**: Write continuation in specific tones (Ominous, Fantastical, Fast-Paced, Upbeat, Authoritative, Conflicted, Romantic, Sensual)
- **Related Words**: Smart thesaurus with contextual suggestions and expandable word cloud interface for single-word selections
- **Streaming Generation**: Real-time text generation with pause/resume capabilities, progress indicators, and cancellation options

#### Brainstorm System
- **Category-Specific Brainstorming**: Predefined categories (Dialogue, Characters, World building, Plot points, Names, Places, Objects, Descriptions, Article ideas, Tweets, Something else)
- **Thumbs Up/Down System**: Voting system with Keepers List functionality for saving preferred suggestions
- **Save & Exit Workflow**: Session management with persistent Keepers List across brainstorm sessions
- **Refresh Functionality**: Generate new suggestions while maintaining existing Keepers List
- **History Integration**: Brainstorm results automatically saved to History panel as organized cards

#### Community & Learning
- **Community Integration**: Discord community features, learning resources integration, and user-generated content sharing
- **Prose Quality Guidelines**: Built-in system to avoid vagueness, conflicting information, and confusing wording with real-time suggestions
- **Laser Tools**: Community term for targeted AI features like Rewrite, Describe, Expand with precision editing capabilities
- **Kitbashing Workflow**: Advanced feature for combining multiple AI generations and drafts with merge capabilities and version tracking
- **Muse-Specific Advanced Features**: 
  - Creativity Level 11 (special ultra-creative mode with different algorithms)
  - Up to 10,000 words generation in Draft tool
  - 128,000 words context reading capability with pause/resume
  - Cliché detection and removal system during training
  - Unfiltered content generation without safety restrictions
  - Style Examples integration (up to 1,000 words of user writing samples)
- **Advanced Plugin Variables**: Extended variable system including `previous_document_text`, `chapter_scenes_extra_instructions`, `is_story_bible_active`, etc.
- **Hover Menu System**: Context-aware tool access when selecting text with adaptive options
- **Card Stacking Interface**: Visual organization of AI responses with collapsible cards and prompt context
- **Project Card System**: Visual project representation on homepage with preview capabilities and recent document access
- **Deleted Projects Management**: Comprehensive trash and recovery system with folder restoration
- **File Path Navigation**: Breadcrumb system for nested folder navigation with drag-and-drop support
- **Document Type Recognition**: Smart handling of different document types and purposes
- **Token Management**: Behind-the-scenes token counting and optimization for all AI models
- **Model-Specific Optimizations**: Tailored configurations for different AI models with performance tuning
- **Content Filter Levels**: Adjustable content filtering based on selected AI model capabilities
- **Streaming Generation**: Real-time text generation with pause/resume capabilities and progress indicators
- **Context Window Management**: Intelligent handling of large context windows across different models
- **Rate Limiting**: Built-in rate limiting for API calls and credit management with queue system
- **Error Recovery**: Graceful handling of API failures and network issues with retry mechanisms
- **Performance Monitoring**: Built-in performance tracking and optimization with usage analytics
- **Accessibility Features**: Screen reader support and comprehensive keyboard navigation
- **Internationalization Support**: Framework for future multi-language support with locale management
- **High Quality Mode**: Toggle for Quick Tools that uses credits for more complex tasks
- **Free Quick Tools**: Default free usage of Quick Edit and Quick Chat without credit consumption
- **Story-Aware Context**: Quick Tools have full access to Story Bible and document context
- **Inline Editing**: Quick Edit shows struck-through original text with green replacement text
- **Tab Toggle**: Switch between Quick Edit and Quick Chat with Tab key
- **Keyboard Shortcuts**: Ctrl+K (PC) / ⌘+K (Mac) for instant Quick Tools access
- **Thumbs Up/Down System**: Brainstorm feature voting system with Keepers List functionality
- **Save & Exit Workflow**: Brainstorm session management with persistent Keepers List
- **Category-Specific Brainstorming**: Predefined categories (Dialogue, Characters, World building, Plot points, Names, Places, Objects, Descriptions, Article ideas, Tweets, Something else)
- **Refresh Functionality**: Generate new brainstorm suggestions while maintaining Keepers List
- **History Integration**: Brainstorm results saved to History panel as cards
- **Three-Column Layout**: Left project navigation, center editor, right history/cards with responsive design
- **Toolbar Positioning**: Top toolbar spanning all three columns with feature access
- **Card Interaction**: Click to expand/collapse card stacks with prompt context display
- **Prompt Visibility**: Italicized prompt text showing AI consideration context
- **Feature Attribution**: Clear labeling of which feature generated each card
- **Document Organization**: Left sidebar with Add New, Import, and folder creation options
- **Homepage Integration**: Project cards on main homepage with direct project access

### Additional Features Identified from Reference Review:
- **Laser Tools**: Community term for targeted AI features like Rewrite, Describe, Expand
- **Kitbashing Workflow**: Advanced feature for combining multiple AI generations and drafts
- **Prose Quality Guidelines**: Built-in system to avoid vagueness, conflicting information, and confusing wording
- **Community Integration**: Discord community features and learning resources
- **Muse-Specific Features**: 
  - Creativity Level 11 (special ultra-creative mode)
  - Up to 10,000 words generation in Draft
  - 128,000 words context reading capability
  - Cliché detection and removal system
  - Unfiltered content generation
- **Advanced Plugin Variables**: Extended variable system including `previous_document_text`, `chapter_scenes_extra_instructions`, etc.
- **Hover Menu System**: Context-aware tool access when selecting text
- **Card Stacking**: Visual organization of AI responses with collapsible cards
- **Project Card System**: Visual project representation on homepage with preview capabilities
- **Deleted Projects Management**: Comprehensive trash and recovery system
- **File Path Navigation**: Breadcrumb system for nested folder navigation
- **Document Type Recognition**: Smart handling of different document types and purposes
- **Token Management**: Behind-the-scenes token counting and optimization
- **Model-Specific Optimizations**: Tailored configurations for different AI models
- **Content Filter Levels**: Adjustable content filtering based on selected AI model
- **Streaming Generation**: Real-time text generation with pause/resume capabilities
- **Context Window Management**: Intelligent handling of large context windows across models
- **Rate Limiting**: Built-in rate limiting for API calls and credit management
- **Error Recovery**: Graceful handling of API failures and network issues
- **Performance Monitoring**: Built-in performance tracking and optimization
- **Accessibility Features**: Screen reader support and keyboard navigation
- **Internationalization Support**: Framework for future multi-language support

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
