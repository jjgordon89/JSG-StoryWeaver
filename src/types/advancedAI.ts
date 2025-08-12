// Advanced AI Features Types for Phase 4

export interface ProseMode {
  name: string;
  description: string;
  model_config: AIModelConfiguration;
  generation_settings: GenerationSettings;
  is_premium: boolean;
  credits_multiplier: number;
}

export interface AIModelConfiguration {
  provider: string;
  model: string;
  temperature: number;
  max_tokens: number;
  top_p: number;
  frequency_penalty: number;
  presence_penalty: number;
}

export interface GenerationSettings {
  max_words: number;
  creativity_boost: number;
  style_consistency: number;
  cliche_avoidance: number;
  special_instructions?: string;
}

export interface ProseGenerationRequest {
  project_id: string;
  document_id?: string;
  prose_mode: string;
  text_context: string;
  generation_type: string;
  max_words?: number;
  ultra_creative: boolean;
  use_saliency_engine: boolean;
  style_examples: string[];
  special_instructions?: string;
  story_bible?: StoryBibleElements;
}

export interface AdvancedGenerationResult {
  generated_text: string;
  prose_mode_used: string;
  saliency_context?: SaliencyContext;
  cliche_detection?: ClicheDetectionResult;
  token_count: number;
  credits_used: number;
  generation_id: string;
}

export interface ClicheDetectionResult {
  cliches_found: ClicheMatch[];
  overall_score: number;
  suggestions: string[];
}

export interface ClicheMatch {
  text: string;
  start_pos: number;
  end_pos: number;
  severity: 'low' | 'medium' | 'high';
  suggestion?: string;
}

export interface SaliencyContext {
  project_id: string;
  context_hash: string;
  selected_elements: SelectedElements;
  relevance_scores: Record<string, number>;
  token_count: number;
  created_at: string;
}

export interface SelectedElements {
  characters: Character[];
  locations: Location[];
  plot_threads: PlotThread[];
  worldbuilding: WorldbuildingElement[];
}

export interface Character {
  id: string;
  name: string;
  description: string;
  traits: string[];
  relationships: Record<string, string>;
}

export interface Location {
  id: string;
  name: string;
  description: string;
  atmosphere: string;
  significance: string;
}

export interface PlotThread {
  id: string;
  title: string;
  description: string;
  status: string;
  importance: number;
}

export interface WorldbuildingElement {
  id: string;
  name: string;
  type: string;
  description: string;
  rules: string[];
}

export interface StoryBibleElements {
  characters: Character[];
  locations: Location[];
  plot_threads: PlotThread[];
  worldbuilding: WorldbuildingElement[];
}

// Image Generation Types
export interface ImageGenerationRequest {
  project_id: string;
  document_id?: string;
  text_content: string;
  style_preference: string;
  resolution: string;
  enhance_prompt: boolean;
  custom_prompt?: string;
}

export interface GeneratedImage {
  id: string;
  project_id: string;
  document_id?: string;
  image_prompt: string;
  enhanced_prompt: string;
  style_preference: string;
  resolution: ImageResolution;
  image_url?: string;
  image_data?: string;
  credits_used: number;
  created_at: string;
  metadata: Record<string, any>;
}

export interface ImageResolution {
  width: number;
  height: number;
  name: string;
}

// Brainstorming Types
export interface BrainstormSessionRequest {
  project_id: string;
  category: string;
  focus_area: string;
  num_ideas: number;
  creativity_level: number;
  context: string;
  constraints: string[];
}

export interface BrainstormSession {
  id: string;
  project_id: string;
  category: string;
  focus_area: string;
  ideas: BrainstormIdea[];
  created_at: string;
  updated_at: string;
  status: 'active' | 'completed' | 'archived';
}

export interface BrainstormIdea {
  id: string;
  content: string;
  tags: string[];
  rating?: number;
  is_keeper: boolean;
  notes?: string;
  created_at: string;
}

// Style Analysis Types
export interface StyleExampleRequest {
  project_id: string;
  name: string;
  content: string;
  is_active: boolean;
}

export interface StyleExample {
  id: string;
  project_id: string;
  name: string;
  content: string;
  word_count: number;
  analysis_result?: StyleAnalysis;
  is_active: boolean;
}

export interface StyleAnalysis {
  sentence_length_avg: number;
  vocabulary_complexity: number;
  dialogue_ratio: number;
  description_ratio: number;
  action_ratio: number;
  tone_indicators: string[];
  common_phrases: string[];
}

// Credit Management Types
export interface CreditUsageResponse {
  project_usage: number;
  daily_usage: number;
  monthly_limit?: number;
  remaining_credits?: number;
}

export interface CreditUsage {
  operation_type: string;
  credits_used: number;
  cost_estimate?: number;
  provider: string;
  model: string;
  details: Record<string, any>;
}

// Streaming Types
export interface StreamingGenerationRequest {
  request: ProseGenerationRequest;
  stream_id?: string;
}

export interface StreamingStatus {
  status: 'pending' | 'generating' | 'completed' | 'error';
  progress: number;
  current_text?: string;
  error_message?: string;
  estimated_completion?: string;
  stream_id?: string;
}

// Smart Import Types
export interface SmartImportRequest {
  project_id: string;
  content: string;
  content_type: string;
}

