import { useCallback, useEffect, useState } from 'react';
import { useAIStore, WriteSettings, BrainstormSettings, WriteResult, useAIStreaming, StreamingEnvelope } from '../stores/aiStore';
import { useCardStore } from '../stores/cardStore';
import { AICard } from '../components/cards/CardSystem';
import { invoke, listen } from '../utils/tauriSafe';

// Re-export useAIStreaming for convenience
export { useAIStreaming };

/**
 * Main AI hook that provides a unified interface for all AI operations in StoryWeaver.
 * 
 * This hook combines AI writing functionality with card generation, providing a seamless
 * experience for writers to generate content and organize it into cards for later reference.
 * 
 * @returns {Object} An object containing all AI functions and state
 * @returns {Function} writeWithCards - Enhanced write function that creates cards from generated content
 * @returns {Function} generateCardsFromText - Generate AI cards from existing text
 * @returns {boolean} isAnyLoading - Whether any AI operation is currently loading
 * @returns {boolean} hasError - Whether there's an active error state
 * 
 * @example
 * ```tsx
 * const { writeWithCards, generateCardsFromText, isAnyLoading } = useAI();
 * 
 * // Generate content with automatic card creation
 * const result = await writeWithCards(documentId, cursorPosition, "Write a dramatic scene", {
 *   card_count: 3,
 *   tone: 'dramatic'
 * });
 * ```
 */
export const useAI = () => {
  const aiStore = useAIStore();
  const cardStore = useCardStore();
  
  /**
   * Enhanced write function that generates AI content and automatically creates cards.
   * 
   * @param {number} documentId - The ID of the document to write in
   * @param {number} cursorPosition - The cursor position for auto-write mode
   * @param {string} [userPrompt] - Optional user prompt for guided writing
   * @param {Partial<WriteSettings>} [settings] - Optional write settings override
   * @returns {Promise<WriteResult>} The result of the write operation
   * 
   * @throws {Error} When the write operation fails
   */
  const writeWithCards = useCallback(async (
    documentId: number,
    cursorPosition: number,
    userPrompt?: string,
    settings?: Partial<WriteSettings>
  ) => {
    try {
      let result: WriteResult;
      
      if (userPrompt) {
        result = await aiStore.guidedWrite(documentId, userPrompt, settings);
      } else {
        result = await aiStore.autoWrite(documentId, cursorPosition, settings);
      }
      
      // Create cards from the generated content if card generation is enabled
      const finalSettings = { ...aiStore.defaultWriteSettings, ...settings };
      if (finalSettings.card_count && finalSettings.card_count > 0) {
        await generateCardsFromText(result.generated_text, documentId, finalSettings.card_count);
      }
      
      return result;
    } catch (error) {
      console.error('Write with cards failed:', error);
      throw error;
    }
  }, [aiStore, cardStore]);
  
  /**
   * Generate AI cards from existing text content.
   * 
   * Creates multiple AI cards based on the provided text, useful for organizing
   * generated content or creating suggestions from existing writing.
   * 
   * @param {string} text - The source text to generate cards from
   * @param {number} documentId - The document ID to associate cards with
   * @param {number} [cardCount=2] - Number of cards to generate
   * @param {number} [projectId] - Project ID (defaults to 1 if not provided)
   * @param {string} [featureType='suggestion'] - The type of feature that generated the cards
   * @returns {Promise<AICard[]>} Array of created AI cards
   * 
   * @throws {Error} When card generation fails
   */
  const generateCardsFromText = useCallback(async (
    text: string,
    documentId: number,
    cardCount: number = 2,
    projectId?: number,
    featureType: string = 'suggestion'
  ) => {
    try {
      const cards: AICard[] = [];
      
      for (let i = 0; i < cardCount; i++) {
        const cardData = {
          project_id: projectId || 1, // This should come from context
          document_id: documentId,
          feature_type: featureType,
          prompt_context: `Generated from text: ${text.substring(0, 100)}...`,
          response_text: `Generated card ${i + 1} based on: ${text.substring(0, 200)}...`,
          model_used: 'gpt-4',
          token_count: 50,
          cost_estimate: 0.001,
          is_stacked: false,
          is_starred: false,
          is_collapsed: false
        };
        
        // Create card via backend
        const createdCard = await invoke('create_ai_card', { cardData });
        if (createdCard.success) {
          cards.push(createdCard.data);
        }
      }
      
      // Refresh cards in store
      await cardStore.fetchCards(projectId || 1, documentId);
      
      return cards;
    } catch (error) {
      console.error('Failed to generate cards:', error);
      throw error;
    }
  }, [cardStore]);
  
  return {
    // Core AI functions
    ...aiStore,
    
    // Enhanced functions
    writeWithCards,
    generateCardsFromText,
    
    // Convenience methods
    isAnyLoading: aiStore.isLoading,
    hasError: !!aiStore.error,
  };
};

