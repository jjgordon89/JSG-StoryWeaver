<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import Button from '../../../components/ui/Button.svelte';
  import Input from '../../../components/ui/Input.svelte';
  import Card from '../../../components/ui/Card.svelte';
  import Modal from '../../../components/ui/Modal.svelte';
  import LoadingSpinner from '../../../components/ui/LoadingSpinner.svelte';
  import { showToast } from '../../../utils/toast';

  export let projectId: string;
  export let userId: string = 'default-user';

  interface StyleExample {
    id: string;
    project_id: string;
    user_id: string;
    example_text: string;
    analysis_result?: string;
    generated_style_prompt?: string;
    word_count: number;
    created_at: string;
    updated_at: string;
  }

  interface AnalysisResult {
    tone: string;
    style_elements: string[];
    sentence_structure: string;
    vocabulary_level: string;
    pacing: string;
    voice: string;
    key_phrases: string[];
  }

  // State
  let styleExamples: StyleExample[] = [];
  let isLoading = false;
  let isAnalyzing = false;
  let showCreateModal = false;
  let showViewModal = false;
  let selectedExample: StyleExample | null = null;
  let searchQuery = '';
  
  // Form state
  let newExampleText = '';
  let editingExample: StyleExample | null = null;
  let isEditing = false;

  // Computed
  $: filteredExamples = styleExamples.filter(example => 
    example.example_text.toLowerCase().includes(searchQuery.toLowerCase()) ||
    (example.analysis_result && example.analysis_result.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  $: analyzedExamples = styleExamples.filter(example => example.analysis_result);
  $: unanalyzedExamples = styleExamples.filter(example => !example.analysis_result);

  onMount(() => {
    loadStyleExamples();
  });

  async function loadStyleExamples() {
    isLoading = true;
    try {
      const examples = await invoke('get_style_examples_by_project', { projectId });
      styleExamples = examples as StyleExample[];
    } catch (error) {
      console.error('Failed to load style examples:', error);
      showToast('Failed to load style examples', 'error');
    } finally {
      isLoading = false;
    }
  }

  async function createStyleExample() {
    if (!newExampleText.trim()) {
      showToast('Please enter some example text', 'error');
      return;
    }

    if (newExampleText.split(' ').length > 1000) {
      showToast('Example text must be 1000 words or less', 'error');
      return;
    }

    try {
      const request = {
        project_id: projectId,
        user_id: userId,
        example_text: newExampleText.trim()
      };

      const newExample = await invoke('create_style_example', request);
      styleExamples = [newExample as StyleExample, ...styleExamples];
      newExampleText = '';
      showCreateModal = false;
      showToast('Style example created successfully', 'success');
    } catch (error) {
      console.error('Failed to create style example:', error);
      showToast('Failed to create style example', 'error');
    }
  }

  async function analyzeStyleExample(example: StyleExample) {
    isAnalyzing = true;
    try {
      const result = await invoke('analyze_style_example', {
        styleExampleId: example.id,
        exampleText: example.example_text
      });

      // Update the example in our local state
      styleExamples = styleExamples.map(ex => 
        ex.id === example.id 
          ? { ...ex, analysis_result: result.analysis_result, generated_style_prompt: result.generated_style_prompt }
          : ex
      );

      showToast('Style analysis completed', 'success');
    } catch (error) {
      console.error('Failed to analyze style example:', error);
      showToast('Failed to analyze style example', 'error');
    } finally {
      isAnalyzing = false;
    }
  }

  async function updateStyleExample() {
    if (!editingExample) return;

    try {
      const request = {
        id: editingExample.id,
        example_text: editingExample.example_text,
        analysis_result: editingExample.analysis_result,
        generated_style_prompt: editingExample.generated_style_prompt
      };

      const updated = await invoke('update_style_example', request);
      styleExamples = styleExamples.map(ex => 
        ex.id === editingExample.id ? updated as StyleExample : ex
      );
      
      editingExample = null;
      isEditing = false;
      showToast('Style example updated successfully', 'success');
    } catch (error) {
      console.error('Failed to update style example:', error);
      showToast('Failed to update style example', 'error');
    }
  }

  async function deleteStyleExample(example: StyleExample) {
    if (!confirm('Are you sure you want to delete this style example?')) return;

    try {
      await invoke('delete_style_example', { id: example.id });
      styleExamples = styleExamples.filter(ex => ex.id !== example.id);
      showToast('Style example deleted successfully', 'success');
    } catch (error) {
      console.error('Failed to delete style example:', error);
      showToast('Failed to delete style example', 'error');
    }
  }

  function openCreateModal() {
    newExampleText = '';
    showCreateModal = true;
  }

  function openViewModal(example: StyleExample) {
    selectedExample = example;
    showViewModal = true;
  }

  function openEditModal(example: StyleExample) {
    editingExample = { ...example };
    isEditing = true;
    showViewModal = false;
  }

  function closeModals() {
    showCreateModal = false;
    showViewModal = false;
    selectedExample = null;
    editingExample = null;
    isEditing = false;
  }

  function getWordCount(text: string): number {
    return text.trim().split(/\s+/).length;
  }

  function parseAnalysisResult(analysisJson: string): AnalysisResult | null {
    try {
      return JSON.parse(analysisJson);
    } catch {
      return null;
    }
  }

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString();
  }
</script>

<div class="style-examples-manager">
  <div class="manager-header">
    <div class="header-content">
      <h2>📝 Style Examples</h2>
      <p class="subtitle">Manage writing style examples and AI analysis (up to 1,000 words each)</p>
    </div>
    
    <div class="header-actions">
      <Button variant="primary" on:click={openCreateModal}>
        ➕ Add Example
      </Button>
    </div>
  </div>

  <div class="content-area">
    <!-- Search and Filters -->
    <div class="search-section">
      <div class="search-bar">
        <Input
          bind:value={searchQuery}
          placeholder="Search style examples..."
          class="search-input"
        />
      </div>
      
      <div class="stats">
        <span class="stat-item">Total: {styleExamples.length}</span>
        <span class="stat-item">Analyzed: {analyzedExamples.length}</span>
        <span class="stat-item">Pending: {unanalyzedExamples.length}</span>
      </div>
    </div>

    <!-- Examples List -->
    {#if isLoading}
      <div class="loading-container">
        <LoadingSpinner size="large" />
        <p>Loading style examples...</p>
      </div>
    {:else if filteredExamples.length === 0}
      <div class="empty-state">
        <div class="empty-icon">📝</div>
        <h3>No Style Examples</h3>
        <p>Create your first style example to help AI understand your writing style.</p>
        <Button variant="primary" on:click={openCreateModal}>
          Add Your First Example
        </Button>
      </div>
    {:else}
      <div class="examples-grid">
        {#each filteredExamples as example (example.id)}
          <Card class="example-card">
            <div class="example-header">
              <div class="example-meta">
                <span class="word-count">{example.word_count} words</span>
                <span class="date">{formatDate(example.created_at)}</span>
              </div>
              
              <div class="example-actions">
                {#if !example.analysis_result}
                  <Button 
                    variant="secondary" 
                    size="small" 
                    on:click={() => analyzeStyleExample(example)}
                    disabled={isAnalyzing}
                  >
                    {isAnalyzing ? '🔄' : '🔍'} Analyze
                  </Button>
                {/if}
                
                <Button 
                  variant="ghost" 
                  size="small" 
                  on:click={() => openViewModal(example)}
                >
                  👁️ View
                </Button>
                
                <Button 
                  variant="ghost" 
                  size="small" 
                  on:click={() => deleteStyleExample(example)}
                  class="delete-btn"
                >
                  🗑️
                </Button>
              </div>
            </div>
            
            <div class="example-preview">
              <p class="example-text">
                {example.example_text.length > 200 
                  ? example.example_text.substring(0, 200) + '...' 
                  : example.example_text}
              </p>
            </div>
            
            {#if example.analysis_result}
              <div class="analysis-preview">
                <div class="analysis-badge">✅ Analyzed</div>
                {#if example.generated_style_prompt}
                  <p class="style-prompt-preview">
                    <strong>Style Prompt:</strong> 
                    {example.generated_style_prompt.length > 100 
                      ? example.generated_style_prompt.substring(0, 100) + '...' 
                      : example.generated_style_prompt}
                  </p>
                {/if}
              </div>
            {:else}
              <div class="analysis-preview pending">
                <div class="analysis-badge pending">⏳ Pending Analysis</div>
              </div>
            {/if}
          </Card>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Example Modal -->
<Modal bind:show={showCreateModal} title="Add Style Example" size="large">
  <div class="create-form">
    <div class="form-field">
      <label for="example-text">Example Text (up to 1,000 words)</label>
      <textarea
        id="example-text"
        bind:value={newExampleText}
        placeholder="Paste a sample of your writing that represents your desired style..."
        rows={12}
        class="example-textarea"
      ></textarea>
      <div class="word-count-info">
        <span class="word-count">{getWordCount(newExampleText)} / 1,000 words</span>
        {#if getWordCount(newExampleText) > 1000}
          <span class="error">⚠️ Exceeds word limit</span>
        {/if}
      </div>
    </div>
  </div>
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Cancel
    </Button>
    <Button 
      variant="primary" 
      on:click={createStyleExample}
      disabled={!newExampleText.trim() || getWordCount(newExampleText) > 1000}
    >
      Create Example
    </Button>
  </div>
</Modal>

<!-- View/Edit Example Modal -->
<Modal bind:show={showViewModal} title="Style Example" size="large">
  {#if selectedExample}
    <div class="view-content">
      <div class="example-details">
        <div class="detail-header">
          <div class="detail-meta">
            <span class="word-count">{selectedExample.word_count} words</span>
            <span class="date">Created: {formatDate(selectedExample.created_at)}</span>
            {#if selectedExample.updated_at !== selectedExample.created_at}
              <span class="date">Updated: {formatDate(selectedExample.updated_at)}</span>
            {/if}
          </div>
          
          <div class="detail-actions">
            {#if !selectedExample.analysis_result}
              <Button 
                variant="secondary" 
                on:click={() => analyzeStyleExample(selectedExample)}
                disabled={isAnalyzing}
              >
                {isAnalyzing ? '🔄 Analyzing...' : '🔍 Analyze Style'}
              </Button>
            {/if}
            
            <Button variant="secondary" on:click={() => openEditModal(selectedExample)}>
              ✏️ Edit
            </Button>
          </div>
        </div>
        
        <div class="example-text-full">
          <h4>Example Text</h4>
          <div class="text-content">
            {selectedExample.example_text}
          </div>
        </div>
        
        {#if selectedExample.analysis_result}
          {@const analysis = parseAnalysisResult(selectedExample.analysis_result)}
          <div class="analysis-results">
            <h4>AI Analysis</h4>
            {#if analysis}
              <div class="analysis-grid">
                <div class="analysis-item">
                  <strong>Tone:</strong> {analysis.tone}
                </div>
                <div class="analysis-item">
                  <strong>Voice:</strong> {analysis.voice}
                </div>
                <div class="analysis-item">
                  <strong>Sentence Structure:</strong> {analysis.sentence_structure}
                </div>
                <div class="analysis-item">
                  <strong>Vocabulary Level:</strong> {analysis.vocabulary_level}
                </div>
                <div class="analysis-item">
                  <strong>Pacing:</strong> {analysis.pacing}
                </div>
                {#if analysis.style_elements && analysis.style_elements.length > 0}
                  <div class="analysis-item full-width">
                    <strong>Style Elements:</strong>
                    <div class="style-tags">
                      {#each analysis.style_elements as element}
                        <span class="style-tag">{element}</span>
                      {/each}
                    </div>
                  </div>
                {/if}
                {#if analysis.key_phrases && analysis.key_phrases.length > 0}
                  <div class="analysis-item full-width">
                    <strong>Key Phrases:</strong>
                    <div class="key-phrases">
                      {#each analysis.key_phrases as phrase}
                        <span class="key-phrase">"{phrase}"</span>
                      {/each}
                    </div>
                  </div>
                {/if}
              </div>
            {:else}
              <div class="raw-analysis">
                <pre>{selectedExample.analysis_result}</pre>
              </div>
            {/if}
          </div>
        {/if}
        
        {#if selectedExample.generated_style_prompt}
          <div class="style-prompt">
            <h4>Generated Style Prompt</h4>
            <div class="prompt-content">
              {selectedExample.generated_style_prompt}
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Close
    </Button>
    {#if selectedExample && !selectedExample.analysis_result}
      <Button 
        variant="primary" 
        on:click={() => analyzeStyleExample(selectedExample)}
        disabled={isAnalyzing}
      >
        {isAnalyzing ? '🔄 Analyzing...' : '🔍 Analyze Style'}
      </Button>
    {/if}
  </div>
</Modal>

<!-- Edit Example Modal -->
<Modal bind:show={isEditing} title="Edit Style Example" size="large">
  {#if editingExample}
    <div class="edit-form">
      <div class="form-field">
        <label for="edit-example-text">Example Text (up to 1,000 words)</label>
        <textarea
          id="edit-example-text"
          bind:value={editingExample.example_text}
          rows={12}
          class="example-textarea"
        ></textarea>
        <div class="word-count-info">
          <span class="word-count">{getWordCount(editingExample.example_text)} / 1,000 words</span>
          {#if getWordCount(editingExample.example_text) > 1000}
            <span class="error">⚠️ Exceeds word limit</span>
          {/if}
        </div>
      </div>
    </div>
  {/if}
  
  <div slot="footer" class="modal-actions">
    <Button variant="secondary" on:click={closeModals}>
      Cancel
    </Button>
    <Button 
      variant="primary" 
      on:click={updateStyleExample}
      disabled={!editingExample?.example_text.trim() || getWordCount(editingExample?.example_text || '') > 1000}
    >
      Update Example
    </Button>
  </div>
</Modal>

<style>
  .style-examples-manager {
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

  .content-area {
    flex: 1;
    padding: 2rem;
    overflow-y: auto;
  }

  .search-section {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    gap: 1rem;
  }

  .search-bar {
    flex: 1;
    max-width: 400px;
  }

  .stats {
    display: flex;
    gap: 1rem;
  }

  .stat-item {
    padding: 0.5rem 1rem;
    background: var(--bg-secondary);
    border-radius: 0.5rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    gap: 1rem;
    color: var(--text-secondary);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem;
    text-align: center;
    color: var(--text-secondary);
  }

  .empty-icon {
    font-size: 4rem;
    margin-bottom: 1rem;
  }

  .empty-state h3 {
    margin: 0 0 1rem 0;
    color: var(--text-primary);
  }

  .empty-state p {
    margin: 0 0 2rem 0;
    max-width: 400px;
  }

  .examples-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
    gap: 1.5rem;
  }

  .example-card {
    border: 1px solid var(--border-color);
    transition: all 0.2s ease;
  }

  .example-card:hover {
    border-color: var(--accent-primary);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .example-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .example-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .example-actions {
    display: flex;
    gap: 0.5rem;
  }

  .example-preview {
    margin-bottom: 1rem;
  }

  .example-text {
    margin: 0;
    line-height: 1.5;
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .analysis-preview {
    padding-top: 1rem;
    border-top: 1px solid var(--border-color);
  }

  .analysis-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 1rem;
    font-size: 0.8rem;
    font-weight: 500;
    background: var(--success-bg);
    color: var(--success-text);
  }

  .analysis-badge.pending {
    background: var(--warning-bg);
    color: var(--warning-text);
  }

  .style-prompt-preview {
    margin: 0.5rem 0 0 0;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .create-form,
  .edit-form {
    padding: 1rem 0;
  }

  .form-field {
    margin-bottom: 1.5rem;
  }

  .form-field label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: var(--text-primary);
  }

  .example-textarea {
    width: 100%;
    min-height: 300px;
    padding: 1rem;
    border: 1px solid var(--border-color);
    border-radius: 0.5rem;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-family: inherit;
    font-size: 0.9rem;
    line-height: 1.5;
    resize: vertical;
  }

  .example-textarea:focus {
    outline: none;
    border-color: var(--accent-primary);
  }

  .word-count-info {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 0.5rem;
    font-size: 0.85rem;
  }

  .word-count {
    color: var(--text-secondary);
  }

  .error {
    color: var(--error-text);
    font-weight: 500;
  }

  .view-content {
    max-height: 70vh;
    overflow-y: auto;
  }

  .detail-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .detail-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .detail-actions {
    display: flex;
    gap: 0.5rem;
  }

  .example-text-full {
    margin-bottom: 2rem;
  }

  .example-text-full h4 {
    margin: 0 0 1rem 0;
    color: var(--text-primary);
  }

  .text-content {
    padding: 1.5rem;
    background: var(--bg-secondary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
    line-height: 1.6;
    white-space: pre-wrap;
    color: var(--text-primary);
  }

  .analysis-results {
    margin-bottom: 2rem;
  }

  .analysis-results h4 {
    margin: 0 0 1rem 0;
    color: var(--text-primary);
  }

  .analysis-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
  }

  .analysis-item {
    padding: 1rem;
    background: var(--bg-secondary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .analysis-item.full-width {
    grid-column: 1 / -1;
  }

  .analysis-item strong {
    display: block;
    margin-bottom: 0.5rem;
    color: var(--text-primary);
  }

  .style-tags,
  .key-phrases {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .style-tag,
  .key-phrase {
    padding: 0.25rem 0.75rem;
    background: var(--accent-bg);
    color: var(--accent-text);
    border-radius: 1rem;
    font-size: 0.8rem;
  }

  .key-phrase {
    background: var(--info-bg);
    color: var(--info-text);
    font-style: italic;
  }

  .raw-analysis {
    padding: 1rem;
    background: var(--bg-secondary);
    border-radius: 0.5rem;
    border: 1px solid var(--border-color);
  }

  .raw-analysis pre {
    margin: 0;
    white-space: pre-wrap;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .style-prompt {
    margin-bottom: 2rem;
  }

  .style-prompt h4 {
    margin: 0 0 1rem 0;
    color: var(--text-primary);
  }

  .prompt-content {
    padding: 1.5rem;
    background: var(--success-bg);
    border-radius: 0.5rem;
    border: 1px solid var(--success-border);
    line-height: 1.6;
    color: var(--success-text);
    font-weight: 500;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 1rem;
    padding: 1rem 0 0 0;
    border-top: 1px solid var(--border-color);
  }

  .delete-btn {
    color: var(--error-text) !important;
  }

  .delete-btn:hover {
    background: var(--error-bg) !important;
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .manager-header {
      flex-direction: column;
      gap: 1rem;
      padding: 1.5rem;
    }

    .search-section {
      flex-direction: column;
      align-items: stretch;
    }

    .stats {
      justify-content: center;
    }

    .examples-grid {
      grid-template-columns: 1fr;
    }

    .example-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
    }

    .example-actions {
      justify-content: center;
    }

    .detail-header {
      flex-direction: column;
      gap: 1rem;
      align-items: stretch;
    }

    .detail-actions {
      justify-content: center;
    }

    .analysis-grid {
      grid-template-columns: 1fr;
    }

    .modal-actions {
      flex-direction: column;
    }
  }
</style>