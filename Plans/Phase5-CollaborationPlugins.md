# Phase 5: Collaboration & Plugins (Weeks 20-22)

## Overview

Implement comprehensive collaboration features and the plugin system, including document sharing with Clean Copy commenting, plugin builder interface, plugin marketplace, and Canvas implementation with visual story planning tools.

## Implementation Status

**Overall Progress: 100% Complete**

### ✅ Completed Features

**Database Infrastructure:**

- Complete database schema for collaboration, plugins, and canvas features
- Migration files for all Phase 5 tables
- Models for SharedDocument, CollaborationSession, Plugin, PluginMarketplace, Canvas entities

**Backend Implementation:**

- Collaboration commands and API endpoints
- Plugin system commands and execution engine
- Canvas collaboration session management
- Comment system with threading support
- Document sharing with secure token generation
- Plugin marketplace with rating and discovery systems

**Frontend Components:**

- Plugin builder interface with Basic and Advanced editors
- Comment system UI components
- Canvas implementation with drag-and-drop functionality
- Plugin marketplace interface
- Collaboration session management UI

### ✅ All Tasks Completed

- ✅ Unpublish/republish functionality for shared documents
- ✅ Notification system for collaboration events
- ✅ Outline generation from single sentences or paragraphs

## Key Objectives

- Document sharing with Clean Copy commenting system
- Plugin system with builder interface and testing environment
- Plugin marketplace with visibility controls and ratings
- Canvas implementation with drag-and-drop functionality
- Visual story planning tools and outline templates
- Advanced collaboration features with reader management
- Plugin sandboxing and security measures
- Community integration and plugin sharing

## Technical Tasks

### Week 20: Document Sharing & Collaboration

- [x] Implement Clean Copy document sharing system
- [x] Create secure share link generation with tokens
- [x] Build reader display name and anonymous options
- [x] Add private commenting between author and individual readers
- [x] Implement document duplication for multiple share links
- [x] Create unpublish/republish functionality
- [x] Build comment threading and reply system
- [x] Add notification system for new comments and collaboration events
- [x] Implement reader permission management
- [x] Create collaboration session management

### Week 21: Plugin System Foundation

- [x] Design and implement plugin architecture with WASM sandboxing
- [x] Create plugin builder interface with Basic and Advanced editors
- [x] Implement variable injection system with Story Bible data access
- [x] Add multi-stage prompt support (up to 2 stages)
- [x] Build plugin testing environment with sample data
- [x] Create AI model selection for plugins (GPT-4o-mini, GPT-4.1, Gemini-2.5-pro)
- [x] Implement plugin parameter configuration (temperature, penalties, stop sequences)
- [x] Add plugin validation and safety checks
- [x] Build plugin execution engine with resource limits

### Week 22: Plugin Marketplace & Canvas