/**
 * Hook for AI writing with real-time streaming support.
 * 
 * Provides streaming functionality for AI writing operations, allowing users to see
 * content being generated in real-time with typewriter effects. Supports both
 * auto-write and guided-write modes with streaming.
 * 
 * @returns {Object} Streaming write interface
 * @returns {Object} streaming - Current streaming state
 * @returns {string} streamedContent - The content being streamed
 * @returns {Function} startStreamingWrite - Start a streaming write operation
 * @returns {Function} stopStreamingWrite - Stop the current streaming operation
 * @returns {boolean} isLoading - Whether a write operation is in progress
 * 
 * @example
 * ```tsx
 * const { startStreamingWrite, streamedContent, streaming } = useAIWriteStream();
 * 
 * // Start streaming write
 * const result = await startStreamingWrite(documentId, cursorPosition, "Continue the story");
 * 
 * // Monitor streaming content
 * useEffect(() => {
 *   if (streaming.isStreaming) {
 *     console.log('Current content:', streamedContent);
 *   }
 * }, [streamedContent, streaming.isStreaming]);
 * ```
 */
export const useAIWriteStream = () => {
  const { 
    streaming, 
    startStreaming, 
    stopStreaming, 
    updateStreamingText,
    autoWrite,
    guidedWrite,
    isLoading 
  } = useAIStore();
  
  const [streamedContent, setStreamedContent] = useState<string>('');
  
  /**
   * Start a streaming write operation with real-time content generation.
   * 
   * @param {number} documentId - The document ID to write in
   * @param {number} cursorPosition - Cursor position for auto-write mode
   * @param {string} [userPrompt] - Optional prompt for guided writing
   * @param {Partial<WriteSettings>} [settings] - Optional settings override
   * @returns {Promise<WriteResult>} Promise that resolves when streaming completes
   * 
   * @throws {Error} When streaming fails to start or encounters an error
   */
  const startStreamingWrite = useCallback(async (
    documentId: number,
    cursorPosition: number,
    userPrompt?: string,
    settings?: Partial<WriteSettings>
  ) => {
    const streamId = `write_${Date.now()}`;
    startStreaming(streamId);
    setStreamedContent('');
    
    try {
      // Set up event listeners for streaming
            const unlistenChunk = await listen('ai_stream_chunk', (event: { payload: StreamingEnvelope }) => {
              const { type, payload } = event.payload;
      
              if (payload.stream_id !== streamId) return;
      
              switch (type) {
                case 'chunk':
                  updateStreamingText(payload.content);
                  setStreamedContent(prev => prev + payload.content);
                  break;
                case 'complete':
                  stopStreaming();
                  break;
                case 'error':
                  console.error('Streaming error:', payload);
                  stopStreaming();
                  break;
              }
            });
      
      const unlistenError = await listen('ai_stream_error', (event: any) => {
        console.error('Streaming error:', event.payload);
        stopStreaming();
      });
      
      // Clean up listeners when streaming stops
      const cleanup = () => {
        unlistenChunk?.();
        unlistenError?.();
      };
      
      // Start the streaming command
      let response: any;
      if (userPrompt) {
        response = await invoke('guided_write_stream', {
          document_id: documentId,
          user_prompt: userPrompt,
          settings: { ...useAIStore.getState().defaultWriteSettings, ...settings }
        });
      } else {
        response = await invoke('auto_write_stream', {
          document_id: documentId,
          cursor_position: cursorPosition,
          settings: { ...useAIStore.getState().defaultWriteSettings, ...settings }
        });
      }
      
      if (!response.success) {
        cleanup();
        throw new Error('Failed to start streaming');
      }
      
      // Return a promise that resolves when streaming is complete
      return new Promise<WriteResult>((resolve) => {
        const checkComplete = () => {
          if (!streaming.isStreaming) {
            cleanup();
            resolve({
                generated_text: streamedContent,
                tokens_used: Math.floor(streamedContent.length / 4),
                credits_used: Math.floor(streamedContent.length / 10),
                word_count: streamedContent.split(' ').length
            });
          } else {
            setTimeout(checkComplete, 100);
          }
        };
        checkComplete();
      });
    } catch (error) {
      stopStreaming();
      throw error;
    }
  }, [streaming, startStreaming, stopStreaming, updateStreamingText, autoWrite, guidedWrite]);
  
  const stopStreamingWrite = useCallback(() => {
    stopStreaming();
    setStreamedContent('');
  }, [stopStreaming]);
  
  return {
    streaming,
    streamedContent,
    startStreamingWrite,
    stopStreamingWrite,
    isLoading,
  };
};

