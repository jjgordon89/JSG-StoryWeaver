# Phase 4: Advanced AI Features (Weeks 16-19)

## Overview
Implement advanced AI capabilities including multiple AI models, prose modes, specialized tools like Visualize and advanced brainstorming, chapter continuity, style examples, and comprehensive credit management.

## Key Objectives
- Multiple AI models and prose modes (Muse, Excellent, Basic, Experimental)
- Saliency Engine implementation for intelligent context selection
- Chapter continuity with document linking awareness
- Style examples and Match My Style functionality
- Credit tracking system with transparent usage monitoring
- Visualize feature for AI-generated images
- Advanced brainstorming with Keepers List and voting
- Smart import with character extraction (120K words, 30 characters)
- Streaming generation with pause/resume capabilities

## Technical Tasks

### Week 16: Multiple AI Models & Prose Modes
- [ ] Implement AI model configuration system
- [ ] Create Muse prose mode with advanced features:
  - Creativity Level 11 (ultra-creative mode)
  - Up to 10,000 words generation in Draft
  - 128,000 words context reading capability
  - Cliché detection and removal system
  - Unfiltered content generation
- [ ] Add Excellent and Basic prose modes
- [ ] Implement experimental model support (GPT-4.1, Claude 3.5, DeepSeek, Gemini)
- [ ] Create model-specific optimizations and configurations
- [ ] Add content filter levels based on selected model
- [ ] Build prose mode selection dropdown interface

### Week 17: Saliency Engine & Context Management
- [ ] Implement Saliency Engine for intelligent Story Bible selection
- [ ] Create relevance scoring algorithms for characters and worldbuilding
- [ ] Build context window management for different AI models
- [ ] Add raw data alternatives for Story Bible elements
- [ ] Implement intelligent context truncation and prioritization
- [ ] Create context assembly pipeline with optimization
- [ ] Add Story Bible element detection and highlighting
- [ ] Build context debugging and visualization tools

### Week 18: Style Examples & Advanced Features
- [ ] Implement Style Examples system (up to 1,000 words)
- [ ] Create Match My Style AI analysis functionality
- [ ] Build style prompt generation from user writing samples
- [ ] Add chapter continuity with document linking awareness
- [ ] Implement streaming generation with pause/resume
- [ ] Create progress indicators and cancellation capabilities
- [ ] Add token management and optimization
- [ ] Build model switching based on task requirements

### Week 19: Visualize & Advanced Tools
- [ ] Implement Visualize feature for AI-generated images
- [ ] Add DALL-E 3 integration with content filtering
- [ ] Create image processing and optimization
- [ ] Build advanced brainstorming with category-specific prompts
- [ ] Implement Keepers List with thumbs up/down voting
- [ ] Add brainstorm session management and persistence
- [ ] Create refresh functionality while maintaining keepers
- [ ] Build smart import with novel analysis (120K words)

## AI Model Architecture

### Prose Mode System
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProseMode {
    pub id: String,
    pub name: String,
    pub description: String,
    pub model_configuration_id: i32,
    pub creativity_level: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub special_instructions: Option<String>,
    pub is_experimental: bool,
    pub max_context_words: usize,
    pub max_generation_words: usize,
    pub supports_streaming: bool,
    pub supports_unfiltered: bool,
}

pub struct ProseModelManager {
    modes: HashMap<String, ProseMode>,
    model_configs: HashMap<i32, AIModelConfiguration>,
    active_mode: Option<String>,
}

impl ProseModelManager {
    pub fn new() -> Self {
        let mut manager = Self {
            modes: HashMap::new(),
            model_configs: HashMap::new(),
            active_mode: None,
        };
        
        // Initialize default prose modes
        manager.add_muse_mode();
        manager.add_excellent_mode();
        manager.add_basic_mode();
        
        manager
    }
    
