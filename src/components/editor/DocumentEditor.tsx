import React, { useState, useEffect, useRef, useCallback } from 'react';
import * as monaco from 'monaco-editor';
import { useStore } from '../../stores/documentStore';
import { invoke } from '@tauri-apps/api/core';

interface DocumentEditorProps {
  documentId: number;
  initialContent: string;
}

const DocumentEditor: React.FC<DocumentEditorProps> = ({ documentId, initialContent }) => {
  const [content, setContent] = useState(initialContent);
  const [isSaving, setIsSaving] = useState(false);
  const [saveStatus, setSaveStatus] = useState<'saved' | 'saving' | 'error'>('saved');
  const [wordCount, setWordCount] = useState(0);
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  
  const saveDocument = useStore((state) => state.saveDocument);

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
    <div className="flex flex-col h-full">
      <div className="flex justify-between items-center px-4 py-2 bg-gray-100 dark:bg-gray-800 border-b">
        <div className="text-sm">
          Words: {wordCount}
        </div>
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
      </div>
      <div 
        ref={containerRef} 
        className="flex-grow overflow-hidden"
      />
    </div>
  );
};

export default DocumentEditor;
