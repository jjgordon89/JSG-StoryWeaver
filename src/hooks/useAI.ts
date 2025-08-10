import { useCallback, useEffect, useState } from 'react';
import { useAIStore, WriteSettings, BrainstormSettings, WriteResult, useAIStreaming } from '../stores/aiStore';
import { useCardStore } from '../stores/cardStore';
import { AICard } from '../components/cards/CardSystem';
import { invoke, listen } from '../utils/tauriSafe';

// Re-export useAIStreaming for convenience
export { useAIStreaming };

// Main AI hook that provides a unified interface
export const useAI = () => {
  const aiStore = useAIStore();
  const cardStore = useCardStore();
  
  // Enhanced write function that also creates cards
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
  
  // Generate AI cards from text
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

// Hook for AI writing with streaming support
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
  
  // Start a streaming write operation
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
      const unlistenChunk = await listen('ai_stream_chunk', (event: any) => {
        const chunk = event.payload;
        if (chunk.stream_id === streamId) {
          updateStreamingText(chunk.content);
          setStreamedContent(chunk.content);
          
          if (chunk.is_complete) {
            stopStreaming();
          }
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
          documentId,
          userPrompt,
          settings: { ...useAIStore.getState().defaultWriteSettings, ...settings }
        });
      } else {
        response = await invoke('auto_write_stream', {
          documentId,
          cursorPosition,
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

// Hook for AI text processing (rewrite, expand, etc.)
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

// Hook for AI brainstorming and creative tools
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

// Hook for AI quick tools and chat
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

// Hook for AI settings management
export const useAISettings = () => {
  const {
    defaultWriteSettings,
    defaultRewriteSettings,
    defaultExpandSettings,
    defaultBrainstormSettings,
    updateWriteSettings,
    updateRewriteSettings,
    updateExpandSettings,
    updateBrainstormSettings,
  } = useAIStore();
  
  return {
    settings: {
      write: defaultWriteSettings,
      rewrite: defaultRewriteSettings,
      expand: defaultExpandSettings,
      brainstorm: defaultBrainstormSettings,
    },
    updateSettings: {
      write: updateWriteSettings,
      rewrite: updateRewriteSettings,
      expand: updateExpandSettings,
      brainstorm: updateBrainstormSettings,
    },
  };
};

// Hook for AI credit management
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