import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/tauri';
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
  SmartImportResult,
  AdvancedAIResponse
} from '../types/advancedAI';

export const useAdvancedAIStore = defineStore('advancedAI', {
  state: (): AdvancedAIState => ({
    // Prose Modes
    availableProseModes: [],
    currentProseMode: 'Excellent',
    
    // Generation State
    isGenerating: false,
    lastGenerationResult: undefined,
    streamingStatus: undefined,
    
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
    ultraCreativeMode: false,
    autoEnhancePrompts: true,
    clicheDetectionEnabled: true
  }),

  getters: {
    currentProseModeDetails: (state) => {
      return state.availableProseModes.find(mode => mode.name === state.currentProseMode);
    },
    
    activeStyleExamplesList: (state) => {
      return state.styleExamples.filter(example => 
        state.activeStyleExamples.includes(example.id) && example.is_active
      );
    },
    
    totalCreditsUsed: (state) => {
      return state.creditUsage.project_usage;
    },
    
    remainingCredits: (state) => {
      if (state.creditUsage.monthly_limit) {
        return Math.max(0, state.creditUsage.monthly_limit - state.creditUsage.project_usage);
      }
      return undefined;
    },
    
    canGenerate: (state) => {
      return !state.isGenerating && !state.isGeneratingImage;
    },
    
    currentBrainstormSessionData: (state) => {
      if (!state.currentBrainstormSession) return null;
      return state.activeBrainstormSessions.find(session => 
        session.id === state.currentBrainstormSession
      );
    }
  },

  actions: {
    // Initialize the store
    async initialize() {
      try {
        await this.loadAvailableProseModes();
        // Load other initial data as needed
      } catch (error) {
        console.error('Failed to initialize advanced AI store:', error);
      }
    },

    // Prose Mode Management
    async loadAvailableProseModes() {
      try {
        const response = await invoke<ProseMode[]>('get_available_prose_modes');
        this.availableProseModes = response;
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
      if (this.availableProseModes.some(mode => mode.name === modeName)) {
        this.currentProseMode = modeName;
      }
    },

    // Advanced Text Generation
    async generateWithProseMode(request: ProseGenerationRequest): Promise<AdvancedGenerationResult> {
      this.isGenerating = true;
      try {
        const response = await invoke<AdvancedGenerationResult>('generate_with_prose_mode', { request });
        this.lastGenerationResult = response;
        
        // Update credit usage
        await this.updateCreditUsage(request.project_id);
        
        return response;
      } catch (error) {
        console.error('Failed to generate with prose mode:', error);
        throw error;
      } finally {
        this.isGenerating = false;
      }
    },

    // Streaming Generation
    async startStreamingGeneration(request: ProseGenerationRequest): Promise<string> {
      this.isGenerating = true;
      try {
        const streamId = await invoke<string>('start_streaming_generation', { request });
        this.streamingStatus = {
          status: 'pending',
          progress: 0
        };
        
        // Start polling for status updates
        this.pollStreamingStatus(streamId);
        
        return streamId;
      } catch (error) {
        console.error('Failed to start streaming generation:', error);
        this.isGenerating = false;
        throw error;
      }
    },

    async pollStreamingStatus(streamId: string) {
      const pollInterval = setInterval(async () => {
        try {
          const status = await invoke<Record<string, any>>('get_stream_status', { streamId });
          
          this.streamingStatus = {
            status: status.status as any,
            progress: status.progress || 0,
            current_text: status.current_text,
            error_message: status.error_message,
            estimated_completion: status.estimated_completion
          };
          
          if (status.status === 'completed' || status.status === 'error') {
            clearInterval(pollInterval);
            this.isGenerating = false;
            
            if (status.status === 'completed') {
              // Handle completion
              this.lastGenerationResult = {
                generated_text: status.current_text || '',
                prose_mode_used: this.currentProseMode,
                token_count: 0,
                credits_used: 0,
                generation_id: streamId
              };
            }
          }
        } catch (error) {
          console.error('Failed to poll streaming status:', error);
          clearInterval(pollInterval);
          this.isGenerating = false;
        }
      }, 1000); // Poll every second
    },

    // Image Generation
    async generateImage(request: ImageGenerationRequest): Promise<GeneratedImage> {
      this.isGeneratingImage = true;
      try {
        const response = await invoke<GeneratedImage>('generate_image', { request });
        this.generatedImages.unshift(response); // Add to beginning of array
        
        // Update credit usage
        await this.updateCreditUsage(request.project_id);
        
        return response;
      } catch (error) {
        console.error('Failed to generate image:', error);
        throw error;
      } finally {
        this.isGeneratingImage = false;
      }
    },

    async loadProjectImages(projectId: string) {
      try {
        const response = await invoke<GeneratedImage[]>('get_project_images', { projectId });
        this.generatedImages = response;
      } catch (error) {
        console.error('Failed to load project images:', error);
        throw error;
      }
    },

    async deleteGeneratedImage(imageId: string) {
      try {
        await invoke('delete_generated_image', { imageId });
        this.generatedImages = this.generatedImages.filter(img => img.id !== imageId);
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
        const session = await this.getBrainstormSession(sessionId);
        if (session) {
          this.activeBrainstormSessions.unshift(session);
          this.currentBrainstormSession = sessionId;
        }
        
        // Update credit usage
        await this.updateCreditUsage(request.project_id);
        
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
        const session = this.activeBrainstormSessions.find(s => s.id === sessionId);
        if (session) {
          const idea = session.ideas.find(i => i.id === ideaId);
          if (idea) {
            idea.rating = rating;
          }
        }
      } catch (error) {
        console.error('Failed to rate idea:', error);
        throw error;
      }
    },

    async markIdeaAsKeeper(sessionId: string, ideaId: string, isKeeper: boolean) {
      try {
        await invoke('mark_idea_as_keeper', { sessionId, ideaId, isKeeper });
        
        // Update local state
        const session = this.activeBrainstormSessions.find(s => s.id === sessionId);
        if (session) {
          const idea = session.ideas.find(i => i.id === ideaId);
          if (idea) {
            idea.is_keeper = isKeeper;
          }
        }
      } catch (error) {
        console.error('Failed to mark idea as keeper:', error);
        throw error;
      }
    },

    setCurrentBrainstormSession(sessionId: string | undefined) {
      this.currentBrainstormSession = sessionId;
    },

    // Style Examples
    async addStyleExample(request: StyleExampleRequest): Promise<StyleExample> {
      try {
        const response = await invoke<StyleExample>('add_style_example', { request });
        this.styleExamples.unshift(response);
        
        if (response.is_active) {
          this.activeStyleExamples.push(response.id);
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
      if (active && !this.activeStyleExamples.includes(exampleId)) {
        this.activeStyleExamples.push(exampleId);
      } else if (!active) {
        this.activeStyleExamples = this.activeStyleExamples.filter(id => id !== exampleId);
      }
      
      // Update the example's active status
      const example = this.styleExamples.find(ex => ex.id === exampleId);
      if (example) {
        example.is_active = active;
      }
    },

    // Credit Management
    async updateCreditUsage(projectId: string) {
      try {
        const response = await invoke<CreditUsageResponse>('get_credit_usage', { projectId });
        this.creditUsage = response;
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
        this.lastSaliencyContext = response;
        return response;
      } catch (error) {
        console.error('Failed to build saliency context:', error);
        throw error;
      }
    },

    toggleSaliencyEngine(enabled: boolean) {
      this.saliencyEnabled = enabled;
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
    toggleUltraCreativeMode(enabled: boolean) {
      this.ultraCreativeMode = enabled;
    },

    toggleAutoEnhancePrompts(enabled: boolean) {
      this.autoEnhancePrompts = enabled;
    },

    toggleClicheDetection(enabled: boolean) {
      this.clicheDetectionEnabled = enabled;
    },

    // Utility Methods
    clearLastGeneration() {
      this.lastGenerationResult = undefined;
      this.streamingStatus = undefined;
    },

    clearBrainstormSessions() {
      this.activeBrainstormSessions = [];
      this.currentBrainstormSession = undefined;
    },

    clearGeneratedImages() {
      this.generatedImages = [];
    },

    // Error Handling
    handleError(error: any, context: string) {
      console.error(`Advanced AI Error in ${context}:`, error);
      
      // Reset loading states
      this.isGenerating = false;
      this.isGeneratingImage = false;
      
      // You could emit events here for global error handling
      // or show notifications
    }
  }
});