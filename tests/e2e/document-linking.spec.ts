import { test, expect } from './test-helpers';

test.describe('Document Linking Feature', () => {
  test('should create a link between two documents', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Projects")');
    
    // Create a test project
    const projectName = 'Document Linking Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create first document
    const firstDocTitle = 'Chapter 1';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(firstDocTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Add some content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type('This is the first chapter.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Go back to the project view
    await page.getByRole('link', { name: /documents/i }).click();
    
    // Create second document
    const secondDocTitle = 'Chapter 2';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(secondDocTitle);
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Add some content
    await page.locator('.monaco-editor').click();
    await page.keyboard.type('This is the second chapter.');
    
    // Wait for auto-save
    await page.waitForTimeout(1000);
    
    // Go back to the project view
    await page.getByRole('link', { name: /documents/i }).click();
    
    // Navigate to the first document
    await page.getByText(firstDocTitle).click();
    
    // Open the document linking interface
    await page.getByRole('button', { name: /link documents/i }).click();
    
    // Wait for the linking interface to appear
    await page.waitForSelector('text=Link Documents');
    
    // Select the second document
    await page.getByLabel(/select document/i).selectOption({ label: secondDocTitle });
    
    // Create the link
    await page.getByRole('button', { name: /create link/i }).click();
    
    // Wait for the link to be created
    await page.waitForSelector(`text=Linked to ${secondDocTitle}`);
    
    // Verify the link exists
    await expect(page.locator(`text=Linked to ${secondDocTitle}`)).toBeVisible();
    
    // Navigate to the second document to check for incoming links
    await page.getByRole('link', { name: /documents/i }).click();
    await page.getByText(secondDocTitle).click();
    
    // Open the document linking interface
    await page.getByRole('button', { name: /link documents/i }).click();
    
    // Check for incoming links section
    await expect(page.locator('text=Incoming Links')).toBeVisible();
    await expect(page.locator(`text=${firstDocTitle}`)).toBeVisible();
  });
  
  test('should delete a document link', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Projects")');
    
    // Create a test project
    const projectName = 'Delete Link Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create two documents
    const docTitles = ['First Chapter', 'Second Chapter'];
    
    for (const title of docTitles) {
      await page.getByRole('button', { name: /new document/i }).click();
      await page.getByLabel(/title/i).fill(title);
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the document editor to load
      await page.waitForSelector('.monaco-editor');
      
      // Add some content
      await page.locator('.monaco-editor').click();
      await page.keyboard.type(`This is ${title}.`);
      
      // Wait for auto-save
      await page.waitForTimeout(1000);
      
      // Go back to the project view
      await page.getByRole('link', { name: /documents/i }).click();
    }
    
    // Navigate to the first document
    await page.getByText(docTitles[0]).click();
    
    // Open the document linking interface
    await page.getByRole('button', { name: /link documents/i }).click();
    
    // Select the second document
    await page.getByLabel(/select document/i).selectOption({ label: docTitles[1] });
    
    // Create the link
    await page.getByRole('button', { name: /create link/i }).click();
    
    // Wait for the link to be created
    await page.waitForSelector(`text=Linked to ${docTitles[1]}`);
    
    // Delete the link
    await page.getByRole('button', { name: /delete link/i }).click();
    
    // Confirm deletion if there's a confirmation dialog
    const confirmButton = page.getByRole('button', { name: /confirm|yes|delete/i });
    if (await confirmButton.isVisible()) {
      await confirmButton.click();
    }
    
    // Verify the link is deleted
    await expect(page.locator(`text=Linked to ${docTitles[1]}`)).not.toBeVisible();
  });
  
  test('should navigate between linked documents', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Projects")');
    
    // Create a test project
    const projectName = 'Navigation Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create three documents in sequence
    const docTitles = ['Chapter 1', 'Chapter 2', 'Chapter 3'];
    
    for (const title of docTitles) {
      await page.getByRole('button', { name: /new document/i }).click();
      await page.getByLabel(/title/i).fill(title);
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the document editor to load
      await page.waitForSelector('.monaco-editor');
      
      // Add some content
      await page.locator('.monaco-editor').click();
      await page.keyboard.type(`This is ${title}.`);
      
      // Wait for auto-save
      await page.waitForTimeout(1000);
      
      // Go back to the project view
      await page.getByRole('link', { name: /documents/i }).click();
    }
    
    // Create links between chapters in sequence
    for (let i = 0; i < docTitles.length - 1; i++) {
      // Navigate to the current document
      await page.getByText(docTitles[i]).click();
      
      // Open the document linking interface
      await page.getByRole('button', { name: /link documents/i }).click();
      
      // Select the next document
      await page.getByLabel(/select document/i).selectOption({ label: docTitles[i + 1] });
      
      // Create the link
      await page.getByRole('button', { name: /create link/i }).click();
      
      // Wait for the link to be created
      await page.waitForSelector(`text=Linked to ${docTitles[i + 1]}`);
      
      // Go back to the project view
      await page.getByRole('link', { name: /documents/i }).click();
    }
    
    // Navigate to the first document
    await page.getByText(docTitles[0]).click();
    
    // Verify we can navigate through the linked documents
    for (let i = 0; i < docTitles.length - 1; i++) {
      // Verify we're on the correct document
      await expect(page.locator('h1, h2')).toContainText(docTitles[i]);
      
      // Click the navigation link to the next document
      await page.getByRole('link', { name: new RegExp(docTitles[i + 1], 'i') }).click();
      
      // Verify we've navigated to the next document
      await expect(page.locator('h1, h2')).toContainText(docTitles[i + 1]);
      
      // Verify the document content is loaded
      await expect(page.locator('.monaco-editor')).toContainText(`This is ${docTitles[i + 1]}.`);
    }
  });
});
