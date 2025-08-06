import React, { useState, useEffect, useRef } from 'react';
import { Editor } from 'monaco-editor';
import { useStore } from '../../stores/documentStore';
import { invoke } from '@tauri-apps/api';

interface DocumentEditorProps {
  documentId: number;
  initialContent: string;
}

const DocumentEditor: React.FC<DocumentEditorProps> = ({ documentId, initialContent }) => {
  const [content, setContent] = useState(initialContent);
  const [isSaving, setIsSaving] = useState(false);
  const editorRef = useRef<Editor | null>(null);

  const saveDocument = useStore((state) => state.saveDocument);

  // Auto-save debouncer
  const saveDebouncer = useRef<NodeJS.Timeout | null>(null);

  const handleEditorChange = (value: string) => {
    setContent(value);
    
    // Debounced auto-save
    if (saveDebouncer.current) {
      global.clearTimeout(saveDebouncer.current);
    }
    
    saveDebouncer.current = global.setTimeout(async () => {
      try {
        setIsSaving(true);
        await invoke('saveDocument', {
          documentId,
          content: value,
        });
        console.log('Document saved successfully');
      } catch (error) {
        console.error('Error saving document:', error);
      } finally {
        setIsSaving(false);
      }
    }, 2000); // 2-second debounce
  };

  useEffect(() => {
    const editor = new Editor(document.getElementById('editor'), {
      value: initialContent,
      language: 'markdown',
      automaticLayout: true,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      overviewRulerLane: 'right',
    });

    editorRef.current = editor;

    return () => {
      editor.dispose();
      editorRef.current = null;
    };
  }, [initialContent]);

  return (
    <div className="flex-grow overflow-hidden">
      <div id="editor" className="h-full w-full"></div>
      {isSaving && <div className="fixed bottom-0 left-0 p-2 bg-green-100 text-green-700">
        Saving...
      </div>}
    </div>
  );
};

export default DocumentEditor;
