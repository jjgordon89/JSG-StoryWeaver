import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { TextArea } from '../../../../components/ui/TextArea';
import { Input } from '../../../../components/ui/input';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { CreateStoryBibleRequest, GenerateSynopsisRequest } from '../../../../types/storyBible';
import './BraindumpEditor.css';

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
  const { generateSynopsis: generateSynopsisAction } = useStoryBible();
  
  const [isEditing, setIsEditing] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);
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
  
  const startEditing = () => {
    setIsEditing(true);
  };
  
  const cancelEditing = () => {
    // Reset form data
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
  
  const saveChanges = async () => {
    if (!hasChanges) return;
    
    setIsSaving(true);
    
    try {
      const request: CreateStoryBibleRequest = {
        project_id: projectId,
        ...formData
      };
      
      // TODO: Implement save functionality
      console.log('Saving story bible:', request);
      
      setIsEditing(false);
      setHasChanges(false);
    } catch (error) {
      console.error('Failed to save story bible:', error);
    } finally {
      setIsSaving(false);
    }
  };
  
  const handleGenreSelect = (selectedGenre: string) => {
    setFormData(prev => ({ ...prev, genre: selectedGenre }));
  };
  
  const generateSynopsis = async () => {
    if (!projectId || !formData.braindump.trim()) return;
    
    setIsGeneratingSynopsis(true);
    
    try {
      const request: GenerateSynopsisRequest = {
        project_id: projectId,
        braindump: formData.braindump,
        genre: formData.genre,
        style: formData.style
      };
      
      const response = await generateSynopsisAction(request);
      
      if (response && response.generated_content) {
        setFormData(prev => ({ ...prev, synopsis: response.generated_content }));
      }
    } catch (error) {
      console.error('Failed to generate synopsis:', error);
    } finally {
      setIsGeneratingSynopsis(false);
    }
  };
  
  const generateBrainstormIdeas = async () => {
    if (!brainstormPrompt.trim()) return;
    
    setIsBrainstorming(true);
    
    try {
      // TODO: Implement AI brainstorming
      const mockIdeas = [
        'A mysterious character with a hidden past',
        'An unexpected plot twist that changes everything',
        'A unique setting that becomes a character itself',
        'A conflict that tests the protagonist\'s values',
        'A theme that resonates throughout the story'
      ];
      
      setBrainstormIdeas(mockIdeas);
      setShowBrainstormResults(true);
    } catch (error) {
      console.error('Failed to generate brainstorm ideas:', error);
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
  
  const addKeepersToContent = () => {
    if (keepersList.length === 0) return;
    
    const keepersText = '\n\n--- Brainstorm Ideas ---\n' + keepersList.map(idea => `• ${idea}`).join('\n') + '\n';
    setFormData(prev => ({ ...prev, braindump: prev.braindump + keepersText }));
    
    // Clear keepers and hide results
    setKeepersList([]);
    setShowBrainstormResults(false);
    setBrainstormIdeas([]);
    setBrainstormPrompt('');
  };
  
  const clearBrainstormResults = () => {
    setBrainstormIdeas([]);
    setKeepersList([]);
    setShowBrainstormResults(false);
    setBrainstormPrompt('');
  };
  
  const updateFormData = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };
  
  return (
    <div className="braindump-editor">
      {/* Header */}
      <div className="editor-header">
        <div className="header-content">
          <h2>Story Bible & Braindump</h2>
          <p className="subtitle">
            Capture your story's core elements, world-building notes, and creative brainstorming.
          </p>
        </div>
        
        <div className="header-actions">
          {!isEditing ? (
            <Button variant="primary" onClick={startEditing}>
              <span className="icon">✏️</span>
              Edit
            </Button>
          ) : (
            <div className="edit-actions">
              <Button 
                variant="secondary" 
                onClick={cancelEditing}
                disabled={isSaving}
              >
                Cancel
              </Button>
              <Button 
                variant="primary" 
                onClick={saveChanges}
                disabled={!hasChanges || isSaving}
              >
                {isSaving ? 'Saving...' : 'Save Changes'}
              </Button>
            </div>
          )}
        </div>
      </div>
      
      {/* Content */}
      <div className="editor-content">
        {/* Story Metadata */}
        <Card className="metadata-card">
          <CardHeader>
            <CardTitle>Story Metadata</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="metadata-grid">
              {/* Genre */}
              <div className="field-group">
                <label htmlFor="genre">Genre</label>
                {isEditing ? (
                  <div className="genre-input-container">
                    <Input
                      id="genre"
                      value={formData.genre}
                      onChange={(e) => updateFormData('genre', e.target.value)}
                      placeholder="Enter genre..."
                      list="genre-suggestions"
                    />
                    <datalist id="genre-suggestions">
                      {genreSuggestions.map((suggestion) => (
                        <option key={suggestion} value={suggestion}></option>
                      ))}
                    </datalist>
                  </div>
                ) : (
                  <p className="field-value">{genre || 'Not specified'}</p>
                )}
              </div>
              
              {/* POV Mode */}
              <div className="field-group">
                <label htmlFor="pov-mode">Point of View</label>
                {isEditing ? (
                  <Select value={formData.pov_mode} onValueChange={(value) => updateFormData('pov_mode', value)}>
                    <SelectTrigger>
                      <SelectValue placeholder="Select POV Mode" />
                    </SelectTrigger>
                    <SelectContent>
                      {povModeOptions.map((option) => (
                        <SelectItem key={option.value} value={option.value}>
                          {option.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                ) : (
                  <p className="field-value">
                    {povModeOptions.find(opt => opt.value === povMode)?.label || 'Not specified'}
                  </p>
                )}
              </div>
              
              {/* Global Tense */}
              <div className="field-group">
                <label htmlFor="global-tense">Narrative Tense</label>
                {isEditing ? (
                  <Select value={formData.global_tense} onValueChange={(value) => updateFormData('global_tense', value)}>
                    <SelectTrigger>
                      <SelectValue placeholder="Select Tense" />
                    </SelectTrigger>
                    <SelectContent>
                      {tenseOptions.map((option) => (
                        <SelectItem key={option.value} value={option.value}>
                          {option.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                ) : (
                  <p className="field-value">
                    {tenseOptions.find(opt => opt.value === globalTense)?.label || 'Not specified'}
                  </p>
                )}
              </div>
              
              {/* Global POV Character */}
              <div className="field-group">
                <label htmlFor="global-pov">Primary POV Character</label>
                {isEditing ? (
                  <Input
                    id="global-pov"
                    value={formData.global_pov}
                    onChange={(e) => updateFormData('global_pov', e.target.value)}
                    placeholder="Main character name..."
                  />
                ) : (
                  <p className="field-value">{globalPov || 'Not specified'}</p>
                )}
              </div>
            </div>
          </CardContent>
        </Card>
        
        {/* Synopsis */}
        <Card className="synopsis-card">
          <CardHeader className="card-header-with-actions">
            <CardTitle>Synopsis</CardTitle>
            {isEditing && (
              <Button
                variant="secondary"
                size="sm"
                onClick={generateSynopsis}
                disabled={isGeneratingSynopsis || !formData.braindump.trim()}
                className="ai-generate-btn"
              >
                <span className="icon">{isGeneratingSynopsis ? '⏳' : '✨'}</span>
                {isGeneratingSynopsis ? 'Generating...' : 'Generate with AI'}
              </Button>
            )}
          </CardHeader>
          <CardContent>
            {isEditing ? (
              <TextArea
                value={formData.synopsis}
                onChange={(value) => updateFormData('synopsis', value)}
                placeholder="Write a brief synopsis of your story..."
                rows={4}
                className="synopsis-textarea"
              />
            ) : (
              <div className="synopsis-content">
                {synopsis ? (
                  <p>{synopsis}</p>
                ) : (
                  <p className="empty-state">No synopsis written yet. Click Edit to add one.</p>
                )}
              </div>
            )}
          </CardContent>
        </Card>
        
        {/* Writing Style */}
        <Card className="style-card">
          <CardHeader>
            <CardTitle>Writing Style & Voice</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="style-content">
              {/* Style Description */}
              <div className="field-group">
                <label htmlFor="style">Style Description</label>
                {isEditing ? (
                  <TextArea
                    id="style"
                    value={formData.style}
                    onChange={(value) => updateFormData('style', value)}
                    placeholder="Describe your writing style, tone, and voice..."
                    rows={3}
                  />
                ) : (
                  <div className="field-content">
                    {style ? (
                      <p>{style}</p>
                    ) : (
                      <p className="empty-state">No style description yet.</p>
                    )}
                  </div>
                )}
              </div>
              
              {/* Style Examples */}
              <div className="field-group">
                <label htmlFor="style-examples">Style Examples</label>
                {isEditing ? (
                  <TextArea
                    id="style-examples"
                    value={formData.style_examples}
                    onChange={(value) => updateFormData('style_examples', value)}
                    placeholder="Paste example sentences or paragraphs that capture your desired style..."
                    rows={4}
                  />
                ) : (
                  <div className="field-content">
                    {styleExamples ? (
                      <pre className="style-examples">{styleExamples}</pre>
                    ) : (
                      <p className="empty-state">No style examples yet.</p>
                    )}
                  </div>
                )}
              </div>
            </div>
          </CardContent>
        </Card>
        
        {/* AI Brainstorming */}
        {isEditing && (
          <Card className="brainstorm-card">
            <CardHeader>
              <CardTitle>AI Brainstorming</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="brainstorm-content">
                <div className="brainstorm-controls">
                  <div className="brainstorm-input-row">
                    <div className="category-select">
                      <label htmlFor="brainstorm-category">Category:</label>
                      <Select value={selectedCategory} onValueChange={(value: any) => setSelectedCategory(value)}>
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          {brainstormCategories.map((category) => (
                            <SelectItem key={category.value} value={category.value}>
                              {category.label}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>
                    </div>
                    
                    <div className="prompt-input">
                      <Input
                        value={brainstormPrompt}
                        onChange={(e) => setBrainstormPrompt(e.target.value)}
                        placeholder="What would you like to brainstorm? (e.g., 'mysterious characters for a fantasy tavern')"
                        className="brainstorm-prompt"
                      />
                    </div>
                    
                    <Button
                      variant="primary"
                      onClick={generateBrainstormIdeas}
                      disabled={isBrainstorming || !brainstormPrompt.trim()}
                    >
                      <span className="icon">{isBrainstorming ? '⏳' : '💡'}</span>
                      {isBrainstorming ? 'Generating...' : 'Generate Ideas'}
                    </Button>
                  </div>
                </div>
                
                {showBrainstormResults && (
                  <div className="brainstorm-results">
                    <div className="results-header">
                      <h4>Generated Ideas</h4>
                      <div className="results-actions">
                        {keepersList.length > 0 && (
                          <Button
                            variant="primary"
                            size="sm"
                            onClick={addKeepersToContent}
                          >
                            <span className="icon">📝</span>
                            Add {keepersList.length} to Braindump
                          </Button>
                        )}
                        <Button
                          variant="secondary"
                          size="sm"
                          onClick={clearBrainstormResults}
                        >
                          Clear
                        </Button>
                      </div>
                    </div>
                    
                    <div className="ideas-list">
                      {brainstormIdeas.map((idea, index) => (
                        <div key={index} className="idea-item">
                          <div className="idea-content">
                            <span className="idea-text">{idea}</span>
                          </div>
                          <div className="idea-actions">
                            {keepersList.includes(idea) ? (
                              <Button
                                variant="secondary"
                                size="sm"
                                onClick={() => removeFromKeepers(idea)}
                                title="Remove from keepers"
                              >
                                <span className="icon">👍</span>
                              </Button>
                            ) : (
                              <Button
                                variant="secondary"
                                size="sm"
                                onClick={() => addToKeepers(idea)}
                                title="Add to keepers"
                              >
                                <span className="icon">👍</span>
                              </Button>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                    
                    {keepersList.length > 0 && (
                      <div className="keepers-list">
                        <h5>Keepers List ({keepersList.length})</h5>
                        <div className="keepers-items">
                          {keepersList.map((keeper, index) => (
                            <div key={index} className="keeper-item">
                              <span className="keeper-text">{keeper}</span>
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={() => removeFromKeepers(keeper)}
                                title="Remove from keepers"
                              >
                                <span className="icon">✕</span>
                              </Button>
                            </div>
                          ))}
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        )}
        
        {/* Braindump */}
        <Card className="braindump-card">
          <CardHeader>
            <CardTitle>Creative Braindump</CardTitle>
          </CardHeader>
          <CardContent>
            {isEditing ? (
              <TextArea
                value={formData.braindump}
                onChange={(value) => updateFormData('braindump', value)}
                placeholder="Let your creativity flow! Jot down ideas, plot points, character thoughts, world-building details, or anything else related to your story..."
                rows={12}
                className="braindump-textarea"
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
                    <Button variant="primary" onClick={startEditing}>
                      Start Writing
                    </Button>
                  </div>
                )}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default BraindumpEditor;