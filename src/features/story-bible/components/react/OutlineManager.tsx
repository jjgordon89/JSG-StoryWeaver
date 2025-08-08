import React, { useState, useEffect, useMemo } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select } from '../../../../components/ui/select';
import { Sparkles, Loader2 } from 'lucide-react';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { Outline, CreateOutlineRequest, UpdateOutlineRequest } from '../../../../types/storyBible';

interface OutlineManagerProps {
  projectId: string;
  seriesId?: string;
}

interface OutlineFormData {
  id?: string;
  title: string;
  content: string;
  outline_type: string;
  chapter_number: number | null;
  character_pov: string;
  act_number: number | null;
  scene_number: number | null;
  visibility: string;
  series_shared: boolean;
}

const OutlineManager: React.FC<OutlineManagerProps> = ({ projectId, seriesId }) => {
  const {
    outlines,
    isLoadingOutlines,
    outlinesError,
    outlineFilter,
    createOutline,
    updateOutline,
    deleteOutline,
    loadOutlines,
    searchOutlines,
    setOutlineFilter,
    clearError,
    generateOutline
  } = useStoryBible();

  // AI generation state
  const [isGeneratingOutline, setIsGeneratingOutline] = useState(false);

  // Modal states
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [editingOutline, setEditingOutline] = useState<Outline | null>(null);
  const [viewingOutline, setViewingOutline] = useState<Outline | null>(null);

  // Search state
  const [searchQuery, setSearchQuery] = useState('');

  // Form states
  const [createForm, setCreateForm] = useState<OutlineFormData>({
    title: '',
    content: '',
    outline_type: '',
    chapter_number: null,
    character_pov: '',
    act_number: null,
    scene_number: null,
    visibility: 'always',
    series_shared: false
  });

  const [editForm, setEditForm] = useState<OutlineFormData>({
    title: '',
    content: '',
    outline_type: '',
    chapter_number: null,
    character_pov: '',
    act_number: null,
    scene_number: null,
    visibility: 'always',
    series_shared: false
  });

  // Options
  const outlineTypeOptions = [
    { value: '', label: 'Select type...' },
    { value: 'chapter', label: 'Chapter' },
    { value: 'scene', label: 'Scene' },
    { value: 'act', label: 'Act' },
    { value: 'character_arc', label: 'Character Arc' },
    { value: 'plot_thread', label: 'Plot Thread' },
    { value: 'subplot', label: 'Subplot' },
    { value: 'theme', label: 'Theme' },
    { value: 'conflict', label: 'Conflict' },
    { value: 'pacing', label: 'Pacing' },
    { value: 'structure', label: 'Structure' },
    { value: 'other', label: 'Other' }
  ];

  const visibilityOptions = [
    { value: 'always', label: 'Always Visible' },
    { value: 'chapter', label: 'Chapter Context' },
    { value: 'never', label: 'Hidden' }
  ];

  // Mock available characters - in real app, this would come from the store
  const availableCharacters = useMemo(() => [
    { id: '1', name: 'Character 1' },
    { id: '2', name: 'Character 2' }
  ], []);

  // Load outlines on mount
  useEffect(() => {
    loadOutlines(projectId, seriesId);
  }, [projectId, seriesId, loadOutlines]);

  // Modal handlers
  const openCreateModal = () => {
    setCreateForm({
      title: '',
      content: '',
      outline_type: '',
      chapter_number: null,
      character_pov: '',
      act_number: null,
      scene_number: null,
      visibility: 'always',
      series_shared: false
    });
    setShowCreateModal(true);
  };

  const openEditModal = (outline: Outline) => {
    setEditingOutline(outline);
    setEditForm({
      id: outline.id,
      title: outline.title,
      content: outline.content,
      outline_type: outline.outline_type,
      chapter_number: outline.chapter_number,
      character_pov: outline.character_pov || '',
      act_number: outline.act_number,
      scene_number: outline.scene_number,
      visibility: outline.visibility,
      series_shared: outline.series_shared
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
    setEditingOutline(null);
    setViewingOutline(null);
  };

  // CRUD handlers
  const handleCreateOutline = async () => {
    if (!createForm.title || !createForm.content || !createForm.outline_type) {
      return;
    }

    const request: CreateOutlineRequest = {
      project_id: projectId,
      series_id: seriesId,
      title: createForm.title,
      content: createForm.content,
      outline_type: createForm.outline_type,
      chapter_number: createForm.chapter_number,
      character_pov: createForm.character_pov || undefined,
      act_number: createForm.act_number,
      scene_number: createForm.scene_number,
      visibility: createForm.visibility,
      series_shared: createForm.series_shared
    };

    await createOutline(request);
    closeModals();
  };

  const handleUpdateOutline = async () => {
    if (!editForm.id || !editForm.title || !editForm.content || !editForm.outline_type) {
      return;
    }

    const request: UpdateOutlineRequest = {
      id: editForm.id,
      title: editForm.title,
      content: editForm.content,
      outline_type: editForm.outline_type,
      chapter_number: editForm.chapter_number,
      character_pov: editForm.character_pov || undefined,
      act_number: editForm.act_number,
      scene_number: editForm.scene_number,
      visibility: editForm.visibility,
      series_shared: editForm.series_shared
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
    if (!createForm.outline_type || !createForm.title) return;
    
    setIsGeneratingOutline(true);
    
    try {
      const request = {
        project_id: projectId,
        outline_type: createForm.outline_type,
        title: createForm.title,
        chapter_number: createForm.chapter_number,
        scene_number: createForm.scene_number
      };
      
      const generatedContent = await generateOutline(request);
      
      if (generatedContent) {
        setCreateForm(prev => ({ ...prev, content: generatedContent }));
      }
    } catch (err) {
      console.error('Failed to generate outline content:', err);
    } finally {
      setIsGeneratingOutline(false);
    }
  };

  // Search and filter handlers
  const handleSearch = async () => {
    if (searchQuery.trim()) {
      await searchOutlines(projectId, searchQuery, seriesId);
    } else {
      await loadOutlines(projectId, seriesId);
    }
  };

  const handleFilterChange = (filterType: string, value: any) => {
    const currentFilter = outlineFilter;
    setOutlineFilter({
      ...currentFilter,
      [filterType]: value || undefined
    });
  };

  // Helper functions
  const getOutlineTypeLabel = (outlineType: string): string => {
    return outlineTypeOptions.find(opt => opt.value === outlineType)?.label || outlineType;
  };

  const getVisibilityLabel = (visibility: string): string => {
    return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
  };

  const getCharacterName = (characterId: string): string => {
    return availableCharacters.find(char => char.id === characterId)?.name || characterId;
  };

  const getOutlineIcon = (outlineType: string): string => {
    const icons: Record<string, string> = {
      chapter: '📖',
      scene: '🎬',
      act: '🎭',
      character_arc: '👤',
      plot_thread: '🧵',
      subplot: '📝',
      theme: '💭',
      conflict: '⚔️',
      pacing: '⏱️',
      structure: '🏗️',
      other: '📋'
    };
    return icons[outlineType] || '📋';
  };

  const formatOutlineReference = (outline: Outline): string => {
    const parts = [];
    if (outline.act_number) parts.push(`Act ${outline.act_number}`);
    if (outline.chapter_number) parts.push(`Ch. ${outline.chapter_number}`);
    if (outline.scene_number) parts.push(`Scene ${outline.scene_number}`);
    if (outline.character_pov) parts.push(`POV: ${getCharacterName(outline.character_pov)}`);
    return parts.join(' • ');
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
        
        <div className="header-actions">
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
                <label className="text-sm font-medium">Filter by Type:</label>
                <Select
                  value={outlineFilter.outlineType || ''}
                  onValueChange={(value) => handleFilterChange('outlineType', value)}
                >
                  <option value="">All types</option>
                  {outlineTypeOptions.slice(1).map(option => (
                    <option key={option.value} value={option.value}>{option.label}</option>
                  ))}
                </Select>
              </div>
              
              <div className="filter-group flex flex-col gap-2 min-w-[150px]">
                <label className="text-sm font-medium">Filter by Character POV:</label>
                <Select
                  value={outlineFilter.characterPov || ''}
                  onValueChange={(value) => handleFilterChange('characterPov', value)}
                >
                  <option value="">All characters</option>
                  {availableCharacters.map(char => (
                    <option key={char.id} value={char.id}>{char.name}</option>
                  ))}
                </Select>
              </div>
              
              <div className="filter-group flex flex-col gap-2 min-w-[150px]">
                <label className="text-sm font-medium">Filter by Chapter:</label>
                <Input
                  type="number"
                  placeholder="Chapter #"
                  value={outlineFilter.chapterNumber || ''}
                  onChange={(e) => handleFilterChange('chapterNumber', e.target.value ? parseInt(e.target.value) : undefined)}
                />
              </div>
              
              <div className="filter-group flex flex-col gap-2 min-w-[150px]">
                <label className="text-sm font-medium">Filter by Visibility:</label>
                <Select
                  value={outlineFilter.visibility || ''}
                  onValueChange={(value) => handleFilterChange('visibility', value)}
                >
                  <option value="">All visibility</option>
                  {visibilityOptions.map(option => (
                    <option key={option.value} value={option.value}>{option.label}</option>
                  ))}
                </Select>
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
                      <span className="outline-icon text-2xl">{getOutlineIcon(outline.outline_type)}</span>
                      <div className="title-content flex-1">
                        <h4 className="outline-name text-lg font-semibold text-foreground mb-1">{outline.title}</h4>
                        {formatOutlineReference(outline) && (
                          <div className="outline-reference text-sm text-muted-foreground">
                            {formatOutlineReference(outline)}
                          </div>
                        )}
                      </div>
                    </div>
                    <div className="outline-badges flex gap-2 flex-wrap">
                      <span className="type-badge px-2 py-1 bg-primary/10 text-primary text-xs rounded-md font-medium">
                        {getOutlineTypeLabel(outline.outline_type)}
                      </span>
                      <span className={`visibility-badge px-2 py-1 text-xs rounded-md font-medium ${
                        outline.visibility === 'always' ? 'bg-green-100 text-green-800' :
                        outline.visibility === 'chapter' ? 'bg-yellow-100 text-yellow-800' :
                        'bg-red-100 text-red-800'
                      }`}>
                        {getVisibilityLabel(outline.visibility)}
                      </span>
                      {outline.series_shared && (
                        <span className="series-badge px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-md font-medium">
                          Series Shared
                        </span>
                      )}
                    </div>
                  </div>
                  
                  <div className="outline-actions flex gap-1">
                    <Button 
                      variant="ghost" 
                      size="sm"
                      onClick={() => openDetailModal(outline)}
                      title="View Details"
                    >
                      👁️
                    </Button>
                    <Button 
                      variant="ghost" 
                      size="sm"
                      onClick={() => openEditModal(outline)}
                      title="Edit"
                    >
                      ✏️
                    </Button>
                    <Button 
                      variant="ghost" 
                      size="sm"
                      onClick={() => handleDeleteOutline(outline.id)}
                      title="Delete"
                    >
                      🗑️
                    </Button>
                  </div>
                </div>
                
                <div className="outline-content px-4 pb-2">
                  <p className="outline-preview text-sm text-muted-foreground">
                    {outline.content.substring(0, 200)}{outline.content.length > 200 ? '...' : ''}
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
                <label className="block text-sm font-medium mb-2">Title:</label>
                <Input
                  value={createForm.title}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, title: e.target.value }))}
                  placeholder="Enter outline title..."
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Outline Type:</label>
                <Select
                  value={createForm.outline_type}
                  onValueChange={(value) => setCreateForm(prev => ({ ...prev, outline_type: value }))}
                >
                  {outlineTypeOptions.map(option => (
                    <option key={option.value} value={option.value}>{option.label}</option>
                  ))}
                </Select>
              </div>
              
              <div className="form-row grid grid-cols-3 gap-4">
                <div className="form-group">
                  <label className="block text-sm font-medium mb-2">Act Number:</label>
                  <Input
                    type="number"
                    value={createForm.act_number || ''}
                    onChange={(e) => setCreateForm(prev => ({ ...prev, act_number: e.target.value ? parseInt(e.target.value) : null }))}
                    placeholder="Act #"
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
                  <label className="block text-sm font-medium mb-2">Scene Number:</label>
                  <Input
                    type="number"
                    value={createForm.scene_number || ''}
                    onChange={(e) => setCreateForm(prev => ({ ...prev, scene_number: e.target.value ? parseInt(e.target.value) : null }))}
                    placeholder="Scene #"
                  />
                </div>
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
                    disabled={isGeneratingOutline || !createForm.outline_type || !createForm.title}
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
                  value={createForm.content}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, content: e.target.value }))}
                  placeholder="Write your outline content..."
                  rows={6}
                />
                {(!createForm.outline_type || !createForm.title) && (
                  <p className="text-sm text-gray-500 mt-1">
                    💡 Enter a title and select an outline type to enable AI generation
                  </p>
                )}
              </div>
              
              <div className="form-row grid grid-cols-2 gap-4">
                <div className="form-group">
                  <label className="block text-sm font-medium mb-2">Visibility:</label>
                  <Select
                    value={createForm.visibility}
                    onValueChange={(value) => setCreateForm(prev => ({ ...prev, visibility: value }))}
                  >
                    {visibilityOptions.map(option => (
                      <option key={option.value} value={option.value}>{option.label}</option>
                    ))}
                  </Select>
                </div>
                
                <div className="form-group">
                  <label className="flex items-center gap-2 text-sm font-medium">
                    <input 
                      type="checkbox" 
                      checked={createForm.series_shared}
                      onChange={(e) => setCreateForm(prev => ({ ...prev, series_shared: e.target.checked }))}
                      className="rounded"
                    />
                    Share across series
                  </label>
                </div>
              </div>
            </div>
            
            <div className="p-6 border-t border-border flex justify-end gap-3">
              <Button variant="secondary" onClick={closeModals}>
                Cancel
              </Button>
              <Button 
                onClick={handleCreateOutline}
                disabled={!createForm.title || !createForm.outline_type || !createForm.content}
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
                <label className="block text-sm font-medium mb-2">Title:</label>
                <Input
                  value={editForm.title}
                  onChange={(e) => setEditForm(prev => ({ ...prev, title: e.target.value }))}
                  placeholder="Enter outline title..."
                />
              </div>
              
              <div className="form-group">
                <label className="block text-sm font-medium mb-2">Outline Type:</label>
                <Select
                  value={editForm.outline_type}
                  onValueChange={(value) => setEditForm(prev => ({ ...prev, outline_type: value }))}
                >
                  {outlineTypeOptions.map(option => (
                    <option key={option.value} value={option.value}>{option.label}</option>
                  ))}
                </Select>
              </div>
              
              <div className="form-row grid grid-cols-3 gap-4">
                <div className="form-group">
                  <label className="block text-sm font-medium mb-2">Act Number:</label>
                  <Input
                    type="number"
                    value={editForm.act_number || ''}
                    onChange={(e) => setEditForm(prev => ({ ...prev, act_number: e.target.value ? parseInt(e.target.value) : null }))}
                    placeholder="Act #"
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
                  <label className="block text-sm font-medium mb-2">Scene Number:</label>
                  <Input
                    type="number"
                    value={editForm.scene_number || ''}
                    onChange={(e) => setEditForm(prev => ({ ...prev, scene_number: e.target.value ? parseInt(e.target.value) : null }))}
                    placeholder="Scene #"
                  />
                </div>
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
                <label className="block text-sm font-medium mb-2">Content:</label>
                <Textarea
                  value={editForm.content}
                  onChange={(e) => setEditForm(prev => ({ ...prev, content: e.target.value }))}
                  placeholder="Write your outline content..."
                  rows={6}
                />
              </div>
              
              <div className="form-row grid grid-cols-2 gap-4">
                <div className="form-group">
                  <label className="block text-sm font-medium mb-2">Visibility:</label>
                  <Select
                    value={editForm.visibility}
                    onValueChange={(value) => setEditForm(prev => ({ ...prev, visibility: value }))}
                  >
                    {visibilityOptions.map(option => (
                      <option key={option.value} value={option.value}>{option.label}</option>
                    ))}
                  </Select>
                </div>
                
                <div className="form-group">
                  <label className="flex items-center gap-2 text-sm font-medium">
                    <input 
                      type="checkbox" 
                      checked={editForm.series_shared}
                      onChange={(e) => setEditForm(prev => ({ ...prev, series_shared: e.target.checked }))}
                      className="rounded"
                    />
                    Share across series
                  </label>
                </div>
              </div>
            </div>
            
            <div className="p-6 border-t border-border flex justify-end gap-3">
              <Button variant="secondary" onClick={closeModals}>
                Cancel
              </Button>
              <Button 
                onClick={handleUpdateOutline}
                disabled={!editForm.title || !editForm.outline_type || !editForm.content}
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
              <h3 className="text-lg font-semibold">{viewingOutline.title}</h3>
            </div>
            
            <div className="p-6">
              <div className="detail-view">
                <div className="detail-header mb-6">
                  <div className="detail-title flex items-start gap-3 mb-4">
                    <span className="detail-icon text-3xl">{getOutlineIcon(viewingOutline.outline_type)}</span>
                    <div className="flex-1">
                      <h3 className="text-xl font-semibold mb-2">{viewingOutline.title}</h3>
                      <div className="detail-meta flex gap-4 text-sm text-muted-foreground">
                        <span className="detail-type">{getOutlineTypeLabel(viewingOutline.outline_type)}</span>
                        {formatOutlineReference(viewingOutline) && (
                          <span className="detail-reference">{formatOutlineReference(viewingOutline)}</span>
                        )}
                      </div>
                    </div>
                  </div>
                  
                  <div className="detail-badges flex gap-2 flex-wrap">
                    <span className={`visibility-badge px-2 py-1 text-xs rounded-md font-medium ${
                      viewingOutline.visibility === 'always' ? 'bg-green-100 text-green-800' :
                      viewingOutline.visibility === 'chapter' ? 'bg-yellow-100 text-yellow-800' :
                      'bg-red-100 text-red-800'
                    }`}>
                      {getVisibilityLabel(viewingOutline.visibility)}
                    </span>
                    {viewingOutline.series_shared && (
                      <span className="series-badge px-2 py-1 bg-blue-100 text-blue-800 text-xs rounded-md font-medium">
                        Series Shared
                      </span>
                    )}
                  </div>
                </div>
                
                <div className="detail-content">
                  <div className="detail-section mb-6">
                    <h4 className="text-lg font-medium mb-3">Content</h4>
                    <div className="content-text p-4 bg-muted/50 rounded-lg whitespace-pre-wrap">
                      {viewingOutline.content}
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