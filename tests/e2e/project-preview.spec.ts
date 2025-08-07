import { test, expect } from './test-helpers';

test.describe('Project Preview Feature', () => {
  test('should display project preview with correct information', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Test Preview Project';
    const projectDescription = 'A project for testing the preview feature';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    await page.getByLabel(/description/i).fill(projectDescription);
    
    // Set genre and target word count if those fields exist
    const genreField = page.getByLabel(/genre/i);
    if (await genreField.isVisible()) {
      await genreField.fill('Science Fiction');
    }
    
    const targetWordCountField = page.getByLabel(/target word count/i);
    if (await targetWordCountField.isVisible()) {
      await targetWordCountField.fill('50000');
    }
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create a test document
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill('Chapter 1');
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Type some content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type('This is the first chapter of my test project. It contains some sample text to test the word count functionality.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Go back to the project list
    await page.getByRole('link', { name: /projects/i }).click();
    
    // Find the project card
    const projectCard = page.locator(`.project-card:has-text("${projectName}")`);
    
    // Click the preview button on the project card
    await projectCard.getByRole('button', { name: /preview/i }).click();
    
    // Wait for the preview modal to appear
    await page.waitForSelector('div[role="dialog"]');
    
    // Verify the project information is displayed correctly
    await expect(page.locator('div[role="dialog"] h2')).toHaveText(projectName);
    await expect(page.locator('div[role="dialog"] p')).toContainText(projectDescription);
    
    // Check for document count
    await expect(page.locator('div[role="dialog"] text=Documents')).toBeVisible();
    await expect(page.locator('div[role="dialog"] text=1')).toBeVisible();
    
    // Check for word count
    await expect(page.locator('div[role="dialog"] text=Word count:')).toBeVisible();
    
    // Check for recent documents section
    await expect(page.locator('div[role="dialog"] text=Recent Documents')).toBeVisible();
    await expect(page.locator('div[role="dialog"] text=Chapter 1')).toBeVisible();
    
    // Close the preview
    await page.locator('div[role="dialog"] button:has-text("Close")').click();
    
    // Verify the preview is closed
    await expect(page.locator('div[role="dialog"]')).not.toBeVisible();
  });
  
  test('should open project when clicking Open Project button', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Test Open Project';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Go back to the project list if needed
    const projectsLink = page.getByRole('link', { name: /projects/i });
    if (await projectsLink.isVisible()) {
      await projectsLink.click();
    }
    
    // Find the project card
    const projectCard = page.locator(`.project-card:has-text("${projectName}")`);
    
    // Click the preview button on the project card
    await projectCard.getByRole('button', { name: /preview/i }).click();
    
    // Wait for the preview modal to appear
    await page.waitForSelector('div[role="dialog"]');
    
    // Click the "Open Project" button
    await page.locator('div[role="dialog"] button:has-text("Open Project")').click();
    
    // Verify the project is opened
    // This could be checking for project-specific UI elements or the project name in a header
    await expect(page.locator('h1, h2')).toContainText(projectName);
    
    // Verify we're in the project view, not the project list
    await expect(page.locator('text=Documents')).toBeVisible();
  });
  
  test('should display project statistics in preview', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Test Statistics Project';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Set target word count if the field exists
    const targetWordCountField = page.getByLabel(/target word count/i);
    if (await targetWordCountField.isVisible()) {
      await targetWordCountField.fill('10000');
    }
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create multiple documents to test statistics
    for (let i = 1; i <= 3; i++) {
      // Create a new document
      await page.getByRole('button', { name: /new document/i }).click();
      await page.getByLabel(/title/i).fill(`Chapter ${i}`);
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the document editor to load
      await page.waitForSelector('.monaco-editor');
      
      // Type some content
      await page.locator('.monaco-editor').click();
      await page.keyboard.type(`This is chapter ${i} with some sample text. `.repeat(10));
      
      // Wait for auto-save
      await page.waitForTimeout(1000);
      
      // Go back to the project view
      await page.getByRole('link', { name: /documents/i }).click();
    }
    
    // Go back to the project list
    await page.getByRole('link', { name: /projects/i }).click();
    
    // Find the project card
    const projectCard = page.locator(`.project-card:has-text("${projectName}")`);
    
    // Click the preview button on the project card
    await projectCard.getByRole('button', { name: /preview/i }).click();
    
    // Wait for the preview modal to appear
    await page.waitForSelector('div[role="dialog"]');
    
    // Check for document count
    await expect(page.locator('div[role="dialog"] text=Documents')).toBeVisible();
    await expect(page.locator('div[role="dialog"] text=3')).toBeVisible();
    
    // Check for word count and progress
    await expect(page.locator('div[role="dialog"] text=Word count:')).toBeVisible();
    
    // Check for progress bar if target word count was set
    const progressBar = page.locator('div[role="dialog"] .bg-blue-600.h-2\\.5.rounded-full');
    if (await targetWordCountField.isVisible()) {
      await expect(progressBar).toBeVisible();
    }
    
    // Check for recent documents section
    await expect(page.locator('div[role="dialog"] text=Recent Documents')).toBeVisible();
    
    // Verify all three documents are listed (or at least some of them if there's pagination)
    for (let i = 1; i <= 3; i++) {
      // Only check for the most recent documents if they're visible
      // Some UIs might limit the number of recent documents shown
      const docElement = page.locator(`div[role="dialog"] text=Chapter ${i}`);
      if (i >= 1) { // Assuming at least the most recent document is shown
        await expect(docElement).toBeVisible();
      }
    }
    
    // Close the preview
    await page.locator('div[role="dialog"] button:has-text("Close")').click();
  });
});
