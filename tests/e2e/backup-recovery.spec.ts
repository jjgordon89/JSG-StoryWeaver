import { test, expect } from './test-helpers';

test.describe('Backup and Recovery System', () => {
  test('should create a backup of the project', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Backup Test Project';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create a document
    const documentTitle = 'Test Document';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(documentTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Add some content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type('This is a test document for backup testing.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Navigate to the backup management section
    await page.getByRole('link', { name: /settings/i }).click();
    await page.getByRole('link', { name: /backup/i }).click();
    
    // Create a backup
    const backupName = 'Test Backup';
    await page.getByRole('button', { name: /create backup/i }).click();
    
    // Fill in the backup name if there's a field for it
    const backupNameField = page.getByLabel(/backup name/i);
    if (await backupNameField.isVisible()) {
      await backupNameField.fill(backupName);
    }
    
    // Add a description if there's a field for it
    const descriptionField = page.getByLabel(/description/i);
    if (await descriptionField.isVisible()) {
      await descriptionField.fill('Backup for testing purposes');
    }
    
    // Submit the form
    await page.getByRole('button', { name: /create|save|backup/i }).click();
    
    // Wait for the backup to be created
    await page.waitForSelector('text=Backup created successfully');
    
    // Verify the backup appears in the list
    if (backupName) {
      await expect(page.locator(`text=${backupName}`)).toBeVisible();
    } else {
      // If no name was provided, just check that a new backup entry exists
      await expect(page.locator('text=Backup created')).toBeVisible();
    }
  });
  
  test('should restore from a backup', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Restore Test Project';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create a document with specific content
    const documentTitle = 'Original Document';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(documentTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Add some content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type('This is the original content.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Navigate to the backup management section
    await page.getByRole('link', { name: /settings/i }).click();
    await page.getByRole('link', { name: /backup/i }).click();
    
    // Create a backup
    const backupName = 'Restore Point';
    await page.getByRole('button', { name: /create backup/i }).click();
    
    // Fill in the backup name if there's a field for it
    const backupNameField = page.getByLabel(/backup name/i);
    if (await backupNameField.isVisible()) {
      await backupNameField.fill(backupName);
    }
    
    // Submit the form
    await page.getByRole('button', { name: /create|save|backup/i }).click();
    
    // Wait for the backup to be created
    await page.waitForSelector('text=Backup created successfully');
    
    // Go back to the document
    await page.getByRole('link', { name: /documents/i }).click();
    await page.getByText(documentTitle).click();
    
    // Modify the document content
    await page.locator('.monaco-editor').click();
    await page.keyboard.press('Control+A');
    await page.keyboard.press('Delete');
    await page.keyboard.type('This is the modified content that should be reverted.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Navigate back to the backup management section
    await page.getByRole('link', { name: /settings/i }).click();
    await page.getByRole('link', { name: /backup/i }).click();
    
    // Find and click on the restore button for our backup
    const backupRow = page.locator(`tr:has-text("${backupName}")`);
    await backupRow.getByRole('button', { name: /restore/i }).click();
    
    // Confirm the restore operation
    await page.getByRole('button', { name: /confirm|yes|restore/i }).click();
    
    // Wait for the restore to complete
    await page.waitForSelector('text=Restore completed successfully');
    
    // Navigate back to the document
    await page.getByRole('link', { name: /documents/i }).click();
    await page.getByText(documentTitle).click();
    
    // Verify the original content has been restored
    await expect(page.locator('.monaco-editor')).toContainText('This is the original content.');
    await expect(page.locator('.monaco-editor')).not.toContainText('This is the modified content');
  });
  
  test('should manage backup retention and deletion', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Backup Management Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Navigate to the backup management section
    await page.getByRole('link', { name: /settings/i }).click();
    await page.getByRole('link', { name: /backup/i }).click();
    
    // Create multiple backups
    const backupNames = ['Backup 1', 'Backup 2', 'Backup 3'];
    
    for (const backupName of backupNames) {
      await page.getByRole('button', { name: /create backup/i }).click();
      
      // Fill in the backup name if there's a field for it
      const backupNameField = page.getByLabel(/backup name/i);
      if (await backupNameField.isVisible()) {
        await backupNameField.fill(backupName);
      }
      
      // Submit the form
      await page.getByRole('button', { name: /create|save|backup/i }).click();
      
      // Wait for the backup to be created
      await page.waitForSelector('text=Backup created successfully');
      
      // Wait a bit between backups
      await page.waitForTimeout(500);
    }
    
    // Verify all backups are listed
    for (const backupName of backupNames) {
      await expect(page.locator(`text=${backupName}`)).toBeVisible();
    }
    
    // Delete the second backup
    const backupToDelete = backupNames[1];
    const backupRow = page.locator(`tr:has-text("${backupToDelete}")`);
    await backupRow.getByRole('button', { name: /delete/i }).click();
    
    // Confirm the deletion
    await page.getByRole('button', { name: /confirm|yes|delete/i }).click();
    
    // Wait for the deletion to complete
    await page.waitForSelector('text=Backup deleted successfully');
    
    // Verify the backup is no longer listed
    await expect(page.locator(`text=${backupToDelete}`)).not.toBeVisible();
    
    // Verify the other backups are still there
    await expect(page.locator(`text=${backupNames[0]}`)).toBeVisible();
    await expect(page.locator(`text=${backupNames[2]}`)).toBeVisible();
    
    // Check if there's a cleanup old backups feature
    const cleanupButton = page.getByRole('button', { name: /cleanup|clean up|remove old/i });
    
    if (await cleanupButton.isVisible()) {
      // Test the cleanup feature
      await cleanupButton.click();
      
      // Set retention period if there's a field for it
      const retentionField = page.getByLabel(/retention|days|period/i);
      if (await retentionField.isVisible()) {
        await retentionField.fill('7');
      }
      
      // Confirm the cleanup
      await page.getByRole('button', { name: /confirm|yes|cleanup/i }).click();
      
      // Wait for the cleanup to complete
      await page.waitForSelector('text=Cleanup completed');
    }
  });
});
