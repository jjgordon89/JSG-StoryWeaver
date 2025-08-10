# Phase 2: Core Writing Features (Weeks 6-10)

## Overview

Implement the fundamental AI-powered writing tools that form the core of StoryWeaver's functionality. This phase focuses on AI API integrations, a highly responsive text editor, essential writing assistance features, and the foundational systems for context management, credit tracking, and robust error handling.

## Key Objectives

- **AI Integration:** Implement a flexible provider system for OpenAI, Claude, Gemini, and OpenAI-compatible endpoints with full streaming support.
- **Advanced Editor:** Enhance the editor with an intelligent selection menu, a responsive three-column layout, and real-time UI updates for streaming generation.
- **Core Writing Tools:** Build the main writing and editing tools, including Write, Rewrite, Expand, Describe, Brainstorm, and Visualize.
- **Intelligent Systems:** Develop the foundational logic for the Saliency Engine (context assembly), a comprehensive Credit Management system, and advanced Error Handling workflows.
- **Quick Tools:** Implement the Quick Edit and Quick Chat features with story-aware context and a High Quality mode.
- **Collaboration:** Introduce a basic commenting system and notification framework.
- **UI/UX:** Implement the card stacking system for AI responses and purple text highlighting for AI-generated content.

## Technical Tasks

### Week 6: AI & Systems Foundation

- **AI Provider Framework:**
  - [ ] Implement AI provider abstraction layer with support for streaming and embeddings.
  - [ ] Create integrations for OpenAI, Claude, Gemini, and OpenAI-compatible endpoints.
  - [ ] Implement DALL-E 3 / Google Imagen integration for the Visualize feature.
- **Core Systems:**
  - [ ] **Saliency Engine (Foundation):** Develop initial context relevance algorithms and the context optimizer.
  - [ ] **Token Management:** Implement precise token counting, optimization strategies, and a token budget calculator.
  - [ ] **Credit Management:** Build the cost estimation engine, usage tracker, and low-balance warning system.
  - [ ] **Error Handling:** Create the error recovery manager with strategies for network timeouts, API rate limits, and content filtering.

### Week 7: Enhanced & Responsive Editor

- **UI Framework:**
  - [ ] **Three-Column Layout:** Implement the responsive layout manager with collapsible side panels.
  - [ ] Implement column resizing logic with saved user preferences.
- **Editor Features:**
  - [ ] Upgrade Monaco Editor with custom features and syntax highlighting.
  - [ ] **Intelligent Selection Menu:** Implement dynamic tool selection based on context (word count, document type, etc.).
  - [ ] Add hover menu for context-sensitive tool access.
  - [ ] **Streaming UI:** Build the UI for real-time text generation (typewriter effect, progress indicators, pause/resume controls).
  - [ ] Create purple text highlighting for AI-generated content with automatic removal on edit.
  - [ ] Implement a distraction-free Focus Mode.

### Week 8: Core Writing & Creative Tools

- **Writing Tools:**
  - [ ] Implement **Write** feature with multiple modes (Auto, Guided, Tone Shift).
  - [ ] Build **Rewrite** tool with multiple styles (Rephrase, Shorter, More Descriptive, etc.).
  - [ ] Create **Expand** and **Describe** features with sensory detail toggles.
  - [ ] Add configurable creativity levels (1-10) and Key Details for project-level context.
- **Creative Tools:**
  - [ ] **Brainstorm:** Implement the brainstorming tool with category-specific prompts and a "Keepers List" with voting.
  - [ ] **Visualize:** Build the UI for generating images from text descriptions.
- **UI:**
  - [ ] **Card Stacking System:** Implement the UI for organizing AI responses into collapsible, stackable cards with prompt context.

### Week 9: Quick Tools & Contextual Systems

- **Quick Tools:**
  - [ ] Implement Quick Edit and Quick Chat functionality accessible via `Ctrl/Cmd+K`.
  - [ ] Integrate **High Quality Mode** with credit system warnings.
  - [ ] Build inline editing UI with struck-through original text and green suggestions.
  - [ ] Implement Tab toggle between Quick Edit and Quick Chat.
- **Context & State Management:**
  - [ ] **Story-Aware Context:** Integrate the Saliency Engine to provide deep context to Quick Tools.
  - [ ] Implement session management for undo/redo of Quick Tool actions.
  - [ ] **State Synchronization:** Develop initial logic for multi-document state management and conflict detection.
  - [ ] Build background processing queue for non-critical AI operations.

### Week 10: Collaboration & Final Touches

- **Collaboration Features:**
  - [ ] Implement the basic commenting system with threading and replies.
  - [ ] Add author vs. reader comment distinction and visibility controls.
  - [ ] Build a notification system for new comments and other events.
- **Refinements:**
  - [ ] **Related Words:** Implement the smart thesaurus with contextual analysis and an expandable word cloud interface.
  - [ ] **Performance Optimization:** Introduce lazy loading and caching for documents and AI responses.
  - [ ] Ensure all new features are integrated with the error handling and credit management systems.

## AI Provider Implementation

### Provider Architecture

```rust
pub trait AIProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String>;
    async fn generate_text_stream(&self, prompt: &str, context: &AIContext) -> Result<TextStream>;
    async fn rewrite_text(&self, text: &str, style: &RewriteStyle) -> Result<String>;
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>>;
    fn supports_streaming(&self) -> bool;
    fn get_context_window(&self) -> usize;
    fn get_model_name(&self) -> &str;
}

pub struct AIProviderManager {
    providers: HashMap<String, Box<dyn AIProvider + Send + Sync>>,
    default_provider: Option<String>,
    rate_limiter: Arc<RateLimiter>,
}
```

