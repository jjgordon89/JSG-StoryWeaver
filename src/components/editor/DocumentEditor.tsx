import React, { useState, useEffect, useRef, useCallback } from 'react';
import * as monaco from 'monaco-editor';
import { useStore } from '../../stores/documentStore';
import { useSettingsStore } from '../../stores/settingsStore';
import { invoke } from '@tauri-apps/api/core';
import FocusMode from './FocusMode';
import FocusModeSettings from './FocusModeSettings';
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
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  
  const saveDocument = useStore((state) => state.saveDocument);
  const { focusModeEnabled } = useSettingsStore();

  // Auto-save debouncer
  const saveDebouncer = useRef<NodeJS.Timeout | null>(null);

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

    // Set initial word count
    setWordCount(calculateWordCount(initialContent));
    
    // Set up change handler
    const changeDisposable = editor.onDidChangeModelContent(() => {
      handleEditorChange(editor.getValue());
    });

    editorRef.current = editor;

    // Cleanup
    return () => {
      changeDisposable.dispose();
      editor.dispose();
      editorRef.current = null;
      
      // Cancel any pending auto-save
      if (saveDebouncer.current) {
        clearTimeout(saveDebouncer.current);
      }
    };
  }, [initialContent, handleEditorChange, calculateWordCount]);

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
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleEditorChange]);

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
        <div 
          ref={containerRef} 
          className={`flex-grow overflow-hidden ${focusModeEnabled ? 'focus-mode-editor' : ''}`}
        />
        
        {/* Focus Mode Settings Dialog */}
        <FocusModeSettings 
          isOpen={showFocusModeSettings} 
          onClose={() => setShowFocusModeSettings(false)} 
        />
      </div>
    </FocusMode>
  );
};

export default DocumentEditor;
