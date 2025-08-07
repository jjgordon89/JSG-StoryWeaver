<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { storyBibleActions } from '../../../stores/storyBibleStore';
  import type { CreateStoryBibleRequest, UpdateStoryBibleRequest } from '../../../types/storyBible';
  
  import Button from '../../../components/ui/Button.svelte';
  import TextArea from '../../../components/ui/TextArea.svelte';
  import Input from '../../../components/ui/Input.svelte';
  import Select from '../../../components/ui/Select.svelte';
  import Card from '../../../components/ui/Card.svelte';
  
  export let projectId: string;
  export let content: string = '';
  export let synopsis: string = '';
  export let genre: string = '';
  export let style: string = '';
  export let styleExamples: string = '';
  export let povMode: string = '';
  export let globalPov: string = '';
  export let globalTense: string = '';
  export let globalCharacterPovIds: string = '';
  
  const dispatch = createEventDispatcher();
  
  let isEditing = false;
  let isSaving = false;
  let hasChanges = false;
  
  // Form data
  let formData = {
    braindump: content,
    synopsis: synopsis,
    genre: genre,
    style: style,
    style_examples: styleExamples,
    pov_mode: povMode,
    global_pov: globalPov,
    global_tense: globalTense,
    global_character_pov_ids: globalCharacterPovIds
  };
  
  // POV Mode options
  const povModeOptions = [
    { value: '', label: 'Select POV Mode' },
    { value: 'first_person', label: 'First Person' },
    { value: 'second_person', label: 'Second Person' },
    { value: 'third_person_limited', label: 'Third Person Limited' },
    { value: 'third_person_omniscient', label: 'Third Person Omniscient' },
    { value: 'multiple_pov', label: 'Multiple POV' }
  ];
  
  // Tense options
  const tenseOptions = [
    { value: '', label: 'Select Tense' },
    { value: 'past', label: 'Past Tense' },
    { value: 'present', label: 'Present Tense' },
    { value: 'future', label: 'Future Tense' },
    { value: 'mixed', label: 'Mixed Tenses' }
  ];
  
  // Genre suggestions
  const genreSuggestions = [
    'Fantasy', 'Science Fiction', 'Mystery', 'Romance', 'Thriller',
    'Horror', 'Historical Fiction', 'Contemporary Fiction', 'Young Adult',
    'Literary Fiction', 'Adventure', 'Crime', 'Dystopian', 'Urban Fantasy'
  ];
  
  // Watch for changes
  $: {
    hasChanges = 
      formData.braindump !== content ||
      formData.synopsis !== synopsis ||
      formData.genre !== genre ||
      formData.style !== style ||
      formData.style_examples !== styleExamples ||
      formData.pov_mode !== povMode ||
      formData.global_pov !== globalPov ||
      formData.global_tense !== globalTense ||
      formData.global_character_pov_ids !== globalCharacterPovIds;
  }
  
  function startEditing() {
    isEditing = true;
  }
  
  function cancelEditing() {
    // Reset form data
    formData = {
      braindump: content,
      synopsis: synopsis,
      genre: genre,
      style: style,
      style_examples: styleExamples,
      pov_mode: povMode,
      global_pov: globalPov,
      global_tense: globalTense,
      global_character_pov_ids: globalCharacterPovIds
    };
    isEditing = false;
    hasChanges = false;
  }
  
  async function saveChanges() {
    if (!hasChanges) return;
    
    isSaving = true;
    
    try {
      const request: CreateStoryBibleRequest = {
        project_id: projectId,
        ...formData
      };
      
      await storyBibleActions.createOrUpdateStoryBible(request);
      
      // Update props
      content = formData.braindump;
      synopsis = formData.synopsis;
      genre = formData.genre;
      style = formData.style;
      styleExamples = formData.style_examples;
      povMode = formData.pov_mode;
      globalPov = formData.global_pov;
      globalTense = formData.global_tense;
      globalCharacterPovIds = formData.global_character_pov_ids;
      
      isEditing = false;
      hasChanges = false;
      
      dispatch('saved');
    } catch (error) {
      console.error('Failed to save story bible:', error);
    } finally {
      isSaving = false;
    }
  }
  
  function handleGenreSelect(selectedGenre: string) {
    formData.genre = selectedGenre;
  }
