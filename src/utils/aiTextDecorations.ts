import * as monaco from 'monaco-editor';

// Interface for tracking AI-generated text ranges
interface AITextRange {
  id: string;
  startLineNumber: number;
  startColumn: number;
  endLineNumber: number;
  endColumn: number;
  timestamp: number;
}

// Class to manage AI text decorations in Monaco Editor
export class AITextDecorationManager {
  private editor: monaco.editor.IStandaloneCodeEditor;
  private aiRanges: Map<string, AITextRange> = new Map();
  private decorationIds: string[] = [];
  private isEnabled: boolean = true;

  constructor(editor: monaco.editor.IStandaloneCodeEditor) {
    this.editor = editor;
    this.setupEventListeners();
  }

  // Add purple highlighting to AI-generated text
  addAITextRange(range: monaco.IRange, id?: string): string {
    if (!this.isEnabled) return '';

    const rangeId = id || `ai-text-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    
    const aiRange: AITextRange = {
      id: rangeId,
      startLineNumber: range.startLineNumber,
      startColumn: range.startColumn,
      endLineNumber: range.endLineNumber,
      endColumn: range.endColumn,
      timestamp: Date.now()
    };

    this.aiRanges.set(rangeId, aiRange);
    this.updateDecorations();
    
    return rangeId;
  }

  // Remove AI text highlighting
  removeAITextRange(id: string): void {
    this.aiRanges.delete(id);
    this.updateDecorations();
  }

  // Clear all AI text highlighting
  clearAllAIText(): void {
    this.aiRanges.clear();
    this.updateDecorations();
  }

  // Enable/disable AI text highlighting
  setEnabled(enabled: boolean): void {
    this.isEnabled = enabled;
    if (!enabled) {
      this.clearDecorations();
    } else {
      this.updateDecorations();
    }
  }

  // Get all AI text ranges
  getAIRanges(): AITextRange[] {
    return Array.from(this.aiRanges.values());
  }

  // Check if a position is within an AI text range
  isPositionInAIText(position: monaco.IPosition): string | null {
    for (const [id, range] of this.aiRanges) {
      if (this.isPositionInRange(position, range)) {
        return id;
      }
    }
    return null;
  }

  // Update decorations in the editor
  private updateDecorations(): void {
    if (!this.isEnabled) {
      this.clearDecorations();
      return;
    }

    const decorations: monaco.editor.IModelDeltaDecoration[] = [];

    for (const range of this.aiRanges.values()) {
      decorations.push({
        range: new monaco.Range(
          range.startLineNumber,
          range.startColumn,
          range.endLineNumber,
          range.endColumn
        ),
        options: {
          className: 'ai-generated-text',
          hoverMessage: {
            value: 'AI-generated content (will be removed when edited)'
          },
          minimap: {
            color: '#9333ea',
            position: monaco.editor.MinimapPosition.Inline
          }
        }
      });
    }

    this.decorationIds = this.editor.deltaDecorations(this.decorationIds, decorations);
  }

  // Clear all decorations
  private clearDecorations(): void {
    this.decorationIds = this.editor.deltaDecorations(this.decorationIds, []);
  }

  // Setup event listeners for automatic removal on edit
  private setupEventListeners(): void {
    this.editor.onDidChangeModelContent((e) => {
      if (!this.isEnabled) return;

      // Check if any AI ranges were modified
      const rangesToRemove: string[] = [];

      for (const change of e.changes) {
        const changeRange = {
          startLineNumber: change.range.startLineNumber,
          startColumn: change.range.startColumn,
          endLineNumber: change.range.endLineNumber,
          endColumn: change.range.endColumn
        };

        // Check if this change overlaps with any AI ranges
        for (const [id, aiRange] of this.aiRanges) {
          if (this.rangesOverlap(changeRange, aiRange)) {
            rangesToRemove.push(id);
          }
        }
      }

      // Remove overlapping AI ranges
      for (const id of rangesToRemove) {
        this.removeAITextRange(id);
      }
    });
  }

  // Check if a position is within a range
  private isPositionInRange(position: monaco.IPosition, range: AITextRange): boolean {
    if (position.lineNumber < range.startLineNumber || position.lineNumber > range.endLineNumber) {
      return false;
    }
    
    if (position.lineNumber === range.startLineNumber && position.column < range.startColumn) {
      return false;
    }
    
    if (position.lineNumber === range.endLineNumber && position.column > range.endColumn) {
      return false;
    }
    
    return true;
  }

  // Check if two ranges overlap
  private rangesOverlap(range1: any, range2: AITextRange): boolean {
    // Check if ranges are completely separate
    if (range1.endLineNumber < range2.startLineNumber || 
        range2.endLineNumber < range1.startLineNumber) {
      return false;
    }

    // Check column overlap for same line ranges
    if (range1.startLineNumber === range1.endLineNumber && 
        range2.startLineNumber === range2.endLineNumber && 
        range1.startLineNumber === range2.startLineNumber) {
      return !(range1.endColumn <= range2.startColumn || range2.endColumn <= range1.startColumn);
    }

    return true;
  }

  // Dispose of the decoration manager
  dispose(): void {
    this.clearDecorations();
    this.aiRanges.clear();
  }
}

// CSS styles for AI-generated text highlighting
export const AI_TEXT_STYLES = `
.ai-generated-text {
  background-color: rgba(147, 51, 234, 0.15) !important;
  border-bottom: 1px solid rgba(147, 51, 234, 0.4) !important;
  color: #9333ea !important;
  transition: all 0.2s ease;
}

.ai-generated-text:hover {
  background-color: rgba(147, 51, 234, 0.25) !important;
  border-bottom: 1px solid rgba(147, 51, 234, 0.6) !important;
}

/* Dark theme support */
.monaco-editor.vs-dark .ai-generated-text {
  background-color: rgba(147, 51, 234, 0.2) !important;
  border-bottom: 1px solid rgba(147, 51, 234, 0.5) !important;
  color: #c084fc !important;
}

.monaco-editor.vs-dark .ai-generated-text:hover {
  background-color: rgba(147, 51, 234, 0.3) !important;
  border-bottom: 1px solid rgba(147, 51, 234, 0.7) !important;
}
`;