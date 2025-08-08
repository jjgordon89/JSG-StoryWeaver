<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { 
    seriesConsistencyActions, 
    consistencyReports, 
    consistencyLoading, 
    consistencyErrors,
    filteredConflicts,
    filtersState
  } from '../stores/seriesConsistencyStore';
  import { ConflictSeverity, ConflictType } from '../types/seriesConsistency';
  import type { SeriesConsistencyReport, ConsistencyConflict } from '../types/seriesConsistency';
  import Button from './ui/Button.svelte';
  import Card from './ui/Card.svelte';
  import LoadingSpinner from './ui/LoadingSpinner.svelte';
  import ErrorMessage from './ui/ErrorMessage.svelte';
  import Input from './ui/Input.svelte';
  import Select from './ui/Select.svelte';
  import Badge from './ui/Badge.svelte';
  
  export let seriesId: string;
  export let seriesName: string = '';
  
  let report: SeriesConsistencyReport | null = null;
  let loading = false;
  let error: string | null = null;
  let refreshInterval: number;
  
  // Filter state
  let selectedSeverities: ConflictSeverity[] = [];
  let searchTerm = '';
  
  // Reactive statements
  $: report = $consistencyReports[seriesId] || null;
  $: loading = $consistencyLoading[seriesId] || false;
  $: error = $consistencyErrors[seriesId] || null;
  $: conflicts = $filteredConflicts;
  
  // Severity options for filter
  const severityOptions = [
    { value: ConflictSeverity.Critical, label: 'Critical', color: 'red' },
    { value: ConflictSeverity.High, label: 'High', color: 'orange' },
    { value: ConflictSeverity.Medium, label: 'Medium', color: 'yellow' },
    { value: ConflictSeverity.Low, label: 'Low', color: 'blue' }
  ];
  
  // Conflict type labels
  const conflictTypeLabels = {
    [ConflictType.CharacterInconsistency]: 'Character Inconsistency',
    [ConflictType.WorldElementInconsistency]: 'World Element Inconsistency',
    [ConflictType.StoryBibleMismatch]: 'Story Bible Mismatch',
    [ConflictType.TimelineConflict]: 'Timeline Conflict',
    [ConflictType.PlotInconsistency]: 'Plot Inconsistency'
  };
  
  onMount(async () => {
    await loadReport();
    
    // Auto-refresh every 5 minutes
    refreshInterval = setInterval(() => {
      if (seriesConsistencyActions.needsRefresh(seriesId)) {
        loadReport();
      }
    }, 5 * 60 * 1000);
  });
  
  onDestroy(() => {
    if (refreshInterval) {
      clearInterval(refreshInterval);
    }
  });
  
  async function loadReport() {
    try {
      await seriesConsistencyActions.generateReport(seriesId);
    } catch (err) {
      console.error('Failed to load consistency report:', err);
    }
  }
  
  function handleRefresh() {
    loadReport();
  }
  
  function updateFilters() {
    seriesConsistencyActions.updateFilters({
      severity: selectedSeverities,
      searchTerm
    });
  }
  
  function clearFilters() {
    selectedSeverities = [];
    searchTerm = '';
    seriesConsistencyActions.resetFilters();
  }
  
  function getSeverityColor(severity: ConflictSeverity): string {
    const option = severityOptions.find(opt => opt.value === severity);
    return option?.color || 'gray';
  }
  
  function getScoreColor(score: number): string {
    if (score >= 0.9) return 'text-green-600';
    if (score >= 0.7) return 'text-yellow-600';
    if (score >= 0.5) return 'text-orange-600';
    return 'text-red-600';
  }
  
  function formatScore(score: number): string {
    return `${Math.round(score * 100)}%`;
  }
  
  // Update filters when local state changes
  $: if (selectedSeverities || searchTerm !== undefined) {
    updateFilters();
  }
</script>

