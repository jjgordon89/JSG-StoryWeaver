//! Security validation tests

use super::validation::*;
use crate::error::StoryWeaverError;

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        // Valid emails
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.email+tag@domain.co.uk").is_ok());
        assert!(validate_email("user123@test-domain.org").is_ok());
        
        // Invalid emails
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("").is_err());
        assert!(validate_email("user@domain").is_err());
    }

    #[test]
    fn test_validate_filename() {
        // Valid filenames
        assert!(validate_filename("document.txt").is_ok());
        assert!(validate_filename("my-file_123.pdf").is_ok());
        assert!(validate_filename("report.docx").is_ok());
        
        // Invalid filenames
        assert!(validate_filename("../../../etc/passwd").is_err());
        assert!(validate_filename("file<script>").is_err());
        assert!(validate_filename("con.txt").is_err()); // Windows reserved
        assert!(validate_filename("").is_err());
        assert!(validate_filename("file|pipe").is_err());
    }

    #[test]
    fn test_validate_path() {
        // Valid paths
        assert!(validate_path("/home/user/documents").is_ok());
        assert!(validate_path("./local/path").is_ok());
        assert!(validate_path("C:/Users/Documents").is_ok());
        
        // Invalid paths (path traversal attempts)
        assert!(validate_path("../../../etc/passwd").is_err());
        assert!(validate_path("/home/user/../../../etc").is_err());
        assert!(validate_path("../../windows/system32").is_err());
        assert!(validate_path("").is_err());
    }

    #[test]
    fn test_sanitize_sql_input() {
        assert_eq!(sanitize_sql_input("normal text"), "normal text");
        assert_eq!(sanitize_sql_input("text with 'quotes'"), "text with quotes");
        assert_eq!(sanitize_sql_input("text with \"double quotes\""), "text with \"double quotes\"");
        assert_eq!(sanitize_sql_input("text; DROP TABLE users;"), "text  TABLE users");
    }

    #[test]
    fn test_sanitize_html() {
        assert_eq!(sanitize_html("<script>alert('xss')</script>"), "alert(&#x27;xss&#x27;)");
        assert_eq!(sanitize_html("<b>bold</b> text"), "bold text");
        assert_eq!(sanitize_html("normal text"), "normal text");
        assert_eq!(sanitize_html("<img src=x onerror=alert(1)>"), "");
    }

    #[test]
    fn test_detect_xss_attempt() {
        // Should detect XSS attempts
        assert!(detect_xss_attempt("<script>alert('xss')</script>"));
        assert!(detect_xss_attempt("javascript:alert(1)"));
        assert!(detect_xss_attempt("<img src=x onerror=alert(1)>"));
        assert!(detect_xss_attempt("<iframe src=javascript:alert(1)></iframe>"));
        
        // Should not flag normal content
        assert!(!detect_xss_attempt("normal text content"));
        assert!(!detect_xss_attempt("This is a regular sentence."));
        assert!(!detect_xss_attempt("Email: user@example.com"));
    }

    #[test]
    fn test_validate_security_input() {
        // Valid inputs
        assert!(validate_security_input("normal text").is_ok());
        assert!(validate_security_input("user@example.com").is_ok());
        assert!(validate_security_input("Some content with numbers 123").is_ok());
        
        // Invalid inputs (SQL injection attempts)
        assert!(validate_security_input("'; DROP TABLE users; --").is_err());
        assert!(validate_security_input("1' OR '1'='1").is_err());
        assert!(validate_security_input("UNION SELECT * FROM passwords").is_err());
        
        // Invalid inputs (XSS attempts)
        assert!(validate_security_input("<script>alert('xss')</script>").is_err());
        assert!(validate_security_input("javascript:alert(1)").is_err());
        assert!(validate_security_input("<img src=x onerror=alert(1)>").is_err());
    }

    #[test]
    fn test_validate_api_key() {
        // Valid API keys
        assert!(validate_api_key("sk-1234567890abcdef1234567890abcdef12345678").is_ok());
        assert!(validate_api_key("ak_prod_1234567890abcdef1234567890abcdef").is_ok());
        
        // Invalid API keys
        assert!(validate_api_key("short").is_err());
        assert!(validate_api_key("test123").is_err());
        assert!(validate_api_key("demo1234").is_err());
        assert!(validate_api_key("123456").is_err());
        assert!(validate_api_key("").is_err());
    }

    #[test]
    fn test_validate_safe_names() {
        // Valid names using public functions
        assert!(validate_project_name("My Project").is_ok());
        assert!(validate_document_name("Document_123").is_ok());
        assert!(validate_folder_name("Test-File").is_ok());
        
        // Invalid names
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("con").is_err()); // Reserved name
        assert!(validate_document_name("aux").is_err()); // Reserved name
        assert!(validate_folder_name("file<script>").is_err());
        assert!(validate_series_name("../../../etc/passwd").is_err());
    }

    #[test]
    fn test_validate_project_name() {
        // Valid project names
        assert!(validate_project_name("My Story Project").is_ok());
        assert!(validate_project_name("Novel_2024").is_ok());
        
        // Invalid project names
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("<script>alert('xss')</script>").is_err());
    }

    #[test]
    fn test_validate_document_name() {
        // Valid document names
        assert!(validate_document_name("Chapter 1").is_ok());
        assert!(validate_document_name("Character_Notes").is_ok());
        
        // Invalid document names
        assert!(validate_document_name("").is_err());
        assert!(validate_document_name("<script>alert('xss')</script>").is_err());
    }

    #[test]
    fn test_validate_content_length() {
        // Valid content lengths
        assert!(validate_content_length("short content", 100).is_ok());
        assert!(validate_content_length("a".repeat(50).as_str(), 100).is_ok());
        
        // Invalid content lengths
        assert!(validate_content_length("a".repeat(101).as_str(), 100).is_err());
        assert!(validate_content_length("content\0with\0nulls", 100).is_err());
        
        // Excessive whitespace
        let excessive_whitespace = " ".repeat(1000) + "content";
        assert!(validate_content_length(&excessive_whitespace, 100).is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Test with Unicode characters
        assert!(validate_project_name("Проект").is_ok()); // Cyrillic
        assert!(validate_document_name("项目").is_ok()); // Chinese
        assert!(validate_folder_name("プロジェクト").is_ok()); // Japanese
        
        // Test with very long inputs
        let long_string = "a".repeat(10000);
        assert!(validate_content_length(&long_string, 5000).is_err());
        
        // Test with mixed attacks
        let mixed_attack = "<script>'; DROP TABLE users; --</script>";
        assert!(validate_security_input(mixed_attack).is_err());
    }

    #[test]
    fn test_performance_with_large_inputs() {
        // Test validation performance with large inputs
        let large_content = "This is a test sentence. ".repeat(1000);
        
        let start = std::time::Instant::now();
        let result = validate_content_length(&large_content, 100000);
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        assert!(duration.as_millis() < 100); // Should complete within 100ms
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::commands::projects::CreateProjectRequest;
    use crate::commands::documents::CreateDocumentRequest;
    use crate::database::models::{DocumentType};

    #[test]
    fn test_project_validation_integration() {
        // Test valid project creation request
        let valid_request = CreateProjectRequest {
            name: "My Story Project".to_string(),
            description: Some("A great project overview".to_string()),
            genre: Some("Fantasy".to_string()),
            target_word_count: Some(50000),
        };
        
        // These should pass validation
        assert!(validate_project_name(&valid_request.name).is_ok());
        if let Some(ref desc) = valid_request.description {
            assert!(validate_content_length(desc, 5000).is_ok());
            assert!(validate_security_input(desc).is_ok());
        }
        
        // Test invalid project creation request
        let invalid_request = CreateProjectRequest {
            name: "<script>alert('xss')</script>".to_string(),
            description: Some("'; DROP TABLE projects; --".to_string()),
            genre: Some("<img src=x onerror=alert(1)>".to_string()),
            target_word_count: Some(-1),
        };
        
        // These should fail validation
        assert!(validate_project_name(&invalid_request.name).is_err());
        if let Some(ref desc) = invalid_request.description {
            assert!(validate_security_input(desc).is_err());
        }
        if let Some(ref genre) = invalid_request.genre {
            assert!(validate_security_input(genre).is_err());
        }
    }

    #[test]
    fn test_document_validation_integration() {
        // Test valid document creation request
        let valid_request = CreateDocumentRequest {
            project_id: "valid-uuid-string".to_string(),
            title: "Chapter 1 The Beginning".to_string(),
            content: Some("This is the beginning of our story...".to_string()),
            document_type: DocumentType::Chapter,
            order_index: Some(1),
            parent_id: None,
        };
        
        // These should pass validation
        assert!(validate_security_input(&valid_request.project_id).is_ok());
        assert!(validate_document_name(&valid_request.title).is_ok());
        if let Some(ref content) = valid_request.content {
            assert!(validate_content_length(content, 1_000_000).is_ok());
            assert!(validate_security_input(content).is_ok());
        }
        
        // Test invalid document creation request
        let invalid_request = CreateDocumentRequest {
            project_id: "'; DROP TABLE documents; --".to_string(),
            title: "<script>alert('xss')</script>".to_string(),
            content: Some("javascript:alert(1)".to_string()),
            document_type: DocumentType::Chapter,
            order_index: Some(-1),
            parent_id: Some("<img src=x onerror=alert(1)>".to_string()),
        };
        
        // These should fail validation
        assert!(validate_security_input(&invalid_request.project_id).is_err());
        assert!(validate_document_name(&invalid_request.title).is_err());
        if let Some(ref content) = invalid_request.content {
            assert!(validate_security_input(content).is_err());
        }
        if let Some(ref parent_id) = invalid_request.parent_id {
            assert!(validate_security_input(parent_id).is_err());
        }
    }
}