import React from 'react';
import ProjectView from '../../features/projects/ProjectView';
import DocumentEditor from '../editor/DocumentEditor';
import AdvancedAI from '../AdvancedAI/AdvancedAI';
import { useStore } from '../../stores/documentStore';
import { useSettingsStore } from '../../stores/settingsStore';
import TopBar from './TopBar';

const MainLayout: React.FC = () => {
  const { currentDocument, loadDocument } = useStore();
  const { focusModeEnabled } = useSettingsStore();

  const handleDocumentSelect = async (documentId: number) => {
    try {
      await loadDocument(documentId);
    } catch (error) {
      console.error('Error loading document:', error);
    }
  };

  return (
    <div
      className={`flex min-h-screen flex-col text-slate-900 dark:text-slate-100 ${
        focusModeEnabled ? 'focus-mode-layout' : ''
      }`}
    >
      <TopBar />

      <div className="relative flex-1 overflow-hidden">
        {/* Decorative gradient rails on page edges */}
        <div
          aria-hidden
          className="pointer-events-none absolute inset-y-0 left-0 w-2 rounded-full bg-gradient-to-b from-pink-200/70 via-pink-300/40 to-purple-200/30"
        />
        <div
          aria-hidden
          className="pointer-events-none absolute inset-y-0 right-0 w-2 rounded-full bg-gradient-to-b from-purple-200/70 via-purple-300/40 to-pink-200/30"
        />

        <div className="relative mx-auto max-w-screen-2xl h-full px-3 py-3">
          <div className="flex h-full gap-3">
            {/* Left Sidebar */}
            <aside className="w-72 shrink-0">
              <div className="h-full overflow-y-auto rounded-2xl border border-slate-200/70 bg-white/70 p-3 shadow-sm backdrop-blur dark:border-slate-800 dark:bg-slate-800/40">
                <ProjectView onDocumentSelect={handleDocumentSelect} />
              </div>
            </aside>

            {/* Main Editor */}
            <main className="flex min-w-0 flex-1 flex-col">
              <div className="mb-2 flex items-center justify-between">
                <h2 className="text-lg font-semibold">
                  {currentDocument ? currentDocument.name : 'Document Editor'}
                </h2>
              </div>

              <div className="flex-1 overflow-hidden rounded-2xl border border-slate-200/70 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-800/60">
                {currentDocument ? (
                  <DocumentEditor
                    documentId={currentDocument.id}
                    initialContent={currentDocument.content}
                  />
                ) : (
                  <div className="flex h-full items-center justify-center text-slate-500">
                    Select a document to start editing
                  </div>
                )}
              </div>
            </main>

            {/* Right Sidebar */}
            <aside className="w-80 shrink-0">
              <div className="h-full overflow-y-auto rounded-2xl border border-slate-200/70 bg-white/70 p-3 shadow-sm backdrop-blur dark:border-slate-800 dark:bg-slate-800/40">
                <h2 className="mb-3 text-lg font-semibold">History & Cards</h2>
                {currentDocument ? (
                  <div className="space-y-4">
                    <div className="rounded-md border border-slate-200/70 bg-white p-3 shadow-sm dark:border-slate-700 dark:bg-slate-800/60">
                      <h3 className="text-sm font-medium text-slate-500 dark:text-slate-400">
                        Document Info
                      </h3>
                      <p className="mt-1">Word count: {currentDocument.word_count || 0}</p>
                      <p className="mt-2 text-xs text-slate-500">
                        Last updated:{' '}
                        {currentDocument.updated_at
                          ? new Date(currentDocument.updated_at).toLocaleString()
                          : 'Unknown'}
                      </p>
                    </div>

                    <div className="overflow-hidden rounded-md border border-slate-200/70 bg-white shadow-sm dark:border-slate-700 dark:bg-slate-800/60">
                      <AdvancedAI />
                    </div>
                  </div>
                ) : (
                  <p className="text-slate-500">Select a document to view details</p>
                )}
              </div>
            </aside>
          </div>
        </div>
      </div>
    </div>
  );
};

export default MainLayout;
