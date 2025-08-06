import React, { useEffect, useState } from 'react';
import { usePerformanceStore, PerformanceMetric, MemoryTrendPoint } from '../../stores/performanceStore';

// Simple line chart component for memory usage trend
const MemoryUsageChart: React.FC<{ data: MemoryTrendPoint[] }> = ({ data }) => {
  if (data.length === 0) return <div className="text-center p-4">No memory data available</div>;
  
  // Find min and max values for scaling
  const values = data.map(point => point.used_memory_mb);
  const maxValue = Math.max(...values);
  const minValue = Math.min(...values);
  const range = maxValue - minValue;
  
  // Chart dimensions
  const height = 150;
  const width = 400;
  const padding = 20;
  
  // Scale points to fit chart
  const points = data.map((point, index) => {
    const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
    const normalizedValue = range === 0 ? 0.5 : (point.used_memory_mb - minValue) / range;
    const y = height - padding - normalizedValue * (height - 2 * padding);
    return `${x},${y}`;
  }).join(' ');
  
  return (
    <div className="mt-4">
      <svg width={width} height={height} className="bg-white dark:bg-gray-800 rounded shadow">
        {/* Chart grid */}
        <line x1={padding} y1={height - padding} x2={width - padding} y2={height - padding} stroke="#ccc" strokeWidth="1" />
        <line x1={padding} y1={padding} x2={padding} y2={height - padding} stroke="#ccc" strokeWidth="1" />
        
        {/* Memory usage line */}
        <polyline
          points={points}
          fill="none"
          stroke="#3b82f6"
          strokeWidth="2"
        />
        
        {/* Data points */}
        {data.map((point, index) => {
          const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
          const normalizedValue = range === 0 ? 0.5 : (point.used_memory_mb - minValue) / range;
          const y = height - padding - normalizedValue * (height - 2 * padding);
          
          return (
            <circle
              key={index}
              cx={x}
              cy={y}
              r="3"
              fill="#3b82f6"
            />
          );
        })}
        
        {/* Y-axis labels */}
        <text x={5} y={padding} fontSize="10" fill="#666">
          {maxValue.toFixed(1)} MB
        </text>
        <text x={5} y={height - padding} fontSize="10" fill="#666">
          {minValue.toFixed(1)} MB
        </text>
      </svg>
      <div className="flex justify-between text-xs text-gray-500 mt-1">
        <span>{new Date(data[data.length - 1].timestamp).toLocaleTimeString()}</span>
        <span>{new Date(data[0].timestamp).toLocaleTimeString()}</span>
      </div>
    </div>
  );
};

