import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type {
  StoryBible,
  CharacterTrait,
  WorldElement,
  Outline,
  OutlineAct,
  Scene,
  StoryBibleState,
  CreateStoryBibleRequest,
  UpdateStoryBibleRequest,
  CreateCharacterTraitRequest,
  UpdateCharacterTraitRequest,
  CreateWorldElementRequest,
  UpdateWorldElementRequest,
  CreateOutlineRequest,
  UpdateOutlineRequest,
  CreateSceneRequest,
  UpdateSceneRequest,
  GenerateSynopsisRequest,
  GenerateCharacterTraitsRequest,
  GenerateWorldElementRequest,
  AIGenerationResponse,
  SearchWorldElementsRequest,
  SearchOutlinesRequest,
  SearchScenesRequest
} from '../types/storyBible';

// Initial state
const initialState: StoryBibleState = {
  // Data
  storyBible: null,
  characterTraits: [],
  worldElements: [],
  outlines: [],
  outlineActs: [],
  scenes: [],
  
  // Loading states
  isLoading: false,
  isLoadingTraits: false,
  isLoadingWorldElements: false,
  isLoadingOutlines: false,
  isLoadingScenes: false,
  
  // Error states
  error: null,
  traitsError: null,
  worldElementsError: null,
  outlinesError: null,
  scenesError: null,
  
  // UI state
  activeTab: 'braindump',
  selectedCharacterId: null,
  selectedOutlineId: null,
  
  // Filters
  characterTraitFilter: {},
  worldElementFilter: {},
  outlineFilter: {}
};

// Create the main store
const storyBibleStore = writable<StoryBibleState>(initialState);

// Helper function to handle errors
function handleError(error: any, errorType: keyof StoryBibleState): string {
  console.error(`Story Bible ${errorType}:`, error);
  const message = error?.message || error?.toString() || 'An unknown error occurred';
  
  storyBibleStore.update(state => ({
    ...state,
    [errorType]: message,
    isLoading: false,
    isLoadingTraits: false,
    isLoadingWorldElements: false,
    isLoadingOutlines: false,
    isLoadingScenes: false
  }));
  
  return message;
}

// Helper function to clear loading states
function clearLoadingStates() {
  storyBibleStore.update(state => ({
    ...state,
    isLoading: false,
    isLoadingTraits: false,
    isLoadingWorldElements: false,
    isLoadingOutlines: false,
    isLoadingScenes: false
  }));
}

