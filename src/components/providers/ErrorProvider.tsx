import React, { createContext, useContext, useCallback, useState, ReactNode } from 'react';
import { AppError } from '../../hooks/useErrorHandler';
import { showErrorToast, showWarningToast, showInfoToast } from './ToastProvider';

interface ErrorContextType {
  errors: AppError[];
  addError: (error: unknown, context?: Record<string, any>) => AppError;
  addWarning: (message: string, context?: Record<string, any>) => AppError;
  addInfo: (message: string, context?: Record<string, any>) => AppError;
  removeError: (id: string) => void;
  clearAllErrors: () => void;
  hasErrors: boolean;
  getErrorsByContext: (contextKey: string) => AppError[];
}

const ErrorContext = createContext<ErrorContextType | undefined>(undefined);

interface ErrorProviderProps {
  children: ReactNode;
  maxErrors?: number;
  autoRemoveAfter?: number; // milliseconds
}

export const ErrorProvider: React.FC<ErrorProviderProps> = ({
  children,
  maxErrors = 50,
  autoRemoveAfter = 30000, // 30 seconds
}) => {
  const [errors, setErrors] = useState<AppError[]>([]);

  const addError = useCallback((error: unknown, context?: Record<string, any>): AppError => {
    const appError: AppError = {
      id: crypto.randomUUID(),
      message: error instanceof Error ? error.message : String(error),
      severity: 'error',
      timestamp: new Date(),
      context,
      recoverable: true,
    };

    setErrors(prev => {
      const newErrors = [appError, ...prev].slice(0, maxErrors);
      return newErrors;
    });

    // Show toast notification
    showErrorToast(appError.message);

    // Log for debugging
    console.error('Global Error:', {
      error: appError,
      originalError: error,
      stack: error instanceof Error ? error.stack : undefined,
    });

    // Auto-remove after specified time
    if (autoRemoveAfter > 0) {
      setTimeout(() => {
        setErrors(prev => prev.filter(e => e.id !== appError.id));
      }, autoRemoveAfter);
    }

    return appError;
  }, [maxErrors, autoRemoveAfter]);

  const addWarning = useCallback((message: string, context?: Record<string, any>): AppError => {
    const warning: AppError = {
      id: crypto.randomUUID(),
      message,
      severity: 'warning',
      timestamp: new Date(),
      context,
    };

    setErrors(prev => [warning, ...prev].slice(0, maxErrors));
    showWarningToast(message);

    // Auto-remove warnings faster
    if (autoRemoveAfter > 0) {
      setTimeout(() => {
        setErrors(prev => prev.filter(e => e.id !== warning.id));
      }, autoRemoveAfter / 2);
    }

    return warning;
  }, [maxErrors, autoRemoveAfter]);

  const addInfo = useCallback((message: string, context?: Record<string, any>): AppError => {
    const info: AppError = {
      id: crypto.randomUUID(),
      message,
      severity: 'info',
      timestamp: new Date(),
      context,
    };

    setErrors(prev => [info, ...prev].slice(0, maxErrors));
    showInfoToast(message);

    // Auto-remove info messages even faster
    if (autoRemoveAfter > 0) {
      setTimeout(() => {
        setErrors(prev => prev.filter(e => e.id !== info.id));
      }, autoRemoveAfter / 3);
    }

    return info;
  }, [maxErrors, autoRemoveAfter]);

  const removeError = useCallback((id: string) => {
    setErrors(prev => prev.filter(error => error.id !== id));
  }, []);

  const clearAllErrors = useCallback(() => {
    setErrors([]);
  }, []);

  const getErrorsByContext = useCallback((contextKey: string) => {
    return errors.filter(error => 
      error.context && Object.keys(error.context).includes(contextKey)
    );
  }, [errors]);

  const hasErrors = errors.some(error => error.severity === 'error');

  const value: ErrorContextType = {
    errors,
    addError,
    addWarning,
    addInfo,
    removeError,
    clearAllErrors,
    hasErrors,
    getErrorsByContext,
  };

  return (
    <ErrorContext.Provider value={value}>
      {children}
    </ErrorContext.Provider>
  );
};

export const useGlobalError = (): ErrorContextType => {
  const context = useContext(ErrorContext);
  if (context === undefined) {
    throw new Error('useGlobalError must be used within an ErrorProvider');
  }
  return context;
};

// Convenience hooks for specific error types
export const useErrorReporting = () => {
  const { addError, addWarning, addInfo } = useGlobalError();
  
  return {
    reportError: addError,
    reportWarning: addWarning,
    reportInfo: addInfo,
  };
};

export default ErrorProvider;