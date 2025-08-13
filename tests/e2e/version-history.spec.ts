import { test, expect } from './test-helpers';

test.describe('Document Version History', () => {
  test('should create document versions', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Projects")');
    
    // Create a test project
    const projectName = 'Version History Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create a document
    const documentTitle = 'Versioned Document';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(documentTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Add initial content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type('This is version 1 of the document.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Create a version
    await page.getByRole('button', { name: /version history|create version/i }).click();
    
    // Fill in version details if there's a form
    const versionNameField = page.getByLabel(/version name|label/i);
    if (await versionNameField.isVisible()) {
      await versionNameField.fill('Version 1');
      
      // Add a description if there's a field for it
      const descriptionField = page.getByLabel(/description|notes/i);
      if (await descriptionField.isVisible()) {
        await descriptionField.fill('Initial draft');
      }
      
      // Submit the form
      await page.getByRole('button', { name: /create|save|add/i }).click();
    }
    
    // Wait for the version to be created
    await page.waitForSelector('text=Version created successfully');
    
    // Modify the document content
    await page.locator('.monaco-editor').click();
    await page.keyboard.press('Control+A');
    await page.keyboard.press('Delete');
    await page.keyboard.type('This is version 2 of the document with significant changes.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Create another version
    await page.getByRole('button', { name: /version history|create version/i }).click();
    
    // Fill in version details if there's a form
    if (await versionNameField.isVisible()) {
      await versionNameField.fill('Version 2');
      
      // Add a description if there's a field for it
      const descriptionField = page.getByLabel(/description|notes/i);
      if (await descriptionField.isVisible()) {
        await descriptionField.fill('Major revision');
      }
      
      // Submit the form
      await page.getByRole('button', { name: /create|save|add/i }).click();
    }
    
    // Wait for the version to be created
    await page.waitForSelector('text=Version created successfully');
    
    // Open the version history panel
    await page.getByRole('button', { name: /version history|history|versions/i }).click();
    
    // Verify both versions are listed
    await expect(page.locator('text=Version 1')).toBeVisible();
    await expect(page.locator('text=Version 2')).toBeVisible();
  });
  
  test('should view and restore previous document versions', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Projects")');
    
    // Create a test project
    const projectName = 'Version Restore Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create a document
    const documentTitle = 'Document to Restore';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(documentTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Add initial content
    const version1Content = 'This is the first version content.';
    await page.locator('.monaco-editor').click();
    await page.keyboard.type(version1Content);
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Create a version
    await page.getByRole('button', { name: /version history|create version/i }).click();
    
    // Fill in version details if there's a form
    const versionNameField = page.getByLabel(/version name|label/i);
    if (await versionNameField.isVisible()) {
      await versionNameField.fill('First Version');
      
      // Submit the form
      await page.getByRole('button', { name: /create|save|add/i }).click();
    }
    
    // Wait for the version to be created
    await page.waitForSelector('text=Version created successfully');
    
    // Modify the document content
    const version2Content = 'This is the second version with completely different content.';
    await page.locator('.monaco-editor').click();
    await page.keyboard.press('Control+A');
    await page.keyboard.press('Delete');
    await page.keyboard.type(version2Content);
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Create another version
    await page.getByRole('button', { name: /version history|create version/i }).click();
    
    // Fill in version details if there's a form
    if (await versionNameField.isVisible()) {
      await versionNameField.fill('Second Version');
      
      // Submit the form
      await page.getByRole('button', { name: /create|save|add/i }).click();
    }
    
    // Wait for the version to be created
    await page.waitForSelector('text=Version created successfully');
    
    // Open the version history panel
    await page.getByRole('button', { name: /version history|history|versions/i }).click();
    
    // Click on the first version to view it
    await page.getByText('First Version').click();
    
    // Wait for the version preview to load
    await page.waitForSelector('text=Version Preview');
    
    // Verify the preview shows the first version content
    await expect(page.locator('.version-preview, .preview-content')).toContainText(version1Content);
    
    // Click the restore button
    await page.getByRole('button', { name: /restore|revert/i }).click();
    
    // Confirm the restore operation if there's a confirmation dialog
    const confirmButton = page.getByRole('button', { name: /confirm|yes|restore/i });
    if (await confirmButton.isVisible()) {
      await confirmButton.click();
    }
    
    // Wait for the restore to complete
    await page.waitForSelector('text=Version restored successfully');
    
    // Close the version history panel if it's still open
    const closeButton = page.getByRole('button', { name: /close|back/i });
    if (await closeButton.isVisible()) {
      await closeButton.click();
    }
    
    // Verify the document now contains the first version content
    await expect(page.locator('.monaco-editor')).toContainText(version1Content);
    await expect(page.locator('.monaco-editor')).not.toContainText(version2Content);
  });
  
  test('should delete document versions', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Projects")');
    
    // Create a test project
    const projectName = 'Version Deletion Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create a document
    const documentTitle = 'Document with Versions';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(documentTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Create multiple versions
    const versionNames = ['Version A', 'Version B', 'Version C'];
    
    for (let i = 0; i < versionNames.length; i++) {
      // Add some content
      await page.locator('.monaco-editor').click();
      await page.keyboard.press('Control+A');
      await page.keyboard.press('Delete');
      await page.keyboard.type(`This is content for ${versionNames[i]}.`);
      
      // Wait for auto-save
      await page.waitForTimeout(1000);
      
      // Create a version
      await page.getByRole('button', { name: /version history|create version/i }).click();
      
      // Fill in version details if there's a form
      const versionNameField = page.getByLabel(/version name|label/i);
      if (await versionNameField.isVisible()) {
        await versionNameField.fill(versionNames[i]);
        
        // Submit the form
        await page.getByRole('button', { name: /create|save|add/i }).click();
      }
      
      // Wait for the version to be created
      await page.waitForSelector('text=Version created successfully');
    }
    
    // Open the version history panel
    await page.getByRole('button', { name: /version history|history|versions/i }).click();
    
    // Verify all versions are listed
    for (const versionName of versionNames) {
      await expect(page.locator(`text=${versionName}`)).toBeVisible();
    }
    
    // Delete the middle version
    const versionToDelete = versionNames[1];
    const versionItem = page.locator(`tr:has-text("${versionToDelete}"), div:has-text("${versionToDelete}")`).first();
    
    // Click the delete button for this version
    await versionItem.getByRole('button', { name: /delete|remove/i }).click();
    
    // Confirm the deletion if there's a confirmation dialog
    const confirmButton = page.getByRole('button', { name: /confirm|yes|delete/i });
    if (await confirmButton.isVisible()) {
      await confirmButton.click();
    }
    
    // Wait for the deletion to complete
    await page.waitForSelector('text=Version deleted successfully');
    
    // Verify the deleted version is no longer listed
    await expect(page.locator(`text=${versionToDelete}`)).not.toBeVisible();
    
    // Verify the other versions are still there
    await expect(page.locator(`text=${versionNames[0]}`)).toBeVisible();
    await expect(page.locator(`text=${versionNames[2]}`)).toBeVisible();
  });
});
