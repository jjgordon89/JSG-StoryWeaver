import { test as base, expect, Page } from '@playwright/test';
import { join } from 'path';
import fs from 'fs';

// Define the Tauri window interface
declare global {
  interface Window {
    __TAURI__: {
      invoke: (cmd: string, args?: any) => Promise<any>;
    }
  }
}

// Extend the base test with custom fixtures
export const test = base.extend<{
  cleanDatabase: void;
}>({
  // Add a fixture for a clean test database
  cleanDatabase: async ({ page }, use) => {
    // Setup: Create a clean test database before the test
    await page.evaluate(async () => {
      try {
        // Use the Tauri API to initialize a clean database
        // This assumes there's a command to reset the database for testing
        await window.__TAURI__.invoke('init_database');
        return true;
      } catch (error) {
        console.error('Failed to initialize test database:', error);
        return false;
      }
    });
    
    // Use the fixture
    await use();
    
    // Teardown: No need to clean up as each test gets a fresh database
  },
});

// Re-export expect
export { expect };

// Helper functions
export async function createTestProject(page: Page, name: string, description: string = '') {
  // Click the "New Project" button
  await page.getByRole('button', { name: /new project/i }).click();
  
  // Fill in the project form
  await page.getByLabel(/project name/i).fill(name);
  if (description) {
    await page.getByLabel(/description/i).fill(description);
  }
  
  // Submit the form
  await page.getByRole('button', { name: /create/i }).click();
  
  // Wait for the project to be created
  await page.waitForSelector(`text=${name}`);
}

export async function createTestDocument(page: Page, title: string, content: string = '') {
  // Click the "New Document" button
  await page.getByRole('button', { name: /new document/i }).click();
  
  // Fill in the document form
  await page.getByLabel(/title/i).fill(title);
  
  // Submit the form
  await page.getByRole('button', { name: /create/i }).click();
  
  // Wait for the document to be created and opened in the editor
  await page.waitForSelector(`text=${title}`);
  
  // If content is provided, add it to the document
  if (content) {
    // Wait for the Monaco editor to be ready
    await page.waitForSelector('.monaco-editor');
    
    // Focus the editor and type the content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type(content);
    
    // Wait for auto-save to complete
    await page.waitForTimeout(1000); // Assuming auto-save debounce is less than 1s
  }
}

export async function openProjectPreview(page: Page, projectName: string) {
  // Find the project card
  const projectCard = page.locator(`.project-card:has-text("${projectName}")`);
  
  // Click the preview button on the project card
  await projectCard.locator('button[aria-label="Preview"]').click();
  
  // Wait for the preview modal to appear
  await page.waitForSelector('div[role="dialog"]');
}

export async function createFolder(page: Page, folderName: string, parentFolder: string = '') {
  // Click the "New Folder" button
  await page.getByRole('button', { name: /new folder/i }).click();
  
  // Fill in the folder name
  await page.getByLabel(/folder name/i).fill(folderName);
  
  // If a parent folder is specified, select it
  if (parentFolder) {
    await page.getByLabel(/parent folder/i).selectOption({ label: parentFolder });
  }
  
  // Submit the form
  await page.getByRole('button', { name: /create/i }).click();
  
  // Wait for the folder to be created
  await page.waitForSelector(`text=${folderName}`);
}

export async function createSeries(page: Page, seriesName: string, description: string = '') {
  // Navigate to series management
  await page.getByRole('link', { name: /series/i }).click();
  
  // Click the "New Series" button
  await page.getByRole('button', { name: /new series/i }).click();
  
  // Fill in the series form
  await page.getByLabel(/series name/i).fill(seriesName);
  if (description) {
    await page.getByLabel(/description/i).fill(description);
  }
  
  // Submit the form
  await page.getByRole('button', { name: /create/i }).click();
  
  // Wait for the series to be created
  await page.waitForSelector(`text=${seriesName}`);
}

export async function linkDocuments(page: Page, sourceDocTitle: string, targetDocTitle: string) {
  // Navigate to the source document
  await page.getByText(sourceDocTitle).click();
  
  // Open the document linking interface
  await page.getByRole('button', { name: /link documents/i }).click();
  
  // Select the target document
  await page.getByLabel(/select document/i).selectOption({ label: targetDocTitle });
  
  // Create the link
  await page.getByRole('button', { name: /create link/i }).click();
  
  // Wait for the link to be created
  await page.waitForSelector(`text=Linked to ${targetDocTitle}`);
}

export async function createBackup(page: Page, backupName: string = '') {
  // Navigate to backup management
  await page.getByRole('link', { name: /settings/i }).click();
  await page.getByRole('link', { name: /backup/i }).click();
  
  // Click the "Create Backup" button
  await page.getByRole('button', { name: /create backup/i }).click();
  
  // If a name is provided, fill it in
  if (backupName) {
    await page.getByLabel(/backup name/i).fill(backupName);
  }
  
  // Submit the form
  await page.getByRole('button', { name: /create/i }).click();
  
  // Wait for the backup to be created
  await page.waitForSelector(backupName ? `text=${backupName}` : 'text=Backup created');
}

export async function moveToTrash(page: Page, itemName: string, itemType: 'project' | 'document' | 'folder') {
  // Find and click on the item
  await page.getByText(itemName).click();
  
  // Click the delete/trash button
  if (itemType === 'project') {
    await page.getByRole('button', { name: /delete project/i }).click();
  } else if (itemType === 'document') {
    await page.getByRole('button', { name: /delete document/i }).click();
  } else if (itemType === 'folder') {
    await page.getByRole('button', { name: /delete folder/i }).click();
  }
  
  // Confirm the deletion
  await page.getByRole('button', { name: /confirm/i }).click();
  
  // Wait for the item to be moved to trash
  await page.waitForSelector(`text=${itemName}`, { state: 'detached' });
}

export async function createDocumentVersion(page: Page, documentTitle: string, versionName: string = '') {
  // Navigate to the document
  await page.getByText(documentTitle).click();
  
  // Open the version history
  await page.getByRole('button', { name: /version history/i }).click();
  
  // Click the "Create Version" button
  await page.getByRole('button', { name: /create version/i }).click();
  
  // If a name is provided, fill it in
  if (versionName) {
    await page.getByLabel(/version name/i).fill(versionName);
  }
  
  // Submit the form
  await page.getByRole('button', { name: /create/i }).click();
  
  // Wait for the version to be created
  await page.waitForSelector(versionName ? `text=${versionName}` : 'text=Version created');
}

export async function toggleFocusMode(page: Page) {
  // Click the focus mode toggle button
  await page.getByRole('button', { name: /focus mode/i }).click();
  
  // Wait for the focus mode to be toggled
  // This could be checking for a specific class or attribute that indicates focus mode is active
  await page.waitForSelector('.focus-mode-active');
}

export async function updateSettings(page: Page, settingKey: string, settingValue: string) {
  // Navigate to settings
  await page.getByRole('link', { name: /settings/i }).click();
  
  // Find the setting input by its label or key
  const settingInput = page.getByLabel(new RegExp(settingKey, 'i'));
  
  // Update the setting value
  if (await settingInput.isVisible()) {
    // If it's a text input
    await settingInput.fill(settingValue);
  } else {
    // If it's a select/dropdown
    await page.getByLabel(new RegExp(settingKey, 'i')).selectOption(settingValue);
  }
  
  // Save the settings
  await page.getByRole('button', { name: /save settings/i }).click();
  
  // Wait for the settings to be saved
  await page.waitForSelector('text=Settings saved');
}