// Component to display metrics in a table
const MetricsTable: React.FC<{ metrics: PerformanceMetric[] }> = ({ metrics }) => {
  if (metrics.length === 0) return <div className="text-center p-4">No metrics available</div>;
  
  return (
    <div className="overflow-x-auto">
      <table className="min-w-full bg-white dark:bg-gray-800 rounded shadow">
        <thead>
          <tr className="bg-gray-100 dark:bg-gray-700">
            <th className="px-4 py-2 text-left">Metric</th>
            <th className="px-4 py-2 text-left">Value</th>
            <th className="px-4 py-2 text-left">Component</th>
            <th className="px-4 py-2 text-left">Time</th>
          </tr>
        </thead>
        <tbody>
          {metrics.map((metric) => (
            <tr key={metric.id} className="border-t border-gray-200 dark:border-gray-700">
              <td className="px-4 py-2">{metric.metric_name}</td>
              <td className="px-4 py-2">
                {metric.metric_value.toFixed(2)} {metric.metric_unit || ''}
              </td>
              <td className="px-4 py-2">{metric.component}</td>
              <td className="px-4 py-2">{new Date(metric.recorded_at).toLocaleTimeString()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

// Bottleneck indicator component
const BottleneckIndicator: React.FC<{ count: number }> = ({ count }) => {
  const getColor = () => {
    if (count === 0) return 'bg-green-500';
    if (count < 3) return 'bg-yellow-500';
    return 'bg-red-500';
  };
  
  return (
    <div className="flex items-center">
      <div className={`w-3 h-3 rounded-full ${getColor()} mr-2`}></div>
      <span>{count} active bottlenecks</span>
    </div>
  );
};

// Main dashboard component
export const PerformanceDashboard: React.FC = () => {
  const {
    fetchSummary,
    fetchMetricsByComponent,
    summary,
    metrics,
    isLoading,
    error
  } = usePerformanceStore();
  
  const [selectedComponent, setSelectedComponent] = useState<string>('all');
  const [refreshInterval, setRefreshInterval] = useState<number | null>(null);
  
  // Load summary when component mounts
  useEffect(() => {
    fetchSummary();
    
    // Initial metrics load
    if (selectedComponent === 'all') {
      // Just load database metrics as an example
      fetchMetricsByComponent('database', 10);
    } else {
      fetchMetricsByComponent(selectedComponent, 10);
    }
  }, [fetchSummary, fetchMetricsByComponent, selectedComponent]);
  
  // Handle auto-refresh
  useEffect(() => {
    if (refreshInterval) {
      const intervalId = setInterval(() => {
        fetchSummary();
        if (selectedComponent !== 'all') {
          fetchMetricsByComponent(selectedComponent, 10);
        }
      }, refreshInterval * 1000);
      
      return () => clearInterval(intervalId);
    }
  }, [refreshInterval, fetchSummary, fetchMetricsByComponent, selectedComponent]);
  
  // Handle component selection change
  const handleComponentChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedComponent(e.target.value);
  };
  
  // Handle refresh interval change
  const handleRefreshIntervalChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const value = e.target.value;
    setRefreshInterval(value === 'none' ? null : parseInt(value));
  };
  
  return (
    <div className="p-6 max-w-4xl mx-auto">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold">Performance Dashboard</h1>
        
        <div className="flex space-x-4">
          <div>
            <label htmlFor="componentSelect" className="mr-2">Component:</label>
            <select
              id="componentSelect"
              value={selectedComponent}
              onChange={handleComponentChange}
              className="p-1 border rounded"
            >
              <option value="all">All</option>
              <option value="database">Database</option>
              <option value="ui">UI</option>
              <option value="ai">AI</option>
              <option value="system">System</option>
              <option value="editor">Editor</option>
              <option value="file_io">File I/O</option>
              <option value="network">Network</option>
            </select>
          </div>
          
          <div>
            <label htmlFor="refreshInterval" className="mr-2">Auto-refresh:</label>
            <select
              id="refreshInterval"
              value={refreshInterval?.toString() || 'none'}
              onChange={handleRefreshIntervalChange}
              className="p-1 border rounded"
            >
              <option value="none">Off</option>
              <option value="5">5s</option>
              <option value="10">10s</option>
              <option value="30">30s</option>
              <option value="60">60s</option>
            </select>
          </div>
          
          <button
            onClick={() => {
              fetchSummary();
              if (selectedComponent !== 'all') {
                fetchMetricsByComponent(selectedComponent, 10);
              }
            }}
            disabled={isLoading}
            className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:bg-gray-400"
          >
            {isLoading ? 'Loading...' : 'Refresh'}
          </button>
        </div>
      </div>
      
      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          Error: {error}
        </div>
      )}
      
      {summary && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
          <div className="bg-white dark:bg-gray-800 p-4 rounded shadow">
            <h2 className="text-lg font-medium mb-3">System Health</h2>
            <BottleneckIndicator count={summary.active_bottlenecks} />
            <div className="mt-2">
              <p>Total Metrics: {summary.total_metrics_count}</p>
              <p>Resolved Bottlenecks: {summary.resolved_bottlenecks}</p>
            </div>
          </div>
          
          <div className="bg-white dark:bg-gray-800 p-4 rounded shadow">
            <h2 className="text-lg font-medium mb-3">Database Performance</h2>
            <p>Average Query Time: {summary.average_query_time_ms.toFixed(2)} ms</p>
            <p className={summary.slow_query_percentage > 5 ? 'text-red-500' : ''}>
              Slow Query Percentage: {summary.slow_query_percentage.toFixed(2)}%
            </p>
          </div>
          
          <div className="bg-white dark:bg-gray-800 p-4 rounded shadow md:col-span-2">
            <h2 className="text-lg font-medium mb-3">Memory Usage Trend</h2>
            <MemoryUsageChart data={summary.memory_usage_trend} />
          </div>
        </div>
      )}
      
      <div className="mb-8">
        <h2 className="text-xl font-semibold mb-4">Recent Metrics</h2>
        <MetricsTable metrics={metrics} />
      </div>
      
      <div className="text-sm text-gray-500 mt-8">
        <p>
          Note: This dashboard shows a simplified view of performance metrics.
          For more detailed analysis, use the Performance Settings page.
        </p>
      </div>
    </div>
  );
};
