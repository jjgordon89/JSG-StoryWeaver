# StoryWeaver Implementation Guide

## Critical Issue Resolution

This guide provides specific implementation steps to resolve the most critical issues identified in the StoryWeaver application.

## Phase 1: Backend Safety Fixes

### 1.1 Replace Unsafe Error Handling

#### Current Problematic Pattern:
```rust
// DANGEROUS - Can cause application crash
let result = some_operation().unwrap();
let value = option.expect("Must have value");
```

#### Recommended Safe Pattern:
```rust
// SAFE - Proper error handling
match some_operation() {
    Ok(result) => {
        // Handle success
    },
    Err(e) => {
        log::error!("Operation failed: {}", e);
        return Err(StoryWeaverError::from(e));
    }
}

// For Options
let value = match option {
    Some(v) => v,
    None => {
        log::warn!("Expected value not found, using default");
        default_value
    }
};
```

### 1.2 Specific File Fixes

#### Fix: `src-tauri/src/database/ai_response_cache.rs`

**Current Issues (8 unwrap calls)**:
```rust
// Line examples that need fixing
let conn = self.pool.get().unwrap(); // DANGEROUS
let result = sqlx::query!("...").fetch_one(&mut conn).await.unwrap(); // DANGEROUS
```

**Recommended Fix**:
```rust
use crate::error::StoryWeaverError;

pub async fn get_cached_response(&self, key: &str) -> Result<Option<String>, StoryWeaverError> {
    let mut conn = self.pool.get()
        .map_err(|e| {
            log::error!("Failed to get database connection: {}", e);
            StoryWeaverError::Database(format!("Connection pool error: {}", e))
        })?
    
    let result = sqlx::query_as::<_, CachedResponse>(
        "SELECT response_data FROM ai_response_cache WHERE cache_key = ? AND expires_at > datetime('now')"
    )
    .bind(key)
    .fetch_optional(&mut *conn)
    .await
    .map_err(|e| {
        log::error!("Database query failed: {}", e);
        StoryWeaverError::Database(format!("Cache lookup failed: {}", e))
    })?
    
    Ok(result.map(|r| r.response_data))
}
```

#### Fix: `src-tauri/src/security/api_keys.rs`

**Current TODOs to Implement**:
```rust
// TODO: Implement proper keychain storage
// TODO: Implement proper keychain retrieval
// TODO: Implement proper keychain deletion
```

**Implementation**:
```rust
use keyring::Entry;
use crate::error::StoryWeaverError;

const SERVICE_NAME: &str = "StoryWeaver";

pub struct ApiKeyManager {
    service: String,
}

impl ApiKeyManager {
    pub fn new() -> Self {
        Self {
            service: SERVICE_NAME.to_string(),
        }
    }

    pub fn store_api_key(&self, provider: &str, api_key: &str) -> Result<(), StoryWeaverError> {
        let entry = Entry::new(&self.service, provider)
            .map_err(|e| StoryWeaverError::Security(format!("Keychain entry creation failed: {}", e)))?
        
        entry.set_password(api_key)
            .map_err(|e| StoryWeaverError::Security(format!("Failed to store API key: {}", e)))?
        
        log::info!("API key stored successfully for provider: {}", provider);
        Ok(())
    }

    pub fn retrieve_api_key(&self, provider: &str) -> Result<Option<String>, StoryWeaverError> {
        let entry = Entry::new(&self.service, provider)
            .map_err(|e| StoryWeaverError::Security(format!("Keychain entry creation failed: {}", e)))?
        
        match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => {
                log::debug!("No API key found for provider: {}", provider);
                Ok(None)
            },
            Err(e) => {
                log::error!("Failed to retrieve API key for {}: {}", provider, e);
                Err(StoryWeaverError::Security(format!("Keychain retrieval failed: {}", e)))
            }
        }
    }

    pub fn delete_api_key(&self, provider: &str) -> Result<(), StoryWeaverError> {
        let entry = Entry::new(&self.service, provider)
            .map_err(|e| StoryWeaverError::Security(format!("Keychain entry creation failed: {}", e)))?
        
        match entry.delete_password() {
            Ok(()) => {
                log::info!("API key deleted successfully for provider: {}", provider);
                Ok(())
            },
            Err(keyring::Error::NoEntry) => {
                log::debug!("No API key to delete for provider: {}", provider);
                Ok(()) // Not an error if key doesn't exist
            },
            Err(e) => {
                log::error!("Failed to delete API key for {}: {}", provider, e);
                Err(StoryWeaverError::Security(format!("Keychain deletion failed: {}", e)))
            }
        }
    }
}
```

**Add to Cargo.toml**:
```toml
[dependencies]
keyring = "2.0"
```

### 1.3 Token Counting Implementation

#### Fix: `src-tauri/src/commands/story_bible_ai.rs`

**Current Issue**:
```rust
tokens_used: 0, // TODO: Implement token counting
cost_estimate: 0.0, // TODO: Implement cost estimation
```

