import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { StreamingStatus, GenerationResult } from '../../types/advancedAI';

interface StreamingStatusOverlayProps {
  isVisible: boolean;
  onClose: () => void;
}

const StreamingStatusOverlay: React.FC<StreamingStatusOverlayProps> = ({ isVisible, onClose }) => {
  const {
    streamingStatus,
    generationResult,
    isGenerating,
    generationProgress,
    cancelGeneration,
    saveGeneratedContent,
    copyToClipboard
  } = useAdvancedAIStore();
  
  const { insertTextAtCursor, currentDocument } = useProjectStore();
  
  const [showActions, setShowActions] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [saveLocation, setSaveLocation] = useState<'document' | 'snippet' | 'note'>('document');
  const [snippetTitle, setSnippetTitle] = useState('');
  const [showSaveOptions, setShowSaveOptions] = useState(false);

  useEffect(() => {
    if (generationResult && !isGenerating) {
      setShowActions(true);
    } else {
      setShowActions(false);
    }
  }, [generationResult, isGenerating]);

  useEffect(() => {
    if (generationResult?.content) {
      // Auto-generate snippet title from first few words
      const words = generationResult.content.split(' ').slice(0, 5).join(' ');
      setSnippetTitle(words + (generationResult.content.split(' ').length > 5 ? '...' : ''));
    }
  }, [generationResult]);

  const handleCancel = () => {
    if (isGenerating) {
      cancelGeneration();
    }
    onClose();
  };

  const handleCopy = async () => {
    if (generationResult?.content) {
      try {
        await copyToClipboard(generationResult.content);
        // Show success feedback
      } catch (error) {
        console.error('Error copying to clipboard:', error);
      }
    }
  };

  const handleInsert = () => {
    if (generationResult?.content && currentDocument) {
      insertTextAtCursor(generationResult.content);
      onClose();
    }
  };

  const handleSave = async () => {
    if (!generationResult?.content) return;
    
    setIsSaving(true);
    try {
      const saveData = {
        content: generationResult.content,
        location: saveLocation,
        title: saveLocation === 'snippet' ? snippetTitle : undefined,
        metadata: {
          prompt: generationResult.prompt,
          timestamp: new Date().toISOString(),
          settings: generationResult.settings,
          credits: generationResult.creditsUsed
        }
      };
      
      await saveGeneratedContent(saveData);
      setShowSaveOptions(false);
      onClose();
    } catch (error) {
      console.error('Error saving content:', error);
    } finally {
      setIsSaving(false);
    }
  };

  const getStatusIcon = () => {
    if (isGenerating) {
      return 'fas fa-spinner fa-spin';
    }
    if (streamingStatus?.error) {
      return 'fas fa-exclamation-triangle';
    }
    if (generationResult) {
      return 'fas fa-check-circle';
    }
    return 'fas fa-magic';
  };

  const getStatusText = () => {
    if (isGenerating) {
      return streamingStatus?.currentPhase || 'Generating...';
    }
    if (streamingStatus?.error) {
      return 'Generation failed';
    }
    if (generationResult) {
      return 'Generation complete';
    }
    return 'Ready to generate';
  };

  const getStatusClass = () => {
    if (isGenerating) return 'generating';
    if (streamingStatus?.error) return 'error';
    if (generationResult) return 'complete';
    return 'ready';
  };

  if (!isVisible) return null;

  return (
    <div className="streaming-status-overlay">
      <div className="overlay-content">
        {/* Header */}
        <div className="overlay-header">
          <div className={`status-indicator ${getStatusClass()}`}>
            <i className={getStatusIcon()}></i>
            <span>{getStatusText()}</span>
          </div>
          
          <button className="close-btn" onClick={handleCancel}>
            <i className="fas fa-times"></i>
          </button>
        </div>

        {/* Progress Bar */}
        {isGenerating && (
          <div className="progress-section">
            <div className="progress-bar">
              <div 
                className="progress-fill" 
                style={{ width: `${generationProgress}%` }}
              ></div>
            </div>
            <div className="progress-details">
              <span className="progress-text">{generationProgress.toFixed(0)}% complete</span>
              {streamingStatus?.estimatedTimeRemaining && (
                <span className="time-remaining">
                  ~{Math.ceil(streamingStatus.estimatedTimeRemaining / 1000)}s remaining
                </span>
              )}
            </div>
          </div>
        )}

        {/* Streaming Content */}
        {(streamingStatus?.partialContent || generationResult?.content) && (
          <div className="content-section">
            <div className="content-header">
              <h4>Generated Content</h4>
              {generationResult && (
                <div className="content-stats">
                  <span>{generationResult.content.split(' ').length} words</span>
                  <span>{generationResult.content.length} characters</span>
                  {generationResult.creditsUsed && (
                    <span>{generationResult.creditsUsed} credits used</span>
                  )}
                </div>
              )}
            </div>
            
            <div className="content-display">
              <div className="content-text">
                {generationResult?.content || streamingStatus?.partialContent}
                {isGenerating && (
                  <span className="cursor-blink">|</span>
                )}
              </div>
            </div>
          </div>
        )}

        {/* Error Display */}
        {streamingStatus?.error && (
          <div className="error-section">
            <div className="error-header">
              <i className="fas fa-exclamation-triangle"></i>
              <h4>Generation Error</h4>
            </div>
            <div className="error-content">
              <p>{streamingStatus.error.message}</p>
              {streamingStatus.error.details && (
                <details>
                  <summary>Error Details</summary>
                  <pre>{JSON.stringify(streamingStatus.error.details, null, 2)}</pre>
                </details>
              )}
            </div>
          </div>
        )}

        {/* Cliché Detection Results */}
        {generationResult?.clicheDetection && generationResult.clicheDetection.length > 0 && (
          <div className="cliche-section">
            <div className="cliche-header">
              <i className="fas fa-search"></i>
              <h4>Cliché Detection</h4>
              <span className="cliche-count">
                {generationResult.clicheDetection.length} potential clichés found
              </span>
            </div>
            <div className="cliche-list">
              {generationResult.clicheDetection.map((cliche, index) => (
                <div key={index} className="cliche-item">
                  <div className="cliche-text">"{cliche.text}"</div>
                  <div className="cliche-details">
                    <span className={`confidence ${cliche.confidence > 0.7 ? 'high' : cliche.confidence > 0.4 ? 'medium' : 'low'}`}>
                      {(cliche.confidence * 100).toFixed(0)}% confidence
                    </span>
                    {cliche.suggestion && (
                      <span className="suggestion">Suggestion: {cliche.suggestion}</span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Smart Context Info */}
        {generationResult?.smartContext && (
          <div className="context-section">
            <div className="context-header">
              <i className="fas fa-brain"></i>
              <h4>Smart Context Used</h4>
            </div>
            <div className="context-details">
              <div className="context-sources">
                {generationResult.smartContext.sources.map((source, index) => (
                  <div key={index} className="context-source">
                    <i className={`fas fa-${source.type === 'character' ? 'user' : source.type === 'location' ? 'map-marker' : 'book'}`}></i>
                    <span>{source.name}</span>
                    <span className="relevance">{(source.relevance * 100).toFixed(0)}% relevant</span>
                  </div>
                ))}
              </div>
              {generationResult.smartContext.insights && (
                <div className="context-insights">
                  <h5>Key Insights Applied:</h5>
                  <ul>
                    {generationResult.smartContext.insights.map((insight, index) => (
                      <li key={index}>{insight}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          </div>
        )}

        {/* Generation Statistics */}
        {generationResult && (
          <div className="stats-section">
            <div className="stats-grid">
              <div className="stat-item">
                <i className="fas fa-clock"></i>
                <span>Generation Time</span>
                <span>{(generationResult.generationTime / 1000).toFixed(1)}s</span>
              </div>
              <div className="stat-item">
                <i className="fas fa-coins"></i>
                <span>Credits Used</span>
                <span>{generationResult.creditsUsed || 0}</span>
              </div>
              <div className="stat-item">
                <i className="fas fa-file-alt"></i>
                <span>Word Count</span>
                <span>{generationResult.content.split(' ').length}</span>
              </div>
              <div className="stat-item">
                <i className="fas fa-chart-line"></i>
                <span>Quality Score</span>
                <span>{generationResult.qualityScore ? (generationResult.qualityScore * 100).toFixed(0) + '%' : 'N/A'}</span>
              </div>
            </div>
          </div>
        )}

        {/* Actions */}
        {showActions && generationResult && (
          <div className="actions-section">
            <div className="primary-actions">
              <button className="copy-btn" onClick={handleCopy}>
                <i className="fas fa-copy"></i>
                Copy
              </button>
              
              {currentDocument && (
                <button className="insert-btn" onClick={handleInsert}>
                  <i className="fas fa-plus"></i>
                  Insert
                </button>
              )}
              
              <button 
                className="save-btn" 
                onClick={() => setShowSaveOptions(!showSaveOptions)}
              >
                <i className="fas fa-save"></i>
                Save
              </button>
            </div>

            {/* Save Options */}
            {showSaveOptions && (
              <div className="save-options">
                <div className="save-location">
                  <label>Save to:</label>
                  <div className="location-options">
                    <label className="radio-label">
                      <input
                        type="radio"
                        name="saveLocation"
                        value="document"
                        checked={saveLocation === 'document'}
                        onChange={(e) => setSaveLocation(e.target.value as any)}
                      />
                      <span>Current Document</span>
                    </label>
                    <label className="radio-label">
                      <input
                        type="radio"
                        name="saveLocation"
                        value="snippet"
                        checked={saveLocation === 'snippet'}
                        onChange={(e) => setSaveLocation(e.target.value as any)}
                      />
                      <span>New Snippet</span>
                    </label>
                    <label className="radio-label">
                      <input
                        type="radio"
                        name="saveLocation"
                        value="note"
                        checked={saveLocation === 'note'}
                        onChange={(e) => setSaveLocation(e.target.value as any)}
                      />
                      <span>Story Notes</span>
                    </label>
                  </div>
                </div>

                {saveLocation === 'snippet' && (
                  <div className="snippet-title">
                    <label htmlFor="snippet-title">Snippet Title:</label>
                    <input
                      id="snippet-title"
                      type="text"
                      value={snippetTitle}
                      onChange={(e) => setSnippetTitle(e.target.value)}
                      placeholder="Enter snippet title..."
                    />
                  </div>
                )}

                <div className="save-actions">
                  <button 
                    className="confirm-save-btn" 
                    onClick={handleSave}
                    disabled={isSaving || (saveLocation === 'snippet' && !snippetTitle.trim())}
                  >
                    {isSaving ? (
                      <>
                        <i className="fas fa-spinner fa-spin"></i>
                        Saving...
                      </>
                    ) : (
                      <>
                        <i className="fas fa-check"></i>
                        Confirm Save
                      </>
                    )}
                  </button>
                  <button 
                    className="cancel-save-btn" 
                    onClick={() => setShowSaveOptions(false)}
                  >
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </div>
        )}

        {/* Cancel/Close Button */}
        <div className="overlay-footer">
          {isGenerating ? (
            <button className="cancel-generation-btn" onClick={handleCancel}>
              <i className="fas fa-stop"></i>
              Cancel Generation
            </button>
          ) : (
            <button className="close-overlay-btn" onClick={onClose}>
              <i className="fas fa-times"></i>
              Close
            </button>
          )}
        </div>
      </div>
    </div>
  );
};

export default StreamingStatusOverlay;