### OpenAI Integration

```rust
pub struct OpenAIProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    rate_limiter: Arc<RateLimiter>,
}

impl AIProvider for OpenAIProvider {
    async fn generate_text(&self, prompt: &str, context: &AIContext) -> Result<String> {
        let messages = self.build_messages(prompt, context)?;
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            max_tokens: Some(2000),
            temperature: Some(0.7),
            stream: Some(false),
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
            
        // Parse response and return content
        Ok(response_content)
    }
}
```

## Writing Tools Implementation

### Write Feature

```rust
pub struct WriteProcessor {
    ai_provider: Arc<dyn AIProvider>,
    context_builder: ContextBuilder,
}

impl WriteProcessor {
    pub async fn auto_write(&self, 
        document_id: i32, 
        cursor_position: usize,
        settings: &WriteSettings
    ) -> Result<WriteResult> {
        // Get up to 1000 words of context
        let context = self.context_builder
            .build_write_context(document_id, cursor_position, 1000)
            .await?;
        
        let prompt = format!(
            "Continue this story naturally. Context: {}\n\nContinue from here:",
            context.preceding_text
        );
        
        let result = self.ai_provider
            .generate_text(&prompt, &context.ai_context)
            .await?;
        
        Ok(WriteResult {
            generated_text: result,
            credits_used: self.calculate_credits(&result),
            word_count: self.count_words(&result),
        })
    }
    
    pub async fn guided_write(&self,
        document_id: i32,
        user_prompt: &str,
        settings: &WriteSettings
    ) -> Result<WriteResult> {
        let context = self.context_builder
            .build_write_context(document_id, 0, 1000)
            .await?;
        
        let prompt = format!(
            "Write the next part of this story based on this direction: '{}'\n\nStory context: {}",
            user_prompt, context.story_summary
        );
        
        let result = self.ai_provider
            .generate_text(&prompt, &context.ai_context)
            .await?;
        
        Ok(WriteResult {
            generated_text: result,
            credits_used: self.calculate_credits(&result),
            word_count: self.count_words(&result),
        })
    }
}
```

### Selection Menu System

```typescript
interface SelectionMenuProps {
  selectedText: string;
  selectionStart: number;
  selectionEnd: number;
  onToolSelect: (tool: string) => void;
}

export const SelectionMenu: React.FC<SelectionMenuProps> = ({
  selectedText,
  selectionStart,
  selectionEnd,
  onToolSelect
}) => {
  const availableTools = useMemo(() => {
    const wordCount = selectedText.split(' ').length;
    
    if (wordCount === 1) {
      return ['Related Words', 'Quick Edit'];
    } else if (wordCount <= 50) {
      return ['Describe', 'Quick Edit', 'Expand', 'Rewrite'];
    } else {
      return ['Rewrite', 'Quick Edit', 'Visualize'];
    }
  }, [selectedText]);

  return (
    <div className="selection-menu">
      {availableTools.map(tool => (
        <button
          key={tool}
          onClick={() => onToolSelect(tool)}
          className="tool-button"
        >
          {tool}
        </button>
      ))}
    </div>
  );
};
```

## Database Schema Extensions

### AI History and Responses

```sql
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

-- AI Response Cards
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

-- Comments
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

-- Purple Text Highlighting
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
```

## Frontend Components

### Quick Tools Interface

```typescript
interface QuickToolsProps {
  documentId: number;
  selectedText?: string;
  onClose: () => void;
}

export const QuickTools: React.FC<QuickToolsProps> = ({
  documentId,
  selectedText,
  onClose
}) => {
  const [mode, setMode] = useState<'edit' | 'chat'>('edit');
  const [highQuality, setHighQuality] = useState(false);
  const [userInput, setUserInput] = useState('');

  const handleTabToggle = useCallback((e: KeyboardEvent) => {
    if (e.key === 'Tab') {
      e.preventDefault();
      setMode(mode === 'edit' ? 'chat' : 'edit');
    }
  }, [mode]);

  useEffect(() => {
    document.addEventListener('keydown', handleTabToggle);
    return () => document.removeEventListener('keydown', handleTabToggle);
  }, [handleTabToggle]);

  return (
    <div className="quick-tools-modal">
      <div className="quick-tools-header">
        <button 
          className={mode === 'edit' ? 'active' : ''}
          onClick={() => setMode('edit')}
        >
          Quick Edit
        </button>
        <button 
          className={mode === 'chat' ? 'active' : ''}
          onClick={() => setMode('chat')}
        >
          Quick Chat
        </button>
        <label>
          <input 
            type="checkbox" 
            checked={highQuality}
            onChange={(e) => setHighQuality(e.target.checked)}
          />
          High Quality Mode
        </label>
      </div>
      
      {mode === 'edit' ? (
        <QuickEditInterface 
          selectedText={selectedText}
          highQuality={highQuality}
          onSubmit={handleQuickEdit}
        />
      ) : (
        <QuickChatInterface 
          documentId={documentId}
          highQuality={highQuality}
          onSubmit={handleQuickChat}
        />
      )}
    </div>
  );
};
```

## Success Criteria

- [ ] All AI providers (OpenAI, Claude, Gemini) integrate successfully
- [ ] Write tools generate contextually appropriate content
- [ ] Selection menu adapts based on text selection length
- [ ] Quick Tools accessible via Ctrl+K shortcut
- [ ] Purple highlighting tracks AI-generated content
- [ ] Related Words provides contextual alternatives
- [ ] Comments system supports basic collaboration
- [ ] Rate limiting prevents API quota issues
- [ ] Error handling gracefully manages API failures
- [ ] Card system organizes AI responses effectively

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
