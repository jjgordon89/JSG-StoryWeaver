import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  PenTool, 
  RefreshCw, 
  Expand, 
  MessageSquare, 
  Lightbulb, 
  Eye, 
  Zap, 
  Settings, 
  X, 
  ChevronDown, 
  ChevronUp,
  Copy,
  Check,
  RotateCcw
} from 'lucide-react';
import { Button } from '../../ui/components/common';
import { Textarea, Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../ui/components/common';
import { Slider } from '../ui/slider';
import { Card, CardContent, CardHeader, CardTitle } from '../../ui/components/common';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { estimateTokensFromText, estimateExpectedOutputTokensForWrite, estimateExpectedOutputTokensForExpand, estimateOperationCredits, estimateOperationCreditsWithModel } from '../../utils/aiCost';

import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '../ui/collapsible';
import { useAI, useAIWriteStream, useAITextProcessor, useAICreative, useAISettings, useAICredits } from '../../hooks/useAI';
import { StreamingText } from './StreamingText';
import { useCards } from '../../hooks/useCards';

interface AIWritingPanelProps {
  selectedText?: string;
  documentId?: string;
  projectId?: string;
  onInsertText?: (text: string) => void;
  onReplaceText?: (text: string) => void;
  className?: string;
}

type AITool = 'write' | 'rewrite' | 'expand' | 'brainstorm' | 'describe' | 'visualize' | 'quickEdit' | 'chat';

const toolConfig = {
  write: {
    icon: PenTool,
    title: 'Write',
    description: 'Generate new content based on context',
    color: 'blue',
    requiresPrompt: true,
    requiresSelection: false
  },
  rewrite: {
    icon: RefreshCw,
    title: 'Rewrite',
    description: 'Rewrite selected text with different style',
    color: 'green',
    requiresPrompt: false,
    requiresSelection: true
  },
  expand: {
    icon: Expand,
    title: 'Expand',
    description: 'Add more detail to selected text',
    color: 'purple',
    requiresPrompt: false,
    requiresSelection: true
  },
  brainstorm: {
    icon: Lightbulb,
    title: 'Brainstorm',
    description: 'Generate creative ideas and concepts',
    color: 'yellow',
    requiresPrompt: true,
    requiresSelection: false
  },
  describe: {
    icon: MessageSquare,
    title: 'Describe',
    description: 'Create detailed scene descriptions',
    color: 'indigo',
    requiresPrompt: true,
    requiresSelection: false
  },
  visualize: {
    icon: Eye,
    title: 'Visualize',
    description: 'Generate visual scene descriptions',
    color: 'pink',
    requiresPrompt: true,
    requiresSelection: false
  },
  quickEdit: {
    icon: Zap,
    title: 'Quick Edit',
    description: 'Fast text improvements',
    color: 'orange',
    requiresPrompt: true,
    requiresSelection: true
  },
  chat: {
    icon: MessageSquare,
    title: 'Chat',
    description: 'Interactive AI conversation',
    color: 'teal',
    requiresPrompt: true,
    requiresSelection: false
  }
};

export const AIWritingPanel: React.FC<AIWritingPanelProps> = ({
  selectedText = '',
  documentId,
  projectId,
  onInsertText,
  onReplaceText,
  className = ''
}) => {
  const [activeTool, setActiveTool] = useState<AITool>('write');
  const [prompt, setPrompt] = useState('');
  const [isExpanded, setIsExpanded] = useState(true);
  const [showSettings, setShowSettings] = useState(false);
  const [result, setResult] = useState('');
  const [copied, setCopied] = useState(false);
  const [chatHistory, setChatHistory] = useState<Array<{role: 'user' | 'assistant', content: string}>>([]);
  
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  
  // Hooks
  const { autoWrite, guidedWrite } = useAI();
  const { streaming, startStreamingWrite, stopStreamingWrite } = useAIWriteStream();
  const { processText } = useAITextProcessor();
  const { generateIdeas, generateSceneDescription, generateVisualization } = useAICreative();
  const { settings, updateSettings } = useAISettings();
  const { creditsRemaining } = useAICredits();
  const { addCard } = useCards({ projectId: parseInt(projectId || '0', 10), documentId: parseInt(documentId || '0', 10) });
  
  const currentTool = toolConfig[activeTool];
  const canExecute = currentTool.requiresPrompt ? prompt.trim().length > 0 : true;
  const hasSelection = selectedText.length > 0;
  const needsSelection = currentTool.requiresSelection && !hasSelection;
  
  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [prompt]);
  
  // Handle tool execution
  const handleExecute = async () => {
    if (!canExecute || needsSelection) return;
    
    try {
      let response = '';
      
      switch (activeTool) {
        case 'write':
          if (settings.write.prose_mode === 'streaming') {
            if (documentId) {
              startStreamingWrite(parseInt(documentId, 10), 0)
                .then((writeResult) => {
                  handleStreamingComplete(writeResult.generated_text);
                })
                .catch((err) => {
                  console.error('Streaming write failed:', err);
                });
            }
            return;
          } else {
            const writeResult = await autoWrite(
              parseInt(documentId!, 10),
              0, // cursor position
              {
                creativity_level: settings.write.creativity_level,
                tone: settings.write.tone,
                key_details: settings.write.key_details || '',
                card_count: settings.write.card_count,
                card_length: settings.write.card_length
              }
            );
            response = writeResult.generated_text;
          }
          break;
          
        case 'rewrite':
          response = await processText(selectedText, 'rewrite', {
            style: settings.rewrite.style || 'rephrase',
            creativity_level: settings.rewrite.creativity_level || 5,
            preserve_meaning: settings.rewrite.preserve_meaning ?? true
          });
          break;
          
        case 'expand':
          response = await processText(selectedText, 'expand', {
            focus: settings.expand.focus || 'sensory_details',
            length_multiplier: settings.expand.length_multiplier || 2,
            creativity_level: settings.expand.creativity_level || 5
          });
          break;
          
        case 'brainstorm':
          const brainstormResult = await generateIdeas(prompt, {
            category: settings.brainstorm.category || 'plot_points',
            count: settings.brainstorm.count || 5,
            creativity_level: settings.brainstorm.creativity_level || 5
          });
          response = brainstormResult.join('\n');
          break;
          
        case 'describe':
          response = await generateSceneDescription(
            prompt,
            settings.write.tone
          );
          break;
          
        case 'visualize':
          response = await generateVisualization(
            prompt
          );
          break;
          
        case 'quickEdit':
          response = await processText(selectedText, 'quickEdit', {
            instruction: prompt
          });
          break;
          
        case 'chat':
          const newHistory = [...chatHistory, { role: 'user' as const, content: prompt }];
          setChatHistory(newHistory);
          
          // For chat, we'll use guided write with conversation context
          const writeResult = await guidedWrite(
            parseInt(documentId!, 10),
            prompt,
            settings.write
          );
          response = writeResult.generated_text;
          
          setChatHistory([...newHistory, { role: 'assistant', content: response }]);
          break;
      }
      
      setResult(response);
      
      // Add to cards if enabled
      if (documentId && projectId) {
        await addCard({
          content: response,
          type: activeTool,
          documentId: parseInt(documentId, 10),
          projectId: parseInt(projectId, 10),
          metadata: {
            tool: activeTool,
            prompt: prompt,
            selectedText: selectedText
          }
        });
      }
      
    } catch (error) {
      console.error('AI tool execution failed:', error);
    }
  };
  
  // Handle streaming completion
  const handleStreamingComplete = (text: string) => {
    setResult(text);
    
    // Add to cards if enabled
    if (documentId && projectId) {
      addCard({
        content: text,
        type: activeTool,
        documentId: parseInt(documentId, 10),
        projectId: parseInt(projectId, 10),
        metadata: {
          tool: activeTool,
          prompt: prompt,
          selectedText: selectedText
        }
      });
    }
  };
  
  // Copy result to clipboard
  const handleCopy = async () => {
    if (result) {
      await navigator.clipboard.writeText(result);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };
  
  // Insert or replace text
  const handleInsert = () => {
    if (result) {
      if (currentTool.requiresSelection && hasSelection) {
        onReplaceText?.(result);
      } else {
        onInsertText?.(result);
      }
    }
  };
  
  // Clear result and reset
  const handleClear = () => {
    setResult('');
    setPrompt('');
    if (activeTool === 'chat') {
      setChatHistory([]);
    }
  };
  
  // Credit estimation based on tool, model, and input/output heuristics
  const estimatedCost = (() => {
    try {
      let inputTokens = 0;
      let outputTokens = 0;
      switch (activeTool) {
        case 'write': {
          inputTokens = estimateTokensFromText(prompt);
          outputTokens = estimateExpectedOutputTokensForWrite({
            card_length: settings.write.card_length,
            card_count: settings.write.card_count
          });
          break;
        }
        case 'rewrite': {
          inputTokens = estimateTokensFromText(selectedText);
          outputTokens = inputTokens; // similar size rewrite
          break;
        }
        case 'expand': {
          inputTokens = estimateTokensFromText(selectedText);
          outputTokens = estimateExpectedOutputTokensForExpand(inputTokens, settings.expand.length_multiplier);
          break;
        }
        case 'quickEdit': {
          inputTokens = estimateTokensFromText(selectedText);
          outputTokens = inputTokens; // minor edits roughly same size
          break;
        }
        case 'brainstorm':
        case 'describe':
        case 'visualize':
        case 'chat': {
          inputTokens = estimateTokensFromText(prompt);
          outputTokens = 200; // default heuristic for single-response ops
          break;
        }
        default:
          return 0;
      }
      const modelName = (settings as any)?.defaultModel || settings.write.prose_mode;
      return estimateOperationCreditsWithModel(inputTokens, outputTokens, modelName);
    } catch {
      return 0;
    }
  })();
  
  return (
    <Card className={`w-full max-w-2xl ${className}`}>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-lg font-semibold flex items-center gap-2">
            <currentTool.icon className="w-5 h-5" />
            AI Writing Tools
          </CardTitle>
          
          <div className="flex items-center gap-2">
            <Badge variant="outline" className="text-xs">
              {creditsRemaining || 'Unlimited'} credits
            </Badge>
            
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowSettings(!showSettings)}
            >
              <Settings className="w-4 h-4" />
            </Button>
            
            <Collapsible open={isExpanded} onOpenChange={setIsExpanded}>
              <CollapsibleTrigger asChild>
                <Button variant="ghost" size="sm">
                  {isExpanded ? <ChevronUp className="w-4 h-4" /> : <ChevronDown className="w-4 h-4" />}
                </Button>
              </CollapsibleTrigger>
            </Collapsible>
          </div>
        </div>
        
        {/* Tool Selection */}
        <Collapsible open={isExpanded}>
          <CollapsibleContent>
            <div className="grid grid-cols-4 gap-2 mt-4">
              {Object.entries(toolConfig).map(([key, config]) => {
                const Icon = config.icon;
                const isActive = activeTool === key;
                const isDisabled = config.requiresSelection && !hasSelection;
                
                return (
                  <Button
                    key={key}
                    variant={isActive ? 'default' : 'outline'}
                    size="sm"
                    disabled={isDisabled}
                    onClick={() => setActiveTool(key as AITool)}
                    className={`h-auto p-3 flex flex-col gap-1 ${isActive ? `bg-${config.color}-500 hover:bg-${config.color}-600` : ''}`}
                  >
                    <Icon className="w-4 h-4" />
                    <span className="text-xs">{config.title}</span>
                  </Button>
                );
              })}
            </div>
          </CollapsibleContent>
        </Collapsible>
      </CardHeader>
      
      <CardContent className="space-y-4">
        {/* Settings Panel */}
        <AnimatePresence>
          {showSettings && (
            <motion.div
              initial={{ opacity: 0, height: 0 }}
              animate={{ opacity: 1, height: 'auto' }}
              exit={{ opacity: 0, height: 0 }}
              className="border rounded-lg p-4 space-y-4"
            >
              <div className="flex items-center justify-between">
                <h4 className="font-medium">Settings</h4>
                <Button variant="ghost" size="sm" onClick={() => setShowSettings(false)}>
                  <X className="w-4 h-4" />
                </Button>
              </div>
              
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <label className="text-sm font-medium">Tone</label>
                  <Select value={settings.write.tone || 'professional'} onValueChange={(value) => updateSettings.write({ tone: value })}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="professional">Professional</SelectItem>
                      <SelectItem value="casual">Casual</SelectItem>
                      <SelectItem value="creative">Creative</SelectItem>
                      <SelectItem value="academic">Academic</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                
                <div className="space-y-2">
                  <label className="text-sm font-medium">Prose Mode</label>
                  <Select value={settings.write.prose_mode || 'default'} onValueChange={(value) => updateSettings.write({ prose_mode: value })}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="default">Default</SelectItem>
                      <SelectItem value="streaming">Streaming</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                
                <div className="space-y-2">
                  <label className="text-sm font-medium">Card Count: {settings.write.card_count || 0}</label>
                  <Slider
                    value={[settings.write.card_count || 0]}
                    onValueChange={([value]) => updateSettings.write({ card_count: value })}
                    min={0}
                    max={5}
                    step={1}
                    className="w-full"
                  />
                </div>
                
                <div className="space-y-2">
                  <label className="text-sm font-medium">Creativity: {settings.write.creativity_level}</label>
                  <Slider
                    value={[settings.write.creativity_level]}
                    onValueChange={([value]) => updateSettings.write({ creativity_level: value })}
                    min={1}
                    max={10}
                    step={1}
                    className="w-full"
                  />
                </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
        
        {/* Tool Description */}
        <div className="text-sm text-gray-600 dark:text-gray-400">
          {currentTool.description}
          {needsSelection && (
            <span className="text-amber-600 dark:text-amber-400 ml-2">
              (Select text first)
            </span>
          )}
        </div>
        
        {/* Selected Text Display */}
        {hasSelection && (
          <div className="p-3 bg-gray-50 dark:bg-gray-800 rounded-lg border">
            <div className="text-xs text-gray-500 dark:text-gray-400 mb-1">Selected Text:</div>
            <div className="text-sm italic">"{selectedText}"</div>
          </div>
        )}
        
        {/* Chat History */}
        {activeTool === 'chat' && chatHistory.length > 0 && (
          <div className="max-h-40 overflow-y-auto space-y-2 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
            {chatHistory.map((msg, index) => (
              <div key={index} className={`text-sm ${msg.role === 'user' ? 'text-blue-600 dark:text-blue-400' : 'text-gray-700 dark:text-gray-300'}`}>
                <span className="font-medium">{msg.role === 'user' ? 'You' : 'AI'}:</span> {msg.content}
              </div>
            ))}
          </div>
        )}
        
        {/* Prompt Input */}
        {currentTool.requiresPrompt && (
          <div className="space-y-2">
            <label className="text-sm font-medium">
              {activeTool === 'chat' ? 'Message' : 'Prompt'}
            </label>
            <Textarea
              ref={textareaRef}
              value={prompt}
              onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => setPrompt(e.target.value)}
              placeholder={`Enter your ${activeTool === 'chat' ? 'message' : 'prompt'}...`}
              className="min-h-[80px] resize-none"
              onKeyDown={(e: React.KeyboardEvent<HTMLTextAreaElement>) => {
                if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
                  e.preventDefault();
                  handleExecute();
                }
              }}
            />
          </div>
        )}
        
        {/* Action Buttons */}
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Button
              onClick={handleExecute}
              disabled={!canExecute || needsSelection || streaming.isStreaming}
              className="flex items-center gap-2"
            >
              <currentTool.icon className="w-4 h-4" />
              {streaming.isStreaming ? 'Generating...' : currentTool.title}
            </Button>
            
            {result && (
              <Button variant="outline" size="sm" onClick={handleClear}>
                <RotateCcw className="w-4 h-4 mr-1" />
                Clear
              </Button>
            )}
          </div>
          
          {estimatedCost > 0 && (
            <Badge variant="outline" className="text-xs">
              ~{estimatedCost} credits
            </Badge>
          )}
        </div>
        
        {/* Streaming Display */}
        {streaming.isStreaming && (
          <StreamingText
            text={streaming.currentText}
            isStreaming={streaming.isStreaming}
            isPaused={streaming.isPaused}
            onComplete={handleStreamingComplete}
            className="border rounded-lg p-4"
          />
        )}
        
        {/* Result Display */}
        {result && !streaming.isStreaming && (
          <div className="space-y-3">
            <Separator />
            
            <div className="flex items-center justify-between">
              <h4 className="font-medium">Result</h4>
              <div className="flex items-center gap-2">
                <Button variant="outline" size="sm" onClick={handleCopy}>
                  {copied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
                  {copied ? 'Copied!' : 'Copy'}
                </Button>
                
                {(onInsertText || onReplaceText) && (
                  <Button size="sm" onClick={handleInsert}>
                    {currentTool.requiresSelection && hasSelection ? 'Replace' : 'Insert'}
                  </Button>
                )}
              </div>
            </div>
            
            <div className="p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border max-h-60 overflow-y-auto">
              <div className="prose dark:prose-invert max-w-none text-sm whitespace-pre-wrap">
                {result}
              </div>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
};

export default AIWritingPanel;
