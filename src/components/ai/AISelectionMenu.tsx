import React, { useState, useEffect, useRef, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Wand2, 
  Edit3, 
  Expand, 
  Lightbulb, 
  Image, 
  MessageSquare, 
  Zap,
  Settings,
  X,
  Play,
  Pause,
  Square
} from 'lucide-react';
import { useAI, useAIWriteStream, useAITextProcessor, useAICreative } from '../../hooks/useAI';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Textarea } from '../ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { Slider } from '../ui/slider';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { useAICredits } from '../../hooks/useAI';

interface AISelectionMenuProps {
  isOpen: boolean;
  onClose: () => void;
  selectedText?: string;
  cursorPosition?: number;
  documentId?: number;
  onTextInsert?: (text: string) => void;
  onTextReplace?: (text: string) => void;
}

type AITool = 
  | 'write'
  | 'rewrite' 
  | 'expand'
  | 'brainstorm'
  | 'describe'
  | 'visualize'
  | 'quickEdit'
  | 'chat';

interface ToolConfig {
  id: AITool;
  name: string;
  description: string;
  icon: React.ComponentType<{ className?: string }>;
  requiresText: boolean;
  category: 'write' | 'edit' | 'creative' | 'quick';
}

const AI_TOOLS: ToolConfig[] = [
  {
    id: 'write',
    name: 'Write',
    description: 'Generate new content based on context',
    icon: Wand2,
    requiresText: false,
    category: 'write',
  },
  {
    id: 'rewrite',
    name: 'Rewrite',
    description: 'Rephrase and improve selected text',
    icon: Edit3,
    requiresText: true,
    category: 'edit',
  },
  {
    id: 'expand',
    name: 'Expand',
    description: 'Add more detail and depth',
    icon: Expand,
    requiresText: true,
    category: 'edit',
  },
  {
    id: 'brainstorm',
    name: 'Brainstorm',
    description: 'Generate creative ideas',
    icon: Lightbulb,
    requiresText: false,
    category: 'creative',
  },
  {
    id: 'describe',
    name: 'Describe',
    description: 'Create vivid scene descriptions',
    icon: MessageSquare,
    requiresText: true,
    category: 'creative',
  },
  {
    id: 'visualize',
    name: 'Visualize',
    description: 'Generate scene imagery',
    icon: Image,
    requiresText: true,
    category: 'creative',
  },
  {
    id: 'quickEdit',
    name: 'Quick Edit',
    description: 'Make specific changes with instructions',
    icon: Zap,
    requiresText: true,
    category: 'quick',
  },
  {
    id: 'chat',
    name: 'Chat',
    description: 'Ask questions about your writing',
    icon: MessageSquare,
    requiresText: false,
    category: 'quick',
  },
];

