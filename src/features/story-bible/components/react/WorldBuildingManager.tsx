import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Sparkles, Loader2, Download } from 'lucide-react';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { WorldElement, CreateWorldElementRequest, UpdateWorldElementRequest } from '../../../../types/storyBible';

interface WorldBuildingManagerProps {
  projectId: string;
  seriesId?: string;
}

interface CreateForm {
  name: string;
  element_type: string;
  description: string;
  details: string;
  visibility: 'always' | 'chapter' | 'never';
  series_shared: boolean;
}

interface EditForm extends CreateForm {
  id: string;
}

const WorldBuildingManager: React.FC<WorldBuildingManagerProps> = ({ projectId, seriesId }) => {
  const {
    worldElements,
    filteredWorldElements,
    worldElementFilter,
    isLoadingWorldElements,
    worldElementsError,
    loadWorldElements,
    createWorldElement,
    updateWorldElement,
    deleteWorldElement,
    searchWorldElements,
    setWorldElementFilter,
    clearError,
    generateWorldBuilding
  } = useStoryBible();

  // AI generation state
  const [isGeneratingWorldBuilding, setIsGeneratingWorldBuilding] = useState(false);

  // Modal state
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [viewingElement, setViewingElement] = useState<WorldElement | null>(null);

  // Form state
  const [createForm, setCreateForm] = useState<CreateForm>({
    name: '',
    element_type: '',
    description: '',
    significance: '',
    visibility: 'always',
    series_shared: false
  });

  const [editForm, setEditForm] = useState<EditForm>({
    id: '',
    name: '',
    element_type: '',
    description: '',
    details: '',
    visibility: 'always',
    series_shared: false
  });

  // Search state
  const [searchQuery, setSearchQuery] = useState('');

  // Element type options
  const elementTypeOptions = [
    { value: '', label: 'Select element type' },
    { value: 'location', label: 'Location' },
    { value: 'organization', label: 'Organization' },
    { value: 'culture', label: 'Culture' },
    { value: 'religion', label: 'Religion' },
    { value: 'government', label: 'Government' },
    { value: 'technology', label: 'Technology' },
    { value: 'magic', label: 'Magic System' },
    { value: 'history', label: 'Historical Event' },
    { value: 'language', label: 'Language' },
    { value: 'currency', label: 'Currency' },
    { value: 'law', label: 'Laws & Rules' },
    { value: 'custom', label: 'Custom/Tradition' },
    { value: 'artifact', label: 'Artifact/Item' },
    { value: 'creature', label: 'Creature/Species' },
    { value: 'other', label: 'Other' }
  ];

  // Visibility options
  const visibilityOptions = [
    { value: 'always', label: 'Always Visible' },
    { value: 'chapter', label: 'Chapter Context' },
    { value: 'never', label: 'Hidden' }
  ];

  useEffect(() => {
    loadWorldElements(projectId);
  }, [projectId, loadWorldElements]);

  const openCreateModal = () => {
    setCreateForm({
      name: '',
      element_type: '',
      description: '',
      details: '',
      visibility: 'always',
      series_shared: false
    });
    setShowCreateModal(true);
  };

  const openEditModal = (element: WorldElement) => {
    setEditForm({
      id: element.id,
      name: element.name,
      element_type: element.element_type,
      description: element.description,
      details: element.details || '',
      visibility: element.visibility,
      series_shared: element.series_shared
    });
    setShowEditModal(true);
  };

  const openDetailModal = (element: WorldElement) => {
    setViewingElement(element);
    setShowDetailModal(true);
  };

  const closeModals = () => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setShowDetailModal(false);
    setViewingElement(null);
  };

  const handleCreateElement = async () => {
    if (!createForm.name || !createForm.element_type || !createForm.description) {
      return;
    }

    const request: CreateWorldElementRequest = {
      project_id: projectId,
      series_id: seriesId,
      name: createForm.name,
      element_type: createForm.element_type,
      description: createForm.description,
      details: createForm.details || undefined,
      visibility: createForm.visibility,
      series_shared: createForm.series_shared
    };

    await createWorldElement(request);
    closeModals();
  };

  const handleUpdateElement = async () => {
    if (!editForm.id || !editForm.name || !editForm.element_type || !editForm.description) {
      return;
    }

    const request: UpdateWorldElementRequest = {
      id: editForm.id,
      name: editForm.name,
      element_type: editForm.element_type,
      description: editForm.description,
      details: editForm.details || undefined,
      visibility: editForm.visibility,
      series_shared: editForm.series_shared
    };

    await updateWorldElement(request);
    closeModals();
  };

  const handleDeleteElement = async (elementId: string) => {
    if (window.confirm('Are you sure you want to delete this world element?')) {
      await deleteWorldElement(elementId);
    }
  };

  const handleGenerateWorldBuilding = async () => {
    if (!createForm.element_type || !createForm.name) return;
    
    setIsGeneratingWorldBuilding(true);
    
    try {
      const request = {
        project_id: projectId,
        element_type: createForm.element_type,
        element_name: createForm.name,
        story_context: storyBible?.braindump || '',
        existing_elements: worldElements.map(el => el.name)
      };
      
      const generatedContent = await generateWorldBuilding(request);
      
      if (generatedContent) {
        setCreateForm(prev => ({ ...prev, description: generatedContent }));
      }
    } catch (err) {
      console.error('Failed to generate worldbuilding content:', err);
    } finally {
      setIsGeneratingWorldBuilding(false);
    }
  };

  const handleExportCSV = () => {
    if (worldElements.length === 0) return;
    
    // Create CSV content
    const headers = ['Name', 'Type', 'Description', 'Visibility', 'Series Shared'];
    const rows = worldElements.map(element => [
      element.name,
      getElementTypeLabel(element.element_type),
      element.description.replace(/"/g, '""'), // Escape quotes
      getVisibilityLabel(element.visibility),
      element.series_shared ? 'Yes' : 'No'
    ]);
    
    const csvContent = [
      headers.join(','),
      ...rows.map(row => row.map(cell => `"${cell}"`).join(','))
    ].join('\n');
    
    // Download CSV
    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
    const link = document.createElement('a');
    const url = URL.createObjectURL(blob);
    link.setAttribute('href', url);
    link.setAttribute('download', 'worldbuilding_elements.csv');
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const handleSearch = async () => {
    if (searchQuery.trim()) {
      await searchWorldElements(projectId, searchQuery, seriesId);
    } else {
      await loadWorldElements(projectId, seriesId);
    }
  };

  const handleFilterChange = (filterType: string, value: any) => {
    const currentFilter = worldElementFilter;
    setWorldElementFilter({
      ...currentFilter,
      [filterType]: value || undefined
    });
  };

  const getElementTypeLabel = (elementType: string): string => {
    return elementTypeOptions.find(opt => opt.value === elementType)?.label || elementType;
  };

  const getVisibilityLabel = (visibility: string): string => {
    return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
  };

  const getElementIcon = (elementType: string): string => {
    const icons: Record<string, string> = {
      location: '🏛️',
      organization: '🏢',
      culture: '🎭',
      religion: '⛪',
      government: '🏛️',
      technology: '⚙️',
      magic: '✨',
      history: '📜',
      language: '🗣️',
      currency: '💰',
      law: '⚖️',
      custom: '🎪',
      artifact: '🏺',
      creature: '🐉',
      other: '📋'
    };
    return icons[elementType] || '📋';
  };

  return (
    <div className="world-building-manager space-y-6">
      {/* Header */}
      <div className="flex justify-between items-start">
        <div>
          <h2 className="text-2xl font-bold">World Building</h2>
          <p className="text-muted-foreground mt-1">
            Create and manage the world elements that shape your story's universe.
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            onClick={handleExportCSV}
            variant="outline"
            disabled={worldElements.length === 0}
          >
            <Download className="h-4 w-4 mr-2" />
            Export CSV
          </Button>
          <Button onClick={openCreateModal}>
            <span className="mr-2">➕</span>
            Add Element
          </Button>
        </div>
      </div>

      {/* Search and Filters */}
      <Card>
        <CardHeader>
          <CardTitle>Search & Filter</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex gap-2">
            <Input
              type="text"
              placeholder="Search world elements..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="flex-1"
            />
            <Button variant="secondary" onClick={handleSearch}>
              🔍 Search
            </Button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <label className="text-sm font-medium mb-2 block">Filter by Type:</label>
              <Select
                value={worldElementFilter.elementType || ''}
                onValueChange={(value) => handleFilterChange('elementType', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All types" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All types</SelectItem>
                  {elementTypeOptions.slice(1).map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div>
              <label className="text-sm font-medium mb-2 block">Filter by Visibility:</label>
              <Select
                value={worldElementFilter.visibility || ''}
                onValueChange={(value) => handleFilterChange('visibility', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All visibility" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All visibility</SelectItem>
                  {visibilityOptions.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="flex items-end">
              <label className="flex items-center space-x-2">
                <input
                  type="checkbox"
                  checked={worldElementFilter.seriesShared || false}
                  onChange={(e) => handleFilterChange('seriesShared', e.target.checked)}
                  className="rounded"
                />
                <span className="text-sm font-medium">Series Shared Only</span>
              </label>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Content Area */}
      <div>
        {worldElementsError && (
          <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-4">
            <div className="flex justify-between items-center">
              <p className="text-red-800">{worldElementsError}</p>
              <Button variant="ghost" size="sm" onClick={clearError}>
                ✕
              </Button>
            </div>
          </div>
        )}

        {isLoadingWorldElements ? (
          <div className="flex flex-col items-center justify-center py-12">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mb-4"></div>
            <p className="text-muted-foreground">Loading world elements...</p>
          </div>
        ) : filteredWorldElements.length === 0 ? (
          <div className="text-center py-12">
            <span className="text-6xl mb-4 block">🌍</span>
            <h3 className="text-xl font-semibold mb-2">No World Elements</h3>
            <p className="text-muted-foreground mb-4">
              Start building your story's world by adding locations, cultures, organizations, and more.
            </p>
            <Button onClick={openCreateModal}>
              Create First Element
            </Button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {filteredWorldElements.map((element) => (
              <Card key={element.id} className="hover:shadow-md transition-shadow">
                <CardHeader className="pb-3">
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="text-xl">{getElementIcon(element.element_type)}</span>
                        <h4 className="font-semibold truncate">{element.name}</h4>
                      </div>
                      <div className="flex flex-wrap gap-1">
                        <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                          {getElementTypeLabel(element.element_type)}
                        </span>
                        <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                          element.visibility === 'always' ? 'bg-green-100 text-green-800' :
                          element.visibility === 'chapter' ? 'bg-yellow-100 text-yellow-800' :
                          'bg-gray-100 text-gray-800'
                        }`}>
                          {getVisibilityLabel(element.visibility)}
                        </span>
                        {element.series_shared && (
                          <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-purple-100 text-purple-800">
                            Series Shared
                          </span>
                        )}
                      </div>
                    </div>
                    <div className="flex gap-1">
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => openDetailModal(element)}
                        title="View Details"
                      >
                        👁️
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => openEditModal(element)}
                        title="Edit"
                      >
                        ✏️
                      </Button>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => handleDeleteElement(element.id)}
                        title="Delete"
                      >
                        🗑️
                      </Button>
                    </div>
                  </div>
                </CardHeader>
                <CardContent>
                  <p className="text-sm text-muted-foreground mb-2 line-clamp-3">
                    {element.description}
                  </p>
                  {element.significance && (
                    <div className="text-sm">
                      <strong>Significance:</strong> {element.significance}
                    </div>
                  )}
                  <div className="text-xs text-muted-foreground mt-2">
                    Updated {new Date(element.updated_at).toLocaleDateString()}
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Create Element Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold">Add World Element</h3>
              <Button variant="ghost" size="sm" onClick={closeModals}>
                ✕
              </Button>
            </div>

            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium mb-2 block">Name:</label>
                <Input
                  value={createForm.name}
                  onChange={(e) => setCreateForm({ ...createForm, name: e.target.value })}
                  placeholder="Enter element name..."
                />
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Element Type:</label>
                <Select
                  value={createForm.element_type}
                  onValueChange={(value) => setCreateForm({ ...createForm, element_type: value })}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select element type" />
                  </SelectTrigger>
                  <SelectContent>
                    {elementTypeOptions.map((option) => (
                      <SelectItem key={option.value} value={option.value} disabled={!option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div>
                <div className="flex items-center justify-between mb-2">
                  <label className="text-sm font-medium">Description:</label>
                  <Button
                    onClick={handleGenerateWorldBuilding}
                    disabled={isGeneratingWorldBuilding || !createForm.element_type || !createForm.name}
                    variant="outline"
                    size="sm"
                    className="flex items-center gap-2"
                  >
                    {isGeneratingWorldBuilding ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      <Sparkles className="h-4 w-4" />
                    )}
                    {isGeneratingWorldBuilding ? 'Generating...' : 'Generate with AI'}
                  </Button>
                </div>
                <Textarea
                  value={createForm.description}
                  onChange={(e) => setCreateForm({ ...createForm, description: e.target.value })}
                  placeholder="Describe this world element..."
                  rows={4}
                />
                {(!createForm.element_type || !createForm.name) && (
                  <p className="text-sm text-gray-500 mt-1">
                    💡 Enter a name and select an element type to enable AI generation
                  </p>
                )}
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Significance (Optional):</label>
                <Textarea
                  value={createForm.significance}
                  onChange={(e) => setCreateForm({ ...createForm, significance: e.target.value })}
                  placeholder="Why is this element important to your story?"
                  rows={2}
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="text-sm font-medium mb-2 block">Visibility:</label>
                  <Select
                    value={createForm.visibility}
                    onValueChange={(value: 'always' | 'chapter' | 'never') => setCreateForm({ ...createForm, visibility: value })}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {visibilityOptions.map((option) => (
                        <SelectItem key={option.value} value={option.value}>
                          {option.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>

                <div className="flex items-end">
                  <label className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      checked={createForm.series_shared}
                      onChange={(e) => setCreateForm({ ...createForm, series_shared: e.target.checked })}
                      className="rounded"
                    />
                    <span className="text-sm font-medium">Share across series</span>
                  </label>
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-2 mt-6">
              <Button variant="secondary" onClick={closeModals}>
                Cancel
              </Button>
              <Button
                onClick={handleCreateElement}
                disabled={!createForm.name || !createForm.element_type || !createForm.description}
              >
                Add Element
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Edit Element Modal */}
      {showEditModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold">Edit World Element</h3>
              <Button variant="ghost" size="sm" onClick={closeModals}>
                ✕
              </Button>
            </div>

            <div className="space-y-4">
              <div>
                <label className="text-sm font-medium mb-2 block">Name:</label>
                <Input
                  value={editForm.name}
                  onChange={(e) => setEditForm({ ...editForm, name: e.target.value })}
                  placeholder="Enter element name..."
                />
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Element Type:</label>
                <Select
                  value={editForm.element_type}
                  onValueChange={(value) => setEditForm({ ...editForm, element_type: value })}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select element type" />
                  </SelectTrigger>
                  <SelectContent>
                    {elementTypeOptions.map((option) => (
                      <SelectItem key={option.value} value={option.value} disabled={!option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Description:</label>
                <Textarea
                  value={editForm.description}
                  onChange={(e) => setEditForm({ ...editForm, description: e.target.value })}
                  placeholder="Describe this world element..."
                  rows={4}
                />
              </div>

              <div>
                <label className="text-sm font-medium mb-2 block">Significance (Optional):</label>
                <Textarea
                  value={editForm.significance}
                  onChange={(e) => setEditForm({ ...editForm, significance: e.target.value })}
                  placeholder="Why is this element important to your story?"
                  rows={2}
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="text-sm font-medium mb-2 block">Visibility:</label>
                  <Select
                    value={editForm.visibility}
                    onValueChange={(value: 'always' | 'chapter' | 'never') => setEditForm({ ...editForm, visibility: value })}
                  >
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      {visibilityOptions.map((option) => (
                        <SelectItem key={option.value} value={option.value}>
                          {option.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>

                <div className="flex items-end">
                  <label className="flex items-center space-x-2">
                    <input
                      type="checkbox"
                      checked={editForm.series_shared}
                      onChange={(e) => setEditForm({ ...editForm, series_shared: e.target.checked })}
                      className="rounded"
                    />
                    <span className="text-sm font-medium">Share across series</span>
                  </label>
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-2 mt-6">
              <Button variant="secondary" onClick={closeModals}>
                Cancel
              </Button>
              <Button
                onClick={handleUpdateElement}
                disabled={!editForm.name || !editForm.element_type || !editForm.description}
              >
                Save Changes
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Detail View Modal */}
      {showDetailModal && viewingElement && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold">{viewingElement.name}</h3>
              <Button variant="ghost" size="sm" onClick={closeModals}>
                ✕
              </Button>
            </div>

            <div className="space-y-4">
              <div className="flex items-center gap-3">
                <span className="text-2xl">{getElementIcon(viewingElement.element_type)}</span>
                <div>
                  <h4 className="font-semibold">{viewingElement.name}</h4>
                  <span className="text-sm text-muted-foreground">
                    {getElementTypeLabel(viewingElement.element_type)}
                  </span>
                </div>
                <div className="ml-auto flex gap-2">
                  <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${
                    viewingElement.visibility === 'always' ? 'bg-green-100 text-green-800' :
                    viewingElement.visibility === 'chapter' ? 'bg-yellow-100 text-yellow-800' :
                    'bg-gray-100 text-gray-800'
                  }`}>
                    {getVisibilityLabel(viewingElement.visibility)}
                  </span>
                  {viewingElement.series_shared && (
                    <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-purple-100 text-purple-800">
                      Series Shared
                    </span>
                  )}
                </div>
              </div>

              <div>
                <h4 className="font-semibold mb-2">Description</h4>
                <p className="text-sm text-muted-foreground">{viewingElement.description}</p>
              </div>

              {viewingElement.significance && (
                <div>
                  <h4 className="font-semibold mb-2">Significance</h4>
                  <p className="text-sm text-muted-foreground">{viewingElement.significance}</p>
                </div>
              )}

              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <strong>Created:</strong> {new Date(viewingElement.created_at).toLocaleDateString()}
                </div>
                <div>
                  <strong>Last Updated:</strong> {new Date(viewingElement.updated_at).toLocaleDateString()}
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-2 mt-6">
              <Button variant="secondary" onClick={closeModals}>
                Close
              </Button>
              <Button onClick={() => { closeModals(); openEditModal(viewingElement); }}>
                Edit Element
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default WorldBuildingManager;