// Story Bible store for managing story bible state with Zustand

import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { invoke } from '../utils/tauriSafe';
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

// Store actions interface
interface StoryBibleActions {
  // Core operations
  createOrUpdateStoryBible: (request: CreateStoryBibleRequest | UpdateStoryBibleRequest) => Promise<void>;
  loadStoryBible: (projectId: string) => Promise<void>;
  
  // Character operations
  loadCharacters: (projectId: string) => Promise<void>;
  createCharacterTrait: (request: CreateCharacterTraitRequest) => Promise<void>;
  updateCharacterTrait: (request: UpdateCharacterTraitRequest) => Promise<void>;
  deleteCharacterTrait: (traitId: string) => Promise<void>;
  
  // World element operations
  loadWorldElements: (projectId: string) => Promise<void>;
  createWorldElement: (request: CreateWorldElementRequest) => Promise<void>;
  updateWorldElement: (request: UpdateWorldElementRequest) => Promise<void>;
  deleteWorldElement: (elementId: string) => Promise<void>;
  searchWorldElements: (request: SearchWorldElementsRequest) => Promise<void>;
  
  // Outline operations
  loadOutlines: (projectId: string) => Promise<void>;
  createOutline: (request: CreateOutlineRequest) => Promise<void>;
  updateOutline: (request: UpdateOutlineRequest) => Promise<void>;
  deleteOutline: (outlineId: string) => Promise<void>;
  searchOutlines: (request: SearchOutlinesRequest) => Promise<void>;
  
  // Scene operations
  loadScenes: (projectId: string) => Promise<void>;
  createScene: (request: CreateSceneRequest) => Promise<void>;
  updateScene: (request: UpdateSceneRequest) => Promise<void>;
  deleteScene: (sceneId: string) => Promise<void>;
  searchScenes: (request: SearchScenesRequest) => Promise<void>;
  
  // AI generation operations
  generateSynopsis: (request: GenerateSynopsisRequest) => Promise<void>;
  generateCharacterTraits: (request: GenerateCharacterTraitsRequest) => Promise<void>;
  generateWorldElement: (request: GenerateWorldElementRequest) => Promise<void>;
  
  // UI state operations
  setActiveTab: (tab: 'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes') => void;
  setSelectedCharacterId: (id: string | null) => void;
  setSelectedOutlineId: (id: string | null) => void;
  setCharacterTraitFilter: (filter: any) => void;
  setWorldElementFilter: (filter: any) => void;
  setOutlineFilter: (filter: any) => void;
  
  // Utility operations
  clearError: () => void;
  clearAllErrors: () => void;
}

type StoryBibleStore = StoryBibleState & StoryBibleActions;

// Helper functions
const handleError = (error: any): string => {
  if (typeof error === 'string') return error;
  if (error?.message) return error.message;
  if (error?.error) return error.error;
  return 'An unexpected error occurred';
};



