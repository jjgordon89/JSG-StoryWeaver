<script lang="ts">
  import { onMount } from 'svelte';
  import { storyBibleStore, storyBibleActions, filteredOutlines } from '../../../stores/storyBibleStore';
  import type { Outline, CreateOutlineRequest, UpdateOutlineRequest } from '../../../types/storyBible';
  
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
  $: outlines = $filteredOutlines;
  
  // Modal state
  let showCreateModal = false;
  let showEditModal = false;
  let showDetailModal = false;
  let editingOutline: Outline | null = null;
  let viewingOutline: Outline | null = null;
  
  // Form state
  let createForm = {
    title: '',
    content: '',
    outline_type: '',
    chapter_number: null as number | null,
    character_pov: '',
    act_number: null as number | null,
    scene_number: null as number | null,
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  let editForm = {
    id: '',
    title: '',
    content: '',
    outline_type: '',
    chapter_number: null as number | null,
    character_pov: '',
    act_number: null as number | null,
    scene_number: null as number | null,
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
  
  // Outline type options
  const outlineTypeOptions = [
    { value: '', label: 'Select outline type' },
    { value: 'chapter', label: 'Chapter Outline' },
    { value: 'scene', label: 'Scene Outline' },
    { value: 'act', label: 'Act Outline' },
    { value: 'character_arc', label: 'Character Arc' },
    { value: 'plot_thread', label: 'Plot Thread' },
    { value: 'subplot', label: 'Subplot' },
    { value: 'theme', label: 'Theme Development' },
    { value: 'conflict', label: 'Conflict Resolution' },
    { value: 'pacing', label: 'Pacing Notes' },
    { value: 'structure', label: 'Story Structure' },
    { value: 'other', label: 'Other' }
  ];
  
  // Visibility options
  const visibilityOptions = [
    { value: 'always', label: 'Always Visible' },
    { value: 'chapter', label: 'Chapter Context' },
    { value: 'never', label: 'Hidden' }
  ];
  
  onMount(() => {
    loadOutlines();
  });
  
  async function loadOutlines() {
    await storyBibleActions.loadOutlines(projectId, seriesId);
  }
  
  function openCreateModal() {
    createForm = {
      title: '',
      content: '',
      outline_type: '',
      chapter_number: null,
      character_pov: '',
      act_number: null,
      scene_number: null,
      visibility: 'always',
      series_shared: false
    };
    showCreateModal = true;
  }
  
  function openEditModal(outline: Outline) {
    editingOutline = outline;
    editForm = {
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
    };
    showEditModal = true;
  }
  
  function openDetailModal(outline: Outline) {
    viewingOutline = outline;
    showDetailModal = true;
  }
  
  function closeModals() {
    showCreateModal = false;
    showEditModal = false;
    showDetailModal = false;
    editingOutline = null;
    viewingOutline = null;
  }
  
  async function handleCreateOutline() {
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
    
    await storyBibleActions.createOutline(request);
    closeModals();
  }
  
  async function handleUpdateOutline() {
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
    
    await storyBibleActions.updateOutline(request);
    closeModals();
  }
  
  async function handleDeleteOutline(outlineId: string) {
    if (confirm('Are you sure you want to delete this outline?')) {
      await storyBibleActions.deleteOutline(outlineId);
    }
  }
  
  async function handleSearch() {
    if (searchQuery.trim()) {
      await storyBibleActions.searchOutlines(projectId, searchQuery, seriesId);
    } else {
      await loadOutlines();
    }
  }
  
  function handleFilterChange(filterType: string, value: any) {
    const currentFilter = state.outlineFilter;
    storyBibleActions.setOutlineFilter({
      ...currentFilter,
      [filterType]: value || undefined
    });
  }
  
  function getOutlineTypeLabel(outlineType: string): string {
    return outlineTypeOptions.find(opt => opt.value === outlineType)?.label || outlineType;
  }
  
  function getVisibilityLabel(visibility: string): string {
    return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
  }
  
  function getCharacterName(characterId: string): string {
    return availableCharacters.find(char => char.id === characterId)?.name || characterId;
  }
  
  function getOutlineIcon(outlineType: string): string {
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
  }
  
  function formatOutlineReference(outline: Outline): string {
    const parts = [];
    if (outline.act_number) parts.push(`Act ${outline.act_number}`);
    if (outline.chapter_number) parts.push(`Ch. ${outline.chapter_number}`);
    if (outline.scene_number) parts.push(`Scene ${outline.scene_number}`);
    if (outline.character_pov) parts.push(`POV: ${getCharacterName(outline.character_pov)}`);
    return parts.join(' • ');
  }
</script>

<div class="outline-manager">
  <!-- Header -->
  <div class="manager-header">
    <div class="header-content">
      <h2>Story Outline</h2>
      <p class="subtitle">
        Plan and organize your story structure, chapters, scenes, and character arcs.
      </p>
    </div>
    
    <div class="header-actions">
      <Button variant="primary" on:click={openCreateModal}>
        <span class="icon">➕</span>
        Add Outline
      </Button>
    </div>
  </div>
  
  <!-- Search and Filters -->
  <Card title="Search & Filter" class="search-card">
    <div class="search-content">
      <div class="search-bar">
        <Input
          type="text"
          placeholder="Search outlines..."
          bind:value={searchQuery}
          on:input={handleSearch}
        />
        <Button variant="secondary" on:click={handleSearch}>
          🔍 Search
        </Button>
      </div>
      
      <div class="filters">
        <div class="filter-group">
          <label for="outline-type-filter">Filter by Type:</label>
          <Select
            id="outline-type-filter"
            value={state.outlineFilter.outlineType || ''}
            on:change={(e) => handleFilterChange('outlineType', e.detail)}
            options={[
              { value: '', label: 'All types' },
              ...outlineTypeOptions.slice(1)
            ]}
          />
        </div>
        
        <div class="filter-group">
          <label for="character-filter">Filter by Character POV:</label>
          <Select
            id="character-filter"
            value={state.outlineFilter.characterPov || ''}
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
            value={state.outlineFilter.chapterNumber || ''}
            on:input={(e) => handleFilterChange('chapterNumber', e.target.value ? parseInt(e.target.value) : undefined)}
          />
        </div>
        
        <div class="filter-group">
          <label for="visibility-filter">Filter by Visibility:</label>
          <Select
            id="visibility-filter"
            value={state.outlineFilter.visibility || ''}
            on:change={(e) => handleFilterChange('visibility', e.detail)}
            options={[
              { value: '', label: 'All visibility' },
              ...visibilityOptions
            ]}
          />
        </div>
      </div>
    </div>
  </Card>
  
  <!-- Content Area -->
  <div class="content-area">
    {#if state.outlinesError}
      <ErrorMessage 
        message={state.outlinesError} 
        onDismiss={() => storyBibleActions.clearError()}
        type="error"
      />
    {/if}
    
    {#if state.isLoadingOutlines}
      <div class="loading-container">
        <LoadingSpinner size="medium" />
        <p>Loading outlines...</p>
      </div>
    {:else if outlines.length === 0}
      <div class="empty-state">
        <span class="empty-icon">📝</span>
        <h3>No Outlines</h3>
        <p>Start planning your story by creating chapter outlines, scene breakdowns, and character arcs.</p>
        <Button variant="primary" on:click={openCreateModal}>
          Create First Outline
        </Button>
      </div>
    {:else}
      <!-- Outlines List -->
      <div class="outlines-list">
        {#each outlines as outline (outline.id)}
          <Card class="outline-card">
            <div class="outline-header">
              <div class="outline-meta">
                <div class="outline-title">
                  <span class="outline-icon">{getOutlineIcon(outline.outline_type)}</span>
                  <div class="title-content">
                    <h4 class="outline-name">{outline.title}</h4>
                    <div class="outline-reference">
                      {formatOutlineReference(outline)}
                    </div>
                  </div>
                </div>
                <div class="outline-badges">
                  <span class="type-badge">{getOutlineTypeLabel(outline.outline_type)}</span>
                  <span class="visibility-badge" class:always={outline.visibility === 'always'} class:chapter={outline.visibility === 'chapter'} class:never={outline.visibility === 'never'}>
                    {getVisibilityLabel(outline.visibility)}
                  </span>
                  {#if outline.series_shared}
                    <span class="series-badge">Series Shared</span>
                  {/if}
                </div>
              </div>
              
              <div class="outline-actions">
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openDetailModal(outline)}
                  title="View Details"
                >
                  👁️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openEditModal(outline)}
                  title="Edit"
                >
                  ✏️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => handleDeleteOutline(outline.id)}
                  title="Delete"
                >
                  🗑️
                </Button>
              </div>
            </div>
            
            <div class="outline-content">
              <p class="outline-preview">{outline.content.substring(0, 200)}{outline.content.length > 200 ? '...' : ''}</p>
            </div>
            
            <div class="outline-footer">
              <span class="outline-date">
                Updated {new Date(outline.updated_at).toLocaleDateString()}
              </span>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Outline Modal -->
<Modal bind:show={showCreateModal} title="Add Outline" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="create-title">Title:</label>
      <Input
        id="create-title"
        bind:value={createForm.title}
        placeholder="Enter outline title..."
      />
    </div>
    
    <div class="form-group">
      <label for="create-outline-type">Outline Type:</label>
      <Select
        id="create-outline-type"
        bind:value={createForm.outline_type}
        options={outlineTypeOptions}
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="create-act">Act Number:</label>
        <Input
          id="create-act"
          type="number"
          bind:value={createForm.act_number}
          placeholder="Act #"
        />
      </div>
      
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
        <label for="create-scene">Scene Number:</label>
        <Input
          id="create-scene"
          type="number"
          bind:value={createForm.scene_number}
          placeholder="Scene #"
        />
      </div>
    </div>
    
    <div class="form-group">
      <label for="create-character-pov">Character POV (Optional):</label>
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
      <label for="create-content">Content:</label>
      <TextArea
        id="create-content"
        bind:value={createForm.content}
        placeholder="Write your outline content..."
        rows={6}
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
      on:click={handleCreateOutline}
      disabled={!createForm.title || !createForm.outline_type || !createForm.content}
    >
      Add Outline
    </Button>
  </div>
</Modal>

<!-- Edit Outline Modal -->
<Modal bind:show={showEditModal} title="Edit Outline" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="edit-title">Title:</label>
      <Input
        id="edit-title"
        bind:value={editForm.title}
        placeholder="Enter outline title..."
      />
    </div>
    
    <div class="form-group">
      <label for="edit-outline-type">Outline Type:</label>
      <Select
        id="edit-outline-type"
        bind:value={editForm.outline_type}
        options={outlineTypeOptions}
      />
    </div>
    
    <div class="form-row">
      <div class="form-group">
        <label for="edit-act">Act Number:</label>
        <Input
          id="edit-act"
          type="number"
          bind:value={editForm.act_number}
          placeholder="Act #"
        />
      </div>
      
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
        <label for="edit-scene">Scene Number:</label>
        <Input
          id="edit-scene"
          type="number"
          bind:value={editForm.scene_number}
          placeholder="Scene #"
        />
      </div>
    </div>
    
    <div class="form-group">
      <label for="edit-character-pov">Character POV (Optional):</label>
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
      <label for="edit-content">Content:</label>
      <TextArea
        id="edit-content"
        bind:value={editForm.content}
        placeholder="Write your outline content..."
        rows={6}
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
      on:click={handleUpdateOutline}
      disabled={!editForm.title || !editForm.outline_type || !editForm.content}
    >
      Save Changes
    </Button>
  </div>
</Modal>

<!-- Detail View Modal -->
<Modal bind:show={showDetailModal} title={viewingOutline?.title || 'Outline Details'} on:close={closeModals}>
  {#if viewingOutline}
    <div class="detail-view">
      <div class="detail-header">
        <div class="detail-title">
          <span class="detail-icon">{getOutlineIcon(viewingOutline.outline_type)}</span>
          <div>
            <h3>{viewingOutline.title}</h3>
            <div class="detail-meta">
              <span class="detail-type">{getOutlineTypeLabel(viewingOutline.outline_type)}</span>
              {#if formatOutlineReference(viewingOutline)}
                <span class="detail-reference">{formatOutlineReference(viewingOutline)}</span>
              {/if}
            </div>
          </div>
        </div>
        
        <div class="detail-badges">
          <span class="visibility-badge" class:always={viewingOutline.visibility === 'always'} class:chapter={viewingOutline.visibility === 'chapter'} class:never={viewingOutline.visibility === 'never'}>
            {getVisibilityLabel(viewingOutline.visibility)}
          </span>
          {#if viewingOutline.series_shared}
            <span class="series-badge">Series Shared</span>
          {/if}
        </div>
      </div>
      
      <div class="detail-content">
        <div class="detail-section">
          <h4>Content</h4>
          <div class="content-text">{viewingOutline.content}</div>
        </div>
        
        <div class="detail-meta-info">
          <div class="meta-item">
            <strong>Created:</strong> {new Date(viewingOutline.created_at).toLocaleDateString()}
          </div>
          <div class="meta-item">
            <strong>Last Updated:</strong> {new Date(viewingOutline.updated_at).toLocaleDateString()}
          </div>
        </div>
      </div>
    </div>
  {/if}
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Close
    </Button>
    {#if viewingOutline}
      <Button variant="primary" on:click={() => { closeModals(); openEditModal(viewingOutline); }}>
        Edit Outline
      </Button>
    {/if}
  </div>
</Modal>

<style>
  .outline-manager {
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
  
  .outlines-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .outline-card {
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    overflow: hidden;
  }
  
  .outline-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1rem 1rem 0.5rem 1rem;
  }
  
  .outline-meta {
    flex: 1;
  }
  
  .outline-title {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }
  
  .outline-icon {
    font-size: 1.5rem;
    margin-top: 0.125rem;
  }
  
  .title-content {
    flex: 1;
  }
  
  .outline-name {
    margin: 0 0 0.25rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .outline-reference {
    font-size: 0.85rem;
    color: var(--text-secondary);
    font-style: italic;
  }
  
  .outline-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  
  .type-badge,
  .visibility-badge,
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
  
  .visibility-badge.always {
    background: #d4edda;
    color: #155724;
  }
  
  .visibility-badge.chapter {
    background: #fff3cd;
    color: #856404;
  }
  
  .visibility-badge.never {
    background: #f8d7da;
    color: #721c24;
  }
  
  .series-badge {
    background: #cce5ff;
    color: #004085;
  }
  
  .outline-actions {
    display: flex;
    gap: 0.25rem;
  }
  
  .outline-content {
    padding: 0 1rem 1rem 1rem;
  }
  
  .outline-preview {
    margin: 0;
    line-height: 1.6;
    color: var(--text-primary);
  }
  
  .outline-footer {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border-color);
    background: var(--bg-tertiary);
  }
  
  .outline-date {
    font-size: 0.8rem;
    color: var(--text-secondary);
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
  
  .content-text {
    line-height: 1.6;
    color: var(--text-primary);
    white-space: pre-wrap;
  }
  
  .detail-meta-info {
    display: flex;
    gap: 2rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }
  
  .meta-item {
    font-size: 0.9rem;
    color: var(--text-secondary);
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
    
    .detail-meta-info {
      flex-direction: column;
      gap: 0.5rem;
    }
    
    .outline-title {
      flex-direction: column;
      gap: 0.5rem;
    }
  }
</style>