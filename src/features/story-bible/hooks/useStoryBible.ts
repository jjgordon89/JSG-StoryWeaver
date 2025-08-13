import { useState, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * Standard Tauri response wrapper for all backend operations.
 */
interface TauriResponse<T = any> {
  /** Whether the operation was successful */
  success: boolean;
  /** The response data if successful */
  data?: T;
  /** Error message if unsuccessful */
  error?: string;
}

/**
 * Response from scene validation operations.
 */
interface ValidationResponse {
  /** Whether the scene passed validation */
  is_validated: boolean;
  /** Description of validation issues if any */
  validation_issues?: string;
}

import type {
  StoryBible,
  Character,
  CharacterTrait,
  WorldElement,
  Outline,
  Scene,
  StoryBibleState,
  CreateStoryBibleRequest,
  UpdateStoryBibleRequest,
  CreateCharacterRequest,
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
  GenerateSynopsisRequest,
  GenerateCharacterTraitsRequest,
  GenerateWorldElementRequest,
  GenerateOutlineRequest,
  GenerateScenesRequest,
  GenerateWorldBuildingRequest,
  AIGenerationResponse,
  UseStoryBibleReturn
} from '../../../types/storyBible';

// Initial state
const initialState: StoryBibleState = {
  // Data
  storyBible: null,
  characters: [],
  characterTraits: [],
  worldElements: [],
  outlines: [],
  outlineActs: [],
  scenes: [],
  
  // Loading states
  isLoading: false,
  isLoadingCharacters: false,
  isLoadingTraits: false,
  isLoadingWorldElements: false,
  isLoadingOutlines: false,
  isLoadingScenes: false,
  
  // Error states
  error: null,
  charactersError: null,
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

/**
 * Comprehensive hook for managing Story Bible functionality in StoryWeaver.
 * 
 * The Story Bible is a central system for organizing all story-related information
 * including characters, world-building elements, outlines, and scenes. This hook
 * provides a complete interface for CRUD operations, AI generation, search, and
 * state management for all Story Bible components.
 * 
 * @returns {UseStoryBibleReturn} Complete Story Bible management interface
 * 
 * @example
 * ```tsx
 * function StoryBibleManager({ projectId }: { projectId: string }) {
 *   const {
 *     storyBible,
 *     characters,
 *     worldElements,
 *     outlines,
 *     scenes,
 *     isLoading,
 *     loadStoryBible,
 *     createCharacter,
 *     generateWorldElement,
 *     setActiveTab
 *   } = useStoryBible();
 * 
 *   useEffect(() => {
 *     loadStoryBible(projectId);
 *   }, [projectId, loadStoryBible]);
 * 
 *   const handleCreateCharacter = async () => {
 *     await createCharacter({
 *       project_id: projectId,
 *       name: "New Character",
 *       description: "Character description"
 *     });
 *   };
 * 
 *   return (
 *     <div>
 *       {isLoading ? <Spinner /> : (
 *         <StoryBibleTabs
 *           characters={characters}
 *           worldElements={worldElements}
 *           onCreateCharacter={handleCreateCharacter}
 *         />
 *       )}
 *     </div>
 *   );
 * }
 * ```
 * 
 * @remarks
 * This hook manages complex state for multiple Story Bible entities:
 * - **Story Bible**: Core project story information and settings
 * - **Characters**: Character profiles with traits and relationships
 * - **World Elements**: Locations, cultures, magic systems, etc.
 * - **Outlines**: Story structure and plot organization
 * - **Scenes**: Individual story scenes with validation
 * - **AI Generation**: AI-powered content creation for all entities
 * 
 * The hook includes:
 * - Comprehensive error handling with specific error states per entity type
 * - Loading states for each operation type
 * - Filtering and search capabilities
 * - AI generation integration
 * - Optimistic UI updates
 * - Memoized filtered data for performance
 */
export const useStoryBible = (): UseStoryBibleReturn => {
  const [state, setState] = useState<StoryBibleState>(initialState);

  /**
   * Centralized error handler for all Story Bible operations.
   * 
   * @param {any} error - The error object or message
   * @param {keyof StoryBibleState} errorType - Which error state to update
   * @returns {string} The formatted error message
   */
  const handleError = useCallback((error: any, errorType: keyof StoryBibleState): string => {
    console.error(`Story Bible ${errorType}:`, error);
    const message = error?.message || error?.toString() || 'An unknown error occurred';
    
    setState(prevState => ({
      ...prevState,
      [errorType]: message,
      isLoading: false,
      isLoadingCharacters: false,
      isLoadingTraits: false,
      isLoadingWorldElements: false,
      isLoadingOutlines: false,
      isLoadingScenes: false
    }));
    
    return message;
  }, []);

  /**
   * Create a new Story Bible or update an existing one.
   * 
   * @param {CreateStoryBibleRequest | UpdateStoryBibleRequest} request - Story Bible data
   * @throws {Error} When the operation fails
   */
  const createOrUpdateStoryBible = useCallback(async (request: CreateStoryBibleRequest | UpdateStoryBibleRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoading: true, error: null }));
    
    try {
      const response = await invoke<TauriResponse<StoryBible>>('create_or_update_story_bible', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          storyBible: response.data || null,
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

  /**
   * Load the Story Bible for a specific project.
   * 
   * @param {string} projectId - The project ID to load Story Bible for
   * @throws {Error} When loading fails
   */
  const loadStoryBible = useCallback(async (projectId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoading: true, error: null }));
    
    try {
      const response = await invoke<TauriResponse<StoryBible>>('get_story_bible', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          storyBible: response.data || null,
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

  /**
   * Load all characters for a specific project.
   * 
   * @param {string} projectId - The project ID to load characters for
   * @throws {Error} When loading fails
   */
  const loadCharacters = useCallback(async (projectId: string): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingCharacters: true, charactersError: null }));
    
    try {
      const response = await invoke<TauriResponse<Character[]>>('get_characters', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characters: response.data || [],
          isLoadingCharacters: false,
          charactersError: null
        }));
      } else {
        handleError(response.error, 'charactersError');
      }
    } catch (error) {
      handleError(error, 'charactersError');
    }
  }, [handleError]);

  /**
   * Create a new character in the Story Bible.
   * 
   * @param {CreateCharacterRequest} request - Character creation data
   * @throws {Error} When creation fails
   */
  const createCharacter = useCallback(async (request: CreateCharacterRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingCharacters: true, charactersError: null }));
    
    try {
      const response = await invoke<TauriResponse<Character>>('create_character', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characters: [...prevState.characters, response.data as Character],
          isLoadingCharacters: false,
          charactersError: null
        }));
      } else {
        handleError(response.error, 'charactersError');
      }
    } catch (error) {
      handleError(error, 'charactersError');
    }
  }, [handleError]);

  // Character Trait operations
  const createCharacterTrait = useCallback(async (request: CreateCharacterTraitRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke<TauriResponse<CharacterTrait>>('create_character_trait', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: [...prevState.characterTraits, response.data as CharacterTrait],
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
      const response = await invoke<TauriResponse<CharacterTrait>>('update_character_trait', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: prevState.characterTraits.map(trait => 
            trait.id === request.id ? (response.data as CharacterTrait) : trait
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
      const response = await invoke<TauriResponse<void>>('delete_character_trait', { id });
      
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
      const response = await invoke<TauriResponse<CharacterTrait[]>>('get_character_traits', { characterId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          characterTraits: response.data || [],
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
      const response = await invoke<TauriResponse<WorldElement>>('create_world_element', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          worldElements: [...prevState.worldElements, response.data as WorldElement],
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
      const response = await invoke<TauriResponse<WorldElement>>('update_world_element', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          worldElements: prevState.worldElements.map(element => 
            element.id === request.id ? (response.data as WorldElement) : element
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
      const response = await invoke<TauriResponse<void>>('delete_world_element', { id });
      
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
      const response = await invoke<TauriResponse<WorldElement[]>>('get_world_elements', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          worldElements: response.data || [], // Ensure worldElements is always an array
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
      const response = await invoke<TauriResponse<WorldElement[]>>('search_world_elements', { request });
      return response.success && response.data ? response.data : [];
    } catch (error) {
      handleError(error, 'worldElementsError');
      return [];
    }
  }, [handleError]);

  // Outline operations
  const createOutline = useCallback(async (request: CreateOutlineRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke<TauriResponse<Outline>>('create_outline', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          outlines: [...prevState.outlines, response.data as Outline],
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
      const response = await invoke<TauriResponse<Outline>>('update_outline', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          outlines: prevState.outlines.map(outline => 
            outline.id === request.id ? (response.data as Outline) : outline
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
      const response = await invoke<TauriResponse<void>>('delete_outline', { id });
      
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
      const response = await invoke<TauriResponse<Outline[]>>('get_outlines', { projectId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          outlines: response.data || [], // Ensure outlines is always an array
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
      const response = await invoke<TauriResponse<Outline[]>>('search_outlines', { request });
      return response.success && response.data ? response.data : [];
    } catch (error) {
      handleError(error, 'outlinesError');
      return [];
    }
  }, [handleError]);

  // Scene operations
  const createScene = useCallback(async (request: CreateSceneRequest): Promise<void> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke<TauriResponse<Scene>>('create_scene', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: [...prevState.scenes, response.data as Scene],
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
      const response = await invoke<TauriResponse<Scene>>('update_scene', { request });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: prevState.scenes.map(scene => 
            scene.id === request.id ? (response.data as Scene) : scene
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
      const response = await invoke<TauriResponse<void>>('delete_scene', { id });
      
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
      const response = await invoke<TauriResponse<ValidationResponse>>('validate_scene', { id });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: prevState.scenes.map(scene => 
            scene.id === id ? { ...scene, is_validated: response.data?.is_validated || true, validation_issues: response.data?.validation_issues } : scene
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
      const response = await invoke<TauriResponse<Scene[]>>('get_scenes', { outlineId });
      
      if (response.success) {
        setState(prevState => ({
          ...prevState,
          scenes: response.data || [], // Ensure scenes is always an array
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
      const response = await invoke<TauriResponse<Scene[]>>('search_scenes', { request });
      return response.success && response.data ? response.data : [];
    } catch (error) {
      handleError(error, 'scenesError');
      return [];
    }
  }, [handleError]);

  /**
   * Generate a story synopsis using AI.
   * 
   * @param {GenerateSynopsisRequest} request - Synopsis generation parameters
   * @returns {Promise<AIGenerationResponse | null>} Generated synopsis or null if failed
   * @throws {Error} When generation fails
   */
  const generateSynopsis = useCallback(async (request: GenerateSynopsisRequest): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoading: true, error: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_synopsis', { request });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoading: false, error: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'error');
        return null;
      }
    } catch (error) {
      handleError(error, 'error');
      return null;
    }
  }, [handleError]);

  /**
   * Generate character traits using AI.
   * 
   * @param {GenerateCharacterTraitsRequest} request - Character trait generation parameters
   * @returns {Promise<AIGenerationResponse | null>} Generated traits or null if failed
   * @throws {Error} When generation fails
   */
  const generateCharacterTraits = useCallback(async (request: GenerateCharacterTraitsRequest): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoadingTraits: true, traitsError: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_character_traits', { request });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoadingTraits: false, traitsError: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'traitsError');
        return null;
      }
    } catch (error) {
      handleError(error, 'traitsError');
      return null;
    }
  }, [handleError]);

  /**
   * Generate world-building elements using AI.
   * 
   * @param {GenerateWorldElementRequest} request - World element generation parameters
   * @returns {Promise<AIGenerationResponse | null>} Generated world element or null if failed
   * @throws {Error} When generation fails
   */
  const generateWorldElement = useCallback(async (request: GenerateWorldElementRequest): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_world_element', { request });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoadingWorldElements: false, worldElementsError: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'worldElementsError');
        return null;
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
      return null;
    }
  }, [handleError]);

  const generateOutline = useCallback(async (request: GenerateOutlineRequest): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoadingOutlines: true, outlinesError: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_outline', { request });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoadingOutlines: false, outlinesError: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'outlinesError');
        return null;
      }
    } catch (error) {
      handleError(error, 'outlinesError');
      return null;
    }
  }, [handleError]);

  const generateScenes = useCallback(async (request: GenerateScenesRequest): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_scenes', { request });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoadingScenes: false, scenesError: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'scenesError');
        return null;
      }
    } catch (error) {
      handleError(error, 'scenesError');
      return null;
    }
  }, [handleError]);

  const generateWorldBuilding = useCallback(async (request: GenerateWorldBuildingRequest): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoadingWorldElements: true, worldElementsError: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_world_building', { request });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoadingWorldElements: false, worldElementsError: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'worldElementsError');
        return null;
      }
    } catch (error) {
      handleError(error, 'worldElementsError');
      return null;
    }
  }, [handleError]);

  const generateSceneContent = useCallback(async (outlineId: string, sceneTitle: string, sceneSummary: string, customPrompt?: string, creativity?: number): Promise<AIGenerationResponse | null> => {
    setState(prevState => ({ ...prevState, isLoadingScenes: true, scenesError: null }));
    
    try {
      const response = await invoke<TauriResponse<AIGenerationResponse>>('generate_scene_content', { 
        outlineId, 
        sceneTitle, 
        sceneSummary, 
        customPrompt, 
        creativity 
      });
      
      if (response.success) {
        setState(prevState => ({ ...prevState, isLoadingScenes: false, scenesError: null }));
        return response.data || null;
      } else {
        handleError(response.error, 'scenesError');
        return null;
      }
    } catch (error) {
      handleError(error, 'scenesError');
      return null;
    }
  }, [handleError]);

  /**
   * Set the active tab in the Story Bible interface.
   * 
   * @param {string} tab - The tab to activate
   */
  const setActiveTab = useCallback((tab: 'braindump' | 'style-examples' | 'characters' | 'worldbuilding' | 'outline' | 'scenes'): void => {
    setState(prevState => ({ ...prevState, activeTab: tab }));
  }, []);

  /**
   * Set the currently selected character for detailed view.
   * 
   * @param {string | null} characterId - Character ID to select, or null to deselect
   */
  const setSelectedCharacterId = useCallback((characterId: string | null): void => {
    setState(prevState => ({ ...prevState, selectedCharacterId: characterId }));
  }, []);

  /**
   * Set the currently selected outline for scene management.
   * 
   * @param {string | null} outlineId - Outline ID to select, or null to deselect
   */
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

  /**
   * Clear all error states across all Story Bible entities.
   */
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
      if (traitType && trait.trait_name !== traitType) return false;
      
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
    characters: state.characters,
    characterTraitFilter: state.characterTraitFilter,
    characterTraits: filteredCharacterTraits,
    worldElements: filteredWorldElements,
    filteredWorldElements,
    worldElementFilter: state.worldElementFilter,
    outlines: filteredOutlines,
    scenes: scenesForSelectedOutline,
    
    // Loading states
    isLoading: state.isLoading,
    isLoadingCharacters: state.isLoadingCharacters,
    isLoadingTraits: state.isLoadingTraits,
    isLoadingWorldElements: state.isLoadingWorldElements,
    isLoadingOutlines: state.isLoadingOutlines,
    isLoadingScenes: state.isLoadingScenes,
    
    // Error states
    error: state.error,
    charactersError: state.charactersError,
    traitsError: state.traitsError,
    worldElementsError: state.worldElementsError,
    outlinesError: state.outlinesError,
    scenesError: state.scenesError,
    
    // Core operations
    createOrUpdateStoryBible,
    loadStoryBible,
    
    // Character operations
    loadCharacters,
    createCharacter,
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
    
    // AI Generation operations
    generateSynopsis,
    generateCharacterTraits,
    generateWorldElement,
    generateOutline,
    generateScenes,
    generateWorldBuilding,
    generateSceneContent,
    
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