<div class="series-consistency-report">
  <div class="header">
    <h2 class="text-2xl font-bold text-gray-900 dark:text-white">
      Series Consistency Report
      {#if seriesName}
        <span class="text-lg font-normal text-gray-600 dark:text-gray-400">- {seriesName}</span>
      {/if}
    </h2>
    
    <div class="actions">
      <Button 
        variant="outline" 
        size="sm" 
        on:click={handleRefresh}
        disabled={loading}
      >
        {#if loading}
          <LoadingSpinner size="sm" />
        {:else}
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
          </svg>
        {/if}
        Refresh
      </Button>
    </div>
  </div>
  
  {#if error}
    <ErrorMessage message={error} />
  {:else if loading && !report}
    <div class="loading-container">
      <LoadingSpinner size="lg" />
      <p class="text-gray-600 dark:text-gray-400 mt-4">Analyzing series consistency...</p>
    </div>
  {:else if report}
    <!-- Consistency Score Overview -->
    <Card class="mb-6">
      <div class="score-overview">
        <div class="score-main">
          <div class="score-circle {getScoreColor(report.consistency_score)}">
            <span class="score-text">{formatScore(report.consistency_score)}</span>
          </div>
          <div class="score-details">
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Overall Consistency</h3>
            <p class="text-sm text-gray-600 dark:text-gray-400">
              {report.total_conflicts} conflicts found
            </p>
          </div>
        </div>
        
        <div class="conflict-breakdown">
          <div class="breakdown-item">
            <span class="breakdown-label">Characters:</span>
            <span class="breakdown-value">{report.character_conflicts}</span>
          </div>
          <div class="breakdown-item">
            <span class="breakdown-label">World Elements:</span>
            <span class="breakdown-value">{report.world_element_conflicts}</span>
          </div>
          <div class="breakdown-item">
            <span class="breakdown-label">Story Bible:</span>
            <span class="breakdown-value">{report.story_bible_conflicts}</span>
          </div>
        </div>
      </div>
    </Card>
    
    <!-- Filters -->
    <Card class="mb-6">
      <div class="filters">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-4">Filter Conflicts</h3>
        
        <div class="filter-row">
          <div class="filter-group">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Severity
            </label>
            <div class="severity-filters">
              {#each severityOptions as option}
                <label class="severity-filter">
                  <input 
                    type="checkbox" 
                    bind:group={selectedSeverities} 
                    value={option.value}
                    class="mr-2"
                  />
                  <Badge color={option.color} size="sm">{option.label}</Badge>
                </label>
              {/each}
            </div>
          </div>
          
          <div class="filter-group">
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              Search
            </label>
            <Input 
              bind:value={searchTerm}
              placeholder="Search conflicts..."
              class="w-full"
            />
          </div>
          
          <div class="filter-actions">
            <Button variant="outline" size="sm" on:click={clearFilters}>
              Clear Filters
            </Button>
          </div>
        </div>
      </div>
    </Card>
    
    <!-- Conflicts List -->
    <Card>
      <div class="conflicts-header">
        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
          Conflicts ({conflicts.length})
        </h3>
      </div>
      
      {#if conflicts.length === 0}
        <div class="no-conflicts">
          <svg class="w-12 h-12 text-green-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <p class="text-gray-600 dark:text-gray-400 text-center">
            {selectedSeverities.length > 0 || searchTerm ? 'No conflicts match your filters' : 'No consistency conflicts found!'}
          </p>
        </div>
      {:else}
        <div class="conflicts-list">
          {#each conflicts as conflict}
            <div class="conflict-item">
              <div class="conflict-header">
                <div class="conflict-meta">
                  <Badge color={getSeverityColor(conflict.severity)} size="sm">
                    {conflict.severity}
                  </Badge>
                  <span class="conflict-type">
                    {conflictTypeLabels[conflict.conflict_type]}
                  </span>
                </div>
              </div>
              
              <div class="conflict-content">
                <p class="conflict-description">{conflict.description}</p>
                
                {#if conflict.affected_projects.length > 0}
                  <div class="affected-projects">
                    <span class="projects-label">Affected Projects:</span>
                    <div class="projects-list">
                      {#each conflict.affected_projects as projectId}
                        <Badge color="gray" size="xs">{projectId}</Badge>
                      {/each}
                    </div>
                  </div>
                {/if}
                
                {#if Object.keys(conflict.details).length > 0}
                  <details class="conflict-details">
                    <summary class="details-toggle">View Details</summary>
                    <pre class="details-content">{JSON.stringify(conflict.details, null, 2)}</pre>
                  </details>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </Card>
    
    <div class="report-footer">
      <p class="text-sm text-gray-500 dark:text-gray-400">
        Report generated: {new Date(report.generated_at).toLocaleString()}
      </p>
    </div>
  {:else}
    <Card>
      <div class="empty-state">
        <svg class="w-12 h-12 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
        </svg>
        <p class="text-gray-600 dark:text-gray-400 text-center mb-4">
          No consistency report available for this series.
        </p>
        <Button on:click={loadReport}>Generate Report</Button>
      </div>
    </Card>
  {/if}
</div>

<style>
  .series-consistency-report {
    @apply space-y-6;
  }
  
  .header {
    @apply flex justify-between items-center;
  }
  
  .actions {
    @apply flex gap-2;
  }
  
  .loading-container {
    @apply flex flex-col items-center justify-center py-12;
  }
  
  .score-overview {
    @apply flex items-center justify-between;
  }
  
  .score-main {
    @apply flex items-center gap-4;
  }
  
  .score-circle {
    @apply w-20 h-20 rounded-full border-4 flex items-center justify-center font-bold text-lg;
  }
  
  .score-text {
    @apply text-2xl font-bold;
  }
  
  .conflict-breakdown {
    @apply space-y-2;
  }
  
  .breakdown-item {
    @apply flex justify-between;
  }
  
  .breakdown-label {
    @apply text-sm text-gray-600 dark:text-gray-400;
  }
  
  .breakdown-value {
    @apply text-sm font-semibold text-gray-900 dark:text-white;
  }
  
  .filters {
    @apply space-y-4;
  }
  
  .filter-row {
    @apply grid grid-cols-1 md:grid-cols-3 gap-4 items-end;
  }
  
  .filter-group {
    @apply space-y-2;
  }
  
  .severity-filters {
    @apply flex flex-wrap gap-2;
  }
  
  .severity-filter {
    @apply flex items-center;
  }
  
  .conflicts-header {
    @apply border-b border-gray-200 dark:border-gray-700 pb-4 mb-4;
  }
  
  .no-conflicts {
    @apply py-12 text-center;
  }
  
  .conflicts-list {
    @apply space-y-4;
  }
  
  .conflict-item {
    @apply border border-gray-200 dark:border-gray-700 rounded-lg p-4;
  }
  
  .conflict-header {
    @apply mb-3;
  }
  
  .conflict-meta {
    @apply flex items-center gap-3;
  }
  
  .conflict-type {
    @apply text-sm font-medium text-gray-700 dark:text-gray-300;
  }
  
  .conflict-content {
    @apply space-y-3;
  }
  
  .conflict-description {
    @apply text-gray-900 dark:text-white;
  }
  
  .affected-projects {
    @apply space-y-2;
  }
  
  .projects-label {
    @apply text-sm font-medium text-gray-700 dark:text-gray-300;
  }
  
  .projects-list {
    @apply flex flex-wrap gap-1;
  }
  
  .conflict-details {
    @apply mt-3;
  }
  
  .details-toggle {
    @apply text-sm text-blue-600 dark:text-blue-400 cursor-pointer;
  }
  
  .details-content {
    @apply mt-2 p-3 bg-gray-50 dark:bg-gray-800 rounded text-xs overflow-x-auto;
  }
  
  .report-footer {
    @apply text-center pt-4;
  }
  
  .empty-state {
    @apply py-12 text-center;
  }
</style>