# StoryWeaver AI Toolkit - Comprehensive Development Plan

## Project Overview

StoryWeaver is a comprehensive AI-powered desktop application for novelists that provides planning, writing, editing, and organizational tools. This single-user Windows desktop application will be built using Tauri with Rust backend, modern web frontend, SQLite for local data storage, and LanceDB for AI vectorization capabilities.

## Technology Stack

### Core Framework
- **Tauri 2.0** - Cross-platform desktop app framework with Rust backend
- **Rust** - Backend logic, database operations, AI integrations
- **TypeScript/React** - Frontend user interface
- **Vite** - Frontend build tool and development server

### Database & Storage
- **SQLite** - Primary database for structured data (projects, documents, characters, etc.)
- **LanceDB** - Vector database for AI embeddings and semantic search
- **Local File System** - Document storage and exports

### AI Integration
- **OpenAI API** - GPT models for text generation and editing
- **Anthropic Claude API** - Alternative AI provider
- **Local AI Models** (Optional) - Ollama integration for offline capabilities
- **Embedding Models** - For semantic search and content analysis

### Additional Dependencies
- **Serde** - Rust serialization/deserialization
- **Tokio** - Async runtime for Rust
- **SQLx** - Rust SQL toolkit
- **Reqwest** - HTTP client for API calls
- **Tauri Plugin Ecosystem** - File system, dialog, notification plugins
- **React Query/TanStack Query** - State management and caching
- **Zustand** - Lightweight state management
- **Tailwind CSS** - Styling framework
- **Radix UI** - Accessible component library
- **Monaco Editor** - Rich text editor component
- **React DnD** - Drag and drop functionality
- **Framer Motion** - Animations and transitions

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
- **Guided Write**: Directed writing with user prompts and AI-generated story ideas
- **Tone Shift**: Style-specific writing variations and changes
- **First Draft**: Complete scene generation from prompts (up to 3000 words, only available in empty documents)

#### Editing Tools
- **Rewrite**: Multiple rewriting styles (Rephrase, Shorter, More descriptive, Show-Not-Tell, More Inner Conflict, More Intense) with custom options (max 6,000 words)
- **Describe**: Sensory detail generation with toggleable senses (sight, sound, touch, taste, smell) and metaphors (considers paragraph + 200 preceding words)
- **Expand**: Detailed expansion of brief passages (minimum 3 words, maximum 1,000 words)
- **Brainstorm**: Rapid-fire idea generation with categories (Dialogue, Characters, World building, Plot points, Names, Places, Objects, Descriptions, Article ideas, Tweets, Something else) and "Keepers List" for saving favorites
- **Quick Tools**: Quick Edit and Quick Chat for in-document AI assistance
- **Selection Menu**: Context-sensitive tools (Describe, Quick Edit, Related Words, Expand, Visualize)
- **Related Words**: Smart thesaurus for finding contextually appropriate alternatives
- **Visualize**: Generate images from text descriptions (minimum 10 words, 2500 credits)

#### AI Model Selection & Control
- **Multiple Prose Modes**: Muse, Excellent, Basic and experimental models
- **Saliency Engine**: Intelligent exposure of relevant story information to AI
- **Visibility Settings**: Control AI access to specific Story Bible elements

#### Analysis Tools
- **Content Analysis**: Character consistency, plot holes
- **Style Analysis**: Writing pattern recognition
- **Pacing Analysis**: Scene rhythm and flow

### 4. Plugin System
- **Custom AI Functions**: User-created AI tools with access to Story Bible data
- **Plugin Builder**: Visual interface for creating plugins with prompts and variables
- **Available Variables**: Access to `highlighted_text`, `preceding_text`, and Story Bible elements
- **Plugin Testing**: Built-in testing environment with sample data
- **Plugin Marketplace**: Share and discover community-created plugins
- **Template System**: Pre-built plugin templates for common writing tasks
- **Plugin Guidelines**: Best practices and validation for plugin creation
- **Credit System**: Plugins use standard credit system based on AI model usage
- **Sandboxed Execution**: Secure plugin environment with controlled access
- **API Integration**: Third-party service connections

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
}

// Context for AI operations
pub struct AIContext {
    pub story_bible: Option<StoryBible>,
    pub characters: Vec<Character>,
    pub worldbuilding: Vec<WorldElement>,
    pub previous_text: String,
    pub user_preferences: UserPreferences,
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
- **Point of View (POV) & Tense Settings**: Added controls for narrative perspective and tense.
- **Series Support**: Share Characters, Worldbuilding, and Outlines across multiple projects
- **Chapter Continuity**: Link documents for seamless AI context across chapters
- **Braindump**: Free-form text area in Story Bible for core story vision
- **Visibility Settings**: Control AI access to specific Story Bible elements
- **Saliency Engine**: Intelligent selection of relevant story information for AI
- **Multiple AI Models**: Support for Muse, Claude, GPT with different prose modes
- **Style Examples**: Train AI on user's writing style for personalized prose
- **Document Sharing**: Clean Copy system with private commenting
- **Smart Import**: Novel import with auto-populated Story Bible
- **Selection Menu**: Context-sensitive tools when text is highlighted
- **Quick Tools**: Quick Edit and Quick Chat for in-document assistance
- **Related Words**: Smart thesaurus with contextual suggestions
- **Visualize**: Generate images from text descriptions
- **Advanced Brainstorming**: Keepers List for saving favorite ideas
- **Plugin Testing**: Built-in environment for testing custom plugins
- **Credit Tracking**: Transparent usage monitoring across all AI features
- **Comment System**: In-document commenting for collaboration and notes

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
