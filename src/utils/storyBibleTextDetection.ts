import * as monaco from 'monaco-editor';
import { CharacterTrait, WorldElement, Outline, Scene } from '../types/storyBible';

// Interface for detected Story Bible elements in text
interface DetectedElement {
  id: string;
  type: 'character' | 'world_element' | 'outline' | 'scene';
  name: string;
  elementId: string;
  startLineNumber: number;
  startColumn: number;
  endLineNumber: number;
  endColumn: number;
  confidence: number; // 0-1 confidence score
  description?: string;
}

// Interface for Story Bible data used for detection
interface StoryBibleData {
  characterTraits: CharacterTrait[];
  worldElements: WorldElement[];
  outlines: Outline[];
  scenes: Scene[];
}

// Class to manage Story Bible element detection and highlighting in Monaco Editor
export class StoryBibleTextDetector {
  private editor: monaco.editor.IStandaloneCodeEditor;
  private storyBibleData: StoryBibleData;
  private detectedElements: Map<string, DetectedElement> = new Map();
  private decorationIds: string[] = [];
  private isEnabled: boolean = true;
  private detectionDebouncer: NodeJS.Timeout | null = null;

  constructor(editor: monaco.editor.IStandaloneCodeEditor, storyBibleData: StoryBibleData) {
    this.editor = editor;
    this.storyBibleData = storyBibleData;
    this.setupEventListeners();
    this.performDetection();
  }

  // Update Story Bible data and re-run detection
  updateStoryBibleData(data: StoryBibleData): void {
    this.storyBibleData = data;
    this.performDetection();
  }

  // Enable/disable Story Bible detection
  setEnabled(enabled: boolean): void {
    this.isEnabled = enabled;
    if (!enabled) {
      this.clearDecorations();
    } else {
      this.performDetection();
    }
  }

  // Get all detected elements
  getDetectedElements(): DetectedElement[] {
    return Array.from(this.detectedElements.values());
  }

  // Get detected element at position
  getElementAtPosition(position: monaco.IPosition): DetectedElement | null {
    for (const element of Array.from(this.detectedElements.values())) {
      if (this.isPositionInRange(position, element)) {
        return element;
      }
    }
    return null;
  }

  // Perform text analysis and element detection
  private performDetection(): void {
    if (!this.isEnabled) return;

    const model = this.editor.getModel();
    if (!model) return;

    const text = model.getValue();
    this.detectedElements.clear();

    // Detect character names and traits
    this.detectCharacterElements(text);
    
    // Detect world elements (locations, objects, concepts)
    this.detectWorldElements(text);
    
    // Detect outline references
    this.detectOutlineElements(text);
    
    // Detect scene references
    this.detectSceneElements(text);

    this.updateDecorations();
  }

  // Detect character-related elements in text
  private detectCharacterElements(text: string): void {
    const lines = text.split('\n');
    
    this.storyBibleData.characterTraits.forEach(trait => {
      // Extract character name from trait content or use trait type as identifier
      const searchTerms = this.extractSearchTerms(trait.content, trait.trait_type);
      
      searchTerms.forEach(term => {
        this.findTermInText(term, lines, {
          type: 'character',
          elementId: trait.id,
          name: term,
          description: `Character trait: ${trait.trait_type}`
        });
      });
    });
  }

  // Detect world elements in text
  private detectWorldElements(text: string): void {
    const lines = text.split('\n');
    
    this.storyBibleData.worldElements.forEach(element => {
      const searchTerms = [element.name, ...this.extractSearchTerms(element.description)];
      
      searchTerms.forEach(term => {
        this.findTermInText(term, lines, {
          type: 'world_element',
          elementId: element.id,
          name: element.name,
          description: `${element.element_type}: ${element.description}`
        });
      });
    });
  }

  // Detect outline references in text
  private detectOutlineElements(text: string): void {
    const lines = text.split('\n');
    
    this.storyBibleData.outlines.forEach(outline => {
      const searchTerms = [
        outline.chapter_title,
        ...this.extractSearchTerms(outline.summary)
      ].filter(Boolean) as string[];
      
      searchTerms.forEach(term => {
        this.findTermInText(term, lines, {
          type: 'outline',
          elementId: outline.id,
          name: outline.chapter_title || `Chapter ${outline.chapter_number}`,
          description: `Outline: ${outline.summary}`
        });
      });
    });
  }

  // Detect scene references in text
  private detectSceneElements(text: string): void {
    const lines = text.split('\n');
    
    this.storyBibleData.scenes.forEach(scene => {
      const searchTerms = [
        scene.title,
        scene.setting,
        ...this.extractSearchTerms(scene.summary)
      ].filter(Boolean) as string[];
      
      searchTerms.forEach(term => {
        this.findTermInText(term, lines, {
          type: 'scene',
          elementId: scene.id,
          name: scene.title || `Scene ${scene.scene_number}`,
          description: `Scene: ${scene.summary}`
        });
      });
    });
  }

