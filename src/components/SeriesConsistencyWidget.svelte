<script lang="ts">
  import { onMount } from 'svelte';
  import { seriesConsistencyActions } from '../stores/seriesConsistencyStore';
  import type { SeriesConsistencyStatus } from '../types/seriesConsistency';
  import Badge from './ui/Badge.svelte';
  import Button from './ui/Button.svelte';
  import LoadingSpinner from './ui/LoadingSpinner.svelte';
  
  export let seriesId: string;
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let showDetails = true;
  export let onViewReport: (() => void) | null = null;
  
  let status: SeriesConsistencyStatus | null = null;
  let loading = false;
  let error: string | null = null;
  
  onMount(async () => {
    await loadStatus();
  });
  
  async function loadStatus() {
    if (!seriesId) return;
    
    loading = true;
    error = null;
    
    try {
      status = await seriesConsistencyActions.getStatus(seriesId);
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load consistency status';
      console.error('Error loading consistency status:', err);
    } finally {
      loading = false;
    }
  }
  
  function getScoreColor(score: number): string {
    if (score >= 0.9) return 'green';
    if (score >= 0.7) return 'yellow';
    if (score >= 0.5) return 'orange';
    return 'red';
  }
  
  function getScoreLabel(score: number): string {
    if (score >= 0.9) return 'Excellent';
    if (score >= 0.7) return 'Good';
    if (score >= 0.5) return 'Fair';
    return 'Poor';
  }
  
  function formatScore(score: number): string {
    return `${Math.round(score * 100)}%`;
  }
  
  function getConflictSeverity(conflictCount: number): { color: string; label: string } {
    if (conflictCount === 0) return { color: 'green', label: 'No Issues' };
    if (conflictCount <= 3) return { color: 'yellow', label: 'Minor Issues' };
    if (conflictCount <= 10) return { color: 'orange', label: 'Some Issues' };
    return { color: 'red', label: 'Major Issues' };
  }
</script>

<div class="consistency-widget size-{size}" class:loading>
  {#if loading}
    <div class="loading-state">
      <LoadingSpinner size="sm" />
      <span class="loading-text">Checking...</span>
    </div>
  {:else if error}
    <div class="error-state">
      <svg class="error-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span class="error-text">Error</span>
    </div>
  {:else if status}
    <div class="status-content">
      <!-- Score Display -->
      <div class="score-section">
        <div class="score-circle {getScoreColor(status.consistency_score)}">
          <span class="score-value">{formatScore(status.consistency_score)}</span>
        </div>
        
        {#if showDetails}
          <div class="score-details">
            <div class="score-label">{getScoreLabel(status.consistency_score)}</div>
            <div class="score-subtitle">Consistency</div>
          </div>
        {/if}
      </div>
      
      <!-- Conflicts Display -->
      {#if showDetails}
        <div class="conflicts-section">
          <Badge 
            color={getConflictSeverity(status.conflict_count).color} 
            size={size === 'sm' ? 'xs' : 'sm'}
          >
            {status.conflict_count} {status.conflict_count === 1 ? 'Conflict' : 'Conflicts'}
          </Badge>
          
          {#if status.conflict_count > 0}
            <div class="conflict-label">
              {getConflictSeverity(status.conflict_count).label}
            </div>
          {/if}
        </div>
      {/if}
      
      <!-- Actions -->
      {#if onViewReport && size !== 'sm'}
        <div class="actions-section">
          <Button 
            variant="outline" 
            size={size === 'lg' ? 'sm' : 'xs'}
            on:click={onViewReport}
          >
            View Report
          </Button>
        </div>
      {/if}
    </div>
  {:else}
    <div class="empty-state">
      <span class="empty-text">No data</span>
    </div>
  {/if}
</div>

<style>
  .consistency-widget {
    @apply bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg;
  }
  
  .consistency-widget.loading {
    @apply opacity-75;
  }
  
  /* Size variants */
  .size-sm {
    @apply p-2;
  }
  
  .size-md {
    @apply p-4;
  }
  
  .size-lg {
    @apply p-6;
  }
  
  /* Loading state */
  .loading-state {
    @apply flex items-center justify-center gap-2;
  }
  
  .loading-text {
    @apply text-sm text-gray-600 dark:text-gray-400;
  }
  
  /* Error state */
  .error-state {
    @apply flex items-center justify-center gap-2 text-red-600 dark:text-red-400;
  }
  
  .error-icon {
    @apply w-4 h-4;
  }
  
  .error-text {
    @apply text-sm;
  }
  
  /* Status content */
  .status-content {
    @apply space-y-3;
  }
  
  .size-sm .status-content {
    @apply space-y-2;
  }
  
  .size-lg .status-content {
    @apply space-y-4;
  }
  
  /* Score section */
  .score-section {
    @apply flex items-center gap-3;
  }
  
  .size-sm .score-section {
    @apply gap-2;
  }
  
  .score-circle {
    @apply rounded-full border-2 flex items-center justify-center font-bold;
  }
  
  .size-sm .score-circle {
    @apply w-8 h-8 text-xs;
  }
  
  .size-md .score-circle {
    @apply w-12 h-12 text-sm;
  }
  
  .size-lg .score-circle {
    @apply w-16 h-16 text-base;
  }
  
  .score-circle.green {
    @apply border-green-500 text-green-600 bg-green-50 dark:bg-green-900/20;
  }
  
  .score-circle.yellow {
    @apply border-yellow-500 text-yellow-600 bg-yellow-50 dark:bg-yellow-900/20;
  }
  
  .score-circle.orange {
    @apply border-orange-500 text-orange-600 bg-orange-50 dark:bg-orange-900/20;
  }
  
  .score-circle.red {
    @apply border-red-500 text-red-600 bg-red-50 dark:bg-red-900/20;
  }
  
  .score-details {
    @apply flex-1;
  }
  
  .score-label {
    @apply font-semibold text-gray-900 dark:text-white;
  }
  
  .size-sm .score-label {
    @apply text-sm;
  }
  
  .size-md .score-label {
    @apply text-base;
  }
  
  .size-lg .score-label {
    @apply text-lg;
  }
  
  .score-subtitle {
    @apply text-sm text-gray-600 dark:text-gray-400;
  }
  
  .size-sm .score-subtitle {
    @apply text-xs;
  }
  
  /* Conflicts section */
  .conflicts-section {
    @apply space-y-1;
  }
  
  .conflict-label {
    @apply text-xs text-gray-600 dark:text-gray-400;
  }
  
  /* Actions section */
  .actions-section {
    @apply flex justify-center;
  }
  
  /* Empty state */
  .empty-state {
    @apply flex items-center justify-center;
  }
  
  .empty-text {
    @apply text-sm text-gray-500 dark:text-gray-400;
  }
</style>