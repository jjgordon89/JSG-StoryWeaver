# Input Validation Audit — StoryWeaver Tauri Commands
Generated: 2025-08-12

Scope: Review validation coverage and rate limiting across backend Tauri command handlers. This audit highlights present checks and gaps, with clickable refs to functions.

Legend: OK = validations present and sufficient; Needs = missing or weak checks

Standard we expect per endpoint:
- validate_security_input on all id-like and free-text fields
- validate_content_length and validate_request_body_size where large strings can arrive
- Bounds checks on numeric inputs
- rl_* rate limit where appropriate

Files and findings

1) Advanced AI Commands
File: src-tauri/src/commands/advanced_ai_commands.rs
- OK generate_with_prose_mode: [generate_with_prose_mode()](src-tauri/src/commands/advanced_ai_commands.rs:119) validates ids, prose_mode, text sizes, lists, numeric bounds
- OK generate_image: [generate_image()](src-tauri/src/commands/advanced_ai_commands.rs:174) validates project_id, text fields, resolution, optional fields
- OK create_brainstorm_session: [create_brainstorm_session()](src-tauri/src/commands/advanced_ai_commands.rs:210) validates fields and bounds
- OK get_brainstorm_session: [get_brainstorm_session()](src-tauri/src/commands/advanced_ai_commands.rs:253)
- OK rate_brainstorm_idea: [rate_brainstorm_idea()](src-tauri/src/commands/advanced_ai_commands.rs:265)
- OK mark_idea_as_keeper: [mark_idea_as_keeper()](src-tauri/src/commands/advanced_ai_commands.rs:284)
- OK add_style_example: [add_style_example()](src-tauri/src/commands/advanced_ai_commands.rs:300)
- OK analyze_text_style: [analyze_text_style()](src-tauri/src/commands/advanced_ai_commands.rs:339)
- OK prose mode queries: [get_available_prose_modes()](src-tauri/src/commands/advanced_ai_commands.rs:356), [get_prose_mode_details()](src-tauri/src/commands/advanced_ai_commands.rs:365)
- OK credit usage: [get_credit_usage()](src-tauri/src/commands/advanced_ai_commands.rs:382), images: [get_project_images()](src-tauri/src/commands/advanced_ai_commands.rs:405), [delete_generated_image()](src-tauri/src/commands/advanced_ai_commands.rs:417)
- OK saliency: [build_saliency_context()](src-tauri/src/commands/advanced_ai_commands.rs:430)
- OK smart import: [smart_import_content()](src-tauri/src/commands/advanced_ai_commands.rs:451)
- OK streaming stubs: [start_streaming_generation()](src-tauri/src/commands/advanced_ai_commands.rs:471), [get_stream_status()](src-tauri/src/commands/advanced_ai_commands.rs:522), [cancel_streaming_generation()](src-tauri/src/commands/advanced_ai_commands.rs:615)
- OK save generated content: [save_generated_content()](src-tauri/src/commands/advanced_ai_commands.rs:546) validates location, optional ids/title, and sizes

2) AI Writing
File: src-tauri/src/commands/ai_writing.rs
- OK auto_write: [auto_write()](src-tauri/src/commands/ai_writing.rs:271) validates document_id and settings
- OK guided_write: [guided_write()](src-tauri/src/commands/ai_writing.rs:292) validates prompt, settings
- OK streaming: [auto_write_stream()](src-tauri/src/commands/ai_writing.rs:324), [guided_write_stream()](src-tauri/src/commands/ai_writing.rs:391)
- OK tools: [rewrite_text()](src-tauri/src/commands/ai_writing.rs:491), [expand_text()](src-tauri/src/commands/ai_writing.rs:529), [describe_scene()](src-tauri/src/commands/ai_writing.rs:566), [brainstorm_ideas()](src-tauri/src/commands/ai_writing.rs:605), [visualize_scene()](src-tauri/src/commands/ai_writing.rs:641), [quick_edit()](src-tauri/src/commands/ai_writing.rs:664), [quick_chat()](src-tauri/src/commands/ai_writing.rs:696), [get_related_words()](src-tauri/src/commands/ai_writing.rs:766)

3) Documents
File: src-tauri/src/commands/documents.rs
- OK create_document: [create_document()](src-tauri/src/commands/documents.rs:44) uses validate_security_input, validate_document_name, request size guards, numeric bounds
- OK get_documents/get_document: [get_documents()](src-tauri/src/commands/documents.rs:95), [get_document()](src-tauri/src/commands/documents.rs:109)
- OK update_document: [update_document()](src-tauri/src/commands/documents.rs:123) includes content and metadata size checks
- OK save_document: [save_document()](src-tauri/src/commands/documents.rs:193) includes request size guard
- OK delete_document: [delete_document()](src-tauri/src/commands/documents.rs:221)
- OK search/tree/stats: [search_documents()](src-tauri/src/commands/documents.rs:235), [get_document_tree()](src-tauri/src/commands/documents.rs:267), [get_document_stats()](src-tauri/src/commands/documents.rs:347)

4) Projects
File: src-tauri/src/commands/projects.rs
- OK create_project: [create_project()](src-tauri/src/commands/projects.rs:36) validates name, description size, genre, target_word_count bounds
- OK get/update/delete: [get_project()](src-tauri/src/commands/projects.rs:94), [update_project()](src-tauri/src/commands/projects.rs:108), [delete_project()](src-tauri/src/commands/projects.rs:179)
- OK summary and stats: [get_project_summary()](src-tauri/src/commands/projects.rs:230), [get_projects()](src-tauri/src/commands/projects.rs:82), [update_project_word_count()](src-tauri/src/commands/projects.rs:196)

