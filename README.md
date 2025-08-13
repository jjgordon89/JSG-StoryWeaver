# StoryWeaver

StoryWeaver is a comprehensive desktop application designed for writers, novelists, and storytellers. It combines a powerful writing environment with AI-assisted features to streamline the creative process from initial idea to final manuscript.

## Features

### Core Writing Environment
- **Monaco Editor Integration**: Professional-grade text editor with syntax highlighting and advanced editing features
- **Project Management**: Organize multiple writing projects with hierarchical folder structures
- **Document Management**: Create, edit, and organize documents within projects
- **Series Support**: Manage multi-book series with consistency tracking
- **Auto-save**: Automatic document saving to prevent data loss

### AI-Powered Writing Tools
- **AI Writing Assistant**: Generate content with multiple AI providers (OpenAI, Claude, Gemini)
- **Real-time Streaming**: Watch AI content generation in real-time with typewriter effects
- **Multiple Writing Modes**: Auto-write, guided writing, rewriting, and expansion
- **AI Quick Tools**: Fast text editing and chat functionality
- **Cost Estimation**: Real-time credit/cost estimation for AI operations
- **AI Cards System**: Organize and manage AI-generated content suggestions

### Story Bible System
- **Character Management**: Detailed character profiles with traits and relationships
- **World Building**: Locations, cultures, magic systems, and world elements
- **Story Outlines**: Structured plot organization with scene management
- **AI Generation**: AI-powered content creation for all story elements
- **Search & Filtering**: Advanced search across all story bible components

### Advanced Features
- **Performance Monitoring**: Built-in performance tracking and optimization
- **Error Handling**: Comprehensive error management with user-friendly notifications
- **Theme Support**: Light and dark modes with accessibility features
- **Backup & Recovery**: Automatic backups with restore functionality
- **Version History**: Track document changes over time
- **Focus Mode**: Distraction-free writing environment

## Architecture

### Frontend (React + TypeScript)
- **Component Architecture**: Modular React components with TypeScript
- **State Management**: Zustand for client state, React Query for server state
- **UI Framework**: Custom components with Tailwind CSS
- **Performance**: Optimized rendering with React.memo and useMemo

### Backend (Rust + Tauri)
- **Tauri Framework**: Secure desktop app framework with Rust backend
- **SQLite Database**: Local data storage with migrations
- **AI Provider Integration**: Modular AI provider system
- **Security**: Input validation, rate limiting, and secure API handling

## AI Streaming Architecture

StoryWeaver implements a sophisticated AI streaming system that provides real-time content generation:

### Streaming Flow
1. **Initiation**: User triggers AI writing operation (auto-write or guided-write)
2. **Stream Setup**: Backend creates unique stream ID and establishes event channel
3. **AI Provider**: Connects to AI service (OpenAI, Claude, etc.) with streaming enabled
4. **Real-time Updates**: Content chunks streamed via Tauri events to frontend
5. **UI Updates**: React components update in real-time showing typewriter effect
6. **Completion**: Stream ends, final content processed and saved

### Key Components

#### Backend Streaming (`src-tauri/src/ai/`)
- **AI Providers**: Modular provider system with streaming support
- **Stream Manager**: Handles concurrent streams and cleanup
- **Event System**: Tauri event emission for real-time updates
- **Error Handling**: Graceful stream interruption and recovery

#### Frontend Streaming (`src/hooks/useAI.ts`)
- **useAIWriteStream**: Hook for managing streaming operations
- **Event Listeners**: Tauri event listeners for stream chunks
- **State Management**: Real-time content state updates
- **UI Integration**: Seamless integration with writing components

### Streaming Commands
```rust
// Backend Tauri commands
guided_write_stream(document_id, user_prompt, settings) -> StreamResponse
auto_write_stream(document_id, cursor_position, settings) -> StreamResponse
cancel_streaming_generation(stream_id) -> CancelResponse
```

### Frontend Usage
```typescript
const { startStreamingWrite, streamedContent, streaming } = useAIWriteStream();

// Start streaming
const result = await startStreamingWrite(
  documentId, 
  cursorPosition, 
  "Continue the story"
);

// Monitor real-time content
useEffect(() => {
  if (streaming.isStreaming) {
    console.log('Streaming content:', streamedContent);
  }
}, [streamedContent, streaming.isStreaming]);
```

## AI Card Persistence System

The AI Cards system organizes and persists AI-generated content for easy access and reuse:

### Card Data Flow
1. **Generation**: AI operations create content and metadata
2. **Card Creation**: Content automatically converted to cards (if enabled)
3. **Persistence**: Cards saved to SQLite database with full metadata
4. **Retrieval**: Cards loaded and filtered based on project/document context
5. **Management**: Users can star, collapse, delete, and organize cards