- [x] Create plugin marketplace with publishing system
- [x] Implement plugin visibility controls (published/unlisted/private)
- [x] Add plugin rating and review system
- [x] Build plugin discovery and search functionality
- [x] Create plugin templates for common writing tasks
- [x] Implement Canvas with drag-and-drop story planning
- [x] Add outline templates (Hero's Journey, Hollywood Beats, Story Circle, Romance)
- [x] Build visual story mapping with character arcs and plot threads
- [x] Create keyboard shortcuts for Canvas (Select all, Delete, Pan, Zoom, Undo/Redo)
- [x] Add outline generation from single sentences or paragraphs

## Collaboration System Architecture

### Document Sharing

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedDocument {
    pub id: i32,
    pub document_id: i32,
    pub share_token: String,
    pub is_active: bool,
    pub allow_comments: bool,
    pub allow_anonymous: bool,
    pub max_participants: Option<i32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub id: i32,
    pub document_id: i32,
    pub session_token: String,
    pub session_name: Option<String>,
    pub is_active: bool,
    pub allow_anonymous: bool,
    pub max_participants: i32,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

pub struct CollaborationManager {
    session_store: Arc<dyn SessionStore>,
    comment_processor: CommentProcessor,
    notification_service: NotificationService,
    token_generator: TokenGenerator,
}

impl CollaborationManager {
    pub async fn create_share_link(&self,
        document_id: i32,
        settings: ShareSettings
    ) -> Result<ShareLink> {
        let share_token = self.token_generator.generate_secure_token();
        
        let shared_document = SharedDocument {
            id: 0, // Will be set by database
            document_id,
            share_token: share_token.clone(),
            is_active: true,
            allow_comments: settings.allow_comments,
            allow_anonymous: settings.allow_anonymous,
            max_participants: settings.max_participants,
            expires_at: settings.expires_at,
            created_at: Utc::now(),
        };
        
        let saved_document = self.session_store
            .create_shared_document(shared_document)
            .await?;
        
        Ok(ShareLink {
            token: share_token,
            url: format!("https://storyweaver.app/shared/{}", share_token),
            document_id,
            settings,
            created_at: saved_document.created_at,
        })
    }
    
    pub async fn add_comment(&self,
        document_id: i32,
        comment_request: CommentRequest
    ) -> Result<Comment> {
        // Validate permissions
        self.validate_comment_permissions(document_id, &comment_request.user_token).await?;
        
        let comment = Comment {
            id: 0, // Will be set by database
            document_id,
            user_name: comment_request.user_name,
            comment_text: comment_request.text,
            start_position: comment_request.start_position,
            end_position: comment_request.end_position,
            is_author_comment: comment_request.is_author,
            parent_comment_id: comment_request.parent_comment_id,
            created_at: Utc::now(),
        };
        
        let saved_comment = self.comment_processor.save_comment(&comment).await?;
        
        // Send notifications
        if !comment_request.is_author {
            self.notification_service
                .notify_author_of_comment(document_id, &saved_comment)
                .await?;
        }
        
        Ok(saved_comment)
    }
    
    pub async fn duplicate_document_for_sharing(&self,
        original_document_id: i32,
        new_share_settings: ShareSettings
    ) -> Result<(i32, ShareLink)> {
        // Create document copy
        let duplicated_document = self.session_store
            .duplicate_document(original_document_id)
            .await?;
        
        // Create new share link for the duplicate
        let share_link = self.create_share_link(
            duplicated_document.id,
            new_share_settings
        ).await?;
        
        Ok((duplicated_document.id, share_link))
    }
}
```

## Implemented Technical Components

### Database Schema

**Collaboration Tables:**

- `shared_documents` - Document sharing configuration and tokens
- `collaboration_sessions` - Active collaboration sessions
- `document_comments` - Comment system with threading support
- `collaboration_participants` - Session participant management

**Plugin System Tables:**

- `plugins` - Plugin definitions and metadata
- `plugin_marketplace` - Marketplace listings and visibility
- `plugin_ratings` - User ratings and reviews
- `plugin_usage_stats` - Usage analytics and metrics
- `plugin_templates` - Pre-built plugin templates

**Canvas Tables:**

- `canvas` - Canvas workspace definitions
- `canvas_elements` - Individual canvas elements and positioning
- `canvas_collaboration_sessions` - Canvas-specific collaboration

### Backend Implementation

**Rust Commands:**

- `commands::collaboration` - Document sharing and comment management
- `commands::plugin` - Plugin execution and marketplace operations
- `commands::canvas` - Canvas collaboration and element management

**Data Models:**

- `SharedDocument` - Document sharing configuration
- `CollaborationSession` - Session management
- `Comment` and `CommentRequest` - Comment system
- `Plugin` and `PluginMarketplace` - Plugin definitions
- `CanvasCollaborationSession` - Canvas collaboration

**Key Features Implemented:**

- Secure token generation for document sharing
- Comment threading and reply system
- Plugin execution engine with sandboxing
- Canvas element positioning and collaboration
- Marketplace rating and discovery systems

### Frontend Components

**React Components:**

- `PluginBuilder` - Plugin creation interface
- `CommentSystem` - Comment UI and threading
- Canvas components with drag-and-drop
- Plugin marketplace interface
- Collaboration session management

**Dependencies Added:**

- `reactflow` - Canvas flow diagrams
- `react-dnd` - Drag and drop functionality
- `konva` and `react-konva` - Canvas rendering
- `@monaco-editor/react` - Code editor for plugins
- `yjs` and `y-webrtc` - Real-time collaboration

### Comment System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub document_id: i32,
    pub user_name: Option<String>,
    pub comment_text: String,
    pub start_position: i32,
    pub end_position: i32,
    pub is_author_comment: bool,
    pub parent_comment_id: Option<i32>,
    pub is_resolved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRequest {
    pub user_token: String,
    pub user_name: Option<String>,
    pub text: String,
    pub start_position: i32,
    pub end_position: i32,
    pub is_author: bool,
    pub parent_comment_id: Option<i32>,
}

pub struct CommentProcessor {
    database: Arc<Database>,
    text_analyzer: TextAnalyzer,
}

impl CommentProcessor {
    pub async fn save_comment(&self, comment: &Comment) -> Result<Comment> {
        // Validate comment content
        self.validate_comment_content(&comment.comment_text)?;
        
        // Check for spam or inappropriate content
        self.check_content_safety(&comment.comment_text).await?;
        
        // Save to database
        let saved_comment = self.database.insert_comment(comment).await?;
        
        // Update comment statistics
        self.update_comment_statistics(comment.document_id).await?;
        
        Ok(saved_comment)
    }
    
    pub async fn get_comments_for_document(&self, 
        document_id: i32,
        user_token: Option<&str>
    ) -> Result<Vec<CommentThread>> {
        let comments = self.database
            .get_comments_by_document(document_id)
            .await?;
        
        // Filter comments based on user permissions
        let filtered_comments = self.filter_comments_by_permissions(
            comments, 
            document_id, 
            user_token
        ).await?;
        
        // Organize into threads
        let comment_threads = self.organize_into_threads(filtered_comments);
        
        Ok(comment_threads)
    }
}
```

## Plugin System Architecture

### Plugin Engine

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub prompt_template: String,
    pub variables: Vec<PluginVariable>,
    pub ai_model: String,
    pub model_parameters: ModelParameters,
    pub is_multi_stage: bool,
    pub stage_count: i32,
    pub is_public: bool,
    pub created_by: String,
    pub test_data: Option<PluginTestData>,
    pub guidelines: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginVariable {
    pub name: String,
    pub variable_type: PluginVariableType,
    pub description: String,
    pub is_required: bool,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginVariableType {
    HighlightedText,
    PrecedingText,
    UserTextInput,
    PreviousDocumentText,
    Braindump,
    Genre,
    Style,
    Synopsis,
    Characters,
    CharactersRaw,
    Outline,
    SceneSummary,
    IsStoryBibleActive,
    ChapterScenes,
    ChapterScenesExtraInstructions,
    Worldbuilding,
    WorldbuildingRaw,
}

pub struct PluginExecutionEngine {
    wasm_runtime: Arc<WasmRuntime>,
    ai_provider_manager: Arc<AIProviderManager>,
    variable_injector: VariableInjector,
    security_manager: SecurityManager,
}

impl PluginExecutionEngine {
    pub async fn execute_plugin(&self,
        plugin: &Plugin,
        context: &PluginExecutionContext
    ) -> Result<PluginExecutionResult> {
        // Security validation
        self.security_manager.validate_plugin_safety(plugin)?;
        
        // Resource limits check
        self.security_manager.check_resource_limits(plugin)?;
        
        // Inject variables into prompt template
        let processed_prompts = self.variable_injector
            .inject_variables(&plugin.prompt_template, context)
            .await?;
        
        let mut stage_results = Vec::new();
        let mut intermediate_results = Vec::new();
        
        // Execute multi-stage prompts
        for (stage_index, prompt) in processed_prompts.iter().enumerate() {
            // Update context with intermediate results
            let mut stage_context = context.clone();
            stage_context.intermediate_results = intermediate_results.clone();
            
            // Build AI context for this stage
            let ai_context = self.build_ai_context_for_stage(&stage_context, stage_index).await?;
            
            // Execute AI generation
            let result = self.ai_provider_manager
                .generate_text(
                    Some(&plugin.ai_model),
                    prompt,
                    &ai_context
                ).await?;
            
            stage_results.push(PluginStageResult {
                stage: stage_index,
                prompt: prompt.clone(),
                result: result.clone(),
                credits_used: self.calculate_stage_credits(&result, &plugin.ai_model),
            });
            
            // Store intermediate result for next stage
            intermediate_results.push(result);
        }
        
        // Calculate total credits used
        let total_credits = stage_results.iter()
            .map(|r| r.credits_used)
            .sum();
        
        // Track plugin usage
        self.track_plugin_usage(plugin, context, &stage_results).await?;
        
        Ok(PluginExecutionResult {
            plugin_id: plugin.id,
            execution_id: Uuid::new_v4().to_string(),
            stage_results,
            total_credits_used: total_credits,
            execution_time_ms: context.start_time.elapsed().as_millis() as u32,
            success: true,
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
        
        // Execute in test mode (no credit consumption)
        let execution_result = self.execute_plugin_test_mode(plugin, &test_context).await?;
        
        // Analyze results for potential issues
        let warnings = self.analyze_test_results(&execution_result);
        
        Ok(PluginTestResult {
            success: execution_result.success,
            execution_result,
            test_warnings: warnings,
            estimated_credits: execution_result.total_credits_used,
        })
    }
}
```

### Plugin Marketplace

```rust
pub struct PluginMarketplace {
    database: Arc<Database>,
    search_engine: SearchEngine,
    rating_system: RatingSystem,
}

impl PluginMarketplace {
    pub async fn publish_plugin(&self,
        plugin_id: i32,
        creator_name: &str,
        visibility: PluginVisibility
    ) -> Result<MarketplaceEntry> {
        let plugin = self.database.get_plugin(plugin_id).await?;
        
        // Validate plugin for marketplace
        self.validate_plugin_for_marketplace(&plugin)?;
        
        let marketplace_entry = MarketplaceEntry {
            plugin_id,
            creator_name: creator_name.to_string(),
            visibility,
            download_count: 0,
            rating_average: 0.0,
            rating_count: 0,
            category: plugin.category,
            tags: plugin.tags,
            featured: false,
            published_at: Utc::now(),
        };
        
        let saved_entry = self.database
            .insert_marketplace_entry(&marketplace_entry)
            .await?;
        
        // Index for search
        self.search_engine.index_plugin(&plugin, &saved_entry).await?;
        
        Ok(saved_entry)
    }
    
    pub async fn search_plugins(&self,
        query: &str,
        category: Option<&str>,
        tags: Option<&[String]>,
        sort_by: PluginSortOrder
    ) -> Result<Vec<PluginSearchResult>> {
        let search_results = self.search_engine
            .search_plugins(query, category, tags)
            .await?;
        
        // Apply sorting
        let sorted_results = match sort_by {
            PluginSortOrder::Relevance => search_results,
            PluginSortOrder::Rating => {
                let mut results = search_results;
                results.sort_by(|a, b| b.rating_average.partial_cmp(&a.rating_average).unwrap());
                results
            },
            PluginSortOrder::Downloads => {
                let mut results = search_results;
                results.sort_by(|a, b| b.download_count.cmp(&a.download_count));
                results
            },
            PluginSortOrder::Recent => {
                let mut results = search_results;
                results.sort_by(|a, b| b.published_at.cmp(&a.published_at));
                results
            },
        };
        
        Ok(sorted_results)
    }
    
    pub async fn rate_plugin(&self,
        plugin_id: i32,
        user_identifier: &str,
        rating: i32,
        review_text: Option<&str>
    ) -> Result<PluginRating> {
        // Validate rating (1-5 stars)
        if rating < 1 || rating > 5 {
            return Err(MarketplaceError::InvalidRating);
        }
        
        // Check if user already rated this plugin
        if self.database.has_user_rated_plugin(plugin_id, user_identifier).await? {
            return Err(MarketplaceError::AlreadyRated);
        }
        
        let plugin_rating = PluginRating {
            plugin_id,
            user_identifier: user_identifier.to_string(),
            rating,
            review_text: review_text.map(String::from),
            created_at: Utc::now(),
        };
        
        let saved_rating = self.database.insert_plugin_rating(&plugin_rating).await?;
        
        // Update plugin's average rating
        self.rating_system.update_plugin_rating(plugin_id).await?;
        
        Ok(saved_rating)
    }
}
```

## Canvas Implementation

### Visual Planning System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasElement {
    pub id: String,
    pub project_id: i32,
    pub element_type: CanvasElementType,
    pub position_x: f64,
    pub position_y: f64,
    pub width: f64,
    pub height: f64,
    pub content: serde_json::Value,
    pub style_data: serde_json::Value,
    pub connections: Vec<String>, // IDs of connected elements
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanvasElementType {
    Card,
    Text,
    OutlineCard,
    CharacterArc,
    PlotThread,
    Connection,
}

pub struct CanvasProcessor {
    element_manager: ElementManager,
    layout_engine: LayoutEngine,
    outline_generator: OutlineGenerator,
    template_manager: TemplateManager,
}

impl CanvasProcessor {
    pub async fn create_outline_template(&self,
        project_id: i32,
        template_type: OutlineTemplateType,
        seed_text: &str
    ) -> Result<Vec<CanvasElement>> {
        let template = self.template_manager.get_template(template_type)?;
        
        // Generate outline content from seed text
        let generated_outline = self.outline_generator
            .generate_from_seed(seed_text, &template)
            .await?;
        
        // Convert to canvas elements with template-specific positioning
        let mut elements = Vec::new();
        let positions = self.layout_engine
            .calculate_template_positions(&generated_outline, template_type);
        
        for (chapter, position) in generated_outline.chapters.iter().zip(positions.iter()) {
            let element = CanvasElement {
                id: Uuid::new_v4().to_string(),
                project_id,
                element_type: CanvasElementType::OutlineCard,
                position_x: position.x,
                position_y: position.y,
                width: 200.0,
                height: 150.0,
                content: serde_json::json!({
                    "title": chapter.title,
                    "summary": chapter.summary,
                    "chapter_number": chapter.chapter_number,
                    "template_type": template_type
                }),
                style_data: self.get_template_card_style(template_type),
                connections: Vec::new(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            elements.push(element);
        }
        
        // Add template-specific connections
        self.add_template_connections(&mut elements, template_type);
        
        Ok(elements)
    }
    
    pub async fn export_canvas_to_outline(&self,
        project_id: i32,
        canvas_elements: &[CanvasElement]
    ) -> Result<Outline> {
        // Filter outline cards
        let outline_cards: Vec<_> = canvas_elements
            .iter()
            .filter(|e| matches!(e.element_type, CanvasElementType::OutlineCard))
            .collect();
        
        // Sort by position and connections to determine order
        let sorted_cards = self.layout_engine
            .sort_elements_by_narrative_flow(&outline_cards)?;
        
        // Convert to outline chapters
        let mut chapters = Vec::new();
        for (index, card) in sorted_cards.iter().enumerate() {
            let chapter = OutlineChapter {
                chapter_number: index + 1,
                title: card.content.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled")
                    .to_string(),
                summary: card.content.get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                pov: card.content.get("pov")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                tense: card.content.get("tense")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                character_pov_ids: card.content.get("character_pov_ids")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutlineTemplateType {
    HerosJourney,
    HollywoodBeats,
    StoryCircle,
    RomanceOutline,
    ThreeAct,
    Custom,
}
```

## Database Schema Extensions

### Collaboration Tables

```sql
-- Shared Documents
CREATE TABLE shared_documents (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    share_token TEXT UNIQUE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    allow_comments BOOLEAN DEFAULT TRUE,
    allow_anonymous BOOLEAN DEFAULT TRUE,
    max_participants INTEGER,
    expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
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

-- Enhanced Comments
CREATE TABLE document_comments (
    id INTEGER PRIMARY KEY,
    document_id INTEGER NOT NULL,
    user_name TEXT,
    user_token TEXT,
    comment_text TEXT NOT NULL,
    start_position INTEGER,
    end_position INTEGER,
    is_author_comment BOOLEAN DEFAULT FALSE,
    parent_comment_id INTEGER,
    is_resolved BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents(id),
    FOREIGN KEY (parent_comment_id) REFERENCES document_comments(id)
);

-- Plugin System
CREATE TABLE plugins (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    prompt_template TEXT,
    variables JSON,
    ai_model TEXT DEFAULT 'gpt-4o-mini',
    model_parameters JSON,
    is_multi_stage BOOLEAN DEFAULT FALSE,
    stage_count INTEGER DEFAULT 1,
    is_public BOOLEAN DEFAULT FALSE,
    created_by TEXT,
    test_data JSON,
    guidelines TEXT,
    category TEXT,
    tags JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Plugin Marketplace
CREATE TABLE plugin_marketplace (
    id INTEGER PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    creator_name TEXT NOT NULL,
    visibility TEXT DEFAULT 'published',
    download_count INTEGER DEFAULT 0,
    rating_average REAL DEFAULT 0.0,
    rating_count INTEGER DEFAULT 0,
    category TEXT,
    tags JSON,
    featured BOOLEAN DEFAULT FALSE,
    published_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id)
);

-- Plugin Ratings
CREATE TABLE plugin_ratings (
    id INTEGER PRIMARY KEY,
    plugin_id INTEGER NOT NULL,
    user_identifier TEXT NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (plugin_id) REFERENCES plugins(id),
    UNIQUE(plugin_id, user_identifier)
);

-- Canvas Elements
CREATE TABLE canvas_elements (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    element_type TEXT NOT NULL,
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    width REAL,
    height REAL,
    content JSON,
    style_data JSON,
    connections JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);
```

## Frontend Components

### Plugin Builder Interface

```typescript
interface PluginBuilderProps {
  plugin?: Plugin;
  onSave: (plugin: Plugin) => void;
  onTest: (plugin: Plugin, testData: PluginTestData) => void;
}

export const PluginBuilder: React.FC<PluginBuilderProps> = ({
  plugin,
  onSave,
  onTest
}) => {
  const [mode, setMode] = useState<'basic' | 'advanced'>('basic');
  const [pluginData, setPluginData] = useState<Partial<Plugin>>(
    plugin || {
      name: '',
      description: '',
      prompt_template: '',
      variables: [],
      ai_model: 'gpt-4o-mini',
      is_multi_stage: false,
      stage_count: 1,
    }
  );

  const availableVariables = [
    'highlighted_text',
    'preceding_text', 
    'user_text_input',
    'previous_document_text',
    'braindump',
    'genre',
    'style',
    'synopsis',
    'characters',
    'characters_raw',
    'outline',
    'scene_summary',
    'is_story_bible_active',
    'chapter_scenes',
    'chapter_scenes_extra_instructions',
    'worldbuilding',
    'worldbuilding_raw'
  ];

  return (
    <div className="plugin-builder">
      <div className="plugin-builder-header">
        <button 
          className={mode === 'basic' ? 'active' : ''}
          onClick={() => setMode('basic')}
        >
          Basic Editor
        </button>
        <button 
          className={mode === 'advanced' ? 'active' : ''}
          onClick={() => setMode('advanced')}
        >
          Advanced Editor
        </button>
      </div>

      <div className="plugin-builder-content">
        <div className="plugin-metadata">
          <input
            type="text"
            placeholder="Plugin Name"
            value={pluginData.name}
            onChange={(e) => setPluginData({...pluginData, name: e.target.value})}
          />
          <textarea
            placeholder="Plugin Description"
            value={pluginData.description}
            onChange={(e) => setPluginData({...pluginData, description: e.target.value})}
          />
        </div>

        {mode === 'basic' ? (
          <BasicPluginEditor 
            pluginData={pluginData}
            onUpdate={setPluginData}
            availableVariables={availableVariables}
          />
        ) : (
          <AdvancedPluginEditor 
            pluginData={pluginData}
            onUpdate={setPluginData}
            availableVariables={availableVariables}
          />
        )}

        <div className="plugin-actions">
          <button onClick={() => handleTest()}>Test Plugin</button>
          <button onClick={() => onSave(pluginData as Plugin)}>Save Plugin</button>
        </div>
      </div>
    </div>
  );
};
```

## Success Criteria

- [ ] Document sharing with Clean Copy commenting works seamlessly
- [ ] Plugin system allows creation and execution of custom AI tools
- [ ] Plugin marketplace enables discovery and sharing of community plugins
- [ ] Canvas provides intuitive visual story planning capabilities
- [ ] Collaboration features support multiple readers with proper permissions
- [ ] Plugin sandboxing ensures security and resource management
- [ ] Visual outline templates generate appropriate story structures
- [ ] Comment threading and notifications work correctly
- [ ] Plugin testing environment validates functionality before deployment
- [ ] Canvas export creates usable outlines for Story Bible integration

## Risk Mitigation

- **Security**: Robust plugin sandboxing and input validation
- **Performance**: Efficient canvas rendering and collaboration synchronization
- **Scalability**: Optimized database queries for marketplace and comments
- **User Safety**: Content moderation and spam prevention in comments
- **Plugin Quality**: Validation and testing requirements for marketplace

## Dependencies

### Rust

- wasmtime = "15.0" # WASM runtime for plugin sandboxing
- wit-bindgen = "0.16"
- wasmtime-wasi = "15.0"
- serde_json = "1.0"
- uuid = { version = "1.0", features = ["v4"] }
- y-sync = "0.2.0" # For CRDT-based real-time sync
- webrtc = "0.7"   # For peer-to-peer collaboration

### Frontend

- reactflow = "^11.10.4"
- react-dnd = "^16.0.1"
- react-dnd-html5-backend = "^16.0.1"
- konva = "^9.2.0"
- react-konva = "^18.2.10"
- @monaco-editor/react = "^4.6.0"
- yjs = "^13.6.10" # CRDT library for collaboration
- y-webrtc = "^10.2.5"

## Integration with Core Systems

To ensure alignment with the overall architecture defined in the `StoryWeaver-MasterPlan.md` and `Enhancement-Specifications.md`, the implementation of Phase 5 will incorporate the following core systems:

### 1. Real-time Collaboration Engine

The document sharing and collaboration features will be built upon the real-time synchronization framework.

- **Conflict-Free Replicated Data Types (CRDTs)**: The implementation will use `yjs` on the frontend and a compatible Rust library on the backend to manage concurrent edits and ensure data consistency without relying on a central server for every change. This aligns with the `StateSynchronizationManager` specified in the enhancements document.
- **WebRTC for Peer-to-Peer Communication**: For smaller collaboration sessions, WebRTC will be utilized for direct peer-to-peer data synchronization, reducing server load and latency.

### 2. Credit System Integration

The plugin system's credit consumption logic must be fully integrated with the application-wide `CreditManager`.

- **Pre-generation Cost Estimation**: Before executing a plugin, the `PluginExecutionEngine` will query the `CreditManager` to provide the user with a precise cost estimate. This includes calculating input/output token costs based on the selected model and the complexity of the context variables.
- **Credit Consumption**: Upon successful execution, the engine will report the actual tokens used to the `CreditManager` to deduct the appropriate amount from the user's balance.
- **Usage Analytics**: All plugin executions will be logged with the `UsageAnalytics` system to provide users with detailed spending breakdowns.

### 3. Comprehensive Error Handling & Recovery

The collaboration and plugin systems are complex and prone to network, API, and data errors. The implementation must use the `ErrorRecoveryManager` defined in the enhancement specifications.

- **Graceful Degradation**: If real-time sync fails, the system should fall back to a "polling" or "manual refresh" mode. If a plugin's selected AI model is unavailable, it should automatically offer to switch to a compatible alternative.
- **User Feedback**: Clear, actionable error messages will be provided to the user, guiding them on how to resolve issues (e.g., "Content filtered by AI provider, try rephrasing" or "Insufficient credits to run this plugin").

### 4. Centralized State Management

The frontend components for the Plugin Builder, Marketplace, and Canvas will integrate with the `CentralizedStateManager`. This ensures that UI state is consistent, predictable, and resilient, especially when handling real-time updates from collaboration sessions or background processing from the plugin engine.

### 5. Saliency Engine for Plugin Context

The `PluginExecutionEngine`'s `VariableInjector` will not assemble context naively. It will use the `SaliencyEngine` to populate variables like `characters`, `worldbuilding`, and `outline`. This ensures that only the most relevant information from the Story Bible is passed to the AI, optimizing both the quality of the output and the credit cost of the operation.

## Next Phase

Phase 6 will focus on polish and optimization, including performance improvements, UI/UX refinements, comprehensive testing, and preparation for deployment.