5) Characters
File: src-tauri/src/commands/characters.rs
- OK all endpoints validate ids, sizes, and numeric bounds; see [create_character()](src-tauri/src/commands/characters.rs:47), [update_character()](src-tauri/src/commands/characters.rs:160), [delete_character()](src-tauri/src/commands/characters.rs:267) and readers

6) Locations
File: src-tauri/src/commands/locations.rs
- OK endpoints include validate_request_body_size and content length for many fields; see [create_location()](src-tauri/src/commands/locations.rs:44), [update_location()](src-tauri/src/commands/locations.rs:165)

7) AI Cards
File: src-tauri/src/commands/ai_cards.rs
- OK create/update and query filters validate provider/model/status and sizes; see [create_ai_response_card()](src-tauri/src/commands/ai_cards.rs:11), [update_ai_response_card()](src-tauri/src/commands/ai_cards.rs:78), and filter endpoints

8) AI History
File: src-tauri/src/commands/ai_history.rs
- OK create/get/delete/clear covered; includes numeric bounds; see [create_ai_history()](src-tauri/src/commands/ai_history.rs:68), [get_ai_history()](src-tauri/src/commands/ai_history.rs:140), [delete_ai_history()](src-tauri/src/commands/ai_history.rs:281)

9) Backup Commands
File: src-tauri/src/commands/backup_commands.rs
- OK create/restore/delete validate filename/text; see [create_backup()](src-tauri/src/commands/backup_commands.rs:7), [restore_from_backup()](src-tauri/src/commands/backup_commands.rs:28), [delete_backup()](src-tauri/src/commands/backup_commands.rs:52)

10) Background Commands
File: src-tauri/src/commands/background_commands.rs
- OK create/get/cancel/cleanup validate text and numeric bounds; see [create_background_task()](src-tauri/src/commands/background_commands.rs:13), [get_all_background_tasks()](src-tauri/src/commands/background_commands.rs:91)

11) Canvas
File: src-tauri/src/commands/canvas.rs
- OK extensive validations for names, sizes, numeric ranges; see [create_canvas()](src-tauri/src/commands/canvas.rs:12), [create_canvas_element()](src-tauri/src/commands/canvas.rs:112)

12) Story Bible
File: src-tauri/src/commands/story_bible.rs
- OK multiple endpoints validate ids and content sizes across bible, traits, outlines, scenes; e.g. [create_or_update_story_bible()](src-tauri/src/commands/story_bible.rs:30), [create_outline()](src-tauri/src/commands/story_bible.rs:496), [create_scene()](src-tauri/src/commands/story_bible.rs:734)

13) Folder Commands
File: src-tauri/src/commands/folder_commands.rs
- Mixed: most endpoints validate with crate::security::validate_security_input; however [delete_folder()](src-tauri/src/commands/folder_commands.rs:171) uses is_safe_input; Needs replace with validate_security_input and add content length guards for name if not present

14) Document Link Commands
File: src-tauri/src/commands/document_link_commands.rs
- Needs validation across all endpoints; currently only rate limiting and DB ops. Add:
  - validate_security_input for id-like fields: from_document_id, to_document_id, id
  - Bounds: link_order &#62;= 1 and reasonable upper bound (e.g., 10_000)
  - request size guards if any free-text added in future

15) Collaboration
File: src-tauri/src/commands/collaboration.rs
- Needs: validate_security_input for document_id, project_id, share_type, tokens and content fields; validate_content_length where applicable; numeric bounds for expires_in_hours, max_participants; sanitize messages/comments. Current functions rely on DB layer only.

16) Optimization Commands
File: src-tauri/src/commands/optimization_commands.rs
- Needs: validate_security_input for string params in [create_index()](src-tauri/src/commands/optimization_commands.rs:114) and [schedule_maintenance()](src-tauri/src/commands/optimization_commands.rs:224); optional length guards; values are admin-only but should be validated.

17) Commands Mod
File: src-tauri/src/commands/mod.rs
- Health check [health_check()](src-tauri/src/commands/mod.rs:85) and greet [greet()](src-tauri/src/commands/mod.rs:157) are low-risk; consider length guard on greet name if exposed outside dev

Rate limiting overview
- Present and appropriate in most CRUD endpoints: rl_create/update/delete/list/search/save used consistently in files above

Priority fixes (mapped to CODEBASE_ACTION_PLAN)
- P1 Implement input validation in Document Link Commands across create/update/get/list/delete
- P1 Replace is_safe_input in Folder delete with validate_security_input and align with other endpoints
- P2 Add validations in Collaboration endpoints (ids, text sizes, numeric bounds)
- P2 Add validations in Optimization Commands for admin string inputs

Acceptance criteria per endpoint (to implement)
- All id-like strings: validate_security_input
- All variable-length strings: validate_content_length with appropriate max and validate_request_body_size if applicable
- Numeric fields: min/max bounds with clear errors
- Maintain existing rl_* rate limiting

Next steps
- Implement fixes for Document Link Commands and Folder delete
- Add unit/integration tests for invalid inputs per updated endpoints
- Update CODEBASE_ACTION_PLAN and SECURITY_ANALYSIS_REPORT checkboxes and references

End of audit