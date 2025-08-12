import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import { useProjectStore } from '../../stores/projectStore';
import type { AdvancedGenerationResult } from '../../types/advancedAI';

interface StreamingStatusOverlayProps {
  isVisible: boolean;
  onClose: () => void;
}

const StreamingStatusOverlay: React.FC<StreamingStatusOverlayProps> = ({ isVisible, onClose }) => {
  const {
    streamingStatus,
    lastGenerationResult,
    isGenerating
  } = useAdvancedAIStore();
  
  const { insertTextAtCursor, currentDocument } = useProjectStore();
  
  const [showActions, setShowActions] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [saveLocation, setSaveLocation] = useState<'document' | 'snippet' | 'note'>('document');
  const [snippetTitle, setSnippetTitle] = useState('');
  const [showSaveOptions, setShowSaveOptions] = useState(false);

  useEffect(() => {
    if (lastGenerationResult && !isGenerating) {
      setShowActions(true);
    } else {
      setShowActions(false);
    }
  }, [lastGenerationResult, isGenerating]);

  useEffect(() => {
    if (lastGenerationResult?.generated_text) {
      // Auto-generate snippet title from first few words
      const words = lastGenerationResult.generated_text.split(' ').slice(0, 5).join(' ');
       setSnippetTitle(words + (lastGenerationResult.generated_text.split(' ').length > 5 ? '...' : ''));
    }
  }, [lastGenerationResult]);

  const handleCancel = () => {
    if (isGenerating) {
      // TODO: Implement cancelGeneration in advancedAIStore
      console.log('Cancel generation requested');
    }
    onClose();
  };

  const handleCopy = async () => {
    if (lastGenerationResult?.generated_text) {
      try {
        // TODO: Implement copyToClipboard in advancedAIStore
         await navigator.clipboard.writeText(lastGenerationResult.generated_text);
        // Show success feedback
      } catch (error) {
        console.error('Error copying to clipboard:', error);
      }
    }
  };

  const handleInsert = () => {
    if (lastGenerationResult?.generated_text && currentDocument) {
      insertTextAtCursor(lastGenerationResult.generated_text);
      onClose();
    }
  };

  const handleSave = async () => {
    if (!lastGenerationResult?.content) return;
    
    setIsSaving(true);
    try {
      const saveData = {
        content: lastGenerationResult.generated_text,
        location: saveLocation,
        title: saveLocation === 'snippet' ? snippetTitle : undefined,
        metadata: {
          prompt: 'N/A', // TODO: Store original prompt
          timestamp: new Date().toISOString(),
          settings: lastGenerationResult.prose_mode_used,
          credits: lastGenerationResult.credits_used
        }
      };
      
      // TODO: Implement saveGeneratedContent in advancedAIStore
      console.log('Save data:', saveData);
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
    if (streamingStatus?.error_message) {
      return 'fas fa-exclamation-triangle';
    }
    if (lastGenerationResult) {
      return 'fas fa-check-circle';
    }
    return 'fas fa-magic';
  };

  const getStatusText = () => {
    if (isGenerating) {
      return 'Generating...';
    }
    if (streamingStatus?.error_message) {
      return 'Generation failed';
    }
    if (lastGenerationResult) {
      return 'Generation complete';
    }
    return 'Ready to generate';
  };

  const getStatusClass = () => {
    if (isGenerating) return 'generating';
    if (streamingStatus?.error_message) return 'error';
    if (lastGenerationResult) return 'complete';
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
                style={{ width: `${streamingStatus?.progress || 0}%` }}
              ></div>
            </div>
            <div className="progress-details">
              <span className="progress-text">{(streamingStatus?.progress || 0).toFixed(0)}% complete</span>
              {streamingStatus?.estimated_completion && (
                <span className="time-remaining">
                  ETA: {streamingStatus.estimated_completion}
                </span>
              )}
            </div>
          </div>
        )}

        {/* Streaming Content */}
        {(streamingStatus?.current_text || lastGenerationResult?.content) && (
          <div className="content-section">
            <div className="content-header">
              <h4>Generated Content</h4>
              {lastGenerationResult && (
                <div className="content-stats">
                  <span>{lastGenerationResult.generated_text.split(' ').length} words</span>
                  <span>{lastGenerationResult.generated_text.length} characters</span>
                  {lastGenerationResult.credits_used && (
                    <span>{lastGenerationResult.credits_used} credits used</span>
                  )}
                </div>
              )}
            </div>
            
            <div className="content-display">
              <div className="content-text">
                {lastGenerationResult?.generated_text || streamingStatus?.current_text}
                {isGenerating && (
                  <span className="cursor-blink">|</span>
                )}
              </div>
            </div>
          </div>
        )}

        {/* Error Display */}
        {streamingStatus?.error_message && (
          <div className="error-section">
            <div className="error-header">
              <i className="fas fa-exclamation-triangle"></i>
              <h4>Generation Error</h4>
            </div>
            <div className="error-content">
              <p>{streamingStatus.error_message}</p>
              {streamingStatus.error_message && (
                <details>
                  <summary>Error Details</summary>
                  <pre>{streamingStatus.error_message}</pre>
                </details>
              )}
            </div>
          </div>
        )}

        {/* Cliché Detection Results */}
        {lastGenerationResult?.cliche_detection && lastGenerationResult.cliche_detection.cliches_found.length > 0 && (
          <div className="cliche-section">
            <div className="cliche-header">
              <i className="fas fa-search"></i>
              <h4>Cliché Detection</h4>
              <span className="cliche-count">
                {lastGenerationResult.cliche_detection.cliches_found.length} potential clichés found
              </span>
            </div>
            <div className="cliche-list">
              {lastGenerationResult.cliche_detection.cliches_found.map((cliche: any, index: number) => (
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

        {/* Saliency Context Info */}
        {lastGenerationResult?.saliency_context && (
          <div className="context-section">
            <div className="context-header">
              <i className="fas fa-brain"></i>
              <h4>Saliency Context Used</h4>
            </div>
            <div className="context-details">
              <div className="context-info">
                <span>Project ID: {lastGenerationResult.saliency_context.project_id}</span>
                <span>Token Count: {lastGenerationResult.saliency_context.token_count}</span>
              </div>
            </div>
          </div>
        )}

        {/* Generation Statistics */}
        {lastGenerationResult && (
          <div className="stats-section">
            <div className="stats-grid">
              <div className="stat-item">
                <i className="fas fa-clock"></i>
                <span>Generation Time</span>
                <span>N/A</span> {/* TODO: Add generation time tracking */}
              </div>
              <div className="stat-item">
                <i className="fas fa-coins"></i>
                <span>Credits Used</span>
                <span>{lastGenerationResult.credits_used || 0}</span>
              </div>
              <div className="stat-item">
                <i className="fas fa-file-alt"></i>
                <span>Word Count</span>
                <span>{lastGenerationResult.generated_text.split(' ').length}</span>
              </div>
              <div className="stat-item">
                <i className="fas fa-chart-line"></i>
                <span>Quality Score</span>
                <span>{lastGenerationResult.cliche_detection?.overall_score ? (lastGenerationResult.cliche_detection.overall_score * 100).toFixed(0) + '%' : 'N/A'}</span>
              </div>
            </div>
          </div>
        )}

        {/* Actions */}
        {showActions && lastGenerationResult && (
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