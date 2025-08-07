# Phase 2: Core Writing Features (Weeks 6-10) - Updated Plan

## Overview
Implement the fundamental AI-powered writing tools that form the core of StoryWeaver's functionality. This phase focuses on AI API integrations, a highly responsive text editor, essential writing assistance features, and the foundational systems for context management, credit tracking, and robust error handling.

## Current Status
Phase 2 implementation has begun with the foundation of the AI provider system and some initial writing features. The following components have been partially implemented:

- **AI Provider Framework:** The basic abstraction layer has been created with support for OpenAI, Claude, and Gemini providers.
- **Write Processor:** Initial implementation of the WriteProcessor with support for auto_write, guided_write, and tone_shift_write modes.
- **Card System UI:** Frontend components for displaying AI responses with stacking, filtering, and sorting capabilities (using mock data).
- **AI Writing Commands:** Basic Tauri commands for AI writing features.

## Key Objectives (Remaining)
- **Complete AI Integration:** Finish implementing the AI provider system with full streaming support and connect to the frontend.
- **Advanced Editor:** Enhance the editor with an intelligent selection menu, a responsive three-column layout, and real-time UI updates for streaming generation.
- **Core Writing Tools:** Complete the implementation of all writing and editing tools, including Write, Rewrite, Expand, Describe, Brainstorm, and Visualize.
- **Intelligent Systems:** Develop the foundational logic for the Saliency Engine (context assembly), a comprehensive Credit Management system, and advanced Error Handling workflows.
- **Quick Tools:** Implement the Quick Edit and Quick Chat features with story-aware context and a High Quality mode.
- **Collaboration:** Introduce a basic commenting system and notification framework.
- **UI/UX:** Implement purple text highlighting for AI-generated content and connect the card stacking system to the backend.

## Technical Tasks

### Week 6: AI & Systems Foundation
- **AI Provider Framework:**
  - [x] Implement AI provider abstraction layer with trait-based interface
  - [x] Create basic integrations for OpenAI, Claude, Gemini providers
  - [ ] Complete streaming implementation for all providers
  - [ ] Implement DALL-E 3 / Google Imagen integration for the Visualize feature
  - [ ] Connect AI providers to the frontend with proper error handling
- **Core Systems:**
  - [ ] **Saliency Engine (Foundation):** Develop initial context relevance algorithms and the context optimizer
  - [x] **Token Management:** Implement basic token counting and credit calculation
  - [ ] Enhance token management with optimization strategies and a token budget calculator
  - [ ] **Credit Management:** Build the cost estimation engine, usage tracker, and low-balance warning system
  - [ ] **Error Handling:** Create the error recovery manager with strategies for network timeouts, API rate limits, and content filtering

### Week 7: Enhanced & Responsive Editor
- **UI Framework:**
  - [ ] **Three-Column Layout:** Implement the responsive layout manager with collapsible side panels
  - [ ] Implement column resizing logic with saved user preferences
- **Editor Features:**
  - [ ] Upgrade Monaco Editor with custom features and syntax highlighting
  - [ ] **Intelligent Selection Menu:** Implement dynamic tool selection based on context (word count, document type, etc.)
  - [ ] Add hover menu for context-sensitive tool access
  - [ ] **Streaming UI:** Build the UI for real-time text generation (typewriter effect, progress indicators, pause/resume controls)
  - [ ] Create purple text highlighting for AI-generated content with automatic removal on edit
  - [x] Implement a distraction-free Focus Mode

### Week 8: Core Writing & Creative Tools
- **Writing Tools:**
  - [x] Implement basic **Write** feature with multiple modes (Auto, Guided, Tone Shift)
  - [ ] Complete the Write feature with frontend integration and streaming support
  - [ ] Build **Rewrite** tool with multiple styles (Rephrase, Shorter, More Descriptive, etc.)
  - [ ] Create **Expand** and **Describe** features with sensory detail toggles
  - [ ] Add configurable creativity levels (1-10) and Key Details for project-level context
- **Creative Tools:**
  - [ ] **Brainstorm:** Implement the brainstorming tool with category-specific prompts and a "Keepers List" with voting
  - [ ] **Visualize:** Build the UI for generating images from text descriptions
- **UI:**
  - [x] **Card Stacking System:** Implement the UI for organizing AI responses into collapsible, stackable cards
  - [ ] Connect the Card System to the backend for persistent storage and retrieval of AI responses

### Week 9: Quick Tools & Contextual Systems
- **Quick Tools:**
  - [ ] Implement Quick Edit and Quick Chat functionality accessible via `Ctrl/Cmd+K`
  - [ ] Integrate **High Quality Mode** with credit system warnings
  - [ ] Build inline editing UI with struck-through original text and green suggestions
  - [ ] Implement Tab toggle between Quick Edit and Quick Chat
- **Context & State Management:**
  - [ ] **Story-Aware Context:** Integrate the Saliency Engine to provide deep context to Quick Tools
  - [ ] Implement session management for undo/redo of Quick Tool actions
  - [x] **State Synchronization:** Develop initial logic for multi-document state management and conflict detection
  - [x] Build background processing queue for non-critical AI operations

