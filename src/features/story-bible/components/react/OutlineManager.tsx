import React, { useState, useEffect, useMemo } from 'react';
import { Button } from '../../../../ui/components/common';
import { Card } from '../../../../ui/components/common';
import { Input } from '../../../../ui/components/common';
import { Textarea } from '../../../../ui/components/common';
import { Select } from '../../../../ui/components/common';
import { Sparkles, Loader2, Download } from 'lucide-react';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { Outline, CreateOutlineRequest, UpdateOutlineRequest } from '../../../../types/storyBible';

interface OutlineManagerProps {
  projectId: string;
  seriesId?: string;
}

interface OutlineFormData {
  id?: string;
  chapter_title: string;
  summary: string;
  chapter_number: number | null;
  character_pov: string;
  linked_document_id?: string;
}

const OutlineManager: React.FC<OutlineManagerProps> = ({ projectId, seriesId }) => {
  const {
    outlines,
    isLoadingOutlines,
    outlinesError,
    createOutline,
    updateOutline,
    deleteOutline,
    loadOutlines,
    generateOutline,
    clearError
  } = useStoryBible();

  // AI generation state
  const [isGeneratingOutline, setIsGeneratingOutline] = useState(false);

  // Modal states
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  // Removed unused editingOutline state
  const [viewingOutline, setViewingOutline] = useState<Outline | null>(null);

  // Search state
  const [searchQuery, setSearchQuery] = useState('');
  
  // Filter state
  const [localFilter, setLocalFilter] = useState({
    outlineType: '',
    characterPov: '',
    chapterNumber: '',
    visibility: ''
  });

  // Form states
  const [createForm, setCreateForm] = useState<OutlineFormData>({
    chapter_title: '',
    summary: '',
    chapter_number: null,
    character_pov: '',
    linked_document_id: undefined
  });

  const [editForm, setEditForm] = useState<OutlineFormData>({
    chapter_title: '',
    summary: '',
    chapter_number: null,
    character_pov: '',
    linked_document_id: undefined
  });

  // Options (removed unused options arrays)

  // Mock available characters - in real app, this would come from the store
  const availableCharacters = useMemo(() => [
    { id: '1', name: 'Character 1' },
    { id: '2', name: 'Character 2' }
  ], []);

  // Load outlines on mount
  useEffect(() => {
    loadOutlines(projectId);
  }, [projectId, seriesId, loadOutlines]);

  // Modal handlers
  const openCreateModal = () => {
    setCreateForm({
      chapter_title: '',
      summary: '',
      chapter_number: null,
      character_pov: '',
      linked_document_id: undefined
    });
    setShowCreateModal(true);
  };

  const openEditModal = (outline: Outline) => {
    setEditForm({
      id: outline.id,
      chapter_title: outline.chapter_title || '',
      summary: outline.summary,
      chapter_number: outline.chapter_number || null,
      character_pov: outline.character_pov || '',
      linked_document_id: outline.linked_document_id
    });
    setShowEditModal(true);
  };

  const openDetailModal = (outline: Outline) => {
    setViewingOutline(outline);
    setShowDetailModal(true);
  };

  const closeModals = () => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setShowDetailModal(false);
    setViewingOutline(null);
  };

  // CRUD handlers
  const handleCreateOutline = async () => {
    if (!createForm.summary) {
      return;
    }

    const request: CreateOutlineRequest = {
      project_id: projectId,
      chapter_title: createForm.chapter_title || undefined,
      summary: createForm.summary,
      chapter_number: createForm.chapter_number || undefined,
      character_pov: createForm.character_pov || undefined,
      linked_document_id: createForm.linked_document_id
    };

    await createOutline(request);
    closeModals();
  };

  const handleUpdateOutline = async () => {
    if (!editForm.id || !editForm.summary) {
      return;
    }

    const request: UpdateOutlineRequest = {
      id: editForm.id,
      chapter_title: editForm.chapter_title || undefined,
      summary: editForm.summary,
      chapter_number: editForm.chapter_number || undefined,
      character_pov: editForm.character_pov || undefined,
      linked_document_id: editForm.linked_document_id
    };

    await updateOutline(request);
    closeModals();
  };

  const handleDeleteOutline = async (outlineId: string) => {
    if (window.confirm('Are you sure you want to delete this outline?')) {
      await deleteOutline(outlineId);
    }
  };

  const handleGenerateOutline = async () => {
    if (!createForm.chapter_title) return;
    
    setIsGeneratingOutline(true);
    
    try {
      const request = {
        project_id: projectId,
        outline_type: 'chapter', // Default to chapter type
        title: createForm.chapter_title,
        chapter_number: createForm.chapter_number || undefined,
        story_context: '', // Add required field
        existing_outlines: [] // Add required field
      };
      
      const generatedResponse = await generateOutline(request);
      
      if (generatedResponse && generatedResponse.generated_content) {
        setCreateForm(prev => ({ ...prev, summary: generatedResponse.generated_content }));
      }
    } catch (err) {
      console.error('Failed to generate outline content:', err);
    } finally {
      setIsGeneratingOutline(false);
    }
  };

  // Search and filter handlers
  const handleSearch = async () => {
    // Simple reload for now - search functionality to be implemented
    await loadOutlines(projectId);
  };

  const handleFilterChange = (filterType: string, value: any) => {
    setLocalFilter(prev => ({
      ...prev,
      [filterType]: value || ''
    }));
  };

  // CSV Export function
  const exportOutlinesToCSV = () => {
    if (outlines.length === 0) {
      alert('No outlines to export');
      return;
    }

    const headers = [
      'Chapter Title',
      'Chapter Number',
      'Character POV',
      'Summary',
      'Linked Document ID',
      'Created Date',
      'Updated Date'
    ];

    const csvData = outlines.map(outline => [
      outline.chapter_title || '',
      outline.chapter_number || '',
      outline.character_pov ? getCharacterName(outline.character_pov) : '',
      outline.summary.replace(/"/g, '""'), // Escape quotes
      outline.linked_document_id || '',
      new Date(outline.created_at).toLocaleDateString(),
      new Date(outline.updated_at).toLocaleDateString()
    ]);

    const csvContent = [headers, ...csvData]
      .map(row => row.map(field => `"${field}"`).join(','))
      .join('\n');

    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', `outlines_${new Date().toISOString().split('T')[0]}.csv`);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  // Helper functions
  const getCharacterName = (characterId: string): string => {
    return availableCharacters.find(char => char.id === characterId)?.name || characterId;
  };

  const formatOutlineReference = (outline: Outline): string => {
    const parts = [];
    if (outline.chapter_number) parts.push(`Ch. ${outline.chapter_number}`);
    if (outline.chapter_title) parts.push(outline.chapter_title);
    return parts.length > 0 ? parts.join(' - ') : 'No reference';
  };

  return (
    <div className="outline-manager flex flex-col h-full bg-background">
      {/* Header */}
      <div className="manager-header flex justify-between items-start p-8 border-b border-border bg-muted/50">
        <div className="header-content">
          <h2 className="text-2xl font-semibold text-foreground mb-2">Story Outline</h2>
          <p className="text-sm text-muted-foreground">
            Plan and organize your story structure, chapters, scenes, and character arcs.
          </p>
        </div>
        
        <div className="header-actions flex gap-2">
          <Button variant="outline" onClick={exportOutlinesToCSV}>
            <Download className="h-4 w-4 mr-2" />
            Export CSV
          </Button>
          <Button onClick={openCreateModal}>
            <span className="mr-2">➕</span>
            Add Outline
          </Button>
        </div>
      </div>

      {/* Search and Filters */}
      <Card className="m-8 mb-4">
        <div className="p-6">
          <h3 className="text-lg font-medium mb-4">Search & Filter</h3>
          
          <div className="search-content flex flex-col gap-6">
            <div className="search-bar flex gap-3 items-center">
              <Input
                type="text"
                placeholder="Search outlines..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
                className="flex-1"
              />
              <Button variant="secondary" onClick={handleSearch}>
                🔍 Search
              </Button>
            </div>
            
            <div className="filters flex gap-4 flex-wrap">
              <div className="filter-group flex flex-col gap-2 min-w-[150px]">
                <label className="text-sm font-medium">Filter by Character POV:</label>
                <Input
                  placeholder="Character POV"
                  value={localFilter.characterPov}
                  onChange={(e) => handleFilterChange('characterPov', e.target.value)}
                />
              </div>
              
              <div className="filter-group flex flex-col gap-2 min-w-[150px]">
                <label className="text-sm font-medium">Filter by Chapter:</label>
                <Input
                  type="number"
                  placeholder="Chapter #"
                  value={localFilter.chapterNumber}
                  onChange={(e) => handleFilterChange('chapterNumber', e.target.value)}
                />
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Content Area */}
      <div className="content-area flex-1 overflow-y-auto px-8 pb-8">
        {outlinesError && (
          <div className="bg-destructive/15 border border-destructive/20 rounded-lg p-4 mb-6">
            <div className="flex justify-between items-center">
              <p className="text-destructive font-medium">{outlinesError}</p>
              <Button variant="ghost" size="sm" onClick={clearError}>
                ✕
              </Button>
            </div>
          </div>
        )}
        
        {isLoadingOutlines ? (
          <div className="loading-container flex flex-col items-center justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mb-4"></div>
            <p className="text-muted-foreground">Loading outlines...</p>
          </div>
        ) : outlines.length === 0 ? (
          <div className="empty-state flex flex-col items-center justify-center py-12 text-center">
            <span className="text-6xl mb-4">📝</span>
            <h3 className="text-xl font-semibold mb-2">No Outlines</h3>
            <p className="text-muted-foreground mb-6 max-w-md">
              Start planning your story by creating chapter outlines, scene breakdowns, and character arcs.
            </p>
            <Button onClick={openCreateModal}>
              Create First Outline
            </Button>
          </div>
        ) : (
          <div className="outlines-list flex flex-col gap-6">
            {outlines.map((outline) => (
              <Card key={outline.id} className="outline-card border border-border rounded-xl overflow-hidden">
                <div className="outline-header flex justify-between items-start p-4 pb-2">
                  <div className="outline-meta flex-1">
                    <div className="outline-title flex items-start gap-3 mb-3">
                      <span className="outline-icon text-2xl">📖</span>
                      <div className="title-content flex-1">
                        <h4 className="outline-name text-lg font-semibold text-foreground mb-1">{outline.chapter_title || `Chapter ${outline.chapter_number || 'Untitled'}`}</h4>
                        {formatOutlineReference(outline) && (
                          <div className="outline-reference text-sm text-muted-foreground">
                            {formatOutlineReference(outline)}
                          </div>
                        )}
                      </div>
                    </div>
                    <div className="outline-badges flex gap-2 flex-wrap">
                      <span className="type-badge px-2 py-1 bg-primary/10 text-primary text-xs rounded-md font-medium">
                        Chapter Outline
                      </span>
                      {outline.character_pov && (
                        <span className="pov-badge px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-md font-medium">
                          POV: {outline.character_pov}
                        </span>
                      )}
                    </div>
                  </div>
                  
                  <div className="outline-actions flex gap-1">
                    <Button 
                      variant="ghost" 
                      size="sm"
                      onClick={() => openDetailModal(outline)}
                    >
                      👁️
                    </Button>
                    <Button 
                      variant="ghost" 
                      size="sm"
                      onClick={() => openEditModal(outline)}
                    >
                      ✏️
                    </Button>
                    <Button 
                      variant="ghost" 
                      size="sm"
                      onClick={() => handleDeleteOutline(outline.id)}
                    >
                      🗑️
                    </Button>
                  </div>
                </div>
                
                <div className="outline-content px-4 pb-2">
                  <p className="outline-preview text-sm text-muted-foreground">
                    {outline.summary.substring(0, 200)}{outline.summary.length > 200 ? '...' : ''}
                  </p>
                </div>
                
                <div className="outline-footer px-4 pb-4">
                  <span className="outline-date text-xs text-muted-foreground">
                    Updated {new Date(outline.updated_at).toLocaleDateString()}
                  </span>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Create Outline Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-background rounded-lg shadow-lg w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="p-6 border-b border-border">
              <h3 className="text-lg font-semibold">Add Outline</h3>
            </div>
            
            <div className="p-6 space-y-4">
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Chapter Title:</label>
                <Input
                  value={createForm.chapter_title}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, chapter_title: e.target.value }))}
                  placeholder="Enter chapter title..."
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Chapter Number:</label>
                <Input
                  type="number"
                  value={createForm.chapter_number || ''}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, chapter_number: e.target.value ? parseInt(e.target.value) : null }))}
                  placeholder="Chapter #"
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Character POV (Optional):</label>
                <Select
                  value={createForm.character_pov}
                  onValueChange={(value) => setCreateForm(prev => ({ ...prev, character_pov: value }))}
                >
                  <option value="">No specific POV</option>
                  {availableCharacters.map(char => (
                    <option key={char.id} value={char.id}>{char.name}</option>
                  ))}
                </Select>
              </div>
              
              <div className="form-group">
                <div className="flex items-center justify-between mb-2">
                  <label className="block text-sm font-medium">Content:</label>
                  <Button
                    onClick={handleGenerateOutline}
                    disabled={isGeneratingOutline || !createForm.chapter_title}
                    variant="outline"
                    size="sm"
                    className="flex items-center gap-2"
                  >
                    {isGeneratingOutline ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      <Sparkles className="h-4 w-4" />
                    )}
                    {isGeneratingOutline ? 'Generating...' : 'Generate with AI'}
                  </Button>
                </div>
                <Textarea
                  value={createForm.summary}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, summary: e.target.value }))}
                  placeholder="Write your outline summary..."
                  rows={6}
                />
                {!createForm.chapter_title && (
                  <p className="text-sm text-gray-500 mt-1">
                    💡 Enter a chapter title to enable AI generation
                  </p>
                )}
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Linked Document ID (Optional):</label>
                <Input
                  value={createForm.linked_document_id || ''}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, linked_document_id: e.target.value || undefined }))}
                  placeholder="Enter linked document ID..."
                />
              </div>
            </div>
            
            <div className="p-6 border-t border-border flex justify-end gap-3">
              <Button variant="secondary" onClick={closeModals}>
                Cancel
              </Button>
              <Button 
                onClick={handleCreateOutline}
                disabled={!createForm.summary}
              >
                Add Outline
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Edit Outline Modal */}
      {showEditModal && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-background rounded-lg shadow-lg w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="p-6 border-b border-border">
              <h3 className="text-lg font-semibold">Edit Outline</h3>
            </div>
            
            <div className="p-6 space-y-4">
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Chapter Title:</label>
                <Input
                  value={editForm.chapter_title}
                  onChange={(e) => setEditForm(prev => ({ ...prev, chapter_title: e.target.value }))}
                  placeholder="Enter chapter title..."
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Chapter Number:</label>
                <Input
                  type="number"
                  value={editForm.chapter_number || ''}
                  onChange={(e) => setEditForm(prev => ({ ...prev, chapter_number: e.target.value ? parseInt(e.target.value) : null }))}
                  placeholder="Chapter #"
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Character POV (Optional):</label>
                <Select
                  value={editForm.character_pov}
                  onValueChange={(value) => setEditForm(prev => ({ ...prev, character_pov: value }))}
                >
                  <option value="">No specific POV</option>
                  {availableCharacters.map(char => (
                    <option key={char.id} value={char.id}>{char.name}</option>
                  ))}
                </Select>
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Summary:</label>
                <Textarea
                  value={editForm.summary}
                  onChange={(e) => setEditForm(prev => ({ ...prev, summary: e.target.value }))}
                  placeholder="Write your outline summary..."
                  rows={6}
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Linked Document ID (Optional):</label>
                <Input
                  value={editForm.linked_document_id || ''}
                  onChange={(e) => setEditForm(prev => ({ ...prev, linked_document_id: e.target.value || undefined }))}
                  placeholder="Enter linked document ID..."
                />
              </div>
            </div>
            
            <div className="p-6 border-t border-border flex justify-end gap-3">
              <Button variant="secondary" onClick={closeModals}>
                Cancel
              </Button>
              <Button 
                onClick={handleUpdateOutline}
                disabled={!editForm.summary}
              >
                Save Changes
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Detail View Modal */}
      {showDetailModal && viewingOutline && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-background rounded-lg shadow-lg w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="p-6 border-b border-border">
              <h3 className="text-lg font-semibold">{viewingOutline.chapter_title || `Chapter ${viewingOutline.chapter_number || 'Untitled'}`}</h3>
            </div>
            
            <div className="p-6">
              <div className="detail-view">
                <div className="detail-header mb-6">
                  <div className="detail-title flex items-start gap-3 mb-4">
                    <span className="detail-icon text-3xl">📖</span>
                    <div className="flex-1">
                      <h3 className="text-xl font-semibold mb-2">{viewingOutline.chapter_title || `Chapter ${viewingOutline.chapter_number || 'Untitled'}`}</h3>
                      <div className="detail-meta flex gap-4 text-sm text-muted-foreground">
                        <span className="detail-type">Chapter Outline</span>
                        {formatOutlineReference(viewingOutline) && (
                          <span className="detail-reference">{formatOutlineReference(viewingOutline)}</span>
                        )}
                      </div>
                    </div>
                  </div>
                  
                  <div className="detail-badges flex gap-2 flex-wrap">
                    <span className="chapter-badge px-2 py-1 text-xs rounded-md font-medium bg-blue-100 text-blue-800">
                      Chapter {viewingOutline.chapter_number || 'Untitled'}
                    </span>
                    {viewingOutline.character_pov && (
                      <span className="pov-badge px-2 py-1 bg-purple-100 text-purple-800 text-xs rounded-md font-medium">
                        POV: {viewingOutline.character_pov}
                      </span>
                    )}
                  </div>
                </div>
                
                <div className="detail-content">
                  <div className="detail-section mb-6">
                    <h4 className="text-lg font-medium mb-3">Summary</h4>
                    <div className="content-text p-4 bg-muted/50 rounded-lg whitespace-pre-wrap">
                      {viewingOutline.summary}
                    </div>
                  </div>
                  
                  <div className="detail-meta-info grid grid-cols-2 gap-4 text-sm">
                    <div className="meta-item">
                      <strong>Created:</strong> {new Date(viewingOutline.created_at).toLocaleDateString()}
                    </div>
                    <div className="meta-item">
                      <strong>Last Updated:</strong> {new Date(viewingOutline.updated_at).toLocaleDateString()}
                    </div>
                  </div>
                </div>
              </div>
            </div>
            
            <div className="p-6 border-t border-border flex justify-end gap-3">
              <Button variant="secondary" onClick={closeModals}>
                Close
              </Button>
              <Button onClick={() => { closeModals(); openEditModal(viewingOutline); }}>
                Edit Outline
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default OutlineManager;