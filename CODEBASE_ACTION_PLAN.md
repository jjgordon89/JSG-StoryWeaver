# CODEBASE ACTION PLAN — StoryWeaver

Generated: 2025-08-12

This action plan documents discovered technical debt, incomplete implementations, and potential issues across the codebase. Each item includes a priority, effort estimate, dependencies, checkboxes for tracking, file references, and a suggested implementation approach.

Legend:
- Priority: [Critical] [High] [Medium] [Low]
- Effort: h = hours, d = days
- [ ] = not started, [~] = in progress, [x] = done

Note on line numbers: Where exact line numbers are not reliable due to tooling constraints, a search token is provided to quickly navigate (Ctrl/Cmd+F). The file references are precise and the snippets are uniquely identifiable.

---

## 0) Executive Summary

- Critical security and reliability work remains around input validation coverage and backend error-handling standardization (unwrap/expect usage).
- High-priority product work includes implementing AI streaming write path, cost estimation, and completing the Advanced AI overlay actions (cancel/copy/save/time tracking).
- Backend filter logic for AI card operations is stubbed and should be completed (date range, provider/model, cost).
- Context building lacks Story Bible integration; Brainstorm flow does not yet use base_prompt.
- Testing coverage is minimal on the frontend beyond two tests; backend mainly has security tests. Integration and e2e coverage needed.

---

## 1) Incomplete Implementations

1.1 Implement streaming write functionality in AIWritingPanel
- Priority: High
- Effort: 8–12h
- Dependencies: useAIWriteStream, backend streaming commands (guided_write_stream, write stream route), token/cost accounting
- Files:
  - src/components/ai/AIWritingPanel.tsx (search: "TODO: Implement streaming write functionality")
- [x] Implement streaming branch in "write" case (when settings.write.prose_mode === 'streaming') — wired to useAIWriteStream.startStreamingWrite in AIWritingPanel
- [~] Wire to useAIWriteStream hook to start/pause/stop subscriptions — start wired; pause/stop UI controls pending
- [x] Forward streaming segments into StreamingText component and cards — StreamingText consumes store updates; onComplete persists card
- Suggested approach:
  - Create a startWriteStream(documentId, cursorPosition, settings) that requests a stream_id from backend
  - Subscribe to Tauri event channel for stream chunks
  - On complete, call handleStreamingComplete to persist card entry

1.2 Implement credit/cost estimation in AIWritingPanel
- Priority: High
- Effort: 3–5h
- Dependencies: Token estimator, pricing table, WriteResult tokens_used
- Files:
  - src/components/ai/AIWritingPanel.tsx (search: "TODO: Implement credit estimation")
- [x] Replace const estimatedCost = 0 with real estimator — added aiCost.ts and integrated estimator in AIWritingPanel
- [x] Display cost badge before execution; include tool, model, and input size as factors — badge shown; tool/input factored; model factor integrated
- Suggested approach:
  - Create shared estimator utility using model-specific prices (per-1k tokens in/out)
  - Use selection length or prompt length for estimate, show range for streaming

1.3 AdvancedAI Overlay: cancel/copy/save/time tracking
- Priority: High
- Effort: 1–2d
- Dependencies: advancedAIStore capabilities, projectStore insert action, persistence layer
- Files:
  - src/components/AdvancedAI/StreamingStatusOverlay.tsx
    - Cancel generation (search: "TODO: Implement cancelGeneration")
    - Copy to clipboard centralization (search: "TODO: Implement copyToClipboard")
    - Store original prompt in metadata (search: "TODO: Store original prompt")
    - Save generated content API (search: "TODO: Implement saveGeneratedContent")
    - Generation time tracking (search: "TODO: Add generation time tracking")
