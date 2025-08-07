import React, { useState, useEffect, useCallback } from 'react';
import { useStoryBible } from '../../hooks/useStoryBible';
import type { Scene, CreateSceneRequest, UpdateSceneRequest } from '../../../../types/storyBible';

import { Button } from '../../../../components/ui/button';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/card';
import { Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle } from '../../../../components/ui/dialog';
import { Checkbox } from '../../../../components/ui/checkbox';
import { Label } from '../../../../components/ui/label';
import { Badge } from '../../../../components/ui/badge';
import { Loader2, Plus, Search, Eye, Edit, Trash2, Check } from 'lucide-react';

interface ScenesManagerProps {
  projectId: string;
  seriesId?: string;
}

interface SceneFormData {
  title: string;
  content: string;
  scene_type: string;
  chapter_number: number | null;
  scene_number: number | null;
  character_pov: string;
  location: string;
  time_of_day: string;
  mood: string;
  purpose: string;
  conflict: string;
  outcome: string;
  notes: string;
  word_count_target: number | null;
  status: 'planned' | 'drafted' | 'revised' | 'final';
  visibility: 'always' | 'chapter' | 'never';
  series_shared: boolean;
}

const initialFormData: SceneFormData = {
  title: '',
  content: '',
  scene_type: '',
  chapter_number: null,
  scene_number: null,
  character_pov: '',
  location: '',
  time_of_day: '',
  mood: '',
  purpose: '',
  conflict: '',
  outcome: '',
  notes: '',
  word_count_target: null,
  status: 'planned',
  visibility: 'always',
  series_shared: false
};

const sceneTypeOptions = [
  { value: '', label: 'Select scene type' },
  { value: 'action', label: 'Action Scene' },
  { value: 'dialogue', label: 'Dialogue Scene' },
  { value: 'exposition', label: 'Exposition' },
  { value: 'flashback', label: 'Flashback' },
  { value: 'transition', label: 'Transition' },
  { value: 'climax', label: 'Climax' },
  { value: 'resolution', label: 'Resolution' },
  { value: 'character_development', label: 'Character Development' },
  { value: 'world_building', label: 'World Building' },
  { value: 'romance', label: 'Romance' },
  { value: 'mystery', label: 'Mystery/Suspense' },
  { value: 'comedy', label: 'Comedy/Humor' },
  { value: 'other', label: 'Other' }
];

const statusOptions = [
  { value: 'planned', label: 'Planned' },
  { value: 'drafted', label: 'Drafted' },
  { value: 'revised', label: 'Revised' },
  { value: 'final', label: 'Final' }
];

const visibilityOptions = [
  { value: 'always', label: 'Always Visible' },
  { value: 'chapter', label: 'Chapter Context' },
  { value: 'never', label: 'Hidden' }
];

