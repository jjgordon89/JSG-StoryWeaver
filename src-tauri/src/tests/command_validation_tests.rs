//! Command validation tests for Tauri command handlers
//! 
//! This test suite focuses on input validation for command handlers,
//! testing both valid inputs (happy path) and invalid inputs (validation failures).
//! 
//! Test categories:
//! - Security input validation (XSS, SQLi patterns)
//! - Size limit validation (content length, body size)
//! - Numeric range validation (order_index, age, etc.)
//! - Empty/null input validation
//! - Rate limiting behavior

use std::env;
use crate::commands::{self, CommandResponse};
use crate::database::{init_test_db, models::*};

#[cfg(test)]
fn reset_rl() {
    #[allow(unused_imports)]
    use crate::security::rate_limit::reset_rate_limits_for_test;
    reset_rate_limits_for_test();
}

#[allow(dead_code)]
async fn setup(namespace: &str) {
    env::set_var("RL_NAMESPACE", namespace);
    init_test_db().await.expect("failed to init test db");
    reset_rl();
}

// ===== LOCATIONS COMMAND VALIDATION TESTS =====

#[tokio::test]
async fn create_location_rejects_malicious_project_id() {
    setup("test_create_location_malicious_id").await;

    let malicious_ids = vec![
        "<script>alert('xss')</script>".to_string(),
        "'; DROP TABLE locations; --".to_string(),
        "../../../etc/passwd".to_string(),
    ];

    for bad_id in malicious_ids {
        let req = commands::locations::CreateLocationRequest {
            project_id: bad_id.clone(),
            name: "Test Location".to_string(),
            description: None,
            location_type: None,
            geography: None,
            climate: None,
            culture: None,
            history: None,
            significance: None,
            visibility: None,
        };

        let resp: CommandResponse<Location> = commands::locations::create_location(req).await;
        assert!(!resp.success, "expected security validation to fail for: {}", bad_id);
        let err = resp.error.unwrap_or_default().to_lowercase();
        assert!(
            err.contains("validation") || err.contains("security") || err.contains("invalid"),
            "unexpected error for {}: {}",
            bad_id,
            err
        );
    }
}

