import React, { useState, useEffect, useCallback, useMemo } from 'react';
import { useStoryBible } from '../../hooks/useStoryBible';
import { useErrorHandler } from '../../../../hooks/useErrorHandler';
// Using standard HTML elements instead of custom UI components
import { Loader2, Lightbulb, Save, Edit, X, Trash2, Plus } from 'lucide-react';
import { toast } from 'react-hot-toast';
import './BraindumpEditor.css';

// Debounce utility function
const debounce = <T extends (...args: any[]) => any>(
  func: T,
  wait: number
): ((...args: Parameters<T>) => void) => {
  let timeout: NodeJS.Timeout;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
};

interface BraindumpEditorProps {
  projectId: string;
  content?: string;
  synopsis?: string;
  genre?: string;
  style?: string;
  styleExamples?: string;
  povMode?: string;
  globalPov?: string;
  globalTense?: string;
  globalCharacterPovIds?: string;
}

const BraindumpEditor: React.FC<BraindumpEditorProps> = ({
  projectId,
  content = '',
  synopsis = '',
  genre = '',
  style = '',
  styleExamples = '',
  povMode = '',
  globalPov = '',
  globalTense = '',
  globalCharacterPovIds = ''
}) => {
  const { generateSynopsis: generateSynopsisAction, createOrUpdateStoryBible } = useStoryBible();
  const { handleError, handleWarning, handleInfo } = useErrorHandler();
  
  const [isEditing, setIsEditing] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);
  const [lastSaved, setLastSaved] = useState<Date | null>(null);
  const [isGeneratingSynopsis, setIsGeneratingSynopsis] = useState(false);
  const [isBrainstorming, setIsBrainstorming] = useState(false);
  const [brainstormIdeas, setBrainstormIdeas] = useState<string[]>([]);
  const [brainstormPrompt, setBrainstormPrompt] = useState('');
  const [selectedCategory, setSelectedCategory] = useState<'characters' | 'plot_points' | 'settings' | 'conflicts' | 'themes'>('characters');
  const [keepersList, setKeepersList] = useState<string[]>([]);
  const [showBrainstormResults, setShowBrainstormResults] = useState(false);
  
  // Form data
  const [formData, setFormData] = useState({
    braindump: content,
    synopsis: synopsis,
    genre: genre,
    style: style,
    style_examples: styleExamples,
    pov_mode: povMode,
    global_pov: globalPov,
    global_tense: globalTense,
    global_character_pov_ids: globalCharacterPovIds
  });
  
  // POV Mode options
  const povModeOptions = [
    { value: '', label: 'Select POV Mode' },
    { value: 'first_person', label: 'First Person' },
    { value: 'second_person', label: 'Second Person' },
    { value: 'third_person_limited', label: 'Third Person Limited' },
    { value: 'third_person_omniscient', label: 'Third Person Omniscient' },
    { value: 'multiple_pov', label: 'Multiple POV' }
  ];
  
  // Tense options
  const tenseOptions = [
    { value: '', label: 'Select Tense' },
    { value: 'past', label: 'Past Tense' },
    { value: 'present', label: 'Present Tense' },
    { value: 'future', label: 'Future Tense' },
    { value: 'mixed', label: 'Mixed Tenses' }
  ];
  
  // Genre suggestions
  const genreSuggestions = [
    'Fantasy', 'Science Fiction', 'Mystery', 'Romance', 'Thriller',
    'Horror', 'Historical Fiction', 'Contemporary Fiction', 'Young Adult',
    'Literary Fiction', 'Adventure', 'Crime', 'Dystopian', 'Urban Fantasy'
  ];
  
  // Brainstorm category options
  const brainstormCategories = [
    { value: 'characters', label: 'Characters' },
    { value: 'plot_points', label: 'Plot Points' },
    { value: 'settings', label: 'Settings & Worldbuilding' },
    { value: 'conflicts', label: 'Conflicts & Tension' },
    { value: 'themes', label: 'Themes & Messages' }
  ];
  
  // Watch for changes
  useEffect(() => {
    const hasChangesValue = 
      formData.braindump !== content ||
      formData.synopsis !== synopsis ||
      formData.genre !== genre ||
      formData.style !== style ||
      formData.style_examples !== styleExamples ||
      formData.pov_mode !== povMode ||
      formData.global_pov !== globalPov ||
      formData.global_tense !== globalTense ||
      formData.global_character_pov_ids !== globalCharacterPovIds;
    
    setHasChanges(hasChangesValue);
  }, [formData, content, synopsis, genre, style, styleExamples, povMode, globalPov, globalTense, globalCharacterPovIds]);
  
  // Event handlers
  const handleInputChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };
  
  const startEditing = () => {
    setIsEditing(true);
  };
  
  const cancelEditing = () => {
    setFormData({
      braindump: content,
      synopsis: synopsis,
      genre: genre,
      style: style,
      style_examples: styleExamples,
      pov_mode: povMode,
      global_pov: globalPov,
      global_tense: globalTense,
      global_character_pov_ids: globalCharacterPovIds
    });
    setIsEditing(false);
    setHasChanges(false);
  };
  
  // Enhanced save functionality with error handling
  const saveContent = useCallback(async (dataToSave = formData, showSuccessMessage = true) => {
    if (!dataToSave.braindump.trim() && !dataToSave.synopsis.trim()) {
      handleWarning('Cannot save empty content');
      return false;
    }

    setIsSaving(true);
    try {
      await createOrUpdateStoryBible({
        project_id: projectId,
        braindump: dataToSave.braindump,
        synopsis: dataToSave.synopsis,
        genre: dataToSave.genre,
        style: dataToSave.style,
        style_examples: dataToSave.style_examples,
        pov_mode: dataToSave.pov_mode,
        global_pov: dataToSave.global_pov,
        global_tense: dataToSave.global_tense,
        global_character_pov_ids: dataToSave.global_character_pov_ids
      });
      
      setLastSaved(new Date());
      setHasChanges(false);
      
      if (showSuccessMessage) {
        handleInfo('Story bible saved successfully');
      }
      
      return true;
    } catch (error) {
      handleError(error, { 
        action: 'save_story_bible',
        projectId,
        contentLength: dataToSave.braindump.length,
        retryAction: () => saveContent(dataToSave, showSuccessMessage)
      });
      return false;
    } finally {
      setIsSaving(false);
    }
  }, [projectId, formData, createOrUpdateStoryBible, handleError, handleWarning, handleInfo]);

  // Manual save handler
  const saveChanges = useCallback(async () => {
    if (!hasChanges) return;
    
    const success = await saveContent(formData, true);
    if (success) {
      setIsEditing(false);
    }
  }, [hasChanges, formData, saveContent]);

  // Debounced auto-save
  const debouncedAutoSave = useMemo(
    () => debounce((data: typeof formData) => {
      if (hasChanges && isEditing) {
        saveContent(data, false); // Auto-save without success message
      }
    }, 3000),
    [saveContent, hasChanges, isEditing]
  );

  // Trigger auto-save when form data changes
  useEffect(() => {
    if (hasChanges && isEditing) {
      debouncedAutoSave(formData);
    }
  }, [formData, hasChanges, isEditing, debouncedAutoSave]);
  
  const generateSynopsis = async () => {
    if (!formData.braindump.trim()) return;
    
    setIsGeneratingSynopsis(true);
    try {
      const result = await generateSynopsisAction({
        project_id: projectId,
        braindump: formData.braindump,
        genre: formData.genre,
        style: formData.style,
        creativity: 0.7
      });
      
      if (result && result.generated_content) {
        setFormData(prev => ({ ...prev, synopsis: result.generated_content }));
      }
    } catch (error) {
      console.error('Failed to generate synopsis:', error);
    } finally {
      setIsGeneratingSynopsis(false);
    }
  };
  
  const startBrainstorming = async () => {
    if (!brainstormPrompt.trim()) return;
    
    setIsBrainstorming(true);
    try {
      // Mock brainstorming for now - replace with actual AI call
      const mockIdeas = [
        `A mysterious ${selectedCategory === 'characters' ? 'character' : 'element'} related to: ${brainstormPrompt}`,
        `An unexpected twist involving: ${brainstormPrompt}`,
        `A compelling backstory for: ${brainstormPrompt}`
      ];
      
      setBrainstormIdeas(mockIdeas);
      setShowBrainstormResults(true);
    } catch (error) {
      console.error('Failed to brainstorm:', error);
    } finally {
      setIsBrainstorming(false);
    }
  };
  
  const addToKeepers = (idea: string) => {
    if (!keepersList.includes(idea)) {
      setKeepersList(prev => [...prev, idea]);
    }
  };
  
  const removeFromKeepers = (idea: string) => {
    setKeepersList(prev => prev.filter(keeper => keeper !== idea));
  };
  
  const clearBrainstormResults = () => {
    setBrainstormIdeas([]);
    setShowBrainstormResults(false);
    setBrainstormPrompt('');
  };
  
  return (
    <div className="braindump-editor">
      {/* Header */}
      <div className="editor-header">
        <div className="header-content">
          <h2>Story Bible</h2>
          <p className="subtitle">Capture your story's essence, characters, and world</p>
          {/* Save Status Indicator */}
          <div className="save-status">
            {isSaving && (
              <span className="saving-indicator">
                <Loader2 className="w-3 h-3 animate-spin mr-1" />
                Auto-saving...
              </span>
            )}
            {lastSaved && !isSaving && (
              <span className="saved-indicator">
                Last saved: {lastSaved.toLocaleTimeString()}
              </span>
            )}
          </div>
        </div>
        <div className="header-actions">
          {isEditing ? (
            <div className="edit-actions">
              <button
                type="button"
                className="btn btn-outline"
                onClick={cancelEditing}
                disabled={isSaving}
              >
                <X className="w-4 h-4 mr-2" />
                Cancel
              </button>
              <button
                type="button"
                className="btn btn-primary"
                onClick={saveChanges}
                disabled={!hasChanges || isSaving}
              >
                {isSaving ? (
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                ) : (
                  <Save className="w-4 h-4 mr-2" />
                )}
                {isSaving ? 'Saving...' : 'Save Changes'}
              </button>
            </div>
          ) : (
            <button type="button" className="btn btn-primary" onClick={startEditing}>
              <Edit className="w-4 h-4 mr-2" />
              Edit
            </button>
          )}
        </div>
      </div>
      
      {/* Content */}
      <div className="editor-content">
        {/* Metadata Grid */}
        <div className="metadata-grid">
          {/* Genre */}
          <div className="field-group">
            <label htmlFor="genre">Genre</label>
            {isEditing ? (
              <input
                type="text"
                id="genre"
                className="form-input"
                value={formData.genre}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) => handleInputChange('genre', e.target.value)}
                placeholder="e.g., Fantasy, Science Fiction, Mystery"
                list="genre-suggestions"
              />
            ) : (
              <div className="field-content">
                {genre || <span className="field-value">No genre specified</span>}
              </div>
            )}
            <datalist id="genre-suggestions">
              {genreSuggestions.map(suggestion => (
                <option key={suggestion} value={suggestion} />
              ))}
            </datalist>
          </div>
          
          {/* POV Mode */}
          <div className="field-group">
            <label htmlFor="pov-mode">Point of View</label>
            {isEditing ? (
              <select
                id="pov-mode"
                className="form-select"
                value={formData.pov_mode}
                onChange={(e) => handleInputChange('pov_mode', e.target.value)}
              >
                {povModeOptions.map(option => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}              </select>
            ) : (
              <div className="field-content">
                {povModeOptions.find(opt => opt.value === povMode)?.label || 
                 <span className="field-value">No POV mode selected</span>}
              </div>
            )}
          </div>
          
          {/* Global Tense */}
          <div className="field-group">
            <label htmlFor="global-tense">Narrative Tense</label>
            {isEditing ? (
              <select
                id="global-tense"
                className="form-select"
                value={formData.global_tense}
                onChange={(e) => handleInputChange('global_tense', e.target.value)}
              >
                {tenseOptions.map(option => (
                  <option key={option.value} value={option.value}>
                    {option.label}
                  </option>
                ))}              </select>
            ) : (
              <div className="field-content">
                {tenseOptions.find(opt => opt.value === globalTense)?.label || 
                 <span className="field-value">No tense selected</span>}
              </div>
            )}
          </div>
        </div>
        
        {/* Synopsis */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title flex items-center justify-between">
              Synopsis
              {isEditing && (
                <button
                  type="button"
                  className="btn btn-outline btn-sm"
                  onClick={generateSynopsis}
                  disabled={!formData.braindump.trim() || isGeneratingSynopsis}
                >
                  {isGeneratingSynopsis ? (
                    <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                  ) : (
                    <Lightbulb className="w-4 h-4 mr-2" />
                  )}
                  {isGeneratingSynopsis ? 'Generating...' : 'AI Generate'}
                </button>
              )}
            </h3>
          </div>
          <div className="card-content">
            {isEditing ? (
              <textarea
                className="form-textarea synopsis-textarea"
                value={formData.synopsis}
                onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => handleInputChange('synopsis', e.target.value)}
                placeholder="Write a brief synopsis of your story..."
                rows={4}
              />
            ) : (
              <div className="synopsis-content">
                {synopsis ? (
                  <p>{synopsis}</p>
                ) : (
                  <p className="field-value">No synopsis written yet. Click Edit to add one.</p>
                )}
              </div>
            )}
          </div>
        </div>
        
        {/* Writing Style */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Writing Style & Voice</h3>
          </div>
          <div className="card-content">
            <div className="style-content">
              {/* Style Description */}
              <div className="field-group">
                <label htmlFor="style">Style Description</label>
                {isEditing ? (
                  <textarea
                    id="style"
                    className="form-textarea"
                    value={formData.style}
                    onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => handleInputChange('style', e.target.value)}
                    placeholder="Describe your writing style, tone, and voice..."
                    rows={3}
                  />
                ) : (
                  <div className="field-content">
                    {style ? (
                      <p>{style}</p>
                    ) : (
                      <p className="field-value">No style description yet.</p>
                    )}
                  </div>
                )}
              </div>
              
              {/* Style Examples */}
              <div className="field-group">
                <label htmlFor="style-examples">Style Examples</label>
                {isEditing ? (
                  <textarea
                    id="style-examples"
                    className="form-textarea"
                    value={formData.style_examples}
                    onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => handleInputChange('style_examples', e.target.value)}
                    placeholder="Paste example sentences or paragraphs that capture your desired style..."
                    rows={4}
                  />
                ) : (
                  <div className="field-content">
                    {styleExamples ? (
                      <pre className="style-examples">{styleExamples}</pre>
                    ) : (
                      <p className="field-value">No style examples yet.</p>
                    )}
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
        
        {/* AI Brainstorming */}
        {isEditing && (
          <div className="card">
            <div className="card-header">
              <h3 className="card-title">AI Brainstorming</h3>
            </div>
            <div className="card-content">
              <div className="brainstorm-content">
                <div className="brainstorm-controls">
                  <div className="brainstorm-input-row">
                    <div className="category-select">
                      <label htmlFor="brainstorm-category">Category:</label>
                      <select
                        id="brainstorm-category"
                        className="form-select"
                        value={selectedCategory}
                        onChange={(e: any) => setSelectedCategory(e.target.value)}
                      >
                        {brainstormCategories.map(category => (
                          <option key={category.value} value={category.value}>
                            {category.label}
                          </option>
                        ))}
                      </select>
                    </div>
                    
                    <div className="prompt-input">
                      <input
                        type="text"
                        className="form-input brainstorm-prompt"
                        value={brainstormPrompt}
                        onChange={(e: React.ChangeEvent<HTMLInputElement>) => setBrainstormPrompt(e.target.value)}
                        placeholder="What would you like to brainstorm? (e.g., 'mysterious characters for a fantasy tavern')"
                      />
                    </div>
                    
                    <button
                      type="button"
                      className="btn btn-primary"
                      onClick={startBrainstorming}
                      disabled={!brainstormPrompt.trim() || isBrainstorming}
                    >
                      {isBrainstorming ? (
                        <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      ) : (
                        <Lightbulb className="w-4 h-4 mr-2" />
                      )}
                      {isBrainstorming ? 'Brainstorming...' : 'Brainstorm'}
                    </button>
                  </div>
                </div>
                
                {/* Brainstorm Results */}
                {showBrainstormResults && (
                  <div className="brainstorm-results">
                    <div className="results-header">
                      <h4>Brainstorm Results</h4>
                      <div className="results-actions">
                        <button
                          type="button"
                          className="btn btn-outline btn-sm"
                          onClick={clearBrainstormResults}
                        >
                          <X className="w-4 h-4" />
                        </button>
                      </div>
                    </div>
                    
                    <div className="ideas-list">
                      {brainstormIdeas.map((idea, index) => (
                        <div key={index} className="idea-item">
                          <div className="idea-content">
                            <p className="idea-text">{idea}</p>
                          </div>
                          <div className="idea-actions">
                            <button
                              type="button"
                              className="btn btn-outline btn-sm"
                              onClick={() => addToKeepers(idea)}
                              disabled={keepersList.includes(idea)}
                            >
                              <Plus className="w-4 h-4" />
                            </button>
                          </div>
                        </div>
                      ))}
                    </div>
                    
                    {/* Keepers List */}
                    {keepersList.length > 0 && (
                      <div className="keepers-list">
                        <h5>Keepers</h5>
                        <div className="keepers-items">
                          {keepersList.map((keeper, index) => (
                            <div key={index} className="keeper-item">
                              <span className="keeper-text">{keeper}</span>
                              <button
                                type="button"
                                className="btn btn-outline btn-sm"
                                onClick={() => removeFromKeepers(keeper)}
                              >
                                <Trash2 className="w-4 h-4" />
                              </button>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
        
        {/* Main Braindump */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Creative Braindump</h3>
          </div>
          <div className="card-content">
            {isEditing ? (
              <textarea
                className="form-textarea braindump-textarea"
                value={formData.braindump}
                onChange={(e: React.ChangeEvent<HTMLTextAreaElement>) => handleInputChange('braindump', e.target.value)}
                placeholder="Let your creativity flow! Jot down ideas, plot points, character thoughts, world-building details, or anything else related to your story..."
                rows={12}
              />
            ) : (
              <div className="braindump-content">
                {content ? (
                  <pre className="braindump-text">{content}</pre>
                ) : (
                  <div className="empty-state">
                    <span className="empty-icon">💭</span>
                    <h3>Start Your Creative Journey</h3>
                    <p>This is your creative space. Use it to brainstorm ideas, capture inspiration, and develop your story's foundation.</p>
                    <button type="button" className="btn btn-primary" onClick={startEditing}>
                      Start Writing
                    </button>
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default BraindumpEditor;