const timeOfDayOptions = [
  { value: '', label: 'Select time' },
  { value: 'dawn', label: 'Dawn' },
  { value: 'morning', label: 'Morning' },
  { value: 'midday', label: 'Midday' },
  { value: 'afternoon', label: 'Afternoon' },
  { value: 'evening', label: 'Evening' },
  { value: 'night', label: 'Night' },
  { value: 'midnight', label: 'Midnight' },
  { value: 'unspecified', label: 'Unspecified' }
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

const getSceneTypeLabel = (sceneType: string): string => {
  return sceneTypeOptions.find(opt => opt.value === sceneType)?.label || sceneType;
};

const getStatusLabel = (status: string): string => {
  return statusOptions.find(opt => opt.value === status)?.label || status;
};

const getVisibilityLabel = (visibility: string): string => {
  return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
};

const getTimeOfDayLabel = (timeOfDay: string): string => {
  return timeOfDayOptions.find(opt => opt.value === timeOfDay)?.label || timeOfDay;
};

const getMoodLabel = (mood: string): string => {
  return moodOptions.find(opt => opt.value === mood)?.label || mood;
};

const getCharacterName = (characterId: string): string => {
  return availableCharacters.find(char => char.id === characterId)?.name || characterId;
};

const getSceneIcon = (sceneType: string): string => {
  const icons: Record<string, string> = {
    action: '⚔️',
    dialogue: '💬',
    exposition: '📖',
    flashback: '⏪',
    transition: '🔄',
    climax: '🎯',
    resolution: '✅',
    character_development: '👤',
    world_building: '🌍',
    romance: '💕',
    mystery: '🔍',
    comedy: '😄',
    other: '🎬'
  };
  return icons[sceneType] || '🎬';
};

const getStatusColor = (status: string): string => {
  const colors: Record<string, string> = {
    planned: '#6c757d',
    drafted: '#ffc107',
    revised: '#17a2b8',
    final: '#28a745'
  };
  return colors[status] || '#6c757d';
};

const formatSceneReference = (scene: Scene): string => {
  const parts = [];
  if (scene.chapter_number) parts.push(`Ch. ${scene.chapter_number}`);
  if (scene.scene_number) parts.push(`Scene ${scene.scene_number}`);
  if (scene.character_pov) parts.push(`POV: ${getCharacterName(scene.character_pov)}`);
  if (scene.location) parts.push(`@ ${scene.location}`);
  return parts.join(' • ');
};

export const ScenesManager: React.FC<ScenesManagerProps> = ({ projectId, seriesId }) => {
  const {
    scenes,
    isLoadingScenes,
    scenesError,
    sceneFilter,
    createScene,
    updateScene,
    deleteScene,
    validateScene,
    loadScenes,
    searchScenes,
    setSceneFilter,
    clearError
  } = useStoryBible();

  // Modal state
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [showDetailModal, setShowDetailModal] = useState(false);
  const [editingScene, setEditingScene] = useState<Scene | null>(null);
  const [viewingScene, setViewingScene] = useState<Scene | null>(null);

  // Form state
  const [createForm, setCreateForm] = useState<SceneFormData>(initialFormData);
  const [editForm, setEditForm] = useState<SceneFormData & { id: string }>({
    ...initialFormData,
    id: ''
  });

  // Search state
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    loadScenes(projectId, seriesId);
  }, [projectId, seriesId, loadScenes]);

  const openCreateModal = useCallback(() => {
    setCreateForm(initialFormData);
    setShowCreateModal(true);
  }, []);

  const openEditModal = useCallback((scene: Scene) => {
    setEditingScene(scene);
    setEditForm({
      id: scene.id,
      title: scene.title,
      content: scene.content,
      scene_type: scene.scene_type,
      chapter_number: scene.chapter_number,
      scene_number: scene.scene_number,
      character_pov: scene.character_pov || '',
      location: scene.location || '',
      time_of_day: scene.time_of_day || '',
      mood: scene.mood || '',
      purpose: scene.purpose || '',
      conflict: scene.conflict || '',
      outcome: scene.outcome || '',
      notes: scene.notes || '',
      word_count_target: scene.word_count_target,
      status: scene.status,
      visibility: scene.visibility,
      series_shared: scene.series_shared
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
    if (!createForm.title || !createForm.content || !createForm.scene_type) {
      return;
    }

    const request: CreateSceneRequest = {
      project_id: projectId,
      series_id: seriesId,
      title: createForm.title,
      content: createForm.content,
      scene_type: createForm.scene_type,
      chapter_number: createForm.chapter_number,
      scene_number: createForm.scene_number,
      character_pov: createForm.character_pov || undefined,
      location: createForm.location || undefined,
      time_of_day: createForm.time_of_day || undefined,
      mood: createForm.mood || undefined,
      purpose: createForm.purpose || undefined,
      conflict: createForm.conflict || undefined,
      outcome: createForm.outcome || undefined,
      notes: createForm.notes || undefined,
      word_count_target: createForm.word_count_target,
      status: createForm.status,
      visibility: createForm.visibility,
      series_shared: createForm.series_shared
    };

    await createScene(request);
    closeModals();
  }, [createForm, projectId, seriesId, createScene, closeModals]);

  const handleUpdateScene = useCallback(async () => {
    if (!editForm.id || !editForm.title || !editForm.content || !editForm.scene_type) {
      return;
    }

    const request: UpdateSceneRequest = {
      id: editForm.id,
      title: editForm.title,
      content: editForm.content,
      scene_type: editForm.scene_type,
      chapter_number: editForm.chapter_number,
      scene_number: editForm.scene_number,
      character_pov: editForm.character_pov || undefined,
      location: editForm.location || undefined,
      time_of_day: editForm.time_of_day || undefined,
      mood: editForm.mood || undefined,
      purpose: editForm.purpose || undefined,
      conflict: editForm.conflict || undefined,
      outcome: editForm.outcome || undefined,
      notes: editForm.notes || undefined,
      word_count_target: editForm.word_count_target,
      status: editForm.status,
      visibility: editForm.visibility,
      series_shared: editForm.series_shared
    };

    await updateScene(request);
    closeModals();
  }, [editForm, updateScene, closeModals]);

  const handleDeleteScene = useCallback(async (sceneId: string) => {
    if (window.confirm('Are you sure you want to delete this scene?')) {
      await deleteScene(sceneId);
    }
  }, [deleteScene]);

  const handleValidateScene = useCallback(async (sceneId: string) => {
    await validateScene(sceneId);
  }, [validateScene]);

  const handleSearch = useCallback(async () => {
    if (searchQuery.trim()) {
      await searchScenes(projectId, searchQuery, seriesId);
    } else {
      await loadScenes(projectId, seriesId);
    }
  }, [searchQuery, projectId, seriesId, searchScenes, loadScenes]);

  const handleFilterChange = useCallback((filterType: string, value: any) => {
    setSceneFilter({
      ...sceneFilter,
      [filterType]: value || undefined
    });
  }, [sceneFilter, setSceneFilter]);

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
        <Button onClick={openCreateModal}>
          <Plus className="w-4 h-4 mr-2" />
          Add Scene
        </Button>
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
                value={sceneFilter.sceneType || ''}
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
                value={sceneFilter.status || ''}
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
                value={sceneFilter.characterPov || ''}
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
                value={sceneFilter.chapterNumber || ''}
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
                          <span className="text-2xl">{getSceneIcon(scene.scene_type)}</span>
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
                          <Badge variant="secondary">{getSceneTypeLabel(scene.scene_type)}</Badge>
                          <Badge 
                            style={{ 
                              backgroundColor: `${getStatusColor(scene.status)}20`, 
                              color: getStatusColor(scene.status),
                              border: `1px solid ${getStatusColor(scene.status)}40`
                            }}
                          >
                            {getStatusLabel(scene.status)}
                          </Badge>
                          {scene.mood && (
                            <Badge variant="outline">{getMoodLabel(scene.mood)}</Badge>
                          )}
                          {scene.time_of_day && (
                            <Badge variant="outline">{getTimeOfDayLabel(scene.time_of_day)}</Badge>
                          )}
                          {scene.series_shared && (
                            <Badge>Series Shared</Badge>
                          )}
                        </div>
                      </div>
                      
                      <div className="flex gap-1">
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => handleValidateScene(scene.id)}
                          title="Validate Scene"
                        >
                          <Check className="w-4 h-4" />
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => openDetailModal(scene)}
                          title="View Details"
                        >
                          <Eye className="w-4 h-4" />
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => openEditModal(scene)}
                          title="Edit"
                        >
                          <Edit className="w-4 h-4" />
                        </Button>
                        <Button 
                          variant="ghost" 
                          size="sm"
                          onClick={() => handleDeleteScene(scene.id)}
                          title="Delete"
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </div>
                    </div>
                    
                    <div className="space-y-4">
                      <p className="text-sm text-muted-foreground">
                        {scene.content.substring(0, 200)}{scene.content.length > 200 ? '...' : ''}
                      </p>
                      
                      {(scene.purpose || scene.conflict || scene.outcome) && (
                        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                          {scene.purpose && (
                            <div>
                              <strong className="text-foreground">Purpose:</strong>
                              <p className="text-muted-foreground">{scene.purpose}</p>
                            </div>
                          )}
                          {scene.conflict && (
                            <div>
                              <strong className="text-foreground">Conflict:</strong>
                              <p className="text-muted-foreground">{scene.conflict}</p>
                            </div>
                          )}
                          {scene.outcome && (
                            <div>
                              <strong className="text-foreground">Outcome:</strong>
                              <p className="text-muted-foreground">{scene.outcome}</p>
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                    
                    <div className="flex justify-between items-center mt-4 pt-4 border-t text-sm text-muted-foreground">
                      <div className="flex gap-4">
                        {scene.word_count_target && (
                          <span>Target: {scene.word_count_target} words</span>
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
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-scene-type">Scene Type:</Label>
                <Select
                  value={createForm.scene_type}
                  onValueChange={(value) => updateCreateForm('scene_type', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select scene type" />
                  </SelectTrigger>
                  <SelectContent>
                    {sceneTypeOptions.map(option => (
                      <SelectItem key={option.value} value={option.value} disabled={!option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="create-status">Status:</Label>
                <Select
                  value={createForm.status}
                  onValueChange={(value) => updateCreateForm('status', value as any)}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {statusOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-chapter">Chapter Number:</Label>
                <Input
                  id="create-chapter"
                  type="number"
                  value={createForm.chapter_number || ''}
                  onChange={(e) => updateCreateForm('chapter_number', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Chapter #"
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
                  value={createForm.word_count_target || ''}
                  onChange={(e) => updateCreateForm('word_count_target', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Target words"
                />
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-character-pov">Character POV:</Label>
                <Select
                  value={createForm.character_pov}
                  onValueChange={(value) => updateCreateForm('character_pov', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="No specific POV" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">No specific POV</SelectItem>
                    {availableCharacters.map(char => (
                      <SelectItem key={char.id} value={char.id}>
                        {char.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="create-location">Location:</Label>
                <Input
                  id="create-location"
                  value={createForm.location}
                  onChange={(e) => updateCreateForm('location', e.target.value)}
                  placeholder="Scene location..."
                />
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-time">Time of Day:</Label>
                <Select
                  value={createForm.time_of_day}
                  onValueChange={(value) => updateCreateForm('time_of_day', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select time" />
                  </SelectTrigger>
                  <SelectContent>
                    {timeOfDayOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
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
              <Label htmlFor="create-purpose">Scene Purpose:</Label>
              <Input
                id="create-purpose"
                value={createForm.purpose}
                onChange={(e) => updateCreateForm('purpose', e.target.value)}
                placeholder="What does this scene accomplish?"
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="create-conflict">Conflict:</Label>
              <Input
                id="create-conflict"
                value={createForm.conflict}
                onChange={(e) => updateCreateForm('conflict', e.target.value)}
                placeholder="What conflict drives this scene?"
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="create-outcome">Outcome:</Label>
              <Input
                id="create-outcome"
                value={createForm.outcome}
                onChange={(e) => updateCreateForm('outcome', e.target.value)}
                placeholder="How does the scene end?"
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="create-content">Scene Content:</Label>
              <Textarea
                id="create-content"
                value={createForm.content}
                onChange={(e) => updateCreateForm('content', e.target.value)}
                placeholder="Write your scene content or detailed breakdown..."
                rows={6}
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="create-notes">Notes:</Label>
              <Textarea
                id="create-notes"
                value={createForm.notes}
                onChange={(e) => updateCreateForm('notes', e.target.value)}
                placeholder="Additional notes or reminders..."
                rows={3}
              />
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="create-visibility">Visibility:</Label>
                <Select
                  value={createForm.visibility}
                  onValueChange={(value) => updateCreateForm('visibility', value as any)}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {visibilityOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="flex items-center space-x-2 pt-6">
                <Checkbox
                  id="create-series-shared"
                  checked={createForm.series_shared}
                  onCheckedChange={(checked) => updateCreateForm('series_shared', checked)}
                />
                <Label htmlFor="create-series-shared">Share across series</Label>
              </div>
            </div>
          </div>
          
          <DialogFooter>
            <Button variant="outline" onClick={closeModals}>
              Cancel
            </Button>
            <Button 
              onClick={handleCreateScene}
              disabled={!createForm.title || !createForm.scene_type || !createForm.content}
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
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="edit-scene-type">Scene Type:</Label>
                <Select
                  value={editForm.scene_type}
                  onValueChange={(value) => updateEditForm('scene_type', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select scene type" />
                  </SelectTrigger>
                  <SelectContent>
                    {sceneTypeOptions.map(option => (
                      <SelectItem key={option.value} value={option.value} disabled={!option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="edit-status">Status:</Label>
                <Select
                  value={editForm.status}
                  onValueChange={(value) => updateEditForm('status', value as any)}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {statusOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="edit-chapter">Chapter Number:</Label>
                <Input
                  id="edit-chapter"
                  type="number"
                  value={editForm.chapter_number || ''}
                  onChange={(e) => updateEditForm('chapter_number', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Chapter #"
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
                  value={editForm.word_count_target || ''}
                  onChange={(e) => updateEditForm('word_count_target', e.target.value ? parseInt(e.target.value) : null)}
                  placeholder="Target words"
                />
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="edit-character-pov">Character POV:</Label>
                <Select
                  value={editForm.character_pov}
                  onValueChange={(value) => updateEditForm('character_pov', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="No specific POV" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="">No specific POV</SelectItem>
                    {availableCharacters.map(char => (
                      <SelectItem key={char.id} value={char.id}>
                        {char.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="edit-location">Location:</Label>
                <Input
                  id="edit-location"
                  value={editForm.location}
                  onChange={(e) => updateEditForm('location', e.target.value)}
                  placeholder="Scene location..."
                />
              </div>
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="edit-time">Time of Day:</Label>
                <Select
                  value={editForm.time_of_day}
                  onValueChange={(value) => updateEditForm('time_of_day', value)}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select time" />
                  </SelectTrigger>
                  <SelectContent>
                    {timeOfDayOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
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
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-purpose">Scene Purpose:</Label>
              <Input
                id="edit-purpose"
                value={editForm.purpose}
                onChange={(e) => updateEditForm('purpose', e.target.value)}
                placeholder="What does this scene accomplish?"
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-conflict">Conflict:</Label>
              <Input
                id="edit-conflict"
                value={editForm.conflict}
                onChange={(e) => updateEditForm('conflict', e.target.value)}
                placeholder="What conflict drives this scene?"
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-outcome">Outcome:</Label>
              <Input
                id="edit-outcome"
                value={editForm.outcome}
                onChange={(e) => updateEditForm('outcome', e.target.value)}
                placeholder="How does the scene end?"
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-content">Scene Content:</Label>
              <Textarea
                id="edit-content"
                value={editForm.content}
                onChange={(e) => updateEditForm('content', e.target.value)}
                placeholder="Write your scene content or detailed breakdown..."
                rows={6}
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="edit-notes">Notes:</Label>
              <Textarea
                id="edit-notes"
                value={editForm.notes}
                onChange={(e) => updateEditForm('notes', e.target.value)}
                placeholder="Additional notes or reminders..."
                rows={3}
              />
            </div>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="edit-visibility">Visibility:</Label>
                <Select
                  value={editForm.visibility}
                  onValueChange={(value) => updateEditForm('visibility', value as any)}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {visibilityOptions.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="flex items-center space-x-2 pt-6">
                <Checkbox
                  id="edit-series-shared"
                  checked={editForm.series_shared}
                  onCheckedChange={(checked) => updateEditForm('series_shared', checked)}
                />
                <Label htmlFor="edit-series-shared">Share across series</Label>
              </div>
            </div>
          </div>
          
          <DialogFooter>
            <Button variant="outline" onClick={closeModals}>
              Cancel
            </Button>
            <Button 
              onClick={handleUpdateScene}
              disabled={!editForm.title || !editForm.scene_type || !editForm.content}
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
                  <span className="text-3xl">{getSceneIcon(viewingScene.scene_type)}</span>
                  <div>
                    <h3 className="text-xl font-semibold">{viewingScene.title}</h3>
                    <div className="flex flex-wrap gap-2 mt-2">
                      <Badge variant="secondary">{getSceneTypeLabel(viewingScene.scene_type)}</Badge>
                      {formatSceneReference(viewingScene) && (
                        <Badge variant="outline">{formatSceneReference(viewingScene)}</Badge>
                      )}
                    </div>
                  </div>
                </div>
                
                <div className="flex flex-wrap gap-2">
                  <Badge 
                    style={{ 
                      backgroundColor: `${getStatusColor(viewingScene.status)}20`, 
                      color: getStatusColor(viewingScene.status),
                      border: `1px solid ${getStatusColor(viewingScene.status)}40`
                    }}
                  >
                    {getStatusLabel(viewingScene.status)}
                  </Badge>
                  {viewingScene.mood && (
                    <Badge variant="outline">{getMoodLabel(viewingScene.mood)}</Badge>
                  )}
                  {viewingScene.series_shared && (
                    <Badge>Series Shared</Badge>
                  )}
                </div>
              </div>
              
              {(viewingScene.purpose || viewingScene.conflict || viewingScene.outcome) && (
                <div>
                  <h4 className="text-lg font-semibold mb-3">Scene Structure</h4>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    {viewingScene.purpose && (
                      <div>
                        <strong className="text-foreground">Purpose:</strong>
                        <p className="text-muted-foreground mt-1">{viewingScene.purpose}</p>
                      </div>
                    )}
                    {viewingScene.conflict && (
                      <div>
                        <strong className="text-foreground">Conflict:</strong>
                        <p className="text-muted-foreground mt-1">{viewingScene.conflict}</p>
                      </div>
                    )}
                    {viewingScene.outcome && (
                      <div>
                        <strong className="text-foreground">Outcome:</strong>
                        <p className="text-muted-foreground mt-1">{viewingScene.outcome}</p>
                      </div>
                    )}
                  </div>
                </div>
              )}
              
              <div>
                <h4 className="text-lg font-semibold mb-3">Scene Content</h4>
                <div className="p-4 bg-muted rounded-lg">
                  <p className="whitespace-pre-wrap">{viewingScene.content}</p>
                </div>
              </div>
              
              {viewingScene.notes && (
                <div>
                  <h4 className="text-lg font-semibold mb-3">Notes</h4>
                  <div className="p-4 bg-muted rounded-lg">
                    <p className="whitespace-pre-wrap">{viewingScene.notes}</p>
                  </div>
                </div>
              )}
              
              <div>
                <h4 className="text-lg font-semibold mb-3">Scene Details</h4>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                  {viewingScene.location && (
                    <div>
                      <strong className="text-foreground">Location:</strong>
                      <span className="ml-2 text-muted-foreground">{viewingScene.location}</span>
                    </div>
                  )}
                  {viewingScene.time_of_day && (
                    <div>
                      <strong className="text-foreground">Time:</strong>
                      <span className="ml-2 text-muted-foreground">{getTimeOfDayLabel(viewingScene.time_of_day)}</span>
                    </div>
                  )}
                  {viewingScene.character_pov && (
                    <div>
                      <strong className="text-foreground">POV Character:</strong>
                      <span className="ml-2 text-muted-foreground">{getCharacterName(viewingScene.character_pov)}</span>
                    </div>
                  )}
                  {viewingScene.word_count_target && (
                    <div>
                      <strong className="text-foreground">Target Word Count:</strong>
                      <span className="ml-2 text-muted-foreground">{viewingScene.word_count_target}</span>
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