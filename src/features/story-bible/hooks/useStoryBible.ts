import { useState, useCallback, useMemo } from 'react';
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
  SearchWorldElementsRequest,
  SearchOutlinesRequest,
  SearchScenesRequest,
  UseStoryBibleReturn
} from '../../../types/storyBible';

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

export const useStoryBible = (): UseStoryBibleReturn => {
  const [state, setState] = useState<StoryBibleState>(initialState);

  // Helper function to handle errors
  const handleError = useCallback((error: any, errorType: keyof StoryBibleState): string => {
    console.error(`Story Bible ${errorType}:`, error);
    const message = error?.message || error?.toString() || 'An unknown error occurred';
    
    setState(prevState => ({
      ...prevState,
      [errorType]: message,
      isLoading: false,
      isLoadingTraits: false,
      isLoadingWorldElements: false,
      isLoadingOutlines: false,
      isLoadingScenes: false
    }));
    
    return message;
  }, []);

  // Core Story Bible operations
  const createOrUpdateStoryBible = useCallback(async (request: CreateStoryBibleRequest | UpdateStoryBibleRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoading: true, error: null }));
    
    try {
      const response = await invoke('create_or_update_story_bible', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
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
  }, [handleError]);

  const loadStoryBible = useCallback(async (projectId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoading: true, error: null }));
    
    try {
      const response = await invoke('get_story_bible', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
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
  }, [handleError]);

  // Character Trait operations
  const createCharacterTrait = useCallback(async (request: CreateCharacterTraitRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('create_character_trait', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: [...prevState.characterTraits, response.data],
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  }, [handleError]);

  const updateCharacterTrait = useCallback(async (request: UpdateCharacterTraitRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('update_character_trait', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: prevState.characterTraits.map(trait => 
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
  }, [handleError]);

  const deleteCharacterTrait = useCallback(async (id: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('delete_character_trait', { id });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: prevState.characterTraits.filter(trait => trait.id !== id),
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  }, [handleError]);

  const loadCharacterTraits = useCallback(async (characterId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke('get_character_traits', { characterId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: response.data,
          isLoadingTraits: false,
          traitsError: null
        }));
      } else {
        handleError(response.error, 'traitsError');
      }
    } catch (error) {
      handleError(error, 'traitsError');
    }
  }, [handleError]);

  // World Element operations
  const createWorldElement = useCallback(async (request: CreateWorldElementRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('create_world_element', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          worldElements: [...prevState.worldElements, response.data],
          isLoadingWorldElements: false,
          worldElementsError: null
        }));
      } else {
        handleError(response.error, 'worldElementsError');
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
    }
  }, [handleError]);

  const updateWorldElement = useCallback(async (request: UpdateWorldElementRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('update_world_element', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          worldElements: prevState.worldElements.map(element => 
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
  }, [handleError]);

  const deleteWorldElement = useCallback(async (id: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('delete_world_element', { id });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          worldElements: prevState.worldElements.filter(element => element.id !== id),
          isLoadingWorldElements: false,
          worldElementsError: null
        }));
      } else {
        handleError(response.error, 'worldElementsError');
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
    }
  }, [handleError]);

  const loadWorldElements = useCallback(async (projectId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke('get_world_elements', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
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
  }, [handleError]);

  const searchWorldElements = useCallback(async (request: SearchWorldElementsRequest): Promise<WorldElement[]> => {
    try {
      const response = await invoke('search_world_elements', { request });
      return response.success ? response.data : [];
    } catch (error) {
      handleError(error, 'worldElementsError');
      return [];
    }
  }, [handleError]);

  // Outline operations
  const createOutline = useCallback(async (request: CreateOutlineRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('create_outline', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          outlines: [...prevState.outlines, response.data],
          isLoadingOutlines: false,
          outlinesError: null
        }));
      } else {
        handleError(response.error, 'outlinesError');
      }
    } catch (error) {
      handleError(error, 'outlinesError');
    }
  }, [handleError]);

  const updateOutline = useCallback(async (request: UpdateOutlineRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('update_outline', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          outlines: prevState.outlines.map(outline => 
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
  }, [handleError]);

  const deleteOutline = useCallback(async (id: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('delete_outline', { id });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          outlines: prevState.outlines.filter(outline => outline.id !== id),
          isLoadingOutlines: false,
          outlinesError: null
        }));
      } else {
        handleError(response.error, 'outlinesError');
      }
    } catch (error) {
      handleError(error, 'outlinesError');
    }
  }, [handleError]);

  const loadOutlines = useCallback(async (projectId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke('get_outlines', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
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
  }, [handleError]);

  const searchOutlines = useCallback(async (request: SearchOutlinesRequest): Promise<Outline[]> => {
    try {
      const response = await invoke('search_outlines', { request });
      return response.success ? response.data : [];
    } catch (error) {
      handleError(error, 'outlinesError');
      return [];
    }
  }, [handleError]);

  // Scene operations
  const createScene = useCallback(async (request: CreateSceneRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('create_scene', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: [...prevState.scenes, response.data],
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  }, [handleError]);

  const updateScene = useCallback(async (request: UpdateSceneRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('update_scene', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: prevState.scenes.map(scene => 
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
  }, [handleError]);

  const deleteScene = useCallback(async (id: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('delete_scene', { id });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: prevState.scenes.filter(scene => scene.id !== id),
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  }, [handleError]);

  const validateScene = useCallback(async (id: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('validate_scene', { id });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: prevState.scenes.map(scene => 
            scene.id === id ? { ...scene, is_validated: true, validation_issues: response.data.issues } : scene
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
  }, [handleError]);

  const loadScenes = useCallback(async (outlineId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke('get_scenes', { outlineId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: response.data,
          isLoadingScenes: false,
          scenesError: null
        }));
      } else {
        handleError(response.error, 'scenesError');
      }
    } catch (error) {
      handleError(error, 'scenesError');
    }
  }, [handleError]);

  const searchScenes = useCallback(async (request: SearchScenesRequest): Promise<Scene[]> => {
    try {
      const response = await invoke('search_scenes', { request });
      return response.success ? response.data : [];
    } catch (error) {
      handleError(error, 'scenesError');
      return [];
    }
  }, [handleError]);

  // UI state management
  const setActiveTab = useCallback((tab: 'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes'): void => {
    setState(prevState => ({ ...prevState, activeTab: tab }));
  }, []);

  const setSelectedCharacterId = useCallback((characterId: string | null): void => {
    setState(prevState => ({ ...prevState, selectedCharacterId: characterId }));
  }, []);

  const setSelectedOutlineId = useCallback((outlineId: string | null): void => {
    setState(prevState => ({ ...prevState, selectedOutlineId: outlineId }));
  }, []);

  const setCharacterTraitFilter = useCallback((filter: Partial<StoryBibleState['characterTraitFilter']>): void => {
    setState(prevState => ({
      ...prevState,
      characterTraitFilter: { ...prevState.characterTraitFilter, ...filter }
    }));
  }, []);

  const setWorldElementFilter = useCallback((filter: Partial<StoryBibleState['worldElementFilter']>): void => {
    setState(prevState => ({
      ...prevState,
      worldElementFilter: { ...prevState.worldElementFilter, ...filter }
    }));
  }, []);

  const setOutlineFilter = useCallback((filter: Partial<StoryBibleState['outlineFilter']>): void => {
    setState(prevState => ({
      ...prevState,
      outlineFilter: { ...prevState.outlineFilter, ...filter }
    }));
  }, []);

  // Error management
  const clearError = useCallback((): void => {
    setState(prevState => ({
      ...prevState,
      error: null,
      traitsError: null,
      worldElementsError: null,
      outlinesError: null,
      scenesError: null
    }));
  }, []);

  // Filtered data using useMemo for performance
  const filteredCharacterTraits = useMemo(() => {
    return state.characterTraits.filter(trait => {
      const { visibility, traitType } = state.characterTraitFilter;
      
      if (visibility && trait.visibility !== visibility) return false;
      if (traitType && trait.trait_type !== traitType) return false;
      
      return true;
    });
  }, [state.characterTraits, state.characterTraitFilter]);

  const filteredWorldElements = useMemo(() => {
    return state.worldElements.filter(element => {
      const { elementType, visibility, seriesShared } = state.worldElementFilter;
      
      if (elementType && element.element_type !== elementType) return false;
      if (visibility && element.visibility !== visibility) return false;
      if (seriesShared !== undefined && element.series_shared !== seriesShared) return false;
      
      return true;
    });
  }, [state.worldElements, state.worldElementFilter]);

  const filteredOutlines = useMemo(() => {
    return state.outlines.filter(outline => {
      const { characterPov } = state.outlineFilter;
      
      if (characterPov && outline.character_pov !== characterPov) return false;
      
      return true;
    });
  }, [state.outlines, state.outlineFilter]);

  const scenesForSelectedOutline = useMemo(() => {
    if (!state.selectedOutlineId) return [];
    return state.scenes.filter(scene => scene.outline_id === state.selectedOutlineId);
  }, [state.scenes, state.selectedOutlineId]);

  return {
    // Data
    storyBible: state.storyBible,
    characterTraits: filteredCharacterTraits,
    worldElements: filteredWorldElements,
    outlines: filteredOutlines,
    scenes: scenesForSelectedOutline,
    
    // Loading states
    isLoading: state.isLoading,
    error: state.error,
    
    // Core operations
    createOrUpdateStoryBible,
    loadStoryBible,
    
    // Character operations
    createCharacterTrait,
    updateCharacterTrait,
    deleteCharacterTrait,
    loadCharacterTraits,
    
    // World element operations
    createWorldElement,
    updateWorldElement,
    deleteWorldElement,
    loadWorldElements,
    searchWorldElements,
    
    // Outline operations
    createOutline,
    updateOutline,
    deleteOutline,
    loadOutlines,
    searchOutlines,
    
    // Scene operations
    createScene,
    updateScene,
    deleteScene,
    validateScene,
    loadScenes,
    searchScenes,
    
    // UI state management
    setActiveTab,
    setSelectedCharacterId,
    setSelectedOutlineId,
    setCharacterTraitFilter,
    setWorldElementFilter,
    setOutlineFilter,
    
    // Error management
    clearError
  };
};

export default useStoryBible;