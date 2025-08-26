import { test, expect } from '@playwright/test';

test.describe('Smoke Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the dashboard page before each test
    await page.goto('/');
    // Expect the dashboard to be loaded
    await expect(page.locator('h1:has-text("Projects")')).toBeVisible();
  });

  test('should load the Project List/Dashboard', async ({ page }) => {
    // The beforeEach hook already handles this, so we just need to assert
    await expect(page).toHaveURL('/');
  });

  test('should navigate to the Document Editor', async ({ page }) => {
    // This test will require creating a project and a document first
    // For now, we will just simulate the navigation
    // TODO: Implement project and document creation once the API is available
    await page.goto('/projects/1/documents/1');
    await expect(page.locator('.monaco-editor')).toBeVisible();
  });

  test('should navigate to the Canvas View', async ({ page }) => {
    // This test will require creating a project first
    // For now, we will just simulate the navigation
    // TODO: Implement project creation once the API is available
    await page.goto('/projects/1/canvas');
    await expect(page.locator('.canvas-container')).toBeVisible();
  });

  test('should navigate to the Settings Page', async ({ page }) => {
    await page.getByTestId('settings-button').click();
    await expect(page.locator('h1:has-text("Settings")')).toBeVisible();
  });
});