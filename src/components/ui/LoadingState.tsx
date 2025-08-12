import React from 'react';
import { Loader2, AlertCircle, RefreshCw } from 'lucide-react';
import { cn } from '../../utils/cn';

export interface LoadingStateProps {
  isLoading?: boolean;
  error?: string | Error | null;
  isEmpty?: boolean;
  loadingText?: string;
  emptyText?: string;
  emptyIcon?: React.ReactNode;
  retryAction?: () => void;
  retryText?: string;
  className?: string;
  size?: 'sm' | 'md' | 'lg';
  variant?: 'spinner' | 'skeleton' | 'pulse';
  children?: React.ReactNode;
}

const sizeClasses = {
  sm: 'h-4 w-4',
  md: 'h-6 w-6',
  lg: 'h-8 w-8',
};

const LoadingSpinner: React.FC<{ size: 'sm' | 'md' | 'lg'; text?: string }> = ({ size, text }) => (
  <div className="flex flex-col items-center justify-center p-6">
    <Loader2 className={cn('animate-spin text-blue-600', sizeClasses[size])} />
    {text && <p className="mt-2 text-sm text-gray-600">{text}</p>}
  </div>
);

const SkeletonLoader: React.FC<{ className?: string }> = ({ className }) => (
  <div className={cn('animate-pulse', className)}>
    <div className="space-y-3">
      <div className="h-4 bg-gray-200 rounded w-3/4"></div>
      <div className="h-4 bg-gray-200 rounded w-1/2"></div>
      <div className="h-4 bg-gray-200 rounded w-5/6"></div>
    </div>
  </div>
);

const PulseLoader: React.FC<{ size: 'sm' | 'md' | 'lg' }> = ({ size }) => (
  <div className="flex items-center justify-center p-6">
    <div className={cn('bg-blue-600 rounded-full animate-pulse', sizeClasses[size])}></div>
  </div>
);

const ErrorState: React.FC<{
  error: string | Error;
  retryAction?: () => void;
  retryText?: string;
}> = ({ error, retryAction, retryText = 'Try Again' }) => {
  const errorMessage = error instanceof Error ? error.message : error;
  
  return (
    <div className="flex flex-col items-center justify-center p-6 text-center">
      <AlertCircle className="w-12 h-12 text-red-500 mb-4" />
      <h3 className="text-lg font-semibold text-gray-900 mb-2">Something went wrong</h3>
      <p className="text-gray-600 mb-4 max-w-md">{errorMessage}</p>
      {retryAction && (
        <button
          onClick={retryAction}
          className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        >
          <RefreshCw className="w-4 h-4" />
          {retryText}
        </button>
      )}
    </div>
  );
};

const EmptyState: React.FC<{
  text: string;
  icon?: React.ReactNode;
}> = ({ text, icon }) => (
  <div className="flex flex-col items-center justify-center p-6 text-center">
    {icon && <div className="mb-4 text-gray-400">{icon}</div>}
    <p className="text-gray-600">{text}</p>
  </div>
);

export const LoadingState: React.FC<LoadingStateProps> = ({
  isLoading = false,
  error = null,
  isEmpty = false,
  loadingText = 'Loading...',
  emptyText = 'No data available',
  emptyIcon,
  retryAction,
  retryText,
  className,
  size = 'md',
  variant = 'spinner',
  children,
}) => {
  // Error state takes precedence
  if (error) {
    return (
      <div className={className}>
        <ErrorState error={error} retryAction={retryAction} retryText={retryText} />
      </div>
    );
  }

  // Loading state
  if (isLoading) {
    return (
      <div className={className}>
        {variant === 'spinner' && <LoadingSpinner size={size} text={loadingText} />}
        {variant === 'skeleton' && <SkeletonLoader className={className} />}
        {variant === 'pulse' && <PulseLoader size={size} />}
      </div>
    );
  }

  // Empty state
  if (isEmpty) {
    return (
      <div className={className}>
        <EmptyState text={emptyText} icon={emptyIcon} />
      </div>
    );
  }

  // Render children when not loading, no error, and not empty
  return <>{children}</>;
};

// Hook for managing loading states
export const useLoadingState = () => {
  const [isLoading, setIsLoading] = React.useState(false);
  const [error, setError] = React.useState<string | Error | null>(null);

  const startLoading = React.useCallback(() => {
    setIsLoading(true);
    setError(null);
  }, []);

  const stopLoading = React.useCallback(() => {
    setIsLoading(false);
  }, []);

  const setLoadingError = React.useCallback((error: string | Error) => {
    setIsLoading(false);
    setError(error);
  }, []);

  const reset = React.useCallback(() => {
    setIsLoading(false);
    setError(null);
  }, []);

  return {
    isLoading,
    error,
    startLoading,
    stopLoading,
    setError: setLoadingError,
    reset,
  };
};

export default LoadingState;