// Story Bible Actions
export const storyBibleActions = {
  // Core Story Bible operations
  async createOrUpdateStoryBible(request: CreateStoryBibleRequest | UpdateStoryBibleRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoading: true, error: null }));
    
    try {
      const response = await invoke('create_or_update_story_bible', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          storyBible: response.data,
          isLoading: false,
          error: null
        }));
      } else {
        handleError(response.error, 'error');
      }
    } catch (error) {
      handleError(error, 'error');
    }
  },

  async loadStoryBible(projectId: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoading: true, error: null }));
    
    try {
      const response = await invoke('get_story_bible', { projectId });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          storyBible: response.data,
          isLoading: false,
          error: null
        }));
      } else {
        handleError(response.error, 'error');
      }
    } catch (error) {
      handleError(error, 'error');
    }
  },

  // Character Trait operations
  async createCharacterTrait(request: CreateCharacterTraitRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('create_character_trait', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          characterTraits: [...state.characterTraits, response.data],
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  },

  async updateCharacterTrait(request: UpdateCharacterTraitRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('update_character_trait', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          characterTraits: state.characterTraits.map(trait => 
            trait.id === request.id ? response.data : trait
          ),
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  },

  async deleteCharacterTrait(id: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('delete_character_trait', { id });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          characterTraits: state.characterTraits.filter(trait => trait.id !== id),
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  },

  async loadCharacterTraits(characterId: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('get_character_traits', { characterId });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          characterTraits: response.data,
          selectedCharacterId: characterId,
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  },

  // World Element operations
  async createWorldElement(request: CreateWorldElementRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('create_world_element', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          worldElements: [...state.worldElements, response.data],
          isLoadingWorldElements: false,
          worldElementsError: null
        }));
      } else {
        handleError(response.error, 'worldElementsError');
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
    }
  },

  async updateWorldElement(request: UpdateWorldElementRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('update_world_element', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          worldElements: state.worldElements.map(element => 
            element.id === request.id ? response.data : element
          ),
          isLoadingWorldElements: false,
          worldElementsError: null
        }));
      } else {
        handleError(response.error, 'worldElementsError');
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
    }
  },

  async deleteWorldElement(id: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('delete_world_element', { id });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          worldElements: state.worldElements.filter(element => element.id !== id),
          isLoadingWorldElements: false,
          worldElementsError: null
        }));
      } else {
        handleError(response.error, 'worldElementsError');
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
    }
  },

  async loadWorldElements(projectId: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('get_world_elements', { projectId });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          worldElements: response.data,
          isLoadingWorldElements: false,
          worldElementsError: null
        }));
      } else {
        handleError(response.error, 'worldElementsError');
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
    }
  },

  async searchWorldElements(request: SearchWorldElementsRequest): Promise<WorldElement[]> {
    try {
      const response = await invoke('search_world_elements', { request });
      
      if (response.success) {
        return response.data;
      } else {
        handleError(response.error, 'worldElementsError');
        return [];
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
      return [];
    }
  },

  // Outline operations
  async createOutline(request: CreateOutlineRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('create_outline', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          outlines: [...state.outlines, response.data],
          isLoadingOutlines: false,
          outlinesError: null
        }));
      } else {
        handleError(response.error, 'outlinesError');
      }
    } catch (error) {
      handleError(error, 'outlinesError');
    }
  },

  async updateOutline(request: UpdateOutlineRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('update_outline', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          outlines: state.outlines.map(outline => 
            outline.id === request.id ? response.data : outline
          ),
          isLoadingOutlines: false,
          outlinesError: null
        }));
      } else {
        handleError(response.error, 'outlinesError');
      }
    } catch (error) {
      handleError(error, 'outlinesError');
    }
  },

  async deleteOutline(id: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('delete_outline', { id });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          outlines: state.outlines.filter(outline => outline.id !== id),
          isLoadingOutlines: false,
          outlinesError: null
        }));
      } else {
        handleError(response.error, 'outlinesError');
      }
    } catch (error) {
      handleError(error, 'outlinesError');
    }
  },

  async loadOutlines(projectId: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('get_outlines', { projectId });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          outlines: response.data,
          isLoadingOutlines: false,
          outlinesError: null
        }));
      } else {
        handleError(response.error, 'outlinesError');
      }
    } catch (error) {
      handleError(error, 'outlinesError');
    }
  },

  async searchOutlines(request: SearchOutlinesRequest): Promise<Outline[]> {
    try {
      const response = await invoke('search_outlines', { request });
      
      if (response.success) {
        return response.data;
      } else {
        handleError(response.error, 'outlinesError');
        return [];
      }
    } catch (error) {
      handleError(error, 'outlinesError');
      return [];
    }
  },

  // Scene operations
  async createScene(request: CreateSceneRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('create_scene', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          scenes: [...state.scenes, response.data],
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  },

  async updateScene(request: UpdateSceneRequest): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('update_scene', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          scenes: state.scenes.map(scene => 
            scene.id === request.id ? response.data : scene
          ),
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  },

  async deleteScene(id: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('delete_scene', { id });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          scenes: state.scenes.filter(scene => scene.id !== id),
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  },

  async validateScene(id: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('validate_scene', { id });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          scenes: state.scenes.map(scene => 
            scene.id === id ? { ...scene, ...response.data } : scene
          ),
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  },

  async loadScenes(outlineId: string): Promise<void> {
    storyBibleStore.update(state => ({ ...state, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('get_scenes', { outlineId });
      
      if (response.success) {
        storyBibleStore.update(state => ({
          ...state,
          scenes: response.data,
          selectedOutlineId: outlineId,
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  },

  async searchScenes(request: SearchScenesRequest): Promise<Scene[]> {
    try {
      const response = await invoke('search_scenes', { request });
      
      if (response.success) {
        return response.data;
      } else {
        handleError(response.error, 'scenesError');
        return [];
      }
    } catch (error) {
      handleError(error, 'scenesError');
      return [];
    }
  },

  // AI Generation operations
  async generateSynopsis(request: GenerateSynopsisRequest): Promise<AIGenerationResponse | null> {
    storyBibleStore.update(state => ({ ...state, isLoading: true, error: null }));
    
    try {
      const response = await invoke('generate_synopsis', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({ ...state, isLoading: false, error: null }));
        return response.data;
      } else {
        handleError(response.error, 'error');
        return null;
      }
    } catch (error) {
      handleError(error, 'error');
      return null;
    }
  },

  async generateCharacterTraits(request: GenerateCharacterTraitsRequest): Promise<AIGenerationResponse | null> {
    storyBibleStore.update(state => ({ ...state, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('generate_character_traits', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({ ...state, isLoadingTraits: false, traitsError: null }));
        return response.data;
      } else {
        handleError(response.error, 'traitsError');
        return null;
      }
    } catch (error) {
      handleError(error, 'traitsError');
      return null;
    }
  },

  async generateWorldElement(request: GenerateWorldElementRequest): Promise<AIGenerationResponse | null> {
    storyBibleStore.update(state => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('generate_world_element', { request });
      
      if (response.success) {
        storyBibleStore.update(state => ({ ...state, isLoadingWorldElements: false, worldElementsError: null }));
        return response.data;
      } else {
        handleError(response.error, 'worldElementsError');
        return null;
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
      return null;
    }
  },

  // UI Actions
  setActiveTab(tab: 'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes'): void {
    storyBibleStore.update(state => ({ ...state, activeTab: tab }));
  },

  setSelectedCharacterId(characterId: string | null): void {
    storyBibleStore.update(state => ({ ...state, selectedCharacterId: characterId }));
  },

  setSelectedOutlineId(outlineId: string | null): void {
    storyBibleStore.update(state => ({ ...state, selectedOutlineId: outlineId }));
  },

  setCharacterTraitFilter(filter: Partial<StoryBibleState['characterTraitFilter']>): void {
    storyBibleStore.update(state => ({
      ...state,
      characterTraitFilter: { ...state.characterTraitFilter, ...filter }
    }));
  },

  setWorldElementFilter(filter: Partial<StoryBibleState['worldElementFilter']>): void {
    storyBibleStore.update(state => ({
      ...state,
      worldElementFilter: { ...state.worldElementFilter, ...filter }
    }));
  },

  setOutlineFilter(filter: Partial<StoryBibleState['outlineFilter']>): void {
    storyBibleStore.update(state => ({
      ...state,
      outlineFilter: { ...state.outlineFilter, ...filter }
    }));
  },

  // Error handling
  clearError(): void {
    storyBibleStore.update(state => ({
      ...state,
      error: null,
      traitsError: null,
      worldElementsError: null,
      outlinesError: null,
      scenesError: null
    }));
  },

  clearAllErrors(): void {
    storyBibleStore.update(state => ({
      ...state,
      error: null,
      traitsError: null,
      worldElementsError: null,
      outlinesError: null,
      scenesError: null
    }));
  },

  // Reset store
  reset(): void {
    storyBibleStore.set(initialState);
  }
};

// Derived stores for filtered data
export const filteredCharacterTraits = derived(
  storyBibleStore,
  ($store) => {
    let traits = $store.characterTraits;
    const filter = $store.characterTraitFilter;
    
    if (filter.visibility) {
      traits = traits.filter(trait => trait.visibility === filter.visibility);
    }
    
    if (filter.traitType) {
      traits = traits.filter(trait => trait.trait_type === filter.traitType);
    }
    
    return traits;
  }
);

export const filteredWorldElements = derived(
  storyBibleStore,
  ($store) => {
    let elements = $store.worldElements;
    const filter = $store.worldElementFilter;
    
    if (filter.elementType) {
      elements = elements.filter(element => element.element_type === filter.elementType);
    }
    
    if (filter.visibility) {
      elements = elements.filter(element => element.visibility === filter.visibility);
    }
    
    if (filter.seriesShared !== undefined) {
      elements = elements.filter(element => element.series_shared === filter.seriesShared);
    }
    
    return elements;
  }
);

export const filteredOutlines = derived(
  storyBibleStore,
  ($store) => {
    let outlines = $store.outlines;
    const filter = $store.outlineFilter;
    
    if (filter.characterPov) {
      outlines = outlines.filter(outline => outline.character_pov === filter.characterPov);
    }
    
    return outlines.sort((a, b) => (a.chapter_number || 0) - (b.chapter_number || 0));
  }
);

export const scenesForSelectedOutline = derived(
  storyBibleStore,
  ($store) => {
    if (!$store.selectedOutlineId) return [];
    
    return $store.scenes
      .filter(scene => scene.outline_id === $store.selectedOutlineId)
      .sort((a, b) => a.scene_number - b.scene_number);
  }
);

// Export the main store and actions
export { storyBibleStore };
export default storyBibleActions;