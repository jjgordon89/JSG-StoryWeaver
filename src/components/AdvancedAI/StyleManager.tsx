import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import type { StyleExample } from '../../types/advancedAI';
import StyleExampleModal from './StyleExampleModal';

const StyleManager: React.FC = () => {
  const {
    styleExamples,
    loadStyleExamples,
    addStyleExample,
    updateStyleExample,
    deleteStyleExample,
    analyzeStyleExample,
    generateFromStyle
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

  useEffect(() => {
    loadStyleExamples();
  }, [loadStyleExamples]);

  const filteredAndSortedExamples = React.useMemo(() => {
    let filtered = styleExamples.filter(example => {
      const matchesSearch = example.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           example.content.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           example.tags.some(tag => tag.toLowerCase().includes(searchTerm.toLowerCase()));
      const matchesCategory = selectedCategory === 'all' || example.category === selectedCategory;
      const matchesTone = selectedTone === 'all' || example.characteristics.tone === selectedTone;
      
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
          aValue = a.category;
          bValue = b.category;
          break;
        case 'tone':
          aValue = a.characteristics.tone;
          bValue = b.characteristics.tone;
          break;
        case 'complexity':
          aValue = a.characteristics.complexity;
          bValue = b.characteristics.complexity;
          break;
        case 'createdAt':
        case 'updatedAt':
          aValue = new Date(a[sortBy]);
          bValue = new Date(b[sortBy]);
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
        await updateStyleExample(editingExample.id, example);
      } else {
        await addStyleExample(example as Omit<StyleExample, 'id' | 'createdAt' | 'updatedAt'>);
      }
      setShowModal(false);
      setEditingExample(null);
    } catch (error) {
      console.error('Error saving style example:', error);
    }
  };

  const handleDeleteExample = async (id: string) => {
    try {
      await deleteStyleExample(id);
      setSelectedExamples(prev => prev.filter(selectedId => selectedId !== id));
    } catch (error) {
      console.error('Error deleting style example:', error);
    }
  };

  const handleBulkDelete = async () => {
    try {
      await Promise.all(selectedExamples.map(id => deleteStyleExample(id)));
      setSelectedExamples([]);
      setShowDeleteConfirm(false);
    } catch (error) {
      console.error('Error deleting style examples:', error);
    }
  };

  const handleAnalyzeExample = async (example: StyleExample) => {
    setIsAnalyzing(example.id);
    try {
      await analyzeStyleExample(example.id);
    } catch (error) {
      console.error('Error analyzing style example:', error);
    } finally {
      setIsAnalyzing(null);
    }
  };

  const handleGenerateFromStyle = async () => {
    if (!selectedStyleForGenerate || !generatePrompt.trim()) return;
    
    try {
      await generateFromStyle(selectedStyleForGenerate.id, generatePrompt);
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

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString();
  };

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
                  <span className="category-badge">{example.category}</span>
                  <span 
                    className="complexity-indicator"
                    style={{ backgroundColor: getComplexityColor(example.characteristics.complexity) }}
                  >
                    {example.characteristics.complexity}
                  </span>
                  <span className="tone-badge">{example.characteristics.tone}</span>
                </div>

                <div className="example-content">
                  <p>{example.content.substring(0, 200)}{example.content.length > 200 ? '...' : ''}</p>
                </div>

                <div className="example-characteristics">
                  <div className="characteristic">
                    <span className="label">Pacing:</span>
                    <span className="value">{example.characteristics.pacing}</span>
                  </div>
                  <div className="characteristic">
                    <span className="label">Perspective:</span>
                    <span className="value">{example.characteristics.perspective}</span>
                  </div>
                </div>

                {example.tags.length > 0 && (
                  <div className="example-tags">
                    {example.tags.slice(0, 3).map((tag, index) => (
                      <span key={index} className="tag">{tag}</span>
                    ))}
                    {example.tags.length > 3 && (
                      <span className="tag-more">+{example.tags.length - 3} more</span>
                    )}
                  </div>
                )}

                <div className="example-footer">
                  <span className="date-info">
                    Created: {formatDate(example.createdAt)}
                  </span>
                  {example.aiAnalysis && (
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
                  <div className="char-item">
                    <span className="label">Tone:</span>
                    <span className="value">{selectedStyleForGenerate.characteristics.tone}</span>
                  </div>
                  <div className="char-item">
                    <span className="label">Complexity:</span>
                    <span className="value">{selectedStyleForGenerate.characteristics.complexity}</span>
                  </div>
                  <div className="char-item">
                    <span className="label">Pacing:</span>
                    <span className="value">{selectedStyleForGenerate.characteristics.pacing}</span>
                  </div>
                  <div className="char-item">
                    <span className="label">Perspective:</span>
                    <span className="value">{selectedStyleForGenerate.characteristics.perspective}</span>
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