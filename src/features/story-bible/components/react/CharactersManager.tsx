import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Sparkles, Loader2 } from 'lucide-react';
import type { CharacterTrait, CharactersManagerProps } from '../../../../types/storyBible';
import useStoryBible from '../../hooks/useStoryBible';

interface CreateTraitForm {
  traitType: string;
  content: string;
  visibility: 'public' | 'private';
  seriesShared: boolean;
}

interface EditTraitForm extends CreateTraitForm {
  id: string;
}

const TRAIT_TYPE_OPTIONS = [
  { value: 'appearance', label: 'Appearance' },
  { value: 'personality', label: 'Personality' },
  { value: 'background', label: 'Background' },
  { value: 'skills', label: 'Skills & Abilities' },
  { value: 'relationships', label: 'Relationships' },
  { value: 'goals', label: 'Goals & Motivations' },
  { value: 'flaws', label: 'Flaws & Weaknesses' },
  { value: 'secrets', label: 'Secrets' },
  { value: 'other', label: 'Other' }
];

const VISIBILITY_OPTIONS = [
  { value: 'public', label: 'Public' },
  { value: 'private', label: 'Private' }
];

const CharactersManager: React.FC<CharactersManagerProps> = ({ 
  projectId, 
  characters = [] 
}) => {
  const { 
    characterTraits, 
    isLoading, 
    error, 
    createCharacterTrait, 
    updateCharacterTrait, 
    deleteCharacterTrait, 
    loadCharacterTraits,
    setSelectedCharacterId,
    setCharacterTraitFilter,
    generateCharacterTraits
  } = useStoryBible();

  const [selectedCharacter, setSelectedCharacter] = useState<string>('');
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showEditModal, setShowEditModal] = useState(false);
  const [editingTrait, setEditingTrait] = useState<CharacterTrait | null>(null);
  const [traitTypeFilter, setTraitTypeFilter] = useState<string>('');
  const [visibilityFilter, setVisibilityFilter] = useState<string>('');
  const [isGeneratingTraits, setIsGeneratingTraits] = useState(false);
  
  const [createForm, setCreateForm] = useState<CreateTraitForm>({
    traitType: '',
    content: '',
    visibility: 'public',
    seriesShared: false
  });
  
  const [editForm, setEditForm] = useState<EditTraitForm>({
    id: '',
    traitType: '',
    content: '',
    visibility: 'public',
    seriesShared: false
  });

  // Load character traits when character is selected
  useEffect(() => {
    if (selectedCharacter) {
      loadCharacterTraits(selectedCharacter);
      setSelectedCharacterId(selectedCharacter);
    }
  }, [selectedCharacter, loadCharacterTraits, setSelectedCharacterId]);

  // Update filters
  useEffect(() => {
    setCharacterTraitFilter({
      traitType: traitTypeFilter || undefined,
      visibility: visibilityFilter as 'public' | 'private' | undefined
    });
  }, [traitTypeFilter, visibilityFilter, setCharacterTraitFilter]);

  const closeModals = () => {
    setShowCreateModal(false);
    setShowEditModal(false);
    setEditingTrait(null);
    
    // Reset forms
    setCreateForm({
      traitType: '',
      content: '',
      visibility: 'public',
      seriesShared: false
    });
    
    setEditForm({
      id: '',
      traitType: '',
      content: '',
      visibility: 'public',
      seriesShared: false
    });
  };

  const handleCreateTrait = async () => {
    if (!selectedCharacter || !createForm.traitType || !createForm.content) {
      return;
    }
    
    try {
      await createCharacterTrait({
        character_id: selectedCharacter,
        trait_type: createForm.traitType,
        content: createForm.content,
        visibility: createForm.visibility,
        series_shared: createForm.seriesShared
      });
      
      closeModals();
    } catch (err) {
      console.error('Failed to create character trait:', err);
    }
  };

  const handleUpdateTrait = async () => {
    if (!editForm.id || !editForm.traitType || !editForm.content) {
      return;
    }
    
    try {
      await updateCharacterTrait({
        id: editForm.id,
        trait_type: editForm.traitType,
        content: editForm.content,
        visibility: editForm.visibility,
        series_shared: editForm.seriesShared
      });
      
      closeModals();
    } catch (err) {
      console.error('Failed to update character trait:', err);
    }
  };

  const handleDeleteTrait = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this trait?')) {
      try {
        await deleteCharacterTrait(id);
      } catch (err) {
        console.error('Failed to delete character trait:', err);
      }
    }
  };

  const handleCharacterSelect = (characterId: string) => {
    setSelectedCharacter(characterId);
  };

  const handleEditTrait = (trait: CharacterTrait) => {
    setEditingTrait(trait);
    setEditForm({
      id: trait.id,
      traitType: trait.trait_type,
      content: trait.content,
      visibility: trait.visibility,
      seriesShared: trait.series_shared
    });
    setShowEditModal(true);
  };

  const handleGenerateTraits = async () => {
    if (!selectedCharacter || !projectId) return;
    
    setIsGeneratingTraits(true);
    
    try {
      const request = {
        project_id: projectId,
        character_id: selectedCharacter,
        trait_type: createForm.traitType || 'personality'
      };
      
      const generatedContent = await generateCharacterTraits(request);
      
      if (generatedContent) {
        setCreateForm(prev => ({ ...prev, content: generatedContent }));
      }
    } catch (err) {
      console.error('Failed to generate character traits:', err);
    } finally {
      setIsGeneratingTraits(false);
    }
  };

  const getTraitTypeLabel = (value: string): string => {
    return TRAIT_TYPE_OPTIONS.find(option => option.value === value)?.label || value;
  };

  const getVisibilityLabel = (value: string): string => {
    return VISIBILITY_OPTIONS.find(option => option.value === value)?.label || value;
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">Characters Manager</h2>
        <Button 
          onClick={() => setShowCreateModal(true)}
          variant="primary"
          disabled={!selectedCharacter}
        >
          Add Trait
        </Button>
      </div>

      {/* Character Selection */}
      <Card>
        <CardHeader>
          <CardTitle>Select Character</CardTitle>
        </CardHeader>
        <CardContent>
          <Select value={selectedCharacter} onValueChange={handleCharacterSelect}>
            <SelectTrigger>
              <SelectValue placeholder="Choose a character to manage" />
            </SelectTrigger>
            <SelectContent>
              {characters.map(character => (
                <SelectItem key={character.id} value={character.id}>
                  {character.name}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </CardContent>
      </Card>

      {selectedCharacter && (
        <>
          {/* Filters */}
          <Card>
            <CardHeader>
              <CardTitle>Filter Traits</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Trait Type
                  </label>
                  <Select value={traitTypeFilter} onValueChange={setTraitTypeFilter}>
                    <SelectTrigger>
                      <SelectValue placeholder="All types" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="">All types</SelectItem>
                      {TRAIT_TYPE_OPTIONS.map(option => (
                        <SelectItem key={option.value} value={option.value}>
                          {option.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Visibility
                  </label>
                  <Select value={visibilityFilter} onValueChange={setVisibilityFilter}>
                    <SelectTrigger>
                      <SelectValue placeholder="All visibility" />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="">All visibility</SelectItem>
                      {VISIBILITY_OPTIONS.map(option => (
                        <SelectItem key={option.value} value={option.value}>
                          {option.label}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </CardContent>
          </Card>

          {/* Character Traits */}
          <Card>
            <CardHeader>
              <CardTitle>Character Traits</CardTitle>
            </CardHeader>
            <CardContent>
              {isLoading ? (
                <div className="text-center py-8 text-gray-500">
                  Loading traits...
                </div>
              ) : error ? (
                <div className="text-center py-8 text-red-600">
                  Error loading traits: {error}
                </div>
              ) : characterTraits.length === 0 ? (
                <div className="text-center py-8 text-gray-500">
                  No traits found for this character. Click "Add Trait" to get started.
                </div>
              ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {characterTraits.map(trait => (
                    <Card key={trait.id} className="border border-gray-200">
                      <CardHeader className="pb-2">
                        <div className="flex items-center justify-between">
                          <CardTitle className="text-sm font-medium">
                            {getTraitTypeLabel(trait.trait_type)}
                          </CardTitle>
                          <div className="flex gap-1">
                            <Button
                              size="sm"
                              variant="ghost"
                              onClick={() => handleEditTrait(trait)}
                            >
                              Edit
                            </Button>
                            <Button
                              size="sm"
                              variant="ghost"
                              onClick={() => handleDeleteTrait(trait.id)}
                              className="text-red-600 hover:text-red-700"
                            >
                              Delete
                            </Button>
                          </div>
                        </div>
                      </CardHeader>
                      <CardContent className="pt-0">
                        <p className="text-sm text-gray-700 mb-2">
                          {trait.content}
                        </p>
                        <div className="flex items-center justify-between text-xs text-gray-500">
                          <span>{getVisibilityLabel(trait.visibility)}</span>
                          {trait.series_shared && (
                            <span className="bg-blue-100 text-blue-800 px-2 py-1 rounded">
                              Series Shared
                            </span>
                          )}
                        </div>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </>
      )}

      {!selectedCharacter && (
        <Card>
          <CardContent className="text-center py-8 text-gray-500">
            Please select a character to view and manage their traits.
          </CardContent>
        </Card>
      )}

      {/* Create Trait Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
            <h3 className="text-lg font-semibold mb-4">Add Character Trait</h3>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Trait Type
                </label>
                <Select 
                  value={createForm.traitType} 
                  onValueChange={(value) => setCreateForm(prev => ({ ...prev, traitType: value }))}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select trait type" />
                  </SelectTrigger>
                  <SelectContent>
                    {TRAIT_TYPE_OPTIONS.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div>
                <div className="flex items-center justify-between mb-1">
                  <label className="block text-sm font-medium text-gray-700">
                    Content
                  </label>
                  <Button
                    onClick={handleGenerateTraits}
                    disabled={isGeneratingTraits || !createForm.traitType}
                    variant="outline"
                    size="sm"
                    className="flex items-center gap-2"
                  >
                    {isGeneratingTraits ? (
                      <Loader2 className="h-4 w-4 animate-spin" />
                    ) : (
                      <Sparkles className="h-4 w-4" />
                    )}
                    {isGeneratingTraits ? 'Generating...' : 'Generate with AI'}
                  </Button>
                </div>
                <Textarea
                  value={createForm.content}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, content: e.target.value }))}
                  placeholder="Describe this character trait..."
                  rows={3}
                />
                {!createForm.traitType && (
                  <p className="text-sm text-gray-500 mt-1">
                    💡 Select a trait type to enable AI generation
                  </p>
                )}
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Visibility
                </label>
                <Select 
                  value={createForm.visibility} 
                  onValueChange={(value: 'public' | 'private') => setCreateForm(prev => ({ ...prev, visibility: value }))}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {VISIBILITY_OPTIONS.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="createSeriesShared"
                  checked={createForm.seriesShared}
                  onChange={(e) => setCreateForm(prev => ({ ...prev, seriesShared: e.target.checked }))}
                  className="mr-2"
                />
                <label htmlFor="createSeriesShared" className="text-sm text-gray-700">
                  Share across series
                </label>
              </div>
            </div>
            
            <div className="flex gap-2 mt-6">
              <Button onClick={closeModals} variant="outline" className="flex-1">
                Cancel
              </Button>
              <Button 
                onClick={handleCreateTrait} 
                variant="primary" 
                className="flex-1"
                disabled={!createForm.traitType || !createForm.content}
              >
                Add Trait
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Edit Trait Modal */}
      {showEditModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
            <h3 className="text-lg font-semibold mb-4">Edit Character Trait</h3>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Trait Type
                </label>
                <Select 
                  value={editForm.traitType} 
                  onValueChange={(value) => setEditForm(prev => ({ ...prev, traitType: value }))}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select trait type" />
                  </SelectTrigger>
                  <SelectContent>
                    {TRAIT_TYPE_OPTIONS.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Content
                </label>
                <Textarea
                  value={editForm.content}
                  onChange={(e) => setEditForm(prev => ({ ...prev, content: e.target.value }))}
                  placeholder="Describe this character trait..."
                  rows={3}
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Visibility
                </label>
                <Select 
                  value={editForm.visibility} 
                  onValueChange={(value: 'public' | 'private') => setEditForm(prev => ({ ...prev, visibility: value }))}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    {VISIBILITY_OPTIONS.map(option => (
                      <SelectItem key={option.value} value={option.value}>
                        {option.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="editSeriesShared"
                  checked={editForm.seriesShared}
                  onChange={(e) => setEditForm(prev => ({ ...prev, seriesShared: e.target.checked }))}
                  className="mr-2"
                />
                <label htmlFor="editSeriesShared" className="text-sm text-gray-700">
                  Share across series
                </label>
              </div>
            </div>
            
            <div className="flex gap-2 mt-6">
              <Button onClick={closeModals} variant="outline" className="flex-1">
                Cancel
              </Button>
              <Button 
                onClick={handleUpdateTrait} 
                variant="primary" 
                className="flex-1"
                disabled={!editForm.traitType || !editForm.content}
              >
                Save Changes
              </Button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default CharactersManager;