### Week 10: Collaboration & Final Touches
- **Collaboration Features:**
  - [ ] Implement the basic commenting system with threading and replies
  - [ ] Add author vs. reader comment distinction and visibility controls
  - [ ] Build a notification system for new comments and other events
- **Refinements:**
  - [ ] **Related Words:** Implement the smart thesaurus with contextual analysis and an expandable word cloud interface
  - [ ] **Performance Optimization:** Introduce lazy loading and caching for documents and AI responses
  - [ ] Ensure all new features are integrated with the error handling and credit management systems

## Implementation Progress

### AI Provider System
The AI provider system has been implemented with a trait-based approach that defines a common interface for all AI providers:

```rust
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> anyhow::Result<String>;
    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> anyhow::Result<TextStream>;
    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> anyhow::Result<String>;
    // Additional methods for other writing features...
}
```

Basic implementations for OpenAI, Claude, and Gemini providers have been created, but they need to be completed with full streaming support and proper error handling.

### Write Processor
The WriteProcessor has been implemented with support for three writing modes:

```rust
pub struct WriteProcessor {
    ai_provider: Arc<dyn AIProvider>,
    context_builder: ContextBuilder,
}

impl WriteProcessor {
    pub async fn auto_write(&self, document_id: i32, cursor_position: usize, settings: &WriteSettings, db_pool: &DbPool) -> Result<WriteResult> {
        // Implementation for auto-continuing text
    }
    
    pub async fn guided_write(&self, document_id: i32, user_prompt: &str, settings: &WriteSettings, db_pool: &DbPool) -> Result<WriteResult> {
        // Implementation for following user's specific instructions
    }
    
    pub async fn tone_shift_write(&self, document_id: i32, cursor_position: usize, tone: &str, settings: &WriteSettings, db_pool: &DbPool) -> Result<WriteResult> {
        // Implementation for writing with a specific tone
    }
}
```

The WriteProcessor needs to be connected to the frontend and enhanced with streaming support.

### Card System
The Card System UI has been implemented with support for displaying AI responses in a stacked, filterable interface:

```typescript
export const CardSystem: React.FC<CardSystemProps> = ({
  projectId,
  documentId,
  onCardAction,
}) => {
  // Implementation for displaying and managing AI response cards
}
```

However, it's currently using mock data and needs to be connected to the backend for persistent storage and retrieval of AI responses.

## Next Steps
1. **Complete AI Provider Integration:**
   - Finish implementing all AI provider methods
   - Add proper streaming support for all providers
   - Connect AI providers to the frontend

2. **Implement Core Writing Tools:**
   - Complete the Write feature with frontend integration
   - Implement Rewrite, Expand, and Describe features
   - Add Brainstorm and Visualize tools

3. **Build Intelligent Selection Menu:**
   - Create context-aware tool selection
   - Implement hover menu for quick access to tools

4. **Develop Saliency Engine:**
   - Implement context relevance algorithms
   - Create context optimizer for token efficiency

5. **Implement Quick Tools:**
   - Build Quick Edit and Quick Chat functionality
   - Add High Quality mode with credit system integration

6. **Connect Card System to Backend:**
   - Implement backend storage for AI responses
   - Connect frontend card components to backend data

7. **Add Collaboration Features:**
   - Implement commenting system
   - Build notification framework

## Success Criteria
- [ ] All AI providers (OpenAI, Claude, Gemini) integrate successfully with streaming support
- [ ] Write tools generate contextually appropriate content and connect to the frontend
- [ ] Selection menu adapts based on text selection length and context
- [ ] Quick Tools accessible via Ctrl+K shortcut with tab toggle between Edit and Chat
- [ ] Purple highlighting tracks AI-generated content
- [ ] Related Words provides contextual alternatives
- [ ] Comments system supports basic collaboration
- [ ] Rate limiting prevents API quota issues
- [ ] Error handling gracefully manages API failures
- [ ] Card system organizes AI responses effectively with backend persistence

## Risk Mitigation
- **API Rate Limits**: Implement robust rate limiting and queuing
- **Token Management**: Accurate token counting for cost control
- **Context Building**: Efficient context assembly without exceeding limits
- **Error Recovery**: Graceful handling of API failures and network issues
- **Performance**: Optimize AI response rendering and storage

## Dependencies
### Rust
- reqwest = { version = "0.11", features = ["json", "stream"] }
- tokio-stream = "0.1"
- serde = { version = "1.0", features = ["derive"] }
- uuid = { version = "1.0", features = ["v4"] }
- tiktoken-rs = "0.5" # Token counting

### Frontend
- @monaco-editor/react = "^4.6.0"
- react-markdown = "^9.0.0"
- framer-motion = "^10.16.0"
- @tanstack/react-query = "^5.0.0"
- react-hotkeys-hook = "^4.4.0"

## Next Phase
Phase 3 will focus on implementing the comprehensive Story Bible system, building upon the writing tools established in this phase.