**Implementation**:
```rust
use tiktoken_rs::tiktoken;

#[derive(Debug, Clone)]
pub struct TokenCounter {
    encoder: tiktoken::CoreBPE,
}

impl TokenCounter {
    pub fn new() -> Result<Self, StoryWeaverError> {
        let encoder = tiktoken::get_bpe_from_model("gpt-4")
            .map_err(|e| StoryWeaverError::Internal(format!("Failed to initialize token counter: {}", e)))?
        
        Ok(Self { encoder })
    }

    pub fn count_tokens(&self, text: &str) -> usize {
        self.encoder.encode_with_special_tokens(text).len()
    }

    pub fn estimate_cost(&self, tokens: usize, model: &str) -> f64 {
        match model {
            "gpt-4" => tokens as f64 * 0.00003, // $0.03 per 1K tokens
            "gpt-3.5-turbo" => tokens as f64 * 0.000002, // $0.002 per 1K tokens
            "claude-3" => tokens as f64 * 0.000015, // $0.015 per 1K tokens
            _ => tokens as f64 * 0.00001, // Default estimate
        }
    }
}

// Update the AI response functions
pub async fn generate_character_analysis(
    app_handle: tauri::AppHandle,
    character_data: serde_json::Value,
) -> Result<AIResponse, StoryWeaverError> {
    let token_counter = TokenCounter::new()?
    let prompt = format!("Analyze this character: {}", character_data);
    let input_tokens = token_counter.count_tokens(&prompt);
    
    // ... existing AI call logic ...
    
    let output_tokens = token_counter.count_tokens(&response_text);
    let total_tokens = input_tokens + output_tokens;
    let cost = token_counter.estimate_cost(total_tokens, "gpt-4");
    
    Ok(AIResponse {
        content: response_text,
        tokens_used: total_tokens,
        cost_estimate: cost,
        model_used: "gpt-4".to_string(),
        timestamp: chrono::Utc::now(),
    })
}
```

**Add to Cargo.toml**:
```toml
[dependencies]
tiktoken-rs = "0.5"
```

## Phase 2: Frontend Error Handling

### 2.1 Error Notification System

#### Create: `src/hooks/useErrorHandler.ts`
```typescript
import { useState, useCallback } from 'react';
import { toast } from 'react-toastify';

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
      position: 'top-right',
      autoClose: 5000,
      hideProgressBar: false,
      closeOnClick: true,
      pauseOnHover: true,
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
    toast.warn(message, { autoClose: 3000 });
  }, []);

  const clearError = useCallback((id: string) => {
    setErrors(prev => prev.filter(error => error.id !== id));
  }, []);

  const clearAllErrors = useCallback(() => {
    setErrors([]);
  }, []);

  return {
    errors,
    handleError,
    handleWarning,
    clearError,
    clearAllErrors,
  };
};
```

### 2.2 Error Boundary Component

#### Create: `src/components/ErrorBoundary.tsx`
```typescript
import React, { Component, ErrorInfo, ReactNode } from 'react';
import { toast } from 'react-toastify';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
}

interface State {
  hasError: boolean;
  error?: Error;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Error Boundary caught an error:', error, errorInfo);
    
    // Call custom error handler if provided
    this.props.onError?.(error, errorInfo);
    
    // Show user notification
    toast.error('Something went wrong. Please try refreshing the page.', {
      position: 'top-center',
      autoClose: false,
      closeOnClick: false,
    });
  }

  render() {
    if (this.state.hasError) {
      return this.props.fallback || (
        <div className="error-boundary">
          <h2>Something went wrong</h2>
          <p>We're sorry, but something unexpected happened.</p>
          <button 
            onClick={() => window.location.reload()}
            className="btn btn-primary"
          >
            Refresh Page
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}
```

### 2.3 Fix BraindumpEditor Save Functionality

#### Update: `src/features/story-bible/components/react/BraindumpEditor.tsx`

**Current Issue**:
```typescript
// TODO: Implement save functionality
```

