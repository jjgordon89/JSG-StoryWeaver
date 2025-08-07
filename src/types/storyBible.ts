// Story Bible Core Types
export interface StoryBible {
  id: string;
  project_id: string;
  braindump?: string;
  synopsis?: string;
  genre?: string;
  style?: string;
  style_examples?: string;
  pov_mode: string;
  global_pov?: string;
  global_tense?: string;
  global_character_pov_ids: string;
  created_at: string;
  updated_at: string;
}

export interface CharacterTrait {
  id: string;
  character_id: string;
  trait_type: string;
  content: string;
  visibility: 'always' | 'chapter' | 'never';
  series_shared: boolean;
  created_at: string;
  updated_at: string;
}

export interface WorldElement {
  id: string;
  project_id: string;
  series_id?: string;
  element_type: string;
  name: string;
  description: string;
  details?: string;
  visibility: 'always' | 'chapter' | 'never';
  series_shared: boolean;
  created_at: string;
  updated_at: string;
}

export interface Outline {
  id: string;
  project_id: string;
  chapter_number?: number;
  chapter_title?: string;
  summary: string;
  character_pov?: string;
  linked_document_id?: string;
  created_at: string;
  updated_at: string;
}

export interface OutlineAct {
  id: string;
  project_id: string;
  act_number: number;
  title: string;
  description?: string;
  start_chapter?: number;
  end_chapter?: number;
  created_at: string;
  updated_at: string;
}

export interface Scene {
  id: string;
  outline_id: string;
  scene_number: number;
  title?: string;
  summary: string;
  characters?: string;
  setting?: string;
  mood?: string;
  extra_instructions?: string;
  word_count_estimate?: number;
  credit_estimate?: number;
  is_validated: boolean;
  validation_issues?: string;
  created_at: string;
  updated_at: string;
}

// Request/Response Types for Tauri Commands
export interface CreateStoryBibleRequest {
  project_id: string;
  braindump?: string;
  synopsis?: string;
  genre?: string;
  style?: string;
  style_examples?: string;
  pov_mode?: string;
  global_pov?: string;
  global_tense?: string;
  global_character_pov_ids?: string;
}

export interface UpdateStoryBibleRequest {
  id: string;
  braindump?: string;
  synopsis?: string;
  genre?: string;
  style?: string;
  style_examples?: string;
  pov_mode?: string;
  global_pov?: string;
  global_tense?: string;
  global_character_pov_ids?: string;
}

export interface CreateCharacterTraitRequest {
  character_id: string;
  trait_type: string;
  content: string;
  visibility?: 'always' | 'chapter' | 'never';
  series_shared?: boolean;
}

export interface UpdateCharacterTraitRequest {
  id: string;
  trait_type?: string;
  content?: string;
  visibility?: 'always' | 'chapter' | 'never';
  series_shared?: boolean;
}

export interface CreateWorldElementRequest {
  project_id: string;
  series_id?: string;
  element_type: string;
  name: string;
  description: string;
  details?: string;
  visibility?: 'always' | 'chapter' | 'never';
  series_shared?: boolean;
}

export interface UpdateWorldElementRequest {
  id: string;
  element_type?: string;
  name?: string;
  description?: string;
  details?: string;
  visibility?: 'always' | 'chapter' | 'never';
  series_shared?: boolean;
}

export interface CreateOutlineRequest {
  project_id: string;
  chapter_number?: number;
  chapter_title?: string;
  summary: string;
  character_pov?: string;
  linked_document_id?: string;
}

export interface UpdateOutlineRequest {
  id: string;
  chapter_number?: number;
  chapter_title?: string;
  summary?: string;
  character_pov?: string;
  linked_document_id?: string;
}

export interface CreateSceneRequest {
  outline_id: string;
  scene_number: number;
  title?: string;
  summary: string;
  characters?: string;
  setting?: string;
  mood?: string;
  extra_instructions?: string;
  word_count_estimate?: number;
  credit_estimate?: number;
}

export interface UpdateSceneRequest {
  id: string;
  scene_number?: number;
  title?: string;
  summary?: string;
  characters?: string;
  setting?: string;
  mood?: string;
  extra_instructions?: string;
  word_count_estimate?: number;
  credit_estimate?: number;
}

export interface SearchWorldElementsRequest {
  project_id: string;
  query: string;
  element_type?: string;
  visibility?: 'always' | 'chapter' | 'never';
}

export interface SearchOutlinesRequest {
  project_id: string;
  query: string;
  character_pov?: string;
}

export interface SearchScenesRequest {
  outline_id: string;
  query: string;
}

export interface ValidateSceneRequest {
  id: string;
}

// UI State Types
export interface StoryBibleState {
  // Data
  storyBible: StoryBible | null;
  characterTraits: CharacterTrait[];
  worldElements: WorldElement[];
  outlines: Outline[];
  outlineActs: OutlineAct[];
  scenes: Scene[];
  