/**
 * Hook for AI text processing operations like rewrite, expand, and quick edit.
 * 
 * Provides functionality to process existing text through various AI operations,
 * maintaining state for both original and processed text for comparison.
 * 
 * @returns {Object} Text processing interface
 * @returns {Function} processText - Process text with specified operation
 * @returns {string} processedText - The result of the last processing operation
 * @returns {string} originalText - The original text that was processed
 * @returns {Function} resetProcessedText - Clear processed text state
 * @returns {boolean} isLoading - Whether a processing operation is active
 * @returns {string|null} error - Current error state, if any
 * 
 * @example
 * ```tsx
 * const { processText, processedText, originalText } = useAITextProcessor();
 * 
 * // Rewrite text with specific tone
 * await processText("Original text", "rewrite", { tone: "formal" });
 * 
 * // Expand text with more detail
 * await processText("Brief text", "expand", { expansion_level: "detailed" });
 * ```
 */
export const useAITextProcessor = () => {
  const { 
    rewriteText, 
    expandText, 
    quickEdit, 
    isLoading, 
    error,
    defaultRewriteSettings,
    defaultExpandSettings 
  } = useAIStore();
  

  
  const [processedText, setProcessedText] = useState<string>('');
  const [originalText, setOriginalText] = useState<string>('');
  
  /**
   * Process text using the specified AI operation.
   * 
   * @param {string} text - The text to process
   * @param {'rewrite' | 'expand' | 'quickEdit'} operation - The type of processing to perform
   * @param {any} [settings] - Operation-specific settings
   * @returns {Promise<string>} The processed text result
   * 
   * @throws {Error} When the processing operation fails or settings are invalid
   */
  const processText = useCallback(async (
    text: string,
    operation: 'rewrite' | 'expand' | 'quickEdit',
    settings?: any
  ) => {
    setOriginalText(text);
    
    try {
      let result: string;
      
      switch (operation) {
        case 'rewrite':
          result = await rewriteText(text, settings);
          break;
        case 'expand':
          result = await expandText(text, settings);
          break;
        case 'quickEdit':
          if (!settings?.instruction) {
            throw new Error('Quick edit requires an instruction');
          }
          result = await quickEdit(text, settings.instruction, settings);
          break;
        default:
          throw new Error(`Unknown operation: ${operation}`);
      }
      
      setProcessedText(result);
      return result;
    } catch (error) {
      console.error(`Text processing failed for ${operation}:`, error);
      throw error;
    }
  }, [rewriteText, expandText, quickEdit]);
  
  const resetProcessedText = useCallback(() => {
    setProcessedText('');
    setOriginalText('');
  }, []);
  
  return {
    processText,
    processedText,
    originalText,
    resetProcessedText,
    isLoading,
    error,
    defaultRewriteSettings,
    defaultExpandSettings,
  };
};

