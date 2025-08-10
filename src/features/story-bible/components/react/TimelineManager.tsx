import React, { useState, useEffect, useRef } from 'react';
import { Button } from '../../../../ui/components/common';
import { Card, CardContent, CardHeader, CardTitle } from '../../../../ui/components/common';
import { Input } from '../../../../ui/components/common';
import { Textarea } from '../../../../ui/components/common';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '../../../../ui/components/common';
import { Badge } from '../../../../components/ui/badge';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '../../../../components/ui/dialog';
import { Calendar, Clock, Plus, Edit, Trash2, Filter, ZoomIn, ZoomOut, Download } from 'lucide-react';

interface TimelineEvent {
  id: string;
  title: string;
  description: string;
  event_type: 'plot' | 'character' | 'world' | 'conflict' | 'resolution' | 'custom';
  date_in_story?: string; // In-story date/time
  chapter_reference?: string;
  scene_reference?: string;
  characters_involved: string[];
  world_elements_involved: string[];
  importance: 'low' | 'medium' | 'high' | 'critical';
  status: 'planned' | 'draft' | 'written' | 'revised';
  tags: string[];
  notes: string;
  created_at: string;
  updated_at: string;
  position_index: number; // For ordering events
  duration?: number; // Duration in story time (minutes/hours/days)
  consequences: string[]; // IDs of events that result from this one
  prerequisites: string[]; // IDs of events that must happen before this one
}

interface Character {
  id: string;
  name: string;
}

interface WorldElement {
  id: string;
  name: string;
  element_type: string;
}

interface TimelineManagerProps {
  projectId: string;
  events: TimelineEvent[];
  characters: Character[];
  worldElements: WorldElement[];
  onCreateEvent: (event: Omit<TimelineEvent, 'id' | 'created_at' | 'updated_at'>) => Promise<void>;
  onUpdateEvent: (eventId: string, updates: Partial<TimelineEvent>) => Promise<void>;
  onDeleteEvent: (eventId: string) => Promise<void>;
  onReorderEvents: (eventIds: string[]) => Promise<void>;
}

interface TimelineFilter {
  eventType: string;
  importance: string;
  status: string;
  character: string;
  worldElement: string;
  tags: string[];
  dateRange: { start?: string; end?: string };
}

const EVENT_TYPES = [
  { value: 'plot', label: 'Plot Point', color: 'bg-blue-500' },
  { value: 'character', label: 'Character Arc', color: 'bg-green-500' },
  { value: 'world', label: 'World Event', color: 'bg-purple-500' },
  { value: 'conflict', label: 'Conflict', color: 'bg-red-500' },
  { value: 'resolution', label: 'Resolution', color: 'bg-yellow-500' },
  { value: 'custom', label: 'Custom', color: 'bg-gray-500' }
];

const IMPORTANCE_LEVELS = [
  { value: 'low', label: 'Low', color: 'bg-gray-400' },
  { value: 'medium', label: 'Medium', color: 'bg-blue-400' },
  { value: 'high', label: 'High', color: 'bg-orange-400' },
  { value: 'critical', label: 'Critical', color: 'bg-red-500' }
];

const STATUS_OPTIONS = [
  { value: 'planned', label: 'Planned', color: 'bg-gray-400' },
  { value: 'draft', label: 'Draft', color: 'bg-yellow-400' },
  { value: 'written', label: 'Written', color: 'bg-blue-400' },
  { value: 'revised', label: 'Revised', color: 'bg-green-400' }
];

