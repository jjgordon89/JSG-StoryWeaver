<script lang="ts">
  import { onMount } from 'svelte';
  import { storyBibleStore, storyBibleActions, filteredWorldElements } from '../../../stores/storyBibleStore';
  import type { WorldElement, CreateWorldElementRequest, UpdateWorldElementRequest } from '../../../types/storyBible';
  
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
  $: worldElements = $filteredWorldElements;
  
  // Modal state
  let showCreateModal = false;
  let showEditModal = false;
  let showDetailModal = false;
  let editingElement: WorldElement | null = null;
  let viewingElement: WorldElement | null = null;
  
  // Form state
  let createForm = {
    name: '',
    element_type: '',
    description: '',
    significance: '',
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  let editForm = {
    id: '',
    name: '',
    element_type: '',
    description: '',
    significance: '',
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  // Search state
  let searchQuery = '';
  
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
  
  onMount(() => {
    loadWorldElements();
  });
  
  async function loadWorldElements() {
    await storyBibleActions.loadWorldElements(projectId, seriesId);
  }
  
  function openCreateModal() {
    createForm = {
      name: '',
      element_type: '',
      description: '',
      significance: '',
      visibility: 'always',
      series_shared: false
    };
    showCreateModal = true;
  }
  
  function openEditModal(element: WorldElement) {
    editingElement = element;
    editForm = {
      id: element.id,
      name: element.name,
      element_type: element.element_type,
      description: element.description,
      significance: element.significance || '',
      visibility: element.visibility,
      series_shared: element.series_shared
    };
    showEditModal = true;
  }
  
  function openDetailModal(element: WorldElement) {
    viewingElement = element;
    showDetailModal = true;
  }
  
  function closeModals() {
    showCreateModal = false;
    showEditModal = false;
    showDetailModal = false;
    editingElement = null;
    viewingElement = null;
  }
  
  async function handleCreateElement() {
    if (!createForm.name || !createForm.element_type || !createForm.description) {
      return;
    }
    
    const request: CreateWorldElementRequest = {
      project_id: projectId,
      series_id: seriesId,
      name: createForm.name,
      element_type: createForm.element_type,
      description: createForm.description,
      significance: createForm.significance || undefined,
      visibility: createForm.visibility,
      series_shared: createForm.series_shared
    };
    
    await storyBibleActions.createWorldElement(request);
    closeModals();
  }
  
  async function handleUpdateElement() {
    if (!editForm.id || !editForm.name || !editForm.element_type || !editForm.description) {
      return;
    }
    
    const request: UpdateWorldElementRequest = {
      id: editForm.id,
      name: editForm.name,
      element_type: editForm.element_type,
      description: editForm.description,
      significance: editForm.significance || undefined,
      visibility: editForm.visibility,
      series_shared: editForm.series_shared
    };
    
    await storyBibleActions.updateWorldElement(request);
    closeModals();
  }
  
  async function handleDeleteElement(elementId: string) {
    if (confirm('Are you sure you want to delete this world element?')) {
      await storyBibleActions.deleteWorldElement(elementId);
    }
  }
  
  async function handleSearch() {
    if (searchQuery.trim()) {
      await storyBibleActions.searchWorldElements(projectId, searchQuery, seriesId);
    } else {
      await loadWorldElements();
    }
  }
  
  function handleFilterChange(filterType: string, value: any) {
    const currentFilter = state.worldElementFilter;
    storyBibleActions.setWorldElementFilter({
      ...currentFilter,
      [filterType]: value || undefined
    });
  }
  
  function getElementTypeLabel(elementType: string): string {
    return elementTypeOptions.find(opt => opt.value === elementType)?.label || elementType;
  }
  
  function getVisibilityLabel(visibility: string): string {
    return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
  }
  
  function getElementIcon(elementType: string): string {
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
  }
</script>

<div class="world-building-manager">
  <!-- Header -->
  <div class="manager-header">
    <div class="header-content">
      <h2>World Building</h2>
      <p class="subtitle">
        Create and manage the world elements that shape your story's universe.
      </p>
    </div>
    
    <div class="header-actions">
      <Button variant="primary" on:click={openCreateModal}>
        <span class="icon">➕</span>
        Add Element
      </Button>
    </div>
  </div>
  
  <!-- Search and Filters -->
  <Card title="Search & Filter" class="search-card">
    <div class="search-content">
      <div class="search-bar">
        <Input
          type="text"
          placeholder="Search world elements..."
          bind:value={searchQuery}
          on:input={handleSearch}
        />
        <Button variant="secondary" on:click={handleSearch}>
          🔍 Search
        </Button>
      </div>
      
      <div class="filters">
        <div class="filter-group">
          <label for="element-type-filter">Filter by Type:</label>
          <Select
            id="element-type-filter"
            value={state.worldElementFilter.elementType || ''}
            on:change={(e) => handleFilterChange('elementType', e.detail)}
            options={[
              { value: '', label: 'All types' },
              ...elementTypeOptions.slice(1)
            ]}
          />
        </div>
        
        <div class="filter-group">
          <label for="visibility-filter">Filter by Visibility:</label>
          <Select
            id="visibility-filter"
            value={state.worldElementFilter.visibility || ''}
            on:change={(e) => handleFilterChange('visibility', e.detail)}
            options={[
              { value: '', label: 'All visibility' },
              ...visibilityOptions
            ]}
          />
        </div>
        
        <div class="filter-group">
          <label class="checkbox-label">
            <input 
              type="checkbox" 
              checked={state.worldElementFilter.seriesShared || false}
              on:change={(e) => handleFilterChange('seriesShared', e.target.checked)}
            />
            Series Shared Only
          </label>
        </div>
      </div>
    </div>
  </Card>
  
  <!-- Content Area -->
  <div class="content-area">
    {#if state.worldElementsError}
      <ErrorMessage 
        message={state.worldElementsError} 
        onDismiss={() => storyBibleActions.clearError()}
        type="error"
      />
    {/if}
    
    {#if state.isLoadingWorldElements}
      <div class="loading-container">
        <LoadingSpinner size="medium" />
        <p>Loading world elements...</p>
      </div>
    {:else if worldElements.length === 0}
      <div class="empty-state">
        <span class="empty-icon">🌍</span>
        <h3>No World Elements</h3>
        <p>Start building your story's world by adding locations, cultures, organizations, and more.</p>
        <Button variant="primary" on:click={openCreateModal}>
          Create First Element
        </Button>
      </div>
    {:else}
      <!-- Elements Grid -->
      <div class="elements-grid">
        {#each worldElements as element (element.id)}
          <Card class="element-card">
            <div class="element-header">
              <div class="element-meta">
                <div class="element-title">
                  <span class="element-icon">{getElementIcon(element.element_type)}</span>
                  <h4 class="element-name">{element.name}</h4>
                </div>
                <div class="element-badges">
                  <span class="type-badge">{getElementTypeLabel(element.element_type)}</span>
                  <span class="visibility-badge" class:always={element.visibility === 'always'} class:chapter={element.visibility === 'chapter'} class:never={element.visibility === 'never'}>
                    {getVisibilityLabel(element.visibility)}
                  </span>
                  {#if element.series_shared}
                    <span class="series-badge">Series Shared</span>
                  {/if}
                </div>
              </div>
              
              <div class="element-actions">
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openDetailModal(element)}
                  title="View Details"
                >
                  👁️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openEditModal(element)}
                  title="Edit"
                >
                  ✏️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => handleDeleteElement(element.id)}
                  title="Delete"
                >
                  🗑️
                </Button>
              </div>
            </div>
            
            <div class="element-content">
              <p class="element-description">{element.description}</p>
              {#if element.significance}
                <div class="element-significance">
                  <strong>Significance:</strong> {element.significance}
                </div>
              {/if}
            </div>
            
            <div class="element-footer">
              <span class="element-date">
                Updated {new Date(element.updated_at).toLocaleDateString()}
              </span>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Element Modal -->
<Modal bind:show={showCreateModal} title="Add World Element" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="create-name">Name:</label>
      <Input
        id="create-name"
        bind:value={createForm.name}
        placeholder="Enter element name..."
      />
    </div>
    
    <div class="form-group">
      <label for="create-element-type">Element Type:</label>
      <Select
        id="create-element-type"
        bind:value={createForm.element_type}
        options={elementTypeOptions}
      />
    </div>
    
    <div class="form-group">
      <label for="create-description">Description:</label>
      <TextArea
        id="create-description"
        bind:value={createForm.description}
        placeholder="Describe this world element..."
        rows={4}
      />
    </div>
    
    <div class="form-group">
      <label for="create-significance">Significance (Optional):</label>
      <TextArea
        id="create-significance"
        bind:value={createForm.significance}
        placeholder="Why is this element important to your story?"
        rows={2}
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
      on:click={handleCreateElement}
      disabled={!createForm.name || !createForm.element_type || !createForm.description}
    >
      Add Element
    </Button>
  </div>
</Modal>

<!-- Edit Element Modal -->
<Modal bind:show={showEditModal} title="Edit World Element" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="edit-name">Name:</label>
      <Input
        id="edit-name"
        bind:value={editForm.name}
        placeholder="Enter element name..."
      />
    </div>
    
    <div class="form-group">
      <label for="edit-element-type">Element Type:</label>
      <Select
        id="edit-element-type"
        bind:value={editForm.element_type}
        options={elementTypeOptions}
      />
    </div>
    
    <div class="form-group">
      <label for="edit-description">Description:</label>
      <TextArea
        id="edit-description"
        bind:value={editForm.description}
        placeholder="Describe this world element..."
        rows={4}
      />
    </div>
    
    <div class="form-group">
      <label for="edit-significance">Significance (Optional):</label>
      <TextArea
        id="edit-significance"
        bind:value={editForm.significance}
        placeholder="Why is this element important to your story?"
        rows={2}
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
      on:click={handleUpdateElement}
      disabled={!editForm.name || !editForm.element_type || !editForm.description}
    >
      Save Changes
    </Button>
  </div>
</Modal>

<!-- Detail View Modal -->
<Modal bind:show={showDetailModal} title={viewingElement?.name || 'World Element Details'} on:close={closeModals}>
  {#if viewingElement}
    <div class="detail-view">
      <div class="detail-header">
        <div class="detail-title">
          <span class="detail-icon">{getElementIcon(viewingElement.element_type)}</span>
          <div>
            <h3>{viewingElement.name}</h3>
            <span class="detail-type">{getElementTypeLabel(viewingElement.element_type)}</span>
          </div>
        </div>
        
        <div class="detail-badges">
          <span class="visibility-badge" class:always={viewingElement.visibility === 'always'} class:chapter={viewingElement.visibility === 'chapter'} class:never={viewingElement.visibility === 'never'}>
            {getVisibilityLabel(viewingElement.visibility)}
          </span>
          {#if viewingElement.series_shared}
            <span class="series-badge">Series Shared</span>
          {/if}
        </div>
      </div>
      
      <div class="detail-content">
        <div class="detail-section">
          <h4>Description</h4>
          <p>{viewingElement.description}</p>
        </div>
        
        {#if viewingElement.significance}
          <div class="detail-section">
            <h4>Significance</h4>
            <p>{viewingElement.significance}</p>
          </div>
        {/if}
        
        <div class="detail-meta">
          <div class="meta-item">
            <strong>Created:</strong> {new Date(viewingElement.created_at).toLocaleDateString()}
          </div>
          <div class="meta-item">
            <strong>Last Updated:</strong> {new Date(viewingElement.updated_at).toLocaleDateString()}
          </div>
        </div>
      </div>
    </div>
  {/if}
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Close
    </Button>
    {#if viewingElement}
      <Button variant="primary" on:click={() => { closeModals(); openEditModal(viewingElement); }}>
        Edit Element
      </Button>
    {/if}
  </div>
</Modal>

<style>
  .world-building-manager {
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
    align-items: end;
  }
  
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 200px;
  }
  
  .filter-group label {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--text-primary);
  }
  
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
    margin-top: 1.5rem;
  }
  
  .content-area {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }
  
  .elements-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 1.5rem;
  }
  
  .element-card {
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    overflow: hidden;
  }
  
  .element-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1rem 1rem 0.5rem 1rem;
  }
  
  .element-meta {
    flex: 1;
  }
  
  .element-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }
  
  .element-icon {
    font-size: 1.5rem;
  }
  
  .element-name {
    margin: 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .element-badges {
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
  
  .element-actions {
    display: flex;
    gap: 0.25rem;
  }
  
  .element-content {
    padding: 0 1rem 1rem 1rem;
  }
  
  .element-description {
    margin: 0 0 0.75rem 0;
    line-height: 1.6;
    color: var(--text-primary);
  }
  
  .element-significance {
    padding: 0.75rem;
    background: var(--bg-tertiary);
    border-radius: 0.5rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }
  
  .element-footer {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border-color);
    background: var(--bg-tertiary);
  }
  
  .element-date {
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
    grid-template-columns: 1fr auto;
    gap: 1rem;
    align-items: end;
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
    align-items: center;
    gap: 1rem;
  }
  
  .detail-icon {
    font-size: 2rem;
  }
  
  .detail-title h3 {
    margin: 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }
  
  .detail-type {
    color: var(--text-secondary);
    font-size: 0.9rem;
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
    margin: 0 0 0.5rem 0;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .detail-section p {
    margin: 0;
    line-height: 1.6;
    color: var(--text-primary);
  }
  
  .detail-meta {
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
    
    .elements-grid {
      grid-template-columns: 1fr;
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
    
    .detail-meta {
      flex-direction: column;
      gap: 0.5rem;
    }
  }
</style>