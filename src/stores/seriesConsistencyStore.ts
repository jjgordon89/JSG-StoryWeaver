// Series consistency store for managing consistency checking state

import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { invoke } from '../utils/tauriSafe';
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
  filters: {
    severity: ConflictSeverity[];
    searchTerm: string;
    showResolved: boolean;
  };
}

interface SeriesConsistencyActions {
  // Report management
  generateReport: (seriesId: string) => Promise<void>;
  getStatus: (seriesId: string) => Promise<SeriesConsistencyStatus>;
  getConflictsBySeverity: (seriesId: string, severity: ConflictSeverity) => Promise<ConsistencyConflict[]>;
  batchCheck: (seriesIds: string[]) => Promise<BatchConsistencyResult[]>;
  clearReport: (seriesId: string) => void;
  clearAllReports: () => void;
  
  // Filter management
  updateFilters: (filters: Partial<SeriesConsistencyState['filters']>) => void;
  resetFilters: () => void;
  
  // Utility functions
  needsRefresh: (seriesId: string) => boolean;
  getFilteredConflicts: (seriesId: string) => ConsistencyConflict[];
}

type SeriesConsistencyStore = SeriesConsistencyState & SeriesConsistencyActions;

export const useSeriesConsistencyStore = create<SeriesConsistencyStore>()(devtools(
  persist(
    (set, get) => ({
      // Initial state
      reports: {},
      loading: {},
      errors: {},
      lastUpdated: {},
      filters: {
        severity: [],
        searchTerm: '',
        showResolved: false
      },

      // Actions
      generateReport: async (seriesId: string) => {
        set((state) => ({
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
            set((state) => ({
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
          set((state) => ({
            ...state,
            loading: { ...state.loading, [seriesId]: false },
            errors: { ...state.errors, [seriesId]: errorMessage }
          }));
          throw error;
        }
      },
  
      getStatus: async (seriesId: string): Promise<SeriesConsistencyStatus> => {
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
  
      getConflictsBySeverity: async (seriesId: string, severity: ConflictSeverity): Promise<ConsistencyConflict[]> => {
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
  
      batchCheck: async (seriesIds: string[]): Promise<BatchConsistencyResult[]> => {
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
  
      clearReport: (seriesId: string) => {
        set((state) => {
          const newReports = { ...state.reports };
          const newLoading = { ...state.loading };
          const newErrors = { ...state.errors };
          const newLastUpdated = { ...state.lastUpdated };
          
          delete newReports[seriesId];
          delete newLoading[seriesId];
          delete newErrors[seriesId];
          delete newLastUpdated[seriesId];
          
          return {
            ...state,
            reports: newReports,
            loading: newLoading,
            errors: newErrors,
            lastUpdated: newLastUpdated
          };
        });
      },
  
      clearAllReports: () => {
        set((state) => ({
          ...state,
          reports: {},
          loading: {},
          errors: {},
          lastUpdated: {}
        }));
      },
  
      updateFilters: (filters: Partial<SeriesConsistencyState['filters']>) => {
        set((state) => ({
          ...state,
          filters: { ...state.filters, ...filters }
        }));
      },
  
      resetFilters: () => {
        set((state) => ({
          ...state,
          filters: {
            severity: [],
            searchTerm: '',
            showResolved: false
          }
        }));
      },
  
      needsRefresh: (seriesId: string): boolean => {
        const state = get();
        const lastUpdated = state.lastUpdated[seriesId];
        if (!lastUpdated) return true;
        
        const fiveMinutes = 5 * 60 * 1000;
        return Date.now() - lastUpdated > fiveMinutes;
      },

      getFilteredConflicts: (seriesId: string): ConsistencyConflict[] => {
        const state = get();
        const report = state.reports[seriesId];
        if (!report) return [];

        let conflicts = report.conflicts;
        const { severity, searchTerm, showResolved } = state.filters;

        // Filter by severity
        if (severity.length > 0) {
          conflicts = conflicts.filter(conflict => severity.includes(conflict.severity));
        }

        // Filter by search term
        if (searchTerm) {
          const searchLower = searchTerm.toLowerCase();
          conflicts = conflicts.filter(conflict => {
            const matchesDescription = conflict.description.toLowerCase().includes(searchLower);
            const matchesProjects = conflict.affected_projects.some(project => 
              project.toLowerCase().includes(searchLower)
            );
            return matchesDescription || matchesProjects;
          });
        }

        return conflicts;
      }
    }),
    {
      name: 'series-consistency-store',
      partialize: (state) => ({
        reports: state.reports,
        lastUpdated: state.lastUpdated,
        filters: state.filters
      })
    }
  ),
  {
    name: 'SeriesConsistencyStore'
  }
));