export const useStoryBibleStore = create<StoryBibleStore>()(devtools(
  persist(
    (set) => ({
      // Initial state
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
      outlineFilter: {},

      // Actions
      createOrUpdateStoryBible: async (request: CreateStoryBibleRequest | UpdateStoryBibleRequest) => {
        set((state) => ({ ...state, isLoading: true, error: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: StoryBible; error?: string }>(
            'create_or_update_story_bible',
            request
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              storyBible: response.data,
              isLoading: false
            }));
          } else {
            set((state) => ({
              ...state,
              error: response.error || 'Failed to save story bible',
              isLoading: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            error: handleError(error),
            isLoading: false
          }));
        }
      },

      loadStoryBible: async (projectId: string) => {
        set((state) => ({ ...state, isLoading: true, error: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: StoryBible; error?: string }>(
            'get_story_bible',
            { project_id: projectId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              storyBible: response.data || null,
              isLoading: false
            }));
          } else {
            set((state) => ({
              ...state,
              error: response.error || 'Failed to load story bible',
              isLoading: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            error: handleError(error),
            isLoading: false
          }));
        }
      },

      loadCharacters: async (projectId: string) => {
        set((state) => ({ ...state, isLoadingCharacters: true, charactersError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Character[]; error?: string }>(
            'get_characters',
            { project_id: projectId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              characters: response.data || [],
              isLoadingCharacters: false
            }));
          } else {
            set((state) => ({
              ...state,
              charactersError: response.error || 'Failed to load characters',
              isLoadingCharacters: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            charactersError: handleError(error),
            isLoadingCharacters: false
          }));
        }
      },

      createCharacterTrait: async (request: CreateCharacterTraitRequest) => {
        set((state) => ({ ...state, isLoadingTraits: true, traitsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: CharacterTrait; error?: string }>(
            'create_character_trait',
            request
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              characterTraits: [...state.characterTraits, response.data],
              isLoadingTraits: false
            }));
          } else {
            set((state) => ({
              ...state,
              traitsError: response.error || 'Failed to create character trait',
              isLoadingTraits: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            traitsError: handleError(error),
            isLoadingTraits: false
          }));
        }
      },

      updateCharacterTrait: async (request: UpdateCharacterTraitRequest) => {
        set((state) => ({ ...state, isLoadingTraits: true, traitsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: CharacterTrait; error?: string }>(
            'update_character_trait',
            request
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              characterTraits: state.characterTraits.map(trait => 
                trait.id === response.data!.id ? response.data! : trait
              ),
              isLoadingTraits: false
            }));
          } else {
            set((state) => ({
              ...state,
              traitsError: response.error || 'Failed to update character trait',
              isLoadingTraits: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            traitsError: handleError(error),
            isLoadingTraits: false
          }));
        }
      },

      deleteCharacterTrait: async (traitId: string) => {
        set((state) => ({ ...state, isLoadingTraits: true, traitsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; error?: string }>(
            'delete_character_trait',
            { trait_id: traitId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              characterTraits: state.characterTraits.filter(trait => trait.id !== traitId),
              isLoadingTraits: false
            }));
          } else {
            set((state) => ({
              ...state,
              traitsError: response.error || 'Failed to delete character trait',
              isLoadingTraits: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            traitsError: handleError(error),
            isLoadingTraits: false
          }));
        }
      },

      loadWorldElements: async (projectId: string) => {
        set((state) => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: WorldElement[]; error?: string }>(
            'get_world_elements',
            { project_id: projectId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              worldElements: response.data || [],
              isLoadingWorldElements: false
            }));
          } else {
            set((state) => ({
              ...state,
              worldElementsError: response.error || 'Failed to load world elements',
              isLoadingWorldElements: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            worldElementsError: handleError(error),
            isLoadingWorldElements: false
          }));
        }
      },

      createWorldElement: async (request: CreateWorldElementRequest) => {
        set((state) => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: WorldElement; error?: string }>(
            'create_world_element',
            request
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              worldElements: [...state.worldElements, response.data],
              isLoadingWorldElements: false
            }));
          } else {
            set((state) => ({
              ...state,
              worldElementsError: response.error || 'Failed to create world element',
              isLoadingWorldElements: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            worldElementsError: handleError(error),
            isLoadingWorldElements: false
          }));
        }
      },

      updateWorldElement: async (request: UpdateWorldElementRequest) => {
        set((state) => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: WorldElement; error?: string }>(
            'update_world_element',
            request
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              worldElements: state.worldElements.map(element => 
                element.id === response.data!.id ? response.data! : element
              ),
              isLoadingWorldElements: false
            }));
          } else {
            set((state) => ({
              ...state,
              worldElementsError: response.error || 'Failed to update world element',
              isLoadingWorldElements: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            worldElementsError: handleError(error),
            isLoadingWorldElements: false
          }));
        }
      },

      deleteWorldElement: async (elementId: string) => {
        set((state) => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; error?: string }>(
            'delete_world_element',
            { element_id: elementId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              worldElements: state.worldElements.filter(element => element.id !== elementId),
              isLoadingWorldElements: false
            }));
          } else {
            set((state) => ({
              ...state,
              worldElementsError: response.error || 'Failed to delete world element',
              isLoadingWorldElements: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            worldElementsError: handleError(error),
            isLoadingWorldElements: false
          }));
        }
      },

      searchWorldElements: async (request: SearchWorldElementsRequest) => {
        set((state) => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: WorldElement[]; error?: string }>(
            'search_world_elements',
            request
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              worldElements: response.data || [],
              isLoadingWorldElements: false
            }));
          } else {
            set((state) => ({
              ...state,
              worldElementsError: response.error || 'Failed to search world elements',
              isLoadingWorldElements: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            worldElementsError: handleError(error),
            isLoadingWorldElements: false
          }));
        }
      },

      loadOutlines: async (projectId: string) => {
        set((state) => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Outline[]; error?: string }>(
            'get_outlines',
            { project_id: projectId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              outlines: response.data || [],
              isLoadingOutlines: false
            }));
          } else {
            set((state) => ({
              ...state,
              outlinesError: response.error || 'Failed to load outlines',
              isLoadingOutlines: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            outlinesError: handleError(error),
            isLoadingOutlines: false
          }));
        }
      },

      createOutline: async (request: CreateOutlineRequest) => {
        set((state) => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Outline; error?: string }>(
            'create_outline',
            request
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              outlines: [...state.outlines, response.data],
              isLoadingOutlines: false
            }));
          } else {
            set((state) => ({
              ...state,
              outlinesError: response.error || 'Failed to create outline',
              isLoadingOutlines: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            outlinesError: handleError(error),
            isLoadingOutlines: false
          }));
        }
      },

      updateOutline: async (request: UpdateOutlineRequest) => {
        set((state) => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Outline; error?: string }>(
            'update_outline',
            { request }
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              outlines: state.outlines.map(outline => 
                outline.id === response.data!.id ? response.data! : outline
              ),
              isLoadingOutlines: false
            }));
          } else {
            set((state) => ({
              ...state,
              outlinesError: response.error || 'Failed to update outline',
              isLoadingOutlines: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            outlinesError: handleError(error),
            isLoadingOutlines: false
          }));
        }
      },

      deleteOutline: async (outlineId: string) => {
        set((state) => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; error?: string }>(
            'delete_outline',
            { outlineId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              outlines: state.outlines.filter(outline => outline.id !== outlineId),
              isLoadingOutlines: false
            }));
          } else {
            set((state) => ({
              ...state,
              outlinesError: response.error || 'Failed to delete outline',
              isLoadingOutlines: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            outlinesError: handleError(error),
            isLoadingOutlines: false
          }));
        }
      },

      searchOutlines: async (request: SearchOutlinesRequest) => {
        set((state) => ({ ...state, isLoadingOutlines: true, outlinesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Outline[]; error?: string }>(
            'search_outlines',
            { request }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              outlines: response.data || [],
              isLoadingOutlines: false
            }));
          } else {
            set((state) => ({
              ...state,
              outlinesError: response.error || 'Failed to search outlines',
              isLoadingOutlines: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            outlinesError: handleError(error),
            isLoadingOutlines: false
          }));
        }
      },

      loadScenes: async (projectId: string) => {
        set((state) => ({ ...state, isLoadingScenes: true, scenesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Scene[]; error?: string }>(
            'get_scenes',
            { projectId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              scenes: response.data || [],
              isLoadingScenes: false
            }));
          } else {
            set((state) => ({
              ...state,
              scenesError: response.error || 'Failed to load scenes',
              isLoadingScenes: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            scenesError: handleError(error),
            isLoadingScenes: false
          }));
        }
      },

      createScene: async (request: CreateSceneRequest) => {
        set((state) => ({ ...state, isLoadingScenes: true, scenesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Scene; error?: string }>(
            'create_scene',
            { request }
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              scenes: [...state.scenes, response.data!],
              isLoadingScenes: false
            }));
          } else {
            set((state) => ({
              ...state,
              scenesError: response.error || 'Failed to create scene',
              isLoadingScenes: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            scenesError: handleError(error),
            isLoadingScenes: false
          }));
        }
      },

      updateScene: async (request: UpdateSceneRequest) => {
        set((state) => ({ ...state, isLoadingScenes: true, scenesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Scene; error?: string }>(
            'update_scene',
            { request }
          );
          
          if (response.success && response.data) {
            set((state) => ({
              ...state,
              scenes: state.scenes.map(scene => 
                scene.id === response.data!.id ? response.data! : scene
              ),
              isLoadingScenes: false
            }));
          } else {
            set((state) => ({
              ...state,
              scenesError: response.error || 'Failed to update scene',
              isLoadingScenes: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            scenesError: handleError(error),
            isLoadingScenes: false
          }));
        }
      },

      deleteScene: async (sceneId: string) => {
        set((state) => ({ ...state, isLoadingScenes: true, scenesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; error?: string }>(
            'delete_scene',
            { sceneId }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              scenes: state.scenes.filter(scene => scene.id !== sceneId),
              isLoadingScenes: false
            }));
          } else {
            set((state) => ({
              ...state,
              scenesError: response.error || 'Failed to delete scene',
              isLoadingScenes: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            scenesError: handleError(error),
            isLoadingScenes: false
          }));
        }
      },

      searchScenes: async (request: SearchScenesRequest) => {
        set((state) => ({ ...state, isLoadingScenes: true, scenesError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: Scene[]; error?: string }>(
            'search_scenes',
            { request }
          );
          
          if (response.success) {
            set((state) => ({
              ...state,
              scenes: response.data || [],
              isLoadingScenes: false
            }));
          } else {
            set((state) => ({
              ...state,
              scenesError: response.error || 'Failed to search scenes',
              isLoadingScenes: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            scenesError: handleError(error),
            isLoadingScenes: false
          }));
        }
      },

      generateSynopsis: async (request: GenerateSynopsisRequest) => {
        set((state) => ({ ...state, isLoading: true, error: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: AIGenerationResponse; error?: string }>(
            'generate_synopsis',
            { request }
          );
          
          if (response.success && response.data) {
            // Handle AI generation response - could update story bible or show in UI
            set((state) => ({ ...state, isLoading: false }));
          } else {
            set((state) => ({
              ...state,
              error: response.error || 'Failed to generate synopsis',
              isLoading: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            error: handleError(error),
            isLoading: false
          }));
        }
      },

      generateCharacterTraits: async (request: GenerateCharacterTraitsRequest) => {
        set((state) => ({ ...state, isLoadingTraits: true, traitsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: AIGenerationResponse; error?: string }>(
            'generate_character_traits',
            { request }
          );
          
          if (response.success && response.data) {
            // Handle AI generation response - could add new traits
            set((state) => ({ ...state, isLoadingTraits: false }));
          } else {
            set((state) => ({
              ...state,
              traitsError: response.error || 'Failed to generate character traits',
              isLoadingTraits: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            traitsError: handleError(error),
            isLoadingTraits: false
          }));
        }
      },

      generateWorldElement: async (request: GenerateWorldElementRequest) => {
        set((state) => ({ ...state, isLoadingWorldElements: true, worldElementsError: null }));
        
        try {
          const response = await invoke<{ success: boolean; data?: AIGenerationResponse; error?: string }>(
            'generate_world_element',
            { request }
          );
          
          if (response.success && response.data) {
            // Handle AI generation response - could add new world elements
            set((state) => ({ ...state, isLoadingWorldElements: false }));
          } else {
            set((state) => ({
              ...state,
              worldElementsError: response.error || 'Failed to generate world element',
              isLoadingWorldElements: false
            }));
          }
        } catch (error) {
          set((state) => ({
            ...state,
            worldElementsError: handleError(error),
            isLoadingWorldElements: false
          }));
        }
      },

      // UI state operations
      setActiveTab: (tab: 'braindump' | 'characters' | 'worldbuilding' | 'outline' | 'scenes') => {
        set((state) => ({ ...state, activeTab: tab }));
      },

      setSelectedCharacterId: (id: string | null) => {
        set((state) => ({ ...state, selectedCharacterId: id }));
      },

      setSelectedOutlineId: (id: string | null) => {
        set((state) => ({ ...state, selectedOutlineId: id }));
      },

      setCharacterTraitFilter: (filter: any) => {
        set((state) => ({ ...state, characterTraitFilter: filter }));
      },

      setWorldElementFilter: (filter: any) => {
        set((state) => ({ ...state, worldElementFilter: filter }));
      },

      setOutlineFilter: (filter: any) => {
        set((state) => ({ ...state, outlineFilter: filter }));
      },

      // Utility operations
      clearError: () => {
        set((state) => ({ ...state, error: null }));
      },

      clearAllErrors: () => {
        set((state) => ({
          ...state,
          error: null,
          charactersError: null,
          traitsError: null,
          worldElementsError: null,
          outlinesError: null,
          scenesError: null
        }));
      }
    }),
    {
      name: 'story-bible-store',
      partialize: (state) => ({
        activeTab: state.activeTab,
        selectedCharacterId: state.selectedCharacterId,
        selectedOutlineId: state.selectedOutlineId,
        characterTraitFilter: state.characterTraitFilter,
        worldElementFilter: state.worldElementFilter,
        outlineFilter: state.outlineFilter
      })
    }
  ),
  { name: 'StoryBibleStore' }
));

// Export the store for use in components
export default useStoryBibleStore;