  // Loading states
  isLoading: boolean;
  isLoadingTraits: boolean;
  isLoadingWorldElements: boolean;
  isLoadingOutlines: boolean;
  isLoadingScenes: boolean;
  
  // Error states
  error: string | null;
  traitsError: string | null;
  worldElementsError: string | null;
  outlinesError: string | null;
  scenesError: string | null;
  
  // UI state
  activeTab: 'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes';
  selectedCharacterId: string | null;
  selectedOutlineId: string | null;
  
  // Filters
  characterTraitFilter: {
    visibility?: 'always' | 'chapter' | 'never';
    traitType?: string;
  };
  worldElementFilter: {
    elementType?: string;
    visibility?: 'always' | 'chapter' | 'never';
    seriesShared?: boolean;
  };
  outlineFilter: {
    characterPov?: string;
  };
}

// Hook Return Types
export interface UseStoryBibleReturn {
  // Data
  storyBible: StoryBible | null;
  characterTraits: CharacterTrait[];
  worldElements: WorldElement[];
  outlines: Outline[];
  scenes: Scene[];
  
  // Loading states
  isLoading: boolean;
  error: string | null;
  
  // Actions
  createOrUpdateStoryBible: (request: CreateStoryBibleRequest | UpdateStoryBibleRequest) => Promise<void>;
  loadStoryBible: (projectId: string) => Promise<void>;
  
  // Character traits
  createCharacterTrait: (request: CreateCharacterTraitRequest) => Promise<void>;
  updateCharacterTrait: (request: UpdateCharacterTraitRequest) => Promise<void>;
  deleteCharacterTrait: (id: string) => Promise<void>;
  loadCharacterTraits: (characterId: string) => Promise<void>;
  
  // World elements
  createWorldElement: (request: CreateWorldElementRequest) => Promise<void>;
  updateWorldElement: (request: UpdateWorldElementRequest) => Promise<void>;
  deleteWorldElement: (id: string) => Promise<void>;
  loadWorldElements: (projectId: string) => Promise<void>;
  searchWorldElements: (request: SearchWorldElementsRequest) => Promise<WorldElement[]>;
  
  // Outlines
  createOutline: (request: CreateOutlineRequest) => Promise<void>;
  updateOutline: (request: UpdateOutlineRequest) => Promise<void>;
  deleteOutline: (id: string) => Promise<void>;
  loadOutlines: (projectId: string) => Promise<void>;
  searchOutlines: (request: SearchOutlinesRequest) => Promise<Outline[]>;
  
  // Scenes
  createScene: (request: CreateSceneRequest) => Promise<void>;
  updateScene: (request: UpdateSceneRequest) => Promise<void>;
  deleteScene: (id: string) => Promise<void>;
  validateScene: (id: string) => Promise<void>;
  loadScenes: (outlineId: string) => Promise<void>;
  searchScenes: (request: SearchScenesRequest) => Promise<Scene[]>;
  
  // Utility
  clearError: () => void;
  setActiveTab: (tab: 'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes') => void;
}

// Component Props Types
export interface StoryBibleProps {
  projectId: string;
  seriesId?: string;
}

export interface BraindumpEditorProps {
  projectId: string;
  content?: string;
  onUpdate: (content: string) => void;
}

export interface CharactersManagerProps {
  projectId: string;
  seriesId?: string;
  characterId?: string;
}

export interface WorldbuildingManagerProps {
  projectId: string;
  seriesId?: string;
}

export interface OutlineManagerProps {
  projectId: string;
}

export interface ScenesManagerProps {
  outlineId: string;
}

// Validation Types
export interface ValidationIssue {
  field: string;
  message: string;
  severity: 'error' | 'warning' | 'info';
}

export interface SceneValidationResult {
  isValid: boolean;
  issues: ValidationIssue[];
  suggestions?: string[];
}

// Export Types
export interface ExportOptions {
  format: 'csv' | 'json' | 'markdown';
  includeCharacterTraits: boolean;
  includeWorldElements: boolean;
  includeOutlines: boolean;
  includeScenes: boolean;
  filterByVisibility?: 'always' | 'chapter' | 'never';
}

// AI Integration Types
export interface StoryBibleAIContext {
  braindump?: string;
  synopsis?: string;
  genre?: string;
  style?: string;
  characters: CharacterTrait[];
  worldElements: WorldElement[];
  currentOutline?: Outline;
  relevantScenes: Scene[];
}

export interface AIGenerationRequest {
  type: 'synopsis' | 'character_trait' | 'world_element' | 'outline' | 'scene';
  context: StoryBibleAIContext;
  prompt?: string;
  settings?: {
    creativity: number;
    length: 'short' | 'medium' | 'long';
    style?: string;
  };
}