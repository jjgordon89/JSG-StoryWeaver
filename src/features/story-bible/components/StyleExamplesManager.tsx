import React, { useState, useEffect, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Button } from '../../../ui/components/common';
import { Input } from '../../../ui/components/common';
import { Card } from '../../../ui/components/common';
import { Modal } from '../../../ui/components/common';
import LoadingSpinner from '../../../components/ui/LoadingSpinner';
import { showToast } from '../../../utils/toast';

interface StyleExample {
  id: string;
  project_id: string;
  user_id: string;
  example_text: string;
  analysis_result?: string;
  generated_style_prompt?: string;
  word_count: number;
  created_at: string;
  updated_at: string;
}

interface AnalysisResult {
  tone: string;
  style_elements: string[];
  sentence_structure: string;
  vocabulary_level: string;
  pacing: string;
  voice: string;
  key_phrases: string[];
}

interface StyleExamplesManagerProps {
  projectId: string;
  userId?: string;
}

const StyleExamplesManager: React.FC<StyleExamplesManagerProps> = ({ 
  projectId, 
  userId = 'default-user' 
}) => {
  // State
  const [styleExamples, setStyleExamples] = useState<StyleExample[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isAnalyzing, setIsAnalyzing] = useState(false);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showViewModal, setShowViewModal] = useState(false);
  const [selectedExample, setSelectedExample] = useState<StyleExample | null>(null);
  const [searchQuery, setSearchQuery] = useState('');
  
  // Form state
  const [newExampleText, setNewExampleText] = useState('');
  const [editingExample, setEditingExample] = useState<StyleExample | null>(null);
  const [isEditing, setIsEditing] = useState(false);

  // Computed values
  const filteredExamples = useMemo(() => {
    return styleExamples.filter(example => 
      example.example_text.toLowerCase().includes(searchQuery.toLowerCase()) ||
      (example.analysis_result && example.analysis_result.toLowerCase().includes(searchQuery.toLowerCase()))
    );
  }, [styleExamples, searchQuery]);

  const analyzedExamples = useMemo(() => 
    styleExamples.filter(example => example.analysis_result), [styleExamples]
  );
  
  const unanalyzedExamples = useMemo(() => 
    styleExamples.filter(example => !example.analysis_result), [styleExamples]
  );

  useEffect(() => {
    loadStyleExamples();
  }, [projectId]);

  const loadStyleExamples = async () => {
    setIsLoading(true);
    try {
      const examples = await invoke('get_style_examples_by_project', { projectId }) as StyleExample[];
      setStyleExamples(examples);
    } catch (error) {
      console.error('Failed to load style examples:', error);
      showToast.error('Failed to load style examples');
    } finally {
      setIsLoading(false);
    }
  };

  const createStyleExample = async () => {
    if (!newExampleText.trim()) {
      showToast.error('Please enter some example text');
      return;
    }

    if (getWordCount(newExampleText) > 1000) {
      showToast.error('Example text must be 1000 words or less');
      return;
    }

    try {
      const request = {
        project_id: projectId,
        user_id: userId,
        example_text: newExampleText.trim()
      };

      const newExample = await invoke('create_style_example', request) as StyleExample;
      setStyleExamples(prev => [newExample, ...prev]);
      setNewExampleText('');
      setShowCreateModal(false);
      showToast.success('Style example created successfully');
    } catch (error) {
      console.error('Failed to create style example:', error);
      showToast.error('Failed to create style example');
    }
  };

  const analyzeStyleExample = async (example: StyleExample) => {
    setIsAnalyzing(true);
    try {
      const result = await invoke('analyze_style_example', {
        styleExampleId: example.id,
        exampleText: example.example_text
      }) as { analysis_result: string; generated_style_prompt: string };

      // Update the example in our local state
      setStyleExamples(prev => prev.map(ex => 
        ex.id === example.id 
          ? { ...ex, analysis_result: result.analysis_result, generated_style_prompt: result.generated_style_prompt }
          : ex
      ));

      showToast.success('Style analysis completed');
    } catch (error) {
      console.error('Failed to analyze style example:', error);
      showToast.error('Failed to analyze style example');
    } finally {
      setIsAnalyzing(false);
    }
  };

  const updateStyleExample = async () => {
    if (!editingExample) return;

    try {
      const request = {
        id: editingExample.id,
        example_text: editingExample.example_text,
        analysis_result: editingExample.analysis_result,
        generated_style_prompt: editingExample.generated_style_prompt
      };

      const updated = await invoke('update_style_example', request) as StyleExample;
      setStyleExamples(prev => prev.map(ex => 
        ex.id === editingExample.id ? updated : ex
      ));
      
      setEditingExample(null);
      setIsEditing(false);
      showToast.success('Style example updated successfully');
    } catch (error) {
      console.error('Failed to update style example:', error);
      showToast.error('Failed to update style example');
    }
  };

  const deleteStyleExample = async (example: StyleExample) => {
    if (!confirm('Are you sure you want to delete this style example?')) return;

    try {
      await invoke('delete_style_example', { id: example.id });
      setStyleExamples(prev => prev.filter(ex => ex.id !== example.id));
      showToast.success('Style example deleted successfully');
    } catch (error) {
      console.error('Failed to delete style example:', error);
      showToast.error('Failed to delete style example');
    }
  };

  const openCreateModal = () => {
    setNewExampleText('');
    setShowCreateModal(true);
  };

  const openViewModal = (example: StyleExample) => {
    setSelectedExample(example);
    setShowViewModal(true);
  };

  const openEditModal = (example: StyleExample) => {
    setEditingExample({ ...example });
    setIsEditing(true);
    setShowViewModal(false);
  };

  const closeModals = () => {
    setShowCreateModal(false);
    setShowViewModal(false);
    setSelectedExample(null);
    setEditingExample(null);
    setIsEditing(false);
  };

  const getWordCount = (text: string): number => {
    return text.trim().split(/\s+/).length;
  };

  const parseAnalysisResult = (analysisJson: string): AnalysisResult | null => {
    try {
      return JSON.parse(analysisJson);
    } catch {
      return null;
    }
  };

  const formatDate = (dateString: string): string => {
    return new Date(dateString).toLocaleDateString();
  };

  return (
    <div className="style-examples-manager">
      <div className="manager-header">
        <div className="header-content">
          <h2>📝 Style Examples</h2>
          <p className="subtitle">Manage writing style examples and AI analysis (up to 1,000 words each)</p>
        </div>
        
        <div className="header-actions">
          <Button variant="primary" onClick={openCreateModal}>
            ➕ Add Example
          </Button>
        </div>
      </div>

      <div className="content-area">
        {/* Search and Filters */}
        <div className="search-section">
          <div className="search-bar">
            <Input
              value={searchQuery}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setSearchQuery(e.target.value)}
              placeholder="Search style examples..."
              className="search-input"
            />
          </div>
          
          <div className="stats">
            <span className="stat-item">Total: {styleExamples.length}</span>
            <span className="stat-item">Analyzed: {analyzedExamples.length}</span>
            <span className="stat-item">Pending: {unanalyzedExamples.length}</span>
          </div>
        </div>

        {/* Examples List */}
        {isLoading ? (
          <div className="loading-container">
            <LoadingSpinner size="large" />
            <p>Loading style examples...</p>
          </div>
        ) : filteredExamples.length === 0 ? (
          <div className="empty-state">
            <div className="empty-icon">📝</div>
            <h3>No Style Examples</h3>
            <p>Create your first style example to help AI understand your writing style.</p>
            <Button variant="primary" onClick={openCreateModal}>
              Add Your First Example
            </Button>
          </div>
        ) : (
          <div className="examples-grid">
            {filteredExamples.map((example) => (
              <Card key={example.id} className="example-card">
                <div className="example-header">
                  <div className="example-meta">
                    <span className="word-count">{example.word_count} words</span>
                    <span className="date">{formatDate(example.created_at)}</span>
                  </div>
                  
                  <div className="example-actions">
                    {!example.analysis_result && (
                    <Button 
                      variant="secondary" 
                      size="sm" 
                      onClick={() => analyzeStyleExample(example)}
                      disabled={isAnalyzing}
                    >
                        {isAnalyzing ? '🔄' : '🔍'} Analyze
                      </Button>
                    )}
                    
                    <Button 
                      variant="ghost" 
                      size="sm" 
                      onClick={() => openViewModal(example)}
                    >
                      👁️ View
                    </Button>
                    
                    <Button 
                      variant="ghost" 
                      size="sm" 
                      onClick={() => deleteStyleExample(example)}
                      className="delete-btn"
                    >
                      🗑️
                    </Button>
                  </div>
                </div>
                
                <div className="example-preview">
                  <p className="example-text">
                    {example.example_text.length > 200 
                      ? example.example_text.substring(0, 200) + '...' 
                      : example.example_text}
                  </p>
                </div>
                
                {example.analysis_result ? (
                  <div className="analysis-preview">
                    <div className="analysis-badge">✅ Analyzed</div>
                    {example.generated_style_prompt && (
                      <p className="style-prompt-preview">
                        <strong>Style Prompt:</strong> 
                        {example.generated_style_prompt.length > 100 
                          ? example.generated_style_prompt.substring(0, 100) + '...' 
                          : example.generated_style_prompt}
                      </p>
                    )}
                  </div>
                ) : (
                  <div className="analysis-preview pending">
                    <div className="analysis-badge pending">⏳ Pending Analysis</div>
                  </div>
                )}
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Create Example Modal */}
      <Modal 
        isOpen={showCreateModal} 
        onClose={closeModals} 
        title="Add Style Example" 
        size="large"
      >
        <div className="create-form">
          <div className="form-field">
            <label htmlFor="example-text">Example Text (up to 1,000 words)</label>
            <textarea
              id="example-text"
              value={newExampleText}
              onChange={(e) => setNewExampleText(e.target.value)}
              placeholder="Paste a sample of your writing that represents your desired style..."
              rows={12}
              className="example-textarea"
            />
            <div className="word-count-info">
              <span className="word-count">{getWordCount(newExampleText)} / 1,000 words</span>
              {getWordCount(newExampleText) > 1000 && (
                <span className="error">⚠️ Exceeds word limit</span>
              )}
            </div>
          </div>
        </div>
        
        <div className="modal-actions">
          <Button variant="secondary" onClick={closeModals}>
            Cancel
          </Button>
          <Button 
            variant="primary" 
            onClick={createStyleExample}
            disabled={!newExampleText.trim() || getWordCount(newExampleText) > 1000}
          >
            Create Example
          </Button>
        </div>
      </Modal>

      {/* View/Edit Example Modal */}
      <Modal 
        isOpen={showViewModal} 
        onClose={closeModals} 
        title="Style Example" 
        size="large"
      >
        {selectedExample && (
          <div className="view-content">
            <div className="example-details">
              <div className="detail-header">
                <div className="detail-meta">
                  <span className="word-count">{selectedExample.word_count} words</span>
                  <span className="date">Created: {formatDate(selectedExample.created_at)}</span>
                  {selectedExample.updated_at !== selectedExample.created_at && (
                    <span className="date">Updated: {formatDate(selectedExample.updated_at)}</span>
                  )}
                </div>
                
                <div className="detail-actions">
                  {!selectedExample.analysis_result && (
                    <Button 
                      variant="secondary" 
                      onClick={() => analyzeStyleExample(selectedExample)}
                      disabled={isAnalyzing}
                    >
                      {isAnalyzing ? '🔄 Analyzing...' : '🔍 Analyze Style'}
                    </Button>
                  )}
                  
                  <Button variant="secondary" onClick={() => openEditModal(selectedExample)}>
                    ✏️ Edit
                  </Button>
                </div>
              </div>
              
              <div className="example-text-full">
                <h4>Example Text</h4>
                <div className="text-content">
                  {selectedExample.example_text}
                </div>
              </div>
              
              {selectedExample.analysis_result && (() => {
                const analysis = parseAnalysisResult(selectedExample.analysis_result);
                return (
                  <div className="analysis-results">
                    <h4>AI Analysis</h4>
                    {analysis ? (
                      <div className="analysis-grid">
                        <div className="analysis-item">
                          <strong>Tone:</strong> {analysis.tone}
                        </div>
                        <div className="analysis-item">
                          <strong>Voice:</strong> {analysis.voice}
                        </div>
                        <div className="analysis-item">
                          <strong>Sentence Structure:</strong> {analysis.sentence_structure}
                        </div>
                        <div className="analysis-item">
                          <strong>Vocabulary Level:</strong> {analysis.vocabulary_level}
                        </div>
                        <div className="analysis-item">
                          <strong>Pacing:</strong> {analysis.pacing}
                        </div>
                        {analysis.style_elements && analysis.style_elements.length > 0 && (
                          <div className="analysis-item full-width">
                            <strong>Style Elements:</strong>
                            <div className="style-tags">
                              {analysis.style_elements.map((element, index) => (
                                <span key={index} className="style-tag">{element}</span>
                              ))}
                            </div>
                          </div>
                        )}
                        {analysis.key_phrases && analysis.key_phrases.length > 0 && (
                          <div className="analysis-item full-width">
                            <strong>Key Phrases:</strong>
                            <div className="key-phrases">
                              {analysis.key_phrases.map((phrase, index) => (
                                <span key={index} className="key-phrase">"{phrase}"</span>
                              ))}
                            </div>
                          </div>
                        )}
                      </div>
                    ) : (
                      <div className="raw-analysis">
                        <pre>{selectedExample.analysis_result}</pre>
                      </div>
                    )}
                  </div>
                );
              })()}
              
              {selectedExample.generated_style_prompt && (
                <div className="style-prompt">
                  <h4>Generated Style Prompt</h4>
                  <div className="prompt-content">
                    {selectedExample.generated_style_prompt}
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
        
        <div className="modal-actions">
          <Button variant="secondary" onClick={closeModals}>
            Close
          </Button>
          {selectedExample && !selectedExample.analysis_result && (
            <Button 
              variant="primary" 
              onClick={() => analyzeStyleExample(selectedExample)}
              disabled={isAnalyzing}
            >
              {isAnalyzing ? '🔄 Analyzing...' : '🔍 Analyze Style'}
            </Button>
          )}
        </div>
      </Modal>

      {/* Edit Example Modal */}
      <Modal 
        isOpen={isEditing} 
        onClose={closeModals} 
        title="Edit Style Example" 
        size="large"
      >
        {editingExample && (
          <div className="edit-form">
            <div className="form-field">
              <label htmlFor="edit-example-text">Example Text (up to 1,000 words)</label>
              <textarea
                id="edit-example-text"
                value={editingExample.example_text}
                onChange={(e) => setEditingExample({ ...editingExample, example_text: e.target.value })}
                rows={12}
                className="example-textarea"
              />
              <div className="word-count-info">
                <span className="word-count">{getWordCount(editingExample.example_text)} / 1,000 words</span>
                {getWordCount(editingExample.example_text) > 1000 && (
                  <span className="error">⚠️ Exceeds word limit</span>
                )}
              </div>
            </div>
          </div>
        )}
        
        <div className="modal-actions">
          <Button variant="secondary" onClick={closeModals}>
            Cancel
          </Button>
          <Button 
            variant="primary" 
            onClick={updateStyleExample}
            disabled={!editingExample?.example_text.trim() || getWordCount(editingExample?.example_text || '') > 1000}
          >
            Update Example
          </Button>
        </div>
      </Modal>

      <style>{`
        .style-examples-manager {
          display: flex;
          flex-direction: column;
          height: 100%;
          background: var(--bg-primary);
        }

        .manager-header {
          display: flex;
          justify-content: space-between;
          align-items: flex-start;
          padding: 2rem;
          border-bottom: 1px solid var(--border-color);
          background: var(--bg-secondary);
        }

        .header-content h2 {
          margin: 0 0 0.5rem 0;
          font-size: 1.5rem;
          font-weight: 600;
          color: var(--text-primary);
        }

        .subtitle {
          margin: 0;
          color: var(--text-secondary);
          font-size: 0.9rem;
        }

        .content-area {
          flex: 1;
          padding: 2rem;
          overflow-y: auto;
        }

        .search-section {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 2rem;
          gap: 1rem;
        }

        .search-bar {
          flex: 1;
          max-width: 400px;
        }

        .stats {
          display: flex;
          gap: 1rem;
        }

        .stat-item {
          padding: 0.5rem 1rem;
          background: var(--bg-secondary);
          border-radius: 0.5rem;
          font-size: 0.9rem;
          color: var(--text-secondary);
        }

        .loading-container {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          padding: 4rem;
          gap: 1rem;
          color: var(--text-secondary);
        }

        .empty-state {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          padding: 4rem;
          text-align: center;
          color: var(--text-secondary);
        }

        .empty-icon {
          font-size: 4rem;
          margin-bottom: 1rem;
        }

        .empty-state h3 {
          margin: 0 0 1rem 0;
          color: var(--text-primary);
        }

        .empty-state p {
          margin: 0 0 2rem 0;
          max-width: 400px;
        }

        .examples-grid {
          display: grid;
          grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
          gap: 1.5rem;
        }

        .example-card {
          border: 1px solid var(--border-color);
          transition: all 0.2s ease;
        }

        .example-card:hover {
          border-color: var(--accent-primary);
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
        }

        .example-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 1rem;
        }

        .example-meta {
          display: flex;
          gap: 1rem;
          font-size: 0.85rem;
          color: var(--text-secondary);
        }

        .example-actions {
          display: flex;
          gap: 0.5rem;
        }

        .example-preview {
          margin-bottom: 1rem;
        }

        .example-text {
          margin: 0;
          line-height: 1.5;
          color: var(--text-primary);
          font-size: 0.9rem;
        }

        .analysis-preview {
          padding-top: 1rem;
          border-top: 1px solid var(--border-color);
        }

        .analysis-badge {
          display: inline-block;
          padding: 0.25rem 0.75rem;
          border-radius: 1rem;
          font-size: 0.8rem;
          font-weight: 500;
          background: var(--success-bg);
          color: var(--success-text);
        }

        .analysis-badge.pending {
          background: var(--warning-bg);
          color: var(--warning-text);
        }

        .style-prompt-preview {
          margin: 0.5rem 0 0 0;
          font-size: 0.85rem;
          color: var(--text-secondary);
        }

        .create-form,
        .edit-form {
          padding: 1rem 0;
        }

        .form-field {
          margin-bottom: 1.5rem;
        }

        .form-field label {
          display: block;
          margin-bottom: 0.5rem;
          font-weight: 500;
          color: var(--text-primary);
        }

        .example-textarea {
          width: 100%;
          min-height: 300px;
          padding: 1rem;
          border: 1px solid var(--border-color);
          border-radius: 0.5rem;
          background: var(--bg-primary);
          color: var(--text-primary);
          font-family: inherit;
          font-size: 0.9rem;
          line-height: 1.5;
          resize: vertical;
        }

        .example-textarea:focus {
          outline: none;
          border-color: var(--accent-primary);
        }

        .word-count-info {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-top: 0.5rem;
          font-size: 0.85rem;
        }

        .word-count {
          color: var(--text-secondary);
        }

        .error {
          color: var(--error-text);
          font-weight: 500;
        }

        .view-content {
          max-height: 70vh;
          overflow-y: auto;
        }

        .detail-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 2rem;
          padding-bottom: 1rem;
          border-bottom: 1px solid var(--border-color);
        }

        .detail-meta {
          display: flex;
          gap: 1rem;
          font-size: 0.9rem;
          color: var(--text-secondary);
        }

        .detail-actions {
          display: flex;
          gap: 0.5rem;
        }

        .example-text-full {
          margin-bottom: 2rem;
        }

        .example-text-full h4 {
          margin: 0 0 1rem 0;
          color: var(--text-primary);
        }

        .text-content {
          padding: 1.5rem;
          background: var(--bg-secondary);
          border-radius: 0.5rem;
          border: 1px solid var(--border-color);
          line-height: 1.6;
          white-space: pre-wrap;
          color: var(--text-primary);
        }

        .analysis-results {
          margin-bottom: 2rem;
        }

        .analysis-results h4 {
          margin: 0 0 1rem 0;
          color: var(--text-primary);
        }

        .analysis-grid {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
          gap: 1rem;
        }

        .analysis-item {
          padding: 1rem;
          background: var(--bg-secondary);
          border-radius: 0.5rem;
          border: 1px solid var(--border-color);
        }

        .analysis-item.full-width {
          grid-column: 1 / -1;
        }

        .analysis-item strong {
          display: block;
          margin-bottom: 0.5rem;
          color: var(--text-primary);
        }

        .style-tags,
        .key-phrases {
          display: flex;
          flex-wrap: wrap;
          gap: 0.5rem;
          margin-top: 0.5rem;
        }

        .style-tag,
        .key-phrase {
          padding: 0.25rem 0.75rem;
          background: var(--accent-bg);
          color: var(--accent-text);
          border-radius: 1rem;
          font-size: 0.8rem;
        }

        .key-phrase {
          background: var(--info-bg);
          color: var(--info-text);
          font-style: italic;
        }

        .raw-analysis {
          padding: 1rem;
          background: var(--bg-secondary);
          border-radius: 0.5rem;
          border: 1px solid var(--border-color);
        }

        .raw-analysis pre {
          margin: 0;
          white-space: pre-wrap;
          font-size: 0.85rem;
          color: var(--text-secondary);
        }

        .style-prompt {
          margin-bottom: 2rem;
        }

        .style-prompt h4 {
          margin: 0 0 1rem 0;
          color: var(--text-primary);
        }

        .prompt-content {
          padding: 1.5rem;
          background: var(--success-bg);
          border-radius: 0.5rem;
          border: 1px solid var(--success-border);
          line-height: 1.6;
          color: var(--success-text);
          font-weight: 500;
        }

        .modal-actions {
          display: flex;
          justify-content: flex-end;
          gap: 1rem;
          padding: 1rem 0 0 0;
          border-top: 1px solid var(--border-color);
        }

        .delete-btn {
          color: var(--error-text) !important;
        }

        .delete-btn:hover {
          background: var(--error-bg) !important;
        }

        /* Responsive Design */
        @media (max-width: 768px) {
          .manager-header {
            flex-direction: column;
            gap: 1rem;
            padding: 1.5rem;
          }

          .search-section {
            flex-direction: column;
            align-items: stretch;
          }

          .stats {
            justify-content: center;
          }

          .examples-grid {
            grid-template-columns: 1fr;
          }

          .example-header {
            flex-direction: column;
            gap: 1rem;
            align-items: stretch;
          }

          .example-actions {
            justify-content: center;
          }

          .detail-header {
            flex-direction: column;
            gap: 1rem;
            align-items: stretch;
          }

          .detail-actions {
            justify-content: center;
          }

          .analysis-grid {
            grid-template-columns: 1fr;
          }

          .modal-actions {
            flex-direction: column;
          }
        }
      `}</style>
    </div>
  );
};

export default StyleExamplesManager;
