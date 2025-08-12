import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  Zap, 
  MessageSquare, 
  RefreshCw, 
  Expand, 
  Lightbulb, 
  Eye, 
  PenTool,
  Check,
  X,
  ArrowRight,
  Sparkles
} from 'lucide-react';
import { Button } from '../../ui/components/common';
import { Input } from '../../ui/components/common';
import { Card, CardContent } from '../../ui/components/common';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { useAI, useAITextProcessor, useAICreative, useAIQuickTools, useAICredits, useAISettings } from '../../hooks/useAI';

// Cost estimation utilities
import {
  estimateTokensFromText,
  estimateExpectedOutputTokensForWrite,
  estimateExpectedOutputTokensForExpand,
  estimateOperationCreditsWithModel
} from '../../utils/aiCost';

interface AIQuickToolsProps {
  selectedText?: string;
  cursorPosition?: number;
  documentId?: number;
  onInsertText?: (text: string) => void;
  onReplaceText?: (text: string) => void;
  onClose?: () => void;
  className?: string;
}

type QuickAction = {
  id: string;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  description: string;
  shortcut?: string;
  requiresSelection?: boolean;
  requiresPrompt?: boolean;
  color: string;
};

const quickActions: QuickAction[] = [
  {
    id: 'continue',
    label: 'Continue Writing',
    icon: PenTool,
    description: 'Continue from current position',
    shortcut: 'Ctrl+Enter',
    color: 'blue'
  },
  {
    id: 'improve',
    label: 'Improve',
    icon: Sparkles,
    description: 'Enhance selected text',
    requiresSelection: true,
    color: 'green'
  },
  {
    id: 'rewrite',
    label: 'Rewrite',
    icon: RefreshCw,
    description: 'Rewrite in different style',
    requiresSelection: true,
    color: 'purple'
  },
  {
    id: 'expand',
    label: 'Expand',
    icon: Expand,
    description: 'Add more detail',
    requiresSelection: true,
    color: 'orange'
  },
  {
    id: 'summarize',
    label: 'Summarize',
    icon: MessageSquare,
    description: 'Create a summary',
    requiresSelection: true,
    color: 'teal'
  },
  {
    id: 'brainstorm',
    label: 'Brainstorm',
    icon: Lightbulb,
    description: 'Generate ideas',
    requiresPrompt: true,
    color: 'yellow'
  },
  {
    id: 'describe',
    label: 'Describe Scene',
    icon: Eye,
    description: 'Create scene description',
    requiresPrompt: true,
    color: 'indigo'
  },
  {
    id: 'quickEdit',
    label: 'Quick Edit',
    icon: Zap,
    description: 'Fast text modification',
    requiresSelection: true,
    requiresPrompt: true,
    color: 'red'
  }
];

