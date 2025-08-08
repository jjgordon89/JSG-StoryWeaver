import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { AdvancedGenerationRequest, AdvancedGenerationResult, GenerationSettings } from '../../types/advancedAI';

interface AdvancedTextGeneratorProps {
  proseMode: string;
  saliencyEnabled: boolean;
  ultraCreative: boolean;
  onGenerationComplete: (result: AdvancedGenerationResult) => void;
}

const AdvancedTextGenerator: React.FC<AdvancedTextGeneratorProps> = ({
  proseMode,
  saliencyEnabled,
  ultraCreative,
  onGenerationComplete
}) => {
  // State
  const [prompt, setPrompt] = useState('');
  const [contextLength, setContextLength] = useState(2000);
  const [outputLength, setOutputLength] = useState(500);
  const [creativity, setCreativity] = useState(0.7);
  const [useStoryContext, setUseStoryContext] = useState(true);
  const [enableStreaming, setEnableStreaming] = useState(true);
  const [enhancePrompt, setEnhancePrompt] = useState(false);
  const [detectCliches, setDetectCliches] = useState(true);
  const [selectedGeneration, setSelectedGeneration] = useState<AdvancedGenerationResult | null>(null);
  const [showAdvancedSettings, setShowAdvancedSettings] = useState(false);

  // Stores
  const advancedAIStore = useAdvancedAIStore();
  const projectStore = useProjectStore();

  // Computed values
  const isGenerating = advancedAIStore.isGenerating;
  const canGenerate = advancedAIStore.canGenerate;
  const recentGenerations = advancedAIStore.recentGenerations;
  const estimatedCredits = advancedAIStore.estimateCredits({
    contextLength,
    outputLength,
    proseMode,
    enhancePrompt,
    useStoryContext
  });

  // Quick prompts
  const quickPrompts = [
    'Continue the current scene',
    'Describe the setting in detail',
    'Add dialogue between characters',
    'Create a plot twist',
    'Develop character backstory',
    'Write an action sequence',
    'Add emotional depth',
    'Create tension and conflict'
  ];

  // Event handlers
  const handleGenerate = async () => {
    if (!prompt.trim() || !canGenerate) return;

    const request: AdvancedGenerationRequest = {
      prompt: prompt.trim(),
      proseMode,
      settings: {
        contextLength,
        outputLength,
        creativity,
        useStoryContext,
        enableStreaming,
        enhancePrompt,
        detectCliches,
        saliencyEnabled,
        ultraCreative
      },
      projectId: projectStore.currentProject?.id
    };

    try {
      const result = await advancedAIStore.generateText(request);
      onGenerationComplete(result);
      setPrompt(''); // Clear prompt after successful generation
    } catch (error) {
      console.error('Generation failed:', error);
    }
  };

  const handleQuickPrompt = (quickPrompt: string) => {
    setPrompt(quickPrompt);
  };

  const handleRegenerateWithSettings = async (generation: AdvancedGenerationResult) => {
    const request: AdvancedGenerationRequest = {
      prompt: generation.originalPrompt,
      proseMode: generation.proseMode,
      settings: generation.settings,
      projectId: projectStore.currentProject?.id
    };

    try {
      const result = await advancedAIStore.generateText(request);
      onGenerationComplete(result);
    } catch (error) {
      console.error('Regeneration failed:', error);
    }
  };

  const handleInsertText = (text: string) => {
    // This would integrate with the document editor
    console.log('Insert text:', text);
  };

  const handleCopyText = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleSaveGeneration = (generation: AdvancedGenerationResult) => {
    advancedAIStore.saveGeneration(generation);
  };

  const handleDeleteGeneration = (generationId: string) => {
    advancedAIStore.deleteGeneration(generationId);
  };

  return (
    <div className="advanced-text-generator">
      {/* Generation Form */}
      <div className="generation-form">
        <div className="prompt-section">
          <label htmlFor="prompt">Prompt:</label>
          <textarea
            id="prompt"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Enter your generation prompt..."
            rows={4}
            className="prompt-textarea"
          />
          
          {/* Quick Prompts */}
          <div className="quick-prompts">
            <span className="quick-prompts-label">Quick prompts:</span>
            <div className="quick-prompt-buttons">
              {quickPrompts.map((quickPrompt, index) => (
                <button
                  key={index}
                  className="quick-prompt-btn"
                  onClick={() => handleQuickPrompt(quickPrompt)}
                >
                  {quickPrompt}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Basic Settings */}
        <div className="basic-settings">
          <div className="setting-group">
            <label htmlFor="output-length">Output Length:</label>
            <input
              id="output-length"
              type="range"
              min="100"
              max="2000"
              step="50"
              value={outputLength}
              onChange={(e) => setOutputLength(Number(e.target.value))}
            />
            <span className="setting-value">{outputLength} words</span>
          </div>

          <div className="setting-group">
            <label htmlFor="creativity">Creativity:</label>
            <input
              id="creativity"
              type="range"
              min="0.1"
              max="1.0"
              step="0.1"
              value={creativity}
              onChange={(e) => setCreativity(Number(e.target.value))}
            />
            <span className="setting-value">{Math.round(creativity * 100)}%</span>
          </div>

          <div className="setting-checkboxes">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={useStoryContext}
                onChange={(e) => setUseStoryContext(e.target.checked)}
              />
              Use story context
            </label>
            
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={enableStreaming}
                onChange={(e) => setEnableStreaming(e.target.checked)}
              />
              Enable streaming
            </label>
          </div>
        </div>

        {/* Advanced Settings Toggle */}
        <button
          className="advanced-settings-toggle"
          onClick={() => setShowAdvancedSettings(!showAdvancedSettings)}
        >
          <i className={`fas fa-chevron-${showAdvancedSettings ? 'up' : 'down'}`}></i>
          Advanced Settings
        </button>

        {/* Advanced Settings */}
        {showAdvancedSettings && (
          <div className="advanced-settings">
            <div className="setting-group">
              <label htmlFor="context-length">Context Length:</label>
              <input
                id="context-length"
                type="range"
                min="500"
                max="8000"
                step="100"
                value={contextLength}
                onChange={(e) => setContextLength(Number(e.target.value))}
              />
              <span className="setting-value">{contextLength} words</span>
            </div>

            <div className="setting-checkboxes">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={enhancePrompt}
                  onChange={(e) => setEnhancePrompt(e.target.checked)}
                />
                Enhance prompt with AI
              </label>
              
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={detectCliches}
                  onChange={(e) => setDetectCliches(e.target.checked)}
                />
                Detect and avoid clichés
              </label>
            </div>
          </div>
        )}

        {/* Generation Controls */}
        <div className="generation-controls">
          <div className="credit-estimate">
            Estimated cost: <strong>{estimatedCredits} credits</strong>
          </div>
          
          <button
            className="generate-btn"
            onClick={handleGenerate}
            disabled={!prompt.trim() || !canGenerate || isGenerating}
          >
            {isGenerating ? (
              <>
                <div className="loading-spinner"></div>
                Generating...
              </>
            ) : (
              <>
                <i className="fas fa-magic"></i>
                Generate Text
              </>
            )}
          </button>
        </div>
      </div>

      {/* Recent Generations */}
      {recentGenerations.length > 0 && (
        <div className="recent-generations">
          <h3>Recent Generations</h3>
          <div className="generations-list">
            {recentGenerations.map((generation) => (
              <div key={generation.id} className="generation-item">
                <div className="generation-header">
                  <div className="generation-info">
                    <span className="generation-prompt">{generation.originalPrompt}</span>
                    <span className="generation-meta">
                      {generation.proseMode} • {generation.wordCount} words • {generation.creditsUsed} credits
                    </span>
                  </div>
                  <div className="generation-actions">
                    <button
                      className="action-btn"
                      onClick={() => handleCopyText(generation.text)}
                      title="Copy text"
                    >
                      <i className="fas fa-copy"></i>
                    </button>
                    <button
                      className="action-btn"
                      onClick={() => handleInsertText(generation.text)}
                      title="Insert into document"
                    >
                      <i className="fas fa-plus"></i>
                    </button>
                    <button
                      className="action-btn"
                      onClick={() => handleRegenerateWithSettings(generation)}
                      title="Regenerate with same settings"
                    >
                      <i className="fas fa-redo"></i>
                    </button>
                    <button
                      className="action-btn"
                      onClick={() => handleSaveGeneration(generation)}
                      title="Save generation"
                    >
                      <i className="fas fa-bookmark"></i>
                    </button>
                    <button
                      className="action-btn delete"
                      onClick={() => handleDeleteGeneration(generation.id)}
                      title="Delete generation"
                    >
                      <i className="fas fa-trash"></i>
                    </button>
                  </div>
                </div>
                
                <div className="generation-content">
                  <p>{generation.text}</p>
                </div>
                
                {generation.enhancedPrompt && (
                  <div className="enhanced-prompt">
                    <strong>Enhanced prompt:</strong> {generation.enhancedPrompt}
                  </div>
                )}
                
                {generation.clicheWarnings && generation.clicheWarnings.length > 0 && (
                  <div className="cliche-warnings">
                    <strong>Cliché warnings:</strong>
                    <ul>
                      {generation.clicheWarnings.map((warning, index) => (
                        <li key={index}>{warning}</li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default AdvancedTextGenerator;