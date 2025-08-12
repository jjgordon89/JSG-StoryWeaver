import { renderHook, act } from '@testing-library/react';
import { vi, describe, it, expect, beforeEach } from 'vitest';
import { useErrorHandler } from '../useErrorHandler';
import { toast } from 'react-hot-toast';

// Mock react-hot-toast
vi.mock('react-hot-toast', () => ({
  error: vi.fn(),
  success: vi.fn(),
  loading: vi.fn(),
  dismiss: vi.fn(),
}));

const mockToast = toast as any;

describe('useErrorHandler', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should handle errors correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleError(new Error('Test error'));
    });
    
    expect(result.current.errors).toHaveLength(1);
    expect(result.current.errors[0].message).toBe('Test error');
    expect(mockToast.error).toHaveBeenCalledWith('Test error', expect.any(Object));
  });

  it('should handle warnings correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleWarning('Test warning');
    });
    
    expect(result.current.errors).toHaveLength(1);
    expect(result.current.errors[0].message).toBe('Test warning');
    expect(result.current.errors[0].type).toBe('warning');
    expect(mockToast.error).toHaveBeenCalledWith('Test warning', expect.any(Object));
  });

  it('should handle info messages correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleInfo('Test info');
    });
    
    expect(mockToast.success).toHaveBeenCalledWith('Test info');
  });

  it('should clear specific errors correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleError(new Error('Test error 1'));
      result.current.handleError(new Error('Test error 2'));
    });
    
    expect(result.current.errors).toHaveLength(2);
    
    const errorId = result.current.errors[0].id;
    
    act(() => {
      result.current.clearError(errorId);
    });
    
    expect(result.current.errors).toHaveLength(1);
    expect(result.current.errors[0].message).toBe('Test error 2');
  });

  it('should clear all errors correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleError(new Error('Test error 1'));
      result.current.handleError(new Error('Test error 2'));
    });
    
    expect(result.current.errors).toHaveLength(2);
    
    act(() => {
      result.current.clearAllErrors();
    });
    
    expect(result.current.errors).toHaveLength(0);
  });

  it('should handle retry actions correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    const mockRetryFn = jest.fn();
    
    act(() => {
      result.current.handleError(new Error('Test error'), {
        action: 'test_action',
        retryAction: mockRetryFn
      });
    });
    
    const errorId = result.current.errors[0].id;
    
    act(() => {
      result.current.retryAction(errorId);
    });
    
    expect(mockRetryFn).toHaveBeenCalled();
  });

  it('should handle errors with context correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    const context = {
      action: 'save_story',
      projectId: '123',
      userId: 'user456'
    };
    
    act(() => {
      result.current.handleError(new Error('Save failed'), context);
    });
    
    expect(result.current.errors).toHaveLength(1);
    expect(result.current.errors[0].context).toEqual(context);
  });
});