- [x] Add cancelGeneration action into advancedAIStore to abort backend stream and mark status
- [x] Provide store-based utility for clipboard copy (with toast)
- [x] Plumb original prompt into generation metadata from calling site
- [x] Implement saveGeneratedContent to chosen location (document/snippet/note) — front-end action added; backend command implemented (save_generated_content)
- [x] Track generation start/end timestamps and compute elapsed time
- Suggested approach:
  - Augment advancedAIStore with fields: startedAt, finishedAt, streamId
  - Persist saves via a Tauri command that writes snippets/notes to DB

1.4 StyleManager actions (update/delete/bulk/generate-from-style)
- Priority: Medium
- Effort: 2–3d
- Dependencies: Backend endpoints for style examples or local persistence
- Files:
  - src/components/AdvancedAI/StyleManager.tsx (update ~101, delete ~120, bulk delete ~130, generate ~154)
- [ ] Implement Update action with optimistic UI and persistence
- [ ] Implement Delete action with optimistic UI and persistence
- [ ] Implement Bulk delete with selection management and confirmation
- [ ] Wire Generate-from-style to Write/Rewrite pipeline with style constraints
- Suggested approach:
  - Introduce a /style_examples table or use existing templates subsystem (if scope fits)
  - Leverage react-query for caching and invalidations

1.5 WriteProcessor: enrich AIContext with Story Bible
- Priority: High
- Effort: 1–2d
- Dependencies: story bible tables and operations
- Files:
  - src-tauri/src/ai/write_processor.rs (search: "TODO: Add Story Bible elements", ~240)
- [ ] Query characters, locations, and key lore for project_id
- [ ] Summarize into AIContext fields (characters, locations, lore)
- [ ] Gate volume via token budget
- Suggested approach:
  - Add database accessors pulling top-N relevant context by recency/frequency
  - Expose a configurable max-context tokens parameter

1.6 Brainstorm: use base_prompt with AI provider
- Priority: Medium
- Effort: 4–6h
- Dependencies: AI provider interface
- Files:
  - src-tauri/src/ai/brainstorm.rs (search: "TODO: Use base_prompt with AI provider", ~233)
- [ ] Replace placeholder returning all cards with real generation call
- [ ] Save results as brainstorm cards

1.7 AI card filtering implementations
- Priority: Medium
- Effort: 2–4d total (splitable)
- Dependencies: Query adjustments, indices
- Files:
  - src-tauri/src/database/operations/ai_card_ops.rs (search tokens):
    - "TODO: Implement actual date range filtering" (~85)
    - "TODO: Implement actual provider filtering" (~92)
    - "TODO: Add model filtering to AICardFilter" (~99)
    - "TODO: Implement actual cost range filtering" (~106)
- [ ] Add WHERE clauses to filter queries
- [ ] Update filter type(s) and UI to pass parameters
- Suggested approach:
  - Add composite indexes on (project_id, created_at), (project_id, provider, model_used)
  - Ensure pagination with filters

1.8 AIResponseCache time-based clearing
- Priority: Medium
- Effort: 6–10h
- Dependencies: cache structure (lru), background tasks
- Files:
  - src-tauri/src/database/optimization/mod.rs (search: "TODO: Implement time-based clearing in AIResponseCache", ~171)
- [ ] Add TTL per entry and background sweeper
- [ ] Expose admin endpoint to trigger cleanup manually

1.9 Extend credit/cost estimation to AIQuickTools
- Priority: High
- Effort: 2–3h
- Dependencies: Token estimator (aiCost.ts), pricing table
- Files:
  - src/components/ai/AIQuickTools.tsx (search: "TODO: Implement credit estimation", ~404)
- [ ] Integrate estimator to replace estimatedCost = 0
- [ ] Display cost badge per action; factor provider/model/tool and selection or prompt length
- Suggested approach:
  - Reuse estimator from AIWritingPanel to ensure consistent pricing
  - Add unit tests to verify badge rendering and disabled states when credits are insufficient

---

## 2) Code Quality Issues

