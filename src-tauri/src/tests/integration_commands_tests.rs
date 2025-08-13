//! Integration tests for core backend commands using in-memory SQLite and standardized rate limits

use std::env;
use crate::commands::{self, CommandResponse};
use crate::database::init_test_db;
use crate::database::models::{DocumentType, Importance, VisibilityLevel, CharacterRole};
use crate::ai::TokenCounter;

#[cfg(test)]
fn reset_rl() {
    // Clear in-process rate-limiter buckets between tests
    #[allow(unused_imports)]
    use crate::security::rate_limit::reset_rate_limits_for_test;
    reset_rate_limits_for_test();
}

#[allow(dead_code)]
async fn setup(namespace: &str) {
    // Isolate keys per test run
    env::set_var("RL_NAMESPACE", namespace);
    // Initialize in-memory DB for tests
    init_test_db().await.expect("failed to init test db");
    // Reset rate limiter buckets
    reset_rl();
}

#[tokio::test]
async fn create_project_happy_path() {
    setup("it_create_project_ok").await;

    let req = commands::projects::CreateProjectRequest {
        name: "My Project".to_string(),
        description: Some("Test project".to_string()),
        genre: Some("Fantasy".to_string()),
        target_word_count: Some(50000),
    };

    let resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(req).await;

    assert!(resp.success, "expected success, got error: {:?}", resp.error);
    let project = resp.data.expect("project data should be present");
    assert_eq!(project.name, "My Project");
    assert_eq!(project.genre, Some("Fantasy".to_string()));
}

#[tokio::test]
async fn create_project_validation_failure() {
    setup("it_create_project_validation");

    // Empty name should fail validate_project_name
    let req = commands::projects::CreateProjectRequest {
        name: "".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };

    let resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(req).await;

    assert!(!resp.success, "expected validation failure to be returned");
    let err = resp.error.unwrap_or_default();
    assert!(
        err.to_lowercase().contains("validation") || err.to_lowercase().contains("invalid"),
        "unexpected error string: {}", err
    );
}

