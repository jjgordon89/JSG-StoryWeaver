import React, { useEffect, useState } from 'react';
import { useSeriesConsistencyStore } from '../stores/seriesConsistencyStore';
import type { SeriesConsistencyStatus } from '../types/seriesConsistency';
import { LoadingSpinner } from './ui';

interface SeriesConsistencyWidgetProps {
  seriesId: string;
  size?: 'sm' | 'md' | 'lg';
  showDetails?: boolean;
  onViewReport?: (() => void) | null;
}

// Simple UI Components
const Button: React.FC<{
  variant?: 'outline' | 'solid';
  size?: 'xs' | 'sm' | 'md' | 'lg';
  onClick?: () => void;
  disabled?: boolean;
  className?: string;
  children: React.ReactNode;
}> = ({ variant = 'solid', size = 'md', onClick, disabled, className = '', children }) => {
  const baseClasses = 'inline-flex items-center justify-center rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2';
  const variantClasses = variant === 'outline' 
    ? 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'
    : 'bg-blue-600 text-white hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600';
  const sizeClasses = {
    xs: 'px-2 py-1 text-xs',
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  }[size];
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
    green: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200',
    gray: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
  }[color] || 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200';
  
  const sizeClasses = size === 'xs' ? 'px-2 py-0.5 text-xs' : size === 'md' ? 'px-3 py-1 text-base' : 'px-2.5 py-0.5 text-sm';
  
  return (
    <span className={`inline-flex items-center rounded-full font-medium ${colorClasses} ${sizeClasses}`}>
      {children}
    </span>
  );
};

export const SeriesConsistencyWidget: React.FC<SeriesConsistencyWidgetProps> = ({ 
  seriesId, 
  size = 'md',
  showDetails = true,
  onViewReport = null
}) => {
  const { getStatus } = useSeriesConsistencyStore();
  const [status, setStatus] = useState<SeriesConsistencyStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadStatus();
  }, [seriesId]);

  const loadStatus = async () => {
    if (!seriesId) return;
    
    setLoading(true);
    setError(null);
    
    try {
      const statusResult = await getStatus(seriesId);
      setStatus(statusResult);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to load consistency status';
      setError(errorMessage);
      console.error('Error loading consistency status:', err);
    } finally {
      setLoading(false);
    }
  };

  const getScoreColor = (score: number): string => {
    if (score >= 0.9) return 'green';
    if (score >= 0.7) return 'yellow';
    if (score >= 0.5) return 'orange';
    return 'red';
  };

  const getScoreLabel = (score: number): string => {
    if (score >= 0.9) return 'Excellent';
    if (score >= 0.7) return 'Good';
    if (score >= 0.5) return 'Fair';
    return 'Poor';
  };

  const formatScore = (score: number): string => {
    return `${Math.round(score * 100)}%`;
  };

  const getConflictSeverity = (conflictCount: number): { color: string; label: string } => {
    if (conflictCount === 0) return { color: 'green', label: 'No Issues' };
    if (conflictCount <= 3) return { color: 'yellow', label: 'Minor Issues' };
    if (conflictCount <= 10) return { color: 'orange', label: 'Some Issues' };
    return { color: 'red', label: 'Major Issues' };
  };

  // Size-specific classes
  const containerClasses = {
    sm: 'p-2',
    md: 'p-4', 
    lg: 'p-6'
  }[size];

  const contentSpacing = {
    sm: 'space-y-2',
    md: 'space-y-3',
    lg: 'space-y-4'
  }[size];

  const scoreCircleSize = {
    sm: 'w-8 h-8 text-xs',
    md: 'w-12 h-12 text-sm',
    lg: 'w-16 h-16 text-base'
  }[size];

  const scoreLabelSize = {
    sm: 'text-sm',
    md: 'text-base',
    lg: 'text-lg'
  }[size];

  const scoreSubtitleSize = {
    sm: 'text-xs',
    md: 'text-sm',
    lg: 'text-sm'
  }[size];

  const badgeSize = size === 'sm' ? 'xs' : 'sm';
  const buttonSize = size === 'lg' ? 'sm' : 'xs';

  return (
    <div className={`consistency-widget bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg ${containerClasses} ${loading ? 'opacity-75' : ''}`}>
      {loading ? (
        <div className="loading-state flex items-center justify-center gap-2">
          <LoadingSpinner size="small" />
          <span className="loading-text text-sm text-gray-600 dark:text-gray-400">Checking...</span>
        </div>
      ) : error ? (
        <div className="error-state flex items-center justify-center gap-2 text-red-600 dark:text-red-400">
          <svg className="error-icon w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span className="error-text text-sm">Error</span>
        </div>
      ) : status ? (
        <div className={`status-content ${contentSpacing}`}>
          {/* Score Display */}
          <div className={`score-section flex items-center ${size === 'sm' ? 'gap-2' : 'gap-3'}`}>
            <div className={`score-circle rounded-full border-2 flex items-center justify-center font-bold ${scoreCircleSize} ${getScoreColor(status.consistency_score) === 'green' ? 'border-green-500 text-green-600 bg-green-50 dark:bg-green-900/20' : 
              getScoreColor(status.consistency_score) === 'yellow' ? 'border-yellow-500 text-yellow-600 bg-yellow-50 dark:bg-yellow-900/20' :
              getScoreColor(status.consistency_score) === 'orange' ? 'border-orange-500 text-orange-600 bg-orange-50 dark:bg-orange-900/20' :
              'border-red-500 text-red-600 bg-red-50 dark:bg-red-900/20'}`}>
              <span className="score-value">{formatScore(status.consistency_score)}</span>
            </div>
            
            {showDetails && (
              <div className="score-details flex-1">
                <div className={`score-label font-semibold text-gray-900 dark:text-white ${scoreLabelSize}`}>
                  {getScoreLabel(status.consistency_score)}
                </div>
                <div className={`score-subtitle text-gray-600 dark:text-gray-400 ${scoreSubtitleSize}`}>
                  Consistency
                </div>
              </div>
            )}
          </div>
          
          {/* Conflicts Display */}
          {showDetails && (
            <div className="conflicts-section space-y-1">
              <Badge 
                color={getConflictSeverity(status.conflict_count).color} 
                size={badgeSize}
              >
                {status.conflict_count} {status.conflict_count === 1 ? 'Conflict' : 'Conflicts'}
              </Badge>
              
              {status.conflict_count > 0 && (
                <div className="conflict-label text-xs text-gray-600 dark:text-gray-400">
                  {getConflictSeverity(status.conflict_count).label}
                </div>
              )}
            </div>
          )}
          
          {/* Actions */}
          {onViewReport && size !== 'sm' && (
            <div className="actions-section flex justify-center">
              <Button 
                variant="outline" 
                size={buttonSize}
                onClick={onViewReport}
              >
                View Report
              </Button>
            </div>
          )}
        </div>
      ) : (
        <div className="empty-state flex items-center justify-center">
          <span className="empty-text text-sm text-gray-500 dark:text-gray-400">No data</span>
        </div>
      )}
    </div>
  );
};

export default SeriesConsistencyWidget;
