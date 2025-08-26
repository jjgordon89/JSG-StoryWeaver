# Comprehensive Accessibility (A11y) Audit Report

## Executive Summary

A comprehensive accessibility audit was conducted on the StoryWeaver application, focusing on three key areas: **Keyboard Traversal**, **ARIA Roles & Labels**, and **Focus Management**. The audit identified significant, systemic issues that impact users relying on assistive technologies.

The most critical findings include a widespread lack of keyboard accessibility for interactive components, improper or missing ARIA attributes, and a complete absence of focus management in modals and dialogs. Many components rely on non-semantic HTML elements (`div`s with `onClick` handlers) for interactivity, which is the root cause of many of the identified issues.

This report provides detailed findings from each of the three audit areas and concludes with high-level, actionable recommendations to address these critical accessibility gaps.

## Key Recommendations

1.  **Prioritize Semantic HTML:** Replace non-semantic elements (e.g., `div` with `onClick`) with their native HTML counterparts (e.g., `<button>`). This provides a significant amount of accessibility for free (keyboard interaction, focusability, and screen reader announcements).
2.  **Adopt a Centralized, Accessible Modal Component:** Refactor all modal/dialog implementations to use a single, robust component that handles focus trapping and restoration. The project already uses Radix UI, whose `Dialog` component (`@radix-ui/react-dialog`) provides this functionality out-of-the-box and is the highly recommended solution.
3.  **Implement Standard Keyboard Interaction Patterns:** Ensure all custom widgets (tabs, menus, tree views) are fully operable with the keyboard, following established ARIA design patterns (e.g., using arrow keys to navigate tabs and menus).
4.  **Provide Keyboard Alternatives:** For features like drag-and-drop, which are inherently mouse-driven, create an equivalent functionality that can be operated entirely with the keyboard (e.g., "Move Up" and "Move Down" buttons).

---

## 1. Keyboard Traversal Audit

This section details interactive elements that cannot be reached or activated using only a keyboard.

### 1.1. Canvas

| Component/File | Location | Issue |
| :--- | :--- | :--- |
| **Canvas Toolbar** | [`src/components/canvas/CanvasToolbar.tsx:108`](src/components/canvas/CanvasToolbar.tsx:108) | The "Add Element" dropdown menu is not keyboard-operable. It cannot be opened or navigated. |
| **Canvas Element** | [`src/components/canvas/CanvasElement.tsx:200`](src/components/canvas/CanvasElement.tsx:200) | Element action buttons ("Edit", "Delete") and resize handles are not individually focusable or operable with a keyboard. |
| **Canvas Export Dialog** | [`src/components/canvas/CanvasExportDialog.tsx:171`](src/components/canvas/CanvasExportDialog.tsx:171) | Export format radio buttons are not selectable with the keyboard. |
| **Outline Template Selector**| [`src/features/story-bible/components/react/OutlineManager.tsx:361`](src/features/story-bible/components/react/OutlineManager.tsx:361)| "Preview" and "Use Template" buttons on template cards are not individually focusable. |

### 1.2. Story Bible

| Component/File | Location | Issue |
| :--- | :--- | :--- |
| **Main View** | [`src/features/story-bible/components/react/StoryBible.tsx:76`](src/features/story-bible/components/react/StoryBible.tsx:76) | The main tabs ("Braindump," "Characters," etc.) are not navigable with arrow keys. |
| **Characters Manager** | [`src/features/story-bible/components/react/CharactersManager.tsx:549`](src/features/story-bible/components/react/CharactersManager.tsx:549)| "Edit" and "Delete" buttons on character trait cards and relationship lists are not focusable. |
| **Scenes Manager** | [`src/features/story-bible/components/react/ScenesManager.tsx:522`](src/features/story-bible/components/react/ScenesManager.tsx:522) | Action buttons ("Validate," "Edit," etc.) on scene cards are not focusable. |
| **Hierarchical Worldbuilding**| [`src/features/story-bible/components/react/HierarchicalWorldbuilding.tsx:294`](src/features/story-bible/components/react/HierarchicalWorldbuilding.tsx:294)| The category tree is not navigable with arrow keys, and its action buttons are not focusable. Drag-and-drop has no keyboard alternative. |
| **Import Modals**| [`src/features/story-bible/components/react/CSVImportDialog.tsx:287`](src/features/story-bible/components/react/CSVImportDialog.tsx:287) | The file dropzone area is not keyboard-operable, and checkboxes in the Smart Import review step are not selectable with the keyboard. |

