<script lang="ts">
  import { onMount } from 'svelte';
  import { storyBibleStore, storyBibleActions } from '../../stores/storyBibleStore';
  import type { StoryBibleProps } from '../../types/storyBible';
  
  import BraindumpEditor from './components/BraindumpEditor.svelte';
  import CharactersManager from './components/CharactersManager.svelte';
  import WorldbuildingManager from './components/WorldbuildingManager.svelte';
  import OutlineManager from './components/OutlineManager.svelte';
  import ScenesManager from './components/ScenesManager.svelte';
  import LoadingSpinner from '../../components/ui/LoadingSpinner.svelte';
  import ErrorMessage from '../../components/ui/ErrorMessage.svelte';
  
  export let projectId: string;
  export let seriesId: string | undefined = undefined;
  
  $: state = $storyBibleStore;
  
  const tabs = [
    { id: 'braindump', label: 'Braindump', icon: '📝' },
    { id: 'characters', label: 'Characters', icon: '👥' },
    { id: 'worldbuilding', label: 'World Building', icon: '🌍' },
    { id: 'outline', label: 'Outline', icon: '📋' },
    { id: 'scenes', label: 'Scenes', icon: '🎬' }
  ] as const;
  
  onMount(async () => {
    // Load the story bible for this project
    await storyBibleActions.loadStoryBible(projectId);
    
    // Load world elements for the project
    await storyBibleActions.loadWorldElements(projectId);
    
    // Load outlines for the project
    await storyBibleActions.loadOutlines(projectId);
  });
  
  function handleTabClick(tabId: typeof state.activeTab) {
    storyBibleActions.setActiveTab(tabId);
  }
  
  function clearErrors() {
    storyBibleActions.clearAllErrors();
  }
</script>

<div class="story-bible-container">
  <!-- Header -->
  <div class="story-bible-header">
    <h1 class="title">
      <span class="icon">📚</span>
      Story Bible
    </h1>
    
    {#if state.error}
      <ErrorMessage 
        message={state.error} 
        onDismiss={clearErrors}
        type="error"
      />
    {/if}
  </div>
  
  <!-- Tab Navigation -->
  <div class="tab-navigation">
    {#each tabs as tab}
      <button 
        class="tab-button" 
        class:active={state.activeTab === tab.id}
        on:click={() => handleTabClick(tab.id)}
        disabled={state.isLoading}
      >
        <span class="tab-icon">{tab.icon}</span>
        <span class="tab-label">{tab.label}</span>
      </button>
    {/each}
  </div>
  
  <!-- Content Area -->
  <div class="content-area">
    {#if state.isLoading}
      <div class="loading-container">
        <LoadingSpinner size="large" />
        <p>Loading Story Bible...</p>
      </div>
    {:else}
      <!-- Braindump Tab -->
      {#if state.activeTab === 'braindump'}
        <BraindumpEditor 
          {projectId}
          content={state.storyBible?.braindump || ''}
          synopsis={state.storyBible?.synopsis || ''}
          genre={state.storyBible?.genre || ''}
          style={state.storyBible?.style || ''}
          styleExamples={state.storyBible?.style_examples || ''}
          povMode={state.storyBible?.pov_mode || ''}
          globalPov={state.storyBible?.global_pov || ''}
          globalTense={state.storyBible?.global_tense || ''}
          globalCharacterPovIds={state.storyBible?.global_character_pov_ids || ''}
        />
      {/if}
      
      <!-- Characters Tab -->
      {#if state.activeTab === 'characters'}
        <CharactersManager 
          {projectId}
          {seriesId}
          characterId={state.selectedCharacterId}
        />
      {/if}
      
      <!-- World Building Tab -->
      {#if state.activeTab === 'worldbuilding'}
        <WorldbuildingManager 
          {projectId}
          {seriesId}
        />
      {/if}
      
      <!-- Outline Tab -->
      {#if state.activeTab === 'outline'}
        <OutlineManager 
          {projectId}
        />
      {/if}
      
      <!-- Scenes Tab -->
      {#if state.activeTab === 'scenes'}
        <ScenesManager 
          outlineId={state.selectedOutlineId || ''}
        />
      {/if}
    {/if}
  </div>
</div>

<style>
  .story-bible-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: var(--bg-primary);
    color: var(--text-primary);
  }
  
  .story-bible-header {
    padding: 1.5rem 2rem;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-secondary);
  }
  
  .title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin: 0;
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .icon {
    font-size: 2rem;
  }
  
  .tab-navigation {
    display: flex;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
  }
  
  .tab-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 1rem 1.5rem;
    background: none;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
    border-bottom: 3px solid transparent;
  }
  
  .tab-button:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }
  
  .tab-button.active {
    color: var(--accent-primary);
    border-bottom-color: var(--accent-primary);
    background: var(--bg-primary);
  }
  
  .tab-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .tab-icon {
    font-size: 1.2rem;
  }
  
  .tab-label {
    font-weight: 500;
  }
  
  .content-area {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  
  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    gap: 1rem;
    color: var(--text-secondary);
  }
  
  /* Responsive Design */
  @media (max-width: 768px) {
    .story-bible-header {
      padding: 1rem;
    }
    
    .title {
      font-size: 1.5rem;
    }
    
    .tab-button {
      padding: 0.75rem 1rem;
    }
    
    .tab-label {
      display: none;
    }
    
    .tab-icon {
      font-size: 1.5rem;
    }
  }
  
  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .story-bible-container {
      --bg-primary: #1a1a1a;
      --bg-secondary: #2d2d2d;
      --bg-hover: #3d3d3d;
      --text-primary: #ffffff;
      --text-secondary: #b3b3b3;
      --border-color: #404040;
      --accent-primary: #4f9eff;
    }
  }
  
  /* Light mode support */
  @media (prefers-color-scheme: light) {
    .story-bible-container {
      --bg-primary: #ffffff;
      --bg-secondary: #f8f9fa;
      --bg-hover: #e9ecef;
      --text-primary: #212529;
      --text-secondary: #6c757d;
      --border-color: #dee2e6;
      --accent-primary: #0066cc;
    }
  }
</style>