export const AIQuickTools: React.FC<AIQuickToolsProps> = ({
  selectedText = '',
  cursorPosition,
  documentId,
  onInsertText,
  onReplaceText,
  onClose,
  className = ''
}) => {
  const [selectedAction, setSelectedAction] = useState<string | null>(null);
  const [prompt, setPrompt] = useState('');
  const [isExecuting, setIsExecuting] = useState(false);
  const [result, setResult] = useState('');
  const [showPromptInput, setShowPromptInput] = useState(false);
  
  const inputRef = useRef<HTMLInputElement>(null);

  
  // Hooks
  const { autoWrite } = useAI();
  const { processText } = useAITextProcessor();
  const { generateIdeas, generateSceneDescription } = useAICreative();
  const { quickEdit } = useAIQuickTools();
  const { creditsRemaining } = useAICredits();
  const { settings } = useAISettings();
  
  const hasSelection = selectedText.length > 0;
  const availableActions = quickActions.filter(action => 
    !action.requiresSelection || hasSelection
  );
  
  // Focus input when prompt is shown
  useEffect(() => {
    if (showPromptInput && inputRef.current) {
      inputRef.current.focus();
    }
  }, [showPromptInput]);
  
  // Handle action selection
  const handleActionSelect = (actionId: string) => {
    const action = quickActions.find(a => a.id === actionId);
    if (!action) return;
    
    setSelectedAction(actionId);
    
    if (action.requiresPrompt) {
      setShowPromptInput(true);
    } else {
      executeAction(actionId);
    }
  };
  
  // Execute the selected action
  const executeAction = async (actionId: string, userPrompt?: string) => {
    setIsExecuting(true);
    
    try {
      let response = '';
      
      switch (actionId) {
        case 'continue':
          const writeResult = await autoWrite(documentId || 0, cursorPosition || 0, {
            tone: 'narrative',
            creativity_level: 7,
            key_details: '',
            card_count: 1
          });
          response = writeResult.generated_text;
          break;
          
        case 'improve':
          response = await processText(selectedText, 'rewrite', {
            instructions: 'Improve the quality, clarity, and flow of this text',
            tone: 'professional',
            style: 'polished'
          });
          break;
          
        case 'rewrite':
          response = await processText(selectedText, 'rewrite', {
            instructions: 'Rewrite this text with a different style while maintaining the core meaning',
            tone: 'creative',
            style: 'varied'
          });
          break;
          
        case 'expand':
          response = await processText(selectedText, 'expand', {
            instructions: 'Expand this text with more detail, description, and depth',
            targetLength: 3
          });
          break;
          
        case 'summarize':
          response = await processText(selectedText, 'rewrite', {
            instructions: 'Create a concise summary of this text, capturing the key points',
            tone: 'professional',
            style: 'concise'
          });
          break;
          
        case 'brainstorm':
          const ideas = await generateIdeas(userPrompt || prompt, {
            category: 'plot_points',
            count: 5,
            creativity_level: 5
          });
          response = ideas.join('\n\n');
          break;
          
        case 'describe':
          response = await generateSceneDescription(userPrompt || prompt, 'general');
          break;
          
        case 'quickEdit':
          response = await quickEdit(selectedText, userPrompt || prompt);
          break;
      }
      
      setResult(response);
      
    } catch (error) {
      console.error('Quick action failed:', error);
    } finally {
      setIsExecuting(false);
    }
  };
  
  // Handle prompt submission
  const handlePromptSubmit = () => {
    if (selectedAction && prompt.trim()) {
      executeAction(selectedAction, prompt);
      setShowPromptInput(false);
    }
  };
  
  // Handle result actions
  const handleAccept = () => {
    if (result) {
      const action = quickActions.find(a => a.id === selectedAction);
      if (action?.requiresSelection && hasSelection) {
        onReplaceText?.(result);
      } else {
        onInsertText?.(result);
      }
      onClose?.();
    }
  };
  
  const handleReject = () => {
    setResult('');
    setSelectedAction(null);
    setPrompt('');
    setShowPromptInput(false);
  };
  
  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (result) {
          handleReject();
        } else {
          onClose?.();
        }
      } else if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
        if (showPromptInput) {
          handlePromptSubmit();
        } else if (result) {
          handleAccept();
        }
      }
    };
    
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [result, showPromptInput, prompt]);
  
  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, scale: 0.95, y: 10 }}
        animate={{ opacity: 1, scale: 1, y: 0 }}
        exit={{ opacity: 0, scale: 0.95, y: 10 }}
        transition={{ duration: 0.15 }}
        className={`fixed z-50 ${className}`}
      >
        <Card className="w-80 shadow-lg border-2">
          <CardContent className="p-4">
            {/* Header */}
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <Zap className="w-4 h-4 text-blue-500" />
                <span className="font-medium text-sm">Quick AI Tools</span>
              </div>
              
              <div className="flex items-center gap-2">
                <Badge variant="outline" className="text-xs">
                  {creditsRemaining || 'Unlimited'} credits
                </Badge>
                <Button variant="ghost" size="sm" onClick={onClose}>
                  <X className="w-3 h-3" />
                </Button>
              </div>
            </div>
            
            {/* Selection Info */}
            {hasSelection && (
              <div className="mb-4 p-2 bg-blue-50 dark:bg-blue-900/20 rounded text-xs">
                <span className="text-blue-600 dark:text-blue-400 font-medium">
                  Selected: 
                </span>
                <span className="text-blue-700 dark:text-blue-300">
                  "{selectedText.slice(0, 50)}{selectedText.length > 50 ? '...' : ''}"
                </span>
              </div>
            )}
            
            {/* Prompt Input */}
            <AnimatePresence>
              {showPromptInput && (
                <motion.div
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: 'auto' }}
                  exit={{ opacity: 0, height: 0 }}
                  className="mb-4 space-y-2"
                >
                  <Input
                    ref={inputRef}
                    value={prompt}
                    onChange={(e) => setPrompt(e.target.value)}
                    placeholder="Enter your prompt..."
                    onKeyDown={(e) => {
                      if (e.key === 'Enter') {
                        e.preventDefault();
                        handlePromptSubmit();
                      }
                    }}
                    className="text-sm"
                  />
                  <div className="flex gap-2">
                    <Button size="sm" onClick={handlePromptSubmit} disabled={!prompt.trim()}>
                      <ArrowRight className="w-3 h-3 mr-1" />
                      Execute
                    </Button>
                    <Button size="sm" variant="outline" onClick={() => setShowPromptInput(false)}>
                      Cancel
                    </Button>
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
            
            {/* Result Display */}
            <AnimatePresence>
              {result && (
                <motion.div
                  initial={{ opacity: 0, height: 0 }}
                  animate={{ opacity: 1, height: 'auto' }}
                  exit={{ opacity: 0, height: 0 }}
                  className="mb-4 space-y-3"
                >
                  <Separator />
                  
                  <div className="space-y-2">
                    <div className="text-xs font-medium text-gray-600 dark:text-gray-400">
                      Result:
                    </div>
                    <div className="p-3 bg-gray-50 dark:bg-gray-800 rounded text-sm max-h-32 overflow-y-auto">
                      {result}
                    </div>
                  </div>
                  
                  <div className="flex gap-2">
                    <Button size="sm" onClick={handleAccept} className="flex-1">
                      <Check className="w-3 h-3 mr-1" />
                      Accept
                    </Button>
                    <Button size="sm" variant="outline" onClick={handleReject}>
                      <X className="w-3 h-3 mr-1" />
                      Reject
                    </Button>
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
            
            {/* Action Grid */}
            {!result && !showPromptInput && (
              <div className="grid grid-cols-2 gap-2">
                {availableActions.map((action) => {
                  const Icon = action.icon;
                  const isDisabled = isExecuting || (action.requiresSelection && !hasSelection);

                  // Per-action credit estimate
                  const modelName = (settings as any)?.defaultModel || settings?.write?.prose_mode;
                  let inputTokens = 0;
                  let outputTokens = 0;

                  switch (action.id) {
                    case 'continue': {
                      // Heuristic: write one medium card when continuing
                      inputTokens = 0;
                      outputTokens = estimateExpectedOutputTokensForWrite({
                        card_length: settings?.write?.card_length ?? 'medium',
                        card_count: 1
                      });
                      break;
                    }
                    case 'improve':
                    case 'rewrite': {
                      inputTokens = estimateTokensFromText(selectedText);
                      outputTokens = inputTokens; // similar size rewrite
                      break;
                    }
                    case 'expand': {
                      inputTokens = estimateTokensFromText(selectedText);
                      const lm = (settings as any)?.expand?.length_multiplier ?? 2;
                      outputTokens = estimateExpectedOutputTokensForExpand(inputTokens, lm);
                      break;
                    }
                    case 'summarize': {
                      inputTokens = estimateTokensFromText(selectedText);
                      outputTokens = Math.ceil(inputTokens * 0.5);
                      break;
                    }
                    case 'brainstorm':
                    case 'describe': {
                      inputTokens = estimateTokensFromText(prompt);
                      outputTokens = 200; // single-response heuristic
                      break;
                    }
                    case 'quickEdit': {
                      inputTokens = estimateTokensFromText(selectedText) + estimateTokensFromText(prompt);
                      outputTokens = estimateTokensFromText(selectedText);
                      break;
                    }
                    default: {
                      inputTokens = 0;
                      outputTokens = 0;
                    }
                  }

                  const estimatedCost = estimateOperationCreditsWithModel(inputTokens, outputTokens, modelName);
                  
                  return (
                    <motion.div
                      key={action.id}
                      whileHover={{ scale: 1.02 }}
                      whileTap={{ scale: 0.98 }}
                    >
                      <Button
                        variant="outline"
                        disabled={isDisabled}
                        onClick={() => handleActionSelect(action.id)}
                        className="w-full h-auto p-3 flex flex-col items-start gap-1 text-left"
                      >
                        <div className="flex items-center gap-2 w-full">
                          <Icon className={`w-4 h-4 text-${action.color}-500`} />
                          <span className="text-sm font-medium">{action.label}</span>
                        </div>
                        <span className="text-xs text-gray-500 dark:text-gray-400">
                          {action.description}
                        </span>
                        {estimatedCost > 0 && (
                          <Badge variant="secondary" className="text-xs mt-1">
                            ~{estimatedCost} credits
                          </Badge>
                        )}
                      </Button>
                    </motion.div>
                  );
                })}
              </div>
            )}
            
            {/* Loading State */}
            {isExecuting && (
              <div className="flex items-center justify-center py-8">
                <div className="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
                  <motion.div
                    animate={{ rotate: 360 }}
                    transition={{ duration: 1, repeat: Infinity, ease: 'linear' }}
                  >
                    <Sparkles className="w-4 h-4" />
                  </motion.div>
                  Generating...
                </div>
              </div>
            )}
            
            {/* Keyboard Shortcuts */}
            {!result && !showPromptInput && (
              <div className="mt-4 pt-3 border-t text-xs text-gray-500 dark:text-gray-400">
                <div className="flex justify-between">
                  <span>Esc to close</span>
                  <span>Ctrl+Enter to continue</span>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </motion.div>
    </AnimatePresence>
  );
};

export default AIQuickTools;