const TimelineManager: React.FC<TimelineManagerProps> = ({
  projectId,
  events,
  characters,
  worldElements,
  onCreateEvent,
  onUpdateEvent,
  onDeleteEvent,
  onReorderEvents
}) => {
  const [view, setView] = useState<'timeline' | 'list' | 'gantt'>('timeline');
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [editingEvent, setEditingEvent] = useState<TimelineEvent | null>(null);
  const [filters, setFilters] = useState<TimelineFilter>({
    eventType: '',
    importance: '',
    status: '',
    character: '',
    worldElement: '',
    tags: [],
    dateRange: {}
  });
  const [searchQuery, setSearchQuery] = useState('');
  const [zoomLevel, setZoomLevel] = useState(1);
  const [selectedEvent, setSelectedEvent] = useState<TimelineEvent | null>(null);
  const [draggedEvent, setDraggedEvent] = useState<string | null>(null);
  const timelineRef = useRef<HTMLDivElement>(null);

  const [newEvent, setNewEvent] = useState<Partial<TimelineEvent>>({
    title: '',
    description: '',
    event_type: 'plot',
    importance: 'medium',
    status: 'planned',
    characters_involved: [],
    world_elements_involved: [],
    tags: [],
    notes: '',
    consequences: [],
    prerequisites: [],
    position_index: events.length
  });

  // Filter and search events
  const filteredEvents = events.filter(event => {
    if (searchQuery && !event.title.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !event.description.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }
    if (filters.eventType && event.event_type !== filters.eventType) return false;
    if (filters.importance && event.importance !== filters.importance) return false;
    if (filters.status && event.status !== filters.status) return false;
    if (filters.character && !event.characters_involved.includes(filters.character)) return false;
    if (filters.worldElement && !event.world_elements_involved.includes(filters.worldElement)) return false;
    if (filters.tags.length > 0 && !filters.tags.some(tag => event.tags.includes(tag))) return false;
    return true;
  }).sort((a, b) => a.position_index - b.position_index);

  const handleCreateEvent = async () => {
    if (!newEvent.title?.trim()) {
      alert('Please enter an event title');
      return;
    }

    try {
      await onCreateEvent({
        ...newEvent,
        title: newEvent.title!,
        description: newEvent.description || '',
        event_type: newEvent.event_type || 'plot',
        importance: newEvent.importance || 'medium',
        status: newEvent.status || 'planned',
        characters_involved: newEvent.characters_involved || [],
        world_elements_involved: newEvent.world_elements_involved || [],
        tags: newEvent.tags || [],
        notes: newEvent.notes || '',
        consequences: newEvent.consequences || [],
        prerequisites: newEvent.prerequisites || [],
        position_index: newEvent.position_index || events.length
      });
      
      setShowCreateDialog(false);
      setNewEvent({
        title: '',
        description: '',
        event_type: 'plot',
        importance: 'medium',
        status: 'planned',
        characters_involved: [],
        world_elements_involved: [],
        tags: [],
        notes: '',
        consequences: [],
        prerequisites: [],
        position_index: events.length
      });
    } catch (error) {
      console.error('Failed to create event:', error);
      alert('Failed to create event');
    }
  };

  const handleUpdateEvent = async (eventId: string, updates: Partial<TimelineEvent>) => {
    try {
      await onUpdateEvent(eventId, updates);
      setEditingEvent(null);
    } catch (error) {
      console.error('Failed to update event:', error);
      alert('Failed to update event');
    }
  };

  const handleDeleteEvent = async (eventId: string) => {
    if (confirm('Are you sure you want to delete this event?')) {
      try {
        await onDeleteEvent(eventId);
        setSelectedEvent(null);
      } catch (error) {
        console.error('Failed to delete event:', error);
        alert('Failed to delete event');
      }
    }
  };

  const handleDragStart = (eventId: string) => {
    setDraggedEvent(eventId);
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const handleDrop = async (e: React.DragEvent, targetIndex: number) => {
    e.preventDefault();
    if (!draggedEvent) return;

    const draggedIndex = events.findIndex(event => event.id === draggedEvent);
    if (draggedIndex === -1 || draggedIndex === targetIndex) return;

    const reorderedEvents = [...events];
    const [draggedItem] = reorderedEvents.splice(draggedIndex, 1);
    reorderedEvents.splice(targetIndex, 0, draggedItem);

    const eventIds = reorderedEvents.map(event => event.id);
    await onReorderEvents(eventIds);
    setDraggedEvent(null);
  };

  const getEventTypeInfo = (type: string) => {
    return EVENT_TYPES.find(t => t.value === type) || EVENT_TYPES[0];
  };

  const getImportanceInfo = (importance: string) => {
    return IMPORTANCE_LEVELS.find(i => i.value === importance) || IMPORTANCE_LEVELS[1];
  };

  const getStatusInfo = (status: string) => {
    return STATUS_OPTIONS.find(s => s.value === status) || STATUS_OPTIONS[0];
  };

  const exportTimeline = () => {
    const data = {
      project_id: projectId,
      events: filteredEvents,
      exported_at: new Date().toISOString()
    };
    
    const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `timeline-${projectId}-${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900">Timeline Manager</h2>
          <p className="text-gray-600">
            Track story events, character arcs, and plot progression
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline" onClick={exportTimeline}>
            <Download className="h-4 w-4 mr-1" />
            Export
          </Button>
          <Dialog open={showCreateDialog} onOpenChange={setShowCreateDialog}>
            <DialogTrigger asChild>
              <Button>
                <Plus className="h-4 w-4 mr-1" />
                Add Event
              </Button>
            </DialogTrigger>
            <DialogContent className="max-w-2xl">
              <DialogHeader>
                <DialogTitle>Create Timeline Event</DialogTitle>
              </DialogHeader>
              <div className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Title *
                    </label>
                    <Input
                      value={newEvent.title || ''}
                      onChange={(e) => setNewEvent({ ...newEvent, title: e.target.value })}
                      placeholder="Event title"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Event Type
                    </label>
                    <Select 
                      value={newEvent.event_type || 'plot'} 
                      onValueChange={(value) => setNewEvent({ ...newEvent, event_type: value as any })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {EVENT_TYPES.map(type => (
                          <SelectItem key={type.value} value={type.value}>
                            {type.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                </div>
                
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Description
                  </label>
                  <Textarea
                    value={newEvent.description || ''}
                    onChange={(e) => setNewEvent({ ...newEvent, description: e.target.value })}
                    placeholder="Describe what happens in this event"
                    rows={3}
                  />
                </div>

                <div className="grid grid-cols-3 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Importance
                    </label>
                    <Select 
                      value={newEvent.importance || 'medium'} 
                      onValueChange={(value) => setNewEvent({ ...newEvent, importance: value as any })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {IMPORTANCE_LEVELS.map(level => (
                          <SelectItem key={level.value} value={level.value}>
                            {level.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Status
                    </label>
                    <Select 
                      value={newEvent.status || 'planned'} 
                      onValueChange={(value) => setNewEvent({ ...newEvent, status: value as any })}
                    >
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {STATUS_OPTIONS.map(status => (
                          <SelectItem key={status.value} value={status.value}>
                            {status.label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Story Date/Time
                    </label>
                    <Input
                      value={newEvent.date_in_story || ''}
                      onChange={(e) => setNewEvent({ ...newEvent, date_in_story: e.target.value })}
                      placeholder="e.g., Day 1, Chapter 3"
                    />
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Chapter Reference
                    </label>
                    <Input
                      value={newEvent.chapter_reference || ''}
                      onChange={(e) => setNewEvent({ ...newEvent, chapter_reference: e.target.value })}
                      placeholder="Chapter number or name"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Scene Reference
                    </label>
                    <Input
                      value={newEvent.scene_reference || ''}
                      onChange={(e) => setNewEvent({ ...newEvent, scene_reference: e.target.value })}
                      placeholder="Scene number or name"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Notes
                  </label>
                  <Textarea
                    value={newEvent.notes || ''}
                    onChange={(e) => setNewEvent({ ...newEvent, notes: e.target.value })}
                    placeholder="Additional notes about this event"
                    rows={2}
                  />
                </div>

                <div className="flex justify-end gap-2">
                  <Button variant="outline" onClick={() => setShowCreateDialog(false)}>
                    Cancel
                  </Button>
                  <Button onClick={handleCreateEvent}>
                    Create Event
                  </Button>
                </div>
              </div>
            </DialogContent>
          </Dialog>
        </div>
      </div>

      {/* Filters and Search */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Filter className="h-5 w-5" />
            Filters & Search
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
            <div>
              <Input
                placeholder="Search events..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full"
              />
            </div>
            <Select value={filters.eventType} onValueChange={(value) => setFilters({ ...filters, eventType: value })}>
              <SelectTrigger>
                <SelectValue placeholder="Event Type" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All Types</SelectItem>
                {EVENT_TYPES.map(type => (
                  <SelectItem key={type.value} value={type.value}>
                    {type.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Select value={filters.importance} onValueChange={(value) => setFilters({ ...filters, importance: value })}>
              <SelectTrigger>
                <SelectValue placeholder="Importance" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All Levels</SelectItem>
                {IMPORTANCE_LEVELS.map(level => (
                  <SelectItem key={level.value} value={level.value}>
                    {level.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Select value={filters.status} onValueChange={(value) => setFilters({ ...filters, status: value })}>
              <SelectTrigger>
                <SelectValue placeholder="Status" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All Statuses</SelectItem>
                {STATUS_OPTIONS.map(status => (
                  <SelectItem key={status.value} value={status.value}>
                    {status.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Select value={filters.character} onValueChange={(value) => setFilters({ ...filters, character: value })}>
              <SelectTrigger>
                <SelectValue placeholder="Character" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">All Characters</SelectItem>
                {characters.map(character => (
                  <SelectItem key={character.id} value={character.id}>
                    {character.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            <div className="flex gap-2">
              <Button
                variant="outline"
                size="sm"
                onClick={() => setZoomLevel(Math.max(0.5, zoomLevel - 0.25))}
              >
                <ZoomOut className="h-4 w-4" />
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={() => setZoomLevel(Math.min(2, zoomLevel + 0.25))}
              >
                <ZoomIn className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* View Toggle */}
      <div className="flex space-x-1 bg-gray-100 p-1 rounded-lg w-fit">
        {[
          { id: 'timeline', label: 'Timeline', icon: Calendar },
          { id: 'list', label: 'List', icon: Clock },
          { id: 'gantt', label: 'Gantt', icon: Calendar }
        ].map(viewOption => {
          const Icon = viewOption.icon;
          return (
            <button
              key={viewOption.id}
              onClick={() => setView(viewOption.id as any)}
              className={`flex items-center gap-2 px-4 py-2 rounded-md text-sm font-medium transition-colors ${
                view === viewOption.id
                  ? 'bg-white text-gray-900 shadow-sm'
                  : 'text-gray-600 hover:text-gray-900'
              }`}
            >
              <Icon className="h-4 w-4" />
              {viewOption.label}
            </button>
          );
        })}
      </div>

      {/* Timeline View */}
      {view === 'timeline' && (
        <Card>
          <CardContent className="p-6">
            <div 
              ref={timelineRef}
              className="relative overflow-x-auto"
              style={{ transform: `scale(${zoomLevel})`, transformOrigin: 'top left' }}
            >
              {filteredEvents.length === 0 ? (
                <div className="text-center py-12">
                  <Calendar className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                  <h3 className="text-lg font-medium text-gray-900 mb-2">
                    No Events Found
                  </h3>
                  <p className="text-gray-600">
                    {events.length === 0 
                      ? 'Create your first timeline event to get started.'
                      : 'Try adjusting your filters or search query.'
                    }
                  </p>
                </div>
              ) : (
                <div className="space-y-4">
                  {filteredEvents.map((event, index) => {
                    const typeInfo = getEventTypeInfo(event.event_type);
                    const importanceInfo = getImportanceInfo(event.importance);
                    const statusInfo = getStatusInfo(event.status);
                    
                    return (
                      <div
                        key={event.id}
                        draggable
                        onDragStart={() => handleDragStart(event.id)}
                        onDragOver={handleDragOver}
                        onDrop={(e) => handleDrop(e, index)}
                        className={`relative flex items-start gap-4 p-4 border rounded-lg cursor-pointer transition-all hover:shadow-md ${
                          selectedEvent?.id === event.id ? 'ring-2 ring-blue-500 bg-blue-50' : 'bg-white'
                        }`}
                        onClick={() => setSelectedEvent(event)}
                      >
                        {/* Timeline Line */}
                        <div className="flex flex-col items-center">
                          <div className={`w-4 h-4 rounded-full ${typeInfo.color} border-2 border-white shadow-md`} />
                          {index < filteredEvents.length - 1 && (
                            <div className="w-0.5 h-16 bg-gray-300 mt-2" />
                          )}
                        </div>
                        
                        {/* Event Content */}
                        <div className="flex-1 min-w-0">
                          <div className="flex items-start justify-between">
                            <div className="flex-1">
                              <h3 className="font-medium text-gray-900">{event.title}</h3>
                              <p className="text-sm text-gray-600 mt-1">{event.description}</p>
                              
                              <div className="flex items-center gap-2 mt-2">
                                <Badge className={`${typeInfo.color} text-white`}>
                                  {typeInfo.label}
                                </Badge>
                                <Badge className={`${importanceInfo.color} text-white`}>
                                  {importanceInfo.label}
                                </Badge>
                                <Badge className={`${statusInfo.color} text-white`}>
                                  {statusInfo.label}
                                </Badge>
                                {event.date_in_story && (
                                  <Badge variant="outline">
                                    {event.date_in_story}
                                  </Badge>
                                )}
                              </div>
                              
                              {(event.characters_involved.length > 0 || event.world_elements_involved.length > 0) && (
                                <div className="mt-2 text-xs text-gray-500">
                                  {event.characters_involved.length > 0 && (
                                    <span>Characters: {event.characters_involved.join(', ')}</span>
                                  )}
                                  {event.characters_involved.length > 0 && event.world_elements_involved.length > 0 && ' • '}
                                  {event.world_elements_involved.length > 0 && (
                                    <span>Elements: {event.world_elements_involved.join(', ')}</span>
                                  )}
                                </div>
                              )}
                            </div>
                            
                            <div className="flex items-center gap-1 ml-4">
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  setEditingEvent(event);
                                }}
                              >
                                <Edit className="h-4 w-4" />
                              </Button>
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={(e) => {
                                  e.stopPropagation();
                                  handleDeleteEvent(event.id);
                                }}
                              >
                                <Trash2 className="h-4 w-4" />
                              </Button>
                            </div>
                          </div>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* List View */}
      {view === 'list' && (
        <Card>
          <CardContent className="p-6">
            {filteredEvents.length === 0 ? (
              <div className="text-center py-12">
                <Clock className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">
                  No Events Found
                </h3>
                <p className="text-gray-600">
                  {events.length === 0 
                    ? 'Create your first timeline event to get started.'
                    : 'Try adjusting your filters or search query.'
                  }
                </p>
              </div>
            ) : (
              <div className="space-y-3">
                {filteredEvents.map((event) => {
                  const typeInfo = getEventTypeInfo(event.event_type);
                  const importanceInfo = getImportanceInfo(event.importance);
                  const statusInfo = getStatusInfo(event.status);
                  
                  return (
                    <div
                      key={event.id}
                      className={`p-4 border rounded-lg cursor-pointer transition-all hover:shadow-md ${
                        selectedEvent?.id === event.id ? 'ring-2 ring-blue-500 bg-blue-50' : 'bg-white'
                      }`}
                      onClick={() => setSelectedEvent(event)}
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex-1">
                          <h3 className="font-medium text-gray-900">{event.title}</h3>
                          <p className="text-sm text-gray-600 mt-1">{event.description}</p>
                          
                          <div className="flex items-center gap-2 mt-2">
                            <Badge className={`${typeInfo.color} text-white`}>
                              {typeInfo.label}
                            </Badge>
                            <Badge className={`${importanceInfo.color} text-white`}>
                              {importanceInfo.label}
                            </Badge>
                            <Badge className={`${statusInfo.color} text-white`}>
                              {statusInfo.label}
                            </Badge>
                            {event.date_in_story && (
                              <Badge variant="outline">
                                {event.date_in_story}
                              </Badge>
                            )}
                            {event.chapter_reference && (
                              <Badge variant="outline">
                                Ch. {event.chapter_reference}
                              </Badge>
                            )}
                          </div>
                          
                          {event.notes && (
                            <p className="text-xs text-gray-500 mt-2">{event.notes}</p>
                          )}
                        </div>
                        
                        <div className="flex items-center gap-1 ml-4">
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={(e) => {
                              e.stopPropagation();
                              setEditingEvent(event);
                            }}
                          >
                            <Edit className="h-4 w-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={(e) => {
                              e.stopPropagation();
                              handleDeleteEvent(event.id);
                            }}
                          >
                            <Trash2 className="h-4 w-4" />
                          </Button>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </CardContent>
        </Card>
      )}

      {/* Gantt View */}
      {view === 'gantt' && (
        <Card>
          <CardContent className="p-6">
            <div className="text-center py-12">
              <Calendar className="h-12 w-12 text-gray-400 mx-auto mb-4" />
              <h3 className="text-lg font-medium text-gray-900 mb-2">
                Gantt View
              </h3>
              <p className="text-gray-600">
                Gantt chart visualization coming soon. This will show event dependencies and durations.
              </p>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Event Details Panel */}
      {selectedEvent && (
        <Card>
          <CardHeader>
            <CardTitle>Event Details</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div>
                <h3 className="font-medium text-gray-900">{selectedEvent.title}</h3>
                <p className="text-gray-600 mt-1">{selectedEvent.description}</p>
              </div>
              
              {selectedEvent.notes && (
                <div>
                  <h4 className="font-medium text-gray-900 mb-1">Notes</h4>
                  <p className="text-sm text-gray-600">{selectedEvent.notes}</p>
                </div>
              )}
              
              <div className="grid grid-cols-2 gap-4 text-sm">
                <div>
                  <span className="font-medium text-gray-700">Type:</span>
                  <span className="ml-2">{getEventTypeInfo(selectedEvent.event_type).label}</span>
                </div>
                <div>
                  <span className="font-medium text-gray-700">Importance:</span>
                  <span className="ml-2">{getImportanceInfo(selectedEvent.importance).label}</span>
                </div>
                <div>
                  <span className="font-medium text-gray-700">Status:</span>
                  <span className="ml-2">{getStatusInfo(selectedEvent.status).label}</span>
                </div>
                {selectedEvent.date_in_story && (
                  <div>
                    <span className="font-medium text-gray-700">Story Date:</span>
                    <span className="ml-2">{selectedEvent.date_in_story}</span>
                  </div>
                )}
                {selectedEvent.chapter_reference && (
                  <div>
                    <span className="font-medium text-gray-700">Chapter:</span>
                    <span className="ml-2">{selectedEvent.chapter_reference}</span>
                  </div>
                )}
                {selectedEvent.scene_reference && (
                  <div>
                    <span className="font-medium text-gray-700">Scene:</span>
                    <span className="ml-2">{selectedEvent.scene_reference}</span>
                  </div>
                )}
              </div>
              
              {(selectedEvent.characters_involved.length > 0 || selectedEvent.world_elements_involved.length > 0) && (
                <div>
                  <h4 className="font-medium text-gray-900 mb-2">Involved Elements</h4>
                  <div className="space-y-2">
                    {selectedEvent.characters_involved.length > 0 && (
                      <div>
                        <span className="text-sm font-medium text-gray-700">Characters:</span>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {selectedEvent.characters_involved.map(characterId => {
                            const character = characters.find(c => c.id === characterId);
                            return (
                              <Badge key={characterId} variant="outline">
                                {character?.name || characterId}
                              </Badge>
                            );
                          })}
                        </div>
                      </div>
                    )}
                    {selectedEvent.world_elements_involved.length > 0 && (
                      <div>
                        <span className="text-sm font-medium text-gray-700">World Elements:</span>
                        <div className="flex flex-wrap gap-1 mt-1">
                          {selectedEvent.world_elements_involved.map(elementId => {
                            const element = worldElements.find(e => e.id === elementId);
                            return (
                              <Badge key={elementId} variant="outline">
                                {element?.name || elementId}
                              </Badge>
                            );
                          })}
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              )}
              
              {selectedEvent.tags.length > 0 && (
                <div>
                  <h4 className="font-medium text-gray-900 mb-2">Tags</h4>
                  <div className="flex flex-wrap gap-1">
                    {selectedEvent.tags.map(tag => (
                      <Badge key={tag} variant="secondary">
                        {tag}
                      </Badge>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
};

export default TimelineManager;