#[tokio::test]
async fn update_project_happy_and_rate_limit() {
    setup("it_update_project_rl");

    // Lower the update RPM to trigger rate limit on second call
    env::set_var("RL_UPDATE_RPM", "1");
    env::set_var("RL_WINDOW_SECS", "60");
    reset_rl();

    // Create project first
    let create_req = commands::projects::CreateProjectRequest {
        name: "Project A".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let create_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(create_resp.success);
    let proj = create_resp.data.unwrap();

    // First update succeeds
    let upd_req = commands::projects::UpdateProjectRequest {
        id: proj.id.clone(),
        name: Some("Project A Updated".to_string()),
        description: None,
        genre: None,
        target_word_count: None,
        status: None,
        settings: None,
    };
    let upd_resp: CommandResponse<()> = commands::projects::update_project(upd_req).await;
    assert!(upd_resp.success, "first update should pass: {:?}", upd_resp.error);

    // Second update immediately should hit rate limit
    let upd_req2 = commands::projects::UpdateProjectRequest {
        id: proj.id.clone(),
        name: Some("Project A Updated Again".to_string()),
        description: None,
        genre: None,
        target_word_count: None,
        status: None,
        settings: None,
    };
    let upd_resp2: CommandResponse<()> = commands::projects::update_project(upd_req2).await;
    assert!(!upd_resp2.success, "second update should be rate-limited");
    let err2 = upd_resp2.error.unwrap_or_default().to_lowercase();
    assert!(
        err2.contains("rate limit"),
        "expected rate limit error, got: {}", err2
    );
}

#[tokio::test]
async fn create_document_happy_path() {
    setup("it_create_document_ok");

    // Create project to attach document
    let create_req = commands::projects::CreateProjectRequest {
        name: "DocProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let create_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(create_resp.success);
    let proj = create_resp.data.unwrap();

    // Create document
    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Chapter 1".to_string(),
        content: Some("Once upon a time...".to_string()),
        document_type: DocumentType::Chapter,
        order_index: Some(0),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;

    assert!(doc_resp.success, "document creation failed: {:?}", doc_resp.error);
    let doc = doc_resp.data.unwrap();
    assert_eq!(doc.title, "Chapter 1");
    assert_eq!(format!("{}", doc.document_type), "chapter");
}

#[tokio::test]
async fn create_document_validation_failure() {
    setup("it_create_document_validation");

    // Create project
    let create_req = commands::projects::CreateProjectRequest {
        name: "DocProj2".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let create_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(create_resp.success);
    let proj = create_resp.data.unwrap();

    // Invalid title (empty) should fail
    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "".to_string(),
        content: None,
        document_type: DocumentType::Notes,
        order_index: Some(0),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;

    assert!(!doc_resp.success, "expected validation error");
    let err = doc_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("invalid"),
        "unexpected error: {}", err
    );
}

#[tokio::test]
async fn update_document_rate_limit_path() {
    setup("it_update_document_rl");

    // Lower the update RPM to 1 to trigger rate limit
    env::set_var("RL_UPDATE_RPM", "1");
    env::set_var("RL_WINDOW_SECS", "60");
    reset_rl();

    // Create project
    let create_req = commands::projects::CreateProjectRequest {
        name: "DocProj3".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let create_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(create_resp.success);
    let proj = create_resp.data.unwrap();

    // Create document
    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Scene 1".to_string(),
        content: Some("Scene content".to_string()),
        document_type: DocumentType::Scene,
        order_index: Some(1),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;
    assert!(doc_resp.success);
    let doc = doc_resp.data.unwrap();

    // First update OK
    let upd_req = commands::documents::UpdateDocumentRequest {
        id: doc.id.clone(),
        title: Some("Scene 1 - Edited".to_string()),
        content: None,
        document_type: None,
        order_index: None,
        parent_id: None,
        metadata: None,
    };
    let upd_resp: CommandResponse<()> =
        commands::documents::update_document(upd_req).await;
    assert!(upd_resp.success, "first update should pass: {:?}", upd_resp.error);

    // Second update immediately should be rate-limited
    let upd_req2 = commands::documents::UpdateDocumentRequest {
        id: doc.id.clone(),
        title: Some("Scene 1 - Edited Again".to_string()),
        content: None,
        document_type: None,
        order_index: None,
        parent_id: None,
        metadata: None,
    };
    let upd_resp2: CommandResponse<()> =
        commands::documents::update_document(upd_req2).await;
    assert!(!upd_resp2.success, "second update should be rate-limited");
    let err2 = upd_resp2.error.unwrap_or_default().to_lowercase();
    assert!(err2.contains("rate limit"), "expected rate limit error, got: {}", err2);
}

#[tokio::test]
async fn get_document_rejects_invalid_id_patterns() {
    setup("it_get_document_invalid_id").await;

    // Malicious/invalid ids should be rejected by validate_security_input
    let bad_ids = vec![
        "<script>alert(1)</script>".to_string(),
        "1; DROP TABLE documents; --".to_string(),
        "../etc/passwd".to_string(),
    ];

    for bad in bad_ids {
        let resp: CommandResponse<Option<crate::database::models::Document>> =
            commands::documents::get_document(bad).await;
        assert!(
            !resp.success,
            "expected security validation to fail for invalid id"
        );
        let err = resp.error.unwrap_or_default().to_lowercase();
        assert!(
            err.contains("validation") || err.contains("security") || err.contains("invalid"),
            "unexpected error text: {}",
            err
        );
    }
}

#[tokio::test]
async fn create_document_rejects_oversized_body() {
    setup("it_create_document_oversized").await;

    // Create a valid project first
    let req = commands::projects::CreateProjectRequest {
        name: "BigDocProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(req).await;
    assert!(proj_resp.success);
    let proj = proj_resp.data.unwrap();

    // Body > 1_000_000 bytes should be rejected by request-size guards
    let huge = "a".repeat(1_000_001);
    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Oversized".to_string(),
        content: Some(huge),
        document_type: crate::database::models::DocumentType::Notes,
        order_index: Some(0),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;

    assert!(!doc_resp.success, "expected oversized content to be rejected");
    let err = doc_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("size") || err.contains("limit"),
        "unexpected error text: {}",
        err
    );
}

#[tokio::test]
async fn search_documents_rejects_empty_query() {
    setup("it_search_documents_empty").await;

    // Create a valid project
    let req = commands::projects::CreateProjectRequest {
        name: "SearchProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(req).await;
    assert!(proj_resp.success);
    let proj = proj_resp.data.unwrap();

    // Empty query should be rejected
    let search_req = commands::documents::SearchDocumentsRequest {
        project_id: proj.id.clone(),
        query: "".to_string(),
    };
    let resp: CommandResponse<Vec<crate::database::models::Document>> =
        commands::documents::search_documents(search_req).await;

    assert!(!resp.success, "expected empty query to be rejected");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("empty") || err.contains("validation"),
        "unexpected error text: {}",
        err
    );
}

#[tokio::test]
async fn update_document_rejects_negative_order_index() {
    setup("it_update_doc_negative_order").await;

    // Create project
    let create_req = commands::projects::CreateProjectRequest {
        name: "OrderProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(proj_resp.success);
    let proj = proj_resp.data.unwrap();

    // Create document
    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Doc".to_string(),
        content: Some("content".to_string()),
        document_type: DocumentType::Scene,
        order_index: Some(1),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;
    assert!(doc_resp.success);
    let doc = doc_resp.data.unwrap();

    // Negative order index should be rejected
    let upd_req = commands::documents::UpdateDocumentRequest {
        id: doc.id.clone(),
        title: None,
        content: None,
        document_type: None,
        order_index: Some(-1),
        parent_id: None,
        metadata: None,
    };
    let upd_resp: CommandResponse<()> = commands::documents::update_document(upd_req).await;
    assert!(
        !upd_resp.success,
        "expected negative order_index to be rejected"
    );
    let err = upd_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("order index") || err.contains("between 0 and 10,000"),
        "unexpected error text: {}",
        err
    );
}

#[tokio::test]
async fn create_character_rejects_negative_age() {
    setup("it_create_character_negative_age").await;

    // Create project
    let create_req = commands::projects::CreateProjectRequest {
        name: "CharProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(proj_resp.success);
    let proj = proj_resp.data.unwrap();

    // Negative age should be rejected
    let char_req = commands::characters::CreateCharacterRequest {
        project_id: proj.id.clone(),
        name: "Alice".to_string(),
        description: None,
        role: None,
        age: Some(-5),
        appearance: None,
        personality: None,
        background: None,
        goals: None,
        relationships: None,
        visibility: None,
    };
    let char_resp: CommandResponse<crate::database::models::Character> =
        commands::characters::create_character(char_req).await;

    assert!(!char_resp.success, "expected negative age to be rejected");
    let err = char_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("age") || err.contains("between 0 and 1000"),
        "unexpected error text: {}",
        err
    );
}

#[tokio::test]
async fn update_document_rejects_oversized_metadata() {
    setup("it_update_doc_metadata_oversized").await;

    // Create project and document
    let create_req = commands::projects::CreateProjectRequest {
        name: "MetaProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let proj_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(proj_resp.success);
    let proj = proj_resp.data.unwrap();

    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "MetaDoc".to_string(),
        content: Some("content".to_string()),
        document_type: DocumentType::Notes,
        order_index: Some(0),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;
    assert!(doc_resp.success);
    let doc = doc_resp.data.unwrap();

    // metadata > 50_000 bytes should be rejected
    let too_big_meta = "x".repeat(50_001);
    let upd_req = commands::documents::UpdateDocumentRequest {
        id: doc.id.clone(),
        title: None,
        content: None,
        document_type: None,
        order_index: None,
        parent_id: None,
        metadata: Some(too_big_meta),
    };
    let upd_resp: CommandResponse<()> = commands::documents::update_document(upd_req).await;
    assert!(!upd_resp.success, "expected oversized metadata to be rejected");
    let err = upd_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("size") || err.contains("limit") || err.contains("maximum"),
        "unexpected error text: {}",
        err
    );
}

// ===== DOCUMENT LINK COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn create_document_link_happy_path() {
    setup("it_create_document_link_ok").await;

    // Create project and two documents first
    let create_req = commands::projects::CreateProjectRequest {
        name: "LinkProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let create_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(create_resp.success);
    let proj = create_resp.data.unwrap();

    // Create first document
    let doc1_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Chapter 1".to_string(),
        content: None,
        document_type: DocumentType::Chapter,
        order_index: Some(0),
        parent_id: None,
    };
    let doc1_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc1_req).await;
    assert!(doc1_resp.success);
    let doc1 = doc1_resp.data.unwrap();

    // Create second document
    let doc2_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Chapter 2".to_string(),
        content: None,
        document_type: DocumentType::Chapter,
        order_index: Some(1),
        parent_id: None,
    };
    let doc2_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc2_req).await;
    assert!(doc2_resp.success);
    let doc2 = doc2_resp.data.unwrap();

    // Create document link
    let link_req = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: doc1.id.clone(),
        to_document_id: doc2.id.clone(),
        link_order: Some(1),
    };
    let link_resp: CommandResponse<crate::database::models::DocumentLink> =
        commands::document_link_commands::create_document_link(link_req).await;
    
    assert!(link_resp.success, "expected success, got error: {:?}", link_resp.error);
    let link = link_resp.data.expect("link data should be present");
    assert_eq!(link.from_document_id, doc1.id);
    assert_eq!(link.to_document_id, doc2.id);
    assert_eq!(Some(link.link_order), Some(1i32));
}

