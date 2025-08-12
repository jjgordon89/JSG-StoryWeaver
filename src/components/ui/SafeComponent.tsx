import React, { Component, ErrorInfo, ReactNode } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';
import { cn } from '../../utils/cn';

interface SafeComponentProps {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
  className?: string;
  errorClassName?: string;
  showRetry?: boolean;
  retryText?: string;
  errorTitle?: string;
  errorMessage?: string;
}

interface SafeComponentState {
  hasError: boolean;
  error?: Error;
  errorInfo?: ErrorInfo;
  retryCount: number;
}

export class SafeComponent extends Component<SafeComponentProps, SafeComponentState> {
  private maxRetries = 3;

  constructor(props: SafeComponentProps) {
    super(props);
    this.state = {
      hasError: false,
      retryCount: 0,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<SafeComponentState> {
    return {
      hasError: true,
      error,
    };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    this.setState({ errorInfo });
    
    // Log error for debugging
    console.error('SafeComponent caught an error:', {
      error,
      errorInfo,
      retryCount: this.state.retryCount,
    });
    
    // Call custom error handler if provided
    this.props.onError?.(error, errorInfo);
  }

  handleRetry = () => {
    const { retryCount } = this.state;
    
    if (retryCount < this.maxRetries) {
      this.setState({
        hasError: false,
        error: undefined,
        errorInfo: undefined,
        retryCount: retryCount + 1,
      });
    }
  };

  render() {
    const {
      children,
      fallback,
      className,
      errorClassName,
      showRetry = true,
      retryText = 'Try Again',
      errorTitle = 'Component Error',
      errorMessage,
    } = this.props;

    const { hasError, error, errorInfo, retryCount } = this.state;

    if (hasError) {
      // Use custom fallback if provided
      if (fallback) {
        return <div className={className}>{fallback}</div>;
      }

      // Default error UI
      const canRetry = showRetry && retryCount < this.maxRetries;
      const displayMessage = errorMessage || error?.message || 'An unexpected error occurred in this component.';

      return (
        <div className={cn('p-4 border border-red-200 bg-red-50 rounded-lg', className, errorClassName)}>
          <div className="flex items-start gap-3">
            <AlertTriangle className="w-5 h-5 text-red-500 mt-0.5 flex-shrink-0" />
            <div className="flex-1 min-w-0">
              <h3 className="text-sm font-medium text-red-800 mb-1">
                {errorTitle}
              </h3>
              <p className="text-sm text-red-700 mb-3">
                {displayMessage}
              </p>
              
              {retryCount > 0 && (
                <p className="text-xs text-red-600 mb-3">
                  Retry attempt: {retryCount}/{this.maxRetries}
                </p>
              )}
              
              <div className="flex items-center gap-2">
                {canRetry && (
                  <button
                    onClick={this.handleRetry}
                    className="inline-flex items-center gap-1 px-3 py-1 text-xs font-medium text-red-700 bg-red-100 border border-red-300 rounded hover:bg-red-200 transition-colors"
                  >
                    <RefreshCw className="w-3 h-3" />
                    {retryText}
                  </button>
                )}
                
                {retryCount >= this.maxRetries && (
                  <span className="text-xs text-red-600 font-medium">
                    Maximum retries exceeded
                  </span>
                )}
              </div>
              
              {process.env.NODE_ENV === 'development' && error && (
                <details className="mt-3">
                  <summary className="cursor-pointer text-xs text-red-700 font-medium hover:text-red-800">
                    Error Details (Development)
                  </summary>
                  <pre className="mt-2 p-2 bg-red-100 border border-red-300 rounded text-xs overflow-auto max-h-32 text-red-800">
                    {error.stack}
                    {errorInfo?.componentStack && `\n\nComponent Stack:${errorInfo.componentStack}`}
                  </pre>
                </details>
              )}
            </div>
          </div>
        </div>
      );
    }

    return <div className={className}>{children}</div>;
  }
}

// Higher-order component for easier usage
export const withSafeComponent = <P extends object>(
  Component: React.ComponentType<P>,
  options?: Omit<SafeComponentProps, 'children'>
) => {
  const WrappedComponent = (props: P) => (
    <SafeComponent {...options}>
      <Component {...props} />
    </SafeComponent>
  );
  
  WrappedComponent.displayName = `withSafeComponent(${Component.displayName || Component.name})`;
  
  return WrappedComponent;
};

export default SafeComponent;