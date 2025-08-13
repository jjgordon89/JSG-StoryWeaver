import { create } from 'zustand';
import { devtools } from 'zustand/middleware';
import { invoke } from '../utils/tauriSafe';
import showToast from '../utils/toast';
import type {
  AdvancedAIState,
  ProseGenerationRequest,
  AdvancedGenerationResult,
  ImageGenerationRequest,
  GeneratedImage,
  BrainstormSessionRequest,
  BrainstormSession,
  StyleExampleRequest,
  StyleExample,
  StyleAnalysis,
  ProseMode,
  CreditUsageResponse,
  SaliencyContext,
  StoryBibleElements,
  StreamingStatus,
  SmartImportRequest,
  SmartImportResult
} from '../types/advancedAI';
const STYLE_EXAMPLES_KEY = 'sw_style_examples_v1';

interface AdvancedAIActions {
  // Initialize the store
  initialize: () => Promise<void>;
  
  // Prose Mode Management
  loadAvailableProseModes: () => Promise<void>;
  getProseMode: (modeName: string) => Promise<ProseMode | null>;
  setCurrentProseMode: (modeName: string) => void;
  
  // Generation Methods
  generateWithProseMode: (request: ProseGenerationRequest) => Promise<AdvancedGenerationResult>;
  startStreamingGeneration: (request: ProseGenerationRequest) => Promise<string>;
  pollStreamingStatus: (streamId: string) => Promise<void>;
  
  // Control streaming
  cancelGeneration: () => Promise<void>;
  copyGeneratedTextToClipboard: () => Promise<void>;
  saveGeneratedContent: (options: {
    content: string;
    location: 'document' | 'snippet' | 'note';
    title?: string;
    metadata?: Record<string, any>;
    projectId?: string;
    documentId?: string;
  }) => Promise<void>;
  
  // Image Generation
  generateImage: (request: ImageGenerationRequest) => Promise<GeneratedImage>;
  loadProjectImages: (projectId: string) => Promise<void>;
  deleteGeneratedImage: (imageId: string) => Promise<void>;
  
  // Brainstorming
  createBrainstormSession: (request: BrainstormSessionRequest) => Promise<string>;
  getBrainstormSession: (sessionId: string) => Promise<BrainstormSession | null>;
  rateIdea: (sessionId: string, ideaId: string, rating: number) => Promise<void>;
  markIdeaAsKeeper: (sessionId: string, ideaId: string, isKeeper: boolean) => Promise<void>;
  setCurrentBrainstormSession: (sessionId: string | undefined) => void;
  
  // Style Management
  addStyleExample: (request: StyleExampleRequest) => Promise<StyleExample>;
  analyzeTextStyle: (content: string) => Promise<StyleAnalysis>;
  toggleStyleExample: (exampleId: string, active: boolean) => void;
  updateStyleExample: (update: Partial<StyleExample> & { id: string }) => Promise<StyleExample>;
  deleteStyleExample: (id: string) => Promise<void>;
  deleteStyleExamples: (ids: string[]) => Promise<void>;
  
  // Credits and Usage
  updateCreditUsage: (projectId: string) => Promise<void>;
  
  // Saliency Engine
  buildSaliencyContext: (projectId: string, textContext: string, storyBible: StoryBibleElements) => Promise<SaliencyContext>;
  toggleSaliencyEngine: (enabled: boolean) => void;
  
  // Smart Import
  smartImportContent: (request: SmartImportRequest) => Promise<SmartImportResult>;
  
  // Settings
  loadSettings: () => Promise<void>;
  updateSettings: (settings: Partial<any>) => Promise<void>;
  resetSettings: () => Promise<void>;
  exportSettings: () => string;
  importSettings: (settingsJson: string) => Promise<void>;
  toggleUltraCreativeMode: (enabled: boolean) => void;
  toggleAutoEnhancePrompts: (enabled: boolean) => void;
  toggleClicheDetection: (enabled: boolean) => void;
  
  // Utility Methods
  clearLastGeneration: () => void;
  clearBrainstormSessions: () => void;
  clearGeneratedImages: () => void;
  handleError: (error: any, context: string) => void;
}

