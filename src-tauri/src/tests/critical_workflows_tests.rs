//! Critical workflow integration tests for StoryWeaver backend
//!
//! This test suite focuses on testing complete workflows and critical business logic
//! that spans multiple commands and modules. These tests ensure that the core
//! functionality works end-to-end and that complex interactions are properly handled.

use std::env;
use crate::commands::{self, CommandResponse};
use crate::database::init_test_db;
use crate::database::models::{DocumentType, Project, Document};

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

// ===== COMPLETE PROJECT WORKFLOW TESTS =====

#[tokio::test]
async fn complete_project_creation_workflow() {
    setup("workflow_project_creation").await;

    // 1. Create a project
    let project_req = commands::projects::CreateProjectRequest {
        name: "Epic Fantasy Novel".to_string(),
        description: Some("A tale of magic and adventure".to_string()),
        genre: Some("Fantasy".to_string()),
        target_word_count: Some(100000),
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success, "Project creation failed: {:?}", project_resp.error);
    let project = project_resp.data.unwrap();

    // 2. Create multiple documents in the project
    let chapter1_req = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Chapter 1: The Beginning".to_string(),
        content: Some("In the beginning, there was magic...".to_string()),
        document_type: DocumentType::Chapter,
        order_index: Some(1),
        parent_id: None,
    };
    let chapter1_resp = commands::documents::create_document(chapter1_req).await;
    assert!(chapter1_resp.success, "Chapter 1 creation failed: {:?}", chapter1_resp.error);
    let chapter1 = chapter1_resp.data.unwrap();

    let notes_req = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Plot Notes".to_string(),
        content: Some("Key plot points and character arcs".to_string()),
        document_type: DocumentType::Notes,
        order_index: Some(1),
        parent_id: None,
    };
    let notes_resp = commands::documents::create_document(notes_req).await;
    assert!(notes_resp.success, "Notes creation failed: {:?}", notes_resp.error);

    // 3. Create characters for the project
    let protagonist_req = commands::characters::CreateCharacterRequest {
        project_id: project.id.clone(),
        name: "Aria Stormwind".to_string(),
        description: Some("A young mage with untapped potential".to_string()),
        age: Some(18),
        appearance: Some("Auburn hair, green eyes, average height".to_string()),
        personality: Some("Curious, brave, sometimes reckless".to_string()),
        background: Some("Orphaned at a young age, raised by the Mage Guild".to_string()),
        goals: Some("Master her magical abilities and find her true heritage".to_string()),
        relationships: Some("Mentored by Master Eldric".to_string()),
        role: Some(crate::database::models::CharacterRole::Protagonist),
        visibility: Some(crate::database::models::VisibilityLevel::Always),
    };
    let protagonist_resp = commands::characters::create_character(protagonist_req).await;
    assert!(protagonist_resp.success, "Protagonist creation failed: {:?}", protagonist_resp.error);

    // 4. Create locations for the project
    let academy_req = commands::locations::CreateLocationRequest {
        project_id: project.id.clone(),
        name: "Arcane Academy".to_string(),
        description: Some("A prestigious magical academy where young mages learn their craft".to_string()),
        location_type: Some("Academy".to_string()),
        geography: Some("Floating towers above the clouds".to_string()),
        climate: Some("Temperate with magical influences".to_string()),
        culture: Some("Academic and scholarly".to_string()),
        history: Some("Founded 500 years ago by the Great Mage Council".to_string()),
        significance: Some(crate::database::models::Importance::High),
        visibility: Some(crate::database::models::VisibilityLevel::Always),
    };
    let academy_resp = commands::locations::create_location(academy_req).await;
    assert!(academy_resp.success, "Academy creation failed: {:?}", academy_resp.error);

    // 5. Create story bible entries
    let magic_system_req = commands::story_bible::CreateOrUpdateStoryBibleRequest {
        project_id: project.id.clone(),
        braindump: Some("Magic in this world is based on elemental affinities. Each mage has a primary element (Fire, Water, Earth, Air, Light, Shadow) and can learn secondary elements with training. Magic requires both mental focus and physical gestures.".to_string()),
        synopsis: Some("Magic System Overview".to_string()),
        genre: Some("Fantasy".to_string()),
        style: None,
        style_examples: None,
        pov_mode: None,
        global_pov: None,
        global_tense: None,
        global_character_pov_ids: None,
    };
    let magic_system_resp = commands::story_bible::create_or_update_story_bible(magic_system_req).await;
    assert!(magic_system_resp.success, "Magic system entry creation failed: {:?}", magic_system_resp.error);

    // 6. Verify project summary includes all created content
    let summary_resp = commands::projects::get_project_summary(project.id.clone()).await;
    assert!(summary_resp.success, "Project summary retrieval failed: {:?}", summary_resp.error);
    let summary = summary_resp.data.unwrap();
    
    // Verify the summary contains our created content
    assert!(summary.document_count >= 2, "Expected at least 2 documents in summary");
    assert!(summary.character_count >= 1, "Expected at least 1 character in summary");
    assert!(summary.location_count >= 1, "Expected at least 1 location in summary");
}