2.1 Standardize error handling: replace unwrap/expect
- Priority: Critical
- Effort: 2–3d
- Dependencies: StoryWeaverError patterns, error.rs factory
- Files (examples; non-exhaustive):
  - src-tauri/src/database/operations/collaboration.rs (multiple unwrap/expect on id, conversions)
  - src-tauri/src/security/encryption.rs (unwrap on path parent)
  - src-tauri/src/lib.rs (expect on tauri application)
  - src-tauri/src/ai/brainstorm.rs (sessions.get(...).unwrap())
  - src-tauri/src/ai/token_counter.rs (unwrap in tests ok; production code must avoid)
- Hotspots (non-test; approximate lines):
  - src-tauri/src/security/encryption.rs:87 parent().unwrap()
  - src-tauri/src/lib.rs:413 expect("error while running tauri application")
  - src-tauri/src/commands/collaboration.rs:34 bcrypt::hash(...).unwrap()
  - src-tauri/src/ai/brainstorm.rs:490 sessions.get(session_id).unwrap()
  - src-tauri/src/background/mod.rs:273 tasks.remove(index).unwrap()
  - src-tauri/src/database/operations/collaboration.rs:49, 287, 414, 566, 661 unwrap/unwrap_or chains
- [ ] Replace unwrap/expect with ? and map_err to StoryWeaverError
- [ ] Standardize error envelope returned to UI
- Suggested approach:
  - Introduce helpers like to_db_error, to_io_error
  - Ensure no sensitive info leaks in messages

2.2 Duplicate/Conflicting UI labels in AIWritingPanel
- Priority: Medium
- Effort: 1–2h
- Dependencies: None
- Files:
  - src/components/ai/AIWritingPanel.tsx (two "Tone" selectors)
- [x] Consolidate tone controls, ensure distinct labels (e.g., “Tone Category” vs “Prose Mode”)
- [x] Validate settings.write shape alignment with store

2.3 Consistency: mock modes and dev stubs
- Priority: Medium
- Effort: 6–10h
- Dependencies: Build-time env gating
- Files:
  - src/utils/tauriSafe.ts (mock mode returns mock stream_id and text)
  - src/stores/versionStore.ts (mock in API simulation)
- [~] Gate all mock paths behind NODE_ENV === 'development' or explicit setting — tauriSafe gated; versionStore pending
- [x] Ensure production build cannot fall back to mock responses
- Suggested approach:
  - Introduce feature flags and a single MockGuard utility

