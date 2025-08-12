#[cfg(test)]
mod tests {
    use crate::error::StoryWeaverError;
    use crate::services::api_key_manager::ApiKeyManager;
    use crate::services::token_counter::TokenCounter;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_api_key_storage_handles_errors() {
        let manager = ApiKeyManager::new();
        
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
        let counter = TokenCounter::new().expect("Failed to create token counter");
        
        let text = "Hello, world!";
        let tokens = counter.count_tokens(text);
        assert!(tokens > 0, "Token count should be greater than 0");
        
        let cost = counter.estimate_cost(tokens, "gpt-4");
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
        let db_error = StoryWeaverError::Database("Connection failed".to_string());
        assert!(matches!(db_error, StoryWeaverError::Database(_)));
        
        // Test validation error
        let validation_error = StoryWeaverError::Validation("Invalid input".to_string());
        assert!(matches!(validation_error, StoryWeaverError::Validation(_)));
        
        // Test API error
        let api_error = StoryWeaverError::Api("API call failed".to_string());
        assert!(matches!(api_error, StoryWeaverError::Api(_)));
        
        // Test IO error
        let io_error = StoryWeaverError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found"
        ));
        assert!(matches!(io_error, StoryWeaverError::Io(_)));
    }

    #[test]
    fn test_error_display_formatting() {
        let error = StoryWeaverError::Database("Connection timeout".to_string());
        let error_string = format!("{}", error);
        assert!(error_string.contains("Database error"));
        assert!(error_string.contains("Connection timeout"));
        
        let validation_error = StoryWeaverError::Validation("Field is required".to_string());
        let validation_string = format!("{}", validation_error);
        assert!(validation_string.contains("Validation error"));
        assert!(validation_string.contains("Field is required"));
    }

    #[tokio::test]
    async fn test_concurrent_api_key_operations() {
        let manager = Arc::new(Mutex::new(ApiKeyManager::new()));
        let mut handles = vec![];
        
        // Test concurrent storage operations
        for i in 0..5 {
            let manager_clone = Arc::clone(&manager);
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
        
        // Verify all keys were stored
        let manager = manager.lock().await;
        for i in 0..5 {
            let provider = format!("test_provider_{}", i);
            let expected_key = format!("test_key_{}", i);
            let retrieved = manager.retrieve_api_key(&provider).await.unwrap();
            assert_eq!(retrieved, Some(expected_key));
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
        let root_cause = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let io_error = StoryWeaverError::Io(root_cause);
        
        // Verify error source chain
        let error_source = std::error::Error::source(&io_error);
        assert!(error_source.is_some());
        
        let source_string = format!("{}", error_source.unwrap());
        assert!(source_string.contains("Access denied"));
    }
}