---

## 2. ARIA Roles & Labels Audit

This section details missing or incorrect ARIA attributes that prevent assistive technologies from understanding component roles and states.

### 2.1. Canvas Components

| Component/File | Issue | Recommendation |
| :--- | :--- | :--- |
| **[`Canvas.tsx`](src/components/canvas/Canvas.tsx:1)** | Main container lacks a role and accessible name. | Add `role="region"` and `aria-label="Canvas"`. |
| **[`CanvasElement.tsx`](src/components/canvas/CanvasElement.tsx:1)** | Interactive `div` lacks a `role`. | Add `role="button"`. |
| **[`CanvasToolbar.tsx`](src/components/canvas/CanvasToolbar.tsx:1)** | Toolbar container lacks a `role`. | Add `role="toolbar"` and `aria-label="Canvas tools"`. |

### 2.2. Story Bible Components

| Component/File | Issue | Recommendation |
| :--- | :--- | :--- |
| **[`StoryBibleView.tsx`](src/features/story-bible/components/StoryBibleView.tsx:1)** | Navigation items are `div`s acting as tabs and lack tab-related roles. | Implement a `role="tablist"` container with `role="tab"` items, and use `aria-selected`. |
| **[`SBCharacterCard.tsx`](src/features/story-bible/components/react/SBCharacterCard.tsx:1)** | Clickable card `div` lacks a `role`. | Add `role="button"`. |
| **[`StoryBibleFloatingMenu.tsx`](src/features/story-bible/components/StoryBibleFloatingMenu.tsx:1)**| Component acts as a menu but lacks menu roles. | Use `role="menu"` for the container and `role="menuitem"` for items. |

### 2.3. Modals & Dialogs

The audit found that most dialogs, like **[`CanvasExportDialog.tsx`](src/components/canvas/CanvasExportDialog.tsx:1)** and **[`ConfirmationDialog.tsx`](src/components/common/ConfirmationDialog.tsx:1)**, correctly implement `role="dialog"`, `aria-modal="true"`, and `aria-labelledby`. This is a positive pattern that should be standardized.

---

## 3. Focus Management Audit

This section details issues with focus trapping and restoration in dynamic components.

### 3.1. Systemic Modal Issues

-   **Issue:** A consistent, application-wide failure to trap focus within modals and restore focus upon modal closure.
-   **Cause:** The application uses multiple custom-built modal implementations that lack focus management logic.
-   **Affected Components:**
    -   [`src/components/AdvancedAI/IdeaDetailModal.tsx`](src/components/AdvancedAI/IdeaDetailModal.tsx)
    -   [`src/components/AdvancedAI/ImageDetailModal.tsx`](src/components/AdvancedAI/ImageDetailModal.tsx)
    -   [`src/components/AdvancedAI/QuickGenerateModal.tsx`](src/components/AdvancedAI/QuickGenerateModal.tsx)
    -   [`src/components/ui/dialog.tsx`](src/components/ui/dialog.tsx)
    -   [`src/components/templates/TemplateApplicationDialog.tsx`](src/components/templates/TemplateApplicationDialog.tsx)
-   **Steps to Reproduce:**
    1.  Open any modal in the application.
    2.  Press the `Tab` key; focus escapes the modal and moves to the underlying page.
    3.  Close the modal; focus is lost to the top of the document, not returned to the triggering element.