**Implementation**:
```typescript
import { useErrorHandler } from '../../../../hooks/useErrorHandler';
import { invoke } from '@tauri-apps/api/tauri';

const BraindumpEditor: React.FC<BraindumpEditorProps> = ({ storyBibleId, onSave }) => {
  const [content, setContent] = useState('');
  const [isSaving, setIsSaving] = useState(false);
  const [lastSaved, setLastSaved] = useState<Date | null>(null);
  const { handleError, handleWarning } = useErrorHandler();

  // Auto-save functionality
  const saveContent = useCallback(async (contentToSave: string) => {
    if (!contentToSave.trim()) {
      handleWarning('Cannot save empty content');
      return;
    }

    setIsSaving(true);
    try {
      await invoke('save_braindump_content', {
        storyBibleId,
        content: contentToSave,
        timestamp: new Date().toISOString(),
      });
      
      setLastSaved(new Date());
      onSave?.(contentToSave);
      
      toast.success('Content saved successfully', { autoClose: 2000 });
    } catch (error) {
      handleError(error, { 
        action: 'save_braindump',
        storyBibleId,
        contentLength: contentToSave.length 
      });
    } finally {
      setIsSaving(false);
    }
  }, [storyBibleId, onSave, handleError, handleWarning]);

  // Debounced auto-save
  const debouncedSave = useMemo(
    () => debounce(saveContent, 2000),
    [saveContent]
  );

  useEffect(() => {
    if (content) {
      debouncedSave(content);
    }
  }, [content, debouncedSave]);

  // Manual save handler
  const handleManualSave = useCallback(() => {
    saveContent(content);
  }, [content, saveContent]);

  return (
    <div className="braindump-editor">
      <div className="editor-header">
        <div className="save-status">
          {isSaving && <span className="saving">Saving...</span>}
          {lastSaved && !isSaving && (
            <span className="saved">
              Last saved: {lastSaved.toLocaleTimeString()}
            </span>
          )}
        </div>
        <button 
          onClick={handleManualSave}
          disabled={isSaving}
          className="btn btn-primary"
        >
          {isSaving ? 'Saving...' : 'Save'}
        </button>
      </div>
      
      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder="Start brainstorming your ideas..."
        className="braindump-textarea"
      />
    </div>
  );
};
```

## Phase 3: Testing Strategy

### 3.1 Backend Error Handling Tests

#### Create: `src-tauri/src/tests/error_handling_tests.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::StoryWeaverError;

    #[tokio::test]
    async fn test_api_key_storage_handles_errors() {
        let manager = ApiKeyManager::new();
        
        // Test with invalid provider name
        let result = manager.store_api_key("", "test_key");
        assert!(result.is_err());
        
        // Test successful storage
        let result = manager.store_api_key("test_provider", "test_key");
        assert!(result.is_ok());
        
        // Test retrieval
        let retrieved = manager.retrieve_api_key("test_provider").unwrap();
        assert_eq!(retrieved, Some("test_key".to_string()));
        
        // Cleanup
        manager.delete_api_key("test_provider").unwrap();
    }

    #[test]
    fn test_token_counter_accuracy() {
        let counter = TokenCounter::new().unwrap();
        
        let text = "Hello, world!";
        let tokens = counter.count_tokens(text);
        assert!(tokens > 0);
        
        let cost = counter.estimate_cost(tokens, "gpt-4");
        assert!(cost > 0.0);
    }
}
```

### 3.2 Frontend Error Handling Tests

#### Create: `src/hooks/__tests__/useErrorHandler.test.ts`
```typescript
import { renderHook, act } from '@testing-library/react';
import { useErrorHandler } from '../useErrorHandler';
import { toast } from 'react-toastify';

jest.mock('react-toastify');

describe('useErrorHandler', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('should handle errors correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleError(new Error('Test error'));
    });
    
    expect(result.current.errors).toHaveLength(1);
    expect(result.current.errors[0].message).toBe('Test error');
    expect(toast.error).toHaveBeenCalledWith('Test error', expect.any(Object));
  });

  it('should clear errors correctly', () => {
    const { result } = renderHook(() => useErrorHandler());
    
    act(() => {
      result.current.handleError(new Error('Test error'));
    });
    
    const errorId = result.current.errors[0].id;
    
    act(() => {
      result.current.clearError(errorId);
    });
    
    expect(result.current.errors).toHaveLength(0);
  });
});
```

## Implementation Checklist

### Backend Safety (Week 1)
- [ ] Replace all `unwrap()` calls in `ai_response_cache.rs`
- [ ] Replace all `unwrap()` calls in `collaboration.rs`
- [ ] Replace all `unwrap()` calls in `memory_optimizer.rs`
- [ ] Replace all `unwrap()` calls in `privacy.rs`
- [ ] Implement proper keychain storage in `api_keys.rs`
- [ ] Add comprehensive error logging
- [ ] Test error scenarios

### Frontend Error Handling (Week 2)
- [ ] Create `useErrorHandler` hook
- [ ] Implement `ErrorBoundary` component
- [ ] Update all stores with proper error handling
- [ ] Add error notifications to UI
- [ ] Test error recovery scenarios

### Feature Completion (Week 3)
- [ ] Implement BraindumpEditor save functionality
- [ ] Add AI brainstorming features
- [ ] Complete token counting system
- [ ] Implement cost estimation
- [ ] Add filtering capabilities to AI card operations

### Testing and Validation (Week 4)
- [ ] Write comprehensive error handling tests
- [ ] Test user experience with error scenarios
- [ ] Performance impact assessment
- [ ] Security audit of new implementations
- [ ] Documentation updates

This implementation guide provides concrete steps to resolve the critical issues identified in the StoryWeaver application. Each fix includes proper error handling, user feedback, and testing strategies to ensure a robust and maintainable codebase.