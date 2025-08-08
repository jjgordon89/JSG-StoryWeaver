import React, { useState } from 'react';
import type { GeneratedImage } from '../../types/advancedAI';

interface ImageDetailModalProps {
  image: GeneratedImage;
  onClose: () => void;
  onDownload: () => void;
  onCopy: () => void;
  onRegenerate: () => void;
  onDelete: () => void;
}

const ImageDetailModal: React.FC<ImageDetailModalProps> = ({
  image,
  onClose,
  onDownload,
  onCopy,
  onRegenerate,
  onDelete
}) => {
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [newPrompt, setNewPrompt] = useState(image.prompt);
  const [isEditingPrompt, setIsEditingPrompt] = useState(false);

  const handleDeleteConfirm = () => {
    onDelete();
    setShowDeleteConfirm(false);
  };

  const handleEditPrompt = () => {
    setIsEditingPrompt(true);
  };

  const handleSavePrompt = () => {
    // This would update the image prompt in the store
    console.log('Save new prompt:', newPrompt);
    setIsEditingPrompt(false);
  };

  const handleCancelEdit = () => {
    setNewPrompt(image.prompt);
    setIsEditingPrompt(false);
  };

  const handleSetAsCover = () => {
    // This would set the image as project cover
    console.log('Set as project cover:', image.id);
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  const usageSuggestions = [
    'Use as chapter illustration',
    'Add to character gallery',
    'Include in scene description',
    'Set as book cover',
    'Share on social media',
    'Print for reference'
  ];

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="image-detail-modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Image Details</h2>
          <button className="close-btn" onClick={onClose}>
            <i className="fas fa-times"></i>
          </button>
        </div>

        <div className="modal-content">
          <div className="image-display">
            <img src={image.url} alt={image.prompt} className="detail-image" />
          </div>

          <div className="image-info">
            {/* Prompt Section */}
            <div className="info-section">
              <h3>Prompt</h3>
              {isEditingPrompt ? (
                <div className="prompt-edit">
                  <textarea
                    value={newPrompt}
                    onChange={(e) => setNewPrompt(e.target.value)}
                    rows={3}
                    className="prompt-textarea"
                  />
                  <div className="edit-actions">
                    <button className="save-btn" onClick={handleSavePrompt}>
                      <i className="fas fa-check"></i> Save
                    </button>
                    <button className="cancel-btn" onClick={handleCancelEdit}>
                      <i className="fas fa-times"></i> Cancel
                    </button>
                  </div>
                </div>
              ) : (
                <div className="prompt-display">
                  <p>{image.prompt}</p>
                  <button className="edit-btn" onClick={handleEditPrompt}>
                    <i className="fas fa-edit"></i> Edit
                  </button>
                </div>
              )}
            </div>

            {/* Enhanced Prompt */}
            {image.enhancedPrompt && (
              <div className="info-section">
                <h3>Enhanced Prompt</h3>
                <p className="enhanced-prompt">{image.enhancedPrompt}</p>
              </div>
            )}

            {/* Technical Details */}
            <div className="info-section">
              <h3>Technical Details</h3>
              <div className="tech-details">
                <div className="detail-item">
                  <span className="label">Resolution:</span>
                  <span className="value">{image.resolution}</span>
                </div>
                <div className="detail-item">
                  <span className="label">Style:</span>
                  <span className="value">{image.style}</span>
                </div>
                <div className="detail-item">
                  <span className="label">Quality:</span>
                  <span className="value">{image.quality}</span>
                </div>
                <div className="detail-item">
                  <span className="label">Aspect Ratio:</span>
                  <span className="value">{image.aspectRatio}</span>
                </div>
                <div className="detail-item">
                  <span className="label">Credits Used:</span>
                  <span className="value">{image.creditsUsed}</span>
                </div>
                <div className="detail-item">
                  <span className="label">Created:</span>
                  <span className="value">{formatDate(image.createdAt)}</span>
                </div>
              </div>
            </div>

            {/* Generation Settings */}
            <div className="info-section">
              <h3>Generation Settings</h3>
              <div className="settings-list">
                <div className="setting-item">
                  <span className="setting-label">Prompt Enhanced:</span>
                  <span className={`setting-value ${image.enhancedPrompt ? 'enabled' : 'disabled'}`}>
                    {image.enhancedPrompt ? 'Yes' : 'No'}
                  </span>
                </div>
                <div className="setting-item">
                  <span className="setting-label">Story Context Used:</span>
                  <span className={`setting-value ${image.storyContext ? 'enabled' : 'disabled'}`}>
                    {image.storyContext ? 'Yes' : 'No'}
                  </span>
                </div>
                {image.variations && image.variations.length > 0 && (
                  <div className="setting-item">
                    <span className="setting-label">Variations:</span>
                    <span className="setting-value">{image.variations.length} generated</span>
                  </div>
                )}
              </div>
            </div>

            {/* Usage Suggestions */}
            <div className="info-section">
              <h3>Usage Suggestions</h3>
              <div className="usage-suggestions">
                {usageSuggestions.map((suggestion, index) => (
                  <button key={index} className="suggestion-btn">
                    {suggestion}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>

        <div className="modal-actions">
          <div className="primary-actions">
            <button className="action-btn download" onClick={onDownload}>
              <i className="fas fa-download"></i>
              Download
            </button>
            <button className="action-btn copy" onClick={onCopy}>
              <i className="fas fa-copy"></i>
              Copy URL
            </button>
            <button className="action-btn cover" onClick={handleSetAsCover}>
              <i className="fas fa-star"></i>
              Set as Cover
            </button>
          </div>

          <div className="secondary-actions">
            <button className="action-btn regenerate" onClick={onRegenerate}>
              <i className="fas fa-redo"></i>
              Regenerate
            </button>
            <button className="action-btn edit" onClick={handleEditPrompt}>
              <i className="fas fa-edit"></i>
              Edit Prompt
            </button>
            <button 
              className="action-btn delete" 
              onClick={() => setShowDeleteConfirm(true)}
            >
              <i className="fas fa-trash"></i>
              Delete
            </button>
          </div>
        </div>

        {/* Delete Confirmation */}
        {showDeleteConfirm && (
          <div className="delete-confirm-overlay">
            <div className="delete-confirm-dialog">
              <h3>Confirm Deletion</h3>
              <p>Are you sure you want to delete this image? This action cannot be undone.</p>
              <div className="confirm-actions">
                <button className="confirm-btn" onClick={handleDeleteConfirm}>
                  <i className="fas fa-trash"></i>
                  Delete
                </button>
                <button className="cancel-btn" onClick={() => setShowDeleteConfirm(false)}>
                  <i className="fas fa-times"></i>
                  Cancel
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default ImageDetailModal;