export interface SmartImportResult {
  status: string;
  suggestions: ImportSuggestion[];
  extracted_elements: {
    characters?: ExtractedCharacter[];
    locations?: ExtractedLocation[];
    plot_points?: string[];
    themes?: string[];
  };
}

export interface ExtractedCharacter {
  name: string;
  description: string;
  traits: string[];
  relationships: string[];
  confidence?: number;
}

export interface ExtractedLocation {
  name: string;
  description: string;
  atmosphere?: string;
  significance?: string;
  confidence?: number;
}

export interface ImportSuggestion {
  type: 'character' | 'location' | 'plot_thread' | 'worldbuilding';
  name: string;
  description: string;
  confidence: number;
  auto_apply: boolean;
}

// Visualize Feature Types
export interface VisualizeRequest {
  project_id: string;
  document_id?: string;
  text_content: string;
  style_preference: string;
  resolution: ImageResolution;
  enhance_prompt: boolean;
  custom_prompt?: string;
}

export interface VisualMoment {
  text_snippet: string;
  start_position: number;
  end_position: number;
  visual_elements: string[];
  importance_score: number;
  suggested_prompt: string;
}

// Advanced AI Store State
export interface AdvancedAISettings {
  general: {
    ultraCreativeMode: boolean;
    autoEnhancePrompts: boolean;
    clicheDetectionEnabled: boolean;
    saliencyEnabled: boolean;
    defaultProseMode: string;
    autoSave: boolean;
    showAdvancedOptions: boolean;
    enableStreaming: boolean;
  };
  generation: {
    defaultContextLength: number;
    defaultOutputLength: number;
    creativityLevel: number;
    maxWords: number;
    creativityBoost: number;
    styleConsistency: number;
    clicheAvoidance: number;
  };
  saliencyEngine: {
    enabled: boolean;
    autoBuild: boolean;
    refreshInterval: number;
    includedElements: string[];
  };
  imageGeneration: {
    qualityLevel: string;
    enablePromptEnhancement: boolean;
    useStoryContext: boolean;
    autoSaveImages: boolean;
  };
  brainstorming: {
    defaultSessionDuration: number;
    ideasPerGeneration: number;
  };
}

export interface AdvancedAIState {
  // Prose Modes
  availableProseModes: ProseMode[];
  currentProseMode: string;
  
  // Generation State
  isGenerating: boolean;
  lastGenerationResult?: AdvancedGenerationResult;
  streamingStatus?: StreamingStatus;
  activeStreamId?: string;
  generationStartedAt?: string;
  generationFinishedAt?: string;
  streamingPollIntervalId?: number;
  lastGenerationRequest?: ProseGenerationRequest;
  
  // Brainstorming
  activeBrainstormSessions: BrainstormSession[];
  currentBrainstormSession?: string;
  
  // Images
  generatedImages: GeneratedImage[];
  isGeneratingImage: boolean;
  
  // Style Examples
  styleExamples: StyleExample[];
  activeStyleExamples: string[];
  
  // Credits
  creditUsage: CreditUsageResponse;
  
  // Saliency Engine
  saliencyEnabled: boolean;
  lastSaliencyContext?: SaliencyContext;
  
  // Settings
  settings: AdvancedAISettings;
  ultraCreativeMode: boolean;
  autoEnhancePrompts: boolean;
  clicheDetectionEnabled: boolean;
}

// API Response Types
export interface AdvancedAIResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

// Advanced AI Actions
export interface AdvancedAIActions {
  // Prose Generation
  generateWithProseMode: (request: ProseGenerationRequest) => Promise<AdvancedGenerationResult>;
  startStreamingGeneration: (request: ProseGenerationRequest) => Promise<string>;
  getStreamStatus: (streamId: string) => Promise<StreamingStatus>;
  
  // Image Generation
  generateImage: (request: ImageGenerationRequest) => Promise<GeneratedImage>;
  getProjectImages: (projectId: string) => Promise<GeneratedImage[]>;
  deleteGeneratedImage: (imageId: string) => Promise<void>;
  
  // Brainstorming
  createBrainstormSession: (request: BrainstormSessionRequest) => Promise<string>;
  getBrainstormSession: (sessionId: string) => Promise<BrainstormSession | null>;
  rateIdea: (sessionId: string, ideaId: string, rating: number) => Promise<void>;
  markIdeaAsKeeper: (sessionId: string, ideaId: string, isKeeper: boolean) => Promise<void>;
  
  // Style Management
  addStyleExample: (request: StyleExampleRequest) => Promise<StyleExample>;
  analyzeTextStyle: (content: string) => Promise<StyleAnalysis>;
  
  // Prose Modes
  getAvailableProseModes: () => Promise<ProseMode[]>;
  getProseMode: (modeName: string) => Promise<ProseMode | null>;
  
  // Credits
  getCreditUsage: (projectId: string) => Promise<CreditUsageResponse>;
  
  // Saliency Engine
  buildSaliencyContext: (projectId: string, textContext: string, storyBible: StoryBibleElements) => Promise<SaliencyContext>;
  
  // Smart Import
  smartImportContent: (request: SmartImportRequest) => Promise<SmartImportResult>;
}
