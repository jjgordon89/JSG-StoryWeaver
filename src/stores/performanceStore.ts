import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface PerformanceMetric {
  id: string;
  metric_name: string;
  metric_value: number;
  metric_unit?: string;
  context_data?: string;
  component: string;
  recorded_at: string;
}

export interface PerformanceBottleneck {
  id: string;
  component: string;
  operation: string;
  threshold_value: number;
  actual_value: number;
  severity: string;
  detected_at: string;
  resolved: boolean;
  resolved_at?: string;
  resolution_notes?: string;
}

export interface MemorySnapshot {
  id: string;
  total_memory_mb: number;
  used_memory_mb: number;
  peak_memory_mb: number;
  component_breakdown: string;
  recorded_at: string;
}

export interface QueryPerformance {
  id: string;
  query_hash: string;
  query_type: string;
  table_name: string;
  execution_time_ms: number;
  row_count?: number;
  is_slow: boolean;
  query_plan?: string;
  recorded_at: string;
}

export interface ComponentMetricCount {
  component: string;
  count: number;
}

export interface MemoryTrendPoint {
  timestamp: string;
  used_memory_mb: number;
}

export interface PerformanceMetricsSummary {
  total_metrics_count: number;
  metrics_by_component: ComponentMetricCount[];
  active_bottlenecks: number;
  resolved_bottlenecks: number;
  average_query_time_ms: number;
  slow_query_percentage: number;
  memory_usage_trend: MemoryTrendPoint[];
}

interface PerformanceState {
  // Data
  metrics: PerformanceMetric[];
  bottlenecks: PerformanceBottleneck[];
  memorySnapshots: MemorySnapshot[];
  queryPerformance: QueryPerformance[];
  summary: PerformanceMetricsSummary | null;
  
  // Status
  isLoading: boolean;
  error: string | null;
  
  // Settings
  monitoringEnabled: boolean;
  bottleneckDetectionEnabled: boolean;
  memoryMonitoringEnabled: boolean;
  queryPerformanceTrackingEnabled: boolean;
  
  // Actions
  fetchSummary: () => Promise<void>;
  fetchMetricsByName: (name: string, limit?: number) => Promise<PerformanceMetric[]>;
  fetchMetricsByComponent: (component: string, limit?: number) => Promise<PerformanceMetric[]>;
  recordMetric: (
    name: string, 
    value: number, 
    unit?: string, 
    component?: string, 
    contextData?: string
  ) => Promise<void>;
  resolveBottleneck: (id: string, notes?: string) => Promise<void>;
  cleanupOldMetrics: () => Promise<number>;
  toggleMonitoring: (enabled: boolean) => Promise<void>;
  toggleBottleneckDetection: (enabled: boolean) => Promise<void>;
  toggleMemoryMonitoring: (enabled: boolean) => Promise<void>;
  toggleQueryPerformanceTracking: (enabled: boolean) => Promise<void>;
}