  // Extract meaningful search terms from text content
  private extractSearchTerms(content: string, additionalTerm?: string): string[] {
    if (!content) return additionalTerm ? [additionalTerm] : [];
    
    // Extract proper nouns and significant terms (simplified approach)
    const words = content.split(/\s+/);
    const terms: string[] = [];
    
    // Add the additional term if provided
    if (additionalTerm) {
      terms.push(additionalTerm);
    }
    
    // Look for capitalized words (potential proper nouns)
    words.forEach(word => {
      const cleanWord = word.replace(/[^a-zA-Z]/g, '');
      if (cleanWord.length > 2 && /^[A-Z][a-z]+$/.test(cleanWord)) {
        terms.push(cleanWord);
      }
    });
    
    // Look for quoted terms
    const quotedTerms = content.match(/"([^"]+)"/g);
    if (quotedTerms) {
      quotedTerms.forEach(quoted => {
        const term = quoted.replace(/"/g, '').trim();
        if (term.length > 2) {
          terms.push(term);
        }
      });
    }
    
    return Array.from(new Set(terms)); // Remove duplicates
  }

  // Find a term in the text and create detection entries
  private findTermInText(
    term: string,
    lines: string[],
    elementInfo: Omit<DetectedElement, 'id' | 'startLineNumber' | 'startColumn' | 'endLineNumber' | 'endColumn' | 'confidence'>
  ): void {
    if (term.length < 3) return; // Skip very short terms
    
    const regex = new RegExp(`\\b${this.escapeRegExp(term)}\\b`, 'gi');
    
    lines.forEach((line, lineIndex) => {
      let match;
      while ((match = regex.exec(line)) !== null) {
        const detectionId = `${elementInfo.type}-${elementInfo.elementId}-${lineIndex}-${match.index}`;
        
        // Calculate confidence based on term length and context
        const confidence = this.calculateConfidence(term, line, match.index);
        
        if (confidence > 0.3) { // Only include matches with reasonable confidence
          const detection: DetectedElement = {
            id: detectionId,
            ...elementInfo,
            startLineNumber: lineIndex + 1,
            startColumn: match.index + 1,
            endLineNumber: lineIndex + 1,
            endColumn: match.index + term.length + 1,
            confidence
          };
          
          this.detectedElements.set(detectionId, detection);
        }
      }
    });
  }

  // Calculate confidence score for a detected term
  private calculateConfidence(term: string, line: string, position: number): number {
    let confidence = 0.5; // Base confidence
    
    // Longer terms are more likely to be significant
    if (term.length > 5) confidence += 0.2;
    if (term.length > 10) confidence += 0.1;
    
    // Capitalized terms in the middle of sentences are more likely to be proper nouns
    if (position > 0 && /^[A-Z]/.test(term)) {
      confidence += 0.2;
    }
    
    // Terms that appear in quotes or special formatting
    const beforeChar = position > 0 ? line[position - 1] : '';
    const afterChar = position + term.length < line.length ? line[position + term.length] : '';
    if (beforeChar === '"' || afterChar === '"' || beforeChar === "'" || afterChar === "'") {
      confidence += 0.3;
    }
    
    return Math.min(confidence, 1.0);
  }

  // Update decorations in the editor
  private updateDecorations(): void {
    if (!this.isEnabled) {
      this.clearDecorations();
      return;
    }

    const decorations: monaco.editor.IModelDeltaDecoration[] = [];

    for (const element of Array.from(this.detectedElements.values())) {
      const className = `story-bible-${element.type}`;
      const color = this.getElementColor(element.type);
      
      decorations.push({
        range: new monaco.Range(
          element.startLineNumber,
          element.startColumn,
          element.endLineNumber,
          element.endColumn
        ),
        options: {
          className,
          hoverMessage: {
            value: `**${element.name}** (${element.type})\n\n${element.description || 'Story Bible element'}\n\n*Confidence: ${Math.round(element.confidence * 100)}%*`
          },
          minimap: {
            color,
            position: monaco.editor.MinimapPosition.Inline
          }
        }
      });
    }

    // Apply decorations
    this.decorationIds = this.editor.deltaDecorations(this.decorationIds, decorations);
  }

  // Get color for different element types
  private getElementColor(type: DetectedElement['type']): string {
    switch (type) {
      case 'character': return '#10b981'; // Green
      case 'world_element': return '#3b82f6'; // Blue
      case 'outline': return '#f59e0b'; // Amber
      case 'scene': return '#8b5cf6'; // Purple
      default: return '#6b7280'; // Gray
    }
  }

  // Clear all decorations
  private clearDecorations(): void {
    this.decorationIds = this.editor.deltaDecorations(this.decorationIds, []);
  }

