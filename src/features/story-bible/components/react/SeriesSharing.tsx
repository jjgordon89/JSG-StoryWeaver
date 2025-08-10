import React, { useState } from 'react';
import { Button } from '../../../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../ui/components/common';
import { Input } from '../../../../ui/components/common';
import { Textarea } from '../../../../ui/components/common';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../ui/components/common';
import { Badge } from '../../../../components/ui/badge';
import { Checkbox } from '../../../../components/ui/checkbox';
import { Share2, Users, Globe, Link, Unlink, Eye, EyeOff, Copy, Check, AlertCircle } from 'lucide-react';

interface Character {
  id: string;
  name: string;
  description: string;
  series_shared: boolean;
  visibility: 'public' | 'private';
  project_id: string;
  project_name?: string;
  traits?: Array<{ trait_name: string; trait_value: string }>;
}

interface WorldElement {
  id: string;
  name: string;
  element_type: string;
  description: string;
  significance?: string;
  series_shared: boolean;
  visibility: 'always' | 'chapter' | 'never';
  project_id: string;
  project_name?: string;
  details?: string;
}

interface Project {
  id: string;
  name: string;
  series_id?: string;
  series_name?: string;
}

interface SeriesSharingProps {
  currentProject: Project;
  seriesProjects: Project[];
  characters: Character[];
  worldElements: WorldElement[];
  onShareCharacter: (characterId: string, share: boolean) => Promise<void>;
  onShareWorldElement: (elementId: string, share: boolean) => Promise<void>;
  onImportFromSeries: (type: 'characters' | 'worldbuilding', items: string[]) => Promise<void>;
  onCreateSeries?: (name: string, description: string) => Promise<void>;
  onJoinSeries?: (seriesId: string) => Promise<void>;
}

interface SharedItem {
  id: string;
  name: string;
  type: 'character' | 'worldbuilding';
  description: string;
  source_project: string;
  shared_in_projects: string[];
  last_updated: string;
}

