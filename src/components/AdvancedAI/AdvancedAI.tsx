import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import './AdvancedAI.css';
import AdvancedTextGenerator from './AdvancedTextGenerator';
import ImageGenerator from './ImageGenerator';
import BrainstormEngine from './BrainstormEngine';
import StyleManager from './StyleManager';
import AdvancedAISettings from './AdvancedAISettings';
import QuickGenerateModal from './QuickGenerateModal';
import StreamingStatusOverlay from './StreamingStatusOverlay';
import type { AdvancedGenerationResult, GeneratedImage, BrainstormSession, StyleAnalysis } from '../../types/advancedAI';

interface Tab {
  id: string;
  label: string;
  icon: string;
}

const AdvancedAI: React.FC = () => {
  // State
  const [activeTab, setActiveTab] = useState('generate');
  const [selectedProseMode, setSelectedProseMode] = useState('Excellent');
  const [showQuickGenerate, setShowQuickGenerate] = useState(false);

  // Stores
  const advancedAIStore = useAdvancedAIStore();
  const projectStore = useProjectStore();

  // Tab configuration
  const tabs: Tab[] = [
    { id: 'generate', label: 'Generate', icon: 'fas fa-pen-fancy' },
    { id: 'visualize', label: 'Visualize', icon: 'fas fa-image' },
    { id: 'brainstorm', label: 'Brainstorm', icon: 'fas fa-lightbulb' },
    { id: 'style', label: 'Style', icon: 'fas fa-palette' },
    { id: 'settings', label: 'Settings', icon: 'fas fa-cog' }
  ];

  // Computed values
  const availableProseModes = advancedAIStore.availableProseModes;
  const totalCreditsUsed = advancedAIStore.totalCreditsUsed;
  const remainingCredits = advancedAIStore.remainingCredits;
  const isGenerating = advancedAIStore.isGenerating;
  const isGeneratingImage = advancedAIStore.isGeneratingImage;
  const canGenerate = advancedAIStore.canGenerate;
  const streamingStatus = advancedAIStore.streamingStatus;
  const saliencyEnabled = advancedAIStore.saliencyEnabled;
  const ultraCreativeMode = advancedAIStore.ultraCreativeMode;
  const autoEnhancePrompts = advancedAIStore.autoEnhancePrompts;
  const clicheDetectionEnabled = advancedAIStore.clicheDetectionEnabled;
  const currentBrainstormSession = advancedAIStore.currentBrainstormSession;
  const activeStyleExamples = advancedAIStore.activeStyleExamplesList;

  const generationStatusText = (() => {
    if (isGenerating) {
      if (streamingStatus) {
        return `Generating... ${Math.round(streamingStatus.progress || 0)}%`;
      }
      return 'Generating text...';
    }
    if (isGeneratingImage) {
      return 'Generating image...';
    }
    return '';
  })();

  const canShowQuickGenerate = projectStore.currentProject && !isGenerating && !isGeneratingImage;

  // Event handlers
  const handleProseModeChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const newMode = event.target.value;
    setSelectedProseMode(newMode);
    advancedAIStore.setCurrentProseMode(newMode);
  };

  const handleGenerationComplete = (result: AdvancedGenerationResult) => {
    console.log('Generation completed:', result);
  };

  const handleImageGenerated = (image: GeneratedImage) => {
    console.log('Image generated:', image);
  };

  const handleBrainstormSessionCreated = (session: BrainstormSession) => {
    console.log('Brainstorm session created:', session);
    if (activeTab !== 'brainstorm') {
      setActiveTab('brainstorm');
    }
  };

  const handleStyleAnalyzed = (analysis: StyleAnalysis) => {
    console.log('Style analyzed:', analysis);
  };

  const handleSettingsChanged = (settings: any) => {
    console.log('Settings changed:', settings);
  };

  const handleQuickGenerate = (request: any) => {
    console.log('Quick generate:', request);
    setShowQuickGenerate(false);
  };

  const cancelStreaming = () => {
    advancedAIStore.cancelStreaming();
  };

  // Initialize on mount
  useEffect(() => {
    const initializeAdvancedAI = async () => {
      try {
        await advancedAIStore.loadProseModes();
        if (projectStore.currentProject) {
          await advancedAIStore.updateCreditUsage(projectStore.currentProject.id);
        }
      } catch (error) {
        console.error('Failed to initialize Advanced AI:', error);
      }
    };

    initializeAdvancedAI();
  }, []);

  // Watch for project changes
  useEffect(() => {
    const handleProjectChange = async () => {
      if (projectStore.currentProject) {
        await advancedAIStore.updateCreditUsage(projectStore.currentProject.id);
        await advancedAIStore.loadProjectImages(projectStore.currentProject.id);
      }
    };

    handleProjectChange();
  }, [projectStore.currentProject]);

  return (
    <div className="advanced-ai-container">
      {/* Header with Mode Selection */}
      <div className="ai-header">
        <div className="mode-selector">
          <label htmlFor="prose-mode">Prose Mode:</label>
          <select 
            id="prose-mode" 
            value={selectedProseMode}
            onChange={handleProseModeChange}
            className="prose-mode-select"
          >
            {availableProseModes.map((mode) => (
              <option key={mode.name} value={mode.name}>
                {mode.name} - {mode.description}
              </option>
            ))}
          </select>
        </div>
        
        <div className="ai-status">
          <div className="credit-usage">
            <span className="credits-used">{totalCreditsUsed} credits used</span>
            {remainingCredits !== undefined && (
              <span className="credits-remaining">
                ({remainingCredits} remaining)
              </span>
            )}
          </div>
          
          {(isGenerating || isGeneratingImage) && (
            <div className="generation-status">
              <div className="loading-spinner"></div>
              <span>{generationStatusText}</span>
            </div>
          )}
        </div>
      </div>

      {/* Main Content Tabs */}
      <div className="ai-tabs">
        <nav className="tab-nav">
          {tabs.map((tab) => (
            <button 
              key={tab.id}
              className={`tab-button ${activeTab === tab.id ? 'active' : ''}`}
              onClick={() => setActiveTab(tab.id)}
            >
              <i className={tab.icon}></i>
              {tab.label}
            </button>
          ))}
        </nav>

        {/* Text Generation Tab */}
        {activeTab === 'generate' && (
          <div className="tab-content">
            <AdvancedTextGenerator 
              proseMode={selectedProseMode}
              saliencyEnabled={saliencyEnabled}
              ultraCreative={ultraCreativeMode}
              onGenerationComplete={handleGenerationComplete}
            />
          </div>
        )}

        {/* Image Generation Tab */}
        {activeTab === 'visualize' && (
          <div className="tab-content">
            <ImageGenerator 
              onImageGenerated={handleImageGenerated}
            />
          </div>
        )}

        {/* Brainstorming Tab */}
        {activeTab === 'brainstorm' && (
          <div className="tab-content">
            <BrainstormEngine 
              currentSession={currentBrainstormSession}
              onSessionCreated={handleBrainstormSessionCreated}
            />
          </div>
        )}

        {/* Style Examples Tab */}
        {activeTab === 'style' && (
          <div className="tab-content">
            <StyleManager 
              activeExamples={activeStyleExamples}
              onStyleAnalyzed={handleStyleAnalyzed}
            />
          </div>
        )}

        {/* Settings Tab */}
        {activeTab === 'settings' && (
          <div className="tab-content">
            <AdvancedAISettings 
              saliencyEnabled={saliencyEnabled}
              ultraCreative={ultraCreativeMode}
              autoEnhance={autoEnhancePrompts}
              clicheDetection={clicheDetectionEnabled}
              onSettingsChanged={handleSettingsChanged}
            />
          </div>
        )}
      </div>

      {/* Floating Action Button for Quick Generation */}
      {canShowQuickGenerate && (
        <button 
          className="quick-generate-fab"
          onClick={() => setShowQuickGenerate(true)}
          disabled={!canGenerate}
        >
          <i className="fas fa-magic"></i>
        </button>
      )}

      {/* Quick Generate Modal */}
      {showQuickGenerate && (
        <QuickGenerateModal 
          proseMode={selectedProseMode}
          onClose={() => setShowQuickGenerate(false)}
          onGenerate={handleQuickGenerate}
        />
      )}

      {/* Streaming Status Overlay */}
      {streamingStatus && streamingStatus.status !== 'completed' && (
        <StreamingStatusOverlay 
          status={streamingStatus}
          onCancel={cancelStreaming}
        />
      )}
    </div>
  );
};

export default AdvancedAI;