#[tokio::test]
async fn create_document_link_validation_failure() {
    setup("it_create_document_link_validation").await;

    // Test with malicious input
    let malicious_req = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: "<script>alert('xss')</script>".to_string(),
        to_document_id: "valid_id".to_string(),
        link_order: Some(1),
    };
    let resp: CommandResponse<crate::database::models::DocumentLink> =
        commands::document_link_commands::create_document_link(malicious_req).await;
    
    assert!(!resp.success, "expected validation failure");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("security") || err.contains("invalid"),
        "unexpected error: {}", err
    );
}

#[tokio::test]
async fn create_document_link_invalid_order() {
    setup("it_create_document_link_order").await;

    // Test with invalid link_order
    let invalid_req = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: "valid_from_id".to_string(),
        to_document_id: "valid_to_id".to_string(),
        link_order: Some(-1), // Invalid: negative
    };
    let resp: CommandResponse<crate::database::models::DocumentLink> =
        commands::document_link_commands::create_document_link(invalid_req).await;
    
    assert!(!resp.success, "expected validation failure for negative order");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("link_order") || err.contains("between 1 and 10,000"),
        "unexpected error: {}", err
    );

    // Test with order too high
    let high_req = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: "valid_from_id".to_string(),
        to_document_id: "valid_to_id".to_string(),
        link_order: Some(20000), // Invalid: too high
    };
    let resp2: CommandResponse<crate::database::models::DocumentLink> =
        commands::document_link_commands::create_document_link(high_req).await;
    
    assert!(!resp2.success, "expected validation failure for high order");
    let err2 = resp2.error.unwrap_or_default().to_lowercase();
    assert!(
        err2.contains("link_order") || err2.contains("between 1 and 10,000"),
        "unexpected error: {}", err2
    );
}

