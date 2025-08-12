import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
// Removed non-existent imports - QuickAction and GenerationOptions don't exist in advancedAI types

interface QuickGenerateModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const QuickGenerateModal: React.FC<QuickGenerateModalProps> = ({ isOpen, onClose }) => {
  const {
    generateText,
    estimateCredits,
    isGenerating,
    generationProgress,
    settings
  } = useAdvancedAIStore();
  
  const { currentProject, currentDocument } = useProjectStore();
  
  const [selectedAction, setSelectedAction] = useState<QuickAction | null>(null);
  const [customPrompt, setCustomPrompt] = useState('');
  const [options, setOptions] = useState<GenerationOptions>({
    contextLength: settings.generation.defaultContextLength,
    outputLength: settings.generation.defaultOutputLength,
    creativityLevel: settings.generation.creativityLevel,
    useContext: true,
    enableStreaming: settings.general.enableStreaming
  });
  const [estimatedCredits, setEstimatedCredits] = useState(0);
  const [showAdvanced, setShowAdvanced] = useState(false);

  const quickActions: QuickAction[] = [
    {
      id: 'continue-scene',
      title: 'Continue Scene',
      description: 'Continue the current scene naturally',
      icon: 'fas fa-play',
      prompt: 'Continue this scene naturally, maintaining the current tone and pacing.',
      category: 'writing'
    },
    {
      id: 'describe-setting',
      title: 'Describe Setting',
      description: 'Add vivid setting description',
      icon: 'fas fa-map-marker-alt',
      prompt: 'Provide a vivid, immersive description of the current setting.',
      category: 'description'
    },
    {
      id: 'character-dialogue',
      title: 'Character Dialogue',
      description: 'Generate character dialogue',
      icon: 'fas fa-comments',
      prompt: 'Write dialogue for the characters in this scene that feels natural and advances the story.',
      category: 'dialogue'
    },
    {
      id: 'action-sequence',
      title: 'Action Sequence',
      description: 'Create an exciting action scene',
      icon: 'fas fa-bolt',
      prompt: 'Write an engaging action sequence that fits the current story context.',
      category: 'action'
    },
    {
      id: 'emotional-moment',
      title: 'Emotional Moment',
      description: 'Develop character emotions',
      icon: 'fas fa-heart',
      prompt: 'Create an emotionally resonant moment that develops the characters.',
      category: 'emotion'
    },
    {
      id: 'plot-twist',
      title: 'Plot Twist',
      description: 'Introduce an unexpected turn',
      icon: 'fas fa-surprise',
      prompt: 'Introduce a surprising but logical plot twist that enhances the story.',
      category: 'plot'
    },
    {
      id: 'transition',
      title: 'Scene Transition',
      description: 'Smooth transition to next scene',
      icon: 'fas fa-exchange-alt',
      prompt: 'Create a smooth transition to the next scene or chapter.',
      category: 'structure'
    },
    {
      id: 'backstory',
      title: 'Character Backstory',
      description: 'Reveal character history',
      icon: 'fas fa-history',
      prompt: 'Reveal relevant character backstory that adds depth to the current situation.',
      category: 'character'
    }
  ];

  useEffect(() => {
    if (isOpen) {
      setSelectedAction(null);
      setCustomPrompt('');
      setShowAdvanced(false);
    }
  }, [isOpen]);

  useEffect(() => {
    const updateEstimate = async () => {
      if (selectedAction || customPrompt) {
        const prompt = selectedAction ? selectedAction.prompt : customPrompt;
        if (prompt) {
          try {
            const credits = await estimateCredits(prompt, options);
            setEstimatedCredits(credits);
          } catch (error) {
            console.error('Error estimating credits:', error);
            setEstimatedCredits(0);
          }
        }
      } else {
        setEstimatedCredits(0);
      }
    };

    updateEstimate();
  }, [selectedAction, customPrompt, options, estimateCredits]);

  const handleActionSelect = (action: QuickAction) => {
    setSelectedAction(action);
    setCustomPrompt('');
  };

  const handleCustomPromptChange = (value: string) => {
    setCustomPrompt(value);
    if (value) {
      setSelectedAction(null);
    }
  };

  const handleGenerate = async () => {
    const prompt = selectedAction ? selectedAction.prompt : customPrompt;
    if (!prompt) return;

    try {
      await generateText(prompt, options);
      onClose();
    } catch (error) {
      console.error('Error generating text:', error);
    }
  };

  const handleOptionChange = (key: keyof GenerationOptions, value: any) => {
    setOptions(prev => ({ ...prev, [key]: value }));
  };

  const canGenerate = (selectedAction || customPrompt.trim()) && !isGenerating;

  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="quick-generate-modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3>
            <i className="fas fa-magic"></i>
            Quick Generate
          </h3>
          <button className="close-btn" onClick={onClose}>
            <i className="fas fa-times"></i>
          </button>
        </div>

