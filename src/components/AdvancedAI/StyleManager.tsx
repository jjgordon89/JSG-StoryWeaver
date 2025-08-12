import React, { useState } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import type { StyleExample } from '../../types/advancedAI';
import StyleExampleModal from './StyleExampleModal';

const StyleManager: React.FC = () => {
  const {
    styleExamples,
    addStyleExample,
    analyzeTextStyle
  } = useAdvancedAIStore();

  const [searchTerm, setSearchTerm] = useState('');
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [selectedTone, setSelectedTone] = useState('all');
  const [sortBy, setSortBy] = useState('name');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [showModal, setShowModal] = useState(false);
  const [editingExample, setEditingExample] = useState<StyleExample | null>(null);
  const [selectedExamples, setSelectedExamples] = useState<string[]>([]);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [isAnalyzing, setIsAnalyzing] = useState<string | null>(null);
  const [showGenerateModal, setShowGenerateModal] = useState(false);
  const [generatePrompt, setGeneratePrompt] = useState('');
  const [selectedStyleForGenerate, setSelectedStyleForGenerate] = useState<StyleExample | null>(null);

  const categories = ['all', 'narrative', 'dialogue', 'description', 'action', 'introspection', 'other'];
  const tones = ['all', 'formal', 'casual', 'dramatic', 'humorous', 'dark', 'light', 'mysterious', 'romantic'];
  const sortOptions = [
    { value: 'name', label: 'Name' },
    { value: 'category', label: 'Category' },
    { value: 'tone', label: 'Tone' },
    { value: 'complexity', label: 'Complexity' },
    { value: 'createdAt', label: 'Date Created' },
    { value: 'updatedAt', label: 'Last Modified' }
  ];

  // Note: Style examples are loaded through other means

  const filteredAndSortedExamples = React.useMemo(() => {
    let filtered = styleExamples.filter(example => {
      const matchesSearch = example.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                            example.content.toLowerCase().includes(searchTerm.toLowerCase());
      const matchesCategory = selectedCategory === 'all' || example.name === selectedCategory; // Use name as category
      const matchesTone = selectedTone === 'all' || (example.analysis_result?.tone_indicators?.includes(selectedTone) ?? false);
      
      return matchesSearch && matchesCategory && matchesTone;
    });

    filtered.sort((a, b) => {
      let aValue: any, bValue: any;
      
      switch (sortBy) {
        case 'name':
          aValue = a.name.toLowerCase();
          bValue = b.name.toLowerCase();
          break;
        case 'category':
          aValue = a.name; // Use name as category since category doesn't exist
          bValue = b.name;
          break;
        case 'tone':
          aValue = a.analysis_result?.tone_indicators?.[0] || '';
          bValue = b.analysis_result?.tone_indicators?.[0] || '';
          break;
        case 'complexity':
          aValue = a.analysis_result?.vocabulary_complexity || 0;
          bValue = b.analysis_result?.vocabulary_complexity || 0;
          break;
        case 'createdAt':
        case 'updatedAt':
          aValue = a.id; // Use id for sorting since created_at doesn't exist
          bValue = b.id;
          break;
        default:
          return 0;
      }
      
      if (aValue < bValue) return sortOrder === 'asc' ? -1 : 1;
      if (aValue > bValue) return sortOrder === 'asc' ? 1 : -1;
      return 0;
    });

    return filtered;
  }, [styleExamples, searchTerm, selectedCategory, selectedTone, sortBy, sortOrder]);

  const handleAddExample = () => {
    setEditingExample(null);
    setShowModal(true);
  };

  const handleEditExample = (example: StyleExample) => {
    setEditingExample(example);
    setShowModal(true);
  };

  const handleSaveExample = async (example: Partial<StyleExample>) => {
    try {
      if (editingExample) {
        // TODO: Implement update functionality
        console.log('Update style example:', editingExample.id, example);
      } else {
          await addStyleExample({
            project_id: example.project_id || '',
            name: example.name || 'Untitled',
            content: example.content || '',
            word_count: example.word_count || 0
          });
        }
      setShowModal(false);
      setEditingExample(null);
    } catch (error) {
      console.error('Error saving style example:', error);
    }
  };

  const handleDeleteExample = async (id: string) => {
    try {
      // TODO: Implement delete functionality
      console.log('Delete style example:', id);
      setSelectedExamples(prev => prev.filter(selectedId => selectedId !== id));
    } catch (error) {
      console.error('Error deleting style example:', error);
    }
  };

  const handleBulkDelete = async () => {
    try {
      // TODO: Implement bulk delete functionality
      console.log('Bulk delete style examples:', selectedExamples);
      setSelectedExamples([]);
      setShowDeleteConfirm(false);
    } catch (error) {
      console.error('Error deleting style examples:', error);
    }
  };

  const handleAnalyzeExample = async (example: StyleExample) => {
    setIsAnalyzing(example.id);
    try {
      await analyzeTextStyle(example.content);
    } catch (error) {
      console.error('Error analyzing style example:', error);
    } finally {
      setIsAnalyzing(null);
    }
  };

  const handleGenerateFromStyle = async () => {
    if (!selectedStyleForGenerate || !generatePrompt.trim()) return;
    
    try {
      // TODO: Implement generation from style
      console.log('Generate from style:', selectedStyleForGenerate.id, generatePrompt);
      setShowGenerateModal(false);
      setGeneratePrompt('');
      setSelectedStyleForGenerate(null);
    } catch (error) {
      console.error('Error generating from style:', error);
    }
  };

  const handleSelectExample = (id: string) => {
    setSelectedExamples(prev => 
      prev.includes(id) 
        ? prev.filter(selectedId => selectedId !== id)
        : [...prev, id]
    );
  };

  const handleSelectAll = () => {
    if (selectedExamples.length === filteredAndSortedExamples.length) {
      setSelectedExamples([]);
    } else {
      setSelectedExamples(filteredAndSortedExamples.map(example => example.id));
    }
  };

  const getComplexityColor = (complexity: string) => {
    switch (complexity) {
      case 'simple': return '#4CAF50';
      case 'moderate': return '#FF9800';
      case 'complex': return '#F44336';
      default: return '#9E9E9E';
    }
  };

  // Removed unused formatDate function

  return (
    <div className="style-manager">
      <div className="style-manager-header">
        <div className="header-title">
          <h2>Style Manager</h2>
          <p>Manage and analyze writing style examples for consistent prose generation</p>
        </div>
        
        <div className="header-actions">
          <button className="add-btn" onClick={handleAddExample}>
            <i className="fas fa-plus"></i>
            Add Style Example
          </button>
        </div>
      </div>

      {/* Filters and Controls */}
      <div className="style-controls">
        <div className="search-section">
          <div className="search-input">
            <i className="fas fa-search"></i>
            <input
              type="text"
              placeholder="Search style examples..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
        </div>

        <div className="filter-section">
          <div className="filter-group">
            <label htmlFor="category-filter">Category:</label>
            <select
              id="category-filter"
              value={selectedCategory}
              onChange={(e) => setSelectedCategory(e.target.value)}
            >
              {categories.map(category => (
                <option key={category} value={category}>
                  {category.charAt(0).toUpperCase() + category.slice(1)}
                </option>
              ))}
            </select>
          </div>

          <div className="filter-group">
            <label htmlFor="tone-filter">Tone:</label>
            <select
              id="tone-filter"
              value={selectedTone}
              onChange={(e) => setSelectedTone(e.target.value)}
            >
              {tones.map(tone => (
                <option key={tone} value={tone}>
                  {tone.charAt(0).toUpperCase() + tone.slice(1)}
                </option>
              ))}
            </select>
          </div>

          <div className="filter-group">
            <label htmlFor="sort-filter">Sort by:</label>
            <select
              id="sort-filter"
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value)}
            >
              {sortOptions.map(option => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
            <button
              className={`sort-order-btn ${sortOrder}`}
              onClick={() => setSortOrder(prev => prev === 'asc' ? 'desc' : 'asc')}
              title={`Sort ${sortOrder === 'asc' ? 'ascending' : 'descending'}`}
            >
              <i className={`fas fa-sort-${sortOrder === 'asc' ? 'up' : 'down'}`}></i>
            </button>
          </div>
        </div>

        <div className="view-controls">
          <div className="view-mode-toggle">
            <button
              className={`view-btn ${viewMode === 'grid' ? 'active' : ''}`}
              onClick={() => setViewMode('grid')}
              title="Grid view"
            >
              <i className="fas fa-th"></i>
            </button>
            <button
              className={`view-btn ${viewMode === 'list' ? 'active' : ''}`}
              onClick={() => setViewMode('list')}
              title="List view"
            >
              <i className="fas fa-list"></i>
            </button>
          </div>

          {selectedExamples.length > 0 && (
            <div className="bulk-actions">
              <span className="selection-count">
                {selectedExamples.length} selected
              </span>
              <button
                className="bulk-delete-btn"
                onClick={() => setShowDeleteConfirm(true)}
              >
                <i className="fas fa-trash"></i>
                Delete Selected
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Results Summary */}
      <div className="results-summary">
        <div className="summary-stats">
          <span className="total-count">
            {filteredAndSortedExamples.length} of {styleExamples.length} examples
          </span>
          {filteredAndSortedExamples.length > 0 && (
            <button
              className="select-all-btn"
              onClick={handleSelectAll}
            >
              {selectedExamples.length === filteredAndSortedExamples.length ? 'Deselect All' : 'Select All'}
            </button>
          )}
        </div>
      </div>

      {/* Style Examples */}
      <div className={`style-examples ${viewMode}`}>
        {filteredAndSortedExamples.length === 0 ? (
          <div className="empty-state">
            <i className="fas fa-palette"></i>
            <h3>No Style Examples Found</h3>
            <p>
              {styleExamples.length === 0
                ? 'Start building your style library by adding your first example.'
                : 'Try adjusting your search or filter criteria.'}
            </p>
            {styleExamples.length === 0 && (
              <button className="add-first-btn" onClick={handleAddExample}>
                <i className="fas fa-plus"></i>
                Add Your First Style Example
              </button>
            )}
          </div>
        ) : (
          filteredAndSortedExamples.map((example) => (
            <div
              key={example.id}
              className={`style-example-card ${selectedExamples.includes(example.id) ? 'selected' : ''}`}
            >
              <div className="card-header">
                <div className="card-title">
                  <input
                    type="checkbox"
                    checked={selectedExamples.includes(example.id)}
                    onChange={() => handleSelectExample(example.id)}
                    className="example-checkbox"
                  />
                  <h3>{example.name}</h3>
                </div>
                
                <div className="card-actions">
                  <button
                    className="action-btn analyze"
                    onClick={() => handleAnalyzeExample(example)}
                    disabled={isAnalyzing === example.id}
                    title="Analyze style characteristics"
                  >
                    {isAnalyzing === example.id ? (
                      <i className="fas fa-spinner fa-spin"></i>
                    ) : (
                      <i className="fas fa-brain"></i>
                    )}
                  </button>
                  
                  <button
                    className="action-btn generate"
                    onClick={() => {
                      setSelectedStyleForGenerate(example);
                      setShowGenerateModal(true);
                    }}
                    title="Generate text in this style"
                  >
                    <i className="fas fa-magic"></i>
                  </button>
                  
                  <button
                    className="action-btn edit"
                    onClick={() => handleEditExample(example)}
                    title="Edit example"
                  >
                    <i className="fas fa-edit"></i>
                  </button>
                  
                  <button
                    className="action-btn delete"
                    onClick={() => handleDeleteExample(example.id)}
                    title="Delete example"
                  >
                    <i className="fas fa-trash"></i>
                  </button>
                </div>
              </div>

              <div className="card-content">
                <div className="example-meta">
                  <span className="category-badge">{example.name}</span>
                  <span
                    className="complexity-badge"
                    style={{ backgroundColor: getComplexityColor(String(example.analysis_result?.vocabulary_complexity || 'medium')) }}
                  >
                    {example.analysis_result?.vocabulary_complexity || 'N/A'}
                  </span>
                  <span className="tone-badge">{example.analysis_result?.tone_indicators?.[0] || 'N/A'}</span>
                </div>

                <div className="example-content">
                  <p>{example.content.substring(0, 200)}{example.content.length > 200 ? '...' : ''}</p>
                </div>

                <div className="example-characteristics">
                  <div className="characteristic">
                    <span className="label">Sentence Length:</span>
                    <span className="value">{example.analysis_result?.sentence_length_avg || 'N/A'}</span>
                  </div>
                  <div className="characteristic">
                    <span className="label">Dialogue Ratio:</span>
                    <span className="value">{example.analysis_result?.dialogue_ratio || 'N/A'}</span>
                  </div>
                </div>

                {/* Tags functionality not available in current StyleExample interface */}

                <div className="example-footer">
                  <span className="date-info">
                    Created: N/A
                  </span>
                  {example.analysis_result && (
                    <span className="ai-analyzed">
                      <i className="fas fa-brain"></i>
                      AI Analyzed
                    </span>
                  )}
                </div>
              </div>
            </div>
          ))
        )}
      </div>

      {/* Style Example Modal */}
      {showModal && (
        <StyleExampleModal
          example={editingExample}
          onSave={handleSaveExample}
          onClose={() => {
            setShowModal(false);
            setEditingExample(null);
          }}
        />
      )}

      {/* Generate Modal */}
      {showGenerateModal && selectedStyleForGenerate && (
        <div className="modal-overlay" onClick={() => setShowGenerateModal(false)}>
          <div className="generate-modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h3>Generate in Style: {selectedStyleForGenerate.name}</h3>
              <button className="close-btn" onClick={() => setShowGenerateModal(false)}>
                <i className="fas fa-times"></i>
              </button>
            </div>
            
            <div className="modal-content">
              <div className="style-preview">
                <h4>Style Characteristics:</h4>
                <div className="characteristics-grid">
                  {selectedStyleForGenerate.analysis_result && (
                    <>
                      <div className="char-item">
                        <span className="label">Tone Indicators:</span>
                        <span className="value">{selectedStyleForGenerate.analysis_result.tone_indicators?.join(', ') || 'N/A'}</span>
                      </div>
                      <div className="char-item">
                        <span className="label">Avg Sentence Length:</span>
                        <span className="value">{selectedStyleForGenerate.analysis_result.sentence_length_avg || 'N/A'}</span>
                      </div>
                      <div className="char-item">
                        <span className="label">Vocabulary Complexity:</span>
                        <span className="value">{selectedStyleForGenerate.analysis_result.vocabulary_complexity || 'N/A'}</span>
                      </div>
                      <div className="char-item">
                        <span className="label">Dialogue Ratio:</span>
                        <span className="value">{selectedStyleForGenerate.analysis_result.dialogue_ratio || 'N/A'}</span>
                      </div>
                    </>
                  )}
                  <div className="char-item">
                    <span className="label">Word Count:</span>
                    <span className="value">{selectedStyleForGenerate.word_count}</span>
                  </div>
                </div>
              </div>
              
              <div className="prompt-section">
                <label htmlFor="generate-prompt">What would you like to generate?</label>
                <textarea
                  id="generate-prompt"
                  value={generatePrompt}
                  onChange={(e) => setGeneratePrompt(e.target.value)}
                  placeholder="Describe what you want to write in this style..."
                  rows={4}
                />
              </div>
            </div>
            
            <div className="modal-actions">
              <button
                className="generate-btn"
                onClick={handleGenerateFromStyle}
                disabled={!generatePrompt.trim()}
              >
                <i className="fas fa-magic"></i>
                Generate
              </button>
              <button className="cancel-btn" onClick={() => setShowGenerateModal(false)}>
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Delete Confirmation */}
      {showDeleteConfirm && (
        <div className="modal-overlay" onClick={() => setShowDeleteConfirm(false)}>
          <div className="delete-confirm-modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h3>Confirm Deletion</h3>
            </div>
            
            <div className="modal-content">
              <p>
                Are you sure you want to delete {selectedExamples.length} style example{selectedExamples.length > 1 ? 's' : ''}?
                This action cannot be undone.
              </p>
            </div>
            
            <div className="modal-actions">
              <button className="delete-btn" onClick={handleBulkDelete}>
                <i className="fas fa-trash"></i>
                Delete
              </button>
              <button className="cancel-btn" onClick={() => setShowDeleteConfirm(false)}>
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default StyleManager;