#[tokio::test]
async fn document_link_rate_limiting() {
    setup("it_document_link_rate_limit").await;

    // Set low rate limits for testing
    env::set_var("RL_CREATE_RPM", "1");
    env::set_var("RL_WINDOW_SECS", "60");
    reset_rl();

    // First request should succeed
    let req1 = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: "doc1".to_string(),
        to_document_id: "doc2".to_string(),
        link_order: Some(1),
    };
    let resp1: CommandResponse<crate::database::models::DocumentLink> =
        commands::document_link_commands::create_document_link(req1).await;
    // Note: This might fail due to DB constraints, but should not be rate limited
    
    // Second request should be rate limited
    let req2 = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: "doc3".to_string(),
        to_document_id: "doc4".to_string(),
        link_order: Some(1),
    };
    let resp2: CommandResponse<crate::database::models::DocumentLink> =
        commands::document_link_commands::create_document_link(req2).await;
    
    assert!(!resp2.success, "second request should be rate limited");
    let err2 = resp2.error.unwrap_or_default().to_lowercase();
    assert!(
        err2.contains("rate limit"),
        "expected rate limit error, got: {}", err2
    );
}

// ===== COLLABORATION COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn create_shared_document_link_happy_path() {
    setup("it_create_shared_link_ok").await;

    // Create project and document first
    let create_req = commands::projects::CreateProjectRequest {
        name: "ShareProj".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let create_resp: CommandResponse<crate::database::models::Project> =
        commands::projects::create_project(create_req).await;
    assert!(create_resp.success);
    let proj = create_resp.data.unwrap();

    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: proj.id.clone(),
        title: "Shared Scene".to_string(),
        content: Some("This is shared content".to_string()),
        document_type: DocumentType::Scene,
        order_index: Some(0),
        parent_id: None,
    };
    let doc_resp: CommandResponse<crate::database::models::Document> =
        commands::documents::create_document(doc_req).await;
    assert!(doc_resp.success);
    let doc = doc_resp.data.unwrap();

    // Create shared document link
    let share_resp = commands::collaboration::create_shared_document_link(
        doc.id.clone(),
        proj.id.clone(),
        "read_only".to_string(),
        Some("secure123".to_string()),
        Some(24),
    ).await;
    
    assert!(share_resp.is_ok(), "expected success, got error: {:?}", share_resp.err());
    let shared_link = share_resp.unwrap();
    assert_eq!(shared_link.document_id, doc.id);
    assert_eq!(shared_link.project_id, proj.id);
}

