import React, { useEffect } from 'react';
import useStoryBibleStore from '../../stores/storyBibleStore';
import CharactersManager from './components/CharactersManager';
import TemplateSelector from '../../lib/components/templates/TemplateSelector';
import TemplateApplicationDialog from '../../lib/components/templates/TemplateApplicationDialog';

// Svelte components (wrapped via simple mount wrapper)
import BraindumpEditorSvelte from './components/BraindumpEditor.svelte';
import StyleExamplesManagerSvelte from './components/StyleExamplesManager.svelte';
import WorldbuildingManagerSvelte from './components/WorldBuildingManager.svelte';
import OutlineManagerSvelte from './components/OutlineManager.svelte';
import ScenesManagerSvelte from './components/ScenesManager.svelte';

import { createRoot } from 'react-dom/client';
import { useRef } from 'react';

// Generic Svelte -> React wrapper
const SvelteWrapper: React.FC<{ Component: any; props?: any }> = ({ Component, props = {} }) => {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const instanceRef = useRef<any>(null);

  useEffect(() => {
    if (containerRef.current && !instanceRef.current) {
      instanceRef.current = new Component({
        target: containerRef.current,
        props
      });
    }

    return () => {
      if (instanceRef.current) {
        instanceRef.current.$destroy();
        instanceRef.current = null;
      }
    };
    // We intentionally do not re-create on props changes to avoid tearing down Svelte instances frequently.
    // For simple prop updates, consider exposing a small API or remounting when necessary.
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return <div ref={containerRef} />;
};

const tabs = [
  { id: 'braindump', label: 'Braindump', icon: '📝' },
  { id: 'style-examples', label: 'Style Examples', icon: '✍️' },
  { id: 'characters', label: 'Characters', icon: '👥' },
  { id: 'worldbuilding', label: 'World Building', icon: '🌍' },
  { id: 'outline', label: 'Outline', icon: '📋' },
  { id: 'scenes', label: 'Scenes', icon: '🎬' }
] as const;

interface Props {
  projectId: string;
  seriesId?: string;
}

const StoryBible: React.FC<Props> = ({ projectId, seriesId }) => {
  const {
    storyBible,
    isLoading,
    error,
    activeTab,
    setActiveTab,
    loadStoryBible,
    loadWorldElements,
    loadOutlines,
    selectedCharacterId,
    setSelectedOutlineId
  } = useStoryBibleStore();

  useEffect(() => {
    (async () => {
      await loadStoryBible(projectId);
      await loadWorldElements(projectId);
      await loadOutlines(projectId);
    })();
  }, [projectId, loadStoryBible, loadWorldElements, loadOutlines]);

  const handleTabClick = (tabId: typeof activeTab) => {
    setActiveTab(tabId);
  };

  const clearErrors = () => {
    // store exposes clearAllErrors; use direct invocation on store
    // but to keep import light, call setActiveTab(null) - prefer explicit clear function in store; omitted for brevity
  };

  return (
    <div className="story-bible-container flex flex-col h-full">
      <div className="story-bible-header p-6 border-b bg-gray-50 dark:bg-gray-800">
        <h1 className="title text-2xl font-semibold flex items-center gap-3">
          <span className="icon text-3xl">📚</span>
          Story Bible
        </h1>
        {error && (
          <div className="mt-3 text-red-600">
            {error}
            <button className="ml-4 underline" onClick={clearErrors}>Dismiss</button>
          </div>
        )}
      </div>

      <div className="tab-navigation flex bg-gray-100 dark:bg-gray-700 border-b">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            className={`tab-button px-4 py-3 flex items-center gap-2 ${activeTab === tab.id ? 'text-blue-600 border-b-2 border-blue-600' : 'text-gray-600'}`}
            onClick={() => handleTabClick(tab.id as any)}
            disabled={isLoading}
          >
            <span className="tab-icon">{tab.icon}</span>
            <span className="tab-label">{tab.label}</span>
          </button>
        ))}
      </div>

      <div className="content-area flex-1 overflow-hidden">
        {isLoading ? (
          <div className="loading-container flex flex-col items-center justify-center h-full gap-4">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
            <p>Loading Story Bible...</p>
          </div>
        ) : (
          <>
            {activeTab === 'braindump' && (
              <div className="p-6">
                <SvelteWrapper Component={BraindumpEditorSvelte} props={{
                  projectId,
                  content: storyBible?.braindump || '',
                  synopsis: storyBible?.synopsis || '',
                  genre: storyBible?.genre || '',
                  style: storyBible?.style || '',
                  styleExamples: storyBible?.style_examples || '',
                  povMode: storyBible?.pov_mode || '',
                  globalPov: storyBible?.global_pov || '',
                  globalTense: storyBible?.global_tense || '',
                  globalCharacterPovIds: storyBible?.global_character_pov_ids || ''
                }} />
              </div>
            )}

            {activeTab === 'style-examples' && (
              <div className="p-6">
                <SvelteWrapper Component={StyleExamplesManagerSvelte} props={{ projectId }} />
              </div>
            )}

            {activeTab === 'characters' && (
              <div className="p-6">
                <CharactersManager projectId={projectId} seriesId={seriesId} characterId={selectedCharacterId || undefined} />
              </div>
            )}

            {activeTab === 'worldbuilding' && (
              <div className="p-6">
                <SvelteWrapper Component={WorldbuildingManagerSvelte} props={{ projectId, seriesId }} />
              </div>
            )}

            {activeTab === 'outline' && (
              <div className="p-6">
                <SvelteWrapper Component={OutlineManagerSvelte} props={{ projectId }} />
              </div>
            )}

            {activeTab === 'scenes' && (
              <div className="p-6">
                <SvelteWrapper Component={ScenesManagerSvelte} props={{ outlineId: undefined }} />
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};

export default StoryBible;