    fn add_muse_mode(&mut self) {
        let muse_mode = ProseMode {
            id: "muse".to_string(),
            name: "Muse".to_string(),
            description: "Advanced creative writing with cliché detection and ultra-creative mode".to_string(),
            model_configuration_id: 1,
            creativity_level: 7,
            temperature: 0.8,
            top_p: 0.9,
            frequency_penalty: 0.1,
            presence_penalty: 0.1,
            special_instructions: Some("Focus on creative, original prose. Avoid clichés and overused phrases.".to_string()),
            is_experimental: false,
            max_context_words: 128_000,
            max_generation_words: 10_000,
            supports_streaming: true,
            supports_unfiltered: true,
        };
        
        self.modes.insert("muse".to_string(), muse_mode);
    }
    
    pub async fn generate_with_mode(&self, 
        mode_id: &str, 
        prompt: &str, 
        context: &AIContext
    ) -> Result<GenerationResult> {
        let mode = self.modes.get(mode_id)
            .ok_or_else(|| anyhow::anyhow!("Prose mode not found: {}", mode_id))?;
        
        let model_config = self.model_configs.get(&mode.model_configuration_id)
            .ok_or_else(|| anyhow::anyhow!("Model configuration not found"))?;
        
        // Apply mode-specific settings
        let generation_params = GenerationParams {
            temperature: mode.temperature,
            top_p: mode.top_p,
            frequency_penalty: mode.frequency_penalty,
            presence_penalty: mode.presence_penalty,
            max_tokens: mode.max_generation_words,
            special_instructions: mode.special_instructions.clone(),
        };
        
        // Handle ultra-creative mode (Level 11)
        if mode.creativity_level == 11 {
            return self.generate_ultra_creative(prompt, context, &generation_params).await;
        }
        
        // Standard generation
        self.generate_standard(prompt, context, &generation_params).await
    }
    
    async fn generate_ultra_creative(&self, 
        prompt: &str, 
        context: &AIContext, 
        params: &GenerationParams
    ) -> Result<GenerationResult> {
        // Ultra-creative mode uses different algorithms
        let enhanced_prompt = format!(
            "{}\n\nInstructions: Be exceptionally creative and original. Avoid any clichés, \
            overused phrases, or predictable plot elements. Push boundaries and explore \
            unique perspectives and unexpected directions.",
            prompt
        );
        
        // Use enhanced parameters for maximum creativity
        let ultra_params = GenerationParams {
            temperature: 0.95,
            top_p: 0.95,
            frequency_penalty: 0.3,
            presence_penalty: 0.3,
            ..params.clone()
        };
        
        self.generate_with_cliche_detection(&enhanced_prompt, context, &ultra_params).await
    }
}
```

### Saliency Engine Implementation
```rust
pub struct SaliencyEngine {
    embedding_service: Arc<EmbeddingService>,
    relevance_calculator: RelevanceCalculator,
    context_optimizer: ContextOptimizer,
}

impl SaliencyEngine {
    pub async fn build_intelligent_context(&self,
        current_text: &str,
        story_bible: &StoryBible,
        max_context_tokens: usize
    ) -> Result<IntelligentContext> {
        // Generate embedding for current context
        let context_embedding = self.embedding_service
            .generate_embedding(current_text)
            .await?;
        
        // Score all Story Bible elements for relevance
        let mut scored_elements = Vec::new();
        
        // Score characters
        for character in &story_bible.characters {
            if character.is_visible {
                let relevance_score = self.calculate_character_relevance(
                    &context_embedding, 
                    character, 
                    current_text
                ).await?;
                
                scored_elements.push(ScoredElement {
                    element: StoryBibleElement::Character(character.clone()),
                    relevance_score,
                    token_cost: self.estimate_token_cost(character),
                });
            }
        }
        
        // Score worldbuilding elements
        for element in &story_bible.worldbuilding {
            if element.is_visible {
                let relevance_score = self.calculate_worldbuilding_relevance(
                    &context_embedding, 
                    element, 
                    current_text
                ).await?;
                
                scored_elements.push(ScoredElement {
                    element: StoryBibleElement::Worldbuilding(element.clone()),
                    relevance_score,
                    token_cost: self.estimate_token_cost(element),
                });
            }
        }
        
        // Optimize selection based on relevance and token budget
        let selected_elements = self.context_optimizer
            .optimize_selection(scored_elements, max_context_tokens)
            .await?;
        
        Ok(IntelligentContext {
            selected_elements,
            total_tokens: selected_elements.iter().map(|e| e.token_cost).sum(),
            relevance_summary: self.generate_relevance_summary(&selected_elements),
        })
    }
    
