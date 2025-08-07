<script lang="ts">
  import { onMount } from 'svelte';
  import { storyBibleStore, storyBibleActions, filteredScenes } from '../../../stores/storyBibleStore';
  import type { Scene, CreateSceneRequest, UpdateSceneRequest } from '../../../types/storyBible';
  
  import Button from '../../../components/ui/Button.svelte';
  import Input from '../../../components/ui/Input.svelte';
  import TextArea from '../../../components/ui/TextArea.svelte';
  import Select from '../../../components/ui/Select.svelte';
  import Card from '../../../components/ui/Card.svelte';
  import Modal from '../../../components/ui/Modal.svelte';
  import LoadingSpinner from '../../../components/ui/LoadingSpinner.svelte';
  import ErrorMessage from '../../../components/ui/ErrorMessage.svelte';
  
  export let projectId: string;
  export let seriesId: string | undefined = undefined;
  
  $: state = $storyBibleStore;
  $: scenes = $filteredScenes;
  
  // Modal state
  let showCreateModal = false;
  let showEditModal = false;
  let showDetailModal = false;
  let editingScene: Scene | null = null;
  let viewingScene: Scene | null = null;
  
  // Form state
  let createForm = {
    title: '',
    content: '',
    scene_type: '',
    chapter_number: null as number | null,
    scene_number: null as number | null,
    character_pov: '',
    location: '',
    time_of_day: '',
    mood: '',
    purpose: '',
    conflict: '',
    outcome: '',
    notes: '',
    word_count_target: null as number | null,
    status: 'planned' as 'planned' | 'drafted' | 'revised' | 'final',
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  let editForm = {
    id: '',
    title: '',
    content: '',
    scene_type: '',
    chapter_number: null as number | null,
    scene_number: null as number | null,
    character_pov: '',
    location: '',
    time_of_day: '',
    mood: '',
    purpose: '',
    conflict: '',
    outcome: '',
    notes: '',
    word_count_target: null as number | null,
    status: 'planned' as 'planned' | 'drafted' | 'revised' | 'final',
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  // Search state
  let searchQuery = '';
  
  // Available characters (mock data - in real app, this would come from characters store)
  let availableCharacters = [
    { id: '1', name: 'Main Character' },
    { id: '2', name: 'Antagonist' },
    { id: '3', name: 'Supporting Character' }
  ];
  
  // Scene type options
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
  
  // Status options
  const statusOptions = [
    { value: 'planned', label: 'Planned' },
    { value: 'drafted', label: 'Drafted' },
    { value: 'revised', label: 'Revised' },
    { value: 'final', label: 'Final' }
  ];
  
  // Visibility options
  const visibilityOptions = [
    { value: 'always', label: 'Always Visible' },
    { value: 'chapter', label: 'Chapter Context' },
    { value: 'never', label: 'Hidden' }
  ];
  
  // Time of day options
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
  
  // Mood options
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
  
  onMount(() => {
    loadScenes();
  });
  
  async function loadScenes() {
    await storyBibleActions.loadScenes(projectId, seriesId);
  }
  
  function openCreateModal() {
    createForm = {
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
    showCreateModal = true;
  }
  
  function openEditModal(scene: Scene) {
    editingScene = scene;
    editForm = {
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
    };
    showEditModal = true;
  }
  
  function openDetailModal(scene: Scene) {
    viewingScene = scene;
    showDetailModal = true;
  }
  
  function closeModals() {
    showCreateModal = false;
    showEditModal = false;
    showDetailModal = false;
    editingScene = null;
    viewingScene = null;
  }
  
  async function handleCreateScene() {
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
    
    await storyBibleActions.createScene(request);
    closeModals();
  }
  
  async function handleUpdateScene() {
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
    
    await storyBibleActions.updateScene(request);
    closeModals();
  }
  
  async function handleDeleteScene(sceneId: string) {
    if (confirm('Are you sure you want to delete this scene?')) {
      await storyBibleActions.deleteScene(sceneId);
    }
  }
  
  async function handleValidateScene(sceneId: string) {
    await storyBibleActions.validateScene(sceneId);
  }
  
  async function handleSearch() {
    if (searchQuery.trim()) {
      await storyBibleActions.searchScenes(projectId, searchQuery, seriesId);
    } else {
      await loadScenes();
    }
  }
  
  function handleFilterChange(filterType: string, value: any) {
    const currentFilter = state.sceneFilter;
    storyBibleActions.setSceneFilter({
      ...currentFilter,
      [filterType]: value || undefined
    });
  }
  
  function getSceneTypeLabel(sceneType: string): string {
    return sceneTypeOptions.find(opt => opt.value === sceneType)?.label || sceneType;
  }
  
  function getStatusLabel(status: string): string {
    return statusOptions.find(opt => opt.value === status)?.label || status;
  }
  
  function getVisibilityLabel(visibility: string): string {
    return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
  }
  
  function getTimeOfDayLabel(timeOfDay: string): string {
    return timeOfDayOptions.find(opt => opt.value === timeOfDay)?.label || timeOfDay;
  }
  
  function getMoodLabel(mood: string): string {
    return moodOptions.find(opt => opt.value === mood)?.label || mood;
  }
  
  function getCharacterName(characterId: string): string {
    return availableCharacters.find(char => char.id === characterId)?.name || characterId;
  }
  
  function getSceneIcon(sceneType: string): string {
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
  }
  
  function getStatusColor(status: string): string {
    const colors: Record<string, string> = {
      planned: '#6c757d',
      drafted: '#ffc107',
      revised: '#17a2b8',
      final: '#28a745'
    };
    return colors[status] || '#6c757d';
  }
  
  function formatSceneReference(scene: Scene): string {
    const parts = [];
    if (scene.chapter_number) parts.push(`Ch. ${scene.chapter_number}`);
    if (scene.scene_number) parts.push(`Scene ${scene.scene_number}`);
    if (scene.character_pov) parts.push(`POV: ${getCharacterName(scene.character_pov)}`);
    if (scene.location) parts.push(`@ ${scene.location}`);
    return parts.join(' • ');
  }
</script>

<div class="scenes-manager">
  <!-- Header -->
  <div class="manager-header">
    <div class="header-content">
      <h2>Story Scenes</h2>
      <p class="subtitle">
        Plan and track individual scenes with detailed breakdowns, character POVs, and story progression.
      </p>
    </div>
    
    <div class="header-actions">
      <Button variant="primary" on:click={openCreateModal}>
        <span class="icon">➕</span>
        Add Scene
      </Button>
    </div>
  </div>
  
  <!-- Search and Filters -->
  <Card title="Search & Filter" class="search-card">
    <div class="search-content">
      <div class="search-bar">
        <Input
          type="text"
          placeholder="Search scenes..."
          bind:value={searchQuery}
          on:input={handleSearch}
        />
        <Button variant="secondary" on:click={handleSearch}>
          🔍 Search
        </Button>
      </div>
      
      <div class="filters">
        <div class="filter-group">
          <label for="scene-type-filter">Filter by Type:</label>
          <Select
            id="scene-type-filter"
            value={state.sceneFilter.sceneType || ''}
            on:change={(e) => handleFilterChange('sceneType', e.detail)}
            options={[
              { value: '', label: 'All types' },
              ...sceneTypeOptions.slice(1)
            ]}
          />
        </div>
        
        <div class="filter-group">
          <label for="status-filter">Filter by Status:</label>
          <Select
            id="status-filter"
            value={state.sceneFilter.status || ''}
            on:change={(e) => handleFilterChange('status', e.detail)}
            options={[
              { value: '', label: 'All statuses' },
              ...statusOptions
            ]}
          />
        </div>
        
        <div class="filter-group">
          <label for="character-filter">Filter by Character POV:</label>
          <Select
            id="character-filter"
            value={state.sceneFilter.characterPov || ''}
            on:change={(e) => handleFilterChange('characterPov', e.detail)}
            options={[
              { value: '', label: 'All characters' },
              ...availableCharacters.map(char => ({ value: char.id, label: char.name }))
            ]}
          />
        </div>
        
        <div class="filter-group">
          <label for="chapter-filter">Filter by Chapter:</label>
          <Input
            id="chapter-filter"
            type="number"
            placeholder="Chapter #"
            value={state.sceneFilter.chapterNumber || ''}
            on:input={(e) => handleFilterChange('chapterNumber', e.target.value ? parseInt(e.target.value) : undefined)}
          />
        </div>
      </div>
    </div>
  </Card>
  
  <!-- Content Area -->
  <div class="content-area">
    {#if state.scenesError}
      <ErrorMessage 
        message={state.scenesError} 
        onDismiss={() => storyBibleActions.clearError()}
        type="error"
      />
    {/if}
    
    {#if state.isLoadingScenes}
      <div class="loading-container">
        <LoadingSpinner size="medium" />
        <p>Loading scenes...</p>
      </div>
    {:else if scenes.length === 0}
      <div class="empty-state">
        <span class="empty-icon">🎬</span>
        <h3>No Scenes</h3>
        <p>Start building your story by creating detailed scene breakdowns with character POVs, locations, and story progression.</p>
        <Button variant="primary" on:click={openCreateModal}>
          Create First Scene
        </Button>
      </div>
    {:else}
      <!-- Scenes List -->
      <div class="scenes-list">
        {#each scenes as scene (scene.id)}
          <Card class="scene-card">
            <div class="scene-header">
              <div class="scene-meta">
                <div class="scene-title">
                  <span class="scene-icon">{getSceneIcon(scene.scene_type)}</span>
                  <div class="title-content">
                    <h4 class="scene-name">{scene.title}</h4>
                    <div class="scene-reference">
                      {formatSceneReference(scene)}
                    </div>
                  </div>
                </div>
                <div class="scene-badges">
                  <span class="type-badge">{getSceneTypeLabel(scene.scene_type)}</span>
                  <span class="status-badge" style="background-color: {getStatusColor(scene.status)}20; color: {getStatusColor(scene.status)}">
                    {getStatusLabel(scene.status)}
                  </span>
                  {#if scene.mood}
                    <span class="mood-badge">{getMoodLabel(scene.mood)}</span>
                  {/if}
                  {#if scene.time_of_day}
                    <span class="time-badge">{getTimeOfDayLabel(scene.time_of_day)}</span>
                  {/if}
                  {#if scene.series_shared}
                    <span class="series-badge">Series Shared</span>
                  {/if}
                </div>
              </div>
              
              <div class="scene-actions">
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => handleValidateScene(scene.id)}
                  title="Validate Scene"
                >
                  ✓
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openDetailModal(scene)}
                  title="View Details"
                >
                  👁️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openEditModal(scene)}
                  title="Edit"
                >
                  ✏️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => handleDeleteScene(scene.id)}
                  title="Delete"
                >
                  🗑️
                </Button>
              </div>
            </div>
            
            <div class="scene-content">
              <p class="scene-preview">{scene.content.substring(0, 200)}{scene.content.length > 200 ? '...' : ''}</p>
              
              {#if scene.purpose || scene.conflict || scene.outcome}
                <div class="scene-details">
                  {#if scene.purpose}
                    <div class="detail-item">
                      <strong>Purpose:</strong> {scene.purpose}
                    </div>
                  {/if}
                  {#if scene.conflict}
                    <div class="detail-item">
                      <strong>Conflict:</strong> {scene.conflict}
                    </div>
                  {/if}
                  {#if scene.outcome}
                    <div class="detail-item">
                      <strong>Outcome:</strong> {scene.outcome}
                    </div>
                  {/if}
                </div>
              {/if}
            </div>
            
            <div class="scene-footer">
              <div class="scene-stats">
                {#if scene.word_count_target}
                  <span class="stat-item">Target: {scene.word_count_target} words</span>
                {/if}
                <span class="scene-date">
                  Updated {new Date(scene.updated_at).toLocaleDateString()}
                </span>
              </div>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Scene Modal -->
<Modal bind:show={showCreateModal} title="Add Scene" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="create-title">Title:</label>
      <Input
        id="create-title"
        bind:value={createForm.title}
        placeholder="Enter scene title..."
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="create-scene-type">Scene Type:</label>
        <Select
          id="create-scene-type"
          bind:value={createForm.scene_type}
          options={sceneTypeOptions}
        />
      </div>
      
      <div class="form-group">
        <label for="create-status">Status:</label>
        <Select
          id="create-status"
          bind:value={createForm.status}
          options={statusOptions}
        />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="create-chapter">Chapter Number:</label>
        <Input
          id="create-chapter"
          type="number"
          bind:value={createForm.chapter_number}
          placeholder="Chapter #"
        />
      </div>
      
      <div class="form-group">
        <label for="create-scene-number">Scene Number:</label>
        <Input
          id="create-scene-number"
          type="number"
          bind:value={createForm.scene_number}
          placeholder="Scene #"
        />
      </div>
      
      <div class="form-group">
        <label for="create-word-target">Word Count Target:</label>
        <Input
          id="create-word-target"
          type="number"
          bind:value={createForm.word_count_target}
          placeholder="Target words"
        />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="create-character-pov">Character POV:</label>
        <Select
          id="create-character-pov"
          bind:value={createForm.character_pov}
          options={[
            { value: '', label: 'No specific POV' },
            ...availableCharacters.map(char => ({ value: char.id, label: char.name }))
          ]}
        />
      </div>
      
      <div class="form-group">
        <label for="create-location">Location:</label>
        <Input
          id="create-location"
          bind:value={createForm.location}
          placeholder="Scene location..."
        />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="create-time">Time of Day:</label>
        <Select
          id="create-time"
          bind:value={createForm.time_of_day}
          options={timeOfDayOptions}
        />
      </div>
      
      <div class="form-group">
        <label for="create-mood">Mood:</label>
        <Select
          id="create-mood"
          bind:value={createForm.mood}
          options={moodOptions}
        />
      </div>
    </div>
    
    <div class="form-group">
      <label for="create-purpose">Scene Purpose:</label>
      <Input
        id="create-purpose"
        bind:value={createForm.purpose}
        placeholder="What does this scene accomplish?"
      />
    </div>
    
    <div class="form-group">
      <label for="create-conflict">Conflict:</label>
      <Input
        id="create-conflict"
        bind:value={createForm.conflict}
        placeholder="What conflict drives this scene?"
      />
    </div>
    
    <div class="form-group">
      <label for="create-outcome">Outcome:</label>
      <Input
        id="create-outcome"
        bind:value={createForm.outcome}
        placeholder="How does the scene end?"
      />
    </div>
    
    <div class="form-group">
      <label for="create-content">Scene Content:</label>
      <TextArea
        id="create-content"
        bind:value={createForm.content}
        placeholder="Write your scene content or detailed breakdown..."
        rows={6}
      />
    </div>
    
    <div class="form-group">
      <label for="create-notes">Notes:</label>
      <TextArea
        id="create-notes"
        bind:value={createForm.notes}
        placeholder="Additional notes or reminders..."
        rows={3}
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="create-visibility">Visibility:</label>
        <Select
          id="create-visibility"
          bind:value={createForm.visibility}
          options={visibilityOptions}
        />
      </div>
      
      <div class="form-group">
        <label class="checkbox-label">
          <input 
            type="checkbox" 
            bind:checked={createForm.series_shared}
          />
          Share across series
        </label>
      </div>
    </div>
  </div>
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Cancel
    </Button>
    <Button 
      variant="primary" 
      on:click={handleCreateScene}
      disabled={!createForm.title || !createForm.scene_type || !createForm.content}
    >
      Add Scene
    </Button>
  </div>
</Modal>

<!-- Edit Scene Modal -->
<Modal bind:show={showEditModal} title="Edit Scene" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="edit-title">Title:</label>
      <Input
        id="edit-title"
        bind:value={editForm.title}
        placeholder="Enter scene title..."
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="edit-scene-type">Scene Type:</label>
        <Select
          id="edit-scene-type"
          bind:value={editForm.scene_type}
          options={sceneTypeOptions}
        />
      </div>
      
      <div class="form-group">
        <label for="edit-status">Status:</label>
        <Select
          id="edit-status"
          bind:value={editForm.status}
          options={statusOptions}
        />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="edit-chapter">Chapter Number:</label>
        <Input
          id="edit-chapter"
          type="number"
          bind:value={editForm.chapter_number}
          placeholder="Chapter #"
        />
      </div>
      
      <div class="form-group">
        <label for="edit-scene-number">Scene Number:</label>
        <Input
          id="edit-scene-number"
          type="number"
          bind:value={editForm.scene_number}
          placeholder="Scene #"
        />
      </div>
      
      <div class="form-group">
        <label for="edit-word-target">Word Count Target:</label>
        <Input
          id="edit-word-target"
          type="number"
          bind:value={editForm.word_count_target}
          placeholder="Target words"
        />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="edit-character-pov">Character POV:</label>
        <Select
          id="edit-character-pov"
          bind:value={editForm.character_pov}
          options={[
            { value: '', label: 'No specific POV' },
            ...availableCharacters.map(char => ({ value: char.id, label: char.name }))
          ]}
        />
      </div>
      
      <div class="form-group">
        <label for="edit-location">Location:</label>
        <Input
          id="edit-location"
          bind:value={editForm.location}
          placeholder="Scene location..."
        />
      </div>
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="edit-time">Time of Day:</label>
        <Select
          id="edit-time"
          bind:value={editForm.time_of_day}
          options={timeOfDayOptions}
        />
      </div>
      
      <div class="form-group">
        <label for="edit-mood">Mood:</label>
        <Select
          id="edit-mood"
          bind:value={editForm.mood}
          options={moodOptions}
        />
      </div>
    </div>
    
    <div class="form-group">
      <label for="edit-purpose">Scene Purpose:</label>
      <Input
        id="edit-purpose"
        bind:value={editForm.purpose}
        placeholder="What does this scene accomplish?"
      />
    </div>
    
    <div class="form-group">
      <label for="edit-conflict">Conflict:</label>
      <Input
        id="edit-conflict"
        bind:value={editForm.conflict}
        placeholder="What conflict drives this scene?"
      />
    </div>
    
    <div class="form-group">
      <label for="edit-outcome">Outcome:</label>
      <Input
        id="edit-outcome"
        bind:value={editForm.outcome}
        placeholder="How does the scene end?"
      />
    </div>
    
    <div class="form-group">
      <label for="edit-content">Scene Content:</label>
      <TextArea
        id="edit-content"
        bind:value={editForm.content}
        placeholder="Write your scene content or detailed breakdown..."
        rows={6}
      />
    </div>
    
    <div class="form-group">
      <label for="edit-notes">Notes:</label>
      <TextArea
        id="edit-notes"
        bind:value={editForm.notes}
        placeholder="Additional notes or reminders..."
        rows={3}
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="edit-visibility">Visibility:</label>
        <Select
          id="edit-visibility"
          bind:value={editForm.visibility}
          options={visibilityOptions}
        />
      </div>
      
      <div class="form-group">
        <label class="checkbox-label">
          <input 
            type="checkbox" 
            bind:checked={editForm.series_shared}
          />
          Share across series
        </label>
      </div>
    </div>
  </div>
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Cancel
    </Button>
    <Button 
      variant="primary" 
      on:click={handleUpdateScene}
      disabled={!editForm.title || !editForm.scene_type || !editForm.content}
    >
      Save Changes
    </Button>
  </div>
</Modal>

<!-- Detail View Modal -->
<Modal bind:show={showDetailModal} title={viewingScene?.title || 'Scene Details'} on:close={closeModals}>
  {#if viewingScene}
    <div class="detail-view">
      <div class="detail-header">
        <div class="detail-title">
          <span class="detail-icon">{getSceneIcon(viewingScene.scene_type)}</span>
          <div>
            <h3>{viewingScene.title}</h3>
            <div class="detail-meta">
              <span class="detail-type">{getSceneTypeLabel(viewingScene.scene_type)}</span>
              {#if formatSceneReference(viewingScene)}
                <span class="detail-reference">{formatSceneReference(viewingScene)}</span>
              {/if}
            </div>
          </div>
        </div>
        
        <div class="detail-badges">
          <span class="status-badge" style="background-color: {getStatusColor(viewingScene.status)}20; color: {getStatusColor(viewingScene.status)}">
            {getStatusLabel(viewingScene.status)}
          </span>
          {#if viewingScene.mood}
            <span class="mood-badge">{getMoodLabel(viewingScene.mood)}</span>
          {/if}
          {#if viewingScene.series_shared}
            <span class="series-badge">Series Shared</span>
          {/if}
        </div>
      </div>
      
      <div class="detail-content">
        {#if viewingScene.purpose || viewingScene.conflict || viewingScene.outcome}
          <div class="detail-section">
            <h4>Scene Structure</h4>
            <div class="structure-grid">
              {#if viewingScene.purpose}
                <div class="structure-item">
                  <strong>Purpose:</strong>
                  <p>{viewingScene.purpose}</p>
                </div>
              {/if}
              {#if viewingScene.conflict}
                <div class="structure-item">
                  <strong>Conflict:</strong>
                  <p>{viewingScene.conflict}</p>
                </div>
              {/if}
              {#if viewingScene.outcome}
                <div class="structure-item">
                  <strong>Outcome:</strong>
                  <p>{viewingScene.outcome}</p>
                </div>
              {/if}
            </div>
          </div>
        {/if}
        
        <div class="detail-section">
          <h4>Scene Content</h4>
          <div class="content-text">{viewingScene.content}</div>
        </div>
        
        {#if viewingScene.notes}
          <div class="detail-section">
            <h4>Notes</h4>
            <div class="notes-text">{viewingScene.notes}</div>
          </div>
        {/if}
        
        <div class="detail-meta-info">
          <div class="meta-grid">
            {#if viewingScene.location}
              <div class="meta-item">
                <strong>Location:</strong> {viewingScene.location}
              </div>
            {/if}
            {#if viewingScene.time_of_day}
              <div class="meta-item">
                <strong>Time:</strong> {getTimeOfDayLabel(viewingScene.time_of_day)}
              </div>
            {/if}
            {#if viewingScene.character_pov}
              <div class="meta-item">
                <strong>POV Character:</strong> {getCharacterName(viewingScene.character_pov)}
              </div>
            {/if}
            {#if viewingScene.word_count_target}
              <div class="meta-item">
                <strong>Target Word Count:</strong> {viewingScene.word_count_target}
              </div>
            {/if}
            <div class="meta-item">
              <strong>Created:</strong> {new Date(viewingScene.created_at).toLocaleDateString()}
            </div>
            <div class="meta-item">
              <strong>Last Updated:</strong> {new Date(viewingScene.updated_at).toLocaleDateString()}
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Close
    </Button>
    {#if viewingScene}
      <Button variant="primary" on:click={() => { closeModals(); openEditModal(viewingScene); }}>
        Edit Scene
      </Button>
    {/if}
  </div>
</Modal>

<style>
  .scenes-manager {
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
  
  .search-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .search-bar {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }
  
  .search-bar :global(.input) {
    flex: 1;
  }
  
  .filters {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }
  
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 150px;
  }
  
  .filter-group label {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary);
  }
  
  .content-area {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }
  
  .scenes-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .scene-card {
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    overflow: hidden;
  }
  
  .scene-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1rem 1rem 0.5rem 1rem;
  }
  
  .scene-meta {
    flex: 1;
  }
  
  .scene-title {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }
  
  .scene-icon {
    font-size: 1.5rem;
    margin-top: 0.125rem;
  }
  
  .title-content {
    flex: 1;
  }
  
  .scene-name {
    margin: 0 0 0.25rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .scene-reference {
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-style: italic;
  }
  
  .scene-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  
  .type-badge,
  .status-badge,
  .mood-badge,
  .time-badge,
  .series-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
  }
  
  .type-badge {
    background: #e3f2fd;
    color: #1565c0;
  }
  
  .mood-badge {
    background: #f3e5f5;
    color: #7b1fa2;
  }
  
  .time-badge {
    background: #fff3e0;
    color: #ef6c00;
  }
  
  .series-badge {
    background: #cce5ff;
    color: #004085;
  }
  
  .scene-actions {
    display: flex;
    gap: 0.25rem;
  }
  
  .scene-content {
    padding: 0 1rem 1rem 1rem;
  }
  
  .scene-preview {
    margin: 0 0 1rem 0;
    line-height: 1.6;
    color: var(--text-primary);
  }
  
  .scene-details {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    padding: 1rem;
    background: var(--bg-tertiary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }
  
  .detail-item {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }
  
  .detail-item strong {
    color: var(--text-primary);
  }
  
  .scene-footer {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border-color);
    background: var(--bg-tertiary);
  }
  
  .scene-stats {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.8rem;
    color: var(--text-secondary);
  }
  
  .stat-item {
    font-weight: 500;
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
    color: var(--text-secondary);
  }
  
  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }
  
  .empty-state h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }
  
  .empty-state p {
    margin: 0 0 1.5rem 0;
    max-width: 400px;
  }
  
  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    gap: 1rem;
    color: var(--text-secondary);
  }
  
  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .form-group label {
    font-weight: 500;
    color: var(--text-primary);
  }
  
  .form-row {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }
  
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    margin-top: 1.5rem;
  }
  
  .modal-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }
  
  .detail-view {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .detail-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border-color);
  }
  
  .detail-title {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    flex: 1;
  }
  
  .detail-icon {
    font-size: 2rem;
  }
  
  .detail-title h3 {
    margin: 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }
  
  .detail-meta {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin-top: 0.25rem;
  }
  
  .detail-type {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }
  
  .detail-reference {
    color: var(--text-secondary);
    font-size: 0.85rem;
    font-style: italic;
  }
  
  .detail-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  
  .detail-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .detail-section h4 {
    margin: 0 0 0.75rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .structure-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }
  
  .structure-item {
    padding: 1rem;
    background: var(--bg-tertiary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }
  
  .structure-item strong {
    display: block;
    margin-bottom: 0.5rem;
    color: var(--text-primary);
    font-size: 0.9rem;
  }
  
  .structure-item p {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.9rem;
    line-height: 1.5;
  }
  
  .content-text,
  .notes-text {
    line-height: 1.6;
    color: var(--text-primary);
    white-space: pre-wrap;
  }
  
  .notes-text {
    padding: 1rem;
    background: var(--bg-tertiary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
    font-style: italic;
  }
  
  .detail-meta-info {
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }
  
  .meta-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }
  
  .meta-item {
    font-size: 0.9rem;
    color: var(--text-secondary);
  }
  
  .meta-item strong {
    color: var(--text-primary);
  }
  
  .icon {
    margin-right: 0.5rem;
  }
  
  /* Responsive Design */
  @media (max-width: 768px) {
    .manager-header {
      flex-direction: column;
      gap: 1rem;
      padding: 1.5rem;
    }
    
    .content-area {
      padding: 1.5rem;
    }
    
    .filters {
      flex-direction: column;
    }
    
    .form-row {
      grid-template-columns: 1fr;
    }
    
    .search-bar {
      flex-direction: column;
    }
    
    .detail-header {
      flex-direction: column;
      gap: 1rem;
    }
    
    .meta-grid,
    .structure-grid {
      grid-template-columns: 1fr;
    }
    
    .scene-title {
      flex-direction: column;
      gap: 0.5rem;
    }
    
    .scene-stats {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>