/**
 * Hook for AI brainstorming and creative tools.
 * 
 * Provides creative AI functionality including idea generation, scene description,
 * and visualization tools to help writers with creative processes.
 * 
 * @returns {Object} Creative AI interface
 * @returns {Function} generateIdeas - Generate brainstorming ideas from a prompt
 * @returns {Function} generateSceneDescription - Create detailed scene descriptions
 * @returns {Function} generateVisualization - Generate visual representations
 * @returns {string[]} ideas - Array of generated ideas
 * @returns {string} sceneDescription - Generated scene description
 * @returns {string} visualizationUrl - URL for generated visualization
 * @returns {Function} clearCreativeResults - Clear all creative results
 * 
 * @example
 * ```tsx
 * const { generateIdeas, ideas, generateSceneDescription } = useAICreative();
 * 
 * // Generate story ideas
 * await generateIdeas("Fantasy adventure with dragons", { count: 5 });
 * 
 * // Create scene description
 * await generateSceneDescription("A dark forest at midnight", "atmosphere");
 * ```
 */
export const useAICreative = () => {
  const { 
    brainstorm, 
    describeScene, 
    visualizeScene, 
    isLoading, 
    error,
    defaultBrainstormSettings 
  } = useAIStore();
  
  const [ideas, setIdeas] = useState<string[]>([]);
  const [sceneDescription, setSceneDescription] = useState<string>('');
  const [visualizationUrl, setVisualizationUrl] = useState<string>('');
  
  const generateIdeas = useCallback(async (
    prompt: string,
    settings?: Partial<BrainstormSettings>
  ) => {
    try {
      const result = await brainstorm(prompt, settings || {});
      setIdeas(result);
      return result;
    } catch (error) {
      console.error('Brainstorming failed:', error);
      throw error;
    }
  }, [brainstorm]);
  
  const generateSceneDescription = useCallback(async (
    text: string,
    focus?: string
  ) => {
    try {
      const result = await describeScene(text, focus);
      setSceneDescription(result);
      return result;
    } catch (error) {
      console.error('Scene description failed:', error);
      throw error;
    }
  }, [describeScene]);
  
  const generateVisualization = useCallback(async (
    description: string
  ) => {
    try {
      const result = await visualizeScene(description);
      setVisualizationUrl(result);
      return result;
    } catch (error) {
      console.error('Visualization failed:', error);
      throw error;
    }
  }, [visualizeScene]);
  
  const clearCreativeResults = useCallback(() => {
    setIdeas([]);
    setSceneDescription('');
    setVisualizationUrl('');
  }, []);
  
  return {
    generateIdeas,
    generateSceneDescription,
    generateVisualization,
    ideas,
    sceneDescription,
    visualizationUrl,
    clearCreativeResults,
    isLoading,
    error,
    defaultBrainstormSettings,
  };
};

/**
 * Hook for AI quick tools and chat functionality.
 * 
 * Provides quick AI interactions including chat and quick editing tools,
 * maintaining chat history for context-aware conversations.
 * 
 * @returns {Object} Quick tools interface
 * @returns {Function} quickEdit - Perform quick text edits
 * @returns {Function} sendQuickChat - Send a message to AI chat
 * @returns {Array} chatHistory - Array of chat messages with roles and timestamps
 * @returns {Function} clearChatHistory - Clear the chat history
 * 
 * @example
 * ```tsx
 * const { sendQuickChat, chatHistory, quickEdit } = useAIQuickTools();
 * 
 * // Quick chat interaction
 * await sendQuickChat("How can I improve this dialogue?", selectedText);
 * 
 * // Quick text edit
 * await quickEdit("Make this more dramatic", selectedText);
 * ```
 */