const SeriesSharing: React.FC<SeriesSharingProps> = ({
  currentProject,
  seriesProjects,
  characters,
  worldElements,
  onShareCharacter,
  onShareWorldElement,
  onImportFromSeries,
  onCreateSeries,
  onJoinSeries
}) => {
  const [activeTab, setActiveTab] = useState<'share' | 'import' | 'manage'>('share');
  const [selectedItems, setSelectedItems] = useState<Set<string>>(new Set());
  const [importType, setImportType] = useState<'characters' | 'worldbuilding'>('characters');
  const [showCreateSeries, setShowCreateSeries] = useState(false);
  const [newSeriesName, setNewSeriesName] = useState('');
  const [newSeriesDescription, setNewSeriesDescription] = useState('');
  const [joinSeriesId, setJoinSeriesId] = useState('');
  const [copiedItems, setCopiedItems] = useState<Set<string>>(new Set());

  // Get shared items from other projects in the series
  const getSharedItemsFromSeries = (): SharedItem[] => {
    const sharedItems: SharedItem[] = [];
    
    // Add characters from other projects
    const otherProjectCharacters = characters.filter(c => 
      c.project_id !== currentProject.id && c.series_shared
    );
    
    otherProjectCharacters.forEach(character => {
      sharedItems.push({
        id: character.id,
        name: character.name,
        type: 'character',
        description: character.description,
        source_project: character.project_name || character.project_id,
        shared_in_projects: [character.project_id],
        last_updated: new Date().toISOString() // This would come from the backend
      });
    });

    // Add world elements from other projects
    const otherProjectElements = worldElements.filter(w => 
      w.project_id !== currentProject.id && w.series_shared
    );
    
    otherProjectElements.forEach(element => {
      sharedItems.push({
        id: element.id,
        name: element.name,
        type: 'worldbuilding',
        description: element.description,
        source_project: element.project_name || element.project_id,
        shared_in_projects: [element.project_id],
        last_updated: new Date().toISOString()
      });
    });

    return sharedItems;
  };

  const handleShareToggle = async (itemId: string, type: 'character' | 'worldbuilding', currentlyShared: boolean) => {
    try {
      if (type === 'character') {
        await onShareCharacter(itemId, !currentlyShared);
      } else {
        await onShareWorldElement(itemId, !currentlyShared);
      }
    } catch (error) {
      console.error('Failed to toggle sharing:', error);
      alert('Failed to update sharing status');
    }
  };

  const handleImportSelected = async () => {
    if (selectedItems.size === 0) {
      alert('Please select items to import');
      return;
    }

    try {
      await onImportFromSeries(importType, Array.from(selectedItems));
      setSelectedItems(new Set());
      alert(`Successfully imported ${selectedItems.size} ${importType}`);
    } catch (error) {
      console.error('Import failed:', error);
      alert('Failed to import selected items');
    }
  };

  const handleCreateSeries = async () => {
    if (!newSeriesName.trim()) {
      alert('Please enter a series name');
      return;
    }

    try {
      if (onCreateSeries) {
        await onCreateSeries(newSeriesName, newSeriesDescription);
        setShowCreateSeries(false);
        setNewSeriesName('');
        setNewSeriesDescription('');
        alert('Series created successfully!');
      }
    } catch (error) {
      console.error('Failed to create series:', error);
      alert('Failed to create series');
    }
  };

  const handleJoinSeries = async () => {
    if (!joinSeriesId.trim()) {
      alert('Please enter a series ID');
      return;
    }

    try {
      if (onJoinSeries) {
        await onJoinSeries(joinSeriesId);
        setJoinSeriesId('');
        alert('Successfully joined series!');
      }
    } catch (error) {
      console.error('Failed to join series:', error);
      alert('Failed to join series');
    }
  };

  const copyToClipboard = async (text: string, itemId: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedItems(prev => new Set([...prev, itemId]));
      setTimeout(() => {
        setCopiedItems(prev => {
          const newSet = new Set(prev);
          newSet.delete(itemId);
          return newSet;
        });
      }, 2000);
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
    }
  };

  const sharedItemsFromSeries = getSharedItemsFromSeries();
  const currentProjectCharacters = characters.filter(c => c.project_id === currentProject.id);
  const currentProjectElements = worldElements.filter(w => w.project_id === currentProject.id);

  const tabs = [
    { id: 'share', label: 'Share Items', icon: Share2 },
    { id: 'import', label: 'Import from Series', icon: Copy },
    { id: 'manage', label: 'Manage Series', icon: Link }
  ];

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Series Sharing</h2>
          <p className="text-gray-600">
            Share Story Bible elements across projects in your series
          </p>
          {currentProject.series_name && (
            <Badge variant="outline" className="mt-2">
              Series: {currentProject.series_name}
            </Badge>
          )}
        </div>
      </div>

      {/* Series Status */}
      {!currentProject.series_id && (
        <Card className="border-yellow-200 bg-yellow-50">
          <CardContent className="p-4">
            <div className="flex items-center gap-2 text-yellow-800">
              <AlertCircle className="h-4 w-4" />
              <span className="font-medium">This project is not part of a series</span>
            </div>
            <p className="text-yellow-700 text-sm mt-1">
              Create a new series or join an existing one to share Story Bible elements across projects.
            </p>
          </CardContent>
        </Card>
      )}

      {/* Tab Navigation */}
      <div className="flex space-x-1 bg-gray-100 p-1 rounded-lg">
        {tabs.map(tab => {
          const Icon = tab.icon;
          return (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={`flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors ${
                activeTab === tab.id
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <Icon className="h-4 w-4" />
              {tab.label}
            </button>
          );
        })}
      </div>

      {/* Share Items Tab */}
      {activeTab === 'share' && (
        <div className="space-y-6">
          {/* Characters */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Users className="h-5 w-5" />
                Characters ({currentProjectCharacters.length})
              </CardTitle>
            </CardHeader>
            <CardContent>
              {currentProjectCharacters.length === 0 ? (
                <p className="text-gray-500 text-center py-4">
                  No characters in this project
                </p>
              ) : (
                <div className="space-y-3">
                  {currentProjectCharacters.map(character => (
                    <div key={character.id} className="flex items-center justify-between p-3 border rounded-lg">
                      <div className="flex-1">
                        <h4 className="font-medium text-gray-900">{character.name}</h4>
                        <p className="text-sm text-gray-600">{character.description}</p>
                        <div className="flex items-center gap-2 mt-1">
                          <Badge variant={character.visibility === 'public' ? 'default' : 'secondary'}>
                            {character.visibility === 'public' ? (
                              <><Eye className="h-3 w-3 mr-1" /> Public</>
                            ) : (
                              <><EyeOff className="h-3 w-3 mr-1" /> Private</>
                            )}
                          </Badge>
                          {character.series_shared && (
                            <Badge variant="outline" className="bg-green-50 text-green-700">
                              <Share2 className="h-3 w-3 mr-1" />
                              Shared
                            </Badge>
                          )}
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <Button
                          variant={character.series_shared ? 'secondary' : 'default'}
                          size="sm"
                          onClick={() => handleShareToggle(character.id, 'character', character.series_shared)}
                          disabled={!currentProject.series_id}
                        >
                          {character.series_shared ? (
                            <><Unlink className="h-4 w-4 mr-1" /> Unshare</>
                          ) : (
                            <><Link className="h-4 w-4 mr-1" /> Share</>
                          )}
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>

          {/* World Elements */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Globe className="h-5 w-5" />
                Worldbuilding Elements ({currentProjectElements.length})
              </CardTitle>
            </CardHeader>
            <CardContent>
              {currentProjectElements.length === 0 ? (
                <p className="text-gray-500 text-center py-4">
                  No worldbuilding elements in this project
                </p>
              ) : (
                <div className="space-y-3">
                  {currentProjectElements.map(element => (
                    <div key={element.id} className="flex items-center justify-between p-3 border rounded-lg">
                      <div className="flex-1">
                        <h4 className="font-medium text-gray-900">{element.name}</h4>
                        <p className="text-sm text-gray-600">{element.description}</p>
                        <div className="flex items-center gap-2 mt-1">
                          <Badge variant="outline">
                            {element.element_type}
                          </Badge>
                          <Badge variant={element.visibility === 'always' ? 'default' : 'secondary'}>
                            {element.visibility}
                          </Badge>
                          {element.series_shared && (
                            <Badge variant="outline" className="bg-green-50 text-green-700">
                              <Share2 className="h-3 w-3 mr-1" />
                              Shared
                            </Badge>
                          )}
                        </div>
                      </div>
                      <div className="flex items-center gap-2">
                        <Button
                          variant={element.series_shared ? 'secondary' : 'default'}
                          size="sm"
                          onClick={() => handleShareToggle(element.id, 'worldbuilding', element.series_shared)}
                          disabled={!currentProject.series_id}
                        >
                          {element.series_shared ? (
                            <><Unlink className="h-4 w-4 mr-1" /> Unshare</>
                          ) : (
                            <><Link className="h-4 w-4 mr-1" /> Share</>
                          )}
                        </Button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </CardContent>
          </Card>
        </div>
      )}

      {/* Import from Series Tab */}
      {activeTab === 'import' && (
        <div className="space-y-6">
          {!currentProject.series_id ? (
            <Card>
              <CardContent className="p-6 text-center">
                <AlertCircle className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  Not Part of a Series
                </h3>
                <p className="text-gray-600">
                  This project must be part of a series to import shared elements.
                </p>
              </CardContent>
            </Card>
          ) : (
            <>
              {/* Import Controls */}
              <Card>
                <CardHeader>
                  <CardTitle>Import Shared Elements</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="flex items-center gap-4 mb-4">
                    <Select value={importType} onValueChange={(value: 'characters' | 'worldbuilding') => setImportType(value)}>
                      <SelectTrigger className="w-48">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="characters">Characters</SelectItem>
                        <SelectItem value="worldbuilding">Worldbuilding</SelectItem>
                      </SelectContent>
                    </Select>
                    <Button 
                      onClick={handleImportSelected}
                      disabled={selectedItems.size === 0}
                    >
                      Import Selected ({selectedItems.size})
                    </Button>
                  </div>
                </CardContent>
              </Card>

              {/* Available Items */}
              <Card>
                <CardHeader>
                  <CardTitle>
                    Available {importType === 'characters' ? 'Characters' : 'Worldbuilding Elements'}
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  {sharedItemsFromSeries.filter(item => item.type === importType).length === 0 ? (
                    <p className="text-gray-500 text-center py-4">
                      No shared {importType} available from other projects in the series
                    </p>
                  ) : (
                    <div className="space-y-3">
                      {sharedItemsFromSeries
                        .filter(item => item.type === importType)
                        .map(item => (
                          <div key={item.id} className="flex items-center gap-3 p-3 border rounded-lg">
                            <Checkbox
                              checked={selectedItems.has(item.id)}
                              onCheckedChange={(checked) => {
                                const newSelected = new Set(selectedItems);
                                if (checked) {
                                  newSelected.add(item.id);
                                } else {
                                  newSelected.delete(item.id);
                                }
                                setSelectedItems(newSelected);
                              }}
                            />
                            <div className="flex-1">
                              <h4 className="font-medium text-gray-900">{item.name}</h4>
                              <p className="text-sm text-gray-600">{item.description}</p>
                              <p className="text-xs text-gray-500 mt-1">
                                From: {item.source_project}
                              </p>
                            </div>
                            <Button
                              variant="ghost"
                              size="sm"
                              onClick={() => copyToClipboard(item.name, item.id)}
                            >
                              {copiedItems.has(item.id) ? (
                                <Check className="h-4 w-4 text-green-500" />
                              ) : (
                                <Copy className="h-4 w-4" />
                              )}
                            </Button>
                          </div>
                        ))
                      }
                    </div>
                  )}
                </CardContent>
              </Card>
            </>
          )}
        </div>
      )}

      {/* Manage Series Tab */}
      {activeTab === 'manage' && (
        <div className="space-y-6">
          {/* Current Series Info */}
          {currentProject.series_id ? (
            <Card>
              <CardHeader>
                <CardTitle>Current Series</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div>
                    <h3 className="font-medium text-gray-900">{currentProject.series_name}</h3>
                    <p className="text-sm text-gray-600">Series ID: {currentProject.series_id}</p>
                  </div>
                  <div>
                    <h4 className="font-medium text-gray-900 mb-2">Projects in Series:</h4>
                    <div className="space-y-2">
                      {seriesProjects.map(project => (
                        <div key={project.id} className="flex items-center justify-between p-2 bg-gray-50 rounded">
                          <span className={project.id === currentProject.id ? 'font-medium' : ''}>
                            {project.name} {project.id === currentProject.id && '(Current)'}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          ) : (
            <div className="space-y-4">
              {/* Create New Series */}
              <Card>
                <CardHeader>
                  <CardTitle>Create New Series</CardTitle>
                </CardHeader>
                <CardContent>
                  {!showCreateSeries ? (
                    <Button onClick={() => setShowCreateSeries(true)}>
                      Create New Series
                    </Button>
                  ) : (
                    <div className="space-y-4">
                      <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                          Series Name
                        </label>
                        <Input
                          value={newSeriesName}
                          onChange={(e) => setNewSeriesName(e.target.value)}
                          placeholder="Enter series name"
                        />
                      </div>
                      <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                          Description (Optional)
                        </label>
                        <Textarea
                          value={newSeriesDescription}
                          onChange={(e) => setNewSeriesDescription(e.target.value)}
                          placeholder="Describe your series"
                          rows={3}
                        />
                      </div>
                      <div className="flex gap-2">
                        <Button onClick={handleCreateSeries}>
                          Create Series
                        </Button>
                        <Button 
                          variant="outline" 
                          onClick={() => {
                            setShowCreateSeries(false);
                            setNewSeriesName('');
                            setNewSeriesDescription('');
                          }}
                        >
                          Cancel
                        </Button>
                      </div>
                    </div>
                  )}
                </CardContent>
              </Card>

              {/* Join Existing Series */}
              <Card>
                <CardHeader>
                  <CardTitle>Join Existing Series</CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm font-medium text-gray-700 mb-1">
                        Series ID
                      </label>
                      <Input
                        value={joinSeriesId}
                        onChange={(e) => setJoinSeriesId(e.target.value)}
                        placeholder="Enter series ID to join"
                      />
                    </div>
                    <Button onClick={handleJoinSeries} disabled={!joinSeriesId.trim()}>
                      Join Series
                    </Button>
                  </div>
                </CardContent>
              </Card>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default SeriesSharing;