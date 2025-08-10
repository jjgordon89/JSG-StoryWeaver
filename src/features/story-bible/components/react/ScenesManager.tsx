import React, { useState, useEffect, useCallback } from 'react';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { Scene, CreateSceneRequest, UpdateSceneRequest } from '../../../../types/storyBible';

import { Button } from '../../../../ui/components/common';
import { Input } from '../../../../ui/components/common';
import { Textarea } from '../../../../ui/components/common';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../ui/components/common';
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '../../../../components/ui/dialog';

import { Label } from '../../../../components/ui/label';
import { Badge } from '../../../../components/ui/badge';
import { Loader2, Plus, Search, Eye, Edit, Trash2, Check, Download } from 'lucide-react';

interface ScenesManagerProps {
  projectId: string;
  seriesId?: string;
}

interface SceneFormData {
  title: string;
  summary: string;
  scene_number: number | null;
  characters: string[];
  setting: string;
  mood: string;
  extra_instructions: string;
  word_count_estimate: number | null;
}

const initialFormData: SceneFormData = {
  title: '',
  summary: '',
  scene_number: null,
  characters: [],
  setting: '',
  mood: '',
  extra_instructions: '',
  word_count_estimate: null
};



// Scene type options
const sceneTypeOptions = [
  { value: '', label: 'Select scene type' },
  { value: 'action', label: 'Action Scene' },
  { value: 'dialogue', label: 'Dialogue Scene' },
  { value: 'exposition', label: 'Exposition' },
  { value: 'flashback', label: 'Flashback' },
  { value: 'transition', label: 'Transition' },
  { value: 'climax', label: 'Climax' },
  { value: 'character_development', label: 'Character Development' },
  { value: 'world_building', label: 'World Building' },
  { value: 'romance', label: 'Romance' },
  { value: 'mystery', label: 'Mystery/Suspense' },
  { value: 'comedy', label: 'Comedy/Humor' },
  { value: 'other', label: 'Other' }
];

// Status options
const statusOptions = [
  { value: 'planned', label: 'Planned' },
  { value: 'drafted', label: 'Drafted' },
  { value: 'revised', label: 'Revised' },
  { value: 'final', label: 'Final' }
];

const moodOptions = [
  { value: '', label: 'Select mood' },
  { value: 'tense', label: 'Tense' },
  { value: 'peaceful', label: 'Peaceful' },
  { value: 'romantic', label: 'Romantic' },
  { value: 'mysterious', label: 'Mysterious' },
  { value: 'exciting', label: 'Exciting' },
  { value: 'melancholy', label: 'Melancholy' },
  { value: 'hopeful', label: 'Hopeful' },
  { value: 'dark', label: 'Dark' },
  { value: 'humorous', label: 'Humorous' },
  { value: 'dramatic', label: 'Dramatic' },
  { value: 'contemplative', label: 'Contemplative' },
  { value: 'other', label: 'Other' }
];

// Mock characters data - in real app, this would come from characters store
const availableCharacters = [
  { id: '1', name: 'Main Character' },
  { id: '2', name: 'Antagonist' },
  { id: '3', name: 'Supporting Character' }
];



const getMoodLabel = (mood: string): string => {
  return moodOptions.find(opt => opt.value === mood)?.label || mood;
};

const getCharacterName = (characterId: string): string => {
  return availableCharacters.find(char => char.id === characterId)?.name || characterId;
};



const formatSceneReference = (scene: Scene): string => {
  const parts = [];
  if (scene.scene_number) parts.push(`Scene ${scene.scene_number}`);
  if (scene.characters && Array.isArray(scene.characters) && scene.characters.length > 0) parts.push(`Characters: ${scene.characters.map((id: string) => getCharacterName(id)).join(', ')}`);
  if (scene.setting) parts.push(`@ ${scene.setting}`);
  return parts.join(' • ');
};

