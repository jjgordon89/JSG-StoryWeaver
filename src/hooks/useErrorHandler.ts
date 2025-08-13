import { useState, useCallback } from 'react';
import { toast } from 'react-hot-toast';

/**
 * Represents an application error with metadata for handling and display.
 */
export interface AppError {
  /** Unique identifier for the error */
  id: string;
  /** Human-readable error message */
  message: string;
  /** Severity level of the error */
  severity: 'error' | 'warning' | 'info';
  /** When the error occurred */
  timestamp: Date;
  /** Additional context data for debugging */
  context?: Record<string, any>;
  /** Whether the error can be recovered from */
  recoverable?: boolean;
  /** Optional function to retry the failed operation */
  retryAction?: () => Promise<void>;
}

/**
 * Hook for centralized error handling and user notification management.
 * 
 * Provides a unified interface for handling errors, warnings, and info messages
 * throughout the application. Automatically displays toast notifications and
 * maintains error history for debugging and user feedback.
 * 
 * @returns {Object} Error handling interface
 * @returns {AppError[]} errors - Array of all errors that have occurred
 * @returns {Function} handleError - Handle and display an error
 * @returns {Function} handleWarning - Handle and display a warning
 * @returns {Function} handleInfo - Handle and display an info message
 * @returns {Function} clearError - Clear a specific error by ID
 * @returns {Function} clearAllErrors - Clear all errors
 * @returns {Function} retryAction - Retry a failed action if available
 * 
 * @example
 * ```tsx
 * const { handleError, handleWarning, handleInfo, retryAction } = useErrorHandler();
 * 
 * // Handle an error with context
 * try {
 *   await saveDocument();
 * } catch (error) {
 *   const appError = handleError(error, { documentId: 123, operation: 'save' });
 *   // Error is automatically displayed as toast and logged
 * }
 * 
 * // Handle warning
 * handleWarning("Document has unsaved changes");
 * 
 * // Handle info message
 * handleInfo("Document saved successfully");
 * 
 * // Retry a failed action
 * await retryAction(errorId);
 * ```
 */
export const useErrorHandler = () => {
  const [errors, setErrors] = useState<AppError[]>([]);

  /**
   * Handle an error and display appropriate user feedback.
   * 
   * @param {unknown} error - The error to handle (Error object or any value)
   * @param {Record<string, any>} [context] - Additional context for debugging
   * @returns {AppError} The created AppError object
   */
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

  /**
   * Handle a warning message and display it to the user.
   * 
   * @param {string} message - The warning message to display
   * @param {Record<string, any>} [context] - Additional context for debugging
   * @returns {AppError} The created warning object
   */
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

  /**
   * Clear a specific error from the error list.
   * 
   * @param {string} id - The ID of the error to clear
   */
  const clearError = useCallback((id: string) => {
    setErrors(prev => prev.filter(error => error.id !== id));
  }, []);

  /**
   * Handle an info message and display it to the user.
   * 
   * @param {string} message - The info message to display
   * @param {Record<string, any>} [context] - Additional context for debugging
   * @returns {AppError} The created info object
   */
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

  /**
   * Clear all errors from the error list.
   */
  const clearAllErrors = useCallback(() => {
    setErrors([]);
  }, []);

  /**
   * Retry a failed action if a retry function is available.
   * 
   * @param {string} errorId - The ID of the error to retry
   * @throws {Error} When retry action fails
   */
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
