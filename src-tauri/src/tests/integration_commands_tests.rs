//! Integration tests for core backend commands using in-memory SQLite and standardized rate limits

use std::env;
use crate::commands::{self, CommandResponse};
use crate::database::init_test_db;
use crate::database::models::DocumentType;

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
        order_index: Some(1),
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