### Database Schema
```sql
CREATE TABLE ai_cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    document_id INTEGER,
    feature_type TEXT NOT NULL,  -- 'write', 'rewrite', 'brainstorm', etc.
    prompt_context TEXT,
    response_text TEXT NOT NULL,
    model_used TEXT,
    provider TEXT,
    token_count INTEGER,
    cost_estimate REAL,
    is_stacked BOOLEAN DEFAULT FALSE,
    is_starred BOOLEAN DEFAULT FALSE,
    is_collapsed BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Card Management Components

#### Backend (`src-tauri/src/database/operations/ai_card_ops.rs`)
- **CRUD Operations**: Create, read, update, delete cards
- **Filtering**: Date range, provider, model, cost filtering
- **Search**: Full-text search across card content
- **Pagination**: Efficient loading of large card collections

#### Frontend (`src/hooks/useCards.ts`)
- **Card Loading**: Automatic fetching with project/document context
- **Filtering**: Real-time filtering by type, starred status, etc.
- **Grouping**: Cards grouped by feature type for stacked display
- **State Management**: Optimistic updates and error handling

### Card Integration
```typescript
const { cardsByFeatureType, toggleStar, addCard } = useCards({
  projectId: 1,
  documentId: 5,
  filterType: 'write',
  sortOrder: 'newest'
});

// Cards automatically created during AI operations
const result = await writeWithCards(documentId, cursorPosition, prompt, {
  card_count: 3  // Generate 3 cards from result
});
```

## Development Setup

### Prerequisites
- Node.js 18+ and npm
- Rust 1.70+
- Tauri CLI: `npm install -g @tauri-apps/cli`

### Installation
```bash
# Clone repository
git clone https://github.com/your-username/storyweaver.git
cd storyweaver

# Install frontend dependencies
npm install

# Install Rust dependencies (handled by Tauri)
npm run tauri build

# Development mode
npm run tauri dev
```

### Project Structure
```
storyweaver/
├── src/                          # Frontend React app
│   ├── components/              # React components
│   │   ├── ai/                 # AI-related components
│   │   ├── cards/              # Card system components
│   │   └── ...
│   ├── hooks/                  # Custom React hooks
│   │   ├── useAI.ts           # Main AI hook
│   │   ├── useCards.ts        # Card management
│   │   └── ...
│   ├── stores/                 # Zustand state stores
│   ├── features/               # Feature-specific code
│   │   └── story-bible/       # Story Bible system
│   └── types/                  # TypeScript definitions
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── ai/                # AI provider system
│   │   ├── commands/          # Tauri command handlers
│   │   ├── database/          # Database operations
│   │   └── ...
│   └── migrations/            # Database migrations
└── tests/                      # Test files
```

## API Documentation

### AI Hooks

#### `useAI()`
Main AI interface providing unified access to all AI operations.

**Returns:**
- `writeWithCards()` - Enhanced write with automatic card creation
- `generateCardsFromText()` - Create cards from existing text
- `isAnyLoading` - Loading state across all AI operations
- `hasError` - Error state indicator

#### `useAIWriteStream()`
Real-time streaming interface for AI writing operations.

**Returns:**
- `startStreamingWrite()` - Begin streaming write operation
- `stopStreamingWrite()` - Cancel active stream
- `streamedContent` - Current streamed content
- `streaming` - Stream state information

#### `useCards()`
Card management interface with filtering and organization.

**Parameters:**
- `projectId` - Project to load cards for
- `documentId` - Optional document filter
- `filterType` - Filter by AI feature type
- `showStarredOnly` - Show only starred cards
- `sortOrder` - Sort by newest/oldest

**Returns:**
- `cardsByFeatureType` - Cards grouped by type
- `toggleStar()` - Toggle card starred state
- `deleteCard()` - Remove card
- `setFilterType()` - Update filter

### Story Bible Hook

#### `useStoryBible()`
Comprehensive Story Bible management system.

**Key Operations:**
- Character management with traits
- World-building elements
- Story outlines and scenes
- AI generation for all components
- Search and filtering capabilities

## Testing

### Frontend Tests
```bash
# Run unit tests
npm test

# Run with coverage
npm run test:coverage
```

### Backend Tests
```bash
# Run Rust tests
cd src-tauri
cargo test

# Run integration tests
cargo test --test integration_tests
```

### End-to-End Tests
```bash
# Run Playwright tests
npm run test:e2e
```

## Building for Production

### Development Build
```bash
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

### Windows MSI Installer
```bash
npm run build:windows
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

### Code Style
- Frontend: ESLint + Prettier configuration
- Backend: Rust standard formatting with `cargo fmt`
- Commit messages: Conventional Commits format

## Security

StoryWeaver implements comprehensive security measures:
- Input validation on all Tauri commands
- Rate limiting for AI operations
- Secure API key storage
- SQL injection prevention
- XSS protection

## Performance

- Component-level performance monitoring
- Database query optimization
- Memory usage tracking
- AI operation cost estimation
- Lazy loading and code splitting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- Documentation: [docs/](docs/)
- Issues: [GitHub Issues](https://github.com/your-username/storyweaver/issues)
- Discussions: [GitHub Discussions](https://github.com/your-username/storyweaver/discussions)

## Acknowledgments

- [Tauri](https://tauri.app/) - Desktop app framework
- [React](https://reactjs.org/) - Frontend framework
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) - Code editor
- [SQLite](https://sqlite.org/) - Database engine
- AI Providers: OpenAI, Anthropic, Google