    async fn calculate_character_relevance(&self,
        context_embedding: &Vec<f32>,
        character: &Character,
        current_text: &str
    ) -> Result<f32> {
        let mut relevance_score = 0.0;
        
        // Name mention frequency
        let name_mentions = current_text.matches(&character.name).count() as f32;
        relevance_score += name_mentions * 0.3;
        
        // Semantic similarity of character description
        if let Some(description) = &character.description {
            let char_embedding = self.embedding_service
                .generate_embedding(description)
                .await?;
            let similarity = self.calculate_cosine_similarity(context_embedding, &char_embedding);
            relevance_score += similarity * 0.4;
        }
        
        // Trait relevance
        for (trait_name, trait_value) in &character.traits {
            let trait_text = format!("{}: {}", trait_name, trait_value);
            let trait_embedding = self.embedding_service
                .generate_embedding(&trait_text)
                .await?;
            let similarity = self.calculate_cosine_similarity(context_embedding, &trait_embedding);
            relevance_score += similarity * 0.2;
        }
        
        // Recent usage boost
        let recent_usage = self.get_recent_character_usage(character.id).await?;
        relevance_score += recent_usage * 0.1;
        
        Ok(relevance_score)
    }
}
```

## Visualize Feature Implementation

### Image Generation System
```rust
pub struct VisualizeProcessor {
    dalle_client: DalleClient,
    image_optimizer: ImageOptimizer,
    content_filter: ContentFilter,
    credit_manager: Arc<CreditManager>,
}

impl VisualizeProcessor {
    pub async fn generate_image(&self,
        project_id: i32,
        source_text: &str,
        user_prompt: Option<&str>
    ) -> Result<VisualizeResult> {
        // Validate text length (10-3000 words)
        let word_count = self.count_words(source_text);
        if word_count < 10 || word_count > 3000 {
            return Err(VisualizeError::InvalidTextLength);
        }
        
        // Check credit balance (2500 credits required)
        let credits_required = 2500;
        self.credit_manager.check_balance(credits_required).await?;
        
        // Generate image prompt from text
        let image_prompt = if let Some(prompt) = user_prompt {
            prompt.to_string()
        } else {
            self.generate_prompt_from_text(source_text).await?
        };
        
        // Apply content filtering
        let filtered_prompt = self.content_filter
            .filter_image_prompt(&image_prompt)
            .await?;
        
        // Generate image using DALL-E 3
        let image_data = self.dalle_client
            .generate_image(&filtered_prompt, "1024x1024")
            .await?;
        
        // Optimize and process image
        let optimized_image = self.image_optimizer
            .optimize_for_display(&image_data)
            .await?;
        
        // Consume credits
        self.credit_manager
            .consume_credits(project_id, "visualize", credits_required)
            .await?;
        
        // Save to database
        let generated_image = GeneratedImage {
            project_id,
            source_text: source_text.to_string(),
            image_prompt: filtered_prompt,
            image_data: optimized_image,
            credits_used: credits_required,
            resolution: "1024x1024".to_string(),
            created_at: Utc::now(),
        };
        
        self.save_generated_image(&generated_image).await?;
        
        Ok(VisualizeResult {
            image_data: generated_image.image_data,
            prompt_used: generated_image.image_prompt,
            credits_used: credits_required,
            resolution: "1024x1024".to_string(),
        })
    }
    
    async fn generate_prompt_from_text(&self, text: &str) -> Result<String> {
        let prompt_generation_request = format!(
            "Create a detailed visual description for an image based on this text. \
            Focus on the most vivid and important visual elements, characters, setting, \
            and atmosphere. Make it suitable for AI image generation:\n\n{}",
            text
        );
        
        // Use AI to generate image prompt
        let ai_response = self.ai_provider
            .generate_text(&prompt_generation_request, &AIContext::default())
            .await?;
        
        Ok(ai_response)
    }
}
```

## Advanced Brainstorming System

### Brainstorm Manager
```rust
pub struct AdvancedBrainstormManager {
    ai_provider: Arc<dyn AIProvider>,
    session_store: Arc<dyn SessionStore>,
    category_prompts: HashMap<String, String>,
}

