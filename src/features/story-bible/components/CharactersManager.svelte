<script lang="ts">
  import { onMount } from 'svelte';
  import { storyBibleStore, storyBibleActions, filteredCharacterTraits } from '../../../stores/storyBibleStore';
  import type { CharacterTrait, CreateCharacterTraitRequest, UpdateCharacterTraitRequest } from '../../../types/storyBible';
  
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
  export let characterId: string | undefined = undefined;
  
  $: state = $storyBibleStore;
  $: traits = $filteredCharacterTraits;
  
  // Modal state
  let showCreateModal = false;
  let showEditModal = false;
  let editingTrait: CharacterTrait | null = null;
  
  // Form state
  let createForm = {
    character_id: '',
    trait_type: '',
    content: '',
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  let editForm = {
    id: '',
    trait_type: '',
    content: '',
    visibility: 'always' as 'always' | 'chapter' | 'never',
    series_shared: false
  };
  
  // Available characters (mock data - in real app, this would come from characters store)
  let availableCharacters = [
    { id: '1', name: 'Main Character' },
    { id: '2', name: 'Antagonist' },
    { id: '3', name: 'Supporting Character' }
  ];
  
  // Trait type options
  const traitTypeOptions = [
    { value: '', label: 'Select trait type' },
    { value: 'physical', label: 'Physical Description' },
    { value: 'personality', label: 'Personality' },
    { value: 'background', label: 'Background' },
    { value: 'motivation', label: 'Motivation' },
    { value: 'goal', label: 'Goals' },
    { value: 'fear', label: 'Fears' },
    { value: 'strength', label: 'Strengths' },
    { value: 'weakness', label: 'Weaknesses' },
    { value: 'relationship', label: 'Relationships' },
    { value: 'quirk', label: 'Quirks' },
    { value: 'secret', label: 'Secrets' },
    { value: 'arc', label: 'Character Arc' },
    { value: 'dialogue', label: 'Dialogue Style' },
    { value: 'other', label: 'Other' }
  ];
  
  // Visibility options
  const visibilityOptions = [
    { value: 'always', label: 'Always Visible' },
    { value: 'chapter', label: 'Chapter Context' },
    { value: 'never', label: 'Hidden' }
  ];
  
  onMount(() => {
    if (characterId) {
      loadCharacterTraits(characterId);
    }
  });
  
  async function loadCharacterTraits(charId: string) {
    await storyBibleActions.loadCharacterTraits(charId);
  }
  
  function openCreateModal() {
    createForm = {
      character_id: characterId || '',
      trait_type: '',
      content: '',
      visibility: 'always',
      series_shared: false
    };
    showCreateModal = true;
  }
  
  function openEditModal(trait: CharacterTrait) {
    editingTrait = trait;
    editForm = {
      id: trait.id,
      trait_type: trait.trait_type,
      content: trait.content,
      visibility: trait.visibility,
      series_shared: trait.series_shared
    };
    showEditModal = true;
  }
  
  function closeModals() {
    showCreateModal = false;
    showEditModal = false;
    editingTrait = null;
  }
  
  async function handleCreateTrait() {
    if (!createForm.character_id || !createForm.trait_type || !createForm.content) {
      return;
    }
    
    const request: CreateCharacterTraitRequest = {
      character_id: createForm.character_id,
      trait_type: createForm.trait_type,
      content: createForm.content,
      visibility: createForm.visibility,
      series_shared: createForm.series_shared
    };
    
    await storyBibleActions.createCharacterTrait(request);
    closeModals();
  }
  
  async function handleUpdateTrait() {
    if (!editForm.id || !editForm.trait_type || !editForm.content) {
      return;
    }
    
    const request: UpdateCharacterTraitRequest = {
      id: editForm.id,
      trait_type: editForm.trait_type,
      content: editForm.content,
      visibility: editForm.visibility,
      series_shared: editForm.series_shared
    };
    
    await storyBibleActions.updateCharacterTrait(request);
    closeModals();
  }
  
  async function handleDeleteTrait(traitId: string) {
    if (confirm('Are you sure you want to delete this character trait?')) {
      await storyBibleActions.deleteCharacterTrait(traitId);
    }
  }
  
  function handleCharacterSelect(charId: string) {
    storyBibleActions.setSelectedCharacterId(charId);
    if (charId) {
      loadCharacterTraits(charId);
    }
  }
  
  function handleFilterChange(filterType: string, value: any) {
    const currentFilter = state.characterTraitFilter;
    storyBibleActions.setCharacterTraitFilter({
      ...currentFilter,
      [filterType]: value || undefined
    });
  }
  
  function getTraitTypeLabel(traitType: string): string {
    return traitTypeOptions.find(opt => opt.value === traitType)?.label || traitType;
  }
  
  function getVisibilityLabel(visibility: string): string {
    return visibilityOptions.find(opt => opt.value === visibility)?.label || visibility;
  }
</script>

<div class="characters-manager">
  <!-- Header -->
  <div class="manager-header">
    <div class="header-content">
      <h2>Character Traits</h2>
      <p class="subtitle">
        Manage detailed character information, personality traits, and development notes.
      </p>
    </div>
    
    <div class="header-actions">
      <Button 
        variant="primary" 
        on:click={openCreateModal}
        disabled={!characterId}
      >
        <span class="icon">➕</span>
        Add Trait
      </Button>
    </div>
  </div>
  
  <!-- Character Selection -->
  <Card title="Character Selection" class="selection-card">
    <div class="selection-content">
      <div class="character-selector">
        <label for="character-select">Select Character:</label>
        <Select
          id="character-select"
          value={characterId || ''}
          on:change={(e) => handleCharacterSelect(e.detail)}
          options={[
            { value: '', label: 'Choose a character...' },
            ...availableCharacters.map(char => ({ value: char.id, label: char.name }))
          ]}
        />
      </div>
      
      {#if characterId}
        <!-- Filters -->
        <div class="filters">
          <div class="filter-group">
            <label for="trait-type-filter">Filter by Type:</label>
            <Select
              id="trait-type-filter"
              value={state.characterTraitFilter.traitType || ''}
              on:change={(e) => handleFilterChange('traitType', e.detail)}
              options={[
                { value: '', label: 'All types' },
                ...traitTypeOptions.slice(1)
              ]}
            />
          </div>
          
          <div class="filter-group">
            <label for="visibility-filter">Filter by Visibility:</label>
            <Select
              id="visibility-filter"
              value={state.characterTraitFilter.visibility || ''}
              on:change={(e) => handleFilterChange('visibility', e.detail)}
              options={[
                { value: '', label: 'All visibility' },
                ...visibilityOptions
              ]}
            />
          </div>
        </div>
      {/if}
    </div>
  </Card>
  
  <!-- Content Area -->
  <div class="content-area">
    {#if state.traitsError}
      <ErrorMessage 
        message={state.traitsError} 
        onDismiss={() => storyBibleActions.clearError()}
        type="error"
      />
    {/if}
    
    {#if !characterId}
      <div class="empty-state">
        <span class="empty-icon">👤</span>
        <h3>Select a Character</h3>
        <p>Choose a character from the dropdown above to view and manage their traits.</p>
      </div>
    {:else if state.isLoadingTraits}
      <div class="loading-container">
        <LoadingSpinner size="medium" />
        <p>Loading character traits...</p>
      </div>
    {:else if traits.length === 0}
      <div class="empty-state">
        <span class="empty-icon">📝</span>
        <h3>No Character Traits</h3>
        <p>This character doesn't have any traits yet. Start building their profile!</p>
        <Button variant="primary" on:click={openCreateModal}>
          Add First Trait
        </Button>
      </div>
    {:else}
      <!-- Traits Grid -->
      <div class="traits-grid">
        {#each traits as trait (trait.id)}
          <Card class="trait-card">
            <div class="trait-header">
              <div class="trait-meta">
                <h4 class="trait-type">{getTraitTypeLabel(trait.trait_type)}</h4>
                <div class="trait-badges">
                  <span class="visibility-badge" class:always={trait.visibility === 'always'} class:chapter={trait.visibility === 'chapter'} class:never={trait.visibility === 'never'}>
                    {getVisibilityLabel(trait.visibility)}
                  </span>
                  {#if trait.series_shared}
                    <span class="series-badge">Series Shared</span>
                  {/if}
                </div>
              </div>
              
              <div class="trait-actions">
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => openEditModal(trait)}
                >
                  ✏️
                </Button>
                <Button 
                  variant="ghost" 
                  size="small"
                  on:click={() => handleDeleteTrait(trait.id)}
                >
                  🗑️
                </Button>
              </div>
            </div>
            
            <div class="trait-content">
              <p>{trait.content}</p>
            </div>
            
            <div class="trait-footer">
              <span class="trait-date">
                Updated {new Date(trait.updated_at).toLocaleDateString()}
              </span>
            </div>
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Trait Modal -->
<Modal bind:show={showCreateModal} title="Add Character Trait" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="create-character">Character:</label>
      <Select
        id="create-character"
        bind:value={createForm.character_id}
        options={[
          { value: '', label: 'Select character...' },
          ...availableCharacters.map(char => ({ value: char.id, label: char.name }))
        ]}
      />
    </div>
    
    <div class="form-group">
      <label for="create-trait-type">Trait Type:</label>
      <Select
        id="create-trait-type"
        bind:value={createForm.trait_type}
        options={traitTypeOptions}
      />
    </div>
    
    <div class="form-group">
      <label for="create-content">Content:</label>
      <TextArea
        id="create-content"
        bind:value={createForm.content}
        placeholder="Describe this character trait..."
        rows={4}
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
      on:click={handleCreateTrait}
      disabled={!createForm.character_id || !createForm.trait_type || !createForm.content}
    >
      Add Trait
    </Button>
  </div>
</Modal>

<!-- Edit Trait Modal -->
<Modal bind:show={showEditModal} title="Edit Character Trait" on:close={closeModals}>
  <div class="modal-form">
    <div class="form-group">
      <label for="edit-trait-type">Trait Type:</label>
      <Select
        id="edit-trait-type"
        bind:value={editForm.trait_type}
        options={traitTypeOptions}
      />
    </div>
    
    <div class="form-group">
      <label for="edit-content">Content:</label>
      <TextArea
        id="edit-content"
        bind:value={editForm.content}
        placeholder="Describe this character trait..."
        rows={4}
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
      on:click={handleUpdateTrait}
      disabled={!editForm.trait_type || !editForm.content}
    >
      Save Changes
    </Button>
  </div>
</Modal>

<style>
  .characters-manager {
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
  
  .selection-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .character-selector {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .character-selector label {
    font-weight: 500;
    color: var(--text-primary);
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
    min-width: 200px;
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
  
  .traits-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 1.5rem;
  }
  
  .trait-card {
    border: 1px solid var(--border-color);
    border-radius: 0.75rem;
    overflow: hidden;
  }
  
  .trait-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 1rem 1rem 0.5rem 1rem;
  }
  
  .trait-meta {
    flex: 1;
  }
  
  .trait-type {
    margin: 0 0 0.5rem 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .trait-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }
  
  .visibility-badge,
  .series-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 0.375rem;
    font-size: 0.75rem;
    font-weight: 500;
    text-transform: uppercase;
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
  
  .trait-actions {
    display: flex;
    gap: 0.25rem;
  }
  
  .trait-content {
    padding: 0 1rem 1rem 1rem;
  }
  
  .trait-content p {
    margin: 0;
    line-height: 1.6;
    color: var(--text-primary);
  }
  
  .trait-footer {
    padding: 0.75rem 1rem;
    border-top: 1px solid var(--border-color);
    background: var(--bg-tertiary);
  }
  
  .trait-date {
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
  
  .checkbox-label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }
  
  .modal-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
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
    
    .traits-grid {
      grid-template-columns: 1fr;
    }
    
    .filters {
      flex-direction: column;
    }
    
    .form-row {
      grid-template-columns: 1fr;
    }
  }
</style>