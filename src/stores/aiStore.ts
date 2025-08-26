import { create } from 'zustand';
import { invoke } from '../utils/tauriSafe';
import { AISettings, StreamingEnvelope } from '../types/ai';

// Types for AI writing functionality
export interface WriteSettings {
  creativity_level: number; // 1-10
  tone: string;
  key_details: string;
  card_count?: number; // Number of cards to generate
  card_length?: 'short' | 'medium' | 'long';
  prose_mode?: string; // AI model selection
}

export interface WriteResult {
  generated_text: string;
  credits_used: number;
  word_count: number;
  tokens_used?: number;
}

export interface RewriteSettings {
  style: 'rephrase' | 'shorter' | 'longer' | 'more_formal' | 'more_casual' | 'more_descriptive' | 'simpler';
  creativity_level: number;
  preserve_meaning: boolean;
}

export interface ExpandSettings {
  focus: 'sensory_details' | 'dialogue' | 'action' | 'emotion' | 'setting';
  length_multiplier: number; // 1.5x, 2x, 3x
  creativity_level: number;
}

export interface BrainstormSettings {
  category: 'characters' | 'plot_points' | 'settings' | 'conflicts' | 'themes';
  count: number; // Number of ideas to generate
  creativity_level: number;
}

export interface QuickEditSettings {
  instruction: string;
  high_quality_mode: boolean;
}

export interface StreamingState {
  isStreaming: boolean;
  currentText: string;
  streamId: string | null;
  canPause: boolean;
  isPaused: boolean;
}

export type { StreamingEnvelope };

interface AIState {
  // Current operation state
  isLoading: boolean;
  error: string | null;
  
  // Streaming state
  streaming: StreamingState;
  
  // Generated content
  lastResult: WriteResult | null;
  
  // Settings
  defaultWriteSettings: WriteSettings;
  defaultRewriteSettings: RewriteSettings;
  defaultExpandSettings: ExpandSettings;
  defaultBrainstormSettings: BrainstormSettings;
  globalSettings: AISettings;
  
  // Credit tracking
  creditsUsed: number;
  creditsRemaining: number | null;
  
  // Actions
  // Write tools
  autoWrite: (documentId: number, cursorPosition: number, settings?: Partial<WriteSettings>) => Promise<WriteResult>;
  guidedWrite: (documentId: number, userPrompt: string, settings?: Partial<WriteSettings>) => Promise<WriteResult>;
  toneShiftWrite: (documentId: number, cursorPosition: number, tone: string, settings?: Partial<WriteSettings>) => Promise<WriteResult>;
  
  // Rewrite tools
  rewriteText: (text: string, settings: Partial<RewriteSettings>) => Promise<string>;
  
  // Expand tools
  expandText: (text: string, settings: Partial<ExpandSettings>) => Promise<string>;
  
  // Creative tools
  brainstorm: (prompt: string, settings: Partial<BrainstormSettings>) => Promise<string[]>;
  describeScene: (text: string, focus?: string) => Promise<string>;
  visualizeScene: (description: string) => Promise<string>; // Returns image URL
  
  // Quick tools
  quickEdit: (text: string, instruction: string, settings?: Partial<QuickEditSettings>) => Promise<string>;
  quickChat: (message: string, context?: string) => Promise<string>;
  
  // Related words
  getRelatedWords: (word: string, context?: string) => Promise<string[]>;
  getGuidedSuggestions: (prompt: string) => Promise<string[]>;
  
  // Streaming controls
  startStreaming: (streamId: string) => void;
  pauseStreaming: () => void;
  resumeStreaming: () => void;
  stopStreaming: () => void;
  updateStreamingText: (text: string) => void;
  
  // Settings management
  updateWriteSettings: (settings: Partial<WriteSettings>) => void;
  updateRewriteSettings: (settings: Partial<RewriteSettings>) => void;
  updateExpandSettings: (settings: Partial<ExpandSettings>) => void;
  updateBrainstormSettings: (settings: Partial<BrainstormSettings>) => void;
  updateGlobalSettings: (settings: Partial<AISettings>) => void;
  
  // Credit management
  updateCredits: (used: number, remaining?: number) => void;
  checkCredits: () => Promise<{ used: number; remaining: number | null }>;
  
  // Error handling
  setError: (error: string | null) => void;
  clearError: () => void;
}