  // Setup event listeners for automatic re-detection
  private setupEventListeners(): void {
    this.editor.onDidChangeModelContent(() => {
      if (!this.isEnabled) return;
      
      // Debounce detection to avoid excessive processing
      if (this.detectionDebouncer) {
        clearTimeout(this.detectionDebouncer);
      }
      
      this.detectionDebouncer = setTimeout(() => {
        this.performDetection();
      }, 1000); // Re-detect after 1 second of inactivity
    });
  }

  // Utility function to check if position is within range
  private isPositionInRange(position: monaco.IPosition, range: DetectedElement): boolean {
    return (
      position.lineNumber >= range.startLineNumber &&
      position.lineNumber <= range.endLineNumber &&
      (position.lineNumber !== range.startLineNumber || position.column >= range.startColumn) &&
      (position.lineNumber !== range.endLineNumber || position.column <= range.endColumn)
    );
  }

  // Utility function to escape regex special characters
  private escapeRegExp(string: string): string {
    return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }

  // Cleanup
  dispose(): void {
    if (this.detectionDebouncer) {
      clearTimeout(this.detectionDebouncer);
    }
    this.clearDecorations();
  }
}

// CSS styles for Story Bible element highlighting
export const STORY_BIBLE_STYLES = `
/* Character elements */
.story-bible-character {
  background-color: rgba(16, 185, 129, 0.15) !important;
  border-bottom: 1px solid rgba(16, 185, 129, 0.4) !important;
  color: #059669 !important;
  transition: all 0.2s ease;
}

.story-bible-character:hover {
  background-color: rgba(16, 185, 129, 0.25) !important;
  border-bottom: 1px solid rgba(16, 185, 129, 0.6) !important;
}

/* World elements */
.story-bible-world_element {
  background-color: rgba(59, 130, 246, 0.15) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.4) !important;
  color: #2563eb !important;
  transition: all 0.2s ease;
}

.story-bible-world_element:hover {
  background-color: rgba(59, 130, 246, 0.25) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.6) !important;
}

/* Outline elements */
.story-bible-outline {
  background-color: rgba(245, 158, 11, 0.15) !important;
  border-bottom: 1px solid rgba(245, 158, 11, 0.4) !important;
  color: #d97706 !important;
  transition: all 0.2s ease;
}

.story-bible-outline:hover {
  background-color: rgba(245, 158, 11, 0.25) !important;
  border-bottom: 1px solid rgba(245, 158, 11, 0.6) !important;
}

/* Scene elements */
.story-bible-scene {
  background-color: rgba(139, 92, 246, 0.15) !important;
  border-bottom: 1px solid rgba(139, 92, 246, 0.4) !important;
  color: #7c3aed !important;
  transition: all 0.2s ease;
}

.story-bible-scene:hover {
  background-color: rgba(139, 92, 246, 0.25) !important;
  border-bottom: 1px solid rgba(139, 92, 246, 0.6) !important;
}

/* Dark theme support */
.monaco-editor.vs-dark .story-bible-character {
  background-color: rgba(16, 185, 129, 0.2) !important;
  border-bottom: 1px solid rgba(16, 185, 129, 0.5) !important;
  color: #34d399 !important;
}

.monaco-editor.vs-dark .story-bible-character:hover {
  background-color: rgba(16, 185, 129, 0.3) !important;
  border-bottom: 1px solid rgba(16, 185, 129, 0.7) !important;
}

.monaco-editor.vs-dark .story-bible-world_element {
  background-color: rgba(59, 130, 246, 0.2) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.5) !important;
  color: #60a5fa !important;
}

.monaco-editor.vs-dark .story-bible-world_element:hover {
  background-color: rgba(59, 130, 246, 0.3) !important;
  border-bottom: 1px solid rgba(59, 130, 246, 0.7) !important;
}

.monaco-editor.vs-dark .story-bible-outline {
  background-color: rgba(245, 158, 11, 0.2) !important;
  border-bottom: 1px solid rgba(245, 158, 11, 0.5) !important;
  color: #fbbf24 !important;
}

.monaco-editor.vs-dark .story-bible-outline:hover {
  background-color: rgba(245, 158, 11, 0.3) !important;
  border-bottom: 1px solid rgba(245, 158, 11, 0.7) !important;
}

.monaco-editor.vs-dark .story-bible-scene {
  background-color: rgba(139, 92, 246, 0.2) !important;
  border-bottom: 1px solid rgba(139, 92, 246, 0.5) !important;
  color: #a78bfa !important;
}

.monaco-editor.vs-dark .story-bible-scene:hover {
  background-color: rgba(139, 92, 246, 0.3) !important;
  border-bottom: 1px solid rgba(139, 92, 246, 0.7) !important;
}
`;