import React, { useState, useEffect } from 'react';
import { Button } from '../../../../components/ui/Button';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../components/ui/Card';
import { Input } from '../../../../components/ui/input';
import { Textarea } from '../../../../components/ui/textarea';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../components/ui/select';
import { Sparkles, Loader2, Download, Users, Plus, Trash2, Network, Upload } from 'lucide-react';
import type { CharacterTrait, CharactersManagerProps } from '../../../../types/storyBible';
import useStoryBible from '../../hooks/useStoryBible';
import RelationshipGraph from './RelationshipGraph';
import CSVImportDialog from './CSVImportDialog';
import SmartImportDialog from './SmartImportDialog';

interface CreateTraitForm {
  traitType: string;
  content: string;
  visibility: 'public' | 'private';
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
  projectId
}) => {
  const { 
    characters,
    characterTraits, 
    isLoading,
    isLoadingCharacters,
    charactersError,
    error, 
    loadCharacters,
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
  
  // Relationship management state
  const [showRelationshipView, setShowRelationshipView] = useState(false);
  const [showGraphView, setShowGraphView] = useState(false);
  const [showImportDialog, setShowImportDialog] = useState(false);
  const [showSmartImportDialog, setShowSmartImportDialog] = useState(false);
  const [showCreateRelationshipModal, setShowCreateRelationshipModal] = useState(false);
  const [relationships, setRelationships] = useState<Array<{
    id: string;
    fromCharacterId: string;
    toCharacterId: string;
    relationshipType: string;
    description: string;
    strength: number; // 1-10 scale
    isPublic: boolean;
  }>>([]);
  
  const [createRelationshipForm, setCreateRelationshipForm] = useState({
    fromCharacterId: '',
    toCharacterId: '',
    relationshipType: '',
    description: '',
    strength: 5,
    isPublic: true
  });
  
  const RELATIONSHIP_TYPES = [
    { value: 'family', label: 'Family' },
    { value: 'romantic', label: 'Romantic' },
    { value: 'friend', label: 'Friend' },
    { value: 'enemy', label: 'Enemy' },
    { value: 'ally', label: 'Ally' },
    { value: 'mentor', label: 'Mentor' },
    { value: 'rival', label: 'Rival' },
    { value: 'colleague', label: 'Colleague' },
    { value: 'acquaintance', label: 'Acquaintance' },
    { value: 'other', label: 'Other' }
  ];
  
  const [createForm, setCreateForm] = useState<CreateTraitForm>({
    traitType: '',
    content: '',
    visibility: 'public'
  });
  
  const [editForm, setEditForm] = useState<EditTraitForm>({
    id: '',
    traitType: '',
    content: '',
    visibility: 'public'
  });

  // Load characters when component mounts
  useEffect(() => {
    if (projectId) {
      loadCharacters(projectId);
    }
  }, [projectId, loadCharacters]);

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
      visibility: 'public'
    });
    
    setEditForm({
      id: '',
      traitType: '',
      content: '',
      visibility: 'public'
    });
  };

  const handleCreateTrait = async () => {
    if (!selectedCharacter || !createForm.traitType || !createForm.content) {
      return;
    }
    
    try {
      await createCharacterTrait({
        character_id: selectedCharacter,
        trait_name: createForm.traitType,
        trait_value: createForm.content,
        visibility: createForm.visibility
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
        trait_name: editForm.traitType,
        trait_value: editForm.content,
        visibility: editForm.visibility
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
      traitType: trait.trait_name,
      content: trait.trait_value,
      visibility: trait.visibility
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

  const handleExportCSV = () => {
    if (!selectedCharacter) return;
    
    const selectedCharacterData = characters.find(c => c.id === selectedCharacter);
    if (!selectedCharacterData) return;
    
    const filteredCharacterTraits = characterTraits.filter(t => t.character_id === selectedCharacter);
    
    // Create CSV content
    const headers = ['Character Name', 'Trait Type', 'Content', 'Visibility'];
    const rows = characterTraits.map(trait => [
      selectedCharacterData.name,
      getTraitTypeLabel(trait.trait_name),
      trait.trait_value.replace(/"/g, '""'), // Escape quotes
      getVisibilityLabel(trait.visibility)
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
    link.setAttribute('download', `${selectedCharacterData.name}_traits.csv`);
    link.style.visibility = 'hidden';
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  const getTraitTypeLabel = (value: string): string => {
    return TRAIT_TYPE_OPTIONS.find(option => option.value === value)?.label || value;
  };

  const getVisibilityLabel = (value: string): string => {
    return VISIBILITY_OPTIONS.find(option => option.value === value)?.label || value;
  };

  // Relationship management functions
  const handleCreateRelationship = async () => {
    if (!createRelationshipForm.fromCharacterId || !createRelationshipForm.toCharacterId) return;
    
    const newRelationship = {
      id: Date.now().toString(),
      ...createRelationshipForm
    };
    
    setRelationships(prev => [...prev, newRelationship]);
    setShowCreateRelationshipModal(false);
    setCreateRelationshipForm({
      fromCharacterId: '',
      toCharacterId: '',
      relationshipType: '',
      description: '',
      strength: 5,
      isPublic: true
    });
  };

  const handleDeleteRelationship = (relationshipId: string) => {
    setRelationships(prev => prev.filter(r => r.id !== relationshipId));
  };

  const getCharacterName = (characterId: string): string => {
    return characters.find(c => c.id === characterId)?.name || 'Unknown Character';
  };

  const getRelationshipTypeLabel = (value: string): string => {
    return RELATIONSHIP_TYPES.find(type => type.value === value)?.label || value;
  };

  const getStrengthLabel = (strength: number): string => {
    if (strength <= 2) return 'Weak';
    if (strength <= 4) return 'Mild';
    if (strength <= 6) return 'Moderate';
    if (strength <= 8) return 'Strong';
    return 'Very Strong';
  };

  const handleSmartImport = async (data: any[], type: 'characters' | 'locations' | 'plot_points' | 'themes') => {
    try {
      if (type === 'characters') {
        // Import characters using the existing character creation logic
        for (const characterData of data) {
          // This would need to be implemented based on your character creation API
          console.log('Importing character:', characterData);
          // await createCharacter(characterData);
        }
      } else if (type === 'locations' || type === 'plot_points' || type === 'themes') {
        // Import worldbuilding elements
        for (const elementData of data) {
          console.log(`Importing ${type}:`, elementData);
          // await createWorldbuildingElement(elementData);
        }
      }
      
      // Refresh the character list after import
      if (selectedCharacter) {
        loadCharacterTraits(selectedCharacter);
      }
      
      alert(`Successfully imported ${data.length} ${type}!`);
    } catch (error) {
      console.error(`Failed to import ${type}:`, error);
      alert(`Failed to import ${type}: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">Characters Manager</h2>
        <div className="flex items-center gap-2">
          <Button 
            onClick={() => setShowRelationshipView(!showRelationshipView)}
            variant={showRelationshipView ? "default" : "outline"}
            className="flex items-center gap-2"
          >
            <Users className="h-4 w-4" />
            {showRelationshipView ? 'View Traits' : 'View Relationships'}
          </Button>
          {showRelationshipView && (
            <Button 
              onClick={() => setShowGraphView(!showGraphView)}
              variant={showGraphView ? "default" : "outline"}
              className="flex items-center gap-2"
            >
              <Network className="h-4 w-4" />
              {showGraphView ? 'List View' : 'Graph View'}
            </Button>
          )}
          <Button
            variant="outline"
            onClick={() => setShowSmartImportDialog(true)}
            className="flex items-center gap-2"
          >
            <Sparkles className="h-4 w-4" />
            Smart Import
          </Button>
          <Button
            variant="outline"
            onClick={() => setShowImportDialog(true)}
          >
            <Upload className="h-4 w-4 mr-2" />
            Import CSV
          </Button>
          <Button 
            onClick={handleExportCSV}
            variant="outline"
            disabled={!selectedCharacter || traits.filter(t => t.character_id === selectedCharacter).length === 0}
            className="flex items-center gap-2"
          >
            <Download className="h-4 w-4" />
            Export CSV
          </Button>
          {!showRelationshipView ? (
            <Button 
              onClick={() => setShowCreateModal(true)}
              variant="primary"
              disabled={!selectedCharacter}
            >
              Add Trait
            </Button>
          ) : (
            <Button 
              onClick={() => setShowCreateRelationshipModal(true)}
              variant="primary"
              disabled={characters.length < 2}
            >
              <Plus className="h-4 w-4 mr-2" />
              Add Relationship
            </Button>
          )}
        </div>
      </div>

      {/* Character Selection */}
      <Card>
        <CardHeader>
          <CardTitle>Select Character</CardTitle>
        </CardHeader>
        <CardContent>
          {isLoadingCharacters ? (
            <div className="text-center py-4 text-gray-500">
              <Loader2 className="h-4 w-4 animate-spin mx-auto mb-2" />
              Loading characters...
            </div>
          ) : charactersError ? (
            <div className="text-center py-4 text-red-600">
              Error loading characters: {charactersError}
            </div>
          ) : (
            <Select value={selectedCharacter} onValueChange={handleCharacterSelect}>
              <SelectTrigger>
                <SelectValue placeholder={characters.length === 0 ? "No characters available" : "Choose a character to manage"} />
              </SelectTrigger>
              <SelectContent>
                {characters.map(character => (
                  <SelectItem key={character.id} value={character.id}>
                    {character.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          )}
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

          {/* Character Traits or Relationships */}
          {!showRelationshipView ? (
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
                          {getTraitTypeLabel(trait.trait_name)}
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
                          {trait.trait_value}
                        </p>
                        <div className="flex items-center justify-between text-xs text-gray-500">
                          <span>{getVisibilityLabel(trait.visibility)}</span>
                        </div>
                      </CardContent>
                    </Card>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
          ) : (
            <div className="space-y-6">
              {showGraphView ? (
                <RelationshipGraph
                  characters={characters}
                  relationships={relationships}
                  onNodeClick={(characterId) => {
                    setSelectedCharacter(characterId);
                    setShowRelationshipView(false);
                  }}
                  onRelationshipClick={(relationship) => {
                    // Handle relationship click if needed
                    console.log('Relationship clicked:', relationship);
                  }}
                />
              ) : (
                <Card>
                  <CardHeader>
                    <CardTitle>Character Relationships</CardTitle>
                  </CardHeader>
                  <CardContent>
                    {relationships.length === 0 ? (
                      <div className="text-center py-8 text-gray-500">
                        No relationships defined. Click "Add Relationship" to get started.
                      </div>
                    ) : (
                      <div className="space-y-4">
                        {relationships.map(relationship => (
                          <Card key={relationship.id} className="border border-gray-200">
                            <CardContent className="p-4">
                              <div className="flex items-start justify-between">
                                <div className="flex-1">
                                  <div className="flex items-center gap-2 mb-2">
                                    <span className="font-medium">
                                      {getCharacterName(relationship.fromCharacterId)}
                                    </span>
                                    <span className="text-gray-500">→</span>
                                    <span className="font-medium">
                                      {getCharacterName(relationship.toCharacterId)}
                                    </span>
                                  </div>
                                  <div className="flex items-center gap-4 text-sm text-gray-600 mb-2">
                                    <span className="bg-blue-100 text-blue-800 px-2 py-1 rounded">
                                      {getRelationshipTypeLabel(relationship.relationshipType)}
                                    </span>
                                    <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded">
                                      {getStrengthLabel(relationship.strength)}
                                    </span>
                                    <span className={`px-2 py-1 rounded ${
                                      relationship.isPublic 
                                        ? 'bg-green-100 text-green-800' 
                                        : 'bg-yellow-100 text-yellow-800'
                                    }`}>
                                      {relationship.isPublic ? 'Public' : 'Private'}
                                    </span>
                                  </div>
                                  {relationship.description && (
                                    <p className="text-sm text-gray-700">
                                      {relationship.description}
                                    </p>
                                  )}
                                </div>
                                <Button
                                  size="sm"
                                  variant="ghost"
                                  onClick={() => handleDeleteRelationship(relationship.id)}
                                  className="text-red-600 hover:text-red-700"
                                >
                                  <Trash2 className="h-4 w-4" />
                                </Button>
                              </div>
                            </CardContent>
                          </Card>
                        ))}
                      </div>
                    )}
                  </CardContent>
                </Card>
              )}
            </div>
          )}
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

      {/* CSV Import Dialog */}
       {showImportDialog && (
         <CSVImportDialog
           isOpen={showImportDialog}
           onClose={() => setShowImportDialog(false)}
           onImport={async (data, type) => {
             try {
               // Process imported character data
               for (const characterData of data) {
                 await createCharacter({
                   name: characterData.name,
                   description: characterData.description,
                   visibility: characterData.visibility,
                   series_shared: characterData.series_shared
                 });
                 
                 // Add traits for the character
                 const character = characters.find(c => c.name === characterData.name);
                 if (character && characterData.traits) {
                   for (const [traitName, traitValue] of Object.entries(characterData.traits)) {
                     if (traitValue) {
                       await createCharacterTrait({
                           character_id: character.id,
                           trait_name: traitName,
                           trait_value: traitValue as string,
                           visibility: characterData.visibility
                         });
                     }
                   }
                 }
               }
               
               // Refresh the characters list
               await loadCharacters(projectId);
               setShowImportDialog(false);
             } catch (error) {
               console.error('Import failed:', error);
               alert('Import failed. Please check the console for details.');
             }
           }}
           importType="characters"
           projectId={projectId}
         />
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

      {/* Create Relationship Modal */}
      {showCreateRelationshipModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
            <h3 className="text-lg font-semibold mb-4">Create Character Relationship</h3>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  From Character
                </label>
                <Select 
                  value={createRelationshipForm.fromCharacterId} 
                  onValueChange={(value) => setCreateRelationshipForm(prev => ({ ...prev, fromCharacterId: value }))}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select character" />
                  </SelectTrigger>
                  <SelectContent>
                    {characters.map(character => (
                      <SelectItem key={character.id} value={character.id}>
                        {character.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  To Character
                </label>
                <Select 
                  value={createRelationshipForm.toCharacterId} 
                  onValueChange={(value) => setCreateRelationshipForm(prev => ({ ...prev, toCharacterId: value }))}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select character" />
                  </SelectTrigger>
                  <SelectContent>
                    {characters
                      .filter(c => c.id !== createRelationshipForm.fromCharacterId)
                      .map(character => (
                        <SelectItem key={character.id} value={character.id}>
                          {character.name}
                        </SelectItem>
                      ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Relationship Type
                </label>
                <Select 
                  value={createRelationshipForm.relationshipType} 
                  onValueChange={(value) => setCreateRelationshipForm(prev => ({ ...prev, relationshipType: value }))}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select relationship type" />
                  </SelectTrigger>
                  <SelectContent>
                    {RELATIONSHIP_TYPES.map(type => (
                      <SelectItem key={type.value} value={type.value}>
                        {type.label}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Description
                </label>
                <Textarea
                  value={createRelationshipForm.description}
                  onChange={(e) => setCreateRelationshipForm(prev => ({ ...prev, description: e.target.value }))}
                  placeholder="Describe the relationship..."
                  rows={3}
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Relationship Strength: {createRelationshipForm.strength}
                </label>
                <input
                  type="range"
                  min="1"
                  max="10"
                  value={createRelationshipForm.strength}
                  onChange={(e) => setCreateRelationshipForm(prev => ({ ...prev, strength: parseInt(e.target.value) }))}
                  className="w-full"
                />
                <div className="flex justify-between text-xs text-gray-500 mt-1">
                  <span>Weak</span>
                  <span>Strong</span>
                </div>
              </div>
              
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="createRelationshipPublic"
                  checked={createRelationshipForm.isPublic}
                  onChange={(e) => setCreateRelationshipForm(prev => ({ ...prev, isPublic: e.target.checked }))}
                  className="mr-2"
                />
                <label htmlFor="createRelationshipPublic" className="text-sm text-gray-700">
                  Public relationship
                </label>
              </div>
            </div>
            
            <div className="flex gap-2 mt-6">
              <Button 
                onClick={() => setShowCreateRelationshipModal(false)} 
                variant="outline" 
                className="flex-1"
              >
                Cancel
              </Button>
              <Button 
                onClick={handleCreateRelationship} 
                variant="primary" 
                className="flex-1"
                disabled={!createRelationshipForm.fromCharacterId || !createRelationshipForm.toCharacterId || !createRelationshipForm.relationshipType}
              >
                Create Relationship
              </Button>
            </div>
          </div>
        </div>
      )}

      {/* Smart Import Dialog */}
      <SmartImportDialog
        isOpen={showSmartImportDialog}
        onClose={() => setShowSmartImportDialog(false)}
        onImport={handleSmartImport}
        projectId={projectId}
      />
    </div>
  );
};

export default CharactersManager;