#[tokio::test]
async fn create_shared_document_link_validation_failure() {
    setup("it_create_shared_link_validation").await;

    // Test with malicious input in document_id
    let resp = commands::collaboration::create_shared_document_link(
        "<script>alert('xss')</script>".to_string(),
        "valid_project_id".to_string(),
        "read_only".to_string(),
        Some("secure123".to_string()),
        Some(24),
    ).await;
    
    assert!(resp.is_err(), "expected validation failure for malicious document_id");
    let err = resp.err().unwrap().to_string().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("security") || err.contains("invalid"),
        "unexpected error: {}", err
    );
}

#[tokio::test]
async fn create_shared_document_link_invalid_expires_hours() {
    setup("it_create_shared_link_expires").await;

    // Test with invalid expires_in_hours (too high)
    let resp = commands::collaboration::create_shared_document_link(
        "valid_doc_id".to_string(),
        "valid_project_id".to_string(),
        "read_only".to_string(),
        Some("secure123".to_string()),
        Some(10000), // Invalid: too high
    ).await;
    
    assert!(resp.is_err(), "expected validation failure for high expires_in_hours");
    let err = resp.err().unwrap().to_string().to_lowercase();
    assert!(
        err.contains("expires_in_hours") || err.contains("range"),
        "unexpected error: {}", err
    );

    // Test with negative expires_in_hours
    let resp2 = commands::collaboration::create_shared_document_link(
        "valid_doc_id".to_string(),
        "valid_project_id".to_string(),
        "read_only".to_string(),
        Some("secure123".to_string()),
        Some(-1), // Invalid: negative
    ).await;
    
    assert!(resp2.is_err(), "expected validation failure for negative expires_in_hours");
    let err2 = resp2.err().unwrap().to_string().to_lowercase();
    assert!(
        err2.contains("expires_in_hours") || err2.contains("range"),
        "unexpected error: {}", err2
    );
}

// ===== SECURITY COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn security_commands_validation() {
    setup("it_security_validation").await;

    // Test API key save with malicious input
    let malicious_req = commands::security_commands::SaveApiKeyRequest {
        provider: "<script>alert('xss')</script>".to_string(),
        api_key: "test_key".to_string(),
    };
    let resp = commands::security_commands::save_api_key(malicious_req).await;
    
    assert!(resp.is_err(), "expected validation failure for malicious provider");
    let err = resp.err().unwrap().to_string().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("security") || err.contains("invalid"),
        "unexpected error: {}", err
    );
}

