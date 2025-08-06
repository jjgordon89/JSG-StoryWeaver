import React, { useEffect, useState } from 'react';
import { usePerformanceStore } from '../../stores/performanceStore';

export const PerformanceSettings: React.FC = () => {
  const {
    monitoringEnabled,
    bottleneckDetectionEnabled,
    memoryMonitoringEnabled,
    queryPerformanceTrackingEnabled,
    toggleMonitoring,
    toggleBottleneckDetection,
    toggleMemoryMonitoring,
    toggleQueryPerformanceTracking,
    fetchSummary,
    summary,
    cleanupOldMetrics,
    isLoading,
    error
  } = usePerformanceStore();
  
  const [cleanupCount, setCleanupCount] = useState<number | null>(null);
  const [cleanupLoading, setCleanupLoading] = useState(false);
  
  // Load performance summary when component mounts
  useEffect(() => {
    fetchSummary();
  }, [fetchSummary]);
  
  // Handle cleanup of old metrics
  const handleCleanup = async () => {
    setCleanupLoading(true);
    try {
      const count = await cleanupOldMetrics();
      setCleanupCount(count);
    } finally {
      setCleanupLoading(false);
    }
  };
  
  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Performance Monitoring</h1>
      
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          Error: {error}
        </div>
      )}
      
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Monitoring Settings</h2>
        
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="flex items-center">
            <input
              type="checkbox"
              id="monitoringEnabled"
              checked={monitoringEnabled}
              onChange={(e) => toggleMonitoring(e.target.checked)}
              className="mr-2"
            />
            <label htmlFor="monitoringEnabled">Enable Performance Monitoring</label>
          </div>
          
          <div className="flex items-center">
            <input
              type="checkbox"
              id="bottleneckDetectionEnabled"
              checked={bottleneckDetectionEnabled}
              onChange={(e) => toggleBottleneckDetection(e.target.checked)}
              className="mr-2"
              disabled={!monitoringEnabled}
            />
            <label 
              htmlFor="bottleneckDetectionEnabled" 
              className={!monitoringEnabled ? "text-gray-400" : ""}
            >
              Enable Bottleneck Detection
            </label>
          </div>
          
          <div className="flex items-center">
            <input
              type="checkbox"
              id="memoryMonitoringEnabled"
              checked={memoryMonitoringEnabled}
              onChange={(e) => toggleMemoryMonitoring(e.target.checked)}
              className="mr-2"
              disabled={!monitoringEnabled}
            />
            <label 
              htmlFor="memoryMonitoringEnabled" 
              className={!monitoringEnabled ? "text-gray-400" : ""}
            >
              Enable Memory Monitoring
            </label>
          </div>
          
          <div className="flex items-center">
            <input
              type="checkbox"
              id="queryPerformanceTrackingEnabled"
              checked={queryPerformanceTrackingEnabled}
              onChange={(e) => toggleQueryPerformanceTracking(e.target.checked)}
              className="mr-2"
              disabled={!monitoringEnabled}
            />
            <label 
              htmlFor="queryPerformanceTrackingEnabled" 
              className={!monitoringEnabled ? "text-gray-400" : ""}
            >
              Enable Query Performance Tracking
            </label>
          </div>
        </div>
      </div>
      
      {summary && (
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Performance Summary</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="bg-white dark:bg-gray-800 p-4 rounded shadow">
              <h3 className="text-lg font-medium mb-2">Metrics Overview</h3>
              <p>Total Metrics: {summary.total_metrics_count}</p>
              <div className="mt-2">
                <h4 className="font-medium">Metrics by Component:</h4>
                <ul className="list-disc pl-5">
                  {summary.metrics_by_component.map((item, index) => (
                    <li key={index}>
                      {item.component}: {item.count}
                    </li>
                  ))}
                </ul>
              </div>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded shadow">
              <h3 className="text-lg font-medium mb-2">Bottlenecks</h3>
              <p>Active: <span className={summary.active_bottlenecks > 0 ? "text-red-500 font-bold" : ""}>
                {summary.active_bottlenecks}
              </span></p>
              <p>Resolved: {summary.resolved_bottlenecks}</p>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded shadow">
              <h3 className="text-lg font-medium mb-2">Database Performance</h3>
              <p>Average Query Time: {summary.average_query_time_ms.toFixed(2)} ms</p>
              <p>Slow Query Percentage: {summary.slow_query_percentage.toFixed(2)}%</p>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded shadow">
              <h3 className="text-lg font-medium mb-2">Memory Usage</h3>
              {summary.memory_usage_trend.length > 0 ? (
                <div>
                  <p>Latest: {summary.memory_usage_trend[0].used_memory_mb.toFixed(2)} MB</p>
                  <p>Trend: {summary.memory_usage_trend.length} data points</p>
                </div>
              ) : (
                <p>No memory data available</p>
              )}
            </div>
          </div>
        </div>
      )}
      
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Maintenance</h2>
        
        <div className="flex items-center">
          <button
            onClick={handleCleanup}
            disabled={cleanupLoading || !monitoringEnabled}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:bg-gray-400"
          >
            {cleanupLoading ? 'Cleaning up...' : 'Clean up old metrics'}
          </button>
          
          {cleanupCount !== null && (
            <span className="ml-4">
              {cleanupCount} metrics removed
            </span>
          )}
        </div>
        
        <p className="mt-2 text-sm text-gray-500">
          This will remove metrics older than the retention period set in the database settings.
        </p>
      </div>
      
      <div className="mt-8">
        <button
          onClick={() => fetchSummary()}
          disabled={isLoading}
          className="px-4 py-2 bg-gray-200 dark:bg-gray-700 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
        >
          {isLoading ? 'Refreshing...' : 'Refresh Data'}
        </button>
      </div>
    </div>
  );
};