export const useAIStore = create<AIState>((set, get) => ({
  // Initial state
  isLoading: false,
  error: null,
  
  streaming: {
    isStreaming: false,
    currentText: '',
    streamId: null,
    canPause: false,
    isPaused: false,
  },
  
  lastResult: null,
  
  defaultWriteSettings: {
    creativity_level: 5,
    tone: 'neutral',
    key_details: '',
    card_count: 2,
    card_length: 'medium',
    prose_mode: 'default',
  },
  
  defaultRewriteSettings: {
    style: 'rephrase',
    creativity_level: 5,
    preserve_meaning: true,
  },
  
  defaultExpandSettings: {
    focus: 'sensory_details',
    length_multiplier: 2,
    creativity_level: 5,
  },
  
  defaultBrainstormSettings: {
    category: 'plot_points',
    count: 5,
    creativity_level: 7,
  },
  
  globalSettings: {
    defaultProvider: 'openai',
    defaultModel: 'gpt-4',
    creativity: 0.5,
    responseLength: 'medium',
    writingStyle: 'balanced',
    customInstructions: '',
    autoSave: true,
    showCosts: true,
    enableStreaming: true,
    contextAware: true,
    requestTimeout: 30000,
    maxConcurrentRequests: 3,
    retryAttempts: 2,
    cacheDuration: 300000,
    logRequests: false,
    shareAnalytics: false,
    dataRetention: 30,
    debugMode: false,
    mockMode: false,
  },
  
  creditsUsed: 0,
  creditsRemaining: null,
  
  // Write tools implementation
  autoWrite: async (documentId: number, cursorPosition: number, settings = {}) => {
    const state = get();
    const finalSettings = { ...state.defaultWriteSettings, ...settings };
    
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<WriteResult>(
        'auto_write',
        {
          document_id: documentId,
          cursor_position: cursorPosition,
          settings: finalSettings,
        }
      );
      
      set({ 
        lastResult: result,
        creditsUsed: state.creditsUsed + result.credits_used,
      });
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  guidedWrite: async (documentId: number, userPrompt: string, settings = {}) => {
    const state = get();
    const finalSettings = { ...state.defaultWriteSettings, ...settings };
    
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<WriteResult>(
        'guided_write',
        {
          document_id: documentId,
          user_prompt: userPrompt,
          settings: finalSettings,
        }
      );
      
      set({ 
        lastResult: result,
        creditsUsed: state.creditsUsed + result.credits_used,
      });
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  toneShiftWrite: async (documentId: number, cursorPosition: number, tone: string, settings = {}) => {
    const state = get();
    const finalSettings = { ...state.defaultWriteSettings, ...settings, tone };
    
    try {
      set({ isLoading: true, error: null });
      
      // Note: This command needs to be implemented in the backend
      const result = await invoke<WriteResult>(
        'tone_shift_write',
        {
          document_id: documentId,
          cursor_position: cursorPosition,
          tone,
          settings: finalSettings,
        }
      );
      
      set({ 
        lastResult: result,
        creditsUsed: state.creditsUsed + result.credits_used,
      });
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  rewriteText: async (text: string, settings = {}) => {
    const state = get();
    const finalSettings = { ...state.defaultRewriteSettings, ...settings };
    
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string>(
        'rewrite_text',
        {
          text,
          settings: finalSettings,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  expandText: async (text: string, settings = {}) => {
    const state = get();
    const finalSettings = { ...state.defaultExpandSettings, ...settings };
    
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string>(
        'expand_text',
        {
          text,
          settings: finalSettings,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  brainstorm: async (prompt: string, settings = {}) => {
    const state = get();
    const finalSettings = { ...state.defaultBrainstormSettings, ...settings };
    
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string[]>(
        'brainstorm_ideas',
        {
          prompt,
          settings: finalSettings,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  describeScene: async (text: string, focus?: string) => {
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string>(
        'describe_scene',
        {
          text,
          focus,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  visualizeScene: async (description: string) => {
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string>(
        'visualize_scene',
        {
          description,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  quickEdit: async (text: string, instruction: string, settings = {}) => {
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string>(
        'quick_edit',
        {
          text,
          instruction,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  quickChat: async (message: string, context?: string) => {
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string>(
        'quick_chat',
        {
          message,
          context,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },

  getRelatedWords: async (word: string, context?: string) => {
    try {
      set({ isLoading: true, error: null });
      
      const result = await invoke<string[]>(
        'get_related_words',
        {
          word,
          context,
        }
      );
      
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },

  getGuidedSuggestions: async (prompt: string) => {
    try {
      set({ isLoading: true, error: null });
      const suggestions = await invoke<string[]>('get_guided_suggestions', { prompt });
      return suggestions;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      set({ error: errorMessage });
      throw error;
    } finally {
      set({ isLoading: false });
    }
  },
  
  // Streaming controls
  startStreaming: (streamId: string) => {
    set({
      streaming: {
        isStreaming: true,
        currentText: '',
        streamId,
        canPause: true,
        isPaused: false,
      },
    });
  },
  
  pauseStreaming: () => {
    set((state) => ({
      streaming: {
        ...state.streaming,
        isPaused: true,
      },
    }));
  },
  
  resumeStreaming: () => {
    set((state) => ({
      streaming: {
        ...state.streaming,
        isPaused: false,
      },
    }));
  },
  
  stopStreaming: () => {
    set({
      streaming: {
        isStreaming: false,
        currentText: '',
        streamId: null,
        canPause: false,
        isPaused: false,
      },
    });
  },
  
  updateStreamingText: (text: string) => {
    set((state) => ({
      streaming: {
        ...state.streaming,
        currentText: text,
      },
    }));
  },
  
  // Settings management
  updateWriteSettings: (settings: Partial<WriteSettings>) => {
    set((state) => ({
      defaultWriteSettings: { ...state.defaultWriteSettings, ...settings },
    }));
  },
  
  updateRewriteSettings: (settings: Partial<RewriteSettings>) => {
    set((state) => ({
      defaultRewriteSettings: { ...state.defaultRewriteSettings, ...settings },
    }));
  },
  
  updateExpandSettings: (settings: Partial<ExpandSettings>) => {
    set((state) => ({
      defaultExpandSettings: { ...state.defaultExpandSettings, ...settings },
    }));
  },
  
  updateBrainstormSettings: (settings: Partial<BrainstormSettings>) => {
    set((state) => ({
      defaultBrainstormSettings: { ...state.defaultBrainstormSettings, ...settings },
    }));
  },
  
  updateGlobalSettings: (settings: Partial<AISettings>) => {
    set((state) => ({
      globalSettings: { ...state.globalSettings, ...settings },
    }));
  },
  
  // Credit management
  updateCredits: (used: number, remaining?: number) => {
    set((state) => ({
      creditsUsed: state.creditsUsed + used,
      creditsRemaining: remaining ?? state.creditsRemaining,
    }));
  },
  
  checkCredits: async () => {
    try {
      // Note: get_credit_usage requires a project_id parameter
      const result = await invoke<{
        project_usage: number;
        daily_usage: number;
        monthly_limit: number | null;
        remaining_credits: number | null;
      }>('get_credit_usage', { project_id: '1' }); // Default project for now
      
      set({ 
        creditsUsed: result.project_usage, 
        creditsRemaining: result.remaining_credits 
      });
      return { 
        used: result.project_usage, 
        remaining: result.remaining_credits 
      };
    } catch (error) {
      console.error('Failed to check credits:', error);
      return { used: get().creditsUsed, remaining: get().creditsRemaining };
    }
  },
  
  // Error handling
  setError: (error: string | null) => {
    set({ error });
  },
  
  clearError: () => {
    set({ error: null });
  },
}));

// Convenience hooks for specific AI features
export const useAIWrite = () => {
  const { autoWrite, guidedWrite, toneShiftWrite, isLoading, error } = useAIStore();
  return { autoWrite, guidedWrite, toneShiftWrite, isLoading, error };
};

export const useAIRewrite = () => {
  const { rewriteText, isLoading, error } = useAIStore();
  return { rewriteText, isLoading, error };
};

export const useAIExpand = () => {
  const { expandText, isLoading, error } = useAIStore();
  return { expandText, isLoading, error };
};

export const useAIBrainstorm = () => {
  const { brainstorm, isLoading, error } = useAIStore();
  return { brainstorm, isLoading, error };
};

export const useAIQuickTools = () => {
  const { quickEdit, quickChat, isLoading, error } = useAIStore();
  return { quickEdit, quickChat, isLoading, error };
};

export const useAIStreaming = () => {
  const { 
    streaming, 
    startStreaming, 
    pauseStreaming, 
    resumeStreaming, 
    stopStreaming, 
    updateStreamingText 
  } = useAIStore();
  return { 
    streaming, 
    startStreaming, 
    pauseStreaming, 
    resumeStreaming, 
    stopStreaming, 
    updateStreamingText 
  };
};

export const useAICredits = () => {
  const { creditsUsed, creditsRemaining, updateCredits, checkCredits } = useAIStore();
  return { creditsUsed, creditsRemaining, updateCredits, checkCredits };
};
