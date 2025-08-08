import React, { useState, useEffect, useRef, useCallback } from 'react';
import * as monaco from 'monaco-editor';
import { useHotkeys } from 'react-hotkeys-hook';
import { useStore } from '../../stores/documentStore';
import { useSettingsStore } from '../../stores/settingsStore';
import { useVersionStore, DocumentVersion } from '../../stores/versionStore';
import { useStoryBible } from '../../features/story-bible/hooks/useStoryBible';
import { invoke } from '../../utils/tauriSafe';
import { AITextDecorationManager } from '../../utils/aiTextDecorations';
import { StoryBibleTextDetector } from '../../utils/storyBibleTextDetection';
import FocusMode from './FocusMode';
import FocusModeSettings from './FocusModeSettings';
import VersionHistory from './VersionHistory';
import StoryBibleBoxes from './StoryBibleBoxes';
import { AISelectionMenu, AIWritingPanel, AIQuickTools } from '../ai';
import { Button } from '../ui/button';
import { Wand2, PanelRightOpen, PanelRightClose } from 'lucide-react';
import '../../styles/focus-mode.css';
import { emitSyncEvent, SyncEventType } from '../../utils/stateSynchronizer';

interface DocumentEditorProps {
  documentId: number;
  initialContent: string;
}

const DocumentEditor: React.FC<DocumentEditorProps> = ({ documentId, initialContent }) => {
  const [content, setContent] = useState(initialContent);
  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<'saved' | 'saving' | 'error'>('saved');
  const [wordCount, setWordCount] = useState(0);
  const [showFocusModeSettings, setShowFocusModeSettings] = useState(false);
  const [showVersionHistory, setShowVersionHistory] = useState(false);
  const [showAIPanel, setShowAIPanel] = useState(false);
  const [showQuickTools, setShowQuickTools] = useState(false);
  const [quickToolsPosition, setQuickToolsPosition] = useState({ x: 0, y: 0 });
  const [storyBibleDetectionEnabled, setStoryBibleDetectionEnabled] = useState(true);
  const [aiDecorationManager, setAiDecorationManager] = useState<AITextDecorationManager | null>(null);
  const [storyBibleDetector, setStoryBibleDetector] = useState<StoryBibleTextDetector | null>(null);
  const [aiMenuVisible, setAIMenuVisible] = useState(false);
  const [aiMenuPosition, setAIMenuPosition] = useState({ x: 0, y: 0 });
  const [selectedText, setSelectedText] = useState('');
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  
  const saveDocument = useStore((state) => state.saveDocument);
  const { focusModeEnabled } = useSettingsStore();
  const { createVersion } = useVersionStore();
  const { storyBible } = useStoryBible();

  // Auto-save debouncer
  const saveDebouncer = useRef<NodeJS.Timeout | null>(null);

  // Handle opening Quick Tools at cursor position
  const handleOpenQuickTools = useCallback(() => {
    if (editorRef.current) {
      const position = editorRef.current.getPosition();
      if (position) {
        const coords = editorRef.current.getScrolledVisiblePosition(position);
        if (coords) {
          setQuickToolsPosition({ x: coords.left + 50, y: coords.top + 50 });
          setShowQuickTools(true);
        }
      }
    }
  }, []);

  // Cleanup effect for decoration managers
  useEffect(() => {
    return () => {
      if (aiDecorationManager) {
        aiDecorationManager.dispose();
      }
      if (storyBibleDetector) {
        storyBibleDetector.dispose();
      }
    };
  }, [aiDecorationManager, storyBibleDetector]);

  // Update Story Bible detector when story bible data changes
  useEffect(() => {
    if (storyBibleDetector && storyBible) {
      storyBibleDetector.updateStoryBibleData({
        characters: storyBible.characters || [],
        worldElements: storyBible.world_elements || [],
        outlines: storyBible.outlines || [],
        scenes: storyBible.scenes || []
      });
      
      // Re-analyze current content with updated data
      if (editorRef.current) {
        storyBibleDetector.analyzeText(editorRef.current.getValue());
      }
    }
  }, [storyBibleDetector, storyBible]);

  // Handle Story Bible detection toggle
  const handleToggleStoryBibleDetection = useCallback(() => {
    setStoryBibleDetectionEnabled(prev => {
      const newEnabled = !prev;
      if (storyBibleDetector) {
        storyBibleDetector.setEnabled(newEnabled);
        if (newEnabled && editorRef.current) {
          // Re-analyze content when re-enabling
          storyBibleDetector.analyzeText(editorRef.current.getValue());
        }
      }
      return newEnabled;
    });
  }, [storyBibleDetector]);

  // Keyboard shortcuts using react-hotkeys-hook
  useHotkeys('ctrl+k, cmd+k', (e) => {
    e.preventDefault();
    handleOpenQuickTools();
  }, { enableOnFormTags: true });

  useHotkeys('escape', () => {
    if (showQuickTools) {
      setShowQuickTools(false);
    } else if (aiMenuVisible) {
      setAIMenuVisible(false);
    }
  }, { enableOnFormTags: true });

  // Calculate word count
  const calculateWordCount = useCallback((text: string): number => {
    // Remove markdown formatting and count words
    const cleanText = text
      .replace(/```[\s\S]*?```/g, '') // Remove code blocks
      .replace(/`[^`]*`/g, '')        // Remove inline code
      .replace(/\[.*?\]\(.*?\)/g, '$1') // Replace links with just the text
      .replace(/[#*_~`]/g, '')        // Remove markdown symbols
      .trim();
      
    return cleanText ? cleanText.split(/\s+/).length : 0;
  }, []);

  // Handle editor content changes
  const handleEditorChange = useCallback((value: string) => {
    setContent(value);
    setWordCount(calculateWordCount(value));
    
    // Update save status
    setSaveStatus('saving');
    
    // Debounced auto-save
    if (saveDebouncer.current) {
      clearTimeout(saveDebouncer.current);
    }
    
    saveDebouncer.current = setTimeout(async () => {
      try {
        setIsSaving(true);
        await saveDocument(documentId, value);
        setSaveStatus('saved');
        console.log('Document saved successfully');
        
        // Create a version every 10 saves (or based on some other logic)
        // This is just a simple example - you might want to implement more sophisticated version creation logic
        if (Math.random() < 0.1) { // 10% chance to create a version on each save
          try {
            await createVersion(documentId.toString());
            console.log('Document version created');
          } catch (err) {
            console.error('Failed to create document version:', err);
          }
        }
        
        // Emit sync event to notify other components about the document update
        emitSyncEvent(SyncEventType.DOCUMENT_UPDATED, {
          documentId,
          projectId: documentId, // We don't have the project ID here, but it's required in the payload
          content: value,
          wordCount: calculateWordCount(value)
        }).catch(err => {
          console.error('Failed to emit document update event:', err);
        });
      } catch (error) {
        console.error('Error saving document:', error);
        setSaveStatus('error');
      } finally {
        setIsSaving(false);
      }
    }, 1500); // 1.5-second debounce
  }, [documentId, saveDocument, calculateWordCount]);

  // Handle text insertion from AI
  const handleInsertText = useCallback((text: string) => {
    if (!editorRef.current) return;
    
    const editor = editorRef.current;
    const selection = editor.getSelection();
    
    if (selection) {
      editor.executeEdits('ai-insert', [{
        range: selection,
        text: text,
        forceMoveMarkers: true
      }]);
      
      // Add purple highlighting to AI-generated content
       if (aiDecorationManager) {
         const newRange = {
           startLineNumber: selection.startLineNumber,
           startColumn: selection.startColumn,
           endLineNumber: selection.startLineNumber,
           endColumn: selection.startColumn + text.length
         };
         aiDecorationManager.addAITextRange(newRange);
       }
    }
    
    setAIMenuVisible(false);
  }, [aiDecorationManager]);

  // Handle text replacement from AI
  const handleReplaceText = useCallback((text: string) => {
    if (!editorRef.current) return;
    
    const editor = editorRef.current;
    const selection = editor.getSelection();
    
    if (selection && !selection.isEmpty()) {
      editor.executeEdits('ai-replace', [{
        range: selection,
        text: text,
        forceMoveMarkers: true
      }]);
      
      // Add purple highlighting to AI-generated content
       if (aiDecorationManager) {
         const newRange = {
           startLineNumber: selection.startLineNumber,
           startColumn: selection.startColumn,
           endLineNumber: selection.startLineNumber,
           endColumn: selection.startColumn + text.length
         };
         aiDecorationManager.addAITextRange(newRange);
       }
    }
    
    setAIMenuVisible(false);
  }, [aiDecorationManager]);

  // Handle selection change to update selected text
  const handleSelectionChange = useCallback(() => {
    if (!editorRef.current) return;
    
    const editor = editorRef.current;
    const selection = editor.getSelection();
    
    if (selection && !selection.isEmpty()) {
      const selectedText = editor.getModel()?.getValueInRange(selection) || '';
      setSelectedText(selectedText);
    } else {
      setSelectedText('');
    }
  }, []);

  // Initialize editor
  useEffect(() => {
    if (!containerRef.current) return;
    
    const editor = monaco.editor.create(containerRef.current, {
      value: initialContent,
      language: 'markdown',
      theme: 'vs-dark',
      automaticLayout: true,
      minimap: { enabled: true },
      scrollBeyondLastLine: false,
      lineNumbers: 'on',
      wordWrap: 'on',
      wrappingIndent: 'same',
      fontSize: 14,
      renderLineHighlight: 'all',
      padding: { top: 16 },
    });

    // Initialize AI decoration manager
    const decorationManager = new AITextDecorationManager(editor);
    setAiDecorationManager(decorationManager);

    // Initialize Story Bible text detector
    const storyBibleDetector = new StoryBibleTextDetector(editor);
    setStoryBibleDetector(storyBibleDetector);
    
    // Set initial Story Bible detection state
    storyBibleDetector.setEnabled(storyBibleDetectionEnabled);

    // Set initial word count
    setWordCount(calculateWordCount(initialContent));
    
    // Set up change handler
    const changeDisposable = editor.onDidChangeModelContent(() => {
      const currentContent = editor.getValue();
      handleEditorChange(currentContent);
      
      // Update Story Bible detection when content changes
      if (storyBibleDetector) {
        storyBibleDetector.analyzeText(currentContent);
      }
    });

    // Set up selection change handler
    const selectionDisposable = editor.onDidChangeCursorSelection(() => {
      handleSelectionChange();
    });

    // Set up context menu for AI tools
    const contextMenuDisposable = editor.onContextMenu((e) => {
      const selection = editor.getSelection();
      if (selection && !selection.isEmpty()) {
        setAIMenuPosition({ x: e.event.posx, y: e.event.posy });
        setAIMenuVisible(true);
      }
    });

    editorRef.current = editor;

    // Cleanup
    return () => {
      changeDisposable.dispose();
      selectionDisposable.dispose();
      contextMenuDisposable.dispose();
      editor.dispose();
      editorRef.current = null;
      
      // Cancel any pending auto-save
      if (saveDebouncer.current) {
        clearTimeout(saveDebouncer.current);
      }
    };
  }, [initialContent, handleEditorChange, calculateWordCount, handleSelectionChange]);

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+S or Cmd+S to save
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        if (editorRef.current) {
          handleEditorChange(editorRef.current.getValue());
        }
      }
      
      // Ctrl+H or Cmd+H to show version history
      if ((e.ctrlKey || e.metaKey) && e.key === 'h') {
        e.preventDefault();
        setShowVersionHistory(true);
      }
      
      // Ctrl+Shift+A or Cmd+Shift+A to toggle AI panel
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'A') {
        e.preventDefault();
        setShowAIPanel(!showAIPanel);
      }
      
      // Ctrl+Space or Cmd+Space to show AI menu
      if ((e.ctrlKey || e.metaKey) && e.key === ' ') {
        e.preventDefault();
        if (editorRef.current) {
          const position = editorRef.current.getPosition();
          if (position) {
            const coords = editorRef.current.getScrolledVisiblePosition(position);
            if (coords) {
              setAIMenuPosition({ x: coords.left, y: coords.top });
              setAIMenuVisible(true);
            }
          }
        }
      }
      
      // Note: Escape handling moved to react-hotkeys-hook
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleEditorChange, showAIPanel]);
  
  // Handle restoring a version
  const handleRestoreVersion = (version: DocumentVersion) => {
    if (editorRef.current && version.content) {
      editorRef.current.setValue(version.content);
      handleEditorChange(version.content);
      setShowVersionHistory(false);
    }
  };
  
  // Create a new version manually
  const handleCreateVersion = async () => {
    if (!editorRef.current) return;
    
    try {
      setSaveStatus('saving');
      const currentContent = editorRef.current.getValue();
      
      // First save the document
      await saveDocument(documentId, currentContent);
      
      // Then create a version
      await createVersion(documentId.toString(), "Manual save");
      
      setSaveStatus('saved');
      console.log('Document version created manually');
    } catch (error) {
      console.error('Error creating version:', error);
      setSaveStatus('error');
    }
  };

  return (
    <FocusMode>
      <div className="flex flex-col h-full">
        <div className={`flex justify-between items-center px-4 py-2 bg-gray-100 dark:bg-gray-800 border-b ${
          focusModeEnabled && useSettingsStore.getState().focusModeOptions.hideHeader ? 'opacity-0 h-0 overflow-hidden' : ''
        }`}>
            <div className="text-sm">
              Words: {wordCount}
            </div>
            <div className="flex items-center space-x-4">
              <div className="text-sm">
                {saveStatus === 'saved' && (
                  <span className="text-green-600 dark:text-green-400">✓ Saved</span>
                )}
                {saveStatus === 'saving' && (
                  <span className="text-yellow-600 dark:text-yellow-400">Saving...</span>
                )}
                {saveStatus === 'error' && (
                  <span className="text-red-600 dark:text-red-400">Error saving</span>
                )}
              </div>
              <button
                onClick={handleCreateVersion}
                className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 mr-2"
                title="Create Version"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M12 2v20M2 12h20"/>
                </svg>
              </button>
              <button
                onClick={() => setShowVersionHistory(true)}
                className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 mr-2"
                title="Version History"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <circle cx="12" cy="12" r="10"></circle>
                  <polyline points="12 6 12 12 16 14"></polyline>
                </svg>
              </button>
              <Button
                variant="ghost"
                size="sm"
                onClick={handleToggleStoryBibleDetection}
                className={`text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 ${
                  storyBibleDetectionEnabled ? 'bg-blue-100 dark:bg-blue-900' : ''
                }`}
                title="Toggle Story Bible Detection"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"/>
                  <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"/>
                  <circle cx="12" cy="12" r="2"/>
                </svg>
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => setShowAIPanel(!showAIPanel)}
                className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
                title="Toggle AI Panel (Ctrl+Shift+A)"
              >
                {showAIPanel ? <PanelRightClose size={18} /> : <PanelRightOpen size={18} />}
              </Button>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  if (editorRef.current) {
                    const position = editorRef.current.getPosition();
                    if (position) {
                      const coords = editorRef.current.getScrolledVisiblePosition(position);
                      if (coords) {
                        setAIMenuPosition({ x: coords.left, y: coords.top });
                        setAIMenuVisible(true);
                      }
                    }
                  }
                }}
                className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
                title="AI Tools (Ctrl+Space)"
              >
                <Wand2 size={18} />
              </Button>
              <button
                onClick={() => setShowFocusModeSettings(true)}
                className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
                title="Focus Mode Settings"
              >
                <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <circle cx="12" cy="12" r="3"></circle>
                  <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
                </svg>
              </button>
            </div>
        </div>
        <div className="flex flex-col flex-grow overflow-hidden">
          <div className="flex flex-grow overflow-hidden">
            <div 
              ref={containerRef} 
              className={`flex-grow overflow-hidden ${focusModeEnabled ? 'focus-mode-editor' : ''}`}
            />
            
            {/* AI Writing Panel */}
            {showAIPanel && (
              <div className="w-80 border-l border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
                <AIWritingPanel
                  isOpen={showAIPanel}
                  onToggle={() => setShowAIPanel(!showAIPanel)}
                  onInsertText={handleInsertText}
                  onReplaceText={handleReplaceText}
                  selectedText={selectedText}
                  documentContext={content}
                />
              </div>
            )}
          </div>
          
          {/* Story Bible Boxes */}
          <StoryBibleBoxes
            projectId={documentId.toString()}
            isVisible={storyBibleDetectionEnabled}
          />
        </div>
        
        {/* AI Selection Menu */}
        <AISelectionMenu
          visible={aiMenuVisible}
          position={aiMenuPosition}
          onClose={() => setAIMenuVisible(false)}
          onInsertText={handleInsertText}
          onReplaceText={handleReplaceText}
          selectedText={selectedText}
          documentContext={content}
        />
        
        {/* AI Quick Tools - Always available */}
        {!focusModeEnabled && (
          <div className="absolute bottom-4 right-4">
            <AIQuickTools
              compact
              onInsertText={handleInsertText}
              onReplaceText={handleReplaceText}
              selectedText={selectedText}
              documentContext={content}
            />
          </div>
        )}
        
        {/* Quick Tools Modal - Triggered by Ctrl+K */}
        {showQuickTools && (
          <div 
            className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50"
            onClick={() => setShowQuickTools(false)}
          >
            <div 
              className="relative"
              style={{
                left: quickToolsPosition.x - 160, // Center the 320px wide component
                top: quickToolsPosition.y - 200   // Position above cursor
              }}
              onClick={(e) => e.stopPropagation()}
            >
              <AIQuickTools
                selectedText={selectedText}
                documentContext={content}
                onInsertText={handleInsertText}
                onReplaceText={handleReplaceText}
                onClose={() => setShowQuickTools(false)}
                className=""
              />
            </div>
          </div>
        )}
        
        {/* Focus Mode Settings Dialog */}
        <FocusModeSettings 
          isOpen={showFocusModeSettings} 
          onClose={() => setShowFocusModeSettings(false)} 
        />
        
        {/* Version History Dialog */}
        {showVersionHistory && (
          <VersionHistory
            documentId={documentId.toString()}
            onClose={() => setShowVersionHistory(false)}
            onRestoreVersion={handleRestoreVersion}
          />
        )}
      </div>
    </FocusMode>
  );
};

export default DocumentEditor;