export const useAdvancedAIStore = create<AdvancedAIState & AdvancedAIActions>()(devtools((set, get) => ({
    // Prose Modes
    availableProseModes: [],
    currentProseMode: 'Excellent',
    
    // Generation State
    isGenerating: false,
    lastGenerationResult: undefined,
    streamingStatus: undefined,
    activeStreamId: undefined,
    generationStartedAt: undefined,
    generationFinishedAt: undefined,
    streamingPollIntervalId: undefined,
    lastGenerationRequest: undefined,
    
    // Brainstorming
    activeBrainstormSessions: [],
    currentBrainstormSession: undefined,
    
    // Images
    generatedImages: [],
    isGeneratingImage: false,
    
    // Style Examples
    styleExamples: [],
    activeStyleExamples: [],
    
    // Credits
    creditUsage: {
      project_usage: 0,
      daily_usage: 0,
      monthly_limit: undefined,
      remaining_credits: undefined
    },
    
    // Saliency Engine
    saliencyEnabled: true,
    lastSaliencyContext: undefined,
    
    // Settings
  settings: {
      general: {
        ultraCreativeMode: false,
        autoEnhancePrompts: true,
        clicheDetectionEnabled: true,
        saliencyEnabled: false,
        defaultProseMode: 'balanced',
        autoSave: true,
        showAdvancedOptions: false,
        enableStreaming: true,
      },
      generation: {
        defaultContextLength: 4000,
        defaultOutputLength: 500,
        creativityLevel: 0.7,
        maxWords: 1000,
        creativityBoost: 50,
        styleConsistency: 70,
        clicheAvoidance: 80,
      },
      saliencyEngine: {
        enabled: false,
        autoBuild: true,
        refreshInterval: 60,
        includedElements: ['characters', 'plot', 'themes'],
      },
      imageGeneration: {
        qualityLevel: 'standard',
        enablePromptEnhancement: true,
        useStoryContext: true,
        autoSaveImages: false,
      },
      brainstorming: {
        defaultSessionDuration: 15,
        ideasPerGeneration: 5,
      },
    },
  ultraCreativeMode: false,
  autoEnhancePrompts: true,
  clicheDetectionEnabled: true,

    // Computed getters as properties
    get currentProseModeDetails() {
      return get().availableProseModes.find(mode => mode.name === get().currentProseMode);
    },
    
    get activeStyleExamplesList() {
      const state = get();
      return state.styleExamples.filter(example => 
        state.activeStyleExamples.includes(example.id) && example.is_active
      );
    },
    
    get totalCreditsUsed() {
      return get().creditUsage.project_usage;
    },
    
    get remainingCredits() {
      const state = get();
      if (state.creditUsage.monthly_limit) {
        return Math.max(0, state.creditUsage.monthly_limit - state.creditUsage.project_usage);
      }
      return undefined;
    },
    
    get canGenerate() {
      const state = get();
      return !state.isGenerating && !state.isGeneratingImage;
    },
    
    get currentBrainstormSessionData() {
      const state = get();
      if (!state.currentBrainstormSession) return null;
      return state.activeBrainstormSessions.find(session => 
        session.id === state.currentBrainstormSession
      );
    },

    // Actions
    // Initialize the store
    async initialize() {
      try {
        // Hydrate locally persisted style examples (optimistic persistence fallback)
        try {
          const raw = localStorage.getItem(STYLE_EXAMPLES_KEY);
          if (raw) {
            const saved = JSON.parse(raw) as StyleExample[];
            if (Array.isArray(saved) && saved.length > 0) {
              set({ styleExamples: saved });
            }
          }
        } catch (e) {
          console.error('Failed to load persisted style examples:', e);
        }

        await get().loadAvailableProseModes();
        // Load other initial data as needed
      } catch (error) {
        console.error('Failed to initialize advanced AI store:', error);
      }
    },

    // Prose Mode Management
    async loadAvailableProseModes() {
      try {
        const response = await invoke<ProseMode[]>('get_available_prose_modes');
        set({ availableProseModes: response });
      } catch (error) {
        console.error('Failed to load prose modes:', error);
        throw error;
      }
    },

    async getProseMode(modeName: string): Promise<ProseMode | null> {
      try {
        const response = await invoke<ProseMode | null>('get_prose_mode_details', { modeName });
        return response;
      } catch (error) {
        console.error('Failed to get prose mode details:', error);
        return null;
      }
    },

    setCurrentProseMode(modeName: string) {
      const { availableProseModes } = get();
      if (availableProseModes.some(mode => mode.name === modeName)) {
        set({ currentProseMode: modeName });
      }
    },

    // Advanced Text Generation
    async generateWithProseMode(request: ProseGenerationRequest): Promise<AdvancedGenerationResult> {
      set({ isGenerating: true, generationStartedAt: new Date().toISOString(), lastGenerationRequest: request });
      try {
        const response = await invoke<AdvancedGenerationResult>('generate_with_prose_mode', { request });
        set({ lastGenerationResult: response });
        
        // Update credit usage
        await get().updateCreditUsage(request.project_id);
        
        return response;
      } catch (error) {
        console.error('Failed to generate with prose mode:', error);
        throw error;
      } finally {
        set({ isGenerating: false, generationFinishedAt: new Date().toISOString() });
      }
    },

    // Streaming Generation
    async startStreamingGeneration(request: ProseGenerationRequest): Promise<string> {
      set({ isGenerating: true, generationStartedAt: new Date().toISOString(), lastGenerationRequest: request });
      try {
        const streamId = await invoke<string>('start_streaming_generation', { request });
        set({ activeStreamId: streamId });
        set({ 
          streamingStatus: {
            status: 'pending',
            progress: 0
          }
        });
        
        // Start polling for status updates
        get().pollStreamingStatus(streamId);
        
        return streamId;
      } catch (error) {
        console.error('Failed to start streaming generation:', error);
        set({ isGenerating: false });
        throw error;
      }
    },

    async pollStreamingStatus(streamId: string) {
      const pollInterval = setInterval(async () => {
        // store interval id so we can cancel externally
        set({ streamingPollIntervalId: Number(pollInterval) });
        try {
          const status = await invoke<Record<string, any>>('get_stream_status', { streamId });
          
          set({ 
            streamingStatus: {
              status: status.status as any,
              progress: status.progress || 0,
              current_text: status.current_text,
              error_message: status.error_message,
              estimated_completion: status.estimated_completion
            }
          });
          
          if (status.status === 'completed' || status.status === 'error') {
            clearInterval(pollInterval);
            set({ 
              isGenerating: false,
              generationFinishedAt: new Date().toISOString(),
              activeStreamId: undefined,
              streamingPollIntervalId: undefined
            });
            
            if (status.status === 'completed') {
              // Handle completion
              const { currentProseMode } = get();
              set({
                lastGenerationResult: {
                  generated_text: status.current_text || '',
                  prose_mode_used: currentProseMode,
                  token_count: 0,
                  credits_used: 0,
                  generation_id: streamId
                }
              });
            }
          }
        } catch (error) {
          console.error('Failed to poll streaming status:', error);
          clearInterval(pollInterval);
          set({ isGenerating: false });
        }
      }, 1000); // Poll every second
    },

    // Image Generation
    async generateImage(request: ImageGenerationRequest): Promise<GeneratedImage> {
      set({ isGeneratingImage: true });
      try {
        const response = await invoke<GeneratedImage>('generate_image', { request });
        const { generatedImages } = get();
        set({ generatedImages: [response, ...generatedImages] }); // Add to beginning of array
        
        // Update credit usage
        await get().updateCreditUsage(request.project_id);
        
        return response;
      } catch (error) {
        console.error('Failed to generate image:', error);
        throw error;
      } finally {
        set({ isGeneratingImage: false });
      }
    },

    async loadProjectImages(projectId: string) {
      try {
        const response = await invoke<GeneratedImage[]>('get_project_images', { projectId });
        set({ generatedImages: response });
      } catch (error) {
        console.error('Failed to load project images:', error);
        throw error;
      }
    },

    async deleteGeneratedImage(imageId: string) {
      try {
        await invoke('delete_generated_image', { imageId });
        const { generatedImages } = get();
        set({ generatedImages: generatedImages.filter(img => img.id !== imageId) });
      } catch (error) {
        console.error('Failed to delete generated image:', error);
        throw error;
      }
    },

    // Brainstorming
    async createBrainstormSession(request: BrainstormSessionRequest): Promise<string> {
      try {
        const sessionId = await invoke<string>('create_brainstorm_session', { request });
        
        // Load the created session
        const session = await get().getBrainstormSession(sessionId);
        if (session) {
          const { activeBrainstormSessions } = get();
          set({ 
            activeBrainstormSessions: [session, ...activeBrainstormSessions],
            currentBrainstormSession: sessionId
          });
        }
        
        // Update credit usage
        await get().updateCreditUsage(request.project_id);
        
        return sessionId;
      } catch (error) {
        console.error('Failed to create brainstorm session:', error);
        throw error;
      }
    },

    async getBrainstormSession(sessionId: string): Promise<BrainstormSession | null> {
      try {
        const response = await invoke<BrainstormSession | null>('get_brainstorm_session', { sessionId });
        return response;
      } catch (error) {
        console.error('Failed to get brainstorm session:', error);
        return null;
      }
    },

    async rateIdea(sessionId: string, ideaId: string, rating: number) {
      try {
        await invoke('rate_brainstorm_idea', { sessionId, ideaId, rating });
        
        // Update local state
        const { activeBrainstormSessions } = get();
        const updatedSessions = activeBrainstormSessions.map(session => {
          if (session.id === sessionId) {
            return {
              ...session,
              ideas: session.ideas.map(idea => 
                idea.id === ideaId ? { ...idea, rating } : idea
              )
            };
          }
          return session;
        });
        set({ activeBrainstormSessions: updatedSessions });
      } catch (error) {
        console.error('Failed to rate idea:', error);
        throw error;
      }
    },

    async markIdeaAsKeeper(sessionId: string, ideaId: string, isKeeper: boolean) {
      try {
        await invoke('mark_idea_as_keeper', { sessionId, ideaId, isKeeper });
        
        // Update local state
        const { activeBrainstormSessions } = get();
        const updatedSessions = activeBrainstormSessions.map(session => {
          if (session.id === sessionId) {
            return {
              ...session,
              ideas: session.ideas.map(idea => 
                idea.id === ideaId ? { ...idea, is_keeper: isKeeper } : idea
              )
            };
          }
          return session;
        });
        set({ activeBrainstormSessions: updatedSessions });
      } catch (error) {
        console.error('Failed to mark idea as keeper:', error);
        throw error;
      }
    },

    setCurrentBrainstormSession(sessionId: string | undefined) {
      set({ currentBrainstormSession: sessionId });
    },

    // Style Examples
    async addStyleExample(request: StyleExampleRequest): Promise<StyleExample> {
      try {
        const response = await invoke<StyleExample>('add_style_example', { request });
        const { styleExamples, activeStyleExamples } = get();
        
        const newExamples = [response, ...styleExamples];
        set({ styleExamples: newExamples });
        
        if (response.is_active) {
          set({ activeStyleExamples: [...activeStyleExamples, response.id] });
        }

        // Persist locally
        try {
          localStorage.setItem(STYLE_EXAMPLES_KEY, JSON.stringify(newExamples));
        } catch (e) {
          console.error('Failed to persist style examples:', e);
        }
        
        return response;
      } catch (error) {
        console.error('Failed to add style example:', error);
        throw error;
      }
    },

    async analyzeTextStyle(content: string): Promise<StyleAnalysis> {
      try {
        const response = await invoke<StyleAnalysis>('analyze_text_style', { content });
        return response;
      } catch (error) {
        console.error('Failed to analyze text style:', error);
        throw error;
      }
    },

    toggleStyleExample(exampleId: string, active: boolean) {
      const { activeStyleExamples, styleExamples } = get();
      
      if (active && !activeStyleExamples.includes(exampleId)) {
        set({ activeStyleExamples: [...activeStyleExamples, exampleId] });
      } else if (!active) {
        set({ activeStyleExamples: activeStyleExamples.filter(id => id !== exampleId) });
      }
      
      // Update the example's active status
      const updatedExamples = styleExamples.map(example => 
        example.id === exampleId ? { ...example, is_active: active } : example
      );
      set({ styleExamples: updatedExamples });
    },

    async updateStyleExample(update: Partial<StyleExample> & { id: string }): Promise<StyleExample> {
      const { styleExamples } = get();
      const existing = styleExamples.find(se => se.id === update.id);
      if (!existing) {
        throw new Error('Style example not found');
      }
      const merged: StyleExample = {
        ...existing,
        ...update,
        word_count: (update.content ?? existing.content).split(/\s+/).filter(Boolean).length
      };
      const updated = styleExamples.map(se => (se.id === merged.id ? merged : se));
      set({
        styleExamples: updated
      });
      // Persist locally
      try {
        localStorage.setItem(STYLE_EXAMPLES_KEY, JSON.stringify(updated));
      } catch (e) {
        console.error('Failed to persist style examples:', e);
      }
      // TODO: Persist to backend when dedicated endpoints exist
      return merged;
    },

    async deleteStyleExample(id: string): Promise<void> {
      const { styleExamples, activeStyleExamples } = get();
      const nextExamples = styleExamples.filter(se => se.id !== id);
      set({
        styleExamples: nextExamples,
        activeStyleExamples: activeStyleExamples.filter(aid => aid !== id)
      });
      // Persist locally
      try {
        localStorage.setItem(STYLE_EXAMPLES_KEY, JSON.stringify(nextExamples));
      } catch (e) {
        console.error('Failed to persist style examples:', e);
      }
      // TODO: Persist deletion to backend when dedicated endpoints exist
    },

    async deleteStyleExamples(ids: string[]): Promise<void> {
      const { styleExamples, activeStyleExamples } = get();
      const idsSet = new Set(ids);
      const nextExamples = styleExamples.filter(se => !idsSet.has(se.id));
      set({
        styleExamples: nextExamples,
        activeStyleExamples: activeStyleExamples.filter(aid => !idsSet.has(aid))
      });
      // Persist locally
      try {
        localStorage.setItem(STYLE_EXAMPLES_KEY, JSON.stringify(nextExamples));
      } catch (e) {
        console.error('Failed to persist style examples:', e);
      }
      // TODO: Persist bulk deletion to backend when dedicated endpoints exist
    },

    // Credit Management
    async updateCreditUsage(projectId: string) {
      try {
        const response = await invoke<CreditUsageResponse>('get_credit_usage', { projectId });
        set({ creditUsage: response });
      } catch (error) {
        console.error('Failed to update credit usage:', error);
      }
    },

    // Saliency Engine
    async buildSaliencyContext(
      projectId: string, 
      textContext: string, 
      storyBible: StoryBibleElements
    ): Promise<SaliencyContext> {
      try {
        const response = await invoke<SaliencyContext>('build_saliency_context', {
          projectId,
          textContext,
          storyBible
        });
        set({ lastSaliencyContext: response });
        return response;
      } catch (error) {
        console.error('Failed to build saliency context:', error);
        throw error;
      }
    },

    toggleSaliencyEngine(enabled: boolean) {
      const currentSettings = get().settings;
      const updatedSettings = { 
        ...currentSettings, 
        general: { ...currentSettings.general, saliencyEnabled: enabled }
      };
      set({ 
        saliencyEnabled: enabled,
        settings: updatedSettings
      });
    },

    // Smart Import
    async smartImportContent(request: SmartImportRequest): Promise<SmartImportResult> {
      try {
        const response = await invoke<Record<string, any>>('smart_import_content', { 
          projectId: request.project_id,
          content: request.content,
          contentType: request.content_type
        });
        
        return {
          status: response.status as string,
          suggestions: response.suggestions || [],
          extracted_elements: response.extracted_elements || {}
        };
      } catch (error) {
        console.error('Failed to smart import content:', error);
        throw error;
      }
    },

    // Settings
    async loadSettings() {
      try {
        const settings = await invoke('load_advanced_ai_settings');
        set({ 
          settings: {
            general: { ...settings.general },
            generation: { ...settings.generation },
            saliencyEngine: { ...settings.saliencyEngine },
            imageGeneration: { ...settings.imageGeneration },
            brainstorming: { ...settings.brainstorming }
          },
          ultraCreativeMode: settings.general?.ultraCreativeMode ?? false,
          autoEnhancePrompts: settings.general?.autoEnhancePrompts ?? true,
          clicheDetectionEnabled: settings.general?.clicheDetectionEnabled ?? true,
          saliencyEnabled: settings.general?.saliencyEnabled ?? false
        });
      } catch (error) {
        console.error('Failed to load settings:', error);
      }
    },

    async updateSettings(newSettings: Partial<any>) {
      try {
        const currentSettings = get().settings;
        const updatedSettings = {
          general: { ...currentSettings.general, ...newSettings.general },
          generation: { ...currentSettings.generation, ...newSettings.generation },
          saliencyEngine: { ...currentSettings.saliencyEngine, ...newSettings.saliencyEngine },
          imageGeneration: { ...currentSettings.imageGeneration, ...newSettings.imageGeneration },
          brainstorming: { ...currentSettings.brainstorming, ...newSettings.brainstorming }
        };
        await invoke('update_advanced_ai_settings', { settings: updatedSettings });
        set({ 
          settings: updatedSettings,
          ultraCreativeMode: updatedSettings.general?.ultraCreativeMode ?? get().ultraCreativeMode,
          autoEnhancePrompts: updatedSettings.general?.autoEnhancePrompts ?? get().autoEnhancePrompts,
          clicheDetectionEnabled: updatedSettings.general?.clicheDetectionEnabled ?? get().clicheDetectionEnabled,
          saliencyEnabled: updatedSettings.general?.saliencyEnabled ?? get().saliencyEnabled
        });
      } catch (error) {
        console.error('Failed to update settings:', error);
      }
    },

    async resetSettings() {
      try {
        await invoke('reset_advanced_ai_settings');
        const defaultSettings = {
           general: {
             ultraCreativeMode: false,
             autoEnhancePrompts: true,
             clicheDetectionEnabled: true,
             saliencyEnabled: false,
             defaultProseMode: 'balanced',
             autoSave: true,
             showAdvancedOptions: false,
             enableStreaming: true,
           },
           generation: {
             defaultContextLength: 4000,
             defaultOutputLength: 500,
             creativityLevel: 0.7,
             maxWords: 1000,
             creativityBoost: 50,
             styleConsistency: 70,
             clicheAvoidance: 80,
           },
           saliencyEngine: {
              enabled: false,
              autoBuild: true,
              refreshInterval: 60,
              includedElements: ['characters', 'plot', 'themes'],
            },
            imageGeneration: {
              qualityLevel: 'standard',
              enablePromptEnhancement: true,
              useStoryContext: true,
              autoSaveImages: false,
            },
            brainstorming: {
              defaultSessionDuration: 15,
              ideasPerGeneration: 5,
            },
          };
        set({ 
          settings: defaultSettings,
          ultraCreativeMode: defaultSettings.general.ultraCreativeMode,
          autoEnhancePrompts: defaultSettings.general.autoEnhancePrompts,
          clicheDetectionEnabled: defaultSettings.general.clicheDetectionEnabled,
          saliencyEnabled: defaultSettings.general.saliencyEnabled
        });
      } catch (error) {
        console.error('Failed to reset settings:', error);
      }
    },

    exportSettings() {
      const settings = get().settings;
      return JSON.stringify(settings, null, 2);
    },

    async importSettings(settingsJson: string) {
      try {
        const settings = JSON.parse(settingsJson);
        await invoke('update_advanced_ai_settings', { settings });
        const currentSettings = get().settings;
        set({ 
          settings: {
            general: { ...currentSettings.general, ...settings.general },
            generation: { ...currentSettings.generation, ...settings.generation },
            saliencyEngine: { ...currentSettings.saliencyEngine, ...settings.saliencyEngine },
            imageGeneration: { ...currentSettings.imageGeneration, ...settings.imageGeneration },
            brainstorming: { ...currentSettings.brainstorming, ...settings.brainstorming }
          },
          ultraCreativeMode: settings.general?.ultraCreativeMode ?? get().ultraCreativeMode,
          autoEnhancePrompts: settings.general?.autoEnhancePrompts ?? get().autoEnhancePrompts,
          clicheDetectionEnabled: settings.general?.clicheDetectionEnabled ?? get().clicheDetectionEnabled,
          saliencyEnabled: settings.general?.saliencyEnabled ?? get().saliencyEnabled
        });
      } catch (error) {
        console.error('Failed to import settings:', error);
        throw error;
      }
    },

    toggleUltraCreativeMode(enabled: boolean) {
      const currentSettings = get().settings;
      const updatedSettings = { 
        ...currentSettings, 
        general: { ...currentSettings.general, ultraCreativeMode: enabled }
      };
      set({ 
        ultraCreativeMode: enabled,
        settings: updatedSettings
      });
    },

    toggleAutoEnhancePrompts(enabled: boolean) {
      const currentSettings = get().settings;
      const updatedSettings = { 
        ...currentSettings, 
        general: { ...currentSettings.general, autoEnhancePrompts: enabled }
      };
      set({ 
        autoEnhancePrompts: enabled,
        settings: updatedSettings
      });
    },

    toggleClicheDetection(enabled: boolean) {
      const currentSettings = get().settings;
      const updatedSettings = { 
        ...currentSettings, 
        general: { ...currentSettings.general, clicheDetectionEnabled: enabled }
      };
      set({ 
        clicheDetectionEnabled: enabled,
        settings: updatedSettings
      });
    },

    // Control streaming and utilities
    async cancelGeneration() {
      try {
        const { activeStreamId, streamingPollIntervalId, streamingStatus } = get();
        if (streamingPollIntervalId) {
          clearInterval(streamingPollIntervalId as unknown as number);
        }
        if (activeStreamId) {
          try {
            await invoke('cancel_streaming_generation', { streamId: activeStreamId });
          } catch (e) {
            console.warn('cancel_streaming_generation not available or failed:', e);
          }
        }
        set({ 
          isGenerating: false,
          activeStreamId: undefined,
          streamingPollIntervalId: undefined,
          generationFinishedAt: new Date().toISOString(),
          streamingStatus: {
            status: 'error',
            progress: streamingStatus?.progress || 0,
            error_message: 'Cancelled by user',
            current_text: streamingStatus?.current_text
          }
        });
      } catch (error) {
        console.error('Failed to cancel generation:', error);
      }
    },

    async copyGeneratedTextToClipboard() {
      try {
        const { lastGenerationResult } = get();
        const text = lastGenerationResult?.generated_text || '';
        if (!text) {
          showToast.info('Nothing to copy');
          return;
        }
        await navigator.clipboard.writeText(text);
        showToast.success('Copied generated text');
      } catch (error) {
        console.error('Clipboard copy failed:', error);
        showToast.error('Copy failed');
      }
    },

    async saveGeneratedContent(options: {
      content: string;
      location: 'document' | 'snippet' | 'note';
      title?: string;
      metadata?: Record<string, any>;
      projectId?: string;
      documentId?: string;
    }) {
      try {
        await invoke('save_generated_content', { data: options });
        showToast.success('Content saved');
      } catch (error) {
        console.error('Failed to save generated content:', error);
        showToast.error('Save failed');
        throw error;
      }
    },

    // Utility Methods
    clearLastGeneration() {
      set({ 
        lastGenerationResult: undefined,
        streamingStatus: undefined 
      });
    },

    clearBrainstormSessions() {
      set({ 
        activeBrainstormSessions: [],
        currentBrainstormSession: undefined 
      });
    },

    clearGeneratedImages() {
      set({ generatedImages: [] });
    },

    // Error Handling
    handleError(error: any, context: string) {
      console.error(`Advanced AI Error in ${context}:`, error);
      
      // Reset loading states
      set({ 
        isGenerating: false,
        isGeneratingImage: false 
      });
      
      // You could emit events here for global error handling
      // or show notifications
    }
  }
)));

export default useAdvancedAIStore;
