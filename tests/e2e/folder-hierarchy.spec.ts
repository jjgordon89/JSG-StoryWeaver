import { test, expect } from './test-helpers';

test.describe('Folder Hierarchy Feature', () => {
  test('should create folders and organize them hierarchically', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Folder Hierarchy Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Navigate to the folder management section if needed
    const foldersLink = page.getByRole('link', { name: /folders/i });
    if (await foldersLink.isVisible()) {
      await foldersLink.click();
    }
    
    // Create parent folders
    const parentFolders = ['Part 1', 'Part 2'];
    
    for (const folderName of parentFolders) {
      // Click the "New Folder" button
      await page.getByRole('button', { name: /new folder/i }).click();
      
      // Fill in the folder name
      await page.getByLabel(/folder name/i).fill(folderName);
      
      // Submit the form
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the folder to be created
      await page.waitForSelector(`text=${folderName}`);
    }
    
    // Create child folders under Part 1
    const childFolders = ['Chapter 1', 'Chapter 2'];
    
    for (const folderName of childFolders) {
      // Click the "New Folder" button
      await page.getByRole('button', { name: /new folder/i }).click();
      
      // Fill in the folder name
      await page.getByLabel(/folder name/i).fill(folderName);
      
      // Select the parent folder
      await page.getByLabel(/parent folder/i).selectOption({ label: parentFolders[0] });
      
      // Submit the form
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the folder to be created
      await page.waitForSelector(`text=${folderName}`);
    }
    
    // Verify the folder hierarchy
    // First, check that the parent folders are visible
    for (const folderName of parentFolders) {
      await expect(page.locator(`.folder-item:has-text("${folderName}")`)).toBeVisible();
    }
    
    // Expand the first parent folder if it's not already expanded
    const part1Folder = page.locator(`.folder-item:has-text("${parentFolders[0]}")`);
    const expandButton = part1Folder.locator('button[aria-label="Expand"]');
    
    if (await expandButton.isVisible()) {
      await expandButton.click();
    }
    
    // Check that the child folders are visible under the expanded parent
    for (const folderName of childFolders) {
      await expect(page.locator(`.folder-item:has-text("${folderName}")`)).toBeVisible();
    }
  });
  
  test('should move documents between folders', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Document Moving Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create folders
    const folders = ['Drafts', 'Final'];
    
    for (const folderName of folders) {
      // Click the "New Folder" button
      await page.getByRole('button', { name: /new folder/i }).click();
      
      // Fill in the folder name
      await page.getByLabel(/folder name/i).fill(folderName);
      
      // Submit the form
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the folder to be created
      await page.waitForSelector(`text=${folderName}`);
    }
    
    // Create a document
    const documentTitle = 'Test Document';
    await page.getByRole('button', { name: /new document/i }).click();
    await page.getByLabel(/title/i).fill(documentTitle);
    
    // Initially assign it to the Drafts folder
    await page.getByLabel(/folder/i).selectOption({ label: folders[0] });
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the document editor to load
    await page.waitForSelector('.monaco-editor');
    
    // Go back to the project view
    await page.getByRole('link', { name: /documents/i }).click();
    
    // Verify the document is in the Drafts folder
    // First, make sure the Drafts folder is expanded
    const draftsFolder = page.locator(`.folder-item:has-text("${folders[0]}")`);
    const expandDraftsButton = draftsFolder.locator('button[aria-label="Expand"]');
    
    if (await expandDraftsButton.isVisible()) {
      await expandDraftsButton.click();
    }
    
    // Check that the document is visible under Drafts
    await expect(page.locator(`.folder-item:has-text("${folders[0]}") ~ .folder-contents .document-item:has-text("${documentTitle}")`)).toBeVisible();
    
    // Move the document to the Final folder
    // This could be done via drag and drop, but that's complex to simulate
    // Instead, we'll use the move option if available
    
    // First, click on the document to select it
    await page.locator(`.document-item:has-text("${documentTitle}")`).click();
    
    // Look for a move button or context menu
    const moveButton = page.getByRole('button', { name: /move|move to folder/i });
    
    if (await moveButton.isVisible()) {
      await moveButton.click();
      
      // Select the destination folder
      await page.getByLabel(/destination folder/i).selectOption({ label: folders[1] });
      
      // Confirm the move
      await page.getByRole('button', { name: /move|confirm/i }).click();
    } else {
      // Alternative: right-click and use context menu
      await page.locator(`.document-item:has-text("${documentTitle}")`).click({ button: 'right' });
      await page.getByText(/move to folder/i).click();
      await page.getByLabel(/destination folder/i).selectOption({ label: folders[1] });
      await page.getByRole('button', { name: /move|confirm/i }).click();
    }
    
    // Verify the document is now in the Final folder
    // First, make sure the Final folder is expanded
    const finalFolder = page.locator(`.folder-item:has-text("${folders[1]}")`);
    const expandFinalButton = finalFolder.locator('button[aria-label="Expand"]');
    
    if (await expandFinalButton.isVisible()) {
      await expandFinalButton.click();
    }
    
    // Check that the document is visible under Final
    await expect(page.locator(`.folder-item:has-text("${folders[1]}") ~ .folder-contents .document-item:has-text("${documentTitle}")`)).toBeVisible();
    
    // Verify it's no longer in the Drafts folder
    await expect(page.locator(`.folder-item:has-text("${folders[0]}") ~ .folder-contents .document-item:has-text("${documentTitle}")`)).not.toBeVisible();
  });
  
  test('should support drag and drop for folder organization', async ({ page, cleanDatabase }) => {
    // Navigate to the app
    await page.goto('/');
    
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("StoryWeaver")');
    
    // Create a test project
    const projectName = 'Drag and Drop Test';
    
    // Click the "New Project" button
    await page.getByRole('button', { name: /new project/i }).click();
    
    // Fill in the project form
    await page.getByLabel(/project name/i).fill(projectName);
    
    // Submit the form
    await page.getByRole('button', { name: /create/i }).click();
    
    // Wait for the project to be created
    await page.waitForSelector(`text=${projectName}`);
    
    // Create folders
    const folders = ['Parent Folder', 'Subfolder', 'Another Folder'];
    
    for (const folderName of folders) {
      // Click the "New Folder" button
      await page.getByRole('button', { name: /new folder/i }).click();
      
      // Fill in the folder name
      await page.getByLabel(/folder name/i).fill(folderName);
      
      // Submit the form
      await page.getByRole('button', { name: /create/i }).click();
      
      // Wait for the folder to be created
      await page.waitForSelector(`text=${folderName}`);
    }
    
    // Perform drag and drop to make Subfolder a child of Parent Folder
    // Get the source element (the folder to be moved)
    const sourceFolder = page.locator(`.folder-item:has-text("${folders[1]}")`);
    
    // Get the target element (the parent folder)
    const targetFolder = page.locator(`.folder-item:has-text("${folders[0]}")`);
    
    // Get the bounding boxes
    const sourceBoundingBox = await sourceFolder.boundingBox();
    const targetBoundingBox = await targetFolder.boundingBox();
    
    if (sourceBoundingBox && targetBoundingBox) {
      // Perform the drag and drop operation
      await page.mouse.move(
        sourceBoundingBox.x + sourceBoundingBox.width / 2,
        sourceBoundingBox.y + sourceBoundingBox.height / 2
      );
      await page.mouse.down();
      await page.mouse.move(
        targetBoundingBox.x + targetBoundingBox.width / 2,
        targetBoundingBox.y + targetBoundingBox.height / 2
      );
      await page.mouse.up();
      
      // Wait for the drag operation to complete
      await page.waitForTimeout(500);
      
      // Expand the parent folder if needed
      const expandButton = targetFolder.locator('button[aria-label="Expand"]');
      if (await expandButton.isVisible()) {
        await expandButton.click();
      }
      
      // Verify that Subfolder is now a child of Parent Folder
      await expect(page.locator(`.folder-item:has-text("${folders[0]}") ~ .folder-contents .folder-item:has-text("${folders[1]}")`)).toBeVisible();
    }
  });
});
