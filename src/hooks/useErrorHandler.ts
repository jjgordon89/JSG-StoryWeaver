import { useState, useCallback } from 'react';
import { toast } from 'react-hot-toast';

export interface AppError {
  id: string;
  message: string;
  severity: 'error' | 'warning' | 'info';
  timestamp: Date;
  context?: Record<string, any>;
  recoverable?: boolean;
  retryAction?: () => Promise<void>;
}

export const useErrorHandler = () => {
  const [errors, setErrors] = useState<AppError[]>([]);

  const handleError = useCallback((error: unknown, context?: Record<string, any>) => {
    const appError: AppError = {
      id: crypto.randomUUID(),
      message: error instanceof Error ? error.message : 'An unexpected error occurred',
      severity: 'error',
      timestamp: new Date(),
      context,
      recoverable: true,
    };

    setErrors(prev => [...prev, appError]);
    
    // Show user-friendly notification
    toast.error(appError.message, {
      duration: 5000,
      position: 'top-right',
    });

    // Log for debugging
    console.error('Application Error:', {
      error: appError,
      originalError: error,
      stack: error instanceof Error ? error.stack : undefined,
    });

    return appError;
  }, []);

  const handleWarning = useCallback((message: string, context?: Record<string, any>) => {
    const warning: AppError = {
      id: crypto.randomUUID(),
      message,
      severity: 'warning',
      timestamp: new Date(),
      context,
    };

    setErrors(prev => [...prev, warning]);
    toast(message, {
      duration: 3000,
      icon: '⚠️',
    });
  }, []);

  const clearError = useCallback((id: string) => {
    setErrors(prev => prev.filter(error => error.id !== id));
  }, []);

  const handleInfo = useCallback((message: string, context?: Record<string, any>) => {
    const info: AppError = {
      id: crypto.randomUUID(),
      message,
      severity: 'info',
      timestamp: new Date(),
      context,
    };

    setErrors(prev => [...prev, info]);
    toast.success(message, {
      duration: 3000,
    });
  }, []);

  const clearAllErrors = useCallback(() => {
    setErrors([]);
  }, []);

  const retryAction = useCallback(async (errorId: string) => {
    const error = errors.find(e => e.id === errorId);
    if (error?.retryAction) {
      try {
        await error.retryAction();
        clearError(errorId);
        toast.success('Action completed successfully');
      } catch (retryError) {
        handleError(retryError, { ...error.context, isRetry: true });
      }
    }
  }, [errors, clearError, handleError]);

  return {
    errors,
    handleError,
    handleWarning,
    handleInfo,
    clearError,
    clearAllErrors,
    retryAction,
  };
};