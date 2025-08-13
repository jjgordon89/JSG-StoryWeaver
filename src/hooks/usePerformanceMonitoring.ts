import { useEffect, useRef } from 'react';
import { usePerformanceStore } from '../stores/performanceStore';

/**
 * Configuration options for performance monitoring.
 */
interface PerformanceOptions {
  /** Component category for grouping metrics */
  component: string;
  /** Whether monitoring is enabled for this component */
  enabled?: boolean;
  /** Track component render times */
  trackRender?: boolean;
  /** Track memory usage (Chrome only) */
  trackMemory?: boolean;
  /** Enable custom event tracking */
  trackEvents?: boolean;
  /** Sample rate (0-1) for render tracking to reduce overhead */
  sampleRate?: number;
}

/**
 * Hook for monitoring component performance and collecting metrics.
 * 
 * Provides comprehensive performance monitoring for React components including
 * render times, memory usage, and custom event tracking. Uses sampling to
 * minimize performance impact while collecting useful metrics.
 * 
 * @param {string} componentName - Name of the component to monitor
 * @param {PerformanceOptions} [options] - Performance monitoring configuration
 * @returns {Object} Performance tracking interface
 * @returns {Function} trackEvent - Track custom event durations
 * @returns {Function} trackMetric - Track custom metric values
 * 
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { trackEvent, trackMetric } = usePerformanceMonitoring('MyComponent', {
 *     component: 'ui',
 *     trackRender: true,
 *     trackMemory: true,
 *     sampleRate: 0.1 // Track 10% of renders
 *   });
 * 
 *   const handleClick = async () => {
 *     trackEvent('button_click', true); // Start timing
 *     await performExpensiveOperation();
 *     trackEvent('button_click', false); // End timing
 *     
 *     trackMetric('items_processed', 42, 'count');
 *   };
 * 
 *   return <button onClick={handleClick}>Process</button>;
 * }
 * ```
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
     * Track a custom event's duration with start/end timing.
     * 
     * Call with startEvent=true to begin timing, then call with startEvent=false
     * to end timing and record the duration metric.
     * 
     * @param {string} eventName - Name of the event to track
     * @param {boolean} [startEvent=true] - Whether to start (true) or end (false) timing
     * 
     * @example
     * ```tsx
     * // Start timing an operation
     * trackEvent('data_fetch', true);
     * 
     * // ... perform operation ...
     * 
     * // End timing and record duration
     * trackEvent('data_fetch', false);
     * ```
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
     * Track a custom value metric.
     * 
     * Records a specific numeric value with optional unit for custom metrics
     * like counts, sizes, or other measurable values.
     * 
     * @param {string} metricName - Name of the metric to track
     * @param {number} value - Numeric value to record
     * @param {string} [unit] - Optional unit of measurement (e.g., 'ms', 'MB', 'count')
     * 
     * @example
     * ```tsx
     * // Track various metrics
     * trackMetric('items_loaded', 150, 'count');
     * trackMetric('response_size', 2.5, 'MB');
     * trackMetric('user_score', 95, 'points');
     * ```
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
