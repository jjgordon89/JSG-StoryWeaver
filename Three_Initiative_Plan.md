# Consolidated 1-2 Week Implementation Plan

This document outlines the execution-ready plan for three key initiatives: i18n Implementation, Playwright E2E Stabilization, and AI Provider Parity.

## 1. Objectives and Non-Goals

### i18n Implementation
*   **Objective**: Adopt `react-i18next` to externalize user-visible strings into locale files, establishing a clear translation pipeline for future languages.
*   **Non-Goal**: Achieving 100% translation coverage in the first two weeks. The initial focus will be on high-visibility components.

### Playwright E2E Stabilization
*   **Objective**: Stabilize the existing E2E test suite for reliable cross-browser execution (Chromium, Firefox, WebKit), enhance the test configuration, and add a suite of smoke tests for all canonical routes.
*   **Non-Goal**: Exhaustive E2E coverage for every edge case. The focus is on stabilizing the foundation and covering critical paths.

### AI Provider Parity
*   **Objective**: Unify the streaming and guided suggestion functionality to ensure a consistent user experience across all integrated AI providers. This includes implementing a standardized streaming envelope and a backend route for guided suggestions.
*   **Non-Goal**: A full-featured, custom prompt library. The focus is on achieving functional parity for the core AI features.

## 2. Assumptions About Current State

*   The `tests/e2e` directory contains a suite of Playwright tests that are functional but may be flaky, and the configuration lacks advanced features like sharding and a comprehensive test harness.
*   AI features are primarily handled by the `aiStore` and `useAI` hook on the frontend, with backend support in the `ai_writing`, `advanced_ai_commands`, and `ai_cards` modules.
*   The application currently has no internationalization support, with all user-visible strings hardcoded in the React components.

## 3. Week-by-Week Timeline and Milestones

### Week 1
*   **Milestone**: i18n Scaffolding and Initial Externalization
    *   **Entry**: No i18n library installed.
    *   **Exit**: `react-i18next` is integrated, and strings in the main layout and canvas components are externalized.
*   **Milestone**: Playwright Configuration and Smoke Tests
    *   **Entry**: Basic Playwright configuration in place.
    *   **Exit**: The `playwright.config.ts` is updated with a cross-browser matrix, and smoke tests are implemented for all canonical routes.
*   **Parallelization**: The i18n and Playwright tasks can be worked on concurrently.

### Week 2
*   **Milestone**: AI Streaming Parity and Guided Suggestions
    *   **Entry**: Disparate streaming implementations exist in the backend.
    *   **Exit**: A unified streaming envelope is implemented, and a new backend command for guided suggestions is created and integrated with the frontend.
*   **Milestone**: Critical Flow E2E Tests and CI Hardening
    *   **Entry**: Basic smoke tests are in place.
    *   **Exit**: E2E tests for critical user flows are implemented, and the CI/CD pipeline is updated to run the full test suite reliably.
*   **Dependency**: The AI parity work should begin after the Playwright stabilization to ensure new features are covered by tests.

## 4. Detailed Task Breakdown with Acceptance Criteria

### i18n Initiative
| Task | Owner | Inputs/Outputs | Acceptance Criteria | Risk/Mitigation |
| --- | --- | --- | --- | --- |
| **Install and configure `react-i18next`** | Frontend | `package.json`, `src/i18n/index.ts`, `src/main.tsx` | App boots with the i18n provider. A language toggle is available in the UI. | **Low**: Well-documented library. |
| **Externalize strings for high-visibility components** | Frontend | `src/components/layout`, `src/components/canvas` | All user-visible strings in the specified components are replaced with `t('key')` calls. | **Medium**: May miss some strings. Mitigation: Code review and QA pass. |
| **Define translation pipeline** | Frontend | `TRANSLATION_GUIDE.md` | A markdown file is created outlining the process for adding new strings and managing translations. | **Low**: Process-oriented task. |

### Playwright E2E Stabilization
| Task | Owner | Inputs/Outputs | Acceptance Criteria | Risk/Mitigation |
| --- | --- | --- | --- | --- |
| **Enhance `playwright.config.ts`** | QA/DevOps | `playwright.config.ts` | The configuration includes a cross-browser matrix, sensible timeouts, and project tags. | **Low**: Configuration-based task. |
| **Implement smoke tests for canonical routes** | QA | `tests/e2e/smoke` | A new suite of smoke tests is created, covering all main application routes. | **Medium**: Identifying all routes may be challenging. Mitigation: Review `react-router` configuration. |
| **Add E2E tests for critical flows** | QA | `tests/e2e/critical` | New tests are added for document creation, editing, and AI feature usage. | **Medium**: Flaky tests may be difficult to stabilize. Mitigation: Implement robust test harness with network mocking. |
| **Update CI workflow** | DevOps | `.github/workflows/playwright.yml` | The CI pipeline is updated to run the full E2E test suite on every pull request. | **High**: CI configuration can be complex. Mitigation: Incremental changes and thorough testing. |