// ===== DOCUMENT LINKING WORKFLOW TESTS =====

#[tokio::test]
async fn document_linking_workflow() {
    setup("workflow_document_linking").await;

    // 1. Create a project
    let project_req = commands::projects::CreateProjectRequest {
        name: "Linked Documents Test".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // 2. Create source document
    let source_doc_req = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Chapter 1".to_string(),
        content: Some("This chapter introduces the main character.".to_string()),
        document_type: DocumentType::Chapter,
        order_index: Some(1),
        parent_id: None,
    };
    let source_doc_resp = commands::documents::create_document(source_doc_req).await;
    assert!(source_doc_resp.success);
    let source_doc = source_doc_resp.data.unwrap();

    // 3. Create target document
    let target_doc_req = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Character Notes".to_string(),
        content: Some("Detailed notes about the main character.".to_string()),
        document_type: DocumentType::Notes,
        order_index: Some(1),
        parent_id: None,
    };
    let target_doc_resp = commands::documents::create_document(target_doc_req).await;
    assert!(target_doc_resp.success);
    let target_doc = target_doc_resp.data.unwrap();

    // 4. Create document link
    let link_req = commands::document_link_commands::CreateDocumentLinkRequest {
        from_document_id: source_doc.id.clone(),
        to_document_id: target_doc.id.clone(),
        link_order: Some(1),
    };
    let link_resp = commands::document_link_commands::create_document_link(link_req).await;
    assert!(link_resp.success, "Document link creation failed: {:?}", link_resp.error);

    // 5. Verify linked documents can be retrieved
    let linked_docs_resp = commands::document_link_commands::get_linked_documents(source_doc.id.clone()).await;
    assert!(linked_docs_resp.success, "Get linked documents failed: {:?}", linked_docs_resp.error);
    let linked_docs = linked_docs_resp.data.unwrap();
    assert!(!linked_docs.next.is_empty(), "Expected at least one linked document");
    
    let first_link = &linked_docs.next[0];
    assert_eq!(first_link.id, target_doc.id);
    assert_eq!(first_link.title, "Character Notes");
}

// ===== FOLDER ORGANIZATION WORKFLOW TESTS =====

#[tokio::test]
async fn folder_organization_workflow() {
    setup("workflow_folder_organization").await;

    // 1. Create a project
    let project_req = commands::projects::CreateProjectRequest {
        name: "Organized Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    // 2. Create parent folder
    let parent_folder_req = commands::folder_commands::CreateFolderRequest {
        name: "Manuscript".to_string(),
        parent_folder_id: None,
        is_series: Some(false),
    };
    let parent_folder_resp = commands::folder_commands::create_folder(parent_folder_req).await;
    assert!(parent_folder_resp.success, "Parent folder creation failed: {:?}", parent_folder_resp.error);
    let parent_folder = parent_folder_resp.data.unwrap();

    // 3. Create child folder
    let child_folder_req = commands::folder_commands::CreateFolderRequest {
        name: "Chapters".to_string(),
        parent_folder_id: Some(parent_folder.id.clone()),
        is_series: Some(false),
    };
    let child_folder_resp = commands::folder_commands::create_folder(child_folder_req).await;
    assert!(child_folder_resp.success, "Child folder creation failed: {:?}", child_folder_resp.error);
    let child_folder = child_folder_resp.data.unwrap();

    // 4. Create document in child folder
    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Chapter 1".to_string(),
        content: Some("First chapter content".to_string()),
        document_type: DocumentType::Chapter,
        order_index: Some(1),
        parent_id: Some(child_folder.id.clone()),
    };
    let doc_resp = commands::documents::create_document(doc_req).await;
    assert!(doc_resp.success, "Document creation in folder failed: {:?}", doc_resp.error);
    let document = doc_resp.data.unwrap();

    // 5. Verify folder hierarchy
    let folders_resp = commands::folder_commands::get_all_folders().await;
    assert!(folders_resp.success, "Get all folders failed: {:?}", folders_resp.error);
    let folders = folders_resp.data.unwrap();
    assert!(folders.len() >= 2, "Expected at least 2 folders");

    // 6. Verify document is in correct folder
    let docs_resp = commands::documents::get_documents(project.id.clone()).await;
    assert!(docs_resp.success, "Get documents failed: {:?}", docs_resp.error);
    let docs = docs_resp.data.unwrap();
    let created_doc = docs.iter().find(|d| d.id == document.id).unwrap();
    assert_eq!(created_doc.parent_id, Some(child_folder.id));
}

// ===== TRASH AND RECOVERY WORKFLOW TESTS =====