export const useAIQuickTools = () => {
  const { quickEdit, quickChat, isLoading, error } = useAIStore();
  
  const [chatHistory, setChatHistory] = useState<Array<{ role: 'user' | 'assistant'; content: string; timestamp: string }>>([]);
  
  const sendQuickChat = useCallback(async (
    message: string,
    context?: string
  ) => {
    // Add user message to history
    const userMessage = {
      role: 'user' as const,
      content: message,
      timestamp: new Date().toISOString(),
    };
    setChatHistory(prev => [...prev, userMessage]);
    
    try {
      const response = await quickChat(message, context);
      
      // Add assistant response to history
      const assistantMessage = {
        role: 'assistant' as const,
        content: response,
        timestamp: new Date().toISOString(),
      };
      setChatHistory(prev => [...prev, assistantMessage]);
      
      return response;
    } catch (error) {
      console.error('Quick chat failed:', error);
      throw error;
    }
  }, [quickChat]);
  
  const clearChatHistory = useCallback(() => {
    setChatHistory([]);
  }, []);
  
  return {
    quickEdit,
    sendQuickChat,
    chatHistory,
    clearChatHistory,
    isLoading,
    error,
  };
};

/**
 * Hook for AI settings management.
 * 
 * Provides access to all AI-related settings including write, rewrite, expand,
 * brainstorm, and global settings with update functions.
 * 
 * @returns {Object} Settings management interface
 * @returns {Object} settings - All current AI settings organized by category
 * @returns {Object} updateSettings - Functions to update each category of settings
 * 
 * @example
 * ```tsx
 * const { settings, updateSettings } = useAISettings();
 * 
 * // Update write settings
 * updateSettings.write({ tone: 'formal', length: 'medium' });
 * 
 * // Update global settings
 * updateSettings.global({ provider: 'openai', model: 'gpt-4' });
 * ```
 */
export const useAISettings = () => {
  const {
    defaultWriteSettings,
    defaultRewriteSettings,
    defaultExpandSettings,
    defaultBrainstormSettings,
    globalSettings,
    updateWriteSettings,
    updateRewriteSettings,
    updateExpandSettings,
    updateBrainstormSettings,
    updateGlobalSettings,
  } = useAIStore();
  
  return {
    settings: {
      write: defaultWriteSettings,
      rewrite: defaultRewriteSettings,
      expand: defaultExpandSettings,
      brainstorm: defaultBrainstormSettings,
      ...globalSettings,
    },
    updateSettings: {
      write: updateWriteSettings,
      rewrite: updateRewriteSettings,
      expand: updateExpandSettings,
      brainstorm: updateBrainstormSettings,
      global: updateGlobalSettings,
    },
  };
};

/**
 * Hook for AI credit management and monitoring.
 * 
 * Manages AI usage credits, providing real-time credit information and
 * automatic refresh functionality. Includes helpful utilities for
 * credit-based UI decisions.
 * 
 * @returns {Object} Credit management interface
 * @returns {number} creditsUsed - Total credits used
 * @returns {number|null} creditsRemaining - Remaining credits (null for unlimited)
 * @returns {Function} updateCredits - Manually update credit count
 * @returns {Function} refreshCredits - Refresh credits from server
 * @returns {boolean} isCheckingCredits - Whether credits are being refreshed
 * @returns {boolean} hasUnlimitedCredits - Whether user has unlimited credits
 * @returns {boolean} isLowOnCredits - Whether user is low on credits (< 100)
 * 
 * @example
 * ```tsx
 * const { creditsRemaining, isLowOnCredits, refreshCredits } = useAICredits();
 * 
 * // Check if user can perform operation
 * if (isLowOnCredits) {
 *   showCreditWarning();
 * }
 * 
 * // Refresh credits after purchase
 * await refreshCredits();
 * ```
 */
export const useAICredits = () => {
  const { creditsUsed, creditsRemaining, updateCredits, checkCredits } = useAIStore();
  const [isCheckingCredits, setIsCheckingCredits] = useState(false);
  
  const refreshCredits = useCallback(async () => {
    setIsCheckingCredits(true);
    try {
      await checkCredits();
    } catch (error) {
      console.error('Failed to refresh credits:', error);
    } finally {
      setIsCheckingCredits(false);
    }
  }, [checkCredits]);
  
  // Auto-refresh credits on mount
  useEffect(() => {
    refreshCredits();
  }, [refreshCredits]);
  
  return {
    creditsUsed,
    creditsRemaining,
    updateCredits,
    refreshCredits,
    isCheckingCredits,
    hasUnlimitedCredits: creditsRemaining === null,
    isLowOnCredits: creditsRemaining !== null && creditsRemaining < 100,
  };
};

export default useAI;
