import React, { useState, useEffect } from 'react';
import type { StyleExample } from '../../types/advancedAI';

interface StyleExampleModalProps {
  example?: StyleExample | null;
  onSave: (example: Partial<StyleExample>) => void;
  onClose: () => void;
}

const StyleExampleModal: React.FC<StyleExampleModalProps> = ({
  example,
  onSave,
  onClose
}) => {
  const [formData, setFormData] = useState({
    name: '',
    category: 'narrative',
    content: '',
    characteristics: {
      tone: 'neutral',
      complexity: 'moderate',
      pacing: 'moderate',
      perspective: 'third-person'
    },
    tags: [] as string[],
    notes: ''
  });
  
  const [newTag, setNewTag] = useState('');
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false);
  const [showUnsavedWarning, setShowUnsavedWarning] = useState(false);
  const [contentStats, setContentStats] = useState({
    words: 0,
    characters: 0,
    sentences: 0
  });

  const categories = ['narrative', 'dialogue', 'description', 'action', 'introspection', 'other'];
  const tones = ['formal', 'casual', 'dramatic', 'humorous', 'dark', 'light', 'mysterious', 'romantic', 'neutral'];
  const complexities = ['simple', 'moderate', 'complex'];
  const pacings = ['slow', 'moderate', 'fast'];
  const perspectives = ['first-person', 'second-person', 'third-person', 'omniscient'];
  
  const suggestedTags = {
    narrative: ['descriptive', 'flowing', 'engaging', 'immersive', 'vivid'],
    dialogue: ['natural', 'witty', 'emotional', 'realistic', 'sharp'],
    description: ['detailed', 'atmospheric', 'sensory', 'poetic', 'precise'],
    action: ['dynamic', 'intense', 'fast-paced', 'visceral', 'exciting'],
    introspection: ['thoughtful', 'deep', 'reflective', 'psychological', 'intimate'],
    other: ['unique', 'experimental', 'creative', 'innovative', 'distinctive']
  };

  useEffect(() => {
    if (example) {
      setFormData({
        name: example.name,
        category: example.category,
        content: example.content,
        characteristics: { ...example.characteristics },
        tags: [...example.tags],
        notes: example.notes || ''
      });
    }
  }, [example]);

  useEffect(() => {
    const stats = calculateContentStats(formData.content);
    setContentStats(stats);
  }, [formData.content]);

  useEffect(() => {
    const handleBeforeUnload = (e: BeforeUnloadEvent) => {
      if (hasUnsavedChanges) {
        e.preventDefault();
        e.returnValue = '';
      }
    };

    window.addEventListener('beforeunload', handleBeforeUnload);
    return () => window.removeEventListener('beforeunload', handleBeforeUnload);
  }, [hasUnsavedChanges]);

  const calculateContentStats = (content: string) => {
    const words = content.trim() ? content.trim().split(/\s+/).length : 0;
    const characters = content.length;
    const sentences = content.trim() ? content.split(/[.!?]+/).filter(s => s.trim()).length : 0;
    
    return { words, characters, sentences };
  };

  const handleInputChange = (field: string, value: any) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));
    setHasUnsavedChanges(true);
  };

  const handleCharacteristicChange = (field: string, value: string) => {
    setFormData(prev => ({
      ...prev,
      characteristics: {
        ...prev.characteristics,
        [field]: value
      }
    }));
    setHasUnsavedChanges(true);
  };

  const handleAddTag = () => {
    if (newTag.trim() && !formData.tags.includes(newTag.trim())) {
      setFormData(prev => ({
        ...prev,
        tags: [...prev.tags, newTag.trim()]
      }));
      setNewTag('');
      setHasUnsavedChanges(true);
    }
  };

  const handleRemoveTag = (tagToRemove: string) => {
    setFormData(prev => ({
      ...prev,
      tags: prev.tags.filter(tag => tag !== tagToRemove)
    }));
    setHasUnsavedChanges(true);
  };

  const handleSuggestedTagClick = (tag: string) => {
    if (!formData.tags.includes(tag)) {
      setFormData(prev => ({
        ...prev,
        tags: [...prev.tags, tag]
      }));
      setHasUnsavedChanges(true);
    }
  };

  const handleAIAnalysis = async () => {
    if (!formData.content.trim()) return;
    
    setIsAnalyzing(true);
    try {
      // Simulate AI analysis - in real implementation, this would call the backend
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Mock analysis results
      const analysisResults = {
        detectedTone: 'dramatic',
        detectedComplexity: 'complex',
        detectedPacing: 'fast',
        suggestedTags: ['intense', 'emotional', 'vivid'],
        complexityScore: 0.75
      };
      
      // Update form with analysis results
      setFormData(prev => ({
        ...prev,
        characteristics: {
          ...prev.characteristics,
          tone: analysisResults.detectedTone,
          complexity: analysisResults.detectedComplexity,
          pacing: analysisResults.detectedPacing
        },
        tags: [...new Set([...prev.tags, ...analysisResults.suggestedTags])]
      }));
      
      setHasUnsavedChanges(true);
    } catch (error) {
      console.error('Error analyzing style:', error);
    } finally {
      setIsAnalyzing(false);
    }
  };

  const handleSave = () => {
    if (!formData.name.trim() || !formData.content.trim()) {
      return;
    }
    
    onSave(formData);
    setHasUnsavedChanges(false);
  };

  const handleClose = () => {
    if (hasUnsavedChanges) {
      setShowUnsavedWarning(true);
    } else {
      onClose();
    }
  };

  const handleForceClose = () => {
    setHasUnsavedChanges(false);
    onClose();
  };

  const isFormValid = formData.name.trim() && formData.content.trim();

  return (
    <div className="modal-overlay" onClick={handleClose}>
      <div className="style-example-modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>{example ? 'Edit Style Example' : 'Add Style Example'}</h2>
          <button className="close-btn" onClick={handleClose}>
            <i className="fas fa-times"></i>
          </button>
        </div>

        <div className="modal-content">
          {/* Basic Information */}
          <div className="form-section">
            <h3>Basic Information</h3>
            
            <div className="form-group">
              <label htmlFor="example-name">Name *</label>
              <input
                id="example-name"
                type="text"
                value={formData.name}
                onChange={(e) => handleInputChange('name', e.target.value)}
                placeholder="Enter a descriptive name for this style example"
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="example-category">Category *</label>
              <select
                id="example-category"
                value={formData.category}
                onChange={(e) => handleInputChange('category', e.target.value)}
                required
              >
                {categories.map((category) => (
                  <option key={category} value={category}>
                    {category.charAt(0).toUpperCase() + category.slice(1)}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Content */}
          <div className="form-section">
            <h3>Content</h3>
            
            <div className="form-group">
              <div className="content-header">
                <label htmlFor="example-content">Text Content *</label>
                <div className="content-stats">
                  <span className="stat">{contentStats.words} words</span>
                  <span className="stat">{contentStats.characters} characters</span>
                  <span className="stat">{contentStats.sentences} sentences</span>
                </div>
              </div>
              <textarea
                id="example-content"
                value={formData.content}
                onChange={(e) => handleInputChange('content', e.target.value)}
                placeholder="Paste or type the text content that represents this writing style..."
                rows={8}
                required
              />
            </div>
          </div>

          {/* Style Characteristics */}
          <div className="form-section">
            <h3>Style Characteristics</h3>
            
            <div className="characteristics-grid">
              <div className="form-group">
                <label htmlFor="char-tone">Tone</label>
                <select
                  id="char-tone"
                  value={formData.characteristics.tone}
                  onChange={(e) => handleCharacteristicChange('tone', e.target.value)}
                >
                  {tones.map((tone) => (
                    <option key={tone} value={tone}>
                      {tone.charAt(0).toUpperCase() + tone.slice(1)}
                    </option>
                  ))}
                </select>
              </div>

              <div className="form-group">
                <label htmlFor="char-complexity">Complexity</label>
                <select
                  id="char-complexity"
                  value={formData.characteristics.complexity}
                  onChange={(e) => handleCharacteristicChange('complexity', e.target.value)}
                >
                  {complexities.map((complexity) => (
                    <option key={complexity} value={complexity}>
                      {complexity.charAt(0).toUpperCase() + complexity.slice(1)}
                    </option>
                  ))}
                </select>
              </div>

              <div className="form-group">
                <label htmlFor="char-pacing">Pacing</label>
                <select
                  id="char-pacing"
                  value={formData.characteristics.pacing}
                  onChange={(e) => handleCharacteristicChange('pacing', e.target.value)}
                >
                  {pacings.map((pacing) => (
                    <option key={pacing} value={pacing}>
                      {pacing.charAt(0).toUpperCase() + pacing.slice(1)}
                    </option>
                  ))}
                </select>
              </div>

              <div className="form-group">
                <label htmlFor="char-perspective">Perspective</label>
                <select
                  id="char-perspective"
                  value={formData.characteristics.perspective}
                  onChange={(e) => handleCharacteristicChange('perspective', e.target.value)}
                >
                  {perspectives.map((perspective) => (
                    <option key={perspective} value={perspective}>
                      {perspective.charAt(0).toUpperCase() + perspective.slice(1).replace('-', ' ')}
                    </option>
                  ))}
                </select>
              </div>
            </div>
          </div>

          {/* Tags */}
          <div className="form-section">
            <h3>Tags</h3>
            
            <div className="tags-section">
              <div className="current-tags">
                {formData.tags.map((tag, index) => (
                  <span key={index} className="tag">
                    {tag}
                    <button
                      className="remove-tag-btn"
                      onClick={() => handleRemoveTag(tag)}
                      type="button"
                    >
                      <i className="fas fa-times"></i>
                    </button>
                  </span>
                ))}
              </div>
              
              <div className="add-tag-section">
                <div className="add-tag-input">
                  <input
                    type="text"
                    value={newTag}
                    onChange={(e) => setNewTag(e.target.value)}
                    placeholder="Add tag..."
                    onKeyPress={(e) => e.key === 'Enter' && (e.preventDefault(), handleAddTag())}
                  />
                  <button
                    type="button"
                    onClick={handleAddTag}
                    disabled={!newTag.trim() || formData.tags.includes(newTag.trim())}
                  >
                    <i className="fas fa-plus"></i>
                  </button>
                </div>
                
                <div className="suggested-tags">
                  <span className="suggestions-label">Suggestions:</span>
                  {suggestedTags[formData.category as keyof typeof suggestedTags]?.map((tag) => (
                    <button
                      key={tag}
                      type="button"
                      className={`suggested-tag ${formData.tags.includes(tag) ? 'added' : ''}`}
                      onClick={() => handleSuggestedTagClick(tag)}
                      disabled={formData.tags.includes(tag)}
                    >
                      {tag}
                    </button>
                  ))}
                </div>
              </div>
            </div>
          </div>

          {/* Notes */}
          <div className="form-section">
            <h3>Notes</h3>
            
            <div className="form-group">
              <label htmlFor="example-notes">Additional Notes</label>
              <textarea
                id="example-notes"
                value={formData.notes}
                onChange={(e) => handleInputChange('notes', e.target.value)}
                placeholder="Add any additional notes about this style example..."
                rows={3}
              />
            </div>
          </div>

          {/* AI Analysis */}
          <div className="form-section">
            <h3>AI Analysis</h3>
            
            <div className="ai-analysis-section">
              <p>Let AI analyze this text to automatically detect style characteristics and suggest tags.</p>
              
              <button
                type="button"
                className="ai-analyze-btn"
                onClick={handleAIAnalysis}
                disabled={!formData.content.trim() || isAnalyzing}
              >
                {isAnalyzing ? (
                  <>
                    <i className="fas fa-spinner fa-spin"></i>
                    Analyzing...
                  </>
                ) : (
                  <>
                    <i className="fas fa-brain"></i>
                    Analyze Style
                  </>
                )}
              </button>
              
              <div className="analysis-features">
                <div className="feature">
                  <i className="fas fa-search"></i>
                  <span>Detect tone and complexity</span>
                </div>
                <div className="feature">
                  <i className="fas fa-tags"></i>
                  <span>Suggest relevant tags</span>
                </div>
                <div className="feature">
                  <i className="fas fa-chart-line"></i>
                  <span>Analyze writing complexity</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="modal-actions">
          <div className="form-validation">
            {!isFormValid && (
              <span className="validation-message">
                <i className="fas fa-exclamation-triangle"></i>
                Name and content are required
              </span>
            )}
          </div>
          
          <div className="action-buttons">
            <button
              className="save-btn"
              onClick={handleSave}
              disabled={!isFormValid}
            >
              <i className="fas fa-save"></i>
              {example ? 'Update Example' : 'Save Example'}
            </button>
            
            <button className="cancel-btn" onClick={handleClose}>
              <i className="fas fa-times"></i>
              Cancel
            </button>
          </div>
        </div>

        {/* Unsaved Changes Warning */}
        {showUnsavedWarning && (
          <div className="unsaved-warning-overlay">
            <div className="unsaved-warning-dialog">
              <h3>Unsaved Changes</h3>
              <p>You have unsaved changes. Are you sure you want to close without saving?</p>
              <div className="warning-actions">
                <button className="discard-btn" onClick={handleForceClose}>
                  <i className="fas fa-trash"></i>
                  Discard Changes
                </button>
                <button className="keep-editing-btn" onClick={() => setShowUnsavedWarning(false)}>
                  <i className="fas fa-edit"></i>
                  Keep Editing
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default StyleExampleModal;