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
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Textarea } from '../ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../ui/select';
import { Slider } from '../ui/slider';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Badge } from '../ui/badge';
import { Separator } from '../ui/separator';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/tabs';
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
    requiresPrompt: true
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
    requiresPrompt: true
  },
  describe: {
    icon: MessageSquare,
    title: 'Describe',
    description: 'Create detailed scene descriptions',
    color: 'indigo',
    requiresPrompt: true
  },
  visualize: {
    icon: Eye,
    title: 'Visualize',
    description: 'Generate visual scene descriptions',
    color: 'pink',
    requiresPrompt: true
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
    requiresPrompt: true
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
  const { startStreaming, streaming } = useAIWriteStream();
  const { rewriteText, expandText, quickEdit } = useAITextProcessor();
  const { brainstorm, describeScene, visualizeScene } = useAICreative();
  const { settings, updateSettings } = useAISettings();
  const { credits, estimateCredits } = useAICredits();
  const { addCard } = useCards();
  
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
          if (settings.streamingEnabled) {
            await startStreaming('write', { prompt, documentId, projectId });
            return;
          } else {
            response = await autoWrite({
              prompt,
              documentId,
              projectId,
              tone: settings.tone,
              style: settings.style,
              length: settings.length
            });
          }
          break;
          
        case 'rewrite':
          response = await rewriteText({
            text: selectedText,
            instructions: prompt || 'Rewrite this text',
            tone: settings.tone,
            style: settings.style
          });
          break;
          
        case 'expand':
          response = await expandText({
            text: selectedText,
            instructions: prompt || 'Expand this text with more detail',
            targetLength: settings.length
          });
          break;
          
        case 'brainstorm':
          response = await brainstorm({
            topic: prompt,
            count: 5,
            style: settings.style
          });
          break;
          
        case 'describe':
          response = await describeScene({
            prompt,
            style: settings.style,
            detail: settings.length
          });
          break;
          
        case 'visualize':
          response = await visualizeScene({
            prompt,
            style: settings.style,
            detail: settings.length
          });
          break;
          
        case 'quickEdit':
          response = await quickEdit({
            text: selectedText,
            instruction: prompt
          });
          break;
          
        case 'chat':
          const newHistory = [...chatHistory, { role: 'user' as const, content: prompt }];
          setChatHistory(newHistory);
          
          // For chat, we'll use guided write with conversation context
          response = await guidedWrite({
            prompt,
            context: newHistory.map(msg => `${msg.role}: ${msg.content}`).join('\n'),
            documentId,
            projectId
          });
          
          setChatHistory([...newHistory, { role: 'assistant', content: response }]);
          break;
      }
      
      setResult(response);
      
      // Add to cards if enabled
      if (settings.saveToCards && documentId) {
        await addCard({
          content: response,
          type: activeTool,
          documentId,
          projectId,
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
    if (settings.saveToCards && documentId) {
      addCard({
        content: text,
        type: activeTool,
        documentId,
        projectId,
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
  
  const estimatedCost = estimateCredits(activeTool, prompt.length + selectedText.length);
  
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
              {credits.remaining} credits
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
                  <Select value={settings.tone} onValueChange={(value) => updateSettings({ tone: value })}>
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
                  <label className="text-sm font-medium">Style</label>
                  <Select value={settings.style} onValueChange={(value) => updateSettings({ style: value })}>
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="narrative">Narrative</SelectItem>
                      <SelectItem value="descriptive">Descriptive</SelectItem>
                      <SelectItem value="dialogue">Dialogue</SelectItem>
                      <SelectItem value="expository">Expository</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                
                <div className="space-y-2">
                  <label className="text-sm font-medium">Length: {settings.length}</label>
                  <Slider
                    value={[settings.length]}
                    onValueChange={([value]) => updateSettings({ length: value })}
                    min={1}
                    max={5}
                    step={1}
                    className="w-full"
                  />
                </div>
                
                <div className="space-y-2">
                  <label className="text-sm font-medium">Creativity: {settings.creativity}</label>
                  <Slider
                    value={[settings.creativity]}
                    onValueChange={([value]) => updateSettings({ creativity: value })}
                    min={0}
                    max={1}
                    step={0.1}
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
              onChange={(e) => setPrompt(e.target.value)}
              placeholder={`Enter your ${activeTool === 'chat' ? 'message' : 'prompt'}...`}
              className="min-h-[80px] resize-none"
              onKeyDown={(e) => {
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