#[tokio::test]
async fn trash_and_recovery_workflow() {
    setup("workflow_trash_recovery").await;

    // 1. Create a project and document
    let project_req = commands::projects::CreateProjectRequest {
        name: "Trash Test Project".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project_resp = commands::projects::create_project(project_req).await;
    assert!(project_resp.success);
    let project = project_resp.data.unwrap();

    let doc_req = commands::documents::CreateDocumentRequest {
        project_id: project.id.clone(),
        title: "Document to Delete".to_string(),
        content: Some("This document will be deleted".to_string()),
        document_type: DocumentType::Notes,
        order_index: Some(1),
        parent_id: None,
    };
    let doc_resp = commands::documents::create_document(doc_req).await;
    assert!(doc_resp.success);
    let document = doc_resp.data.unwrap();

    // 2. Move document to trash
    let trash_resp = commands::trash_commands::trash_document(document.id.clone(), None).await;
    assert!(trash_resp.success, "Move to trash failed: {:?}", trash_resp.error);

    // 3. Verify document is in trash
    let trash_items_resp = commands::trash_commands::get_trash_items().await;
    assert!(trash_items_resp.success, "Get trash items failed: {:?}", trash_items_resp.error);
    let trash_items = trash_items_resp.data.unwrap();
    assert!(!trash_items.is_empty(), "Expected at least one item in trash");
    
    let trashed_doc = trash_items.iter().find(|item| item.item_id == document.id);
    assert!(trashed_doc.is_some(), "Document not found in trash");

    let deleted_item_id = trashed_doc.unwrap().id.clone();

    // 4. Restore document from trash
    let restore_resp = commands::trash_commands::restore_trash_item(deleted_item_id).await;
    assert!(restore_resp.success, "Restore from trash failed: {:?}", restore_resp.error);

    // 5. Verify document is restored
    let docs_resp = commands::documents::get_documents(project.id.clone()).await;
    assert!(docs_resp.success, "Get documents after restore failed: {:?}", docs_resp.error);
    let docs = docs_resp.data.unwrap();
    let restored_doc = docs.iter().find(|d| d.id == document.id);
    assert!(restored_doc.is_some(), "Document not found after restore");
}

// ===== SETTINGS AND CONFIGURATION WORKFLOW TESTS =====

#[tokio::test]
async fn settings_configuration_workflow() {
    setup("workflow_settings_config").await;

    // 1. Set multiple settings
    let settings_to_set = vec![
        ("theme", "dark"),
        ("auto_save_interval", "300"),
        ("default_font_size", "14"),
        ("spell_check_enabled", "true"),
    ];

    for (key, value) in &settings_to_set {
        let resp = commands::settings_commands::set_setting(
            key.to_string(),
            value.to_string(),
        ).await;
        assert!(resp.success, "Failed to set setting {}: {:?}", key, resp.error);
    }

    // 2. Retrieve all settings and verify
    let all_settings_resp = commands::settings_commands::get_all_settings().await;
    assert!(all_settings_resp.success, "Get all settings failed: {:?}", all_settings_resp.error);
    let all_settings = all_settings_resp.data.unwrap();

    // 3. Verify each setting was saved correctly
    for (key, expected_value) in &settings_to_set {
        let setting = all_settings.iter().find(|s| s.key == *key);
        assert!(setting.is_some(), "Setting {} not found", key);
        assert_eq!(setting.unwrap().value, *expected_value, "Setting {} has wrong value", key);
    }

    // 4. Update a setting
    let update_resp = commands::settings_commands::set_setting(
        "theme".to_string(),
        "light".to_string(),
    ).await;
    assert!(update_resp.success, "Failed to update theme setting: {:?}", update_resp.error);

    // 5. Verify the update
    let theme_resp = commands::settings_commands::get_setting("theme".to_string()).await;
    assert!(theme_resp.success, "Get theme setting failed: {:?}", theme_resp.error);
    let theme_setting = theme_resp.data.unwrap();
    assert_eq!(theme_setting.unwrap().value, "light", "Theme setting was not updated correctly");
}

// ===== RATE LIMITING WORKFLOW TESTS =====

#[tokio::test]
async fn rate_limiting_workflow() {
    setup("workflow_rate_limiting").await;

    // Set very restrictive rate limits for testing
    env::set_var("RL_CREATE_RPM", "2");
    env::set_var("RL_WINDOW_SECS", "60");
    reset_rl();

    // 1. Create first project (should succeed)
    let project1_req = commands::projects::CreateProjectRequest {
        name: "Rate Limit Test 1".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project1_resp = commands::projects::create_project(project1_req).await;
    assert!(project1_resp.success, "First project creation should succeed");

    // 2. Create second project (should succeed)
    let project2_req = commands::projects::CreateProjectRequest {
        name: "Rate Limit Test 2".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project2_resp = commands::projects::create_project(project2_req).await;
    assert!(project2_resp.success, "Second project creation should succeed");

    // 3. Create third project (should be rate limited)
    let project3_req = commands::projects::CreateProjectRequest {
        name: "Rate Limit Test 3".to_string(),
        description: None,
        genre: None,
        target_word_count: None,
    };
    let project3_resp = commands::projects::create_project(project3_req).await;
    assert!(!project3_resp.success, "Third project creation should be rate limited");
    
    let error = project3_resp.error.unwrap_or_default().to_lowercase();
    assert!(
        error.contains("rate limit") || error.contains("too many requests"),
        "Expected rate limit error, got: {}", error
    );
}