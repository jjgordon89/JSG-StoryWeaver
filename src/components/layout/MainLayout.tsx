import React, { useState, useEffect } from 'react';
import ProjectView from '../../features/projects/ProjectView';
import DocumentEditor from '../editor/DocumentEditor';
import AdvancedAI from '../AdvancedAI/AdvancedAI';
import { useStore } from '../../stores/documentStore';
import { useSettingsStore } from '../../stores/settingsStore';
import { invoke } from '../../utils/tauriSafe';

const MainLayout: React.FC = () => {
  const [activeDocument, setActiveDocument] = useState<{id: number, content: string} | null>(null);
  const { currentDocument, loadDocument } = useStore();
  const { focusModeEnabled } = useSettingsStore();
  
  // Load a document when selected from the project view
  const handleDocumentSelect = async (documentId: number) => {
    try {
      await loadDocument(documentId);
    } catch (error) {
      console.error('Error loading document:', error);
    }
  };

  return (
    <div className={`flex h-screen bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-gray-100 ${
      focusModeEnabled ? 'focus-mode-layout' : ''
    }`}>
      {/* Left Column (Navigation) */}
      <div className="w-1/4 bg-gray-200 dark:bg-gray-800 p-4 overflow-y-auto">
        <ProjectView onDocumentSelect={handleDocumentSelect} />
      </div>

      {/* Middle Column (Editor) */}
      <div className="flex-1 flex flex-col p-4">
        <h2 className="text-xl font-bold mb-4">
          {currentDocument ? currentDocument.name : 'Document Editor'}
        </h2>
        <div className="flex-grow bg-white dark:bg-gray-700 rounded-md overflow-hidden">
          {currentDocument ? (
            <DocumentEditor 
              documentId={currentDocument.id} 
              initialContent={currentDocument.content} 
            />
          ) : (
            <div className="flex items-center justify-center h-full text-gray-500">
              Select a document to start editing
            </div>
          )}
        </div>
      </div>

      {/* Right Column (History/Cards) */}
      <div className="w-1/4 bg-gray-200 dark:bg-gray-800 p-4 overflow-y-auto">
        <h2 className="text-xl font-bold mb-4">History & Cards</h2>
        {currentDocument ? (
          <div className="space-y-4">
            <div className="bg-white dark:bg-gray-700 p-3 rounded-md shadow">
              <h3 className="font-medium text-sm text-gray-500 dark:text-gray-400">Document Info</h3>
              <p className="mt-1">Word count: {currentDocument.word_count}</p>
              <p className="text-xs text-gray-500 mt-2">
                Last updated: {new Date(currentDocument.updated_at).toLocaleString()}
              </p>
            </div>
            {/* Advanced AI Component */}
            <div className="bg-white dark:bg-gray-700 rounded-md shadow overflow-hidden">
              <AdvancedAI />
            </div>
          </div>
        ) : (
          <p className="text-gray-500">Select a document to view details</p>
        )}
      </div>
    </div>
  );
};

export default MainLayout;