impl AdvancedBrainstormManager {
    pub fn new(ai_provider: Arc<dyn AIProvider>, session_store: Arc<dyn SessionStore>) -> Self {
        let mut manager = Self {
            ai_provider,
            session_store,
            category_prompts: HashMap::new(),
        };
        
        manager.initialize_category_prompts();
        manager
    }
    
    fn initialize_category_prompts(&mut self) {
        self.category_prompts.insert(
            "Dialogue".to_string(),
            "Generate compelling dialogue options that reveal character, advance plot, or create tension.".to_string()
        );
        self.category_prompts.insert(
            "Characters".to_string(),
            "Create interesting character concepts, traits, backgrounds, or relationships.".to_string()
        );
        self.category_prompts.insert(
            "World building".to_string(),
            "Develop world elements like locations, cultures, history, or unique aspects of the setting.".to_string()
        );
        self.category_prompts.insert(
            "Plot points".to_string(),
            "Suggest plot developments, twists, conflicts, or story directions.".to_string()
        );
        // Add more categories...
    }
    
    pub async fn start_brainstorm_session(&self,
        project_id: i32,
        category: &str,
        seed_prompt: Option<&str>,
        story_context: Option<&StoryBible>
    ) -> Result<BrainstormSession> {
        let session_id = Uuid::new_v4().to_string();
        
        // Build context-aware prompt
        let base_prompt = self.category_prompts.get(category)
            .ok_or_else(|| anyhow::anyhow!("Unknown brainstorm category: {}", category))?;
        
        let context_prompt = if let Some(context) = story_context {
            format!(
                "{}\n\nStory context:\nGenre: {}\nSynopsis: {}\n\nSeed prompt: {}",
                base_prompt,
                context.genre.as_deref().unwrap_or("Unknown"),
                context.synopsis.as_deref().unwrap_or("No synopsis"),
                seed_prompt.unwrap_or("Generate creative ideas")
            )
        } else {
            format!("{}\n\nSeed prompt: {}", base_prompt, seed_prompt.unwrap_or("Generate creative ideas"))
        };
        
        // Generate initial ideas
        let ideas = self.generate_category_ideas(&context_prompt, 8).await?;
        
        let session = BrainstormSession {
            id: session_id.clone(),
            project_id,
            category: category.to_string(),
            seed_prompt: seed_prompt.map(String::from),
            ideas,
            keepers_list: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.session_store.save_session(&session).await?;
        Ok(session)
    }
    
    pub async fn vote_on_idea(&self,
        session_id: &str,
        idea_id: &str,
        vote: BrainstormVote
    ) -> Result<BrainstormSession> {
        let mut session = self.session_store.get_session(session_id).await?;
        
        if let Some(idea) = session.ideas.iter_mut().find(|i| i.id == idea_id) {
            match vote {
                BrainstormVote::ThumbsUp => {
                    idea.thumbs_up += 1;
                    // Add to keepers list if not already there
                    if !session.keepers_list.iter().any(|k| k.id == idea_id) {
                        session.keepers_list.push(idea.clone());
                    }
                },
                BrainstormVote::ThumbsDown => {
                    idea.thumbs_down += 1;
                    // Remove from keepers list
                    session.keepers_list.retain(|k| k.id != idea_id);
                }
            }
        }
        
        session.updated_at = Utc::now();
        self.session_store.save_session(&session).await?;
        Ok(session)
    }
    
    pub async fn refresh_ideas(&self, 
        session_id: &str, 
        keep_keepers: bool
    ) -> Result<BrainstormSession> {
        let mut session = self.session_store.get_session(session_id).await?;
        
        // Generate new ideas
        let base_prompt = self.category_prompts.get(&session.category)
            .ok_or_else(|| anyhow::anyhow!("Unknown category"))?;
        
        let refresh_prompt = format!(
            "{}\n\nGenerate fresh, different ideas. Avoid these previous suggestions: {}",
            base_prompt,
            session.ideas.iter().map(|i| &i.content).collect::<Vec<_>>().join(", ")
        );
        
        let new_ideas = self.generate_category_ideas(&refresh_prompt, 8).await?;
        
        if keep_keepers {
            // Merge new ideas with existing keepers
            let mut all_ideas = session.keepers_list.clone();
            all_ideas.extend(new_ideas);
            session.ideas = all_ideas;
        } else {
            session.ideas = new_ideas;
            session.keepers_list.clear();
        }
        
        session.updated_at = Utc::now();
        self.session_store.save_session(&session).await?;
        Ok(session)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrainstormVote {
    ThumbsUp,
    ThumbsDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainstormIdea {
    pub id: String,
    pub content: String,
    pub thumbs_up: i32,
    pub thumbs_down: i32,
    pub created_at: DateTime<Utc>,
}
```

## Database Schema Extensions

### Advanced AI Features
```sql
-- AI Model Configurations
CREATE TABLE ai_model_configurations (
    id INTEGER PRIMARY KEY,
    provider_id INTEGER NOT NULL,
    model_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    context_window INTEGER NOT NULL,
    max_output_tokens INTEGER NOT NULL,
    supports_streaming BOOLEAN DEFAULT TRUE,
    supports_images BOOLEAN DEFAULT FALSE,
    cost_per_input_token REAL,
    cost_per_output_token REAL,
    cost_per_image REAL,
    quality_tier TEXT DEFAULT 'standard',
    specializations JSON,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (provider_id) REFERENCES ai_providers(id)
);

-- Prose Modes
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
    max_context_words INTEGER DEFAULT 4000,
    max_generation_words INTEGER DEFAULT 2000,
    supports_streaming BOOLEAN DEFAULT TRUE,
    supports_unfiltered BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (model_configuration_id) REFERENCES ai_model_configurations(id)
);

-- Style Examples
CREATE TABLE style_examples (
    id INTEGER PRIMARY KEY,
    project_id INTEGER,
    user_id TEXT,
    example_text TEXT NOT NULL,
    analysis_result TEXT,
    generated_style_prompt TEXT,
    word_count INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Generated Images
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

-- Brainstorm Sessions
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

-- Credit Usage Tracking
CREATE TABLE credit_usage (
    id INTEGER PRIMARY KEY,
    project_id INTEGER NOT NULL,
    feature_name TEXT NOT NULL,
    model_used TEXT,
    credits_used INTEGER,
    tokens_input INTEGER,
    tokens_output INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Streaming Sessions
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
```

## Success Criteria
- [ ] Multiple AI models (Muse, Excellent, Basic, Experimental) work correctly
- [ ] Prose mode selection affects generation quality and style appropriately
- [ ] Saliency Engine intelligently selects relevant Story Bible elements
- [ ] Style Examples system analyzes user writing and generates style prompts
- [ ] Chapter continuity provides seamless context across linked documents
- [ ] Credit tracking accurately monitors usage with transparent cost display
- [ ] Visualize feature generates appropriate images from text descriptions
- [ ] Advanced brainstorming with voting and Keepers List functions properly
- [ ] Smart import extracts characters and Story Bible data from large texts
- [ ] Streaming generation with pause/resume works reliably

## Risk Mitigation
- **Model Complexity**: Robust configuration management and fallback systems
- **Credit Management**: Accurate cost calculation and balance monitoring
- **Image Generation**: Content filtering and appropriate usage policies
- **Context Optimization**: Efficient algorithms for large Story Bible data
- **Streaming Reliability**: Error handling and session recovery mechanisms

## Dependencies
### Rust
- image = { version = "0.24", features = ["png", "jpeg"] }
- base64 = "0.21"
- lancedb = "0.4"
- tiktoken-rs = "0.5"
- tokio-stream = "0.1"

### Frontend
- react-image-crop = "^11.0.0"
- canvas = "^2.11.0"
- @tanstack/react-virtual = "^3.0.0"
- framer-motion = "^10.16.0"

## Next Phase
Phase 5 will focus on collaboration features and the plugin system, building upon the advanced AI capabilities established in this phase.
