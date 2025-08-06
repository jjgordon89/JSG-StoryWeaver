import { useEffect, useRef } from 'react';
import { usePerformanceStore } from '../stores/performanceStore';

interface PerformanceOptions {
  component: string;
  enabled?: boolean;
  trackRender?: boolean;
  trackMemory?: boolean;
  trackEvents?: boolean;
  sampleRate?: number; // 0-1, percentage of renders to track
}

/**
 * Hook for monitoring component performance
 * 
 * @param componentName Name of the component to monitor
 * @param options Performance monitoring options
 */
export function usePerformanceMonitoring(
  componentName: string,
  options: PerformanceOptions = { component: 'ui' }
) {
  const { 
    recordMetric, 
    monitoringEnabled 
  } = usePerformanceStore();
  
  const renderStartTime = useRef<number | null>(null);
  const lastEventTime = useRef<Record<string, number>>({});
  const memoryTrackingInterval = useRef<number | null>(null);
  
  // Default options
  const {
    component = 'ui',
    enabled = true,
    trackRender = true,
    trackMemory = false,
    trackEvents = true,
    sampleRate = 0.1 // Only track 10% of renders by default to reduce overhead
  } = options;
  
  // Should we track this render based on sample rate?
  const shouldTrackThisRender = useRef(Math.random() < sampleRate);
  
  // Track component render time
  useEffect(() => {
    if (!monitoringEnabled || !enabled || !trackRender || !shouldTrackThisRender.current) {
      return;
    }
    
    // Mark render start time
    renderStartTime.current = performance.now();
    
    return () => {
      // Measure render duration when component unmounts or re-renders
      if (renderStartTime.current !== null) {
        const renderDuration = performance.now() - renderStartTime.current;
        recordMetric(
          `${componentName}.render_time`,
          renderDuration,
          'ms',
          component,
          JSON.stringify({ component: componentName })
        );
        renderStartTime.current = performance.now(); // Reset for next render
      }
    };
  }, [componentName, component, monitoringEnabled, enabled, trackRender, recordMetric]);
  
  // Track memory usage if enabled
  useEffect(() => {
    if (!monitoringEnabled || !enabled || !trackMemory || !shouldTrackThisRender.current) {
      return;
    }
    
    // Check if performance.memory is available (Chrome only)
    const performanceMemory = (performance as any).memory;
    if (!performanceMemory) {
      return;
    }
    
    // Track memory initially
    recordMetric(
      `${componentName}.memory_usage`,
      performanceMemory.usedJSHeapSize / (1024 * 1024),
      'MB',
      component,
      JSON.stringify({ 
        component: componentName,
        totalHeapSize: performanceMemory.totalJSHeapSize / (1024 * 1024),
        heapLimit: performanceMemory.jsHeapSizeLimit / (1024 * 1024)
      })
    );
    
    // Set up interval to track memory usage
    const intervalId = window.setInterval(() => {
      if (!monitoringEnabled) {
        return;
      }
      
      recordMetric(
        `${componentName}.memory_usage`,
        performanceMemory.usedJSHeapSize / (1024 * 1024),
        'MB',
        component,
        JSON.stringify({ 
          component: componentName,
          totalHeapSize: performanceMemory.totalJSHeapSize / (1024 * 1024),
          heapLimit: performanceMemory.jsHeapSizeLimit / (1024 * 1024)
        })
      );
    }, 30000); // Every 30 seconds
    
    memoryTrackingInterval.current = intervalId;
    
    return () => {
      if (memoryTrackingInterval.current !== null) {
        clearInterval(memoryTrackingInterval.current);
      }
    };
  }, [componentName, component, monitoringEnabled, enabled, trackMemory, recordMetric]);
  
  // Return functions to track custom events
  return {
    /**
     * Track a custom event's duration
     * @param eventName Name of the event to track
     * @param startEvent Whether to start or end the event timing
     */
    trackEvent: (eventName: string, startEvent: boolean = true) => {
      if (!monitoringEnabled || !enabled || !trackEvents) {
        return;
      }
      
      const fullEventName = `${componentName}.${eventName}`;
      
      if (startEvent) {
        // Start timing the event
        lastEventTime.current[eventName] = performance.now();
      } else if (lastEventTime.current[eventName]) {
        // End timing and record the duration
        const duration = performance.now() - lastEventTime.current[eventName];
        recordMetric(
          fullEventName,
          duration,
          'ms',
          component,
          JSON.stringify({ component: componentName, event: eventName })
        );
        delete lastEventTime.current[eventName];
      }
    },
    
    /**
     * Track a custom value metric
     * @param metricName Name of the metric to track
     * @param value Value to record
     * @param unit Optional unit of measurement
     */
    trackMetric: (metricName: string, value: number, unit?: string) => {
      if (!monitoringEnabled || !enabled) {
        return;
      }
      
      const fullMetricName = `${componentName}.${metricName}`;
      recordMetric(
        fullMetricName,
        value,
        unit,
        component,
        JSON.stringify({ component: componentName, metric: metricName })
      );
    }
  };
}
