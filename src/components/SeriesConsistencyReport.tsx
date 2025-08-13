import React, { useEffect, useState, useCallback } from 'react';
import { useSeriesConsistencyStore } from '../stores/seriesConsistencyStore';
import { ConflictSeverity, ConflictType } from '../types/seriesConsistency';
import type { SeriesConsistencyReport, ConsistencyConflict } from '../types/seriesConsistency';
import { LoadingSpinner, ErrorMessage } from './ui';

interface SeriesConsistencyReportProps {
  seriesId: string;
  seriesName?: string;
}

// Simple UI Components
const Button: React.FC<{
  variant?: 'outline' | 'solid';
  size?: 'sm' | 'md' | 'lg';
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  children: React.ReactNode;
}> = ({ variant = 'solid', size = 'md', onClick, disabled, className = '', children }) => {
  const baseClasses = 'inline-flex items-center justify-center rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2';
  const variantClasses = variant === 'outline' 
    ? 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'
    : 'bg-blue-600 text-white hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600';
  const sizeClasses = size === 'sm' ? 'px-3 py-1.5 text-sm' : size === 'lg' ? 'px-6 py-3 text-lg' : 'px-4 py-2 text-base';
  const disabledClasses = disabled ? 'opacity-50 cursor-not-allowed' : '';
  
  return (
    <button
      className={`${baseClasses} ${variantClasses} ${sizeClasses} ${disabledClasses} ${className}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};

const Card: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-sm ${className}`}>
    {children}
  </div>
);

const Badge: React.FC<{
  color?: string;
  size?: 'xs' | 'sm' | 'md';
  children: React.ReactNode;
}> = ({ color = 'gray', size = 'sm', children }) => {
  const colorClasses = {
    red: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200',
    orange: 'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200',
    yellow: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200',
    blue: 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200',
    gray: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
  }[color] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200';
  
  const sizeClasses = size === 'xs' ? 'px-2 py-0.5 text-xs' : size === 'md' ? 'px-3 py-1 text-base' : 'px-2.5 py-0.5 text-sm';
  
  return (
    <span className={`inline-flex items-center rounded-full font-medium ${colorClasses} ${sizeClasses}`}>
      {children}
    </span>
  );
};

export const SeriesConsistencyReportComponent: React.FC<SeriesConsistencyReportProps> = ({ 
  seriesId, 
  seriesName = '' 
}) => {
  const {
    reports,
    loading,
    errors,
    filters,
    generateReport,
    updateFilters,
    resetFilters,
    needsRefresh,
    getFilteredConflicts
  } = useSeriesConsistencyStore();

  const [selectedSeverities, setSelectedSeverities] = useState<ConflictSeverity[]>([]);
  const [searchTerm, setSearchTerm] = useState('');

  const report = reports[seriesId] || null;
  const isLoading = loading[seriesId] || false;
  const error = errors[seriesId] || null;
  const conflicts = report ? getFilteredConflicts(seriesId) : [];

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

  const loadReport = useCallback(async () => {
    try {
      await generateReport(seriesId);
    } catch (err) {
      console.error('Failed to load consistency report:', err);
    }
  }, [seriesId, generateReport]);

  useEffect(() => {
    loadReport();
    
    // Auto-refresh every 5 minutes
    const interval = setInterval(() => {
      if (needsRefresh(seriesId)) {
        loadReport();
      }
    }, 5 * 60 * 1000);

    return () => {
      clearInterval(interval);
    };
  }, [seriesId, loadReport, needsRefresh]);

  const handleRefresh = () => {
    loadReport();
  };

  const handleUpdateFilters = useCallback(() => {
    updateFilters({
      severity: selectedSeverities,
      searchTerm
    });
  }, [selectedSeverities, searchTerm, updateFilters]);

  const clearFilters = () => {
    setSelectedSeverities([]);
    setSearchTerm('');
    resetFilters();
  };

  const getSeverityColor = (severity: ConflictSeverity): string => {
    const option = severityOptions.find(opt => opt.value === severity);
    return option?.color || 'gray';
  };

  const getScoreColor = (score: number): string => {
    if (score >= 0.9) return 'text-green-600';
    if (score >= 0.7) return 'text-yellow-600';
    if (score >= 0.5) return 'text-orange-600';
    return 'text-red-600';
  };

  const formatScore = (score: number): string => {
    return `${Math.round(score * 100)}%`;
  };

  // Update filters when local state changes
  useEffect(() => {
    handleUpdateFilters();
  }, [selectedSeverities, searchTerm, handleUpdateFilters]);

  const handleSeverityChange = (severity: ConflictSeverity, checked: boolean) => {
    if (checked) {
      setSelectedSeverities(prev => [...prev, severity]);
    } else {
      setSelectedSeverities(prev => prev.filter(s => s !== severity));
    }
  };

  return (
    <div className="series-consistency-report space-y-6">
      <div className="header flex justify-between items-center">
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white">
          Series Consistency Report
          {seriesName && (
            <span className="text-lg font-normal text-gray-600 dark:text-gray-400 ml-2">
              - {seriesName}
            </span>
          )}
        </h2>
        
        <div className="actions flex gap-2">
          <Button 
            variant="outline" 
            size="sm" 
            onClick={handleRefresh}
            disabled={isLoading}
            className="flex items-center gap-2"
          >
            {isLoading ? (
              <LoadingSpinner size="small" />
            ) : (
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            )}
            Refresh
          </Button>
        </div>
      </div>
      
      {error ? (
        <ErrorMessage message={error} />
      ) : loading && !report ? (
        <div className="loading-container flex flex-col items-center justify-center py-12">
          <LoadingSpinner size="large" />
          <p className="text-gray-600 dark:text-gray-400 mt-4">Analyzing series consistency...</p>
        </div>
      ) : report ? (
        <>
          {/* Consistency Score Overview */}
          <Card className="mb-6">
            <div className="score-overview flex items-center justify-between p-6">
              <div className="score-main flex items-center gap-4">
                <div className={`score-circle w-20 h-20 rounded-full border-4 flex items-center justify-center font-bold text-lg ${getScoreColor(report.consistency_score)}`}>
                  <span className="score-text text-2xl font-bold">{formatScore(report.consistency_score)}</span>
                </div>
                <div className="score-details">
                  <h3 className="text-lg font-semibold text-gray-900 dark:text-white">Overall Consistency</h3>
                  <p className="text-sm text-gray-600 dark:text-gray-400">
                    {report.total_conflicts} conflicts found
                  </p>
                </div>
              </div>
              
              <div className="conflict-breakdown space-y-2">
                <div className="breakdown-item flex justify-between">
                  <span className="breakdown-label text-sm text-gray-600 dark:text-gray-400">Characters:</span>
                  <span className="breakdown-value text-sm font-semibold text-gray-900 dark:text-white">{report.character_conflicts}</span>
                </div>
                <div className="breakdown-item flex justify-between">
                  <span className="breakdown-label text-sm text-gray-600 dark:text-gray-400">World Elements:</span>
                  <span className="breakdown-value text-sm font-semibold text-gray-900 dark:text-white">{report.world_element_conflicts}</span>
                </div>
                <div className="breakdown-item flex justify-between">
                  <span className="breakdown-label text-sm text-gray-600 dark:text-gray-400">Story Bible:</span>
                  <span className="breakdown-value text-sm font-semibold text-gray-900 dark:text-white">{report.story_bible_conflicts}</span>
                </div>
              </div>
            </div>
          </Card>
          
          {/* Filters */}
          <Card className="mb-6">
            <div className="filters space-y-4 p-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">Filter Conflicts</h3>
              
              <div className="filter-row grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                <div className="filter-group space-y-2">
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Severity
                  </label>
                  <div className="severity-filters flex flex-wrap gap-2">
                    {severityOptions.map(option => (
                      <label key={option.value} className="severity-filter flex items-center">
                        <input 
                          type="checkbox" 
                          checked={selectedSeverities.includes(option.value)}
                          onChange={(e) => handleSeverityChange(option.value, e.target.checked)}
                          className="mr-2"
                        />
                        <Badge color={option.color} size="sm">{option.label}</Badge>
                      </label>
                    ))}
                  </div>
                </div>
                
                <div className="filter-group space-y-2">
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    Search
                  </label>
                  <input 
                    type="text"
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    placeholder="Search conflicts..."
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  />
                </div>
                
                <div className="filter-actions">
                  <Button variant="outline" size="sm" onClick={clearFilters}>
                    Clear Filters
                  </Button>
                </div>
              </div>
            </div>
          </Card>
          
          {/* Conflicts List */}
          <Card>
            <div className="conflicts-header border-b border-gray-200 dark:border-gray-700 pb-4 mb-4 p-6">
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                Conflicts ({conflicts.length})
              </h3>
            </div>
            
            {conflicts.length === 0 ? (
              <div className="no-conflicts py-12 text-center">
                <svg className="w-12 h-12 text-green-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <p className="text-gray-600 dark:text-gray-400 text-center">
                  {selectedSeverities.length > 0 || searchTerm ? 'No conflicts match your filters' : 'No consistency conflicts found!'}
                </p>
              </div>
            ) : (
              <div className="conflicts-list space-y-4 p-6">
                {conflicts.map((conflict, index) => (
                  <div key={index} className="conflict-item border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                    <div className="conflict-header mb-3">
                      <div className="conflict-meta flex items-center gap-3">
                        <Badge color={getSeverityColor(conflict.severity)} size="sm">
                          {conflict.severity}
                        </Badge>
                        <span className="conflict-type text-sm font-medium text-gray-700 dark:text-gray-300">
                          {conflictTypeLabels[conflict.conflict_type]}
                        </span>
                      </div>
                    </div>
                    
                    <div className="conflict-content space-y-3">
                      <p className="conflict-description text-gray-900 dark:text-white">{conflict.description}</p>
                      
                      {conflict.affected_projects.length > 0 && (
                        <div className="affected-projects space-y-2">
                          <span className="projects-label text-sm font-medium text-gray-700 dark:text-gray-300">Affected Projects:</span>
                          <div className="projects-list flex flex-wrap gap-1">
                            {conflict.affected_projects.map((projectId, idx) => (
                              <Badge key={idx} color="gray" size="xs">{projectId}</Badge>
                            ))}
                          </div>
                        </div>
                      )}
                      
                      {Object.keys(conflict.details).length > 0 && (
                        <details className="conflict-details mt-3">
                          <summary className="details-toggle text-sm text-blue-600 dark:text-blue-400 cursor-pointer">View Details</summary>
                          <pre className="details-content mt-2 p-3 bg-gray-50 dark:bg-gray-800 rounded text-xs overflow-x-auto">
                            {JSON.stringify(conflict.details, null, 2)}
                          </pre>
                        </details>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            )}
          </Card>
          
          <div className="report-footer text-center pt-4">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              Report generated: {new Date(report.generated_at).toLocaleString()}
            </p>
          </div>
        </>
      ) : (
        <Card>
          <div className="empty-state py-12 text-center">
            <svg className="w-12 h-12 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
            </svg>
            <p className="text-gray-600 dark:text-gray-400 text-center mb-4">
              No consistency report available for this series.
            </p>
            <Button onClick={loadReport}>Generate Report</Button>
          </div>
        </Card>
      )}
    </div>
  );
};

export default SeriesConsistencyReportComponent;