export const ScenesManager: React.FC<ScenesManagerProps> = ({ projectId, seriesId }) => {
  const {
    scenes,
    isLoadingScenes,
    scenesError,
    createScene,
    updateScene,
    deleteScene,
    validateScene,
    loadScenes,
    searchScenes,
    clearError,
    generateScenes
  } = useStoryBible();

  // AI generation state
  const [, setIsGeneratingScenes] = useState(false);

  // Modal state
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [, setEditingScene] = useState<Scene | null>(null);
  const [viewingScene, setViewingScene] = useState<Scene | null>(null);

  // Form state
  const [createForm, setCreateForm] = useState<SceneFormData>(initialFormData);
  const [editForm, setEditForm] = useState<SceneFormData & { id: string }>({
    ...initialFormData,
    id: ''
  });

  // Filter state
  const [localSceneFilter, setLocalSceneFilter] = useState<any>({});

  // Search state
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    loadScenes(projectId);
  }, [projectId, seriesId, loadScenes]);

  const openCreateModal = useCallback(() => {
    setCreateForm(initialFormData);
    setShowCreateModal(true);
  }, []);

  const openEditModal = useCallback((scene: Scene) => {
    setEditingScene(scene);
    setEditForm({
      id: scene.id,
      title: scene.title || '',
      summary: scene.summary || '',
      scene_number: scene.scene_number,
      characters: typeof scene.characters === 'string' ? scene.characters.split(',').filter(Boolean) : [],
      setting: scene.setting || '',
      mood: scene.mood || '',
      extra_instructions: scene.extra_instructions || '',
      word_count_estimate: scene.word_count_estimate || null
    });
    setShowEditModal(true);
  }, []);

  const openDetailModal = useCallback((scene: Scene) => {
    setViewingScene(scene);
    setShowDetailModal(true);
  }, []);

  const closeModals = useCallback(() => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setShowDetailModal(false);
    setEditingScene(null);
    setViewingScene(null);
  }, []);

  const handleCreateScene = useCallback(async () => {
    if (!createForm.title || !createForm.summary) {
      return;
    }

    const request: CreateSceneRequest = {
        outline_id: projectId,
      title: createForm.title,
      summary: createForm.summary,
      scene_number: createForm.scene_number ?? 0,
      characters: createForm.characters.length > 0 ? createForm.characters.join(',') : undefined,
      setting: createForm.setting || undefined,
      mood: createForm.mood || undefined,
      extra_instructions: createForm.extra_instructions || undefined,
      word_count_estimate: createForm.word_count_estimate || undefined
    };

    await createScene(request);
    closeModals();
  }, [createForm, projectId, seriesId, createScene, closeModals]);

  const handleUpdateScene = useCallback(async () => {
    if (!editForm.id || !editForm.title || !editForm.summary) {
      return;
    }

    const request: UpdateSceneRequest = {
      id: editForm.id,
      title: editForm.title,
      summary: editForm.summary,
      scene_number: editForm.scene_number || undefined,
      characters: editForm.characters.length > 0 ? editForm.characters.join(',') : undefined,
      setting: editForm.setting || undefined,
      mood: editForm.mood || undefined,
      extra_instructions: editForm.extra_instructions || undefined,
      word_count_estimate: editForm.word_count_estimate || undefined
    };

    await updateScene(request);
    closeModals();
  }, [editForm, updateScene, closeModals]);

  const handleDeleteScene = useCallback(async (sceneId: string) => {
    if (window.confirm('Are you sure you want to delete this scene?')) {
      await deleteScene(sceneId);
    }
  }, [deleteScene]);

  const handleGenerateScenes = useCallback(async () => {
    if (!createForm.title) {
      return;
    }

    setIsGeneratingScenes(true);
    try {
      // Build story context from available information
      const storyContext = [
        createForm.summary && `Summary: ${createForm.summary}`,
        createForm.setting && `Setting: ${createForm.setting}`,
        createForm.mood && `Mood: ${createForm.mood}`,
        createForm.extra_instructions && `Instructions: ${createForm.extra_instructions}`
      ].filter(Boolean).join('. ');

      const response = await generateScenes({
        project_id: projectId,
        scene_type: 'scene',
        title: createForm.title,
        scene_number: createForm.scene_number ?? undefined,
        character_pov: createForm.characters.length > 0 ? createForm.characters[0] : undefined,
        mood: createForm.mood,
        // summary property not available in GenerateScenesRequest
        location: createForm.setting,
        custom_prompt: createForm.extra_instructions,
        story_context: storyContext || `A scene titled "${createForm.title}"`,
        existing_scenes: scenes.map(scene => scene.title).filter((title): title is string => title !== undefined)
      });
      
      if (response?.generated_content) {
        setCreateForm(prev => ({
          ...prev,
          extra_instructions: prev.extra_instructions + '\n\nGenerated content: ' + response.generated_content
        }));
      }
    } catch (error) {
      console.error('Failed to generate scene content:', error);
    } finally {
      setIsGeneratingScenes(false);
    }
  }, [createForm, projectId, generateScenes, scenes]);

  // Suppress unused variable warning
  void handleGenerateScenes;

  const handleValidateScene = useCallback(async (sceneId: string) => {
    await validateScene(sceneId);
  }, [validateScene]);

  const handleSearch = useCallback(async () => {
    if (searchQuery.trim()) {
      await searchScenes({ outline_id: projectId, query: searchQuery });
    } else {
      await loadScenes(projectId);
    }
  }, [searchQuery, projectId, searchScenes, loadScenes]);

  const handleExportCSV = useCallback(() => {
    if (scenes.length === 0) return;
    
    // Create CSV content
    const headers = [
      'Title', 'Summary', 'Setting', 'Scene Number', 'Character POV', 
      'Mood', 'Word Count Target', 'Extra Instructions'
    ];
    
    const rows = scenes.map(scene => [
      scene.title,
      scene.summary || '',
      scene.setting || '',
      scene.scene_number || '',
      scene.characters && Array.isArray(scene.characters) && scene.characters.length > 0 ? scene.characters.map((id: string) => getCharacterName(id)).join(', ') : '',
      getMoodLabel(scene.mood || ''),
      scene.word_count_estimate || '',
      scene.extra_instructions ? scene.extra_instructions.replace(/"/g, '""') : '' // Escape quotes
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
    link.setAttribute('download', `scenes_${new Date().toISOString().split('T')[0]}.csv`);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  }, [scenes]);

  const handleFilterChange = useCallback((filterType: string, value: any) => {
    setLocalSceneFilter({
      ...localSceneFilter,
      [filterType]: value || undefined
    });
  }, [localSceneFilter]);

  const updateCreateForm = useCallback((field: keyof SceneFormData, value: any) => {
    setCreateForm(prev => ({ ...prev, [field]: value }));
  }, []);

  const updateEditForm = useCallback((field: keyof SceneFormData, value: any) => {
    setEditForm(prev => ({ ...prev, [field]: value }));
  }, []);

  return (
    <div className="flex flex-col h-full bg-background">
      {/* Header */}
      <div className="flex justify-between items-start p-8 border-b bg-card">
        <div>
          <h2 className="text-2xl font-semibold mb-2">Story Scenes</h2>
          <p className="text-muted-foreground text-sm">
            Plan and track individual scenes with detailed breakdowns, character POVs, and story progression.
          </p>
        </div>
        <div className="flex gap-2">
          <Button
            onClick={handleExportCSV}
            variant="outline"
            disabled={scenes.length === 0}
          >
            <Download className="w-4 h-4 mr-2" />
            Export CSV
          </Button>
          <Button onClick={openCreateModal}>
            <Plus className="w-4 h-4 mr-2" />
            Add Scene
          </Button>
        </div>
      </div>

      {/* Search and Filters */}
      <Card className="m-8 mb-4">
        <CardHeader>
          <CardTitle>Search & Filter</CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="flex gap-3">
            <Input
              placeholder="Search scenes..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
              className="flex-1"
            />
            <Button variant="secondary" onClick={handleSearch}>
              <Search className="w-4 h-4 mr-2" />
              Search
            </Button>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="space-y-2">
              <Label>Filter by Type:</Label>
              <Select
                value={localSceneFilter.sceneType || ''}
                onValueChange={(value) => handleFilterChange('sceneType', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All types" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All types</SelectItem>
                  {sceneTypeOptions.slice(1).map(option => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Filter by Status:</Label>
              <Select
                value={localSceneFilter.status || ''}
                onValueChange={(value) => handleFilterChange('status', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All statuses" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All statuses</SelectItem>
                  {statusOptions.map(option => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Filter by Character POV:</Label>
              <Select
                value={localSceneFilter.characterPov || ''}
                onValueChange={(value) => handleFilterChange('characterPov', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="All characters" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">All characters</SelectItem>
                  {availableCharacters.map(char => (
                    <SelectItem key={char.id} value={char.id}>
                      {char.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Filter by Chapter:</Label>
              <Input
                type="number"
                placeholder="Chapter #"
                value={localSceneFilter.chapterNumber || ''}
                onChange={(e) => handleFilterChange('chapterNumber', e.target.value ? parseInt(e.target.value) : undefined)}
              />
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Content Area */}
      <div className="flex-1 overflow-y-auto p-8 pt-0">
        {scenesError && (
          <div className="mb-6 p-4 bg-destructive/10 border border-destructive/20 rounded-lg">
            <p className="text-destructive">{scenesError}</p>
            <Button variant="outline" size="sm" onClick={clearError} className="mt-2">
              Dismiss
            </Button>
          </div>
        )}

        {isLoadingScenes ? (
          <div className="flex flex-col items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin mb-4" />
            <p className="text-muted-foreground">Loading scenes...</p>
          </div>
        ) : scenes.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12">
            <div className="text-6xl mb-4">🎬</div>
            <h3 className="text-xl font-semibold mb-2">No Scenes</h3>
            <p className="text-muted-foreground mb-6 text-center max-w-md">
              Start building your story by creating detailed scene breakdowns with character POVs, locations, and story progression.
            </p>
            <Button onClick={openCreateModal}>
              Create First Scene
            </Button>
          </div>
        ) : (
          <div className="space-y-6">
            {scenes.map((scene) => (
              <Card key={scene.id} className="overflow-hidden">
                <CardContent className="p-0">
                  <div className="p-6">
                    <div className="flex justify-between items-start mb-4">
                      <div className="flex-1">
                        <div className="flex items-start gap-3 mb-3">
                          <span className="text-2xl">🎬</span>
                          <div className="flex-1">
                            <h4 className="text-lg font-semibold mb-1">{scene.title}</h4>
                            {formatSceneReference(scene) && (
                              <div className="text-sm text-muted-foreground italic">
                                {formatSceneReference(scene)}
                              </div>
                            )}
                          </div>
                        </div>
                        <div className="flex flex-wrap gap-2 mb-4">
                          {scene.mood && (
                            <Badge variant="outline">{getMoodLabel(scene.mood)}</Badge>
                          )}
                          {scene.word_count_estimate && (
                            <Badge variant="secondary">{scene.word_count_estimate} words</Badge>
                          )}
                        </div>
                      </div>
                      
                      <div className="flex gap-1">
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => handleValidateScene(scene.id)}
                        >
                          <Check className="w-4 h-4" />
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => openDetailModal(scene)}
                        >
                          <Eye className="w-4 h-4" />
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => openEditModal(scene)}
                        >
                          <Edit className="w-4 h-4" />
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => handleDeleteScene(scene.id)}
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </div>
                    </div>
                    
                    <div className="space-y-4">
                      <p className="text-sm text-muted-foreground">
                        {scene.summary ? scene.summary.substring(0, 200) + (scene.summary.length > 200 ? '...' : '') : 'No summary available'}
                      </p>
                      
                      {(scene.summary || scene.setting) && (
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                          {scene.summary && (
                            <div>
                              <strong className="text-foreground">Summary:</strong>
                              <p className="text-muted-foreground">{scene.summary}</p>
                            </div>
                          )}
                          {scene.setting && (
                            <div>
                              <strong className="text-foreground">Setting:</strong>
                              <p className="text-muted-foreground">{scene.setting}</p>
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                    
                    <div className="flex justify-between items-center mt-4 pt-4 border-t text-sm text-muted-foreground">
                      <div className="flex gap-4">
                        {scene.word_count_estimate && (
          <span>Target: {scene.word_count_estimate} words</span>
                        )}
                      </div>
                      <span>Updated {new Date(scene.updated_at).toLocaleDateString()}</span>
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Create Scene Modal */}
      <Dialog open={showCreateModal} onOpenChange={setShowCreateModal}>
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Add Scene</DialogTitle>
          </DialogHeader>
          
          <div className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="create-title">Title:</Label>
              <Input
                id="create-title"
                value={createForm.title}
                onChange={(e) => updateCreateForm('title', e.target.value)}
                placeholder="Enter scene title..."
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="create-summary">Summary:</Label>
              <Textarea
                id="create-summary"
                value={createForm.summary}
                onChange={(e) => updateCreateForm('summary', e.target.value)}
                placeholder="Brief summary of what happens in this scene..."
                rows={3}
              />
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-setting">Setting:</Label>
                <Input
                  id="create-setting"
                  value={createForm.setting}
                  onChange={(e) => updateCreateForm('setting', e.target.value)}
                  placeholder="Where does this scene take place?"
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="create-scene-number">Scene Number:</Label>
                <Input
                  id="create-scene-number"
                  type="number"
                  value={createForm.scene_number || ''}
                  onChange={(e) => updateCreateForm('scene_number', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Scene #"
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="create-word-target">Word Count Target:</Label>
                <Input
                  id="create-word-target"
                  type="number"
                  value={createForm.word_count_estimate || ''}
          onChange={(e) => updateCreateForm('word_count_estimate', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Target words"
                />
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-characters">Characters:</Label>
                <Select
                  value={createForm.characters.length > 0 ? createForm.characters[0] : ""}
                  onValueChange={(value) => updateCreateForm('characters', value ? [value] : [])}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="No characters selected" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">No characters selected</SelectItem>
                    {availableCharacters.map(char => (
                      <SelectItem key={char.id} value={char.id}>
                        {char.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="create-mood">Mood:</Label>
                <Select
                  value={createForm.mood}
                  onValueChange={(value) => updateCreateForm('mood', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select mood" />
                  </SelectTrigger>
                  <SelectContent>
                    {moodOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="create-extra-instructions">Extra Instructions:</Label>
              <Textarea
                id="create-extra-instructions"
                value={createForm.extra_instructions}
                onChange={(e) => updateCreateForm('extra_instructions', e.target.value)}
                placeholder="Additional notes, instructions, or details for this scene..."
                rows={6}
              />
            </div>
          </div>
          
          <DialogFooter>
            <Button variant="outline" onClick={closeModals}>
              Cancel
            </Button>
            <Button 
              onClick={handleCreateScene}
              disabled={!createForm.title}
            >
              Add Scene
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Edit Scene Modal */}
      <Dialog open={showEditModal} onOpenChange={setShowEditModal}>
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>Edit Scene</DialogTitle>
          </DialogHeader>
          
          <div className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="edit-title">Title:</Label>
              <Input
                id="edit-title"
                value={editForm.title}
                onChange={(e) => updateEditForm('title', e.target.value)}
                placeholder="Enter scene title..."
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-summary">Summary:</Label>
              <Textarea
                id="edit-summary"
                value={editForm.summary}
                onChange={(e) => updateEditForm('summary', e.target.value)}
                placeholder="Brief summary of the scene..."
                rows={3}
              />
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="edit-setting">Setting:</Label>
                <Input
                  id="edit-setting"
                  value={editForm.setting}
                  onChange={(e) => updateEditForm('setting', e.target.value)}
                  placeholder="Scene setting/location"
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="edit-scene-number">Scene Number:</Label>
                <Input
                  id="edit-scene-number"
                  type="number"
                  value={editForm.scene_number || ''}
                  onChange={(e) => updateEditForm('scene_number', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Scene #"
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="edit-word-target">Word Count Target:</Label>
                <Input
                  id="edit-word-target"
                  type="number"
                  value={editForm.word_count_estimate || ''}
                  onChange={(e) => updateEditForm('word_count_estimate', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Target words"
                />
              </div>
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-characters">Characters:</Label>
              <Select
                value={editForm.characters.length > 0 ? editForm.characters[0] : ""}
                onValueChange={(value) => updateEditForm('characters', value ? [value] : [])}
              >
                <SelectTrigger>
                  <SelectValue placeholder="No characters selected" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">No characters selected</SelectItem>
                  {availableCharacters.map(char => (
                    <SelectItem key={char.id} value={char.id}>
                      {char.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-mood">Mood:</Label>
              <Select
                value={editForm.mood}
                onValueChange={(value) => updateEditForm('mood', value)}
              >
                <SelectTrigger>
                  <SelectValue placeholder="Select mood" />
                </SelectTrigger>
                <SelectContent>
                  {moodOptions.map(option => (
                    <SelectItem key={option.value} value={option.value}>
                      {option.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-extra-instructions">Extra Instructions:</Label>
              <Textarea
                id="edit-extra-instructions"
                value={editForm.extra_instructions}
                onChange={(e) => updateEditForm('extra_instructions', e.target.value)}
                placeholder="Additional notes, instructions, or details for this scene..."
                rows={6}
              />
            </div>
            

          </div>
          
          <DialogFooter>
            <Button variant="outline" onClick={closeModals}>
              Cancel
            </Button>
            <Button 
              onClick={handleUpdateScene}
              disabled={!editForm.title}
            >
              Save Changes
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      {/* Detail View Modal */}
      <Dialog open={showDetailModal} onOpenChange={setShowDetailModal}>
        <DialogContent className="max-w-4xl max-h-[90vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{viewingScene?.title || 'Scene Details'}</DialogTitle>
          </DialogHeader>
          
          {viewingScene && (
            <div className="space-y-6">
              <div className="flex justify-between items-start">
                <div className="flex items-start gap-3">
                  <span className="text-3xl">🎬</span>
                  <div>
                    <h3 className="text-xl font-semibold">{viewingScene.title}</h3>
                    <div className="flex flex-wrap gap-2 mt-2">
                      {formatSceneReference(viewingScene) && (
                        <Badge variant="outline">{formatSceneReference(viewingScene)}</Badge>
                      )}
                    </div>
                  </div>
                </div>
                
                <div className="flex flex-wrap gap-2">
                  {viewingScene.mood && (
                    <Badge variant="outline">{getMoodLabel(viewingScene.mood)}</Badge>
                  )}
                </div>
              </div>
              
              {viewingScene.extra_instructions && (
                <div>
                  <h4 className="text-lg font-semibold mb-3">Extra Instructions</h4>
                  <div className="grid grid-cols-1 gap-4">
                    <div>
                      <p className="text-muted-foreground">{viewingScene.extra_instructions}</p>
                    </div>
                  </div>
                </div>
              )}
              

              
              <div>
                <h4 className="text-lg font-semibold mb-3">Scene Details</h4>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                  {viewingScene.setting && (
                    <div>
                      <strong className="text-foreground">Setting:</strong>
                      <span className="ml-2 text-muted-foreground">{viewingScene.setting}</span>
                    </div>
                  )}
                  {viewingScene.characters && (
                    <div>
                      <strong className="text-foreground">Characters:</strong>
                      <span className="ml-2 text-muted-foreground">{viewingScene.characters}</span>
                    </div>
                  )}
                  {viewingScene.word_count_estimate && (
                    <div>
                      <strong className="text-foreground">Target Word Count:</strong>
                      <span className="ml-2 text-muted-foreground">{viewingScene.word_count_estimate}</span>
                    </div>
                  )}
                  <div>
                    <strong className="text-foreground">Created:</strong>
                    <span className="ml-2 text-muted-foreground">{new Date(viewingScene.created_at).toLocaleDateString()}</span>
                  </div>
                  <div>
                    <strong className="text-foreground">Last Updated:</strong>
                    <span className="ml-2 text-muted-foreground">{new Date(viewingScene.updated_at).toLocaleDateString()}</span>
                  </div>
                </div>
              </div>
            </div>
          )}
          
          <DialogFooter>
            <Button variant="outline" onClick={closeModals}>
              Close
            </Button>
            {viewingScene && (
              <Button onClick={() => { closeModals(); openEditModal(viewingScene); }}>
                Edit Scene
              </Button>
            )}
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ScenesManager;