2.4 Framework mixing (React + Svelte)
- Priority: Medium
- Effort: 2–3d (analysis/design); impl incremental
- Dependencies: Decision on mid/long-term direction
- Files:
  - src/components/SeriesConsistencyReport.svelte
  - src/components/SeriesConsistencyWidget.svelte
  - src/lib/components/templates/*.svelte
- [ ] Decide consolidation target (React-only or hybrid boundary)
- [ ] Create interop boundaries or plan migration of Svelte components to React (or vice versa)
- Suggested approach:
  - Document interop costs and target architecture
  - Prioritize porting components with most usage first

2.5 Documentation gaps
- Priority: Medium
- Effort: 1–2d
- Dependencies: None
- Files: Across src/components/* and hooks/*
- [ ] Add JSDoc/TSDoc for public APIs and hooks
- [ ] Update README sections for AI streaming and card persistence paths

---

## 3) Architectural Concerns

3.1 Input validation coverage across Tauri commands
- Priority: Critical
- Effort: 2–3d
- Dependencies: src-tauri/src/security/validation.rs; SECURITY_ANALYSIS_REPORT.md roadmap
- Files:
  - src-tauri/src/commands/projects.rs
  - src-tauri/src/commands/documents.rs
  - Other commands invoking DB/filesystem
- [ ] Ensure validate_safe_name, path checks, request size guards on all entry points
- [ ] Unit tests for each command path (valid/invalid inputs)
- Suggested approach:
  - Introduce per-command validator modules
  - Add size-limits middleware-like helpers

3.2 Rate limiting completeness
- Priority: High
- Effort: 1–2d
- Dependencies: src-tauri/src/security/rate_limit.rs (foundation exists, tests exist)
- Files:
  - Extend to remaining commands beyond projects/documents
- [ ] Map keys: per-project/per-document/per-feature
- [ ] Include sensible defaults and per-feature overrides
- Suggested approach:
  - Central throttle registry queried by commands before heavy ops

3.3 Card storage coupling and persistence
- Priority: Medium
- Effort: 1–2d
- Dependencies: addCard usage
- Files:
  - src/components/ai/AIWritingPanel.tsx (adds card after results)
- [ ] Ensure addCard persists to DB via Tauri, not only in-memory
- Suggested approach:
  - Confirm underlying useCards hook uses Tauri invoke persistently with retries and error handling

3.4 Performance: cache sweeper and indices
- Priority: Medium
- Effort: 1–2d
- Dependencies: DB schema
- Files:
  - src-tauri/src/database/optimization/*
- [ ] Add time-based cache cleanup (see 1.8)
- [ ] Review DB indices for card/filter queries
- Suggested approach:
  - CREATE INDEX on ai_cards(project_id, created_at), (project_id, provider, model_used)

---

## 4) Security and Validation Follow-ups

4.1 Complete input validation per endpoint
- Priority: Critical
- Effort: 2–3d
- Dependencies: validation.rs patterns and helpers
- Files: All Tauri command entry points
- [ ] Validate names, lengths, disallow unsafe patterns
- [ ] Distinguish dev vs prod lenient modes only where safe

4.2 Error handling standardization (factory pattern)
- Priority: High
- Effort: 1–2d
- Dependencies: src-tauri/src/error.rs (factory incompletely adopted per report)
- Files: All backend modules
- [ ] Produce consistent error envelope, log internal errors with audit tags
- [ ] Tests to assert redaction of sensitive info

4.3 CI Security scanning and automation
- Priority: Medium
- Effort: 1–2d
- Dependencies: GitHub Actions or equivalent
- [ ] Add cargo audit / npm audit
- [ ] Add dependency review and code scanning
- [ ] Schedule weekly scans (per SECURITY_ANALYSIS_REPORT.md)

---

## 5) Testing Gaps

5.1 Frontend unit tests expansion
- Priority: High
- Effort: 2–4d
- Dependencies: Vitest, Testing Library
- Files:
  - Existing: src/components/__tests__/ErrorBoundary.test.tsx
  - Existing: src/hooks/__tests__/useErrorHandler.test.ts
- [ ] Add tests for hooks: useAI, useAIWriteStream, useAICreative, useAISettings, useAICredits
- [ ] Add tests for AIWritingPanel basic flows (prompt entry, button disabled states, insert/replace callback)
- [ ] Add tests for StreamingStatusOverlay behaviors
- [ ] Add tests for AIQuickTools cost estimation badge and disable rules

5.2 Backend integration tests for commands
- Priority: High
- Effort: 3–5d
- Dependencies: Test harness with sqlite temp DB
- Files:
  - src-tauri/src/commands/*.rs
- [ ] Spin up ephemeral DB, cover happy-path and validation failure path
- [ ] Include rate-limit hit path behavior

5.3 E2E coverage with Playwright
- Priority: Medium
- Effort: 3–5d
- Dependencies: playwright.config.ts
- Files: tests/e2e/*
- [ ] Core flows: create project -> open doc -> AI write -> card saved -> reopen app shows persisted card
- [ ] Negative flows: invalid names, too-large content blocked with message

---

## 6) Deprecated/Outdated Patterns and Dependencies

6.1 Dependency audit follow-up
- Priority: Low
- Effort: 4–8h
- Dependencies: SECURITY_ANALYSIS_REPORT.md notes
- Files:
  - Backend: reqwest 0.11 (consider 0.12), tokio "1.0" (pin to ^1.38 to adopt newer runtime improvements), bcrypt 0.17 (ok), rsa monitoring via sqlx-mysql transitive (monitor)
  - Frontend: React 18.2 (ok), Vite 7.1, Playwright 1.40 (can upgrade to latest)
- [ ] Evaluate bump feasibility; run smoke tests

6.2 Build guard for mocks
- Priority: Medium
- Effort: 2–4h
- Files:
  - src/utils/tauriSafe.ts
- [x] Ensure mock mode cannot ship in production builds — tauriSafe throws in production

---

## 7) Task Dependency Graph (High-level)

- Standardized error handling (2.1) underpins robust endpoint behavior; do before heavy new features.
- Input validation coverage (3.1/4.1) should be completed early to secure all code paths.
- AI streaming write (1.1) depends on stable streaming backend and store cancelation (1.3).
- Card filter backends (1.7) benefit from indices decisions (3.4).
- Testing tasks (5.*) should follow feature completions but at least smoke/unit tests should be added in parallel.

---

## 8) Detailed Checklist (Trackable)

- [ ] [Critical] Standardize backend error handling; remove unwrap/expect everywhere
  - Effort: 2–3d
  - Files: collaboration.rs, encryption.rs, lib.rs, brainstorm.rs (sessions.get)
- [ ] [Critical] Complete input validation on all Tauri commands
  - Effort: 2–3d
  - Files: commands/projects.rs, commands/documents.rs, etc.
- [~] [High] AIWritingPanel streaming branch
  - Effort: 8–12h
  - File: src/components/ai/AIWritingPanel.tsx
- [x] [High] Credit/cost estimation display and calculation
  - Effort: 3–5h
  - File: src/components/ai/AIWritingPanel.tsx
- [x] [High] Overlay cancel/copy/save/time tracking
  - Effort: 1–2d
  - File: src/components/AdvancedAI/StreamingStatusOverlay.tsx
- [ ] [High] Backend integration tests for key commands
  - Effort: 3–5d
- [ ] [High] Extend cost estimation to AIQuickTools
  - Effort: 2–3h
  - File: src/components/ai/AIQuickTools.tsx
- [ ] [Medium] WriteProcessor Story Bible enrichment
  - Effort: 1–2d
  - File: src-tauri/src/ai/write_processor.rs
- [ ] [Medium] Brainstorm use base_prompt with provider
  - Effort: 4–6h
  - File: src-tauri/src/ai/brainstorm.rs
- [ ] [Medium] AI card filter implementations (date/provider/model/cost)
  - Effort: 2–4d
  - File: src-tauri/src/database/operations/ai_card_ops.rs
- [ ] [Medium] AIResponseCache time-based clearing
  - Effort: 6–10h
  - File: src-tauri/src/database/optimization/mod.rs
- [x] [Medium] Tone controls duplication cleanup
  - Effort: 1–2h
  - File: src/components/ai/AIWritingPanel.tsx
- [x] [Medium] Mock-mode gating for production safety
  - Effort: 2–4h
  - File: src/utils/tauriSafe.ts
- [ ] [Medium] Framework mixing strategy (React/Svelte)
  - Effort: 2–3d (plan); ongoing to implement
- [ ] [Medium] Frontend unit tests for AI hooks and panels
  - Effort: 2–4d
- [ ] [Medium] E2E tests for core authoring flow
  - Effort: 3–5d
- [ ] [Low] Dependency bump pass (reqwest/tokio/playwright)
  - Effort: 4–8h

---

## 9) File Path References and Snippets

Frontend
- src/components/ai/AIWritingPanel.tsx
  - Search: "TODO: Implement streaming write functionality"
  - Search: "TODO: Implement credit estimation"
  - Note: Two “Tone” selectors; consolidated on 2025-08-12
- src/components/ai/AIQuickTools.tsx
  - Search: "TODO: Implement credit estimation" at ~404
- src/components/AdvancedAI/StyleManager.tsx
  - TODOs: update (~101), delete (~120), bulk delete (~130), generate (~154)
- src/components/AdvancedAI/StreamingStatusOverlay.tsx
  - Implemented: cancelGeneration wired via advancedAIStore.cancelGeneration
  - Implemented: central copy via advancedAIStore.copyGeneratedTextToClipboard
  - Implemented: original prompt plumbed via lastGenerationRequest in store
  - Implemented: saveGeneratedContent implemented in store and backend (save_generated_content Tauri command)
  - Implemented: generation time tracking (elapsed) displayed
- src/utils/tauriSafe.ts
  - Mock responses for guided_write_stream and default generation path

Backend
- src-tauri/src/ai/write_processor.rs
  - Search: "TODO: Add Story Bible elements" at ~240
- src-tauri/src/ai/brainstorm.rs
  - Search: "TODO: Use base_prompt with AI provider" at ~233
- src-tauri/src/database/operations/ai_card_ops.rs
  - Date range TODO at ~85
  - Provider TODO at ~92
  - Model filtering TODO at ~99
  - Cost range TODO at ~106
- src-tauri/src/database/optimization/mod.rs
  - Search: "TODO: Implement time-based clearing in AIResponseCache" at ~171
- src-tauri/src/commands/advanced_ai_commands.rs
  - Implemented: save_generated_content Tauri command
  - Implemented: cancel_streaming_generation Tauri command
- src-tauri/src/lib.rs
  - Handlers registered for save_generated_content and cancel_streaming_generation
- Backend unwrap/expect hotspots (examples)
  - src-tauri/src/database/operations/collaboration.rs: unwraps at 49, 287, 414, 566, 661
  - src-tauri/src/security/encryption.rs: parent().unwrap() at 87
  - src-tauri/src/lib.rs: expect(...) at 413
  - src-tauri/src/ai/brainstorm.rs: sessions.get(...).unwrap() at 490

---

## 10) Suggested Implementation Patterns

- Error handling: Prefer `?` with `map_err(|e| StoryWeaverError::X { ... })` and central factory for shaping responses.
- Validation: Introduce per-command validator modules with typed input structs and unit tests.
- Streaming: Use Tauri event channels; ensure cancel path stops server-side task and unsubscribes listeners.
- Cost estimation: Centralize model pricing in one config; compute (input_tokens * in_price + output_tokens * out_price).
- Cache cleanup: TTL per entry + periodic cleanup via background task; expose manual trigger command.
- Testing: 
  - Frontend: vitest + react-testing-library, mock Tauri invoke.
  - Backend: tokio::test with temp sqlite; property tests for validation (quickcheck style if desired).
  - E2E: Playwright happy-path + negative validation scenarios.

---

## 11) Milestones and Sequencing

Milestone A: Secure and Stabilize Core (1–2 weeks)
- [ ] Complete input validation (Critical)
- [ ] Standardize error handling (Critical)
- [ ] Rate limiting coverage (High)

Milestone B: AI Productivity Features (1–2 weeks)
- [ ] Streaming write + overlay enhancements (High)
- [ ] Credit estimation (High)
- [ ] AI card filters + cache cleanup (Medium)

Milestone C: Quality and Confidence (1–2 weeks)
- [ ] Frontend unit tests for AI flows (Medium)
- [ ] Backend integration tests (High)
- [ ] E2E core flows (Medium)
- [ ] Mock gating + doc updates (Medium)

---

## 12) Risk Notes

- Streaming cancellation requires careful backend task management to prevent orphan processes.
- Mixing frameworks increases maintenance overhead; decide a clear target to reduce long-term complexity.
- Replacing unwraps can uncover latent error cases; allocate time to handle edge scenarios properly.
- Filter queries may require new indices to maintain performance.

---

Prepared as a living document. Update checkboxes as items progress, and extend sections with line-precise refs when specific files are actively being worked on.