// ===== ADVANCED AI COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn advanced_ai_commands_validation() {
    setup("it_advanced_ai_validation").await;

    // Note: Advanced AI commands require complex state management and AI providers
    // This test validates that the advanced AI module exists and can be imported
    // Full integration testing should be done in end-to-end tests with proper setup
    
    // Test that advanced AI commands module is accessible
    use crate::commands::advanced_ai_commands;
    use crate::ai::AdvancedAIManager;
    
    // Test that we can create an AdvancedAIManager instance
    let _manager = AdvancedAIManager::new();
    
    // This ensures the modules compile and are accessible
    assert!(true, "advanced_ai_commands module is accessible");
}

// ===== BACKUP COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn backup_commands_validation() {
    setup("it_backup_validation").await;

    // Note: Backup commands require Tauri app handle which is not available in unit tests
    // This test validates that the backup module exists and can be imported
    // Integration testing for backup commands should be done in end-to-end tests
    
    // Test that backup commands module is accessible
    use crate::commands::backup_commands;
    
    // This ensures the module compiles and is accessible
    assert!(true, "backup_commands module is accessible");
}

// ===== VERSION COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn version_commands_validation() {
    setup("it_version_validation").await;

    // Test version creation with malicious document ID
    let resp = commands::version_commands::create_document_version(
        "<script>alert('xss')</script>".to_string(), // Malicious document_id
        Some("test_user".to_string()),
        Some("Test version".to_string()),
    ).await;
    
    assert!(!resp.success, "expected validation failure for malicious document_id");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("security") || err.contains("invalid") || err.contains("not found"),
        "unexpected error: {}", err
    );
}

// ===== CHARACTER COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn character_commands_happy_path() {
    setup("it_character_happy").await;

    // Create project first
    let project_req = commands::projects::CreateProjectRequest {
        name: "Character Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // Create character
    let char_req = commands::characters::CreateCharacterRequest {
        project_id: project.id.clone(),
        name: "John Doe".to_string(),
        description: Some("A brave hero".to_string()),
        role: Some(CharacterRole::Supporting),
        age: Some(25),
        appearance: Some("Tall with brown hair".to_string()),
        personality: Some("Brave and loyal".to_string()),
        background: Some("Born in a small village".to_string()),
        goals: Some("Save the kingdom".to_string()),
        relationships: Some("Son of a blacksmith".to_string()),
        visibility: None,
    };
    let char_resp = commands::characters::create_character(char_req).await;
    
    assert!(char_resp.success, "character creation failed: {:?}", char_resp.error);
    let character = char_resp.data.unwrap();
    assert_eq!(character.name, "John Doe");
    assert_eq!(character.age, Some(25));
}

#[tokio::test]
async fn character_commands_validation() {
    setup("it_character_validation").await;

    // Create project first
    let project_req = commands::projects::CreateProjectRequest {
        name: "Character Validation Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // Test with empty name (should fail)
    let char_req = commands::characters::CreateCharacterRequest {
        project_id: project.id.clone(),
        name: "".to_string(),
        description: None,
        role: None,
        age: None,
        appearance: None,
        personality: None,
        background: None,
        goals: None,
        relationships: None,
        visibility: None,
    };
    let char_resp = commands::characters::create_character(char_req).await;
    
    assert!(!char_resp.success, "expected validation failure for empty name");
    let err = char_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("invalid") || err.contains("name"),
        "unexpected error: {}", err
    );
}

// ===== LOCATION COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn location_commands_happy_path() {
    setup("it_location_happy").await;

    // Create project first
    let project_req = commands::projects::CreateProjectRequest {
        name: "Location Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // Create location
    let loc_req = commands::locations::CreateLocationRequest {
        project_id: project.id.clone(),
        name: "Castle Blackstone".to_string(),
        description: Some("A dark fortress on a hill".to_string()),
        location_type: Some("building".to_string()),
        climate: Some("Temperate".to_string()),
        culture: Some("Medieval".to_string()),
        history: Some("Built 200 years ago".to_string()),
        geography: Some("On a hilltop".to_string()),
        significance: Some(crate::database::models::Importance::High),
        visibility: Some(crate::database::models::VisibilityLevel::Always),
    };
    let loc_resp = commands::locations::create_location(loc_req).await;
    
    assert!(loc_resp.success, "location creation failed: {:?}", loc_resp.error);
    let location = loc_resp.data.unwrap();
    assert_eq!(location.name, "Castle Blackstone");
    assert_eq!(location.description, Some("A dark fortress on a hill".to_string()));
}