        <div className="modal-content">
          {/* Quick Actions */}
          <div className="quick-actions-section">
            <h4>Quick Actions</h4>
            <div className="actions-grid">
              {quickActions.map((action) => (
                <button
                  key={action.id}
                  className={`action-card ${
                    selectedAction?.id === action.id ? 'selected' : ''
                  }`}
                  onClick={() => handleActionSelect(action)}
                >
                  <div className="action-icon">
                    <i className={action.icon}></i>
                  </div>
                  <div className="action-content">
                    <h5>{action.title}</h5>
                    <p>{action.description}</p>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Custom Prompt */}
          <div className="custom-prompt-section">
            <h4>Or Enter Custom Prompt</h4>
            <textarea
              value={customPrompt}
              onChange={(e) => handleCustomPromptChange(e.target.value)}
              placeholder="Describe what you want the AI to write..."
              rows={3}
              className="custom-prompt-input"
            />
          </div>

          {/* Quick Settings */}
          <div className="quick-settings-section">
            <div className="settings-header">
              <h4>Quick Settings</h4>
              <button
                className="toggle-advanced"
                onClick={() => setShowAdvanced(!showAdvanced)}
              >
                <i className={`fas fa-chevron-${showAdvanced ? 'up' : 'down'}`}></i>
                {showAdvanced ? 'Hide' : 'Show'} Advanced
              </button>
            </div>

            <div className="settings-grid">
              {/* Length */}
              <div className="setting-item">
                <label>Length</label>
                <div className="range-input">
                  <input
                    type="range"
                    min="100"
                    max="2000"
                    step="100"
                    value={options.outputLength}
                    onChange={(e) => handleOptionChange('outputLength', parseInt(e.target.value))}
                  />
                  <span className="range-value">{options.outputLength} tokens</span>
                </div>
              </div>

              {/* Creativity */}
              <div className="setting-item">
                <label>Creativity</label>
                <div className="range-input">
                  <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.1"
                    value={options.creativityLevel}
                    onChange={(e) => handleOptionChange('creativityLevel', parseFloat(e.target.value))}
                  />
                  <span className="range-value">{(options.creativityLevel * 100).toFixed(0)}%</span>
                </div>
              </div>

              {/* Context */}
              <div className="setting-item">
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={options.useContext}
                    onChange={(e) => handleOptionChange('useContext', e.target.checked)}
                  />
                  <span className="checkmark"></span>
                  Use story context
                </label>
              </div>

              {/* Streaming */}
              <div className="setting-item">
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={options.enableStreaming}
                    onChange={(e) => handleOptionChange('enableStreaming', e.target.checked)}
                  />
                  <span className="checkmark"></span>
                  Stream response
                </label>
              </div>
            </div>

            {/* Advanced Settings */}
            {showAdvanced && (
              <div className="advanced-settings">
                <div className="setting-item">
                  <label>Context Length</label>
                  <div className="range-input">
                    <input
                      type="range"
                      min="1000"
                      max="8000"
                      step="500"
                      value={options.contextLength}
                      onChange={(e) => handleOptionChange('contextLength', parseInt(e.target.value))}
                    />
                    <span className="range-value">{options.contextLength} tokens</span>
                  </div>
                </div>

                <div className="setting-item">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={options.enableClicheDetection || false}
                      onChange={(e) => handleOptionChange('enableClicheDetection', e.target.checked)}
                    />
                    <span className="checkmark"></span>
                    Enable cliché detection
                  </label>
                </div>

                <div className="setting-item">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={options.ultraCreativeMode || false}
                      onChange={(e) => handleOptionChange('ultraCreativeMode', e.target.checked)}
                    />
                    <span className="checkmark"></span>
                    Ultra-creative mode
                  </label>
                </div>
              </div>
            )}
          </div>

          {/* Generation Info */}
          {(selectedAction || customPrompt) && (
            <div className="generation-info">
              <div className="info-item">
                <i className="fas fa-coins"></i>
                <span>Estimated cost: {estimatedCredits} credits</span>
              </div>
              {currentDocument && (
                <div className="info-item">
                  <i className="fas fa-file-alt"></i>
                  <span>Target: {currentDocument.title}</span>
                </div>
              )}
              {options.useContext && currentProject && (
                <div className="info-item">
                  <i className="fas fa-book"></i>
                  <span>Using context from: {currentProject.name}</span>
                </div>
              )}
            </div>
          )}

          {/* Generation Progress */}
          {isGenerating && (
            <div className="generation-progress">
              <div className="progress-bar">
                <div 
                  className="progress-fill" 
                  style={{ width: `${generationProgress}%` }}
                ></div>
              </div>
              <span className="progress-text">
                Generating... {generationProgress.toFixed(0)}%
              </span>
            </div>
          )}
        </div>

        <div className="modal-actions">
          <button
            className="generate-btn"
            onClick={handleGenerate}
            disabled={!canGenerate}
          >
            {isGenerating ? (
              <>
                <i className="fas fa-spinner fa-spin"></i>
                Generating...
              </>
            ) : (
              <>
                <i className="fas fa-magic"></i>
                Generate
              </>
            )}
          </button>
          
          <button className="cancel-btn" onClick={onClose}>
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
};

export default QuickGenerateModal;