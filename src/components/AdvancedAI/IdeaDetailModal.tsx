import React, { useState } from 'react';
import type { BrainstormIdea } from '../../types/advancedAI';

interface IdeaDetailModalProps {
  idea: BrainstormIdea;
  onClose: () => void;
  onUpdate: (idea: BrainstormIdea) => void;
  onDelete: () => void;
  onDuplicate: () => void;
}

const IdeaDetailModal: React.FC<IdeaDetailModalProps> = ({
  idea,
  onClose,
  onUpdate,
  onDelete,
  onDuplicate
}) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editedIdea, setEditedIdea] = useState({ ...idea });
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [newTag, setNewTag] = useState('');

  const categories = ['plot', 'character', 'setting', 'theme', 'conflict', 'dialogue', 'other'];
  const suggestedTags = {
    plot: ['twist', 'climax', 'subplot', 'resolution', 'conflict'],
    character: ['backstory', 'motivation', 'flaw', 'arc', 'relationship'],
    setting: ['location', 'atmosphere', 'worldbuilding', 'time', 'culture'],
    theme: ['love', 'betrayal', 'redemption', 'power', 'identity'],
    conflict: ['internal', 'external', 'moral', 'social', 'physical'],
    dialogue: ['tension', 'subtext', 'voice', 'exposition', 'emotion'],
    other: ['research', 'inspiration', 'reference', 'note', 'reminder']
  };

  const handleSave = () => {
    onUpdate(editedIdea);
    setIsEditing(false);
  };

  const handleCancel = () => {
    setEditedIdea({ ...idea });
    setIsEditing(false);
  };

  const handleRatingChange = (rating: number) => {
    const updatedIdea = { ...editedIdea, rating };
    setEditedIdea(updatedIdea);
    if (!isEditing) {
      onUpdate(updatedIdea);
    }
  };

  const handleKeeperToggle = () => {
    const updatedIdea = { ...editedIdea, isKeeper: !editedIdea.isKeeper };
    setEditedIdea(updatedIdea);
    if (!isEditing) {
      onUpdate(updatedIdea);
    }
  };

  const handleAddTag = () => {
    if (newTag.trim() && !editedIdea.tags.includes(newTag.trim())) {
      setEditedIdea({
        ...editedIdea,
        tags: [...editedIdea.tags, newTag.trim()]
      });
      setNewTag('');
    }
  };

  const handleRemoveTag = (tagToRemove: string) => {
    setEditedIdea({
      ...editedIdea,
      tags: editedIdea.tags.filter(tag => tag !== tagToRemove)
    });
  };

  const handleSuggestedTagClick = (tag: string) => {
    if (!editedIdea.tags.includes(tag)) {
      setEditedIdea({
        ...editedIdea,
        tags: [...editedIdea.tags, tag]
      });
    }
  };

  const handleDeleteConfirm = () => {
    onDelete();
    setShowDeleteConfirm(false);
  };

  const handleBuildOn = () => {
    // This would create a new brainstorm session based on this idea
    console.log('Build on idea:', idea.id);
  };

  const handleExportToStoryBible = () => {
    // This would export the idea to the story bible
    console.log('Export to story bible:', idea.id);
  };

  const getUsageSuggestions = () => {
    const suggestions = [];
    
    if (idea.category === 'character') {
      suggestions.push('Add to character profiles', 'Use in character development', 'Include in backstory');
    } else if (idea.category === 'plot') {
      suggestions.push('Add to plot outline', 'Use as scene inspiration', 'Develop into subplot');
    } else if (idea.category === 'setting') {
      suggestions.push('Add to world bible', 'Use in scene descriptions', 'Develop location details');
    } else if (idea.category === 'dialogue') {
      suggestions.push('Use in character conversations', 'Add to dialogue bank', 'Include in scene writing');
    } else {
      suggestions.push('Add to story notes', 'Use as writing prompt', 'Include in research');
    }
    
    if (idea.rating && idea.rating >= 4) {
      suggestions.push('Prioritize in next writing session', 'Share with writing group');
    }
    
    if (idea.tags.includes('twist')) {
      suggestions.push('Plan reveal carefully', 'Set up foreshadowing');
    }
    
    return suggestions;
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="idea-detail-modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Idea Details</h2>
          <div className="header-actions">
            <button
              className={`keeper-btn ${editedIdea.isKeeper ? 'active' : ''}`}
              onClick={handleKeeperToggle}
              title={editedIdea.isKeeper ? 'Remove from keepers' : 'Mark as keeper'}
            >
              <i className={`fas ${editedIdea.isKeeper ? 'fa-star' : 'fa-star-o'}`}></i>
            </button>
            <button className="close-btn" onClick={onClose}>
              <i className="fas fa-times"></i>
            </button>
          </div>
        </div>

        <div className="modal-content">
          {/* Basic Information */}
          <div className="info-section">
            <div className="field-group">
              <label htmlFor="idea-title">Title:</label>
              {isEditing ? (
                <input
                  id="idea-title"
                  type="text"
                  value={editedIdea.title}
                  onChange={(e) => setEditedIdea({ ...editedIdea, title: e.target.value })}
                  className="edit-input"
                />
              ) : (
                <h3 className="idea-title">{editedIdea.title}</h3>
              )}
            </div>

            <div className="field-group">
              <label htmlFor="idea-category">Category:</label>
              {isEditing ? (
                <select
                  id="idea-category"
                  value={editedIdea.category}
                  onChange={(e) => setEditedIdea({ ...editedIdea, category: e.target.value })}
                  className="edit-select"
                >
                  {categories.map((category) => (
                    <option key={category} value={category}>
                      {category.charAt(0).toUpperCase() + category.slice(1)}
                    </option>
                  ))}
                </select>
              ) : (
                <span className="category-badge">{editedIdea.category}</span>
              )}
            </div>

            <div className="field-group">
              <label htmlFor="idea-description">Description:</label>
              {isEditing ? (
                <textarea
                  id="idea-description"
                  value={editedIdea.description}
                  onChange={(e) => setEditedIdea({ ...editedIdea, description: e.target.value })}
                  rows={4}
                  className="edit-textarea"
                />
              ) : (
                <p className="idea-description">{editedIdea.description}</p>
              )}
            </div>

            <div className="field-group">
              <label htmlFor="idea-notes">Notes:</label>
              {isEditing ? (
                <textarea
                  id="idea-notes"
                  value={editedIdea.notes || ''}
                  onChange={(e) => setEditedIdea({ ...editedIdea, notes: e.target.value })}
                  rows={3}
                  className="edit-textarea"
                  placeholder="Add your notes here..."
                />
              ) : (
                <p className="idea-notes">{editedIdea.notes || 'No notes added'}</p>
              )}
            </div>
          </div>

          {/* Rating */}
          <div className="info-section">
            <label>Rating:</label>
            <div className="rating-section">
              {Array.from({ length: 5 }, (_, i) => (
                <button
                  key={i}
                  className={`star-btn ${i < (editedIdea.rating || 0) ? 'filled' : 'empty'}`}
                  onClick={() => handleRatingChange(i + 1)}
                >
                  <i className="fas fa-star"></i>
                </button>
              ))}
              <span className="rating-text">
                {editedIdea.rating ? `${editedIdea.rating}/5` : 'Not rated'}
              </span>
            </div>
          </div>

          {/* Tags */}
          <div className="info-section">
            <label>Tags:</label>
            <div className="tags-section">
              <div className="current-tags">
                {editedIdea.tags.map((tag, index) => (
                  <span key={index} className="tag">
                    {tag}
                    {isEditing && (
                      <button
                        className="remove-tag-btn"
                        onClick={() => handleRemoveTag(tag)}
                      >
                        <i className="fas fa-times"></i>
                      </button>
                    )}
                  </span>
                ))}
              </div>
              
              {isEditing && (
                <div className="add-tag-section">
                  <div className="add-tag-input">
                    <input
                      type="text"
                      value={newTag}
                      onChange={(e) => setNewTag(e.target.value)}
                      placeholder="Add tag..."
                      onKeyPress={(e) => e.key === 'Enter' && handleAddTag()}
                    />
                    <button onClick={handleAddTag} disabled={!newTag.trim()}>
                      <i className="fas fa-plus"></i>
                    </button>
                  </div>
                  
                  <div className="suggested-tags">
                    <span className="suggestions-label">Suggestions:</span>
                    {suggestedTags[editedIdea.category as keyof typeof suggestedTags]?.map((tag) => (
                      <button
                        key={tag}
                        className={`suggested-tag ${editedIdea.tags.includes(tag) ? 'added' : ''}`}
                        onClick={() => handleSuggestedTagClick(tag)}
                        disabled={editedIdea.tags.includes(tag)}
                      >
                        {tag}
                      </button>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>

          {/* Usage Suggestions */}
          <div className="info-section">
            <label>Usage Suggestions:</label>
            <div className="usage-suggestions">
              {getUsageSuggestions().map((suggestion, index) => (
                <button key={index} className="suggestion-btn">
                  {suggestion}
                </button>
              ))}
            </div>
          </div>

          {/* Metadata */}
          <div className="info-section">
            <label>Details:</label>
            <div className="metadata">
              <div className="meta-item">
                <span className="meta-label">Created:</span>
                <span className="meta-value">{formatDate(editedIdea.createdAt)}</span>
              </div>
              {editedIdea.updatedAt && editedIdea.updatedAt !== editedIdea.createdAt && (
                <div className="meta-item">
                  <span className="meta-label">Updated:</span>
                  <span className="meta-value">{formatDate(editedIdea.updatedAt)}</span>
                </div>
              )}
              <div className="meta-item">
                <span className="meta-label">Status:</span>
                <span className={`meta-value ${editedIdea.isKeeper ? 'keeper' : 'regular'}`}>
                  {editedIdea.isKeeper ? 'Keeper' : 'Regular'}
                </span>
              </div>
            </div>
          </div>
        </div>

        <div className="modal-actions">
          {isEditing ? (
            <div className="edit-actions">
              <button className="save-btn" onClick={handleSave}>
                <i className="fas fa-check"></i>
                Save Changes
              </button>
              <button className="cancel-btn" onClick={handleCancel}>
                <i className="fas fa-times"></i>
                Cancel
              </button>
            </div>
          ) : (
            <>
              <div className="primary-actions">
                <button className="action-btn edit" onClick={() => setIsEditing(true)}>
                  <i className="fas fa-edit"></i>
                  Edit
                </button>
                <button className="action-btn build" onClick={handleBuildOn}>
                  <i className="fas fa-plus-circle"></i>
                  Build On
                </button>
                <button className="action-btn duplicate" onClick={onDuplicate}>
                  <i className="fas fa-copy"></i>
                  Duplicate
                </button>
              </div>

              <div className="secondary-actions">
                <button className="action-btn export" onClick={handleExportToStoryBible}>
                  <i className="fas fa-download"></i>
                  Export
                </button>
                <button 
                  className="action-btn delete" 
                  onClick={() => setShowDeleteConfirm(true)}
                >
                  <i className="fas fa-trash"></i>
                  Delete
                </button>
              </div>
            </>
          )}
        </div>

        {/* Delete Confirmation */}
        {showDeleteConfirm && (
          <div className="delete-confirm-overlay">
            <div className="delete-confirm-dialog">
              <h3>Confirm Deletion</h3>
              <p>Are you sure you want to delete this idea? This action cannot be undone.</p>
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

export default IdeaDetailModal;