### AI Provider Parity
| Task | Owner | Inputs/Outputs | Acceptance Criteria | Risk/Mitigation |
| --- | --- | --- | --- | --- |
| **Design and implement a unified streaming envelope** | Backend | `src-tauri/src/ai/streaming.rs` | A new Rust module is created to centralize provider-specific streaming logic. | **High**: Requires deep understanding of each provider's API. Mitigation: Start with a single provider and iterate. |
| **Create a guided suggestions backend command** | Backend | `src-tauri/src/commands/guided_suggestions.rs` | A new Tauri command is created to handle guided suggestions, leveraging the AI Cards pathway. | **Medium**: Data model for suggestions may be complex. Mitigation: Start with a minimal data model and expand as needed. |
| **Integrate guided suggestions with the frontend** | Frontend | `src/stores/aiStore.ts`, `src/hooks/useAI.ts` | The new guided suggestions command is integrated into the frontend, with a clear UI entry point. | **Medium**: UI implementation may be complex. Mitigation: Use a simple UI to start and enhance later. |

## 5. Risks, Constraints, and Mitigation

*   **Provider API Rate Limits**: The application may hit rate limits for the AI providers. **Mitigation**: Implement exponential backoff for API requests and provide clear error messages to the user.
*   **Tauri Event Throughput**: High-frequency streaming events may cause performance issues. **Mitigation**: Batch events where possible and use a separate thread for event processing.
*   **Cross-Browser Timing Differences**: E2E tests may be flaky due to timing differences between browsers. **Mitigation**: Use `waitFor` helpers in Playwright to ensure elements are ready before interaction.
*   **Localization Regressions**: New features may introduce hardcoded strings. **Mitigation**: Add a linter rule to detect hardcoded strings in JSX.

## 6. File-level Change Map and PR Slicing Plan

### Week 1
*   **PR 1: i18n Bootstrap**:
    *   **Create**: `src/i18n/index.ts`, `public/locales/en/common.json`, `public/locales/en/ui.json`
    *   **Modify**: `package.json`, `src/main.tsx`, `src/App.tsx`, `src/components/layout/Header.tsx`, `src/components/canvas/CanvasToolbar.tsx`
*   **PR 2: Playwright Config and Smoke Tests**:
    *   **Modify**: `playwright.config.ts`
    *   **Create**: `tests/e2e/smoke/navigation.spec.ts`, `tests/e2e/smoke/canvas.spec.ts`

### Week 2
*   **PR 3: AI Streaming Backend**:
    *   **Create**: `src-tauri/src/ai/streaming.rs`
    *   **Modify**: `src-tauri/src/commands/ai_writing.rs`, `src-tauri/src/commands/advanced_ai_commands.rs`
*   **PR 4: AI Streaming Frontend**:
    *   **Modify**: `src/stores/aiStore.ts`, `src/hooks/useAI.ts`
*   **PR 5: Guided Suggestions Backend**:
    *   **Create**: `src-tauri/src/commands/guided_suggestions.rs`
    *   **Modify**: `src-tauri/src/commands/ai_cards.rs`
*   **PR 6: Critical Flow E2E Tests**:
    *   **Create**: `tests/e2e/critical/document_creation.spec.ts`, `tests/e2e/critical/ai_writing.spec.ts`

## 7. Test Strategy Summary

*   **Unit Tests**: New backend modules (`streaming.rs`, `guided_suggestions.rs`) will have 80%+ unit test coverage.
*   **E2E Tests**: The existing E2E suite will be stabilized, and new tests will be added for smoke and critical flows.
*   **Contract Tests**: A lightweight contract test will be added for the new streaming envelope to ensure the frontend receives the expected data structure.
*   **i18n Snapshot Tests**: Snapshot tests will be added for key components to verify that all translation keys resolve correctly.

## 8. Memory Bank Updates

*   **`activeContext.md`**: Add an entry for the three new initiatives, outlining the current focus and decisions made.
*   **`progress.md`**: Update the milestones and status to reflect the new plan.
*   **`systemPatterns.md`**: Add sections for the new i18n pattern and the unified streaming envelope design.
*   **`techContext.md`**: Add `react-i18next` and its related dependencies to the tech stack.