#[tokio::test]
async fn create_location_rejects_oversized_description() {
    setup("test_create_location_oversized").await;

    // Create project first
    let proj_req = commands::projects::CreateProjectRequest {
        name: "Size Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<Project> = commands::projects::create_project(proj_req).await;
    assert!(proj_resp.success);
    let project = proj_resp.data.unwrap();

    // Description > 10,000 bytes should be rejected
    let huge_description = "a".repeat(10_001);
    let req = commands::locations::CreateLocationRequest {
        project_id: project.id,
        name: "Test Location".to_string(),
        description: Some(huge_description),
        location_type: None,
        geography: None,
        climate: None,
        culture: None,
        history: None,
        significance: None,
        visibility: None,
    };

    let resp: CommandResponse<Location> = commands::locations::create_location(req).await;
    assert!(!resp.success, "expected oversized description to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("size") || err.contains("limit") || err.contains("exceeds"),
        "unexpected error: {}",
        err
    );
}

// ===== STORY BIBLE COMMAND VALIDATION TESTS =====

#[tokio::test]
async fn create_story_bible_rejects_oversized_braindump() {
    setup("test_story_bible_oversized_braindump").await;

    // Create project first
    let proj_req = commands::projects::CreateProjectRequest {
        name: "Braindump Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<Project> = commands::projects::create_project(proj_req).await;
    assert!(proj_resp.success);
    let project = proj_resp.data.unwrap();

    // braindump > 50,000 bytes should be rejected
    let huge_braindump = "a".repeat(50_001);
    let req = commands::story_bible::CreateOrUpdateStoryBibleRequest {
        project_id: project.id,
        braindump: Some(huge_braindump),
        synopsis: None,
        genre: None,
        style: None,
        style_examples: None,
        pov_mode: None,
        global_pov: None,
        global_tense: None,
        global_character_pov_ids: None,
    };

    let resp: CommandResponse<StoryBible> = commands::story_bible::create_or_update_story_bible(req).await;
    assert!(!resp.success, "expected oversized braindump to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("size") || err.contains("limit") || err.contains("exceeds"),
        "unexpected error: {}",
        err
    );
}

#[tokio::test]
async fn search_world_elements_rejects_empty_query() {
    setup("test_world_elements_empty_query").await;

    let resp: CommandResponse<Vec<WorldElement>> = commands::story_bible::search_world_elements(
        "test_project_id".to_string(),
        "".to_string(),
    ).await;

    assert!(!resp.success, "expected empty query to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("empty") || err.contains("validation"),
        "unexpected error: {}",
        err
    );
}

#[tokio::test]
async fn create_outline_rejects_invalid_chapter_number() {
    setup("test_outline_invalid_chapter").await;

    // Create project first
    let proj_req = commands::projects::CreateProjectRequest {
        name: "Outline Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<Project> = commands::projects::create_project(proj_req).await;
    assert!(proj_resp.success);
    let project = proj_resp.data.unwrap();

    // Test negative chapter number
    let req = commands::story_bible::CreateOutlineRequest {
        project_id: project.id.clone(),
        chapter_number: -1,
        title: Some("Invalid Chapter".to_string()),
        summary: None,
        pov: None,
        tense: None,
        character_pov_ids: None,
    };

    let resp: CommandResponse<Outline> = commands::story_bible::create_outline(req).await;
    assert!(!resp.success, "expected negative chapter number to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("chapter number") || err.contains("between 1 and 10000"),
        "unexpected error: {}",
        err
    );

    // Test chapter number too high
    let req2 = commands::story_bible::CreateOutlineRequest {
        project_id: project.id,
        chapter_number: 10_001,
        title: Some("Invalid Chapter".to_string()),
        summary: None,
        pov: None,
        tense: None,
        character_pov_ids: None,
    };

    let resp2: CommandResponse<Outline> = commands::story_bible::create_outline(req2).await;
    assert!(!resp2.success, "expected chapter number > 10000 to be rejected");
    let err2 = resp2.error.unwrap_or_default().to_lowercase();
    assert!(
        err2.contains("chapter number") || err2.contains("between 1 and 10000"),
        "unexpected error: {}",
        err2
    );
}

#[tokio::test]
async fn create_scene_rejects_invalid_ranges() {
    setup("test_scene_invalid_ranges").await;

    // Test invalid scene number
    let req = commands::story_bible::CreateSceneRequest {
        outline_id: "test_outline_id".to_string(),
        scene_number: 0, // Invalid: must be >= 1
        title: Some("Invalid Scene".to_string()),
        summary: None,
        extra_instructions: None,
        pov: None,
        tense: None,
        character_pov_ids: None,
        word_count_estimate: None,
        credit_estimate: None,
    };

    let resp: CommandResponse<Scene> = commands::story_bible::create_scene(req).await;
    assert!(!resp.success, "expected scene number 0 to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("scene number") || err.contains("between 1 and 10000"),
        "unexpected error: {}",
        err
    );

    // Test invalid word count estimate
    let req2 = commands::story_bible::CreateSceneRequest {
        outline_id: "test_outline_id".to_string(),
        scene_number: 1,
        title: Some("Test Scene".to_string()),
        summary: None,
        extra_instructions: None,
        pov: None,
        tense: None,
        character_pov_ids: None,
        word_count_estimate: Some(-100), // Invalid: negative
        credit_estimate: None,
    };

    let resp2: CommandResponse<Scene> = commands::story_bible::create_scene(req2).await;
    assert!(!resp2.success, "expected negative word count to be rejected");
    let err2 = resp2.error.unwrap_or_default().to_lowercase();
    assert!(
        err2.contains("word count") || err2.contains("between 0 and 1,000,000"),
        "unexpected error: {}",
        err2
    );

    // Test invalid credit estimate
    let req3 = commands::story_bible::CreateSceneRequest {
        outline_id: "test_outline_id".to_string(),
        scene_number: 1,
        title: Some("Test Scene".to_string()),
        summary: None,
        extra_instructions: None,
        pov: None,
        tense: None,
        character_pov_ids: None,
        word_count_estimate: None,
        credit_estimate: Some(-50.0), // Invalid: negative
    };

    let resp3: CommandResponse<Scene> = commands::story_bible::create_scene(req3).await;
    assert!(!resp3.success, "expected negative credit estimate to be rejected");
    let err3 = resp3.error.unwrap_or_default().to_lowercase();
    assert!(
        err3.contains("credit estimate") || err3.contains("between 0 and 1,000,000"),
        "unexpected error: {}",
        err3
    );
}

// ===== SERIES COMMAND VALIDATION TESTS =====

#[tokio::test]
async fn create_series_rejects_malicious_input() {
    setup("test_create_series_malicious").await;

    let malicious_names = vec![
        "<script>alert('xss')</script>".to_string(),
        "'; DROP TABLE series; --".to_string(),
        "../../../etc/passwd".to_string(),
    ];

    for bad_name in malicious_names {
        let req = commands::series_commands::CreateSeriesRequest {
            name: bad_name.clone(),
            description: Some("Test series".to_string()),
            folder_id: None,
        };

        let resp: CommandResponse<Series> = commands::series_commands::create_series(req).await;
        assert!(!resp.success, "expected security validation to fail for: {}", bad_name);
        let err = resp.error.unwrap_or_default().to_lowercase();
        assert!(
            err.contains("validation") || err.contains("security") || err.contains("invalid"),
            "unexpected error for {}: {}",
            bad_name,
            err
        );
    }
}

// ===== FOLDER COMMAND VALIDATION TESTS =====

#[tokio::test]
async fn create_folder_rejects_invalid_input() {
    setup("test_create_folder_validation").await;

    // Test empty folder name
    let req = commands::folder_commands::CreateFolderRequest {
        name: "".to_string(), // Invalid: empty
        parent_folder_id: None,
        is_series: None,
    };

    let resp: CommandResponse<Folder> = commands::folder_commands::create_folder(req).await;
    assert!(!resp.success, "expected empty folder name to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("empty"),
        "unexpected error: {}",
        err
    );

    // Test folder name too long
    let long_name = "a".repeat(256); // Invalid: too long
    let req2 = commands::folder_commands::CreateFolderRequest {
        name: long_name,
        parent_folder_id: None,
        is_series: None,
    };

    let resp2: CommandResponse<Folder> = commands::folder_commands::create_folder(req2).await;
    assert!(!resp2.success, "expected long folder name to be rejected");
    let err2 = resp2.error.unwrap_or_default().to_lowercase();
    assert!(
        err2.contains("validation") || err2.contains("too long"),
        "unexpected error: {}",
        err2
    );
}

// ===== COMPREHENSIVE SECURITY PATTERN TESTS =====

#[tokio::test]
async fn comprehensive_security_input_validation() {
    setup("test_comprehensive_security").await;

    let malicious_patterns = vec![
        // XSS patterns
        "<script>alert('xss')</script>",
        "<img src=x onerror=alert('xss')>",
        "javascript:alert('xss')",
        
        // SQL injection patterns
        "'; DROP TABLE projects; --",
        "' OR '1'='1",
        "1; DELETE FROM documents WHERE 1=1; --",
        
        // Path traversal patterns
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "....//....//....//etc/passwd",
        
        // Command injection patterns
        "; rm -rf /",
        "| cat /etc/passwd",
        "&& whoami",
        
        // LDAP injection patterns
        "*)(&(objectClass=*)",
        "*)(uid=*",
        
        // XML/XXE patterns
        "<!DOCTYPE foo [<!ENTITY xxe SYSTEM \"file:///etc/passwd\">]>",
        
        // NoSQL injection patterns
        "'; return db.collection.drop(); //",
        "$where: function() { return true; }",
    ];

    for pattern in malicious_patterns {
        // Test project creation with malicious name
        let req = commands::projects::CreateProjectRequest {
            name: pattern.to_string(),
            description: None,
            genre: None,
            target_word_count: None,
        };
        let resp: CommandResponse<Project> = commands::projects::create_project(req).await;
        assert!(
            !resp.success,
            "expected security validation to fail for pattern: {}",
            pattern
        );
        let err = resp.error.unwrap_or_default().to_lowercase();
        assert!(
            err.contains("validation") || err.contains("security") || err.contains("invalid"),
            "unexpected error for pattern {}: {}",
            pattern,
            err
        );
    }
}

// ===== SIZE LIMIT BOUNDARY TESTS =====

#[tokio::test]
async fn size_limit_boundary_testing() {
    setup("test_size_boundaries").await;

    // Create project first
    let proj_req = commands::projects::CreateProjectRequest {
        name: "Size Boundary Test".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<Project> = commands::projects::create_project(proj_req).await;
    assert!(proj_resp.success);
    let project = proj_resp.data.unwrap();

    // Test document content at boundary (1,000,000 bytes)
    let boundary_content = "a".repeat(1_000_000); // Exactly at limit
    let req_at_limit = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Boundary Test".to_string(),
        content: Some(boundary_content),
        document_type: DocumentType::Notes,
        order_index: Some(0),
        parent_id: None,
    };
    let resp_at_limit: CommandResponse<Document> = commands::documents::create_document(req_at_limit).await;
    assert!(resp_at_limit.success, "content at exact limit should be accepted");

    // Test document content over boundary (1,000,001 bytes)
    let over_boundary_content = "a".repeat(1_000_001); // Over limit
    let req_over_limit = commands::documents::CreateDocumentRequest {
        project_id: project.id,
        title: "Over Boundary Test".to_string(),
        content: Some(over_boundary_content),
        document_type: DocumentType::Notes,
        order_index: Some(0),
        parent_id: None,
    };
    let resp_over_limit: CommandResponse<Document> = commands::documents::create_document(req_over_limit).await;
    assert!(!resp_over_limit.success, "content over limit should be rejected");
    let err = resp_over_limit.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("size") || err.contains("limit") || err.contains("exceeds"),
        "unexpected error: {}",
        err
    );
}

// ===== NUMERIC VALIDATION EDGE CASES =====

#[tokio::test]
async fn numeric_validation_edge_cases() {
    setup("test_numeric_edges").await;

    // Create project first
    let proj_req = commands::projects::CreateProjectRequest {
        name: "Numeric Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<Project> = commands::projects::create_project(proj_req).await;
    assert!(proj_resp.success);
    let project = proj_resp.data.unwrap();

    // Test character age boundaries
    let valid_ages = vec![0, 1, 500, 999, 1000]; // Valid range: 0-1000
    let invalid_ages = vec![-1, -100, 1001, 10000];

    for age in valid_ages {
        let req = commands::characters::CreateCharacterRequest {
            project_id: project.id.clone(),
            name: format!("Character Age {}", age),
            description: None,
            role: None,
            age: Some(age),
            appearance: None,
            personality: None,
            background: None,
            goals: None,
            relationships: None,
            visibility: None,
        };
        let resp: CommandResponse<Character> = commands::characters::create_character(req).await;
        assert!(resp.success, "age {} should be valid", age);
    }

    for age in invalid_ages {
        let req = commands::characters::CreateCharacterRequest {
            project_id: project.id.clone(),
            name: format!("Character Age {}", age),
            description: None,
            role: None,
            age: Some(age),
            appearance: None,
            personality: None,
            background: None,
            goals: None,
            relationships: None,
            visibility: None,
        };
        let resp: CommandResponse<Character> = commands::characters::create_character(req).await;
        assert!(!resp.success, "age {} should be invalid", age);
        let err = resp.error.unwrap_or_default().to_lowercase();
        assert!(
            err.contains("age") || err.contains("between 0 and 1000"),
            "unexpected error for age {}: {}",
            age,
            err
        );
    }
}

// ===== EMPTY/NULL INPUT VALIDATION =====

#[tokio::test]
async fn empty_input_validation() {
    setup("test_empty_inputs").await;

    // Test empty project name
    let req = commands::projects::CreateProjectRequest {
        name: "".to_string(), // Empty name
        description: None,
        genre: None,
        target_word_count: None,
    };
    let resp: CommandResponse<Project> = commands::projects::create_project(req).await;
    assert!(!resp.success, "empty project name should be rejected");

    // Test whitespace-only project name
    let req2 = commands::projects::CreateProjectRequest {
        name: "   ".to_string(), // Whitespace only
        description: None,
        genre: None,
        target_word_count: None,
    };
    let resp2: CommandResponse<Project> = commands::projects::create_project(req2).await;
    assert!(!resp2.success, "whitespace-only project name should be rejected");

    // Test empty document title
    let req3 = commands::documents::CreateDocumentRequest {
        project_id: "test_project_id".to_string(),
        title: "".to_string(), // Empty title
        content: Some("Test content".to_string()),
        document_type: DocumentType::Notes,
        order_index: Some(0),
        parent_id: None,
    };
    let resp3: CommandResponse<Document> = commands::documents::create_document(req3).await;
    assert!(!resp3.success, "empty document title should be rejected");
}

// ===== RATE LIMITING VALIDATION TESTS =====

#[tokio::test]
async fn rate_limiting_blocks_excessive_requests() {
    setup("test_rate_limiting").await;

    // Set very low rate limits for testing
    env::set_var("RL_CREATE_RPM", "2");
    env::set_var("RL_WINDOW_SECS", "60");
    reset_rl();

    // First two requests should succeed
    for i in 1..=2 {
        let req = commands::projects::CreateProjectRequest {
            name: format!("Project {}", i),
            description: None,
            genre: None,
            target_word_count: None,
        };
        let resp: CommandResponse<Project> = commands::projects::create_project(req).await;
        assert!(resp.success, "request {} should succeed", i);
    }

    // Third request should be rate limited
    let req3 = commands::projects::CreateProjectRequest {
        name: "Project 3".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let resp3: CommandResponse<Project> = commands::projects::create_project(req3).await;
    assert!(!resp3.success, "third request should be rate limited");
    let err3 = resp3.error.unwrap_or_default().to_lowercase();
    assert!(
        err3.contains("rate limit"),
        "expected rate limit error, got: {}",
        err3
    );
}

// ===== VALIDATOR HELPER TESTS =====

#[tokio::test]
async fn validator_helpers_work_correctly() {
    setup("test_validators").await;

    // Test validate_id helper
    use crate::security::validators::validate_id;
    
    // Valid ID should pass
    assert!(validate_id("test_id", "valid-id-123", 64).is_ok());
    
    // Empty ID should fail
    assert!(validate_id("test_id", "", 64).is_err());
    assert!(validate_id("test_id", "   ", 64).is_err());
    
    // Too long ID should fail
    let long_id = "a".repeat(65);
    assert!(validate_id("test_id", &long_id, 64).is_err());
    
    // Malicious ID should fail
    assert!(validate_id("test_id", "<script>alert('xss')</script>", 64).is_err());
    assert!(validate_id("test_id", "'; DROP TABLE test; --", 64).is_err());
}

#[tokio::test]
async fn numeric_range_validators_work_correctly() {
    setup("test_numeric_validators").await;

    use crate::security::validators::{validate_numeric_range_i32, validate_order_index_default, validate_rating_0_to_10};
    
    // Test i32 range validation
    assert!(validate_numeric_range_i32("test_field", 5, 0, 10).is_ok());
    assert!(validate_numeric_range_i32("test_field", 0, 0, 10).is_ok());
    assert!(validate_numeric_range_i32("test_field", 10, 0, 10).is_ok());
    assert!(validate_numeric_range_i32("test_field", -1, 0, 10).is_err());
    assert!(validate_numeric_range_i32("test_field", 11, 0, 10).is_err());
    
    // Test order index validation
    assert!(validate_order_index_default(0).is_ok());
    assert!(validate_order_index_default(5000).is_ok());
    assert!(validate_order_index_default(10_000).is_ok());
    assert!(validate_order_index_default(-1).is_err());
    assert!(validate_order_index_default(10_001).is_err());
    
    // Test rating validation
    assert!(validate_rating_0_to_10(0).is_ok());
    assert!(validate_rating_0_to_10(5).is_ok());
    assert!(validate_rating_0_to_10(10).is_ok());
    assert!(validate_rating_0_to_10(11).is_err());
}

// ===== DOCUMENT LINK VALIDATION TESTS =====

#[tokio::test]
async fn document_link_validation() {
    setup("test_document_link_validation").await;

    // Test malicious document IDs
    let malicious_ids = vec![
        "<script>alert('xss')</script>".to_string(),
        "'; DROP TABLE document_links; --".to_string(),
        "../../../etc/passwd".to_string(),
    ];

    for bad_id in malicious_ids {
        let req = commands::document_link_commands::CreateDocumentLinkRequest {
            from_document_id: bad_id.clone(),
            to_document_id: "valid_target_id".to_string(),
            link_order: None,
        };

        let resp: CommandResponse<DocumentLink> = commands::document_link_commands::create_document_link(req).await;
        assert!(!resp.success, "expected security validation to fail for: {}", bad_id);
        let err = resp.error.unwrap_or_default().to_lowercase();
        assert!(
            err.contains("validation") || err.contains("security") || err.contains("invalid"),
            "unexpected error for {}: {}",
            bad_id,
            err
        );
    }
}

 // ===== BACKUP COMMAND VALIDATION TESTS =====
 // Skipped: backup commands require an AppHandle and are validated in integration tests instead.

 // ===== PERFORMANCE COMMAND VALIDATION TESTS =====
 // Skipped: performance command uses a different positional signature and returns Result<PerformanceMetric, StoryWeaverError>.
 // These should be covered by integration tests or small unit tests that exercise the command wrapper with proper args.
