# StoryWeaver End-to-End Testing

This directory contains end-to-end tests for the StoryWeaver application. These tests verify that the backend and frontend components work together correctly.

## Test Structure

The tests are organized as follows:

- `e2e/` - End-to-end tests using Playwright
  - `test-helpers.ts` - Helper functions and utilities for tests
  - `project-preview.spec.ts` - Tests for the project preview feature
  - `document-linking.spec.ts` - Tests for document linking functionality
  - `folder-hierarchy.spec.ts` - Tests for folder organization and drag-and-drop
  - `backup-recovery.spec.ts` - Tests for backup and recovery system
  - `version-history.spec.ts` - Tests for document version history

## Running Tests

You can run the tests using the following npm scripts:

```bash
# Run all tests
npm run test:e2e

# Run tests with UI mode (shows browser and has a nice UI for debugging)
npm run test:e2e:ui

# Run tests in debug mode
npm run test:e2e:debug

# Run a specific test file
npx playwright test tests/e2e/project-preview.spec.ts

# Run tests in a specific browser
npx playwright test --project=chromium
```

## Test Configuration

The test configuration is defined in `playwright.config.ts` in the project root. This includes:

- Browser configurations
- Timeouts and retry settings
- Web server setup for testing

## Writing New Tests

When writing new tests, follow these guidelines:

1. Use the helper functions in `test-helpers.ts` for common operations
2. Each test file should focus on a specific feature or component
3. Tests should be independent and not rely on state from other tests
4. Use the `cleanDatabase` fixture to ensure a clean state for each test

### Example Test Structure

```typescript
import { test, expect } from './test-helpers';

test.describe('Feature Name', () => {
  test('should perform specific action', async ({ page, cleanDatabase }) => {
    // Test setup
    await page.goto('/');
    
    // Test actions
    await page.getByRole('button', { name: /button name/i }).click();
    
    // Assertions
    await expect(page.locator('selector')).toBeVisible();
  });
});
```

## Backend-UI Integration Testing

The main purpose of these tests is to verify that the backend and frontend components work together correctly. This includes:

1. Verifying that UI actions trigger the correct backend commands
2. Ensuring that backend responses are correctly displayed in the UI
3. Testing data persistence and retrieval
4. Validating error handling and edge cases

## Debugging Tests

If a test fails, you can:

1. Run the test with `--debug` flag to see the browser in action
2. Check the test report for screenshots and traces
3. Add `await page.pause()` in the test to pause execution at a specific point
4. Use `console.log()` statements to output debug information

## CI/CD Integration

These tests are designed to be run in a CI/CD pipeline. The configuration in `playwright.config.ts` includes settings for CI environments.
