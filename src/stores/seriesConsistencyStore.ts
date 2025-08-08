// Series consistency store for managing consistency checking state

import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type {
  SeriesConsistencyReport,
  ConsistencyConflict,
  ConflictSeverity,
  SeriesConsistencyStatus,
  BatchConsistencyResult,
  ConsistencyFilters
} from '../types/seriesConsistency';

// Store state interfaces
interface SeriesConsistencyState {
  reports: Record<string, SeriesConsistencyReport>;
  loading: Record<string, boolean>;
  errors: Record<string, string | null>;
  lastUpdated: Record<string, number>;
}

interface ConsistencyFiltersState {
  severity: ConflictSeverity[];
  searchTerm: string;
  showResolved: boolean;
}

// Create stores
const consistencyState = writable<SeriesConsistencyState>({
  reports: {},
  loading: {},
  errors: {},
  lastUpdated: {}
});

const filtersState = writable<ConsistencyFiltersState>({
  severity: [],
  searchTerm: '',
  showResolved: false
});

// Derived stores
export const consistencyReports = derived(
  consistencyState,
  $state => $state.reports
);

export const consistencyLoading = derived(
  consistencyState,
  $state => $state.loading
);

export const consistencyErrors = derived(
  consistencyState,
  $state => $state.errors
);

// Filtered conflicts derived store
export const filteredConflicts = derived(
  [consistencyState, filtersState],
  ([$state, $filters]) => {
    const allConflicts: ConsistencyConflict[] = [];
    
    Object.values($state.reports).forEach(report => {
      allConflicts.push(...report.conflicts);
    });
    
    return allConflicts.filter(conflict => {
      // Filter by severity
      if ($filters.severity.length > 0 && !$filters.severity.includes(conflict.severity)) {
        return false;
      }
      
      // Filter by search term
      if ($filters.searchTerm) {
        const searchLower = $filters.searchTerm.toLowerCase();
        const matchesDescription = conflict.description.toLowerCase().includes(searchLower);
        const matchesProjects = conflict.affected_projects.some(project => 
          project.toLowerCase().includes(searchLower)
        );
        if (!matchesDescription && !matchesProjects) {
          return false;
        }
      }
      
      return true;
    });
  }
);

// Actions
export const seriesConsistencyActions = {
  // Generate full consistency report for a series
  async generateReport(seriesId: string): Promise<void> {
    consistencyState.update(state => ({
      ...state,
      loading: { ...state.loading, [seriesId]: true },
      errors: { ...state.errors, [seriesId]: null }
    }));
    
    try {
      const response = await invoke<{ success: boolean; data?: SeriesConsistencyReport; error?: string }>(
        'generate_series_consistency_report',
        { seriesId }
      );
      
      if (response.success && response.data) {
        consistencyState.update(state => ({
          ...state,
          reports: { ...state.reports, [seriesId]: response.data! },
          loading: { ...state.loading, [seriesId]: false },
          lastUpdated: { ...state.lastUpdated, [seriesId]: Date.now() }
        }));
      } else {
        throw new Error(response.error || 'Failed to generate consistency report');
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      consistencyState.update(state => ({
        ...state,
        loading: { ...state.loading, [seriesId]: false },
        errors: { ...state.errors, [seriesId]: errorMessage }
      }));
      throw error;
    }
  },
  
  // Get quick consistency status
  async getStatus(seriesId: string): Promise<SeriesConsistencyStatus> {
    try {
      const response = await invoke<{ success: boolean; data?: [number, number]; error?: string }>(
        'get_series_consistency_status',
        { seriesId }
      );
      
      if (response.success && response.data) {
        const [consistency_score, conflict_count] = response.data;
        return { consistency_score, conflict_count };
      } else {
        throw new Error(response.error || 'Failed to get consistency status');
      }
    } catch (error) {
      console.error('Error getting consistency status:', error);
      throw error;
    }
  },
  
  // Get conflicts by severity
  async getConflictsBySeverity(seriesId: string, severity: ConflictSeverity): Promise<ConsistencyConflict[]> {
    try {
      const response = await invoke<{ success: boolean; data?: ConsistencyConflict[]; error?: string }>(
        'get_series_conflicts_by_severity',
        { seriesId, severity }
      );
      
      if (response.success && response.data) {
        return response.data;
      } else {
        throw new Error(response.error || 'Failed to get conflicts by severity');
      }
    } catch (error) {
      console.error('Error getting conflicts by severity:', error);
      throw error;
    }
  },
  
  // Batch check multiple series
  async batchCheck(seriesIds: string[]): Promise<BatchConsistencyResult[]> {
    try {
      const response = await invoke<{ success: boolean; data?: [string, number, number][]; error?: string }>(
        'batch_check_series_consistency',
        { seriesIds }
      );
      
      if (response.success && response.data) {
        return response.data.map(([series_id, consistency_score, conflict_count]) => ({
          series_id,
          consistency_score,
          conflict_count
        }));
      } else {
        throw new Error(response.error || 'Failed to batch check consistency');
      }
    } catch (error) {
      console.error('Error batch checking consistency:', error);
      throw error;
    }
  },
  
  // Clear report for a series
  clearReport(seriesId: string): void {
    consistencyState.update(state => {
      const newReports = { ...state.reports };
      const newLoading = { ...state.loading };
      const newErrors = { ...state.errors };
      const newLastUpdated = { ...state.lastUpdated };
      
      delete newReports[seriesId];
      delete newLoading[seriesId];
      delete newErrors[seriesId];
      delete newLastUpdated[seriesId];
      
      return {
        reports: newReports,
        loading: newLoading,
        errors: newErrors,
        lastUpdated: newLastUpdated
      };
    });
  },
  
  // Clear all reports
  clearAllReports(): void {
    consistencyState.set({
      reports: {},
      loading: {},
      errors: {},
      lastUpdated: {}
    });
  },
  
  // Update filters
  updateFilters(filters: Partial<ConsistencyFiltersState>): void {
    filtersState.update(state => ({ ...state, ...filters }));
  },
  
  // Reset filters
  resetFilters(): void {
    filtersState.set({
      severity: [],
      searchTerm: '',
      showResolved: false
    });
  },
  
  // Check if report needs refresh (older than 5 minutes)
  needsRefresh(seriesId: string): boolean {
    const state = get(consistencyState);
    const lastUpdated = state.lastUpdated[seriesId];
    if (!lastUpdated) return true;
    
    const fiveMinutes = 5 * 60 * 1000;
    return Date.now() - lastUpdated > fiveMinutes;
  }
};

// Export stores
export { consistencyState, filtersState };