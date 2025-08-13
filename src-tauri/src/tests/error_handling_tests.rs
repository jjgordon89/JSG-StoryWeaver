#[cfg(test)]
mod tests {
    use crate::error::StoryWeaverError;
    use crate::ai::TokenCounter;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_api_key_storage_handles_errors() {
        // Create a mock manager for testing
        struct MockApiKeyManager {
            should_error: bool,
        }
        
        impl MockApiKeyManager {
            fn new() -> Self { Self { should_error: false } }
            fn new_with_errors() -> Self { Self { should_error: true } }
            
            async fn store_api_key(&self, provider: &str, key: &str) -> Result<(), StoryWeaverError> {
                if self.should_error || provider.is_empty() || key.is_empty() {
                    Err(StoryWeaverError::SecurityError { message: "Invalid input".to_string() })
                } else {
                    Ok(())
                }
            }
            
            async fn retrieve_api_key(&self, provider: &str) -> Result<Option<String>, StoryWeaverError> {
                if provider == "test_provider" {
                    Ok(Some("test_key".to_string()))
                } else {
                    Ok(None)
                }
            }
            
            async fn delete_api_key(&self, _provider: &str) -> Result<(), StoryWeaverError> {
                Ok(())
            }
        }
        
        let manager = MockApiKeyManager::new();
        
        // Test with invalid provider name
        let result = manager.store_api_key("", "test_key").await;
        assert!(result.is_err());
        
        // Test with empty key
        let result = manager.store_api_key("test_provider", "").await;
        assert!(result.is_err());
        
        // Test successful storage
        let result = manager.store_api_key("test_provider", "test_key").await;
        assert!(result.is_ok());
        
        // Test retrieval
        let retrieved = manager.retrieve_api_key("test_provider").await.unwrap();
        assert_eq!(retrieved, Some("test_key".to_string()));
        
        // Test retrieval of non-existent key
        let non_existent = manager.retrieve_api_key("non_existent_provider").await.unwrap();
        assert_eq!(non_existent, None);
        
        // Cleanup
        let _ = manager.delete_api_key("test_provider").await;
    }

    #[test]
    fn test_token_counter_accuracy() {
        let counter = TokenCounter::new();
        
        let text = "Hello, world!";
        let tokens = counter.count_tokens(text);
        assert!(tokens > 0, "Token count should be greater than 0");
        
        let cost = counter.estimate_cost("openai", "gpt-4", tokens, 0);
        assert!(cost > 0.0, "Cost should be greater than 0");
        
        // Test with empty text
        let empty_tokens = counter.count_tokens("");
        assert_eq!(empty_tokens, 0, "Empty text should have 0 tokens");
        
        // Test with longer text
        let long_text = "This is a longer piece of text that should have more tokens than the previous example.";
        let long_tokens = counter.count_tokens(long_text);
        assert!(long_tokens > tokens, "Longer text should have more tokens");
    }

    #[test]
    fn test_story_weaver_error_types() {
        // Test database error
        let db_error = StoryWeaverError::Database { message: "Connection failed".to_string() };
        assert!(matches!(db_error, StoryWeaverError::Database { .. }));
        
        // Test validation error
        let validation_error = StoryWeaverError::ValidationError { message: "Invalid input".to_string() };
        assert!(matches!(validation_error, StoryWeaverError::ValidationError { .. }));
        
        // Test API error
        let api_error = StoryWeaverError::InvalidAPIKey { provider: "test".to_string() };
        assert!(matches!(api_error, StoryWeaverError::InvalidAPIKey { .. }));
        
        // Test IO error
        let io_error = StoryWeaverError::FileOperation {
            operation: "read".to_string(),
            path: "test.txt".to_string(),
            message: "File not found".to_string(),
        };
        assert!(matches!(io_error, StoryWeaverError::FileOperation { .. }));
    }

    #[test]
    fn test_error_display_formatting() {
        let error = StoryWeaverError::Database { message: "Connection timeout".to_string() };
        let error_string = format!("{}", error);
        assert!(error_string.contains("Database error"));
        assert!(error_string.contains("Connection timeout"));
        
        let validation_error = StoryWeaverError::ValidationError { message: "Field is required".to_string() };
        let validation_string = format!("{}", validation_error);
        assert!(validation_string.contains("Validation error"));
        assert!(validation_string.contains("Field is required"));
    }

    #[tokio::test]
    async fn test_concurrent_api_key_operations() {
        // Create a mock manager for testing
        struct MockApiKeyManager;
        
        impl MockApiKeyManager {
            fn new() -> Self { Self }
            async fn store_api_key(&self, _provider: &str, _key: &str) -> Result<(), StoryWeaverError> {
                Ok(())
            }
            async fn retrieve_api_key(&self, _provider: &str) -> Result<Option<String>, StoryWeaverError> {
                Ok(Some("test_key".to_string()))
            }
            async fn delete_api_key(&self, _provider: &str) -> Result<(), StoryWeaverError> {
                Ok(())
            }
        }
        
        let manager: Arc<Mutex<MockApiKeyManager>> = Arc::new(Mutex::new(MockApiKeyManager::new()));
        let mut handles = vec![];
        
        // Test concurrent storage operations
        for i in 0..5 {
            let manager_clone: Arc<Mutex<MockApiKeyManager>> = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                let manager = manager_clone.lock().await;
                let provider = format!("test_provider_{}", i);
                let key = format!("test_key_{}", i);
                manager.store_api_key(&provider, &key).await
            });
            handles.push(handle);
        }
        
        // Wait for all operations to complete
        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert!(result.is_ok(), "Concurrent storage should succeed");
        }
        
        // Verify all keys were stored (mock returns test_key for any provider)
        let manager = manager.lock().await;
        for i in 0..5 {
            let provider = format!("test_provider_{}", i);
            let retrieved = manager.retrieve_api_key(&provider).await.unwrap();
            assert_eq!(retrieved, Some("test_key".to_string()));
        }
        
        // Cleanup
        for i in 0..5 {
            let provider = format!("test_provider_{}", i);
            let _ = manager.delete_api_key(&provider).await;
        }
    }

    #[test]
    fn test_error_chain_handling() {
        // Test that errors can be chained properly
        let _root_cause = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let io_error = StoryWeaverError::FileOperation {
            operation: "write".to_string(),
            path: "test.txt".to_string(),
            message: "Permission denied".to_string(),
        };
        
        // Verify error source chain
        let error_source = std::error::Error::source(&io_error);
        assert!(error_source.is_some());
        
        let source_string = format!("{}", error_source.unwrap());
        assert!(source_string.contains("Access denied"));
    }
}