export const usePerformanceStore = create<PerformanceState>()((set, get) => ({
  // Initial state
  metrics: [],
  bottlenecks: [],
  memorySnapshots: [],
  queryPerformance: [],
  summary: null,
  isLoading: false,
  error: null,
  monitoringEnabled: true,
  bottleneckDetectionEnabled: true,
  memoryMonitoringEnabled: true,
  queryPerformanceTrackingEnabled: true,
  
  // Actions
  fetchSummary: async () => {
    try {
      set({ isLoading: true, error: null });
      
      const response = await invoke<{ success: boolean; data: PerformanceMetricsSummary; error?: string }>(
        'get_performance_summary'
      );
      
      if (response.success && response.data) {
        set({ summary: response.data, isLoading: false });
      } else {
        set({ error: response.error || 'Failed to fetch performance summary', isLoading: false });
      }
    } catch (error) {
      console.error('Error fetching performance summary:', error);
      set({ error: String(error), isLoading: false });
    }
  },
  
  fetchMetricsByName: async (name: string, limit = 100) => {
    try {
      set({ isLoading: true, error: null });
      
      const response = await invoke<{ success: boolean; data: PerformanceMetric[]; error?: string }>(
        'get_metrics_by_name',
        { metricName: name, limit }
      );
      
      if (response.success && response.data) {
        set({ metrics: response.data, isLoading: false });
        return response.data;
      } else {
        set({ error: response.error || 'Failed to fetch metrics by name', isLoading: false });
        return [];
      }
    } catch (error) {
      console.error('Error fetching metrics by name:', error);
      set({ error: String(error), isLoading: false });
      return [];
    }
  },
  
  fetchMetricsByComponent: async (component: string, limit = 100) => {
    try {
      set({ isLoading: true, error: null });
      
      const response = await invoke<{ success: boolean; data: PerformanceMetric[]; error?: string }>(
        'get_metrics_by_component',
        { component, limit }
      );
      
      if (response.success && response.data) {
        set({ metrics: response.data, isLoading: false });
        return response.data;
      } else {
        set({ error: response.error || 'Failed to fetch metrics by component', isLoading: false });
        return [];
      }
    } catch (error) {
      console.error('Error fetching metrics by component:', error);
      set({ error: String(error), isLoading: false });
      return [];
    }
  },
  
  recordMetric: async (name, value, unit, component = 'system', contextData) => {
    try {
      if (!get().monitoringEnabled) return;
      
      await invoke<{ success: boolean; error?: string }>(
        'record_performance_metric',
        { 
          metricName: name, 
          metricValue: value, 
          metricUnit: unit, 
          component, 
          contextData 
        }
      );
    } catch (error) {
      console.error('Error recording performance metric:', error);
    }
  },
  
  resolveBottleneck: async (id, notes) => {
    try {
      set({ isLoading: true, error: null });
      
      const response = await invoke<{ success: boolean; error?: string }>(
        'resolve_bottleneck',
        { id, resolutionNotes: notes }
      );
      
      if (response.success) {
        // Update bottlenecks list
        const updatedBottlenecks = get().bottlenecks.map(b => 
          b.id === id ? { ...b, resolved: true, resolved_at: new Date().toISOString(), resolution_notes: notes } : b
        );
        
        set({ bottlenecks: updatedBottlenecks, isLoading: false });
      } else {
        set({ error: response.error || 'Failed to resolve bottleneck', isLoading: false });
      }
    } catch (error) {
      console.error('Error resolving bottleneck:', error);
      set({ error: String(error), isLoading: false });
    }
  },
  
  cleanupOldMetrics: async () => {
    try {
      set({ isLoading: true, error: null });
      
      const response = await invoke<{ success: boolean; data: number; error?: string }>(
        'cleanup_old_metrics'
      );
      
      set({ isLoading: false });
      
      if (response.success) {
        return response.data;
      } else {
        set({ error: response.error || 'Failed to cleanup old metrics' });
        return 0;
      }
    } catch (error) {
      console.error('Error cleaning up old metrics:', error);
      set({ error: String(error), isLoading: false });
      return 0;
    }
  },
  
  toggleMonitoring: async (enabled) => {
    try {
      await invoke('set_setting', { key: 'performance_monitoring_enabled', value: String(enabled) });
      set({ monitoringEnabled: enabled });
    } catch (error) {
      console.error('Error toggling performance monitoring:', error);
    }
  },
  
  toggleBottleneckDetection: async (enabled) => {
    try {
      await invoke('set_setting', { key: 'bottleneck_detection_enabled', value: String(enabled) });
      set({ bottleneckDetectionEnabled: enabled });
    } catch (error) {
      console.error('Error toggling bottleneck detection:', error);
    }
  },
  
  toggleMemoryMonitoring: async (enabled) => {
    try {
      await invoke('set_setting', { key: 'memory_monitoring_enabled', value: String(enabled) });
      set({ memoryMonitoringEnabled: enabled });
    } catch (error) {
      console.error('Error toggling memory monitoring:', error);
    }
  },
  
  toggleQueryPerformanceTracking: async (enabled) => {
    try {
      await invoke('set_setting', { key: 'query_performance_tracking_enabled', value: String(enabled) });
      set({ queryPerformanceTrackingEnabled: enabled });
    } catch (error) {
      console.error('Error toggling query performance tracking:', error);
    }
  }
}));