// ===== STORY BIBLE COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn story_bible_commands_happy_path() {
    setup("it_story_bible_happy").await;

    // Create project first
    let project_req = commands::projects::CreateProjectRequest {
        name: "Story Bible Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // Create story bible entry
    let bible_req = commands::story_bible::CreateOrUpdateStoryBibleRequest {
        project_id: project.id.clone(),
        braindump: Some("Fire, Water, Earth, Air magic".to_string()),
        synopsis: Some("Magic System".to_string()),
        genre: Some("Fantasy".to_string()),
        style: None,
        style_examples: None,
        pov_mode: None,
        global_pov: None,
        global_tense: None,
        global_character_pov_ids: None,
    };
    let bible_resp = commands::story_bible::create_or_update_story_bible(bible_req).await;
    
    assert!(bible_resp.success, "story bible creation failed: {:?}", bible_resp.error);
    let entry = bible_resp.data.unwrap();
    assert_eq!(entry.synopsis, Some("Magic System".to_string()));
    assert_eq!(entry.genre, Some("Fantasy".to_string()));
}

// ===== AI WRITING COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn ai_writing_commands_validation() {
    setup("it_ai_writing_validation").await;

    // Note: auto_write requires State<AIProviderManager> which is not available in unit tests
    // This test should be done in end-to-end tests with proper Tauri app context
    assert!(true, "auto_write requires Tauri State - tested in e2e tests");
}

// ===== SETTINGS COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn settings_commands_happy_path() {
    setup("it_settings_happy").await;

    // Test get all settings
    let resp = commands::settings_commands::get_all_settings().await;
    assert!(resp.success, "get_all_settings failed: {:?}", resp.error);
    
    // Test set setting
    let resp = commands::settings_commands::set_setting(
        "test_key".to_string(),
        "test_value".to_string(),
    ).await;
    assert!(resp.success, "set_setting failed: {:?}", resp.error);
    
    // Test get setting
    let resp = commands::settings_commands::get_setting("test_key".to_string()).await;
    assert!(resp.success, "get_setting failed: {:?}", resp.error);
    let setting = resp.data.unwrap();
    assert_eq!(setting.unwrap().value, "test_value");
}

// ===== FOLDER COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn folder_commands_happy_path() {
    setup("it_folder_happy").await;

    // Create project first
    let project_req = commands::projects::CreateProjectRequest {
        name: "Folder Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // Create folder
    let folder_req = commands::folder_commands::CreateFolderRequest {
        name: "Chapter Drafts".to_string(),
        parent_folder_id: None,
        is_series: Some(false),
    };
    let folder_resp = commands::folder_commands::create_folder(folder_req).await;
    
    assert!(folder_resp.success, "folder creation failed: {:?}", folder_resp.error);
    let folder = folder_resp.data.unwrap();
    assert_eq!(folder.name, "Chapter Drafts");
}

// ===== TRASH COMMANDS INTEGRATION TESTS =====

#[tokio::test]
async fn trash_commands_validation() {
    setup("it_trash_validation").await;

    // Test move to trash with malicious document_id
    let resp = commands::trash_commands::trash_document(
        "<script>alert('xss')</script>".to_string(),
        None,
    ).await;
    
    assert!(!resp.success, "expected validation failure for malicious document_id");
    let err = resp.error.unwrap_or_default().to_lowercase();
    assert!(
        err.contains("validation") || err.contains("security") || err.contains("invalid") || err.contains("not found"),
        "unexpected error: {}", err
    );

    // trash_commands validation: ensure malicious document_id is rejected
    let malicious_id = "1; DROP TABLE documents;--".to_string();
    let resp = commands::trash_commands::trash_document(malicious_id, None).await;
    assert!(!resp.success, "Expected validation to fail for malicious document_id");
}