</script>

<div class="braindump-editor">
  <!-- Header -->
  <div class="editor-header">
    <div class="header-content">
      <h2>Story Bible & Braindump</h2>
      <p class="subtitle">
        Capture your story's core elements, world-building notes, and creative brainstorming.
      </p>
    </div>
    
    <div class="header-actions">
      {#if !isEditing}
        <Button variant="primary" on:click={startEditing}>
          <span class="icon">✏️</span>
          Edit
        </Button>
      {:else}
        <div class="edit-actions">
          <Button 
            variant="secondary" 
            on:click={cancelEditing}
            disabled={isSaving}
          >
            Cancel
          </Button>
          <Button 
            variant="primary" 
            on:click={saveChanges}
            disabled={!hasChanges || isSaving}
            loading={isSaving}
          >
            {isSaving ? 'Saving...' : 'Save Changes'}
          </Button>
        </div>
      {/if}
    </div>
  </div>
  
  <!-- Content -->
  <div class="editor-content">
    <!-- Story Metadata -->
    <Card title="Story Metadata" class="metadata-card">
      <div class="metadata-grid">
        <!-- Genre -->
        <div class="field-group">
          <label for="genre">Genre</label>
          {#if isEditing}
            <div class="genre-input-container">
              <Input
                id="genre"
                bind:value={formData.genre}
                placeholder="Enter genre..."
                list="genre-suggestions"
              />
              <datalist id="genre-suggestions">
                {#each genreSuggestions as suggestion}
                  <option value={suggestion}></option>
                {/each}
              </datalist>
            </div>
          {:else}
            <p class="field-value">{genre || 'Not specified'}</p>
          {/if}
        </div>
        
        <!-- POV Mode -->
        <div class="field-group">
          <label for="pov-mode">Point of View</label>
          {#if isEditing}
            <Select
              id="pov-mode"
              bind:value={formData.pov_mode}
              options={povModeOptions}
            />
          {:else}
            <p class="field-value">
              {povModeOptions.find(opt => opt.value === povMode)?.label || 'Not specified'}
            </p>
          {/if}
        </div>
        
        <!-- Global Tense -->
        <div class="field-group">
          <label for="global-tense">Narrative Tense</label>
          {#if isEditing}
            <Select
              id="global-tense"
              bind:value={formData.global_tense}
              options={tenseOptions}
            />
          {:else}
            <p class="field-value">
              {tenseOptions.find(opt => opt.value === globalTense)?.label || 'Not specified'}
            </p>
          {/if}
        </div>
        
        <!-- Global POV Character -->
        <div class="field-group">
          <label for="global-pov">Primary POV Character</label>
          {#if isEditing}
            <Input
              id="global-pov"
              bind:value={formData.global_pov}
              placeholder="Main character name..."
            />
          {:else}
            <p class="field-value">{globalPov || 'Not specified'}</p>
          {/if}
        </div>
      </div>
    </Card>
    
    <!-- Synopsis -->
    <Card title="Synopsis" class="synopsis-card">
      {#if isEditing}
        <TextArea
          bind:value={formData.synopsis}
          placeholder="Write a brief synopsis of your story..."
          rows={4}
          class="synopsis-textarea"
        />
      {:else}
        <div class="synopsis-content">
          {#if synopsis}
            <p>{synopsis}</p>
          {:else}
            <p class="empty-state">No synopsis written yet. Click Edit to add one.</p>
          {/if}
        </div>
      {/if}
    </Card>
    
    <!-- Writing Style -->
    <Card title="Writing Style & Voice" class="style-card">
      <div class="style-content">
        <!-- Style Description -->
        <div class="field-group">
          <label for="style">Style Description</label>
          {#if isEditing}
            <TextArea
              id="style"
              bind:value={formData.style}
              placeholder="Describe your writing style, tone, and voice..."
              rows={3}
            />
          {:else}
            <div class="field-content">
              {#if style}
                <p>{style}</p>
              {:else}
                <p class="empty-state">No style description yet.</p>
              {/if}
            </div>
          {/if}
        </div>
        
        <!-- Style Examples -->
        <div class="field-group">
          <label for="style-examples">Style Examples</label>
          {#if isEditing}
            <TextArea
              id="style-examples"
              bind:value={formData.style_examples}
              placeholder="Paste example sentences or paragraphs that capture your desired style..."
              rows={4}
            />
          {:else}
            <div class="field-content">
              {#if styleExamples}
                <pre class="style-examples">{styleExamples}</pre>
              {:else}
                <p class="empty-state">No style examples yet.</p>
              {/if}
            </div>
          {/if}
        </div>
      </div>
    </Card>
    
    <!-- Braindump -->
    <Card title="Creative Braindump" class="braindump-card">
      {#if isEditing}
        <TextArea
          bind:value={formData.braindump}
          placeholder="Let your creativity flow! Jot down ideas, plot points, character thoughts, world-building details, or anything else related to your story..."
          rows={12}
          class="braindump-textarea"
        />
      {:else}
        <div class="braindump-content">
          {#if content}
            <pre class="braindump-text">{content}</pre>
          {:else}
            <div class="empty-state">
              <span class="empty-icon">💭</span>
              <h3>Start Your Creative Journey</h3>
              <p>This is your creative space. Use it to brainstorm ideas, capture inspiration, and develop your story's foundation.</p>
              <Button variant="primary" on:click={startEditing}>
                Start Writing
              </Button>
            </div>
          {/if}
        </div>
      {/if}
    </Card>
  </div>
</div>

<style>
  .braindump-editor {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
  }
  
  .editor-header {
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
  
  .header-actions {
    display: flex;
    gap: 0.75rem;
  }
  
  .edit-actions {
    display: flex;
    gap: 0.75rem;
  }
  
  .editor-content {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .metadata-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }
  
  .field-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .field-group label {
    font-weight: 500;
    color: var(--text-primary);
    font-size: 0.9rem;
  }
  
  .field-value {
    margin: 0;
    color: var(--text-secondary);
    font-style: italic;
  }
  
  .field-content {
    color: var(--text-primary);
  }
  
  .genre-input-container {
    position: relative;
  }
  
  .style-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .synopsis-content,
  .braindump-content {
    min-height: 100px;
  }
  
  .synopsis-textarea,
  .braindump-textarea {
    resize: vertical;
    font-family: var(--font-mono, 'Courier New', monospace);
  }
  
  .style-examples,
  .braindump-text {
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family: var(--font-mono, 'Courier New', monospace);
    font-size: 0.9rem;
    line-height: 1.6;
    color: var(--text-primary);
    margin: 0;
    padding: 1rem;
    background: var(--bg-tertiary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }
  
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem 2rem;
    text-align: center;
    color: var(--text-secondary);
    background: var(--bg-tertiary);
    border-radius: 0.75rem;
    border: 2px dashed var(--border-color);
  }
  
  .empty-state.empty-state {
    font-style: italic;
    padding: 1rem;
    background: transparent;
    border: none;
  }
  
  .empty-icon {
    font-size: 3rem;
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
  
  .icon {
    margin-right: 0.5rem;
  }
  
  /* Responsive Design */
  @media (max-width: 768px) {
    .editor-header {
      flex-direction: column;
      gap: 1rem;
      padding: 1.5rem;
    }
    
    .header-actions {
      width: 100%;
    }
    
    .edit-actions {
      width: 100%;
    }
    
    .editor-content {
      padding: 1.5rem;
    }
    
    .metadata-grid {
      grid-template-columns: 1fr;
    }
  }
  
  /* CSS Variables for theming */
  :global(.braindump-editor) {
    --bg-primary: #ffffff;
    --bg-secondary: #f8f9fa;
    --bg-tertiary: #f1f3f4;
    --text-primary: #212529;
    --text-secondary: #6c757d;
    --border-color: #dee2e6;
    --font-mono: 'SF Mono', 'Monaco', 'Inconsolata', 'Roboto Mono', monospace;
  }
  
  @media (prefers-color-scheme: dark) {
    :global(.braindump-editor) {
      --bg-primary: #1a1a1a;
      --bg-secondary: #2d2d2d;
      --bg-tertiary: #3d3d3d;
      --text-primary: #ffffff;
      --text-secondary: #b3b3b3;
      --border-color: #404040;
    }
  }
</style>