export const AISelectionMenu: React.FC<AISelectionMenuProps> = ({
  isOpen,
  onClose,
  selectedText = '',
  cursorPosition = 0,
  documentId = 1,
  onTextInsert,
  onTextReplace,
}) => {
  const [selectedTool, setSelectedTool] = useState<AITool | null>(null);
  const [userPrompt, setUserPrompt] = useState('');
  const [quickEditInstruction, setQuickEditInstruction] = useState('');
  const [showSettings, setShowSettings] = useState(false);
  const [result, setResult] = useState<string>('');
  
  const menuRef = useRef<HTMLDivElement>(null);
  
  // AI hooks
  const { writeWithCards, quickEdit, quickChat, isLoading, error } = useAI();
  const { streaming, streamedContent, startStreamingWrite, stopStreamingWrite } = useAIWriteStream();
  const { processText, processedText, resetProcessedText } = useAITextProcessor();
  const { generateIdeas, generateSceneDescription, generateVisualization, ideas, sceneDescription, visualizationUrl } = useAICreative();
  const { creditsUsed, creditsRemaining, isLowOnCredits } = useAICredits();
  
  // Close menu on outside click
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };
    
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isOpen, onClose]);
  
  // Reset state when menu opens/closes
  useEffect(() => {
    if (!isOpen) {
      setSelectedTool(null);
      setUserPrompt('');
      setQuickEditInstruction('');
      setResult('');
      resetProcessedText();
    }
  }, [isOpen, resetProcessedText]);
  
  const handleToolSelect = useCallback((tool: AITool) => {
    setSelectedTool(tool);
    setResult('');
    resetProcessedText();
  }, [resetProcessedText]);
  
  const handleExecuteTool = useCallback(async () => {
    if (!selectedTool) return;
    
    try {
      let output = '';
      
      switch (selectedTool) {
        case 'write':
          if (streaming.isStreaming) {
            stopStreamingWrite();
          } else {
            const writeResult = await startStreamingWrite(
              documentId,
              cursorPosition,
              userPrompt || undefined
            );
            output = writeResult?.generated_text || streamedContent;
          }
          break;
          
        case 'rewrite':
          output = await processText(selectedText, 'rewrite', {
            style: 'rephrase',
            creativity_level: 5,
            preserve_meaning: true,
          });
          break;
          
        case 'expand':
          output = await processText(selectedText, 'expand', {
            focus: 'sensory_details',
            length_multiplier: 2,
            creativity_level: 5,
          });
          break;
          
        case 'brainstorm':
          const brainstormIdeas = await generateIdeas(userPrompt || selectedText || 'Generate creative ideas', {
            category: 'plot_points',
            count: 5,
            creativity_level: 7,
          });
          output = brainstormIdeas.join('\n\n');
          break;
          
        case 'describe':
          output = await generateSceneDescription(selectedText, 'general');
          break;
          
        case 'visualize':
          const imageUrl = await generateVisualization(selectedText);
          output = `![Generated Scene](${imageUrl})`;
          break;
          
        case 'quickEdit':
          if (!quickEditInstruction.trim()) {
            throw new Error('Please provide an instruction for the quick edit');
          }
          output = await quickEdit(selectedText, quickEditInstruction);
          break;
          
        case 'chat':
          output = await quickChat(userPrompt, selectedText);
          break;
      }
      
      setResult(output);
    } catch (error) {
      console.error('AI tool execution failed:', error);
    }
  }, [selectedTool, documentId, cursorPosition, userPrompt, selectedText, quickEditInstruction, streaming, startStreamingWrite, stopStreamingWrite, streamedContent, processText, generateIdeas, generateSceneDescription, generateVisualization, quickEdit, quickChat]);
  
  const handleInsertResult = useCallback(() => {
    const textToInsert = result || processedText || streamedContent;
    if (textToInsert && onTextInsert) {
      onTextInsert(textToInsert);
      onClose();
    }
  }, [result, processedText, streamedContent, onTextInsert, onClose]);
  
  const handleReplaceResult = useCallback(() => {
    const textToReplace = result || processedText || streamedContent;
    if (textToReplace && onTextReplace) {
      onTextReplace(textToReplace);
      onClose();
    }
  }, [result, processedText, streamedContent, onTextReplace, onClose]);
  
  const getAvailableTools = () => {
    return AI_TOOLS.filter(tool => {
      if (tool.requiresText && !selectedText.trim()) {
        return false;
      }
      return true;
    });
  };
  
  const getCurrentResult = () => {
    return result || processedText || streamedContent || sceneDescription || visualizationUrl || ideas.join('\n\n');
  };
  
  if (!isOpen) return null;
  
  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4"
      >
        <motion.div
          ref={menuRef}
          initial={{ scale: 0.95, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          exit={{ scale: 0.95, opacity: 0 }}
          className="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-hidden"
        >
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-700">
            <div className="flex items-center gap-3">
              <Wand2 className="w-5 h-5 text-blue-500" />
              <h2 className="text-lg font-semibold">AI Writing Assistant</h2>
              {isLowOnCredits && (
                <Badge variant="destructive" className="text-xs">
                  Low Credits
                </Badge>
              )}
            </div>
            <div className="flex items-center gap-2">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setShowSettings(!showSettings)}
              >
                <Settings className="w-4 h-4" />
              </Button>
              <Button variant="ghost" size="sm" onClick={onClose}>
                <X className="w-4 h-4" />
              </Button>
            </div>
          </div>
          
          <div className="flex h-[calc(90vh-80px)]">
            {/* Tool Selection Sidebar */}
            <div className="w-64 border-r border-gray-200 dark:border-gray-700 p-4 overflow-y-auto">
              <div className="space-y-2">
                {getAvailableTools().map((tool) => {
                  const Icon = tool.icon;
                  const isSelected = selectedTool === tool.id;
                  
                  return (
                    <button
                      key={tool.id}
                      onClick={() => handleToolSelect(tool.id)}
                      className={`w-full text-left p-3 rounded-lg transition-colors ${
                        isSelected
                          ? 'bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-700'
                          : 'hover:bg-gray-50 dark:hover:bg-gray-700'
                      }`}
                    >
                      <div className="flex items-start gap-3">
                        <Icon className={`w-5 h-5 mt-0.5 ${
                          isSelected ? 'text-blue-500' : 'text-gray-500'
                        }`} />
                        <div>
                          <div className={`font-medium text-sm ${
                            isSelected ? 'text-blue-700 dark:text-blue-300' : 'text-gray-900 dark:text-gray-100'
                          }`}>
                            {tool.name}
                          </div>
                          <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                            {tool.description}
                          </div>
                        </div>
                      </div>
                    </button>
                  );
                })}
              </div>
              
              {/* Credit Display */}
              <div className="mt-6 p-3 bg-gray-50 dark:bg-gray-700 rounded-lg">
                <div className="text-xs text-gray-600 dark:text-gray-400">
                  Credits Used: {creditsUsed}
                </div>
                {creditsRemaining !== null && (
                  <div className="text-xs text-gray-600 dark:text-gray-400">
                    Remaining: {creditsRemaining}
                  </div>
                )}
              </div>
            </div>
            
            {/* Main Content Area */}
            <div className="flex-1 flex flex-col">
              {selectedTool ? (
                <>
                  {/* Tool Configuration */}
                  <div className="p-4 border-b border-gray-200 dark:border-gray-700">
                    <div className="space-y-4">
                      {/* Selected Text Display */}
                      {selectedText && (
                        <div>
                          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            Selected Text
                          </label>
                          <div className="p-3 bg-gray-50 dark:bg-gray-700 rounded-lg text-sm">
                            {selectedText.length > 200 
                              ? `${selectedText.substring(0, 200)}...` 
                              : selectedText
                            }
                          </div>
                        </div>
                      )}
                      
                      {/* Tool-specific inputs */}
                      {(selectedTool === 'write' || selectedTool === 'brainstorm' || selectedTool === 'chat') && (
                        <div>
                          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            {selectedTool === 'write' ? 'Writing Prompt (optional)' : 
                             selectedTool === 'brainstorm' ? 'Brainstorm Topic' :
                             'Your Message'}
                          </label>
                          <Textarea
                            value={userPrompt}
                            onChange={(e) => setUserPrompt(e.target.value)}
                            placeholder={
                              selectedTool === 'write' ? 'Describe what you want to write about...' :
                              selectedTool === 'brainstorm' ? 'What do you want to brainstorm?' :
                              'Ask a question about your writing...'
                            }
                            className="min-h-[80px]"
                          />
                        </div>
                      )}
                      
                      {selectedTool === 'quickEdit' && (
                        <div>
                          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            Edit Instruction
                          </label>
                          <Input
                            value={quickEditInstruction}
                            onChange={(e) => setQuickEditInstruction(e.target.value)}
                            placeholder="e.g., 'Make this more dramatic' or 'Fix grammar'"
                          />
                        </div>
                      )}
                      
                      {/* Action Buttons */}
                      <div className="flex gap-2">
                        <Button
                          onClick={handleExecuteTool}
                          disabled={isLoading || (selectedTool === 'quickEdit' && !quickEditInstruction.trim())}
                          className="flex items-center gap-2"
                        >
                          {streaming.isStreaming ? (
                            <>
                              <Square className="w-4 h-4" />
                              Stop
                            </>
                          ) : isLoading ? (
                            'Processing...'
                          ) : (
                            <>
                              <Play className="w-4 h-4" />
                              Generate
                            </>
                          )}
                        </Button>
                        
                        {streaming.isStreaming && streaming.canPause && (
                          <Button variant="outline" size="sm">
                            {streaming.isPaused ? (
                              <>
                                <Play className="w-4 h-4" />
                                Resume
                              </>
                            ) : (
                              <>
                                <Pause className="w-4 h-4" />
                                Pause
                              </>
                            )}
                          </Button>
                        )}
                      </div>
                    </div>
                  </div>
                  
                  {/* Results Area */}
                  <div className="flex-1 p-4 overflow-y-auto">
                    {getCurrentResult() ? (
                      <div className="space-y-4">
                        <div className="flex items-center justify-between">
                          <h3 className="font-medium text-gray-900 dark:text-gray-100">
                            Generated Content
                          </h3>
                          <div className="flex gap-2">
                            <Button size="sm" onClick={handleInsertResult}>
                              Insert
                            </Button>
                            {selectedText && (
                              <Button size="sm" variant="outline" onClick={handleReplaceResult}>
                                Replace
                              </Button>
                            )}
                          </div>
                        </div>
                        
                        <div className="p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
                          <div className="whitespace-pre-wrap text-sm">
                            {getCurrentResult()}
                          </div>
                        </div>
                        
                        {streaming.isStreaming && (
                          <div className="text-xs text-gray-500 dark:text-gray-400">
                            Streaming in progress...
                          </div>
                        )}
                      </div>
                    ) : (
                      <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
                        <div className="text-center">
                          <Wand2 className="w-12 h-12 mx-auto mb-4 opacity-50" />
                          <p>Click Generate to create content with AI</p>
                        </div>
                      </div>
                    )}
                    
                    {error && (
                      <div className="mt-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-700 rounded-lg">
                        <div className="text-sm text-red-600 dark:text-red-400">
                          {error}
                        </div>
                      </div>
                    )}
                  </div>
                </>
              ) : (
                <div className="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
                  <div className="text-center">
                    <Wand2 className="w-16 h-16 mx-auto mb-4 opacity-50" />
                    <h3 className="text-lg font-medium mb-2">Choose an AI Tool</h3>
                    <p className="text-sm">
                      Select a tool from the sidebar to get started with AI-powered writing assistance.
                    </p>
                  </div>
                </div>
              )}
            </div>
          </div>
        </motion.div>
      </motion.div>
    </AnimatePresence>
  );
};

export default AISelectionMenu;