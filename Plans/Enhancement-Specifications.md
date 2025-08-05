# StoryWeaver Enhancement Specifications

## Overview
This document provides detailed implementation specifications for the five critical areas identified as needing enhancement within the StoryWeaver development plan. Each section includes comprehensive technical details, algorithms, state management patterns, and implementation strategies.

---

## 1. User Interface & Interaction Logic

### Three-Column Layout Logic

#### Responsive Behavior System
```typescript
interface LayoutState {
  leftPanelWidth: number;
  rightPanelWidth: number;
  centerPanelWidth: number;
  isLeftCollapsed: boolean;
  isRightCollapsed: boolean;
  breakpoint: 'mobile' | 'tablet' | 'desktop' | 'ultrawide';
  resizeMode: 'manual' | 'auto' | 'locked';
}

class ResponsiveLayoutManager {
  private state: LayoutState;
  private minPanelWidth = 280;
  private maxPanelWidth = 600;
  private resizeObserver: ResizeObserver;
  
  constructor() {
    this.state = this.getInitialState();
    this.setupResizeObserver();
    this.setupBreakpointListeners();
  }
  
  private getInitialState(): LayoutState {
    const savedState = localStorage.getItem('storyweaver-layout');
    if (savedState) {
      return JSON.parse(savedState);
    }
    
    return {
      leftPanelWidth: 320,
      rightPanelWidth: 380,
      centerPanelWidth: 0, // Calculated dynamically
      isLeftCollapsed: false,
      isRightCollapsed: false,
      breakpoint: this.detectBreakpoint(),
      resizeMode: 'manual'
    };
  }
  
  public handlePanelResize(panel: 'left' | 'right', newWidth: number): void {
    const containerWidth = window.innerWidth;
    const minCenterWidth = 400;
    
    if (panel === 'left') {
      const maxAllowedWidth = containerWidth - this.state.rightPanelWidth - minCenterWidth;
      const clampedWidth = Math.max(this.minPanelWidth, Math.min(newWidth, maxAllowedWidth));
      
      this.setState({
        leftPanelWidth: clampedWidth,
        centerPanelWidth: containerWidth - clampedWidth - this.state.rightPanelWidth
      });
    } else {
      const maxAllowedWidth = containerWidth - this.state.leftPanelWidth - minCenterWidth;
      const clampedWidth = Math.max(this.minPanelWidth, Math.min(newWidth, maxAllowedWidth));
      
      this.setState({
        rightPanelWidth: clampedWidth,
        centerPanelWidth: containerWidth - this.state.leftPanelWidth - clampedWidth
      });
    }
  }
  
  public handleBreakpointChange(breakpoint: LayoutState['breakpoint']): void {
    switch (breakpoint) {
      case 'mobile':
        this.setState({
          isLeftCollapsed: true,
          isRightCollapsed: true,
          resizeMode: 'locked'
        });
        break;
      case 'tablet':
        this.setState({
          isRightCollapsed: true,
          resizeMode: 'auto'
        });
        break;
      case 'desktop':
        this.setState({
          isLeftCollapsed: false,
          isRightCollapsed: false,
          resizeMode: 'manual'
        });
        break;
      case 'ultrawide':
        this.setState({
          leftPanelWidth: 400,
          rightPanelWidth: 450,
          resizeMode: 'manual'
        });
        break;
    }
  }
  
  private detectBreakpoint(): LayoutState['breakpoint'] {
    const width = window.innerWidth;
    if (width < 768) return 'mobile';
    if (width < 1024) return 'tablet';
    if (width < 1920) return 'desktop';
    return 'ultrawide';
  }
}
```

#### Column Resizing Implementation
```typescript
interface ResizeHandle {
  isActive: boolean;
  startX: number;
  startWidth: number;
  panel: 'left' | 'right';
}

class ColumnResizer {
  private handles: Map<string, ResizeHandle> = new Map();
  private layoutManager: ResponsiveLayoutManager;
  
  public initializeHandle(handleId: string, panel: 'left' | 'right'): void {
    const handleElement = document.getElementById(handleId);
    if (!handleElement) return;
    
    handleElement.addEventListener('mousedown', (e) => this.startResize(e, handleId, panel));
    handleElement.addEventListener('touchstart', (e) => this.startResize(e, handleId, panel));
  }
  
  private startResize(event: MouseEvent | TouchEvent, handleId: string, panel: 'left' | 'right'): void {
    event.preventDefault();
    
    const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX;
    const panelElement = document.querySelector(`[data-panel="${panel}"]`) as HTMLElement;
    
    this.handles.set(handleId, {
      isActive: true,
      startX: clientX,
      startWidth: panelElement.offsetWidth,
      panel
    });
    
    document.addEventListener('mousemove', this.handleResize);
    document.addEventListener('touchmove', this.handleResize);
    document.addEventListener('mouseup', () => this.endResize(handleId));
    document.addEventListener('touchend', () => this.endResize(handleId));
    
    document.body.style.cursor = 'col-resize';
    document.body.style.userSelect = 'none';
  }
  
  private handleResize = (event: MouseEvent | TouchEvent): void => {
    const activeHandle = Array.from(this.handles.values()).find(h => h.isActive);
    if (!activeHandle) return;
    
    const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX;
    const deltaX = clientX - activeHandle.startX;
    
    let newWidth: number;
    if (activeHandle.panel === 'left') {
      newWidth = activeHandle.startWidth + deltaX;
    } else {
      newWidth = activeHandle.startWidth - deltaX;
    }
    
    this.layoutManager.handlePanelResize(activeHandle.panel, newWidth);
  };
  
  private endResize(handleId: string): void {
    this.handles.delete(handleId);
    document.removeEventListener('mousemove', this.handleResize);
    document.removeEventListener('touchmove', this.handleResize);
    document.body.style.cursor = '';
    document.body.style.userSelect = '';
  }
}
```

### Selection Menu Intelligence

#### Dynamic Tool Selection Logic
```typescript
interface SelectionContext {
  text: string;
  wordCount: number;
  characterCount: number;
  selectionStart: number;
  selectionEnd: number;
  documentType: 'story' | 'outline' | 'storybible';
  hasStoryBibleData: boolean;
  recentTools: string[];
  userPreferences: ToolPreferences;
}

interface ToolDefinition {
  id: string;
  name: string;
  description: string;
  minWords: number;
  maxWords: number;
  requiresStoryBible: boolean;
  creditCost: number;
  category: 'writing' | 'editing' | 'analysis' | 'creative';
  priority: number;
}

class SelectionMenuIntelligence {
  private tools: Map<string, ToolDefinition> = new Map();
  private contextAnalyzer: ContextAnalyzer;
  
  constructor() {
    this.initializeTools();
    this.contextAnalyzer = new ContextAnalyzer();
  }
  
  public getAvailableTools(context: SelectionContext): ToolDefinition[] {
    const eligibleTools = Array.from(this.tools.values()).filter(tool => 
      this.isToolEligible(tool, context)
    );
    
    // Score tools based on context relevance
    const scoredTools = eligibleTools.map(tool => ({
      tool,
      score: this.calculateToolRelevance(tool, context)
    }));
    
    // Sort by score and return top tools
    return scoredTools
      .sort((a, b) => b.score - a.score)
      .slice(0, this.getMaxToolsForContext(context))
      .map(item => item.tool);
  }
  
  private isToolEligible(tool: ToolDefinition, context: SelectionContext): boolean {
    // Word count constraints
    if (context.wordCount < tool.minWords || context.wordCount > tool.maxWords) {
      return false;
    }
    
    // Story Bible requirement
    if (tool.requiresStoryBible && !context.hasStoryBibleData) {
      return false;
    }
    
    // Document type constraints
    if (tool.id === 'visualize' && context.documentType !== 'story') {
      return false;
    }
    
    return true;
  }
  
  private calculateToolRelevance(tool: ToolDefinition, context: SelectionContext): number {
    let score = tool.priority;
    
    // Boost recently used tools
    if (context.recentTools.includes(tool.id)) {
      score += 10;
    }
    
    // Word count sweet spot
    const wordCountRatio = context.wordCount / (tool.maxWords - tool.minWords);
    if (wordCountRatio >= 0.3 && wordCountRatio <= 0.7) {
      score += 5;
    }
    
    // Context-specific boosts
    if (tool.id === 'related-words' && context.wordCount === 1) {
      score += 20;
    }
    
    if (tool.category === 'editing' && this.contextAnalyzer.needsEditing(context.text)) {
      score += 15;
    }
    
    // User preference adjustments
    score += context.userPreferences.getToolPreference(tool.id) * 5;
    
    return score;
  }
  
  private getMaxToolsForContext(context: SelectionContext): number {
    if (context.wordCount === 1) return 3;
    if (context.wordCount <= 10) return 4;
    if (context.wordCount <= 50) return 5;
    return 6;
  }
  
  private initializeTools(): void {
    this.tools.set('related-words', {
      id: 'related-words',
      name: 'Related Words',
      description: 'Find contextually relevant alternatives',
      minWords: 1,
      maxWords: 1,
      requiresStoryBible: false,
      creditCost: 0,
      category: 'writing',
      priority: 100
    });
    
    this.tools.set('quick-edit', {
      id: 'quick-edit',
      name: 'Quick Edit',
      description: 'Make quick improvements to selected text',
      minWords: 1,
      maxWords: 100,
      requiresStoryBible: false,
      creditCost: 25,
      category: 'editing',
      priority: 90
    });
    
    this.tools.set('describe', {
      id: 'describe',
      name: 'Describe',
      description: 'Add sensory details and descriptions',
      minWords: 2,
      maxWords: 50,
      requiresStoryBible: true,
      creditCost: 50,
      category: 'writing',
      priority: 80
    });
    
    this.tools.set('expand', {
      id: 'expand',
      name: 'Expand',
      description: 'Elaborate and add more detail',
      minWords: 5,
      maxWords: 100,
      requiresStoryBible: true,
      creditCost: 75,
      category: 'writing',
      priority: 75
    });
    
    this.tools.set('rewrite', {
      id: 'rewrite',
      name: 'Rewrite',
      description: 'Rephrase with different styles',
      minWords: 3,
      maxWords: 200,
      requiresStoryBible: false,
      creditCost: 50,
      category: 'editing',
      priority: 85
    });
    
    this.tools.set('visualize', {
      id: 'visualize',
      name: 'Visualize',
      description: 'Generate an image from the description',
      minWords: 10,
      maxWords: 500,
      requiresStoryBible: false,
      creditCost: 2500,
      category: 'creative',
      priority: 60
    });
  }
}
```

### Card Stacking System

#### Visual Organization and Interaction
```typescript
interface AIResponseCard {
  id: string;
  type: 'write' | 'rewrite' | 'expand' | 'describe' | 'brainstorm' | 'visualize';
  content: string;
  metadata: {
    prompt: string;
    model: string;
    creditsUsed: number;
    timestamp: Date;
    wordCount: number;
  };
  isStarred: boolean;
  isCollapsed: boolean;
  stackPosition: number;
  parentStackId?: string;
}

interface CardStack {
  id: string;
  cards: AIResponseCard[];
  isExpanded: boolean;
  stackType: 'chronological' | 'feature' | 'manual';
  title: string;
  totalCredits: number;
}

class CardStackManager {
  private stacks: Map<string, CardStack> = new Map();
  private maxCardsPerStack = 10;
  private autoStackThreshold = 5;
  
  public addCard(card: AIResponseCard): void {
    const existingStack = this.findCompatibleStack(card);
    
    if (existingStack && existingStack.cards.length < this.maxCardsPerStack) {
      this.addToStack(existingStack.id, card);
    } else {
      this.createNewStack(card);
    }
    
    this.checkAutoStacking();
  }
  
  private findCompatibleStack(card: AIResponseCard): CardStack | null {
    for (const stack of this.stacks.values()) {
      if (this.areCardsCompatible(stack.cards[0], card)) {
        return stack;
      }
    }
    return null;
  }
  
  private areCardsCompatible(card1: AIResponseCard, card2: AIResponseCard): boolean {
    // Same feature type
    if (card1.type === card2.type) return true;
    
    // Similar timestamp (within 5 minutes)
    const timeDiff = Math.abs(card1.metadata.timestamp.getTime() - card2.metadata.timestamp.getTime());
    if (timeDiff < 5 * 60 * 1000) return true;
    
    // Related content (similar prompts)
    const promptSimilarity = this.calculatePromptSimilarity(
      card1.metadata.prompt, 
      card2.metadata.prompt
    );
    if (promptSimilarity > 0.7) return true;
    
    return false;
  }
  
  public expandStack(stackId: string): void {
    const stack = this.stacks.get(stackId);
    if (!stack) return;
    
    stack.isExpanded = true;
    this.animateStackExpansion(stackId);
  }
  
  public collapseStack(stackId: string): void {
    const stack = this.stacks.get(stackId);
    if (!stack) return;
    
    stack.isExpanded = false;
    this.animateStackCollapse(stackId);
  }
  
  private animateStackExpansion(stackId: string): void {
    const stackElement = document.querySelector(`[data-stack-id="${stackId}"]`);
    if (!stackElement) return;
    
    const cards = stackElement.querySelectorAll('.ai-response-card');
    cards.forEach((card, index) => {
      (card as HTMLElement).style.transform = `translateY(${index * 10}px)`;
      (card as HTMLElement).style.zIndex = (cards.length - index).toString();
      (card as HTMLElement).style.opacity = '1';
    });
  }
  
  private animateStackCollapse(stackId: string): void {
    const stackElement = document.querySelector(`[data-stack-id="${stackId}"]`);
    if (!stackElement) return;
    
    const cards = stackElement.querySelectorAll('.ai-response-card');
    cards.forEach((card, index) => {
      if (index === 0) return; // Keep top card visible
      
      (card as HTMLElement).style.transform = `translateY(${Math.min(index * 2, 6)}px)`;
      (card as HTMLElement).style.opacity = '0.8';
    });
  }
  
  public reorderStack(stackId: string, cardId: string, newPosition: number): void {
    const stack = this.stacks.get(stackId);
    if (!stack) return;
    
    const cardIndex = stack.cards.findIndex(c => c.id === cardId);
    if (cardIndex === -1) return;
    
    const [card] = stack.cards.splice(cardIndex, 1);
    stack.cards.splice(newPosition, 0, card);
    
    // Update stack positions
    stack.cards.forEach((c, index) => {
      c.stackPosition = index;
    });
    
    this.saveStackState(stackId);
  }
  
  private checkAutoStacking(): void {
    const unStackedCards = this.getUnStackedCards();
    if (unStackedCards.length >= this.autoStackThreshold) {
      this.createAutoStack(unStackedCards);
    }
  }
  
  private createAutoStack(cards: AIResponseCard[]): void {
    const stackId = `auto-${Date.now()}`;
    const stack: CardStack = {
      id: stackId,
      cards: cards.slice(0, this.maxCardsPerStack),
      isExpanded: false,
      stackType: 'chronological',
      title: `Recent Activity (${cards.length} cards)`,
      totalCredits: cards.reduce((sum, card) => sum + card.metadata.creditsUsed, 0)
    };
    
    this.stacks.set(stackId, stack);
  }
}
```

---

## 2. Quick Tools Workflow Logic

### Context Assembly Without Briefing

#### Intelligent Context Builder
```typescript
interface QuickToolsContext {
  selectedText?: string;
  surroundingText: {
    before: string;
    after: string;
  };
  documentContext: {
    title: string;
    genre?: string;
    wordCount: number;
    recentChanges: string[];
  };
  storyBibleElements: {
    relevantCharacters: Character[];
    relevantWorldbuilding: WorldElement[];
    styleNotes: string[];
  };
  sessionHistory: {
    recentEdits: QuickEdit[];
    conversationContext: string[];
  };
}

class QuickToolsContextAssembler {
  private saliencyEngine: SaliencyEngine;
  private documentAnalyzer: DocumentAnalyzer;
  private maxContextTokens = 4000;
  
  public async assembleContext(
    documentId: number,
    selectionRange?: { start: number; end: number },
    toolType: 'edit' | 'chat'
  ): Promise<QuickToolsContext> {
    const document = await this.getDocument(documentId);
    const storyBible = await this.getStoryBible(document.projectId);
    
    // Get surrounding text context
    const surroundingText = this.extractSurroundingText(
      document.content,
      selectionRange,
      1000 // words before and after
    );
    
    // Get relevant Story Bible elements using saliency
    const contextText = selectionRange 
      ? document.content.substring(selectionRange.start, selectionRange.end)
      : surroundingText.before + surroundingText.after;
    
    const relevantElements = await this.saliencyEngine.selectRelevantElements(
      contextText,
      storyBible,
      this.maxContextTokens * 0.4 // 40% of context budget for Story Bible
    );
    
    // Get recent document changes for continuity
    const recentChanges = await this.getRecentChanges(documentId, 5);
    
    // Get session history for Quick Tools
    const sessionHistory = await this.getQuickToolsHistory(documentId, 10);
    
    return {
      selectedText: selectionRange 
        ? document.content.substring(selectionRange.start, selectionRange.end)
        : undefined,
      surroundingText,
      documentContext: {
        title: document.name,
        genre: storyBible.genre,
        wordCount: document.wordCount,
        recentChanges: recentChanges.map(c => c.description)
      },
      storyBibleElements: {
        relevantCharacters: relevantElements.characters,
        relevantWorldbuilding: relevantElements.worldbuilding,
        styleNotes: storyBible.styleExamples ? [storyBible.styleExamples] : []
      },
      sessionHistory: {
        recentEdits: sessionHistory.edits,
        conversationContext: sessionHistory.conversations
      }
    };
  }
  
  private extractSurroundingText(
    content: string,
    selectionRange?: { start: number; end: number },
    maxWords: number
  ): { before: string; after: string } {
    if (!selectionRange) {
      // If no selection, get context around cursor or end of document
      const words = content.split(/\s+/);
      const contextWords = words.slice(-maxWords);
      return {
        before: contextWords.join(' '),
        after: ''
      };
    }
    
    const beforeText = content.substring(0, selectionRange.start);
    const afterText = content.substring(selectionRange.end);
    
    const beforeWords = beforeText.split(/\s+/).slice(-maxWords);
    const afterWords = afterText.split(/\s+/).slice(0, maxWords);
    
    return {
      before: beforeWords.join(' '),
      after: afterWords.join(' ')
    };
  }
  
  public buildPromptContext(context: QuickToolsContext, toolType: 'edit' | 'chat'): string {
    let prompt = '';
    
    // Document context
    if (context.documentContext.genre) {
      prompt += `Genre: ${context.documentContext.genre}\n`;
    }
    
    // Story Bible context (most relevant elements)
    if (context.storyBibleElements.relevantCharacters.length > 0) {
      prompt += '\nRelevant Characters:\n';
      context.storyBibleElements.relevantCharacters.forEach(char => {
        prompt += `- ${char.name}: ${char.description}\n`;
      });
    }
    
    if (context.storyBibleElements.relevantWorldbuilding.length > 0) {
      prompt += '\nRelevant World Elements:\n';
      context.storyBibleElements.relevantWorldbuilding.forEach(element => {
        prompt += `- ${element.name}: ${element.description}\n`;
      });
    }
    
    // Style context
    if (context.storyBibleElements.styleNotes.length > 0) {
      prompt += `\nStyle Notes: ${context.storyBibleElements.styleNotes[0]}\n`;
    }
    
    // Surrounding text for context
    if (context.surroundingText.before) {
      prompt += `\nPreceding text: ...${context.surroundingText.before}\n`;
    }
    
    if (context.selectedText) {
      prompt += `\nSelected text: "${context.selectedText}"\n`;
    }
    
    if (context.surroundingText.after) {
      prompt += `\nFollowing text: ${context.surroundingText.after}...\n`;
    }
    
    // Recent context for continuity
    if (context.sessionHistory.recentEdits.length > 0 && toolType === 'edit') {
      prompt += '\nRecent edits in this session:\n';
      context.sessionHistory.recentEdits.slice(-3).forEach(edit => {
        prompt += `- ${edit.description}\n`;
      });
    }
    
    return prompt;
  }
}
```

### High Quality Mode Toggle Logic

#### Decision Engine for Quality Mode
```typescript
interface QualityModeDecision {
  shouldSuggest: boolean;
  shouldRequire: boolean;
  reason: string;
  confidenceScore: number;
}

class HighQualityModeDecisionEngine {
  private complexityAnalyzer: TextComplexityAnalyzer;
  private creditManager: CreditManager;
  
  public async analyzeRequest(
    context: QuickToolsContext,
    userInput: string,
    toolType: 'edit' | 'chat'
  ): Promise<QualityModeDecision> {
    const factors = await this.analyzeQualityFactors(context, userInput, toolType);
    
    let shouldSuggest = false;
    let shouldRequire = false;
    let reason = '';
    let confidenceScore = 0;
    
    // Complex editing requests
    if (factors.textComplexity > 0.7) {
      shouldSuggest = true;
      reason = 'Complex text detected - High Quality mode will provide better results';
      confidenceScore += 0.3;
    }
    
    // Long selections
    if (factors.selectionLength > 100) {
      shouldSuggest = true;
      reason = 'Long text selection - High Quality mode recommended for comprehensive editing';
      confidenceScore += 0.2;
    }
    
    // Creative writing requests
    if (factors.isCreativeRequest) {
      shouldSuggest = true;
      reason = 'Creative writing request - High Quality mode provides more nuanced results';
      confidenceScore += 0.25;
    }
    
    // Story Bible complexity
    if (factors.storyBibleComplexity > 0.8) {
      shouldRequire = true;
      reason = 'Complex Story Bible data requires High Quality mode for accurate context understanding';
      confidenceScore += 0.4;
    }
    
    // User has sufficient credits
    if (factors.hasEnoughCredits) {
      confidenceScore += 0.1;
    } else {
      // Reduce suggestion if low on credits
      confidenceScore -= 0.2;
      if (shouldRequire) {
        shouldRequire = false;
        shouldSuggest = true;
        reason += ' (Note: High Quality mode uses more credits)';
      }
    }
    
    // Previous session quality
    if (factors.previousSessionUsedHQ && factors.previousSessionSatisfaction > 0.8) {
      shouldSuggest = true;
      confidenceScore += 0.15;
    }
    
    return {
      shouldSuggest: shouldSuggest || shouldRequire,
      shouldRequire,
      reason,
      confidenceScore: Math.min(confidenceScore, 1.0)
    };
  }
  
  private async analyzeQualityFactors(
    context: QuickToolsContext,
    userInput: string,
    toolType: 'edit' | 'chat'
  ): Promise<QualityFactors> {
    const textComplexity = this.complexityAnalyzer.analyze(
      context.selectedText || context.surroundingText.before
    );
    
    const selectionLength = context.selectedText?.split(/\s+/).length || 0;
    
    const isCreativeRequest = this.detectCreativeRequest(userInput);
    
    const storyBibleComplexity = this.calculateStoryBibleComplexity(
      context.storyBibleElements
    );
    
    const hasEnoughCredits = await this.creditManager.hasEnoughCredits(
      toolType === 'edit' ? 100 : 150 // HQ mode costs
    );
    
    const sessionHistory = await this.getSessionQualityHistory();
    
    return {
      textComplexity,
      selectionLength,
      isCreativeRequest,
      storyBibleComplexity,
      hasEnoughCredits,
      previousSessionUsedHQ: sessionHistory.usedHighQuality,
      previousSessionSatisfaction: sessionHistory.satisfactionScore
    };
  }
  
  private detectCreativeRequest(userInput: string): boolean {
    const creativeKeywords = [
      'creative', 'imaginative', 'artistic', 'poetic', 'lyrical',
      'atmospheric', 'evocative', 'vivid', 'compelling', 'engaging',
      'dramatic', 'emotional', 'powerful', 'beautiful', 'elegant'
    ];
    
    const lowerInput = userInput.toLowerCase();
    return creativeKeywords.some(keyword => lowerInput.includes(keyword));
  }
  
  private calculateStoryBibleComplexity(elements: QuickToolsContext['storyBibleElements']): number {
    let complexity = 0;
    
    // Character complexity
    complexity += elements.relevantCharacters.length * 0.1;
    elements.relevantCharacters.forEach(char => {
      complexity += Object.keys(char.traits).length * 0.05;
    });
    
    // Worldbuilding complexity
    complexity += elements.relevantWorldbuilding.length * 0.1;
    elements.relevantWorldbuilding.forEach(element => {
      complexity += Object.keys(element.properties).length * 0.05;
    });
    
    // Style notes complexity
    if (elements.styleNotes.length > 0) {
      const avgStyleLength = elements.styleNotes.reduce((sum, note) => 
        sum + note.split(/\s+/).length, 0) / elements.styleNotes.length;
      complexity += Math.min(avgStyleLength / 100, 0.3);
    }
    
    return Math.min(complexity, 1.0);
  }
}
```

### Inline Editing Flow State Management

#### Struck-through/Green Text Replacement System
```typescript
interface EditingState {
  originalText: string;
  suggestedText: string;
  isAccepted: boolean;
  isRejected: boolean;
  editId: string;
  timestamp: Date;
  range: { start: number; end: number };
}

interface DocumentEditSession {
  documentId: number;
  activeEdits: Map<string, EditingState>;
  editHistory: EditingState[];
  canUndo: boolean;
  canRedo: boolean;
}

class InlineEditingManager {
  private sessions: Map<number, DocumentEditSession> = new Map();
  private editorInstance: monaco.editor.IStandaloneCodeEditor;
  private decorationIds: string[] = [];
  
  public startInlineEdit(
    documentId: number,
    range: { start: number; end: number },
    originalText: string,
    suggestedText: string
  ): string {
    const editId = `edit_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    
    const session = this.getOrCreateSession(documentId);
    
    const editState: EditingState = {
      originalText,
      suggestedText,
      isAccepted: false,
      isRejected: false,
      editId,
      timestamp: new Date(),
      range
    };
    
    session.activeEdits.set(editId, editState);
    
    this.applyInlineDecoration(editState);
    this.setupEditControls(editState);
    
    return editId;
  }
  
  private applyInlineDecoration(editState: EditingState): void {
    const model = this.editorInstance.getModel();
    if (!model) return;
    
    const startPos = model.getPositionAt(editState.range.start);
    const endPos = model.getPositionAt(editState.range.end);
    
    // Create struck-through decoration for original text
    const struckThroughDecoration: monaco.editor.IModelDeltaDecoration = {
      range: new monaco.Range(startPos.lineNumber, startPos.column, endPos.lineNumber, endPos.column),
      options: {
        className: 'inline-edit-original',
        hoverMessage: { value: 'Original text - click to revert' },
        stickiness: monaco.editor.TrackedRangeStickiness.NeverGrowsWhenTypingAtEdges
      }
    };
    
    // Insert suggested text with green highlighting
    const suggestedRange = this.insertSuggestedText(editState);
    const greenDecoration: monaco.editor.IModelDeltaDecoration = {
      range: suggestedRange,
      options: {
        className: 'inline-edit-suggested',
        hoverMessage: { value: 'Suggested text - click to accept' },
        stickiness: monaco.editor.TrackedRangeStickiness.NeverGrowsWhenTypingAtEdges
      }
    };
    
    const decorationIds = this.editorInstance.deltaDecorations([], [
      struckThroughDecoration,
      greenDecoration
    ]);
    
    this.decorationIds.push(...decorationIds);
  }
  
  private insertSuggestedText(editState: EditingState): monaco.Range {
    const model = this.editorInstance.getModel();
    if (!model) throw new Error('No model available');
    
    const endPos = model.getPositionAt(editState.range.end);
    
    // Insert suggested text after original text
    const insertText = `\n${editState.suggestedText}`;
    const insertOperation: monaco.editor.IIdentifiedSingleEditOperation = {
      range: new monaco.Range(endPos.lineNumber, endPos.column, endPos.lineNumber, endPos.column),
      text: insertText
    };
    
    model.pushEditOperations([], [insertOperation], () => null);
    
    // Calculate range of inserted text
    const newEndPos = model.getPositionAt(editState.range.end + insertText.length);
    return new monaco.Range(
      endPos.lineNumber + 1, 1,
      newEndPos.lineNumber, newEndPos.column
    );
  }
  
  private setupEditControls(editState: EditingState): void {
    // Create floating action buttons
    const controlsContainer = document.createElement('div');
    controlsContainer.className = 'inline-edit-controls';
    controlsContainer.innerHTML = `
      <button class="accept-btn" data-edit-id="${editState.editId}">
        <svg><!-- Accept icon --></svg>
        Accept
      </button>
      <button class="reject-btn" data-edit-id="${editState.editId}">
        <svg><!-- Reject icon --></svg>
        Reject
      </button>
      <button class="modify-btn" data-edit-id="${editState.editId}">
        <svg><!-- Edit icon --></svg>
        Modify
      </button>
    `;
    
    // Position controls near the edit
    this.positionControls(controlsContainer, editState.range);
    
    // Add event listeners
    controlsContainer.addEventListener('click', (e) => {
      const target = e.target as HTMLElement;
      const button = target.closest('button');
      if (!button) return;
      
      const editId = button.getAttribute('data-edit-id');
      if (!editId) return;
      
      if (button.classList.contains('accept-btn')) {
        this.acceptEdit(editId);
      } else if (button.classList.contains('reject-btn')) {
        this.rejectEdit(editId);
      } else if (button.classList.contains('modify-btn')) {
        this.modifyEdit(editId);
      }
    });
    
    document.body.appendChild(controlsContainer);
  }
  
  public acceptEdit(editId: string): void {
    const session = this.findSessionByEditId(editId);
    if (!session) return;
    
    const editState = session.activeEdits.get(editId);
    if (!editState) return;
    
    // Replace original text with suggested text
    const model = this.editorInstance.getModel();
    if (!model) return;
    
    const startPos = model.getPositionAt(editState.range.start);
    const endPos = model.getPositionAt(editState.range.end);
    
    const replaceOperation: monaco.editor.IIdentifiedSingleEditOperation = {
      range: new monaco.Range(startPos.lineNumber, startPos.column, endPos.lineNumber, endPos.column),
      text: editState.suggestedText
    };
    
    model.pushEditOperations([], [replaceOperation], () => null);
    
    // Update state
    editState.isAccepted = true;
    session.editHistory.push(editState);
    session.activeEdits.delete(editId);
    
    this.cleanupEditDecorations(editId);
    this.updateUndoRedoState(session);
  }
  
  public rejectEdit(editId: string): void {
    const session = this.findSessionByEditId(editId);
    if (!session) return;
    
    const editState = session.activeEdits.get(editId);
    if (!editState) return;
    
    // Remove suggested text, keep original
    editState.isRejected = true;
    session.editHistory.push(editState);
    session.activeEdits.delete(editId);
    
    this.cleanupEditDecorations(editId);
    this.updateUndoRedoState(session);
  }
  
  public undoLastEdit(documentId: number): void {
    const session = this.sessions.get(documentId);
    if (!session || session.editHistory.length === 0) return;
    
    const lastEdit = session.editHistory.pop();
    if (!lastEdit) return;
    
    if (lastEdit.isAccepted) {
      // Revert accepted edit
      const model = this.editorInstance.getModel();
      if (!model) return;
      
      // Find current position of the edit and revert
      const currentText = model.getValue();
      const newText = currentText.replace(lastEdit.suggestedText, lastEdit.originalText);
      model.setValue(newText);
    }
    
    this.updateUndoRedoState(session);
  }
  
  private cleanupEditDecorations(editId: string): void {
    // Remove decorations and controls for this edit
    const controlsElement = document.querySelector(`[data-edit-id="${editId}"]`)?.closest('.inline-edit-controls');
    if (controlsElement) {
      controlsElement.remove();
    }
    
    // Clear Monaco decorations
    this.editorInstance.deltaDecorations(this.decorationIds, []);
    this.decorationIds = [];
  }
  
  private updateUndoRedoState(session: DocumentEditSession): void {
    session.canUndo = session.editHistory.length > 0;
    session.canRedo = false; // Simplified - would need redo stack for full implementation
    
    // Emit state change event
    this.emitStateChange(session.documentId, {
      canUndo: session.canUndo,
      canRedo: session.canRedo,
      activeEditsCount: session.activeEdits.size
    });
  }
}
```

---

## 3. AI Context Management

### Saliency Engine Logic

#### Advanced Relevance Algorithms
```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::Arc;

#[derive(Debug, Clone)]
pub struct SaliencyEngine {
    embedding_service: Arc<EmbeddingService>,
    relevance_calculator: RelevanceCalculator,
    context_optimizer: ContextOptimizer,
    cache: Arc<tokio::sync::RwLock<HashMap<String, CachedRelevance>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevanceScore {
    pub element_id: String,
    pub element_type: ElementType,
    pub base_score: f32,
    pub context_boost: f32,
    pub recency_boost: f32,
    pub user_preference_boost: f32,
    pub final_score: f32,
    pub reasoning: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContextWindow {
    pub max_tokens: usize,
    pub reserved_tokens: usize,
    pub available_tokens: usize,
    pub priority_allocation: HashMap<ElementType, f32>,
}

impl SaliencyEngine {
    pub async fn select_optimal_context(
        &self,
        current_text: &str,
        story_bible: &StoryBible,
        context_window: &ContextWindow,
        user_preferences: &UserPreferences,
    ) -> Result<OptimizedContext> {
        // Generate embeddings for current context
        let context_embedding = self.embedding_service
            .generate_embedding(current_text)
            .await?;
        
        // Score all Story Bible elements
        let mut scored_elements = Vec::new();
        
        // Score characters with detailed analysis
        for character in &story_bible.characters {
            if !character.is_visible { continue; }
            
            let score = self.calculate_character_relevance(
                &context_embedding,
                character,
                current_text,
                user_preferences,
            ).await?;
            
            scored_elements.push((
                StoryBibleElement::Character(character.clone()),
                score,
            ));
        }
        
        // Score worldbuilding elements
        for element in &story_bible.worldbuilding {
            if !element.is_visible { continue; }
            
            let score = self.calculate_worldbuilding_relevance(
                &context_embedding,
                element,
                current_text,
                user_preferences,
            ).await?;
            
            scored_elements.push((
                StoryBibleElement::Worldbuilding(element.clone()),
                score,
            ));
        }
        
        // Score other elements (plot points, themes, etc.)
        self.score_additional_elements(&mut scored_elements, &context_embedding, story_bible).await?;
        
        // Optimize selection based on token budget and relevance
        let optimized_selection = self.context_optimizer
            .optimize_selection(scored_elements, context_window)
            .await?;
        
        Ok(optimized_selection)
    }
    
    async fn calculate_character_relevance(
        &self,
        context_embedding: &Vec<f32>,
        character: &Character,
        current_text: &str,
        user_preferences: &UserPreferences,
    ) -> Result<RelevanceScore> {
        let mut reasoning = Vec::new();
        let mut base_score = 0.0f32;
        let mut context_boost = 0.0f32;
        let mut recency_boost = 0.0f32;
        
        // 1. Direct name mentions (high weight)
        let name_mentions = self.count_name_mentions(&character.name, current_text);
        if name_mentions > 0 {
            base_score += (name_mentions as f32 * 0.4).min(1.0);
            reasoning.push(format!("Character name mentioned {} times", name_mentions));
        }
        
        // 2. Semantic similarity of character description
        if let Some(description) = &character.description {
            let char_embedding = self.embedding_service
                .generate_embedding(description)
                .await?;
            
            let similarity = self.calculate_cosine_similarity(context_embedding, &char_embedding);
            base_score += similarity * 0.3;
            
            if similarity > 0.7 {
                reasoning.push("High semantic similarity with character description".to_string());
            }
        }
        
        // 3. Trait relevance analysis
        let trait_relevance = self.analyze_trait_relevance(character, current_text).await?;
        base_score += trait_relevance.score * 0.2;
        reasoning.extend(trait_relevance.explanations);
        
        // 4. Relationship network analysis
        let relationship_boost = self.analyze_character_relationships(character, current_text).await?;
        context_boost += relationship_boost * 0.15;
        
        // 5. Recent usage patterns
        let recent_usage = self.get_recent_character_usage(character.id).await?;
        recency_boost = self.calculate_recency_boost(recent_usage);
        
        // 6. User preference adjustments
        let user_preference_boost = user_preferences.get_character_preference(character.id) * 0.1;
        
        // 7. Context-specific boosts
        context_boost += self.calculate_context_specific_boosts(character, current_text).await?;
        
        let final_score = (base_score + context_boost + recency_boost + user_preference_boost).min(1.0);
        
        Ok(RelevanceScore {
            element_id: character.id.to_string(),
            element_type: ElementType::Character,
            base_score,
            context_boost,
            recency_boost,
            user_preference_boost,
            final_score,
            reasoning,
        })
    }
    
    async fn analyze_trait_relevance(
        &self,
        character: &Character,
        current_text: &str,
    ) -> Result<TraitRelevanceAnalysis> {
        let mut total_score = 0.0f32;
        let mut explanations = Vec::new();
        
        for (trait_name, trait_value) in &character.traits {
            // Check for direct trait mentions
            let trait_mentions = current_text.to_lowercase()
                .matches(&trait_name.to_lowercase())
                .count();
            
            if trait_mentions > 0 {
                total_score += 0.2;
                explanations.push(format!("Trait '{}' mentioned in context", trait_name));
            }
            
            // Semantic analysis of trait relevance
            let trait_text = format!("{}: {}", trait_name, trait_value);
            let trait_embedding = self.embedding_service
                .generate_embedding(&trait_text)
                .await?;
            
            let context_embedding = self.embedding_service
                .generate_embedding(current_text)
                .await?;
            
            let similarity = self.calculate_cosine_similarity(&context_embedding, &trait_embedding);
            if similarity > 0.6 {
                total_score += similarity * 0.3;
                explanations.push(format!("Trait '{}' semantically relevant", trait_name));
            }
        }
        
        Ok(TraitRelevanceAnalysis {
            score: total_score.min(1.0),
            explanations,
        })
    }
    
    async fn calculate_context_specific_boosts(
        &self,
        character: &Character,
        current_text: &str,
    ) -> Result<f32> {
        let mut boost = 0.0f32;
        
        // POV character boost
        if self.is_pov_character(character, current_text).await? {
            boost += 0.3;
        }
        
        // Dialogue context boost
        if self.has_recent_dialogue(character, current_text).await? {
            boost += 0.2;
        }
        
        // Action/scene participation boost
        if self.is_in_current_scene(character, current_text).await? {
            boost += 0.25;
        }
        
        // Emotional state relevance
        if self.has_relevant_emotional_state(character, current_text).await? {
            boost += 0.15;
        }
        
        Ok(boost)
    }
    
    fn count_name_mentions(&self, name: &str, text: &str) -> usize {
        let name_lower = name.to_lowercase();
        let text_lower = text.to_lowercase();
        
        // Count exact matches
        let exact_matches = text_lower.matches(&name_lower).count();
        
        // Count partial matches (first name, last name, nicknames)
        let name_parts: Vec<&str> = name_lower.split_whitespace().collect();
        let partial_matches = name_parts.iter()
            .map(|part| text_lower.matches(part).count())
            .sum::<usize>();
        
        exact_matches + (partial_matches / 2) // Weight partial matches less
    }
    
    fn calculate_cosine_similarity(&self, vec1: &Vec<f32>, vec2: &Vec<f32>) -> f32 {
        if vec1.len() != vec2.len() {
            return 0.0;
        }
        
        let dot_product: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let magnitude1: f32 = vec1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude2: f32 = vec2.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if magnitude1 == 0.0 || magnitude2 == 0.0 {
            return 0.0;
        }
        
        dot_product / (magnitude1 * magnitude2)
    }
}

#[derive(Debug)]
pub struct ContextOptimizer {
    token_estimator: TokenEstimator,
}

impl ContextOptimizer {
    pub async fn optimize_selection(
        &self,
        mut scored_elements: Vec<(StoryBibleElement, RelevanceScore)>,
        context_window: &ContextWindow,
    ) -> Result<OptimizedContext> {
        // Sort by relevance score (descending)
        scored_elements.sort_by(|a, b| b.1.final_score.partial_cmp(&a.1.final_score).unwrap());
        
        let mut selected_elements = Vec::new();
        let mut used_tokens = context_window.reserved_tokens;
        let available_tokens = context_window.available_tokens;
        
        // Priority-based selection
        let mut character_tokens_used = 0;
        let mut worldbuilding_tokens_used = 0;
        let max_character_tokens = (available_tokens as f32 * 
            context_window.priority_allocation.get(&ElementType::Character).unwrap_or(&0.6)) as usize;
        let max_worldbuilding_tokens = (available_tokens as f32 * 
            context_window.priority_allocation.get(&ElementType::Worldbuilding).unwrap_or(&0.4)) as usize;
        
        for (element, score) in scored_elements {
            let element_tokens = self.token_estimator.estimate_tokens(&element);
            
            let can_fit = match element {
                StoryBibleElement::Character(_) => {
                    character_tokens_used + element_tokens <= max_character_tokens
                },
                StoryBibleElement::Worldbuilding(_) => {
                    worldbuilding_tokens_used + element_tokens <= max_worldbuilding_tokens
                },
                _ => used_tokens + element_tokens <= available_tokens,
            };
            
            if can_fit && score.final_score > 0.1 { // Minimum relevance threshold
                match element {
                    StoryBibleElement::Character(_) => character_tokens_used += element_tokens,
                    StoryBibleElement::Worldbuilding(_) => worldbuilding_tokens_used += element_tokens,
                    _ => {},
                }
                
                used_tokens += element_tokens;
                selected_elements.push(OptimizedElement {
                    element,
                    relevance_score: score,
                    token_cost: element_tokens,
                });
            }
            
            if used_tokens >= available_tokens {
                break;
            }
        }
        
        Ok(OptimizedContext {
            elements: selected_elements,
            total_tokens_used: used_tokens,
            optimization_stats: OptimizationStats {
                total_candidates: scored_elements.len(),
                selected_count: selected_elements.len(),
                token_utilization: used_tokens as f32 / available_tokens as f32,
                average_relevance: selected_elements.iter()
                    .map(|e| e.relevance_score.final_score)
                    .sum::<f32>() / selected_elements.len() as f32,
            },
        })
    }
}
```

### Token Management System

#### Intelligent Token Optimization
```rust
#[derive(Debug, Clone)]
pub struct TokenManager {
    model_configs: HashMap<String, ModelTokenConfig>,
    usage_tracker: Arc<tokio::sync::RwLock<TokenUsageTracker>>,
    optimization_strategies: Vec<Box<dyn OptimizationStrategy + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub struct ModelTokenConfig {
    pub model_name: String,
    pub context_window: usize,
    pub max_output_tokens: usize,
    pub cost_per_input_token: f64,
    pub cost_per_output_token: f64,
    pub supports_function_calling: bool,
    pub optimal_context_ratio: f32, // Ratio of context to leave for output
}

#[derive(Debug)]
pub struct TokenBudget {
    pub total_available: usize,
    pub reserved_for_output: usize,
    pub system_prompt: usize,
    pub user_prompt: usize,
    pub story_bible_context: usize,
    pub document_context: usize,
    pub conversation_history: usize,
    pub safety_buffer: usize,
}

impl TokenManager {
    pub async fn optimize_context_for_model(
        &self,
        model_name: &str,
        context_data: &ContextData,
        expected_output_length: Option<usize>,
    ) -> Result<OptimizedTokenContext> {
        let model_config = self.model_configs.get(model_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown model: {}", model_name))?;
        
        // Calculate token budget
        let budget = self.calculate_token_budget(model_config, expected_output_length);
        
        // Apply optimization strategies
        let mut optimized_context = context_data.clone();
        
        for strategy in &self.optimization_strategies {
            optimized_context = strategy.optimize(optimized_context, &budget).await?;
            
            let current_tokens = self.estimate_total_tokens(&optimized_context);
            if current_tokens <= budget.total_available - budget.safety_buffer {
                break;
            }
        }
        
        // Final validation and adjustment
        let final_tokens = self.estimate_total_tokens(&optimized_context);
        if final_tokens > budget.total_available - budget.safety_buffer {
            optimized_context = self.apply_emergency_truncation(optimized_context, &budget).await?;
        }
        
        // Track usage
        self.track_token_usage(model_name, &optimized_context).await?;
        
        Ok(OptimizedTokenContext {
            context: optimized_context,
            token_usage: self.calculate_detailed_usage(&optimized_context),
            optimization_applied: self.get_applied_optimizations(),
            estimated_cost: self.calculate_estimated_cost(model_config, &optimized_context),
        })
    }
    
    fn calculate_token_budget(
        &self,
        model_config: &ModelTokenConfig,
        expected_output_length: Option<usize>,
    ) -> TokenBudget {
        let total_available = model_config.context_window;
        let reserved_for_output = expected_output_length
            .unwrap_or((total_available as f32 * model_config.optimal_context_ratio) as usize)
            .min(model_config.max_output_tokens);
        
        let available_for_input = total_available - reserved_for_output;
        let safety_buffer = (available_for_input as f32 * 0.05) as usize; // 5% safety buffer
        
        TokenBudget {
            total_available,
            reserved_for_output,
            system_prompt: (available_for_input as f32 * 0.1) as usize,
            user_prompt: (available_for_input as f32 * 0.15) as usize,
            story_bible_context: (available_for_input as f32 * 0.4) as usize,
            document_context: (available_for_input as f32 * 0.25) as usize,
            conversation_history: (available_for_input as f32 * 0.05) as usize,
            safety_buffer,
        }
    }
    
    async fn apply_emergency_truncation(
        &self,
        mut context: ContextData,
        budget: &TokenBudget,
    ) -> Result<ContextData> {
        // Priority-based truncation
        let target_tokens = budget.total_available - budget.safety_buffer;
        let mut current_tokens = self.estimate_total_tokens(&context);
        
        // 1. Truncate conversation history first (lowest priority)
        if current_tokens > target_tokens {
            context.conversation_history = self.truncate_conversation_history(
                context.conversation_history,
                budget.conversation_history,
            );
            current_tokens = self.estimate_total_tokens(&context);
        }
        
        // 2. Reduce document context
        if current_tokens > target_tokens {
            context.document_context = self.truncate_document_context(
                context.document_context,
                budget.document_context,
            );
            current_tokens = self.estimate_total_tokens(&context);
        }
        
        // 3. Optimize Story Bible context (keep highest relevance)
        if current_tokens > target_tokens {
            context.story_bible_context = self.optimize_story_bible_context(
                context.story_bible_context,
                budget.story_bible_context,
            ).await?;
            current_tokens = self.estimate_total_tokens(&context);
        }
        
        // 4. Final user prompt truncation (last resort)
        if current_tokens > target_tokens {
            context.user_prompt = self.truncate_user_prompt(
                context.user_prompt,
                budget.user_prompt,
            );
        }
        
        Ok(context)
    }
    
    async fn optimize_story_bible_context(
        &self,
        story_bible_context: StoryBibleContext,
        max_tokens: usize,
    ) -> Result<StoryBibleContext> {
        let mut optimized = story_bible_context.clone();
        let mut current_tokens = self.estimate_story_bible_tokens(&optimized);
        
        if current_tokens <= max_tokens {
            return Ok(optimized);
        }
        
        // Remove lowest relevance elements first
        optimized.characters.sort_by(|a, b| 
            b.relevance_score.partial_cmp(&a.relevance_score).unwrap()
        );
        optimized.worldbuilding.sort_by(|a, b| 
            b.relevance_score.partial_cmp(&a.relevance_score).unwrap()
        );
        
        // Iteratively remove elements until under budget
        while current_tokens > max_tokens && (!optimized.characters.is_empty() || !optimized.worldbuilding.is_empty()) {
            // Remove the lowest scoring element
            let char_score = optimized.characters.last().map(|c| c.relevance_score).unwrap_or(0.0);
            let world_score = optimized.worldbuilding.last().map(|w| w.relevance_score).unwrap_or(0.0);
            
            if char_score <= world_score && !optimized.characters.is_empty() {
                optimized.characters.pop();
            } else if !optimized.worldbuilding.is_empty() {
                optimized.worldbuilding.pop();
            }
            
            current_tokens = self.estimate_story_bible_tokens(&optimized);
        }
        
        // If still over budget, truncate descriptions
        if current_tokens > max_tokens {
            optimized = self.truncate_story_bible_descriptions(optimized, max_tokens).await?;
        }
        
        Ok(optimized)
    }
    
    pub async fn estimate_generation_cost(
        &self,
        model_name: &str,
        input_context: &ContextData,
        expected_output_tokens: usize,
    ) -> Result<CostEstimate> {
        let model_config = self.model_configs.get(model_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown model: {}", model_name))?;
        
        let input_tokens = self.estimate_total_tokens(input_context);
        
        let input_cost = input_tokens as f64 * model_config.cost_per_input_token;
        let output_cost = expected_output_tokens as f64 * model_config.cost_per_output_token;
        let total_cost = input_cost + output_cost;
        
        Ok(CostEstimate {
            input_tokens,
            output_tokens: expected_output_tokens,
            input_cost,
            output_cost,
            total_cost,
            model_name: model_name.to_string(),
            timestamp: chrono::Utc::now(),
        })
    }
}

// Optimization strategies
#[async_trait::async_trait]
pub trait OptimizationStrategy {
    async fn optimize(&self, context: ContextData, budget: &TokenBudget) -> Result<ContextData>;
    fn get_name(&self) -> &str;
}

pub struct SemanticCompressionStrategy {
    compression_service: Arc<CompressionService>,
}

#[async_trait::async_trait]
impl OptimizationStrategy for SemanticCompressionStrategy {
    async fn optimize(&self, mut context: ContextData, budget: &TokenBudget) -> Result<ContextData> {
        // Compress repetitive or redundant information
        if context.story_bible_context.characters.len() > 5 {
            context.story_bible_context = self.compression_service
                .compress_character_descriptions(context.story_bible_context)
                .await?;
        }
        
        if context.document_context.len() > budget.document_context {
            context.document_context = self.compression_service
                .extract_key_sentences(context.document_context, budget.document_context)
                .await?;
        }
        
        Ok(context)
    }
    
    fn get_name(&self) -> &str {
        "SemanticCompression"
    }
}

pub struct RelevanceFilteringStrategy;

#[async_trait::async_trait]
impl OptimizationStrategy for RelevanceFilteringStrategy {
    async fn optimize(&self, mut context: ContextData, _budget: &TokenBudget) -> Result<ContextData> {
        // Remove Story Bible elements below relevance threshold
        const MIN_RELEVANCE: f32 = 0.2;
        
        context.story_bible_context.characters.retain(|c| c.relevance_score >= MIN_RELEVANCE);
        context.story_bible_context.worldbuilding.retain(|w| w.relevance_score >= MIN_RELEVANCE);
        
        Ok(context)
    }
    
    fn get_name(&self) -> &str {
        "RelevanceFiltering"
    }
}
```

### Chapter Continuity Logic

#### Document Linking Awareness System
```rust
#[derive(Debug, Clone)]
pub struct ChapterContinuityManager {
    document_service: Arc<DocumentService>,
    link_analyzer: LinkAnalyzer,
    continuity_cache: Arc<tokio::sync::RwLock<HashMap<i32, ContinuityContext>>>,
}

#[derive(Debug, Clone)]
pub struct ContinuityContext {
    pub current_document_id: i32,
    pub linked_documents: Vec<LinkedDocument>,
    pub narrative_flow: NarrativeFlow,
    pub character_states: HashMap<i32, CharacterState>,
    pub plot_threads: Vec<PlotThread>,
    pub temporal_context: TemporalContext,
    pub consistency_warnings: Vec<ConsistencyWarning>,
}

#[derive(Debug, Clone)]
pub struct LinkedDocument {
    pub document_id: i32,
    pub title: String,
    pub link_type: LinkType,
    pub position: LinkPosition,
    pub summary: String,
    pub key_events: Vec<String>,
    pub character_changes: HashMap<i32, CharacterChange>,
}

#[derive(Debug, Clone)]
pub enum LinkType {
    Previous,
    Next,
    Parallel,
    Flashback,
    FlashForward,
    AlternateTimeline,
}

impl ChapterContinuityManager {
    pub async fn build_continuity_context(
        &self,
        document_id: i32,
        context_depth: usize,
    ) -> Result<ContinuityContext> {
        // Check cache first
        {
            let cache = self.continuity_cache.read().await;
            if let Some(cached_context) = cache.get(&document_id) {
                if self.is_cache_valid(cached_context) {
                    return Ok(cached_context.clone());
                }
            }
        }
        
        // Build fresh context
        let current_document = self.document_service.get_document(document_id).await?;
        let linked_documents = self.get_linked_documents(document_id, context_depth).await?;
        
        // Analyze narrative flow
        let narrative_flow = self.analyze_narrative_flow(&current_document, &linked_documents).await?;
        
        // Track character states across documents
        let character_states = self.track_character_states(&linked_documents).await?;
        
        // Identify plot threads
        let plot_threads = self.identify_plot_threads(&linked_documents).await?;
        
        // Build temporal context
        let temporal_context = self.build_temporal_context(&linked_documents).await?;
        
        // Check for consistency issues
        let consistency_warnings = self.check_consistency(&linked_documents, &character_states).await?;
        
        let context = ContinuityContext {
            current_document_id: document_id,
            linked_documents,
            narrative_flow,
            character_states,
            plot_threads,
            temporal_context,
            consistency_warnings,
        };
        
        // Cache the result
        {
            let mut cache = self.continuity_cache.write().await;
            cache.insert(document_id, context.clone());
        }
        
        Ok(context)
    }
    
    async fn get_linked_documents(
        &self,
        document_id: i32,
        depth: usize,
    ) -> Result<Vec<LinkedDocument>> {
        let mut linked_docs = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        queue.push_back((document_id, 0));
        visited.insert(document_id);
        
        while let Some((current_id, current_depth)) = queue.pop_front() {
            if current_depth >= depth {
                continue;
            }
            
            // Get direct links
            let links = self.document_service.get_document_links(current_id).await?;
            
            for link in links {
                let target_id = if link.from_document_id == current_id {
                    link.to_document_id
                } else {
                    link.from_document_id
                };
                
                if !visited.contains(&target_id) {
                    visited.insert(target_id);
                    queue.push_back((target_id, current_depth + 1));
                    
                    let document = self.document_service.get_document(target_id).await?;
                    let link_type = self.determine_link_type(current_id, target_id, &link).await?;
                    let position = self.determine_link_position(current_id, target_id, &link).await?;
                    
                    let linked_doc = LinkedDocument {
                        document_id: target_id,
                        title: document.name.clone(),
                        link_type,
                        position,
                        summary: self.generate_document_summary(&document).await?,
                        key_events: self.extract_key_events(&document).await?,
                        character_changes: self.analyze_character_changes(&document).await?,
                    };
                    
                    linked_docs.push(linked_doc);
                }
            }
        }
        
        // Sort by narrative order
        linked_docs.sort_by(|a, b| self.compare_narrative_order(a, b));
        
        Ok(linked_docs)
    }
    
    async fn analyze_narrative_flow(
        &self,
        current_document: &Document,
        linked_documents: &[LinkedDocument],
    ) -> Result<NarrativeFlow> {
        let mut flow = NarrativeFlow {
            current_position: 0,
            total_documents: linked_documents.len() + 1,
            previous_events: Vec::new(),
            upcoming_events: Vec::new(),
            parallel_events: Vec::new(),
            narrative_arc: NarrativeArc::Unknown,
        };
        
        // Determine current position in sequence
        let previous_docs: Vec<_> = linked_documents.iter()
            .filter(|doc| matches!(doc.link_type, LinkType::Previous))
            .collect();
        flow.current_position = previous_docs.len();
        
        // Extract events from previous documents
        for doc in &previous_docs {
            flow.previous_events.extend(doc.key_events.clone());
        }
        
        // Extract events from upcoming documents
        let next_docs: Vec<_> = linked_documents.iter()
            .filter(|doc| matches!(doc.link_type, LinkType::Next))
            .collect();
        for doc in &next_docs {
            flow.upcoming_events.extend(doc.key_events.clone());
        }
        
        // Extract parallel events
        let parallel_docs: Vec<_> = linked_documents.iter()
            .filter(|doc| matches!(doc.link_type, LinkType::Parallel))
            .collect();
        for doc in &parallel_docs {
            flow.parallel_events.extend(doc.key_events.clone());
        }
        
        // Determine narrative arc position
        flow.narrative_arc = self.determine_narrative_arc(flow.current_position, flow.total_documents);
        
        Ok(flow)
    }
    
    async fn track_character_states(
        &self,
        linked_documents: &[LinkedDocument],
    ) -> Result<HashMap<i32, CharacterState>> {
        let mut character_states = HashMap::new();
        
        // Process documents in chronological order
        let mut chronological_docs = linked_documents.clone();
        chronological_docs.sort_by(|a, b| self.compare_chronological_order(a, b));
        
        for doc in &chronological_docs {
            for (character_id, change) in &doc.character_changes {
                let state = character_states.entry(*character_id)
                    .or_insert_with(|| CharacterState::default());
                
                // Apply character changes
                self.apply_character_change(state, change);
            }
        }
        
        Ok(character_states)
    }
    
    async fn identify_plot_threads(
        &self,
        linked_documents: &[LinkedDocument],
    ) -> Result<Vec<PlotThread>> {
        let mut plot_threads = Vec::new();
        let mut thread_tracker = HashMap::new();
        
        for doc in linked_documents {
            for event in &doc.key_events {
                // Use AI to classify events into plot threads
                let thread_classification = self.classify_plot_thread(event).await?;
                
                let thread = thread_tracker.entry(thread_classification.thread_id)
                    .or_insert_with(|| PlotThread {
                        id: thread_classification.thread_id,
                        name: thread_classification.thread_name,
                        events: Vec::new(),
                        status: PlotThreadStatus::Active,
                        importance: thread_classification.importance,
                    });
                
                thread.events.push(PlotEvent {
                    description: event.clone(),
                    document_id: doc.document_id,
                    timestamp: chrono::Utc::now(), // Would be actual document timestamp
                });
            }
        }
        
        plot_threads.extend(thread_tracker.into_values());
        
        // Sort by importance
        plot_threads.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
        
        Ok(plot_threads)
    }
    
    async fn check_consistency(
        &self,
        linked_documents: &[LinkedDocument],
        character_states: &HashMap<i32, CharacterState>,
    ) -> Result<Vec<ConsistencyWarning>> {
        let mut warnings = Vec::new();
        
        // Check character consistency
        for (character_id, state) in character_states {
            if let Some(inconsistency) = self.check_character_consistency(*character_id, state).await? {
                warnings.push(ConsistencyWarning {
                    warning_type: WarningType::CharacterInconsistency,
                    description: inconsistency.description,
                    affected_documents: inconsistency.affected_documents,
                    severity: inconsistency.severity,
                    suggestions: inconsistency.suggestions,
                });
            }
        }
        
        // Check timeline consistency
        let timeline_warnings = self.check_timeline_consistency(linked_documents).await?;
        warnings.extend(timeline_warnings);
        
        // Check plot thread consistency
        let plot_warnings = self.check_plot_consistency(linked_documents).await?;
        warnings.extend(plot_warnings);
        
        Ok(warnings)
    }
    
    pub async fn generate_continuity_prompt(
        &self,
        context: &ContinuityContext,
        generation_type: GenerationType,
    ) -> Result<String> {
        let mut prompt = String::new();
        
        // Add narrative flow context
        prompt.push_str(&format!(
            "Narrative Context: This is document {} of {} in the sequence.\n",
            context.narrative_flow.current_position + 1,
            context.narrative_flow.total_documents
        ));
        
        // Add previous events for continuity
        if !context.narrative_flow.previous_events.is_empty() {
            prompt.push_str("\nPrevious events:\n");
            for event in &context.narrative_flow.previous_events {
                prompt.push_str(&format!("- {}\n", event));
            }
        }
        
        // Add character states
        if !context.character_states.is_empty() {
            prompt.push_str("\nCharacter states:\n");
            for (character_id, state) in &context.character_states {
                if let Some(character) = self.get_character(*character_id).await? {
                    prompt.push_str(&format!(
                        "- {}: {} (Location: {}, Emotional state: {})\n",
                        character.name,
                        state.current_description,
                        state.current_location.as_deref().unwrap_or("Unknown"),
                        state.emotional_state.as_deref().unwrap_or("Unknown")
                    ));
                }
            }
        }
        
        // Add active plot threads
        let active_threads: Vec<_> = context.plot_threads.iter()
            .filter(|t| matches!(t.status, PlotThreadStatus::Active))
            .collect();
        
        if !active_threads.is_empty() {
            prompt.push_str("\nActive plot threads:\n");
            for thread in &active_threads {
                prompt.push_str(&format!("- {}\n", thread.name));
            }
        }
        
        // Add consistency warnings if any
        if !context.consistency_warnings.is_empty() {
            prompt.push_str("\nConsistency notes:\n");
            for warning in &context.consistency_warnings {
                if matches!(warning.severity, WarningSeverity::High) {
                    prompt.push_str(&format!("- IMPORTANT: {}\n", warning.description));
                }
            }
        }
        
        // Add generation-specific instructions
        match generation_type {
            GenerationType::Continue => {
                prompt.push_str("\nContinue the story naturally from where it left off, maintaining consistency with the established narrative flow and character states.");
            },
            GenerationType::Transition => {
                prompt.push_str("\nCreate a smooth transition that bridges the narrative gap while maintaining story continuity.");
            },
            GenerationType::Resolve => {
                prompt.push_str("\nWork towards resolving the active plot threads while maintaining character consistency.");
            },
        }
        
        Ok(prompt)
    }
}
```

---

## 4. Streaming & Real-time Features

### Streaming Generation Logic

#### Pause/Resume and Progress Tracking
```typescript
interface StreamingSession {
  sessionId: string;
  documentId: number;
  featureType: 'write' | 'rewrite' | 'expand' | 'describe';
  status: 'active' | 'paused' | 'completed' | 'cancelled' | 'error';
  progress: StreamingProgress;
  context: StreamingContext;
  generatedContent: string;
  canResume: boolean;
  pauseReason?: string;
  errorDetails?: string;
}

interface StreamingProgress {
  totalTokensExpected: number;
  tokensGenerated: number;
  percentComplete: number;
  estimatedTimeRemaining: number;
  wordsGenerated: number;
  currentSentence: string;
  generationRate: number; // tokens per second
}

interface StreamingContext {
  originalPrompt: string;
  modelUsed: string;
  temperature: number;
  maxTokens: number;
  storyBibleContext: string;
  documentContext: string;
  resumePoint?: string;
}

class StreamingManager {
  private activeSessions: Map<string, StreamingSession> = new Map();
  private aiProvider: AIProvider;
  private progressCallbacks: Map<string, (progress: StreamingProgress) => void> = new Map();
  private completionCallbacks: Map<string, (result: StreamingResult) => void> = new Map();
  
  public async startStreaming(
    documentId: number,
    featureType: StreamingSession['featureType'],
    prompt: string,
    context: Omit<StreamingContext, 'resumePoint'>,
    onProgress: (progress: StreamingProgress) => void,
    onComplete: (result: StreamingResult) => void
  ): Promise<string> {
    const sessionId = `stream_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    
    const session: StreamingSession = {
      sessionId,
      documentId,
      featureType,
      status: 'active',
      progress: {
        totalTokensExpected: context.maxTokens,
        tokensGenerated: 0,
        percentComplete: 0,
        estimatedTimeRemaining: 0,
        wordsGenerated: 0,
        currentSentence: '',
        generationRate: 0
      },
      context: { ...context, resumePoint: undefined },
      generatedContent: '',
      canResume: true
    };
    
    this.activeSessions.set(sessionId, session);
    this.progressCallbacks.set(sessionId, onProgress);
    this.completionCallbacks.set(sessionId, onComplete);
    
    // Start the streaming process
    this.executeStreaming(sessionId);
    
    return sessionId;
  }
  
  private async executeStreaming(sessionId: string): Promise<void> {
    const session = this.activeSessions.get(sessionId);
    if (!session) return;
    
    try {
      const startTime = Date.now();
      let lastProgressUpdate = startTime;
      
      const stream = await this.aiProvider.generateTextStream(
        session.context.originalPrompt,
        {
          model: session.context.modelUsed,
          temperature: session.context.temperature,
          max_tokens: session.context.maxTokens,
          stream: true
        }
      );
      
      for await (const chunk of stream) {
        // Check if session was paused or cancelled
        const currentSession = this.activeSessions.get(sessionId);
        if (!currentSession || currentSession.status !== 'active') {
          break;
        }
        
        // Process chunk
        const tokenCount = this.estimateTokenCount(chunk.content);
        session.generatedContent += chunk.content;
        session.progress.tokensGenerated += tokenCount;
        session.progress.wordsGenerated = this.countWords(session.generatedContent);
        
        // Update current sentence
        const sentences = session.generatedContent.split(/[.!?]+/);
        session.progress.currentSentence = sentences[sentences.length - 1].trim();
        
        // Calculate progress metrics
        const now = Date.now();
        const elapsedTime = (now - startTime) / 1000; // seconds
        session.progress.generationRate = session.progress.tokensGenerated / elapsedTime;
        session.progress.percentComplete = Math.min(
          (session.progress.tokensGenerated / session.progress.totalTokensExpected) * 100,
          100
        );
        
        const remainingTokens = session.progress.totalTokensExpected - session.progress.tokensGenerated;
        session.progress.estimatedTimeRemaining = remainingTokens / session.progress.generationRate;
        
        // Throttled progress updates (max 10 per second)
        if (now - lastProgressUpdate >= 100) {
          const progressCallback = this.progressCallbacks.get(sessionId);
          if (progressCallback) {
            progressCallback(session.progress);
          }
          lastProgressUpdate = now;
        }
        
        // Auto-save progress periodically
        if (session.progress.tokensGenerated % 50 === 0) {
          await this.saveSessionProgress(session);
        }
      }
      
      // Mark as completed
      session.status = 'completed';
      session.progress.percentComplete = 100;
      
      const completionCallback = this.completionCallbacks.get(sessionId);
      if (completionCallback) {
        completionCallback({
          sessionId,
          generatedContent: session.generatedContent,
          totalTokens: session.progress.tokensGenerated,
          totalWords: session.progress.wordsGenerated,
          generationTime: (Date.now() - startTime) / 1000
        });
      }
      
    } catch (error) {
      session.status = 'error';
      session.errorDetails = error.message;
      
      const completionCallback = this.completionCallbacks.get(sessionId);
      if (completionCallback) {
        completionCallback({
          sessionId,
          error: error.message,
          generatedContent: session.generatedContent,
          totalTokens: session.progress.tokensGenerated,
          totalWords: session.progress.wordsGenerated
        });
      }
    } finally {
      await this.saveSessionProgress(session);
    }
  }
  
  public async pauseStreaming(sessionId: string, reason?: string): Promise<boolean> {
    const session = this.activeSessions.get(sessionId);
    if (!session || session.status !== 'active') {
      return false;
    }
    
    session.status = 'paused';
    session.pauseReason = reason;
    session.context.resumePoint = session.generatedContent;
    
    await this.saveSessionProgress(session);
    
    return true;
  }
  
  public async resumeStreaming(sessionId: string): Promise<boolean> {
    const session = this.activeSessions.get(sessionId);
    if (!session || session.status !== 'paused' || !session.canResume) {
      return false;
    }
    
    // Modify prompt to continue from where we left off
    const resumePrompt = this.buildResumePrompt(session);
    session.context.originalPrompt = resumePrompt;
    session.status = 'active';
    
    // Continue streaming
    this.executeStreaming(sessionId);
    
    return true;
  }
  
  private buildResumePrompt(session: StreamingSession): string {
    const basePrompt = session.context.originalPrompt;
    const generatedSoFar = session.generatedContent;
    
    return `${basePrompt}
    
Generated content so far:
${generatedSoFar}

Continue naturally from where the text left off, maintaining the same style and flow:`;
  }
  
  public async cancelStreaming(sessionId: string): Promise<boolean> {
    const session = this.activeSessions.get(sessionId);
    if (!session) {
      return false;
    }
    
    session.status = 'cancelled';
    session.canResume = false;
    
    await this.saveSessionProgress(session);
    
    // Clean up callbacks
    this.progressCallbacks.delete(sessionId);
    this.completionCallbacks.delete(sessionId);
    
    return true;
  }
  
  public getSessionStatus(sessionId: string): StreamingSession | null {
    return this.activeSessions.get(sessionId) || null;
  }
  
  public async recoverSession(sessionId: string): Promise<StreamingSession | null> {
    // Try to recover from database
    const savedSession = await this.loadSessionFromDatabase(sessionId);
    if (savedSession && savedSession.canResume) {
      this.activeSessions.set(sessionId, savedSession);
      return savedSession;
    }
    return null;
  }
  
  private async saveSessionProgress(session: StreamingSession): Promise<void> {
    // Save to database for recovery
    await this.database.execute(`
      INSERT OR REPLACE INTO streaming_sessions 
      (session_id, document_id, feature_type, status, progress_data, context_data, generated_content, can_resume, created_at, updated_at)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    `, [
      session.sessionId,
      session.documentId,
      session.featureType,
      session.status,
      JSON.stringify(session.progress),
      JSON.stringify(session.context),
      session.generatedContent,
      session.canResume,
      new Date().toISOString(),
      new Date().toISOString()
    ]);
  }
  
  private estimateTokenCount(text: string): number {
    // Rough estimation: ~4 characters per token for English
    return Math.ceil(text.length / 4);
  }
  
  private countWords(text: string): number {
    return text.trim().split(/\s+/).length;
  }
}
```

### Real-time Updates System

#### UI Updates During Streaming
```typescript
interface StreamingUIState {
  isStreaming: boolean;
  currentSession?: StreamingSession;
  displayedContent: string;
  pendingContent: string;
  animationQueue: string[];
  typewriterSpeed: number;
  isUserScrolling: boolean;
  autoScroll: boolean;
}

class StreamingUIManager {
  private state: StreamingUIState;
  private editorInstance: monaco.editor.IStandaloneCodeEditor;
  private streamingManager: StreamingManager;
  private animationFrameId?: number;
  private typewriterIntervalId?: number;
  
  constructor(editor: monaco.editor.IStandaloneCodeEditor, streamingManager: StreamingManager) {
    this.editorInstance = editor;
    this.streamingManager = streamingManager;
    this.state = {
      isStreaming: false,
      displayedContent: '',
      pendingContent: '',
      animationQueue: [],
      typewriterSpeed: 50, // ms per character
      isUserScrolling: false,
      autoScroll: true
    };
    
    this.setupEventListeners();
  }
  
  private setupEventListeners(): void {
    // Detect user scrolling
    this.editorInstance.onDidScrollChange((e) => {
      if (this.state.isStreaming) {
        // If user scrolls up significantly, disable auto-scroll
        const model = this.editorInstance.getModel();
        if (model) {
          const totalLines = model.getLineCount();
          const visibleRange = this.editorInstance.getVisibleRanges()[0];
          const isNearBottom = visibleRange.endLineNumber >= totalLines - 5;
          
          this.state.autoScroll = isNearBottom;
          this.state.isUserScrolling = !isNearBottom;
        }
      }
    });
    
    // Handle window focus/blur for performance optimization
    window.addEventListener('focus', () => {
      if (this.state.isStreaming) {
        this.resumeUIUpdates();
      }
    });
    
    window.addEventListener('blur', () => {
      if (this.state.isStreaming) {
        this.pauseUIUpdates();
      }
    });
  }
  
  public startStreamingUI(sessionId: string): void {
    this.state.isStreaming = true;
    this.state.currentSession = this.streamingManager.getSessionStatus(sessionId);
    this.state.displayedContent = '';
    this.state.pendingContent = '';
    this.state.animationQueue = [];
    
    // Add streaming indicator
    this.showStreamingIndicator();
    
    // Start typewriter effect
    this.startTypewriterEffect();
    
    // Set up progress monitoring
    this.monitorStreamingProgress(sessionId);
  }
  
  private showStreamingIndicator(): void {
    const indicator = document.createElement('div');
    indicator.id = 'streaming-indicator';
    indicator.className = 'streaming-indicator';
    indicator.innerHTML = `
      <div class="streaming-dots">
        <span></span>
        <span></span>
        <span></span>
      </div>
      <span class="streaming-text">AI is writing...</span>
      <button class="pause-btn" onclick="this.pauseStreaming()">Pause</button>
    `;
    
    document.body.appendChild(indicator);
  }
  
  private startTypewriterEffect(): void {
    this.typewriterIntervalId = setInterval(() => {
      if (this.state.pendingContent.length > 0) {
        // Add next character to displayed content
        const nextChar = this.state.pendingContent.charAt(0);
        this.state.displayedContent += nextChar;
        this.state.pendingContent = this.state.pendingContent.substring(1);
        
        // Update editor content
        this.updateEditorContent();
        
        // Auto-scroll if enabled
        if (this.state.autoScroll) {
          this.scrollToBottom();
        }
      }
    }, this.state.typewriterSpeed);
  }
  
  private updateEditorContent(): void {
    const model = this.editorInstance.getModel();
    if (!model) return;
    
    // Get current cursor position
    const position = this.editorInstance.getPosition();
    
    // Update content
    const currentContent = model.getValue();
    const insertPosition = model.getPositionAt(currentContent.length);
    
    // Insert new content at the end
    const newContent = this.state.displayedContent.substring(currentContent.length);
    if (newContent) {
      const range = new monaco.Range(
        insertPosition.lineNumber,
        insertPosition.column,
        insertPosition.lineNumber,
        insertPosition.column
      );
      
      model.pushEditOperations([], [{
        range,
        text: newContent
      }], () => null);
    }
    
    // Restore cursor position if user was editing
    if (!this.state.autoScroll && position) {
      this.editorInstance.setPosition(position);
    }
  }
  
  private monitorStreamingProgress(sessionId: string): void {
    const updateProgress = () => {
      const session = this.streamingManager.getSessionStatus(sessionId);
      if (!session) return;
      
      // Add new content to pending queue
      const newContent = session.generatedContent.substring(this.state.displayedContent.length + this.state.pendingContent.length);
      if (newContent) {
        this.state.pendingContent += newContent;
      }
      
      // Update progress indicator
      this.updateProgressIndicator(session.progress);
      
      // Continue monitoring if still active
      if (session.status === 'active') {
        this.animationFrameId = requestAnimationFrame(updateProgress);
      } else {
        this.handleStreamingComplete(session);
      }
    };
    
    this.animationFrameId = requestAnimationFrame(updateProgress);
  }
  
  private updateProgressIndicator(progress: StreamingProgress): void {
    const indicator = document.getElementById('streaming-indicator');
    if (!indicator) return;
    
    const progressBar = indicator.querySelector('.progress-bar') as HTMLElement;
    const progressText = indicator.querySelector('.streaming-text') as HTMLElement;
    const wordsCount = indicator.querySelector('.words-count') as HTMLElement;
    
    if (progressBar) {
      progressBar.style.width = `${progress.percentComplete}%`;
    }
    
    if (progressText) {
      progressText.textContent = `AI is writing... (${Math.round(progress.percentComplete)}%)`;
    }
    
    if (wordsCount) {
      wordsCount.textContent = `${progress.wordsGenerated} words`;
    }
    
    // Show current sentence being generated
    const currentSentence = indicator.querySelector('.current-sentence') as HTMLElement;
    if (currentSentence && progress.currentSentence) {
      currentSentence.textContent = `Current: "${progress.currentSentence}..."`;
    }
  }
  
  private handleStreamingComplete(session: StreamingSession): void {
    // Finish displaying any remaining content
    if (this.state.pendingContent) {
      this.state.displayedContent += this.state.pendingContent;
      this.state.pendingContent = '';
      this.updateEditorContent();
    }
    
    // Clean up
    this.stopTypewriterEffect();
    this.hideStreamingIndicator();
    
    this.state.isStreaming = false;
    this.state.currentSession = undefined;
    
    // Show completion notification
    this.showCompletionNotification(session);
    
    // Trigger final save
    this.triggerAutoSave();
  }
  
  private stopTypewriterEffect(): void {
    if (this.typewriterIntervalId) {
      clearInterval(this.typewriterIntervalId);
      this.typewriterIntervalId = undefined;
    }
    
    if (this.animationFrameId) {
      cancelAnimationFrame(this.animationFrameId);
      this.animationFrameId = undefined;
    }
  }
  
  private hideStreamingIndicator(): void {
    const indicator = document.getElementById('streaming-indicator');
    if (indicator) {
      indicator.remove();
    }
  }
  
  private showCompletionNotification(session: StreamingSession): void {
    const notification = document.createElement('div');
    notification.className = 'streaming-complete-notification';
    notification.innerHTML = `
      <div class="notification-content">
        <h3>Generation Complete!</h3>
        <p>${session.progress.wordsGenerated} words generated</p>
        <div class="notification-actions">
          <button onclick="this.acceptGeneration()">Accept</button>
          <button onclick="this.rejectGeneration()">Reject</button>
          <button onclick="this.continueGeneration()">Continue</button>
        </div>
      </div>
    `;
    
    document.body.appendChild(notification);
    
    // Auto-hide after 10 seconds
    setTimeout(() => {
      if (notification.parentNode) {
        notification.remove();
      }
    }, 10000);
  }
  
  public pauseStreaming(): void {
    if (this.state.currentSession) {
      this.streamingManager.pauseStreaming(this.state.currentSession.sessionId, 'User requested pause');
      this.showPauseControls();
    }
  }
  
  private showPauseControls(): void {
    const controls = document.createElement('div');
    controls.id = 'pause-controls';
    controls.className = 'pause-controls';
    controls.innerHTML = `
      <div class="pause-message">
        <h3>Generation Paused</h3>
        <p>You can resume generation or make edits to the current content.</p>
        <div class="pause-actions">
          <button onclick="this.resumeStreaming()">Resume</button>
          <button onclick="this.cancelStreaming()">Cancel</button>
          <button onclick="this.editAndContinue()">Edit & Continue</button>
        </div>
      </div>
    `;
    
    document.body.appendChild(controls);
  }
  
  public resumeStreaming(): void {
    if (this.state.currentSession) {
      this.streamingManager.resumeStreaming(this.state.currentSession.sessionId);
      this.hidePauseControls();
      this.startTypewriterEffect();
    }
  }
  
  private hidePauseControls(): void {
    const controls = document.getElementById('pause-controls');
    if (controls) {
      controls.remove();
    }
  }
  
  private scrollToBottom(): void {
    const model = this.editorInstance.getModel();
    if (model) {
      const lastLine = model.getLineCount();
      this.editorInstance.revealLine(lastLine);
    }
  }
  
  private pauseUIUpdates(): void {
    // Reduce update frequency when window is not focused
    this.state.typewriterSpeed = 200; // Slower updates
  }
  
  private resumeUIUpdates(): void {
    // Resume normal update frequency
    this.state.typewriterSpeed = 50;
  }
  
  private triggerAutoSave(): void {
    // Trigger document auto-save after streaming completes
    const model = this.editorInstance.getModel();
    if (model) {
      const content = model.getValue();
      // Emit save event or call save function
      this.emitSaveEvent(content);
    }
  }
  
  private emitSaveEvent(content: string): void {
    const event = new CustomEvent('streaming-complete-save', {
      detail: { content }
    });
    document.dispatchEvent(event);
  }
}
```

### Background Processing System

#### Queue Management for Multiple AI Operations
```rust
use std::collections::{HashMap, VecDeque};
use tokio::sync::{mpsc, RwLock, Semaphore};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BackgroundProcessor {
    task_queue: Arc<RwLock<VecDeque<ProcessingTask>>>,
    active_tasks: Arc<RwLock<HashMap<String, ActiveTask>>>,
    completed_tasks: Arc<RwLock<HashMap<String, CompletedTask>>>,
    semaphore: Arc<Semaphore>,
    task_sender: mpsc::UnboundedSender<ProcessingTask>,
    max_concurrent_tasks: usize,
}

#[derive(Debug, Clone)]
pub struct ProcessingTask {
    pub id: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub project_id: i32,
    pub document_id: Option<i32>,
    pub payload: TaskPayload,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub estimated_duration: Option<std::time::Duration>,
    pub dependencies: Vec<String>,
    pub retry_count: u32,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub enum TaskType {
    TextGeneration,
    ImageGeneration,
    StoryBibleAnalysis,
    DocumentAnalysis,
    ContinuityCheck,
    StyleAnalysis,
    CharacterExtraction,
    PlotAnalysis,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
    UserInitiated = 5,
}

#[derive(Debug, Clone)]
pub struct ActiveTask {
    pub task: ProcessingTask,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub progress: TaskProgress,
    pub can_cancel: bool,
    pub cancel_token: tokio_util::sync::CancellationToken,
}

#[derive(Debug, Clone)]
pub struct TaskProgress {
    pub percentage: f32,
    pub current_step: String,
    pub estimated_remaining: Option<std::time::Duration>,
    pub tokens_processed: Option<usize>,
    pub tokens_total: Option<usize>,
}

impl BackgroundProcessor {
    pub fn new(max_concurrent_tasks: usize) -> Self {
        let (task_sender, task_receiver) = mpsc::unbounded_channel();
        
        let processor = Self {
            task_queue: Arc::new(RwLock::new(VecDeque::new())),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            completed_tasks: Arc::new(RwLock::new(HashMap::new())),
            semaphore: Arc::new(Semaphore::new(max_concurrent_tasks)),
            task_sender,
            max_concurrent_tasks,
        };
        
        // Start the task processor
        processor.start_task_processor(task_receiver);
        
        processor
    }
    
    pub async fn submit_task(&self, mut task: ProcessingTask) -> Result<String> {
        // Assign unique ID if not provided
        if task.id.is_empty() {
            task.id = Uuid::new_v4().to_string();
        }
        
        // Validate dependencies
        self.validate_task_dependencies(&task).await?;
        
        // Add to queue
        {
            let mut queue = self.task_queue.write().await;
            
            // Insert based on priority (higher priority first)
            let insert_position = queue.iter()
                .position(|t| t.priority < task.priority)
                .unwrap_or(queue.len());
            
            queue.insert(insert_position, task.clone());
        }
        
        // Notify processor
        self.task_sender.send(task.clone())
            .map_err(|_| anyhow::anyhow!("Failed to submit task"))?;
        
        Ok(task.id)
    }
    
    fn start_task_processor(&self, mut task_receiver: mpsc::UnboundedReceiver<ProcessingTask>) {
        let task_queue = Arc::clone(&self.task_queue);
        let active_tasks = Arc::clone(&self.active_tasks);
        let completed_tasks = Arc::clone(&self.completed_tasks);
        let semaphore = Arc::clone(&self.semaphore);
        
        tokio::spawn(async move {
            while let Some(_) = task_receiver.recv().await {
                // Process tasks from queue
                loop {
                    let task = {
                        let mut queue = task_queue.write().await;
                        queue.pop_front()
                    };
                    
                    if let Some(task) = task {
                        // Check if dependencies are satisfied
                        if !Self::are_dependencies_satisfied(&task, &completed_tasks).await {
                            // Put back in queue and try later
                            let mut queue = task_queue.write().await;
                            queue.push_back(task);
                            break;
                        }
                        
                        // Try to acquire semaphore (non-blocking)
                        if let Ok(permit) = semaphore.try_acquire() {
                            let active_tasks_clone = Arc::clone(&active_tasks);
                            let completed_tasks_clone = Arc::clone(&completed_tasks);
                            
                            tokio::spawn(async move {
                                let _permit = permit; // Keep permit alive
                                Self::execute_task(task, active_tasks_clone, completed_tasks_clone).await;
                            });
                        } else {
                            // No available slots, put task back and wait
                            let mut queue = task_queue.write().await;
                            queue.push_front(task);
                            break;
                        }
                    } else {
                        break; // No more tasks in queue
                    }
                }
            }
        });
    }
    
    async fn execute_task(
        task: ProcessingTask,
        active_tasks: Arc<RwLock<HashMap<String, ActiveTask>>>,
        completed_tasks: Arc<RwLock<HashMap<String, CompletedTask>>>,
    ) {
        let task_id = task.id.clone();
        let cancel_token = tokio_util::sync::CancellationToken::new();
        
        // Mark as active
        {
            let mut active = active_tasks.write().await;
            active.insert(task_id.clone(), ActiveTask {
                task: task.clone(),
                started_at: chrono::Utc::now(),
                progress: TaskProgress {
                    percentage: 0.0,
                    current_step: "Starting...".to_string(),
                    estimated_remaining: task.estimated_duration,
                    tokens_processed: None,
                    tokens_total: None,
                },
                can_cancel: true,
                cancel_token: cancel_token.clone(),
            });
        }
        
        let result = match task.task_type {
            TaskType::TextGeneration => {
                Self::execute_text_generation(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::ImageGeneration => {
                Self::execute_image_generation(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::StoryBibleAnalysis => {
                Self::execute_story_bible_analysis(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::DocumentAnalysis => {
                Self::execute_document_analysis(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::ContinuityCheck => {
                Self::execute_continuity_check(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::StyleAnalysis => {
                Self::execute_style_analysis(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::CharacterExtraction => {
                Self::execute_character_extraction(task.clone(), cancel_token, &active_tasks).await
            },
            TaskType::PlotAnalysis => {
                Self::execute_plot_analysis(task.clone(), cancel_token, &active_tasks).await
            },
        };
        
        // Remove from active tasks
        {
            let mut active = active_tasks.write().await;
            active.remove(&task_id);
        }
        
        // Add to completed tasks
        {
            let mut completed = completed_tasks.write().await;
            completed.insert(task_id.clone(), CompletedTask {
                task_id: task_id.clone(),
                task_type: task.task_type,
                result,
                completed_at: chrono::Utc::now(),
                execution_time: chrono::Utc::now() - task.created_at,
            });
        }
        
        // Notify completion (could emit event here)
        Self::notify_task_completion(&task_id).await;
    }
    
    async fn execute_text_generation(
        task: ProcessingTask,
        cancel_token: tokio_util::sync::CancellationToken,
        active_tasks: &Arc<RwLock<HashMap<String, ActiveTask>>>,
    ) -> TaskResult {
        // Update progress
        Self::update_task_progress(&task.id, 10.0, "Preparing context...", active_tasks).await;
        
        // Check for cancellation
        if cancel_token.is_cancelled() {
            return TaskResult::Cancelled;
        }
        
        // Extract payload
        let generation_payload = match &task.payload {
            TaskPayload::TextGeneration(payload) => payload,
            _ => return TaskResult::Error("Invalid payload type".to_string()),
        };
        
        // Build context
        Self::update_task_progress(&task.id, 30.0, "Building AI context...", active_tasks).await;
        
        // Simulate context building (would be actual implementation)
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        if cancel_token.is_cancelled() {
            return TaskResult::Cancelled;
        }
        
        // Generate text
        Self::update_task_progress(&task.id, 50.0, "Generating text...", active_tasks).await;
        
        // Simulate text generation with progress updates
        for i in 1..=5 {
            if cancel_token.is_cancelled() {
                return TaskResult::Cancelled;
            }
            
            let progress = 50.0 + (i as f32 * 8.0);
            Self::update_task_progress(
                &task.id, 
                progress, 
                &format!("Generating... ({}/5)", i), 
                active_tasks
            ).await;
            
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
        
        // Finalize
        Self::update_task_progress(&task.id, 95.0, "Finalizing...", active_tasks).await;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        
        Self::update_task_progress(&task.id, 100.0, "Complete", active_tasks).await;
        
        TaskResult::Success(TaskOutput::Text("Generated text content".to_string()))
    }
    
    async fn update_task_progress(
        task_id: &str,
        percentage: f32,
        step: &str,
        active_tasks: &Arc<RwLock<HashMap<String, ActiveTask>>>,
    ) {
        let mut active = active_tasks.write().await;
        if let Some(active_task) = active.get_mut(task_id) {
            active_task.progress.percentage = percentage;
            active_task.progress.current_step = step.to_string();
            
            // Update estimated remaining time
            if percentage > 0.0 {
                let elapsed = chrono::Utc::now() - active_task.started_at;
                let total_estimated = elapsed.num_milliseconds() as f32 / (percentage / 100.0);
                let remaining = total_estimated - elapsed.num_milliseconds() as f32;
                
                if remaining > 0.0 {
                    active_task.progress.estimated_remaining = Some(
                        std::time::Duration::from_millis(remaining as u64)
                    );
                }
            }
        }
    }
    
    pub async fn cancel_task(&self, task_id: &str) -> Result<bool> {
        // Check if task is active
        {
            let active = self.active_tasks.read().await;
            if let Some(active_task) = active.get(task_id) {
                if active_task.can_cancel {
                    active_task.cancel_token.cancel();
                    return Ok(true);
                }
            }
        }
        
        // Check if task is in queue
        {
            let mut queue = self.task_queue.write().await;
            if let Some(pos) = queue.iter().position(|t| t.id == task_id) {
                queue.remove(pos);
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    pub async fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        // Check active tasks
        {
            let active = self.active_tasks.read().await;
            if let Some(active_task) = active.get(task_id) {
                return Some(TaskStatus::Active(active_task.progress.clone()));
            }
        }
        
        // Check completed tasks
        {
            let completed = self.completed_tasks.read().await;
            if let Some(completed_task) = completed.get(task_id) {
                return Some(TaskStatus::Completed(completed_task.clone()));
            }
        }
        
        // Check queued tasks
        {
            let queue = self.task_queue.read().await;
            if let Some(task) = queue.iter().find(|t| t.id == task_id) {
                let position = queue.iter().position(|t| t.id == task_id).unwrap_or(0);
                return Some(TaskStatus::Queued { position });
            }
        }
        
        None
    }
    
    pub async fn get_queue_status(&self) -> QueueStatus {
        let active = self.active_tasks.read().await;
        let queue = self.task_queue.read().await;
        
        QueueStatus {
            active_tasks: active.len(),
            queued_tasks: queue.len(),
            max_concurrent: self.max_concurrent_tasks,
            available_slots: self.max_concurrent_tasks - active.len(),
        }
    }
    
    async fn are_dependencies_satisfied(
        task: &ProcessingTask,
        completed_tasks: &Arc<RwLock<HashMap<String, CompletedTask>>>,
    ) -> bool {
        if task.dependencies.is_empty() {
            return true;
        }
        
        let completed = completed_tasks.read().await;
        task.dependencies.iter().all(|dep_id| {
            completed.get(dep_id)
                .map(|completed_task| matches!(completed_task.result, TaskResult::Success(_)))
                .unwrap_or(false)
        })
    }
    
    async fn validate_task_dependencies(&self, task: &ProcessingTask) -> Result<()> {
        // Check for circular dependencies
        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![task.id.clone()];
        
        while let Some(current_id) = stack.pop() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());
            
            // Find task with this ID and check its dependencies
            let queue = self.task_queue.read().await;
            if let Some(current_task) = queue.iter().find(|t| t.id == current_id) {
                for dep_id in &current_task.dependencies {
                    if dep_id == &task.id {
                        return Err(anyhow::anyhow!("Circular dependency detected"));
                    }
                    stack.push(dep_id.clone());
                }
            }
        }
        
        Ok(())
    }
    
    async fn notify_task_completion(task_id: &str) {
        // Could emit events, send notifications, etc.
        println!("Task {} completed", task_id);
    }
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Queued { position: usize },
    Active(TaskProgress),
    Completed(CompletedTask),
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct QueueStatus {
    pub active_tasks: usize,
    pub queued_tasks: usize,
    pub max_concurrent: usize,
    pub available_slots: usize,
}

#[derive(Debug, Clone)]
pub struct CompletedTask {
    pub task_id: String,
    pub task_type: TaskType,
    pub result: TaskResult,
    pub completed_at: chrono::DateTime<chrono::Utc>,
    pub execution_time: chrono::Duration,
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Success(TaskOutput),
    Error(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum TaskOutput {
    Text(String),
    Image(Vec<u8>),
    Analysis(serde_json::Value),
    Characters(Vec<Character>),
}

#[derive(Debug, Clone)]
pub enum TaskPayload {
    TextGeneration(TextGenerationPayload),
    ImageGeneration(ImageGenerationPayload),
    Analysis(AnalysisPayload),
}

#[derive(Debug, Clone)]
pub struct TextGenerationPayload {
    pub prompt: String,
    pub context: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub model: String,
}
```

---

## 5. Credit System Logic

### Cost Estimation Algorithms

#### Pre-generation Cost Calculation
```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CreditManager {
    model_costs: HashMap<String, ModelCostConfig>,
    feature_costs: HashMap<String, FeatureCostConfig>,
    usage_tracker: Arc<tokio::sync::RwLock<UsageTracker>>,
    balance_manager: BalanceManager,
    optimization_engine: CostOptimizationEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCostConfig {
    pub model_name: String,
    pub provider: String,
    pub cost_per_input_token: f64,
    pub cost_per_output_token: f64,
    pub cost_per_image: Option<f64>,
    pub minimum_charge: f64,
    pub context_window: usize,
    pub quality_tier: QualityTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCostConfig {
    pub feature_name: String,
    pub base_cost: i32, // in credits
    pub cost_per_word: f64,
    pub cost_per_token: f64,
    pub high_quality_multiplier: f64,
    pub minimum_cost: i32,
    pub maximum_cost: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityTier {
    Basic,
    Standard,
    Premium,
    Ultra,
}

#[derive(Debug, Clone)]
pub struct CostEstimate {
    pub feature: String,
    pub model: String,
    pub estimated_input_tokens: usize,
    pub estimated_output_tokens: usize,
    pub base_credits: i32,
    pub quality_adjustment: i32,
    pub optimization_savings: i32,
    pub total_credits: i32,
    pub confidence_level: f32,
    pub breakdown: CostBreakdown,
}

#[derive(Debug, Clone)]
pub struct CostBreakdown {
    pub input_cost: i32,
    pub output_cost: i32,
    pub feature_overhead: i32,
    pub quality_premium: i32,
    pub context_cost: i32,
    pub processing_cost: i32,
}

impl CreditManager {
    pub fn new() -> Self {
        let mut manager = Self {
            model_costs: HashMap::new(),
            feature_costs: HashMap::new(),
            usage_tracker: Arc::new(tokio::sync::RwLock::new(UsageTracker::new())),
            balance_manager: BalanceManager::new(),
            optimization_engine: CostOptimizationEngine::new(),
        };
        
        manager.initialize_cost_configs();
        manager
    }
    
    fn initialize_cost_configs(&mut self) {
        // Initialize model costs
        self.model_costs.insert("gpt-4".to_string(), ModelCostConfig {
            model_name: "gpt-4".to_string(),
            provider: "openai".to_string(),
            cost_per_input_token: 0.00003,
            cost_per_output_token: 0.00006,
            cost_per_image: None,
            minimum_charge: 0.001,
            context_window: 8192,
            quality_tier: QualityTier::Premium,
        });
        
        self.model_costs.insert("gpt-3.5-turbo".to_string(), ModelCostConfig {
            model_name: "gpt-3.5-turbo".to_string(),
            provider: "openai".to_string(),
            cost_per_input_token: 0.0000015,
            cost_per_output_token: 0.000002,
            cost_per_image: None,
            minimum_charge: 0.0001,
            context_window: 4096,
            quality_tier: QualityTier::Standard,
        });
        
        self.model_costs.insert("claude-3-opus".to_string(), ModelCostConfig {
            model_name: "claude-3-opus".to_string(),
            provider: "anthropic".to_string(),
            cost_per_input_token: 0.000015,
            cost_per_output_token: 0.000075,
            cost_per_image: None,
            minimum_charge: 0.001,
            context_window: 200000,
            quality_tier: QualityTier::Ultra,
        });
        
        // Initialize feature costs
        self.feature_costs.insert("write".to_string(), FeatureCostConfig {
            feature_name: "write".to_string(),
            base_cost: 25,
            cost_per_word: 0.5,
            cost_per_token: 0.1,
            high_quality_multiplier: 2.0,
            minimum_cost: 10,
            maximum_cost: Some(500),
        });
        
        self.feature_costs.insert("rewrite".to_string(), FeatureCostConfig {
            feature_name: "rewrite".to_string(),
            base_cost: 15,
            cost_per_word: 0.3,
            cost_per_token: 0.08,
            high_quality_multiplier: 1.8,
            minimum_cost: 8,
            maximum_cost: Some(300),
        });
        
        self.feature_costs.insert("expand".to_string(), FeatureCostConfig {
            feature_name: "expand".to_string(),
            base_cost: 30,
            cost_per_word: 0.6,
            cost_per_token: 0.12,
            high_quality_multiplier: 2.2,
            minimum_cost: 15,
            maximum_cost: Some(600),
        });
        
        self.feature_costs.insert("visualize".to_string(), FeatureCostConfig {
            feature_name: "visualize".to_string(),
            base_cost: 2500,
            cost_per_word: 0.0,
            cost_per_token: 0.0,
            high_quality_multiplier: 1.0,
            minimum_cost: 2500,
            maximum_cost: Some(2500),
        });
    }
    
    pub async fn estimate_cost(
        &self,
        feature: &str,
        context: &CostEstimationContext,
    ) -> Result<CostEstimate> {
        let feature_config = self.feature_costs.get(feature)
            .ok_or_else(|| anyhow::anyhow!("Unknown feature: {}", feature))?;
        
        let model_config = self.model_costs.get(&context.model)
            .ok_or_else(|| anyhow::anyhow!("Unknown model: {}", context.model))?;
        
        // Estimate input tokens
        let input_tokens = self.estimate_input_tokens(context).await?;
        
        // Estimate output tokens based on feature and context
        let output_tokens = self.estimate_output_tokens(feature, context).await?;
        
        // Calculate base costs
        let mut breakdown = CostBreakdown {
            input_cost: self.calculate_input_cost(input_tokens, model_config),
            output_cost: self.calculate_output_cost(output_tokens, model_config),
            feature_overhead: feature_config.base_cost,
            quality_premium: 0,
            context_cost: self.calculate_context_cost(context),
            processing_cost: self.calculate_processing_cost(feature, context),
        };
        
        // Apply quality adjustments
        if context.high_quality_mode {
            let quality_multiplier = feature_config.high_quality_multiplier;
            breakdown.quality_premium = ((breakdown.input_cost + breakdown.output_cost) as f64 * (quality_multiplier - 1.0)) as i32;
        }
        
        // Calculate total before optimization
        let pre_optimization_total = breakdown.input_cost + breakdown.output_cost + 
            breakdown.feature_overhead + breakdown.quality_premium + 
            breakdown.context_cost + breakdown.processing_cost;
        
        // Apply optimization savings
        let optimization_savings = self.optimization_engine
            .calculate_savings(feature, context, pre_optimization_total)
            .await?;
        
        let total_credits = (pre_optimization_total - optimization_savings)
            .max(feature_config.minimum_cost);
        
        // Apply maximum cost limit if set
        let final_total = if let Some(max_cost) = feature_config.maximum_cost {
            total_credits.min(max_cost)
        } else {
            total_credits
        };
        
        // Calculate confidence level based on estimation accuracy
        let confidence_level = self.calculate_confidence_level(context, feature);
        
        Ok(CostEstimate {
            feature: feature.to_string(),
            model: context.model.clone(),
            estimated_input_tokens: input_tokens,
            estimated_output_tokens: output_tokens,
            base_credits: pre_optimization_total,
            quality_adjustment: breakdown.quality_premium,
            optimization_savings,
            total_credits: final_total,
            confidence_level,
            breakdown,
        })
    }
    
    async fn estimate_input_tokens(&self, context: &CostEstimationContext) -> Result<usize> {
        let mut total_tokens = 0;
        
        // System prompt tokens
        total_tokens += self.estimate_text_tokens(&context.system_prompt);
        
        // User prompt tokens
        total_tokens += self.estimate_text_tokens(&context.user_prompt);
        
        // Story Bible context tokens
        if let Some(story_bible) = &context.story_bible_context {
            total_tokens += self.estimate_story_bible_tokens(story_bible);
        }
        
        // Document context tokens
        if let Some(doc_context) = &context.document_context {
            total_tokens += self.estimate_text_tokens(doc_context);
        }
        
        // Selected text tokens
        if let Some(selected_text) = &context.selected_text {
            total_tokens += self.estimate_text_tokens(selected_text);
        }
        
        Ok(total_tokens)
    }
    
    async fn estimate_output_tokens(&self, feature: &str, context: &CostEstimationContext) -> Result<usize> {
        match feature {
            "write" => {
                // Estimate based on requested length or default
                let base_tokens = context.expected_output_length.unwrap_or(200);
                Ok(base_tokens)
            },
            "rewrite" => {
                // Usually similar length to input
                let input_length = context.selected_text.as_ref()
                    .map(|text| self.estimate_text_tokens(text))
                    .unwrap_or(100);
                Ok((input_length as f32 * 1.2) as usize) // 20% longer on average
            },
            "expand" => {
                // Typically 2-3x the original length
                let input_length = context.selected_text.as_ref()
                    .map(|text| self.estimate_text_tokens(text))
                    .unwrap_or(50);
                Ok(input_length * 3)
            },
            "describe" => {
                // Usually adds 100-300 tokens of description
                Ok(200)
            },
            "visualize" => {
                // Image generation doesn't produce text tokens
                Ok(0)
            },
            _ => Ok(150) // Default estimate
        }
    }
    
    fn calculate_input_cost(&self, tokens: usize, model_config: &ModelCostConfig) -> i32 {
        let cost_usd = tokens as f64 * model_config.cost_per_input_token;
        let cost_usd = cost_usd.max(model_config.minimum_charge);
        self.usd_to_credits(cost_usd)
    }
    
    fn calculate_output_cost(&self, tokens: usize, model_config: &ModelCostConfig) -> i32 {
        let cost_usd = tokens as f64 * model_config.cost_per_output_token;
        self.usd_to_credits(cost_usd)
    }
    
    fn calculate_context_cost(&self, context: &CostEstimationContext) -> i32 {
        // Additional cost for complex context assembly
        let mut cost = 0;
        
        if context.story_bible_context.is_some() {
            cost += 5; // Story Bible processing overhead
        }
        
        if context.high_quality_mode {
            cost += 10; // High quality processing overhead
        }
        
        cost
    }
    
    fn calculate_processing_cost(&self, feature: &str, context: &CostEstimationContext) -> i32 {
        match feature {
            "visualize" => 50, // Image processing overhead
            "write" if context.high_quality_mode => 15,
            "expand" => 8,
            _ => 5
        }
    }
    
    fn calculate_confidence_level(&self, context: &CostEstimationContext, feature: &str) -> f32 {
        let mut confidence = 0.8; // Base confidence
        
        // Reduce confidence for complex features
        match feature {
            "write" => confidence -= 0.1,
            "expand" => confidence -= 0.15,
            "visualize" => confidence -= 0.2,
            _ => {}
        }
        
        // Reduce confidence for high variability contexts
        if context.story_bible_context.is_some() {
            confidence -= 0.05;
        }
        
        if context.high_quality_mode {
            confidence -= 0.1;
        }
        
        confidence.max(0.5).min(0.95)
    }
    
    fn estimate_text_tokens(&self, text: &str) -> usize {
        // Rough estimation: ~4 characters per token for English
        (text.len() as f32 / 4.0).ceil() as usize
    }
    
    fn estimate_story_bible_tokens(&self, story_bible: &StoryBibleContext) -> usize {
        let mut tokens = 0;
        
        // Character tokens
        for character in &story_bible.characters {
            tokens += self.estimate_text_tokens(&character.name);
            if let Some(desc) = &character.description {
                tokens += self.estimate_text_tokens(desc);
            }
            // Estimate trait tokens
            tokens += character.traits.len() * 10; // Average trait size
        }
        
        // Worldbuilding tokens
        for element in &story_bible.worldbuilding {
            tokens += self.estimate_text_tokens(&element.name);
            if let Some(desc) = &element.description {
                tokens += self.estimate_text_tokens(desc);
            }
        }
        
        tokens
    }
    
    fn usd_to_credits(&self, usd_amount: f64) -> i32 {
        // Convert USD to credits (example: $0.01 = 100 credits)
        (usd_amount * 10000.0).round() as i32
    }
}

#[derive(Debug, Clone)]
pub struct CostEstimationContext {
    pub model: String,
    pub system_prompt: String,
    pub user_prompt: String,
    pub selected_text: Option<String>,
    pub document_context: Option<String>,
    pub story_bible_context: Option<StoryBibleContext>,
    pub high_quality_mode: bool,
    pub expected_output_length: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct StoryBibleContext {
    pub characters: Vec<CharacterContext>,
    pub worldbuilding: Vec<WorldbuildingContext>,
}

#[derive(Debug, Clone)]
pub struct CharacterContext {
    pub name: String,
    pub description: Option<String>,
    pub traits: HashMap<String, String>,
    pub relevance_score: f32,
}

#[derive(Debug, Clone)]
pub struct WorldbuildingContext {
    pub name: String,
    pub description: Option<String>,
    pub relevance_score: f32,
}
```

### Usage Optimization Engine

#### Automatic Cost Reduction
```rust
#[derive(Debug, Clone)]
pub struct CostOptimizationEngine {
    optimization_strategies: Vec<Box<dyn CostOptimizationStrategy + Send + Sync>>,
    user_preferences: UserOptimizationPreferences,
    historical_data: Arc<tokio::sync::RwLock<OptimizationHistory>>,
}

#[derive(Debug, Clone)]
pub struct UserOptimizationPreferences {
    pub auto_optimize: bool,
    pub max_cost_increase_for_quality: f32, // Percentage
    pub preferred_models: Vec<String>,
    pub avoid_high_cost_features: bool,
    pub optimization_aggressiveness: OptimizationLevel,
}

#[derive(Debug, Clone)]
pub enum OptimizationLevel {
    Conservative, // Minimal optimization, preserve quality
    Balanced,     // Balance cost and quality
    Aggressive,   // Maximum cost reduction
}

impl CostOptimizationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            optimization_strategies: Vec::new(),
            user_preferences: UserOptimizationPreferences::default(),
            historical_data: Arc::new(tokio::sync::RwLock::new(OptimizationHistory::new())),
        };
        
        engine.initialize_strategies();
        engine
    }
    
    fn initialize_strategies(&mut self) {
        self.optimization_strategies.push(Box::new(ModelSelectionStrategy::new()));
        self.optimization_strategies.push(Box::new(ContextOptimizationStrategy::new()));
        self.optimization_strategies.push(Box::new(BatchingStrategy::new()));
        self.optimization_strategies.push(Box::new(CachingStrategy::new()));
    }
    
    pub async fn calculate_savings(
        &self,
        feature: &str,
        context: &CostEstimationContext,
        original_cost: i32,
    ) -> Result<i32> {
        if !self.user_preferences.auto_optimize {
            return Ok(0);
        }
        
        let mut total_savings = 0;
        let mut optimization_context = OptimizationContext {
            feature: feature.to_string(),
            original_cost,
            context: context.clone(),
            applied_optimizations: Vec::new(),
        };
        
        for strategy in &self.optimization_strategies {
            let savings = strategy.calculate_savings(&optimization_context).await?;
            if savings > 0 {
                total_savings += savings;
                optimization_context.applied_optimizations.push(AppliedOptimization {
                    strategy_name: strategy.get_name().to_string(),
                    savings,
                    description: strategy.get_description(&optimization_context).await?,
                });
            }
        }
        
        // Record optimization results
        self.record_optimization_result(&optimization_context, total_savings).await?;
        
        Ok(total_savings)
    }
    
    pub async fn suggest_optimizations(
        &self,
        feature: &str,
        context: &CostEstimationContext,
        current_cost: i32,
    ) -> Result<Vec<OptimizationSuggestion>> {
        let mut suggestions = Vec::new();
        
        let optimization_context = OptimizationContext {
            feature: feature.to_string(),
            original_cost: current_cost,
            context: context.clone(),
            applied_optimizations: Vec::new(),
        };
        
        for strategy in &self.optimization_strategies {
            if let Some(suggestion) = strategy.suggest_optimization(&optimization_context).await? {
                suggestions.push(suggestion);
            }
        }
        
        // Sort by potential savings
        suggestions.sort_by(|a, b| b.potential_savings.cmp(&a.potential_savings));
        
        Ok(suggestions)
    }
    
    async fn record_optimization_result(
        &self,
        context: &OptimizationContext,
        total_savings: i32,
    ) -> Result<()> {
        let mut history = self.historical_data.write().await;
        history.record_optimization(OptimizationRecord {
            feature: context.feature.clone(),
            original_cost: context.original_cost,
            savings: total_savings,
            optimizations_applied: context.applied_optimizations.clone(),
            timestamp: chrono::Utc::now(),
            user_satisfaction: None, // To be filled later by user feedback
        });
        
        Ok(())
    }
}

#[async_trait::async_trait]
pub trait CostOptimizationStrategy {
    async fn calculate_savings(&self, context: &OptimizationContext) -> Result<i32>;
    async fn suggest_optimization(&self, context: &OptimizationContext) -> Result<Option<OptimizationSuggestion>>;
    async fn get_description(&self, context: &OptimizationContext) -> Result<String>;
    fn get_name(&self) -> &str;
}

pub struct ModelSelectionStrategy {
    model_performance_data: HashMap<String, ModelPerformanceData>,
}

impl ModelSelectionStrategy {
    pub fn new() -> Self {
        let mut strategy = Self {
            model_performance_data: HashMap::new(),
        };
        strategy.initialize_performance_data();
        strategy
    }
    
    fn initialize_performance_data(&mut self) {
        self.model_performance_data.insert("gpt-4".to_string(), ModelPerformanceData {
            quality_score: 0.95,
            cost_efficiency: 0.6,
            speed_score: 0.7,
            suitable_features: vec!["write".to_string(), "expand".to_string(), "rewrite".to_string()],
        });
        
        self.model_performance_data.insert("gpt-3.5-turbo".to_string(), ModelPerformanceData {
            quality_score: 0.8,
            cost_efficiency: 0.9,
            speed_score: 0.9,
            suitable_features: vec!["rewrite".to_string(), "quick-edit".to_string()],
        });
        
        self.model_performance_data.insert("claude-3-haiku".to_string(), ModelPerformanceData {
            quality_score: 0.85,
            cost_efficiency: 0.95,
            speed_score: 0.95,
            suitable_features: vec!["rewrite".to_string(), "expand".to_string()],
        });
    }
}

#[async_trait::async_trait]
impl CostOptimizationStrategy for ModelSelectionStrategy {
    async fn calculate_savings(&self, context: &OptimizationContext) -> Result<i32> {
        let current_model = &context.context.model;
        let optimal_model = self.find_optimal_model(&context.feature, context.context.high_quality_mode);
        
        if optimal_model != *current_model {
            // Calculate cost difference
            let current_cost = context.original_cost;
            let optimal_cost = self.estimate_cost_with_model(&optimal_model, context).await?;
            
            if optimal_cost < current_cost {
                return Ok(current_cost - optimal_cost);
            }
        }
        
        Ok(0)
    }
    
    async fn suggest_optimization(&self, context: &OptimizationContext) -> Result<Option<OptimizationSuggestion>> {
        let current_model = &context.context.model;
        let optimal_model = self.find_optimal_model(&context.feature, context.context.high_quality_mode);
        
        if optimal_model != *current_model {
            let savings = self.calculate_savings(context).await?;
            if savings > 0 {
                return Ok(Some(OptimizationSuggestion {
                    title: "Switch to more cost-effective model".to_string(),
                    description: format!(
                        "Switch from {} to {} for this task. This model provides similar quality at lower cost.",
                        current_model, optimal_model
                    ),
                    potential_savings: savings,
                    quality_impact: self.calculate_quality_impact(current_model, &optimal_model),
                    implementation_effort: ImplementationEffort::Low,
                    category: OptimizationCategory::ModelSelection,
                }));
            }
        }
        
        Ok(None)
    }
    
    async fn get_description(&self, context: &OptimizationContext) -> Result<String> {
        Ok("Automatically selected more cost-effective model while maintaining quality".to_string())
    }
    
    fn get_name(&self) -> &str {
        "ModelSelection"
    }
}

impl ModelSelectionStrategy {
    fn find_optimal_model(&self, feature: &str, high_quality_mode: bool) -> String {
        let mut best_model = "gpt-3.5-turbo".to_string();
        let mut best_score = 0.0f32;
        
        for (model_name, performance_data) in &self.model_performance_data {
            if !performance_data.suitable_features.contains(&feature.to_string()) {
                continue;
            }
            
            let quality_weight = if high_quality_mode { 0.7 } else { 0.4 };
            let cost_weight = if high_quality_mode { 0.3 } else { 0.6 };
            
            let score = (performance_data.quality_score * quality_weight) + 
                       (performance_data.cost_efficiency * cost_weight);
            
            if score > best_score {
                best_score = score;
                best_model = model_name.clone();
            }
        }
        
        best_model
    }
    
    async fn estimate_cost_with_model(&self, model: &str, context: &OptimizationContext) -> Result<i32> {
        // Simplified cost estimation for different model
        let cost_multiplier = match model {
            "gpt-4" => 1.0,
            "gpt-3.5-turbo" => 0.1,
            "claude-3-haiku" => 0.15,
            _ => 1.0,
        };
        
        Ok((context.original_cost as f32 * cost_multiplier) as i32)
    }
    
    fn calculate_quality_impact(&self, current_model: &str, new_model: &str) -> QualityImpact {
        let current_quality = self.model_performance_data.get(current_model)
            .map(|data| data.quality_score)
            .unwrap_or(0.8);
        
        let new_quality = self.model_performance_data.get(new_model)
            .map(|data| data.quality_score)
            .unwrap_or(0.8);
        
        let difference = new_quality - current_quality;
        
        if difference > 0.05 {
            QualityImpact::Positive
        } else if difference < -0.05 {
            QualityImpact::Negative
        } else {
            QualityImpact::Neutral
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationContext {
    pub feature: String,
    pub original_cost: i32,
    pub context: CostEstimationContext,
    pub applied_optimizations: Vec<AppliedOptimization>,
}

#[derive(Debug, Clone)]
pub struct AppliedOptimization {
    pub strategy_name: String,
    pub savings: i32,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    pub title: String,
    pub description: String,
    pub potential_savings: i32,
    pub quality_impact: QualityImpact,
    pub implementation_effort: ImplementationEffort,
    pub category: OptimizationCategory,
}

#[derive(Debug, Clone)]
pub enum QualityImpact {
    Positive,
    Neutral,
    Negative,
}

#[derive(Debug, Clone)]
pub enum ImplementationEffort {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub enum OptimizationCategory {
    ModelSelection,
    ContextOptimization,
    Batching,
    Caching,
    FeatureSelection,
}

#[derive(Debug, Clone)]
pub struct ModelPerformanceData {
    pub quality_score: f32,
    pub cost_efficiency: f32,
    pub speed_score: f32,
    pub suitable_features: Vec<String>,
}
```

### Balance Management System

#### Low Balance Warnings and Usage Analytics
```rust
#[derive(Debug, Clone)]
pub struct BalanceManager {
    current_balance: Arc<tokio::sync::RwLock<i32>>,
    usage_analytics: Arc<UsageAnalytics>,
    warning_thresholds: WarningThresholds,
    spending_controls: SpendingControls,
    notification_service: Arc<NotificationService>,
}

#[derive(Debug, Clone)]
pub struct WarningThresholds {
    pub low_balance_warning: i32,      // Warn when balance drops below this
    pub critical_balance_warning: i32, // Critical warning threshold
    pub daily_spending_warning: i32,   // Warn if daily spending exceeds this
    pub feature_cost_warning: i32,     // Warn before expensive operations
}

#[derive(Debug, Clone)]
pub struct SpendingControls {
    pub daily_spending_limit: Option<i32>,
    pub per_feature_limits: HashMap<String, i32>,
    pub require_confirmation_above: i32,
    pub auto_pause_at_balance: i32,
    pub emergency_reserve: i32,
}

impl BalanceManager {
    pub fn new() -> Self {
        Self {
            current_balance: Arc::new(tokio::sync::RwLock::new(10000)), // Default starting balance
            usage_analytics: Arc::new(UsageAnalytics::new()),
            warning_thresholds: WarningThresholds {
                low_balance_warning: 1000,
                critical_balance_warning: 100,
                daily_spending_warning: 500,
                feature_cost_warning: 100,
            },
            spending_controls: SpendingControls {
                daily_spending_limit: Some(1000),
                per_feature_limits: HashMap::new(),
                require_confirmation_above: 200,
                auto_pause_at_balance: 50,
                emergency_reserve: 100,
            },
            notification_service: Arc::new(NotificationService::new()),
        }
    }
    
    pub async fn check_balance(&self) -> i32 {
        *self.current_balance.read().await
    }
    
    pub async fn can_afford(&self, cost: i32) -> Result<AffordabilityCheck> {
        let current_balance = *self.current_balance.read().await;
        let daily_spending = self.usage_analytics.get_daily_spending().await?;
        
        let mut warnings = Vec::new();
        let mut blocks = Vec::new();
        
        // Check if user can afford the operation
        let available_balance = current_balance - self.spending_controls.emergency_reserve;
        if cost > available_balance {
            blocks.push(AffordabilityBlock {
                reason: "Insufficient balance".to_string(),
                required_balance: cost,
                current_balance: available_balance,
                suggested_action: "Add credits to your account".to_string(),
            });
        }
        
        // Check daily spending limits
        if let Some(daily_limit) = self.spending_controls.daily_spending_limit {
            if daily_spending + cost > daily_limit {
                blocks.push(AffordabilityBlock {
                    reason: "Daily spending limit exceeded".to_string(),
                    required_balance: 0,
                    current_balance: daily_limit - daily_spending,
                    suggested_action: "Wait until tomorrow or increase daily limit".to_string(),
                });
            }
        }
        
        // Generate warnings
        if current_balance - cost < self.warning_thresholds.low_balance_warning {
            warnings.push(BalanceWarning {
                warning_type: WarningType::LowBalance,
                message: format!("Your balance will be {} credits after this operation", current_balance - cost),
                severity: if current_balance - cost < self.warning_thresholds.critical_balance_warning {
                    WarningSeverity::Critical
                } else {
                    WarningSeverity::Medium
                },
                suggested_action: Some("Consider adding more credits".to_string()),
            });
        }
        
        if cost > self.warning_thresholds.feature_cost_warning {
            warnings.push(BalanceWarning {
                warning_type: WarningType::HighCost,
                message: format!("This operation will cost {} credits", cost),
                severity: WarningSeverity::Medium,
                suggested_action: Some("Consider using a lower-cost alternative".to_string()),
            });
        }
        
        Ok(AffordabilityCheck {
            can_afford: blocks.is_empty(),
            warnings,
            blocks,
            requires_confirmation: cost > self.spending_controls.require_confirmation_above,
        })
    }
    
    pub async fn consume_credits(&self, cost: i32, feature: &str, description: &str) -> Result<CreditTransaction> {
        // Check affordability first
        let affordability = self.can_afford(cost).await?;
        if !affordability.can_afford {
            return Err(anyhow::anyhow!("Cannot afford operation: {:?}", affordability.blocks));
        }
        
        // Deduct credits
        let mut balance = self.current_balance.write().await;
        let previous_balance = *balance;
        *balance -= cost;
        let new_balance = *balance;
        
        // Record transaction
        let transaction = CreditTransaction {
            id: uuid::Uuid::new_v4().to_string(),
            amount: -cost,
            feature: feature.to_string(),
            description: description.to_string(),
            previous_balance,
            new_balance,
            timestamp: chrono::Utc::now(),
            transaction_type: TransactionType::Consumption,
        };
        
        // Update analytics
        self.usage_analytics.record_usage(cost, feature).await?;
        
        // Check for post-transaction warnings
        self.check_post_transaction_warnings(new_balance, cost, feature).await?;
        
        Ok(transaction)
    }
    
    pub async fn add_credits(&self, amount: i32, source: &str) -> Result<CreditTransaction> {
        let mut balance = self.current_balance.write().await;
        let previous_balance = *balance;
        *balance += amount;
        let new_balance = *balance;
        
        let transaction = CreditTransaction {
            id: uuid::Uuid::new_v4().to_string(),
            amount,
            feature: "credit_purchase".to_string(),
            description: format!("Credits added from {}", source),
            previous_balance,
            new_balance,
            timestamp: chrono::Utc::now(),
            transaction_type: TransactionType::Addition,
        };
        
        // Send notification if balance was previously low
        if previous_balance < self.warning_thresholds.low_balance_warning {
            self.notification_service.send_notification(Notification {
                title: "Credits Added".to_string(),
                message: format!("Your balance has been increased to {} credits", new_balance),
                notification_type: NotificationType::BalanceUpdate,
                timestamp: chrono::Utc::now(),
            }).await?;
        }
        
        Ok(transaction)
    }
    
    async fn check_post_transaction_warnings(&self, new_balance: i32, cost: i32, feature: &str) -> Result<()> {
        let mut notifications = Vec::new();
        
        // Critical balance warning
        if new_balance <= self.warning_thresholds.critical_balance_warning {
            notifications.push(Notification {
                title: "Critical Balance Warning".to_string(),
                message: format!("Your balance is critically low: {} credits remaining", new_balance),
                notification_type: NotificationType::CriticalWarning,
                timestamp: chrono::Utc::now(),
            });
        }
        // Low balance warning
        else if new_balance <= self.warning_thresholds.low_balance_warning {
            notifications.push(Notification {
                title: "Low Balance Warning".to_string(),
                message: format!("Your balance is getting low: {} credits remaining", new_balance),
                notification_type: NotificationType::Warning,
                timestamp: chrono::Utc::now(),
            });
        }
        
        // Auto-pause check
        if new_balance <= self.spending_controls.auto_pause_at_balance {
            notifications.push(Notification {
                title: "Account Paused".to_string(),
                message: "Your account has been automatically paused due to low balance. Please add credits to continue.".to_string(),
                notification_type: NotificationType::AccountPaused,
                timestamp: chrono::Utc::now(),
            });
        }
        
        // Send all notifications
        for notification in notifications {
            self.notification_service.send_notification(notification).await?;
        }
        
        Ok(())
    }
    
    pub async fn get_usage_analytics(&self, period: AnalyticsPeriod) -> Result<UsageReport> {
        self.usage_analytics.generate_report(period).await
    }
    
    pub async fn get_spending_forecast(&self) -> Result<SpendingForecast> {
        let daily_average = self.usage_analytics.get_daily_average_spending().await?;
        let current_balance = self.check_balance().await;
        
        let days_remaining = if daily_average > 0 {
            current_balance / daily_average
        } else {
            365 // If no spending, balance lasts a year
        };
        
        let weekly_forecast = self.usage_analytics.get_weekly_forecast().await?;
        let monthly_forecast = self.usage_analytics.get_monthly_forecast().await?;
        
        Ok(SpendingForecast {
            current_balance,
            daily_average_spending: daily_average,
            estimated_days_remaining: days_remaining,
            weekly_forecast,
            monthly_forecast,
            recommendations: self.generate_spending_recommendations(daily_average, current_balance).await?,
        })
    }
    
    async fn generate_spending_recommendations(&self, daily_average: i32, current_balance: i32) -> Result<Vec<SpendingRecommendation>> {
        let mut recommendations = Vec::new();
        
        // Low balance recommendations
        if current_balance < daily_average * 7 {
            recommendations.push(SpendingRecommendation {
                title: "Add Credits Soon".to_string(),
                description: "Your current balance will last less than a week at current usage".to_string(),
                priority: RecommendationPriority::High,
                estimated_savings: 0,
                action_required: true,
            });
        }
        
        // High spending recommendations
        if daily_average > 100 {
            recommendations.push(SpendingRecommendation {
                title: "Consider Cost Optimization".to_string(),
                description: "Your daily spending is above average. Enable auto-optimization to reduce costs.".to_string(),
                priority: RecommendationPriority::Medium,
                estimated_savings: daily_average / 4, // Estimate 25% savings
                action_required: false,
            });
        }
        
        // Feature-specific recommendations
        let feature_usage = self.usage_analytics.get_feature_usage_breakdown().await?;
        if let Some(highest_cost_feature) = feature_usage.iter().max_by_key(|(_, cost)| *cost) {
            if *highest_cost_feature.1 > daily_average / 2 {
                recommendations.push(SpendingRecommendation {
                    title: format!("Optimize {} Usage", highest_cost_feature.0),
                    description: format!("The {} feature accounts for most of your spending", highest_cost_feature.0),
                    priority: RecommendationPriority::Medium,
                    estimated_savings: highest_cost_feature.1 / 3,
                    action_required: false,
                });
            }
        }
        
        Ok(recommendations)
    }
}

#[derive(Debug, Clone)]
pub struct AffordabilityCheck {
    pub can_afford: bool,
    pub warnings: Vec<BalanceWarning>,
    pub blocks: Vec<AffordabilityBlock>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone)]
pub struct BalanceWarning {
    pub warning_type: WarningType,
    pub message: String,
    pub severity: WarningSeverity,
    pub suggested_action: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AffordabilityBlock {
    pub reason: String,
    pub required_balance: i32,
    pub current_balance: i32,
    pub suggested_action: String,
}

#[derive(Debug, Clone)]
pub enum WarningType {
    LowBalance,
    CriticalBalance,
    HighCost,
    DailyLimitApproaching,
    FeatureLimitExceeded,
}

#[derive(Debug, Clone)]
pub enum WarningSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct CreditTransaction {
    pub id: String,
    pub amount: i32,
    pub feature: String,
    pub description: String,
    pub previous_balance: i32,
    pub new_balance: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub transaction_type: TransactionType,
}

#[derive(Debug, Clone)]
pub enum TransactionType {
    Consumption,
    Addition,
    Refund,
    Adjustment,
}

#[derive(Debug, Clone)]
pub struct SpendingForecast {
    pub current_balance: i32,
    pub daily_average_spending: i32,
    pub estimated_days_remaining: i32,
    pub weekly_forecast: Vec<i32>,
    pub monthly_forecast: Vec<i32>,
    pub recommendations: Vec<SpendingRecommendation>,
}

#[derive(Debug, Clone)]
pub struct SpendingRecommendation {
    pub title: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub estimated_savings: i32,
    pub action_required: bool,
}

#[derive(Debug, Clone)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum AnalyticsPeriod {
    Daily,
    Weekly,
    Monthly,
    AllTime,
}

#[derive(Debug, Clone)]
pub struct UsageReport {
    pub period: AnalyticsPeriod,
    pub total_credits_used: i32,
    pub feature_breakdown: HashMap<String, i32>,
    pub daily_usage: Vec<DailyUsage>,
    pub average_daily_spending: i32,
    pub most_expensive_day: Option<chrono::NaiveDate>,
    pub cost_trends: Vec<CostTrend>,
}

#[derive(Debug, Clone)]
pub struct DailyUsage {
    pub date: chrono::NaiveDate,
    pub credits_used: i32,
    pub feature_breakdown: HashMap<String, i32>,
}

#[derive(Debug, Clone)]
pub struct CostTrend {
    pub period: String,
    pub credits_used: i32,
    pub change_from_previous: f32, // Percentage change
}
```

---

## 6. Missing Critical Workflow Components

### Error Handling & Recovery Workflows

#### Comprehensive Error Recovery Logic
```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIOperationError {
    NetworkTimeout {
        duration: Duration,
        retry_after: Option<Duration>,
    },
    APIRateLimit {
        reset_time: chrono::DateTime<chrono::Utc>,
        remaining_requests: Option<u32>,
    },
    InsufficientCredits {
        required: i32,
        available: i32,
        suggested_action: String,
    },
    ContentFiltered {
        reason: String,
        filtered_content: Option<String>,
        suggestions: Vec<String>,
    },
    ModelUnavailable {
        model_name: String,
        alternative_models: Vec<String>,
        estimated_availability: Option<chrono::DateTime<chrono::Utc>>,
    },
    ContextTooLarge {
        current_tokens: usize,
        max_tokens: usize,
        optimization_suggestions: Vec<String>,
    },
    InvalidInput {
        field: String,
        reason: String,
        valid_examples: Vec<String>,
    },
    ServiceDegraded {
        affected_features: Vec<String>,
        estimated_resolution: Option<chrono::DateTime<chrono::Utc>>,
    },
}

#[derive(Debug, Clone)]
pub struct ErrorRecoveryStrategy {
    pub retry_logic: RetryConfig,
    pub fallback_models: Vec<String>,
    pub user_notification: NotificationStrategy,
    pub graceful_degradation: DegradationOptions,
    pub automatic_fixes: Vec<AutomaticFix>,
}

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter: bool,
    pub retry_conditions: Vec<RetryCondition>,
}

#[derive(Debug, Clone)]
pub enum RetryCondition {
    NetworkTimeout,
    RateLimit,
    ServiceUnavailable,
    TemporaryFailure,
}

#[derive(Debug, Clone)]
pub struct NotificationStrategy {
    pub immediate_feedback: bool,
    pub progress_updates: bool,
    pub error_details_level: ErrorDetailLevel,
    pub suggested_actions: bool,
    pub escalation_threshold: u32,
}

#[derive(Debug, Clone)]
pub enum ErrorDetailLevel {
    Minimal,    // "Something went wrong"
    Basic,      // "Network error occurred"
    Detailed,   // "Connection timeout after 30s"
    Technical,  // Full error details for debugging
}

#[derive(Debug, Clone)]
pub struct DegradationOptions {
    pub fallback_to_cached_results: bool,
    pub reduce_quality_for_speed: bool,
    pub disable_non_essential_features: bool,
    pub offline_mode_available: bool,
    pub partial_results_acceptable: bool,
}

#[derive(Debug, Clone)]
pub struct AutomaticFix {
    pub error_pattern: String,
    pub fix_description: String,
    pub fix_function: String, // Function name to call
    pub requires_user_consent: bool,
    pub success_rate: f32,
}

impl ErrorRecoveryManager {
    pub async fn handle_error(
        &self,
        error: AIOperationError,
        context: &OperationContext,
    ) -> Result<RecoveryResult> {
        let strategy = self.get_recovery_strategy(&error, context);
        
        // Log error for analytics
        self.log_error(&error, context).await?;
        
        // Attempt automatic recovery
        let recovery_result = match error {
            AIOperationError::NetworkTimeout { duration, retry_after } => {
                self.handle_network_timeout(duration, retry_after, &strategy, context).await?
            },
            AIOperationError::APIRateLimit { reset_time, remaining_requests } => {
                self.handle_rate_limit(reset_time, remaining_requests, &strategy, context).await?
            },
            AIOperationError::InsufficientCredits { required, available, suggested_action } => {
                self.handle_insufficient_credits(required, available, suggested_action, &strategy, context).await?
            },
            AIOperationError::ContentFiltered { reason, filtered_content, suggestions } => {
                self.handle_content_filter(reason, filtered_content, suggestions, &strategy, context).await?
            },
            AIOperationError::ModelUnavailable { model_name, alternative_models, estimated_availability } => {
                self.handle_model_unavailable(model_name, alternative_models, estimated_availability, &strategy, context).await?
            },
            AIOperationError::ContextTooLarge { current_tokens, max_tokens, optimization_suggestions } => {
                self.handle_context_too_large(current_tokens, max_tokens, optimization_suggestions, &strategy, context).await?
            },
            _ => self.handle_generic_error(error, &strategy, context).await?
        };
        
        // Update error statistics
        self.update_error_statistics(&error, &recovery_result).await?;
        
        Ok(recovery_result)
    }
    
    async fn handle_network_timeout(
        &self,
        duration: Duration,
        retry_after: Option<Duration>,
        strategy: &ErrorRecoveryStrategy,
        context: &OperationContext,
    ) -> Result<RecoveryResult> {
        // Notify user immediately
        if strategy.user_notification.immediate_feedback {
            self.notify_user(UserNotification {
                title: "Connection Issue".to_string(),
                message: "Experiencing network connectivity issues. Attempting to reconnect...".to_string(),
                notification_type: NotificationType::Warning,
                actions: vec![
                    NotificationAction {
                        label: "Retry Now".to_string(),
                        action: "retry_operation".to_string(),
                    },
                    NotificationAction {
                        label: "Cancel".to_string(),
                        action: "cancel_operation".to_string(),
                    },
                ],
            }).await?;
        }
        
        // Implement exponential backoff retry
        let mut attempt = 0;
        let mut delay = strategy.retry_logic.base_delay;
        
        while attempt < strategy.retry_logic.max_attempts {
            attempt += 1;
            
            // Wait before retry
            let actual_delay = if let Some(retry_after) = retry_after {
                retry_after.max(delay)
            } else {
                delay
            };
            
            if strategy.user_notification.progress_updates {
                self.notify_user(UserNotification {
                    title: "Retrying Connection".to_string(),
                    message: format!("Attempt {} of {}. Waiting {}s...", 
                        attempt, strategy.retry_logic.max_attempts, actual_delay.as_secs()),
                    notification_type: NotificationType::Info,
                    actions: vec![],
                }).await?;
            }
            
            tokio::time::sleep(actual_delay).await;
            
            // Attempt operation again
            match self.retry_operation(context).await {
                Ok(result) => {
                    self.notify_user(UserNotification {
                        title: "Connection Restored".to_string(),
                        message: "Successfully reconnected and completed operation.".to_string(),
                        notification_type: NotificationType::Success,
                        actions: vec![],
                    }).await?;
                    
                    return Ok(RecoveryResult::Success(result));
                },
                Err(e) if self.is_retryable_error(&e) => {
                    // Continue retrying
                    delay = (delay.as_millis() as f64 * strategy.retry_logic.backoff_multiplier) as u64;
                    delay = Duration::from_millis(delay.min(strategy.retry_logic.max_delay.as_millis() as u64));
                },
                Err(e) => {
                    return Ok(RecoveryResult::Failed(format!("Non-retryable error: {}", e)));
                }
            }
        }
        
        // All retries exhausted, try fallback strategies
        if strategy.graceful_degradation.fallback_to_cached_results {
            if let Some(cached_result) = self.get_cached_result(context).await? {
                self.notify_user(UserNotification {
                    title: "Using Cached Result".to_string(),
                    message: "Unable to connect to AI service. Showing previously generated content.".to_string(),
                    notification_type: NotificationType::Warning,
                    actions: vec![],
                }).await?;
                
                return Ok(RecoveryResult::Fallback(cached_result));
            }
        }
        
        Ok(RecoveryResult::Failed("All retry attempts exhausted".to_string()))
    }
    
    async fn handle_insufficient_credits(
        &self,
        required: i32,
        available: i32,
        suggested_action: String,
        strategy: &ErrorRecoveryStrategy,
        context: &OperationContext,
    ) -> Result<RecoveryResult> {
        // Check if we can optimize the operation to reduce cost
        if let Some(optimized_context) = self.optimize_for_cost(context, available).await? {
            self.notify_user(UserNotification {
                title: "Operation Optimized".to_string(),
                message: format!("Reduced operation cost from {} to {} credits to fit your balance.", 
                    required, optimized_context.estimated_cost),
                notification_type: NotificationType::Info,
                actions: vec![
                    NotificationAction {
                        label: "Proceed".to_string(),
                        action: "proceed_optimized".to_string(),
                    },
                    NotificationAction {
                        label: "Add Credits".to_string(),
                        action: "add_credits".to_string(),
                    },
                ],
            }).await?;
            
            return Ok(RecoveryResult::Modified(optimized_context));
        }
        
        // Offer credit purchase or alternative actions
        self.notify_user(UserNotification {
            title: "Insufficient Credits".to_string(),
            message: format!("This operation requires {} credits, but you only have {}. {}", 
                required, available, suggested_action),
            notification_type: NotificationType::Error,
            actions: vec![
                NotificationAction {
                    label: "Add Credits".to_string(),
                    action: "purchase_credits".to_string(),
                },
                NotificationAction {
                    label: "Use Free Alternative".to_string(),
                    action: "use_free_alternative".to_string(),
                },
                NotificationAction {
                    label: "Cancel".to_string(),
                    action: "cancel_operation".to_string(),
                },
            ],
        }).await?;
        
        Ok(RecoveryResult::RequiresUserAction)
    }
    
    async fn handle_content_filter(
        &self,
        reason: String,
        filtered_content: Option<String>,
        suggestions: Vec<String>,
        strategy: &ErrorRecoveryStrategy,
        context: &OperationContext,
    ) -> Result<RecoveryResult> {
        // Try automatic content sanitization
        if let Some(content) = filtered_content {
            if let Some(sanitized) = self.sanitize_content(&content, &reason).await? {
                self.notify_user(UserNotification {
                    title: "Content Automatically Adjusted".to_string(),
                    message: "Content was automatically modified to meet content guidelines.".to_string(),
                    notification_type: NotificationType::Info,
                    actions: vec![
                        NotificationAction {
                            label: "Accept Changes".to_string(),
                            action: "accept_sanitized".to_string(),
                        },
                        NotificationAction {
                            label: "Edit Manually".to_string(),
                            action: "edit_manually".to_string(),
                        },
                    ],
                }).await?;
                
                return Ok(RecoveryResult::Modified(sanitized));
            }
        }
        
        // Provide user guidance
        let suggestion_text = if suggestions.is_empty() {
            "Please review and modify your content to comply with content guidelines.".to_string()
        } else {
            format!("Suggestions: {}", suggestions.join(", "))
        };
        
        self.notify_user(UserNotification {
            title: "Content Policy Violation".to_string(),
            message: format!("Content was filtered due to: {}. {}", reason, suggestion_text),
            notification_type: NotificationType::Warning,
            actions: vec![
                NotificationAction {
                    label: "Edit Content".to_string(),
                    action: "edit_content".to_string(),
                },
                NotificationAction {
                    label: "Learn More".to_string(),
                    action: "content_policy_help".to_string(),
                },
            ],
        }).await?;
        
        Ok(RecoveryResult::RequiresUserAction)
    }
    
    async fn handle_model_unavailable(
        &self,
        model_name: String,
        alternative_models: Vec<String>,
        estimated_availability: Option<chrono::DateTime<chrono::Utc>>,
        strategy: &ErrorRecoveryStrategy,
        context: &OperationContext,
    ) -> Result<RecoveryResult> {
        // Try fallback models automatically
        for fallback_model in &strategy.fallback_models {
            if alternative_models.contains(fallback_model) {
                self.notify_user(UserNotification {
                    title: "Switching Models".to_string(),
                    message: format!("{} is unavailable. Switching to {} automatically.", 
                        model_name, fallback_model),
                    notification_type: NotificationType::Info,
                    actions: vec![],
                }).await?;
                
                let mut modified_context = context.clone();
                modified_context.model = fallback_model.clone();
                
                match self.retry_operation(&modified_context).await {
                    Ok(result) => return Ok(RecoveryResult::Success(result)),
                    Err(_) => continue, // Try next fallback
                }
            }
        }
        
        // No automatic fallback worked, present options to user
        let availability_text = if let Some(eta) = estimated_availability {
            format!("Expected to be available again at {}", eta.format("%Y-%m-%d %H:%M UTC"))
        } else {
            "Availability unknown".to_string()
        };
        
        let mut actions = vec![
            NotificationAction {
                label: "Wait and Retry".to_string(),
                action: "wait_and_retry".to_string(),
            },
        ];
        
        if !alternative_models.is_empty() {
            actions.push(NotificationAction {
                label: "Choose Alternative".to_string(),
                action: "choose_alternative_model".to_string(),
            });
        }
        
        self.notify_user(UserNotification {
            title: "Model Unavailable".to_string(),
            message: format!("{} is currently unavailable. {}. Available alternatives: {}", 
                model_name, availability_text, alternative_models.join(", ")),
            notification_type: NotificationType::Warning,
            actions,
        }).await?;
        
        Ok(RecoveryResult::RequiresUserAction)
    }
}

#[derive(Debug, Clone)]
pub enum RecoveryResult {
    Success(OperationResult),
    Failed(String),
    Fallback(OperationResult),
    Modified(OperationContext),
    RequiresUserAction,
}

#[derive(Debug, Clone)]
pub struct UserNotification {
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub actions: Vec<NotificationAction>,
}

#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub label: String,
    pub action: String,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Info,
    Warning,
    Error,
    Success,
}
```

### State Synchronization Logic

#### Multi-document State Management
```typescript
interface DocumentState {
  documentId: number;
  content: string;
  lastModified: Date;
  version: number;
  linkedDocuments: number[];
  storyBibleReferences: StoryBibleReference[];
  pendingChanges: PendingChange[];
  conflictMarkers: ConflictMarker[];
}

interface StoryBibleReference {
  elementType: 'character' | 'worldbuilding' | 'plot' | 'theme';
  elementId: number;
  referenceLocation: { start: number; end: number };
  lastSynced: Date;
  needsUpdate: boolean;
}

interface PendingChange {
  changeId: string;
  changeType: 'content' | 'story_bible' | 'link';
  source: 'user' | 'ai' | 'sync';
  timestamp: Date;
  data: any;
  dependencies: string[];
  propagationTargets: number[];
}

interface ConflictMarker {
  conflictId: string;
  conflictType: 'content' | 'story_bible' | 'character_state';
  affectedRange: { start: number; end: number };
  conflictingVersions: ConflictVersion[];
  resolutionStrategy: ConflictResolutionStrategy;
  requiresUserInput: boolean;
}

class StateSynchronizationManager {
  private documentStates: Map<number, DocumentState> = new Map();
  private storyBibleState: StoryBibleState;
  private changeQueue: PendingChange[] = [];
  private conflictResolver: ConflictResolver;
  private propagationEngine: PropagationEngine;
  
  constructor() {
    this.conflictResolver = new ConflictResolver();
    this.propagationEngine = new PropagationEngine();
    this.setupChangeListeners();
  }
  
  public async handleDocumentChange(
    documentId: number,
    change: DocumentChange
  ): Promise<SyncResult> {
    const documentState = this.getDocumentState(documentId);
    
    // Create pending change
    const pendingChange: PendingChange = {
      changeId: `change_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      changeType: 'content',
      source: change.source,
      timestamp: new Date(),
      data: change,
      dependencies: [],
      propagationTargets: this.calculatePropagationTargets(documentId, change)
    };
    
    // Check for conflicts
    const conflicts = await this.detectConflicts(documentId, change);
    if (conflicts.length > 0) {
      return this.handleConflicts(documentId, change, conflicts);
    }
    
    // Apply change locally
    await this.applyLocalChange(documentId, change);
    
    // Queue for propagation
    this.changeQueue.push(pendingChange);
    
    // Trigger propagation
    const propagationResult = await this.propagationEngine.propagateChange(pendingChange);
    
    return {
      success: true,
      changeId: pendingChange.changeId,
      propagatedTo: propagationResult.affectedDocuments,
      conflicts: [],
      requiresUserAction: false
    };
  }
  
  public async handleStoryBibleChange(
    elementType: StoryBibleReference['elementType'],
    elementId: number,
    change: StoryBibleChange
  ): Promise<SyncResult> {
    // Find all documents that reference this Story Bible element
    const affectedDocuments = this.findDocumentsReferencingElement(elementType, elementId);
    
    // Create propagation targets
    const propagationTargets = affectedDocuments.map(docId => ({
      documentId: docId,
      updateType: 'story_bible_sync',
      priority: this.calculateUpdatePriority(docId, elementType, elementId)
    }));
    
    // Apply Story Bible change
    await this.applyStoryBibleChange(elementType, elementId, change);
    
    // Propagate to affected documents
    const propagationResults = await Promise.all(
      propagationTargets.map(target => 
        this.propagateStoryBibleChange(target, elementType, elementId, change)
      )
    );
    
    // Check for conflicts
    const conflicts = propagationResults
      .filter(result => result.conflicts.length > 0)
      .flatMap(result => result.conflicts);
    
    return {
      success: conflicts.length === 0,
      changeId: `story_bible_${elementType}_${elementId}_${Date.now()}`,
      propagatedTo: propagationTargets.map(t => t.documentId),
      conflicts,
      requiresUserAction: conflicts.some(c => c.requiresUserInput)
    };
  }
  
  private async detectConflicts(
    documentId: number,
    change: DocumentChange
  ): Promise<ConflictMarker[]> {
    const conflicts: ConflictMarker[] = [];
    const documentState = this.getDocumentState(documentId);
    
    // Check for concurrent edits
    const concurrentChanges = this.changeQueue.filter(pendingChange => 
      pendingChange.propagationTargets.includes(documentId) &&
      this.changesOverlap(change, pendingChange.data)
    );
    
    for (const concurrentChange of concurrentChanges) {
      conflicts.push({
        conflictId: `conflict_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        conflictType: 'content',
        affectedRange: this.calculateAffectedRange(change, concurrentChange.data),
        conflictingVersions: [
          {
            version: documentState.version,
            content: change.newContent,
            source: change.source,
            timestamp: change.timestamp
          },
          {
            version: documentState.version,
            content: concurrentChange.data.newContent,
            source: concurrentChange.source,
            timestamp: concurrentChange.timestamp
          }
        ],
        resolutionStrategy: this.determineResolutionStrategy(change, concurrentChange.data),
        requiresUserInput: this.requiresUserInput(change, concurrentChange.data)
      });
    }
    
    // Check for Story Bible consistency conflicts
    const storyBibleConflicts = await this.checkStoryBibleConsistency(documentId, change);
    conflicts.push(...storyBibleConflicts);
    
    return conflicts;
  }
  
  private async handleConflicts(
    documentId: number,
    change: DocumentChange,
    conflicts: ConflictMarker[]
  ): Promise<SyncResult> {
    const autoResolvableConflicts = conflicts.filter(c => !c.requiresUserInput);
    const userInputRequired = conflicts.filter(c => c.requiresUserInput);
    
    // Auto-resolve conflicts where possible
    for (const conflict of autoResolvableConflicts) {
      const resolution = await this.conflictResolver.resolveAutomatically(conflict);
      if (resolution.success) {
        await this.applyConflictResolution(documentId, conflict, resolution);
      }
    }
    
    // Mark conflicts requiring user input
    for (const conflict of userInputRequired) {
      await this.markConflictForUserResolution(documentId, conflict);
    }
    
    return {
      success: userInputRequired.length === 0,
      changeId: `conflict_resolution_${Date.now()}`,
      propagatedTo: [],
      conflicts: userInputRequired,
      requiresUserAction: userInputRequired.length > 0
    };
  }
  
  private calculatePropagationTargets(
    sourceDocumentId: number,
    change: DocumentChange
  ): number[] {
    const targets: number[] = [];
    const sourceState = this.getDocumentState(sourceDocumentId);
    
    // Add linked documents
    targets.push(...sourceState.linkedDocuments);
    
    // Add documents with Story Bible references that might be affected
    if (this.changeAffectsStoryBible(change)) {
      const affectedElements = this.extractStoryBibleReferences(change);
      for (const element of affectedElements) {
        const referencingDocs = this.findDocumentsReferencingElement(
          element.elementType, 
          element.elementId
        );
        targets.push(...referencingDocs);
      }
    }
    
    // Remove duplicates and source document
    return [...new Set(targets)].filter(id => id !== sourceDocumentId);
  }
  
  private async propagateStoryBibleChange(
    target: { documentId: number; updateType: string; priority: number },
    elementType: StoryBibleReference['elementType'],
    elementId: number,
    change: StoryBibleChange
  ): Promise<PropagationResult> {
    const documentState = this.getDocumentState(target.documentId);
    const references = documentState.storyBibleReferences.filter(
      ref => ref.elementType === elementType && ref.elementId === elementId
    );
    
    const conflicts: ConflictMarker[] = [];
    const updatedReferences: StoryBibleReference[] = [];
    
    for (const reference of references) {
      // Check if reference location has been modified since last sync
      if (this.hasLocalModifications(target.documentId, reference)) {
        // Create conflict marker
        conflicts.push({
          conflictId: `story_bible_conflict_${Date.now()}`,
          conflictType: 'story_bible',
          affectedRange: reference.referenceLocation,
          conflictingVersions: [
            {
              version: documentState.version,
              content: this.getContentAtRange(target.documentId, reference.referenceLocation),
              source: 'local',
              timestamp: documentState.lastModified
            },
            {
              version: this.storyBibleState.version,
              content: this.getStoryBibleElementContent(elementType, elementId),
              source: 'story_bible',
              timestamp: change.timestamp
            }
          ],
          resolutionStrategy: ConflictResolutionStrategy.PreferStoryBible,
          requiresUserInput: true
        });
      } else {
        // Safe to update
        const updatedReference = await this.updateStoryBibleReference(
          target.documentId,
          reference,
          change
        );
        updatedReferences.push(updatedReference);
      }
    }
    
    return {
      documentId: target.documentId,
      success: conflicts.length === 0,
      updatedReferences,
      conflicts
    };
  }
}

class PropagationEngine {
  public async propagateChange(change: PendingChange): Promise<PropagationResult> {
    const results: DocumentPropagationResult[] = [];
    
    // Sort targets by priority
    const sortedTargets = change.propagationTargets.sort((a, b) => 
      this.getPropagationPriority(a) - this.getPropagationPriority(b)
    );
    
    for (const targetId of sortedTargets) {
      try {
        const result = await this.propagateToDocument(targetId, change);
        results.push(result);
        
        // If critical propagation fails, stop
        if (!result.success && this.isCriticalTarget(targetId, change)) {
          break;
        }
      } catch (error) {
        results.push({
          documentId: targetId,
          success: false,
          error: error.message,
          conflicts: []
        });
      }
    }
    
    return {
      affectedDocuments: results.map(r => r.documentId),
      successfulPropagations: results.filter(r => r.success).length,
      failedPropagations: results.filter(r => !r.success).length,
      totalConflicts: results.reduce((sum, r) => sum + r.conflicts.length, 0),
      results
    };
  }
  
  private async propagateToDocument(
    targetDocumentId: number,
    change: PendingChange
  ): Promise<DocumentPropagationResult> {
    const targetState = this.getDocumentState(targetDocumentId);
    
    // Determine propagation strategy
    const strategy = this.determinePropagationStrategy(change, targetState);
    
    switch (strategy) {
      case PropagationStrategy.DirectUpdate:
        return this.applyDirectUpdate(targetDocumentId, change);
      
      case PropagationStrategy.MergeChanges:
        return this.mergeChanges(targetDocumentId, change);
      
      case PropagationStrategy.ConflictResolution:
        return this.handlePropagationConflict(targetDocumentId, change);
      
      case PropagationStrategy.Skip:
        return {
          documentId: targetDocumentId,
          success: true,
          skipped: true,
          reason: "No propagation needed",
          conflicts: []
        };
      
      default:
        throw new Error(`Unknown propagation strategy: ${strategy}`);
    }
  }
}

enum PropagationStrategy {
  DirectUpdate,
  MergeChanges,
  ConflictResolution,
  Skip
}

enum ConflictResolutionStrategy {
  PreferLocal,
  PreferRemote,
  PreferStoryBible,
  RequireUserInput,
  AutoMerge
}

interface SyncResult {
  success: boolean;
  changeId: string;
  propagatedTo: number[];
  conflicts: ConflictMarker[];
  requiresUserAction: boolean;
}

interface PropagationResult {
  affectedDocuments: number[];
  successfulPropagations: number;
  failedPropagations: number;
  totalConflicts: number;
  results: DocumentPropagationResult[];
}

interface DocumentPropagationResult {
  documentId: number;
  success: boolean;
  error?: string;
  skipped?: boolean;
  reason?: string;
  conflicts: ConflictMarker[];
}
```

### Performance Optimization Workflows

#### Lazy Loading and Caching Strategies
```typescript
interface LazyLoadingManager {
  documentCache: Map<number, CachedDocument>;
  storyBibleCache: Map<number, CachedStoryBible>;
  aiHistoryCache: Map<string, CachedAIHistory>;
  loadingStrategies: Map<string, LoadingStrategy>;
  memoryManager: MemoryManager;
}

interface CachedDocument {
  documentId: number;
  content: string;
  metadata: DocumentMetadata;
  lastAccessed: Date;
  loadPriority: LoadPriority;
  isFullyLoaded: boolean;
  partialContent?: PartialContent;
  dependencies: number[];
}

interface PartialContent {
  loadedRanges: ContentRange[];
  totalSize: number;
  loadedSize: number;
  criticalSections: CriticalSection[];
}

interface LoadingStrategy {
  strategyName: string;
  triggerConditions: TriggerCondition[];
  loadingBehavior: LoadingBehavior;
  cachePolicy: CachePolicy;
  prefetchRules: PrefetchRule[];
}

enum LoadPriority {
  Critical = 1,    // Currently active document
  High = 2,        // Recently accessed or linked
  Medium = 3,      // Story Bible elements in use
  Low = 4,         // Background/prefetch
  Deferred = 5     // Load only when explicitly requested
}

class LazyLoadingManager {
  private documentCache = new Map<number, CachedDocument>();
  private storyBibleCache = new Map<number, CachedStoryBible>();
  private aiHistoryCache = new Map<string, CachedAIHistory>();
  private loadingQueue: LoadingTask[] = [];
  private memoryManager: MemoryManager;
  
  constructor() {
    this.memoryManager = new MemoryManager();
    this.setupLoadingStrategies();
    this.startBackgroundLoader();
  }
  
  public async loadDocument(
    documentId: number,
    loadingContext: LoadingContext
  ): Promise<DocumentLoadResult> {
    // Check cache first
    const cached = this.documentCache.get(documentId);
    if (cached && this.isCacheValid(cached, loadingContext)) {
      this.updateAccessTime(cached);
      return {
        document: cached,
        loadedFromCache: true,
        loadTime: 0
      };
    }
    
    const startTime = performance.now();
    
    // Determine loading strategy
    const strategy = this.selectLoadingStrategy(documentId, loadingContext);
    
    let document: CachedDocument;
    
    switch (strategy.loadingBehavior) {
      case LoadingBehavior.Full:
        document = await this.loadFullDocument(documentId);
        break;
        
      case LoadingBehavior.Partial:
        document = await this.loadPartialDocument(documentId, loadingContext);
        break;
        
      case LoadingBehavior.Progressive:
        document = await this.loadProgressiveDocument(documentId, loadingContext);
        break;
        
      case LoadingBehavior.OnDemand:
        document = await this.createDocumentStub(documentId);
        break;
    }
    
    // Cache the document
    this.cacheDocument(document, strategy.cachePolicy);
    
    // Trigger prefetching if configured
    this.triggerPrefetch(document, strategy.prefetchRules);
    
    const loadTime = performance.now() - startTime;
    
    return {
      document,
      loadedFromCache: false,
      loadTime
    };
  }
  
  private async loadPartialDocument(
    documentId: number,
    context: LoadingContext
  ): Promise<CachedDocument> {
    const metadata = await this.loadDocumentMetadata(documentId);
    
    // Determine critical sections to load
    const criticalSections = this.identifyCriticalSections(metadata, context);
    
    // Load only critical sections initially
    const partialContent: PartialContent = {
      loadedRanges: [],
      totalSize: metadata.contentLength,
      loadedSize: 0,
      criticalSections
    };
    
    let content = '';
    for (const section of criticalSections) {
      const sectionContent = await this.loadContentRange(documentId, section.range);
      content += sectionContent;
      partialContent.loadedRanges.push(section.range);
      partialContent.loadedSize += section.range.end - section.range.start;
    }
    
    return {
      documentId,
      content,
      metadata,
      lastAccessed: new Date(),
      loadPriority: this.calculateLoadPriority(documentId, context),
      isFullyLoaded: false,
      partialContent,
      dependencies: metadata.linkedDocuments
    };
  }
  
  private async loadProgressiveDocument(
    documentId: number,
    context: LoadingContext
  ): Promise<CachedDocument> {
    // Start with document stub
    const document = await this.createDocumentStub(documentId);
    
    // Queue progressive loading tasks
    const progressiveTasks = this.createProgressiveTasks(documentId, context);
    
    // Start loading in background
    this.queueProgressiveTasks(progressiveTasks);
    
    return document;
  }
  
  private identifyCriticalSections(
    metadata: DocumentMetadata,
    context: LoadingContext
  ): CriticalSection[] {
    const sections: CriticalSection[] = [];
    
    // Always load document beginning
    sections.push({
      name: 'document_start',
      range: { start: 0, end: Math.min(1000, metadata.contentLength) },
      priority: LoadPriority.Critical,
      reason: 'Document preview'
    });
    
    // Load around cursor position if provided
    if (context.cursorPosition) {
      const start = Math.max(0, context.cursorPosition - 500);
      const end = Math.min(metadata.contentLength, context.cursorPosition + 500);
      sections.push({
        name: 'cursor_context',
        range: { start, end },
        priority: LoadPriority.Critical,
        reason: 'Current editing context'
      });
    }
    
    // Load sections with Story Bible references
    if (context.includeStoryBibleContext) {
      const storyBibleSections = this.findStoryBibleReferenceSections(metadata);
      sections.push(...storyBibleSections.map(section => ({
        name: 'story_bible_reference',
        range: section.range,
        priority: LoadPriority.High,
        reason: 'Story Bible context'
      })));
    }
    
    // Load recent edit locations
    if (metadata.recentEditLocations) {
      for (const editLocation of metadata.recentEditLocations) {
        sections.push({
          name: 'recent_edit',
          range: {
            start: Math.max(0, editLocation - 200),
            end: Math.min(metadata.contentLength, editLocation + 200)
          },
          priority: LoadPriority.High,
          reason: 'Recent edit context'
        });
      }
    }
    
    return this.mergeCriticalSections(sections);
  }
  
  public async loadStoryBibleElement(
    elementType: 'character' | 'worldbuilding' | 'plot',
    elementId: number,
    context: LoadingContext
  ): Promise<StoryBibleLoadResult> {
    const cacheKey = `${elementType}_${elementId}`;
    const cached = this.storyBibleCache.get(elementId);
    
    if (cached && this.isStoryBibleCacheValid(cached, context)) {
      return {
        element: cached,
        loadedFromCache: true,
        loadTime: 0
      };
    }
    
    const startTime = performance.now();
    
    // Load with relevance-based strategy
    const element = await this.loadStoryBibleWithRelevance(
      elementType,
      elementId,
      context
    );
    
    // Cache with appropriate policy
    this.cacheStoryBibleElement(element);
    
    const loadTime = performance.now() - startTime;
    
    return {
      element,
      loadedFromCache: false,
      loadTime
    };
  }
  
  private async loadStoryBibleWithRelevance(
    elementType: string,
    elementId: number,
    context: LoadingContext
  ): Promise<CachedStoryBible> {
    const baseElement = await this.loadBaseStoryBibleElement(elementType, elementId);
    
    // Calculate relevance to current context
    const relevanceScore = await this.calculateStoryBibleRelevance(
      baseElement,
      context
    );
    
    // Load additional details based on relevance
    if (relevanceScore > 0.7) {
      // High relevance - load full details
      baseElement.fullDetails = await this.loadFullStoryBibleDetails(elementType, elementId);
      baseElement.relationships = await this.loadStoryBibleRelationships(elementType, elementId);
    } else if (relevanceScore > 0.3) {
      // Medium relevance - load summary details
      baseElement.summaryDetails = await this.loadSummaryStoryBibleDetails(elementType, elementId);
    }
    // Low relevance - keep minimal data
    
    return {
      ...baseElement,
      relevanceScore,
      lastAccessed: new Date(),
      loadPriority: this.relevanceToLoadPriority(relevanceScore)
    };
  }
}

class MemoryManager {
  private maxMemoryUsage = 500 * 1024 * 1024; // 500MB
  private currentMemoryUsage = 0;
  private evictionPolicy = EvictionPolicy.LRU;
  
  public checkMemoryPressure(): MemoryPressureLevel {
    const usageRatio = this.currentMemoryUsage / this.maxMemoryUsage;
    
    if (usageRatio > 0.9) return MemoryPressureLevel.Critical;
    if (usageRatio > 0.7) return MemoryPressureLevel.High;
    if (usageRatio > 0.5) return MemoryPressureLevel.Medium;
    return MemoryPressureLevel.Low;
  }
  
  public async evictCacheEntries(targetReduction: number): Promise<EvictionResult> {
    const evictedEntries: EvictedEntry[] = [];
    let freedMemory = 0;
    
    // Get eviction candidates
    const candidates = this.getEvictionCandidates();
    
    for (const candidate of candidates) {
      if (freedMemory >= targetReduction) break;
      
      const entrySize = this.calculateEntrySize(candidate);
      await this.evictEntry(candidate);
      
      evictedEntries.push({
        entryId: candidate.id,
        entryType: candidate.type,
        size: entrySize,
        lastAccessed: candidate.lastAccessed
      });
      
      freedMemory += entrySize;
    }
    
    this.currentMemoryUsage -= freedMemory;
    
    return {
      evictedEntries,
      freedMemory,
      remainingMemoryUsage: this.currentMemoryUsage
    };
  }
  
  private getEvictionCandidates(): CacheEntry[] {
    // Implementation depends on eviction policy
    switch (this.evictionPolicy) {
      case EvictionPolicy.LRU:
        return this.getLRUCandidates();
      case EvictionPolicy.LFU:
        return this.getLFUCandidates();
      case EvictionPolicy.Priority:
        return this.getPriorityCandidates();
      default:
        return this.getLRUCandidates();
    }
  }
}

class BackgroundSyncManager {
  private syncQueue: SyncTask[] = [];
  private isRunning = false;
  private syncInterval = 30000; // 30 seconds
  
  public startBackgroundSync(): void {
    if (this.isRunning) return;
    
    this.isRunning = true;
    this.runSyncLoop();
  }
  
  private async runSyncLoop(): Promise<void> {
    while (this.isRunning) {
      try {
        await this.processSyncQueue();
        await this.performAutoSave();
        await this.performBackup();
        await this.cleanupTempFiles();
        
        await this.sleep(this.syncInterval);
      } catch (error) {
        console.error('Background sync error:', error);
        await this.sleep(5000); // Shorter retry interval on error
      }
    }
  }
  
  private async performAutoSave(): Promise<void> {
    const unsavedDocuments = this.getUnsavedDocuments();
    
    for (const document of unsavedDocuments) {
      if (this.shouldAutoSave(document)) {
        await this.saveDocument(document);
      }
    }
  }
  
  private async performBackup(): Promise<void> {
    const documentsNeedingBackup = this.getDocumentsNeedingBackup();
    
    for (const document of documentsNeedingBackup) {
      await this.createBackup(document);
    }
  }
  
  private shouldAutoSave(document: CachedDocument): boolean {
    const timeSinceLastSave = Date.now() - document.metadata.lastSaved.getTime();
    const timeSinceLastEdit = Date.now() - document.lastAccessed.getTime();
    
    // Auto-save if:
    // - 5 minutes since last save and recent activity
    // - 30 seconds since last edit and no recent activity
    return (timeSinceLastSave > 5 * 60 * 1000 && timeSinceLastEdit < 2 * 60 * 1000) ||
           (timeSinceLastEdit > 30 * 1000 && timeSinceLastEdit < 5 * 60 * 1000);
  }
}

enum LoadingBehavior {
  Full,        // Load entire document
  Partial,     // Load critical sections only
  Progressive, // Load incrementally
  OnDemand     // Load when explicitly requested
}

enum EvictionPolicy {
  LRU,      // Least Recently Used
  LFU,      // Least Frequently Used
  Priority, // Based on load priority
}

enum MemoryPressureLevel {
  Low,
  Medium,
  High,
  Critical
}

interface LoadingContext {
  cursorPosition?: number;
  includeStoryBibleContext: boolean;
  expectedUsage: 'read' | 'edit' | 'ai_generation';
  userPreferences: UserPreferences;
}

interface CriticalSection {
  name: string;
  range: ContentRange;
  priority: LoadPriority;
  reason: string;
}

interface ContentRange {
  start: number;
  end: number;
}
```

### User Experience Workflows

#### Onboarding and Feature Discovery
```typescript
interface OnboardingFlow {
  flowId: string;
  targetUserType: UserType;
  steps: OnboardingStep[];
  currentStep: number;
  completionStatus: CompletionStatus;
  userProgress: UserProgress;
  adaptiveElements: AdaptiveElement[];
}

interface OnboardingStep {
  stepId: string;
  title: string;
  description: string;
  stepType: StepType;
  content: StepContent;
  prerequisites: string[];
  estimatedDuration: number; // in seconds
  isOptional: boolean;
  successCriteria: SuccessCriteria;
  helpResources: HelpResource[];
}

enum UserType {
  FirstTimeWriter,
  ExperiencedWriter,
  TechnicalUser,
  CollaborativeUser,
  PowerUser
}

enum StepType {
  Introduction,
  InteractiveDemo,
  HandsOnPractice,
  FeatureExploration,
  Customization,
  Assessment
}

class OnboardingManager {
  private activeFlows: Map<string, OnboardingFlow> = new Map();
  private userProfiler: UserProfiler;
  private progressTracker: ProgressTracker;
  private adaptationEngine: AdaptationEngine;
  
  constructor() {
    this.userProfiler = new UserProfiler();
    this.progressTracker = new ProgressTracker();
    this.adaptationEngine = new AdaptationEngine();
  }
  
  public async startOnboarding(userId: string): Promise<OnboardingFlow> {
    // Profile the user to determine appropriate flow
    const userProfile = await this.userProfiler.profileNewUser(userId);
    
    // Select appropriate onboarding flow
    const flowTemplate = this.selectOnboardingFlow(userProfile);
    
    // Customize flow based on user profile
    const customizedFlow = await this.customizeFlow(flowTemplate, userProfile);
    
    // Initialize flow state
    const flow: OnboardingFlow = {
      flowId: `onboarding_${userId}_${Date.now()}`,
      targetUserType: userProfile.userType,
      steps: customizedFlow.steps,
      currentStep: 0,
      completionStatus: CompletionStatus.InProgress,
      userProgress: {
        stepsCompleted: 0,
        totalSteps: customizedFlow.steps.length,
        timeSpent: 0,
        strugglingAreas: [],
        strengths: []
      },
      adaptiveElements: []
    };
    
    this.activeFlows.set(userId, flow);
    
    // Start first step
    await this.startStep(userId, flow.steps[0]);
    
    return flow;
  }
  
  public async progressToNextStep(
    userId: string,
    currentStepResult: StepResult
  ): Promise<OnboardingStepResult> {
    const flow = this.activeFlows.get(userId);
    if (!flow) {
      throw new Error('No active onboarding flow found');
    }
    
    const currentStep = flow.steps[flow.currentStep];
    
    // Evaluate step completion
    const evaluation = await this.evaluateStepCompletion(currentStep, currentStepResult);
    
    // Update user progress
    this.progressTracker.updateProgress(userId, currentStep, evaluation);
    
    // Check if step was successful
    if (!evaluation.success) {
      // Provide additional help or alternative approach
      return await this.handleStepFailure(userId, currentStep, evaluation);
    }
    
    // Mark step as completed
    flow.userProgress.stepsCompleted++;
    flow.userProgress.timeSpent += currentStepResult.timeSpent;
    
    // Check for adaptive adjustments
    const adaptations = await this.adaptationEngine.analyzeAndAdapt(
      userId,
      flow,
      currentStepResult
    );
    
    if (adaptations.length > 0) {
      await this.applyAdaptations(flow, adaptations);
    }
    
    // Move to next step or complete onboarding
    if (flow.currentStep < flow.steps.length - 1) {
      flow.currentStep++;
      const nextStep = flow.steps[flow.currentStep];
      await this.startStep(userId, nextStep);
      
      return {
        completed: false,
        nextStep,
        adaptations,
        encouragement: this.generateEncouragement(flow.userProgress)
      };
    } else {
      // Onboarding complete
      flow.completionStatus = CompletionStatus.Completed;
      await this.completeOnboarding(userId, flow);
      
      return {
        completed: true,
        completionSummary: this.generateCompletionSummary(flow),
        nextSteps: this.suggestNextSteps(userId, flow)
      };
    }
  }
  
  private async startStep(userId: string, step: OnboardingStep): Promise<void> {
    // Check prerequisites
    const prerequisitesMet = await this.checkPrerequisites(userId, step.prerequisites);
    if (!prerequisitesMet) {
      throw new Error(`Prerequisites not met for step: ${step.stepId}`);
    }
    
    // Initialize step-specific UI
    await this.initializeStepUI(step);
    
    // Track step start
    this.progressTracker.trackStepStart(userId, step.stepId);
    
    // Show step introduction
    await this.showStepIntroduction(step);
  }
  
  private selectOnboardingFlow(userProfile: UserProfile): OnboardingFlowTemplate {
    switch (userProfile.userType) {
      case UserType.FirstTimeWriter:
        return this.getFirstTimeWriterFlow();
      case UserType.ExperiencedWriter:
        return this.getExperiencedWriterFlow();
      case UserType.TechnicalUser:
        return this.getTechnicalUserFlow();
      case UserType.CollaborativeUser:
        return this.getCollaborativeUserFlow();
      case UserType.PowerUser:
        return this.getPowerUserFlow();
      default:
        return this.getDefaultFlow();
    }
  }
  
  private getFirstTimeWriterFlow(): OnboardingFlowTemplate {
    return {
      name: "First-Time Writer Journey",
      description: "Gentle introduction to writing with AI assistance",
      estimatedDuration: 15 * 60, // 15 minutes
      steps: [
        {
          stepId: "welcome",
          title: "Welcome to StoryWeaver",
          description: "Let's start your writing journey together",
          stepType: StepType.Introduction,
          content: {
            type: "video_and_text",
            videoUrl: "/onboarding/welcome-first-time.mp4",
            text: "StoryWeaver is designed to help you bring your stories to life with the power of AI...",
            interactiveElements: [
              {
                type: "personality_quiz",
                questions: [
                  "What type of stories do you want to write?",
                  "How comfortable are you with technology?",
                  "What's your biggest writing challenge?"
                ]
              }
            ]
          },
          prerequisites: [],
          estimatedDuration: 120,
          isOptional: false,
          successCriteria: {
            type: "completion",
            requirements: ["watched_video", "completed_quiz"]
          },
          helpResources: [
            {
              type: "tooltip",
              content: "Take your time - there's no rush!"
            }
          ]
        },
        {
          stepId: "first_document",
          title: "Create Your First Document",
          description: "Let's create your first story document",
          stepType: StepType.HandsOnPractice,
          content: {
            type: "guided_interaction",
            instructions: [
              "Click the 'New Document' button",
              "Give your story a title",
              "Write your first sentence"
            ],
            highlightElements: ["new-document-btn", "title-input", "editor"],
            successMessage: "Great! You've created your first document!"
          },
          prerequisites: ["welcome"],
          estimatedDuration: 180,
          isOptional: false,
          successCriteria: {
            type: "action_completion",
            requirements: ["document_created", "title_entered", "content_written"]
          },
          helpResources: [
            {
              type: "contextual_help",
              content: "Don't worry about making it perfect - you can always edit later!"
            }
          ]
        },
        {
          stepId: "ai_writing_assistant",
          title: "Meet Your AI Writing Assistant",
          description: "Learn how AI can help improve your writing",
          stepType: StepType.InteractiveDemo,
          content: {
            type: "interactive_demo",
            demoScript: [
              {
                action: "select_text",
                text: "The old house stood on the hill.",
                explanation: "Select any text to see AI suggestions"
              },
              {
                action: "show_menu",
                explanation: "The AI offers different ways to help improve your writing"
              },
              {
                action: "demonstrate_expand",
                explanation: "Watch how 'Expand' adds more detail to your sentence"
              }
            ]
          },
          prerequisites: ["first_document"],
          estimatedDuration: 240,
          isOptional: false,
          successCriteria: {
            type: "interaction_completion",
            requirements: ["watched_demo", "tried_ai_feature"]
          },
          helpResources: []
        }
      ]
    };
  }
  
  private async handleStepFailure(
    userId: string,
    step: OnboardingStep,
    evaluation: StepEvaluation
  ): Promise<OnboardingStepResult> {
    // Identify the specific issue
    const failureReason = evaluation.failureReason;
    
    // Provide targeted help
    const helpStrategy = this.selectHelpStrategy(step, failureReason);
    
    switch (helpStrategy.type) {
      case HelpStrategyType.AdditionalGuidance:
        return {
          completed: false,
          needsHelp: true,
          helpContent: {
            type: "additional_guidance",
            title: "Let's try a different approach",
            content: helpStrategy.content,
            actionButton: "Try Again"
          }
        };
        
      case HelpStrategyType.SimplifiedVersion:
        // Create a simplified version of the step
        const simplifiedStep = await this.createSimplifiedStep(step);
        return {
          completed: false,
          needsHelp: true,
          alternativeStep: simplifiedStep,
          helpContent: {
            type: "simplified_approach",
            title: "Let's make this easier",
            content: "We've simplified this step to help you succeed."
          }
        };
        
      case HelpStrategyType.PersonalizedTutorial:
        return {
          completed: false,
          needsHelp: true,
          helpContent: {
            type: "personalized_tutorial",
            title: "Personal guidance",
            content: await this.generatePersonalizedTutorial(userId, step, failureReason)
          }
        };
        
      case HelpStrategyType.SkipOption:
        return {
          completed: false,
          needsHelp: true,
          helpContent: {
            type: "skip_option",
            title: "Having trouble?",
            content: "You can skip this step for now and come back to it later.",
            actions: [
              { label: "Try Again", action: "retry" },
              { label: "Skip for Now", action: "skip" },
              { label: "Get Help", action: "contact_support" }
            ]
          }
        };
    }
  }
}

class FeatureDiscoveryEngine {
  private userBehaviorTracker: UserBehaviorTracker;
  private featureRecommendationEngine: FeatureRecommendationEngine;
  private contextualHelpSystem: ContextualHelpSystem;
  
  public async analyzeUserBehavior(
    userId: string,
    sessionData: UserSessionData
  ): Promise<FeatureDiscoveryOpportunities> {
    const behaviorPattern = await this.userBehaviorTracker.analyzeBehavior(userId, sessionData);
    
    // Identify unused features that could be helpful
    const unusedFeatures = this.identifyUnusedFeatures(userId, behaviorPattern);
    
    // Find contextual opportunities to introduce features
    const contextualOpportunities = this.findContextualOpportunities(behaviorPattern, unusedFeatures);
    
    // Generate recommendations
    const recommendations = await this.featureRecommendationEngine.generateRecommendations(
      userId,
      behaviorPattern,
      contextualOpportunities
    );
    
    return {
      opportunities: contextualOpportunities,
      recommendations,
      suggestedTiming: this.calculateOptimalTiming(behaviorPattern, recommendations)
    };
  }
  
  public async presentFeatureDiscovery(
    userId: string,
    opportunity: FeatureDiscoveryOpportunity
  ): Promise<FeatureDiscoveryResult> {
    const presentationStrategy = this.selectPresentationStrategy(opportunity);
    
    switch (presentationStrategy.type) {
      case PresentationStrategyType.ContextualTooltip:
        return await this.showContextualTooltip(opportunity);
        
      case PresentationStrategyType.InteractiveDemo:
        return await this.showInteractiveDemo(opportunity);
        
      case PresentationStrategyType.GentleNotification:
        return await this.showGentleNotification(opportunity);
        
      case PresentationStrategyType.ProgressiveDisclosure:
        return await this.showProgressiveDisclosure(opportunity);
    }
  }
  
  private async showContextualTooltip(
    opportunity: FeatureDiscoveryOpportunity
  ): Promise<FeatureDiscoveryResult> {
    const tooltip = {
      title: `Try ${opportunity.feature.name}`,
      content: opportunity.feature.shortDescription,
      position: opportunity.contextualPosition,
      actions: [
        {
          label: "Show me how",
          action: () => this.startFeatureDemo(opportunity.feature)
        },
        {
          label: "Maybe later",
          action: () => this.deferFeatureDiscovery(opportunity)
        }
      ],
      timing: {
        showAfter: 2000, // Show after 2 seconds of relevant context
        hideAfter: 10000, // Auto-hide after 10 seconds
        maxShowsPerSession: 1
      }
    };
    
    return {
      presented: true,
      method: "contextual_tooltip",
      userResponse: await this.showTooltip(tooltip)
    };
  }
  
  private identifyUnusedFeatures(
    userId: string,
    behaviorPattern: UserBehaviorPattern
  ): UnusedFeature[] {
    const allFeatures = this.getAllFeatures();
    const usedFeatures = behaviorPattern.featuresUsed;
    
    return allFeatures
      .filter(feature => !usedFeatures.includes(feature.id))
      .map(feature => ({
        feature,
        potentialValue: this.calculatePotentialValue(feature, behaviorPattern),
        discoveryDifficulty: this.calculateDiscoveryDifficulty(feature),
        userReadiness: this.assessUserReadiness(feature, behaviorPattern)
      }))
      .filter(unused => unused.potentialValue > 0.3 && unused.userReadiness > 0.5)
      .sort((a, b) => (b.potentialValue * b.userReadiness) - (a.potentialValue * a.userReadiness));
  }
  
  private findContextualOpportunities(
    behaviorPattern: UserBehaviorPattern,
    unusedFeatures: UnusedFeature[]
  ): FeatureDiscoveryOpportunity[] {
    const opportunities: FeatureDiscoveryOpportunity[] = [];
    
    for (const unusedFeature of unusedFeatures) {
      // Look for patterns that suggest this feature would be helpful
      const triggerPatterns = this.findTriggerPatterns(unusedFeature.feature, behaviorPattern);
      
      for (const pattern of triggerPatterns) {
        opportunities.push({
          feature: unusedFeature.feature,
          triggerPattern: pattern,
          contextualPosition: this.calculateContextualPosition(pattern),
          urgency: this.calculateUrgency(pattern, unusedFeature),
          expectedValue: unusedFeature.potentialValue,
          presentationStrategy: this.selectOptimalPresentation(unusedFeature, pattern)
        });
      }
    }
    
    return opportunities.sort((a, b) => 
      (b.urgency * b.expectedValue) - (a.urgency * a.expectedValue)
    );
  }
}

class WorkflowOptimizationEngine {
  public async analyzeUserWorkflow(
    userId: string,
    timeframe: TimeFrame
  ): Promise<WorkflowAnalysis> {
    const workflowData = await this.collectWorkflowData(userId, timeframe);
    
    // Identify inefficiencies
    const inefficiencies = this.identifyInefficiencies(workflowData);
    
    // Find optimization opportunities
    const optimizations = this.findOptimizationOpportunities(workflowData, inefficiencies);
    
    // Generate recommendations
    const recommendations = this.generateWorkflowRecommendations(optimizations);
    
    return {
      currentWorkflow: workflowData,
      inefficiencies,
      optimizations,
      recommendations,
      potentialTimeSavings: this.calculateTimeSavings(optimizations)
    };
  }
  
  public async suggestWorkflowImprovements(
    userId: string,
    analysis: WorkflowAnalysis
  ): Promise<WorkflowSuggestion[]> {
    const suggestions: WorkflowSuggestion[] = [];
    
    for (const recommendation of analysis.recommendations) {
      const suggestion = {
        id: `suggestion_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        title: recommendation.title,
        description: recommendation.description,
        category: recommendation.category,
        impact: recommendation.estimatedImpact,
        effort: recommendation.implementationEffort,
        steps: recommendation.implementationSteps,
        benefits: recommendation.expectedBenefits,
        metrics: recommendation.successMetrics
      };
      
      suggestions.push(suggestion);
    }
    
    // Sort by impact/effort ratio
    return suggestions.sort((a, b) => 
      (b.impact / b.effort) - (a.impact / a.effort)
    );
  }
  
  private identifyInefficiencies(workflowData: WorkflowData): WorkflowInefficiency[] {
    const inefficiencies: WorkflowInefficiency[] = [];
    
    // Analyze repetitive actions
    const repetitiveActions = this.findRepetitiveActions(workflowData);
    for (const action of repetitiveActions) {
      if (action.frequency > 10 && action.timePerAction > 30) {
        inefficiencies.push({
          type: InefficiencyType.RepetitiveAction,
          description: `Frequently performing: ${action.actionType}`,
          frequency: action.frequency,
          timeWasted: action.frequency * action.timePerAction,
          automationPotential: this.assessAutomationPotential(action)
        });
      }
    }
    
    // Analyze context switching
    const contextSwitches = this.analyzeContextSwitching(workflowData);
    if (contextSwitches.frequency > 20) {
      inefficiencies.push({
        type: InefficiencyType.ExcessiveContextSwitching,
        description: "Frequently switching between documents/features",
        frequency: contextSwitches.frequency,
        timeWasted: contextSwitches.totalSwitchTime,
        automationPotential: 0.3 // Moderate automation potential
      });
    }
    
    // Analyze underutilized features
    const underutilizedFeatures = this.findUnderutilizedFeatures(workflowData);
    for (const feature of underutilizedFeatures) {
      inefficiencies.push({
        type: InefficiencyType.UnderutilizedFeature,
        description: `Could benefit from using: ${feature.name}`,
        frequency: 0,
        timeWasted: feature.potentialTimeSavings,
        automationPotential: 0.8 // High potential since feature exists
      });
    }
    
    return inefficiencies;
  }
}

enum CompletionStatus {
  NotStarted,
  InProgress,
  Completed,
  Abandoned
}

enum HelpStrategyType {
  AdditionalGuidance,
  SimplifiedVersion,
  PersonalizedTutorial,
  SkipOption
}

enum PresentationStrategyType {
  ContextualTooltip,
  InteractiveDemo,
  GentleNotification,
  ProgressiveDisclosure
}

enum InefficiencyType {
  RepetitiveAction,
  ExcessiveContextSwitching,
  UnderutilizedFeature,
  SuboptimalWorkflow
}

interface UserProgress {
  stepsCompleted: number;
  totalSteps: number;
  timeSpent: number;
  strugglingAreas: string[];
  strengths: string[];
}

interface FeatureDiscoveryOpportunity {
  feature: Feature;
  triggerPattern: TriggerPattern;
  contextualPosition: UIPosition;
  urgency: number;
  expectedValue: number;
  presentationStrategy: PresentationStrategy;
}

interface WorkflowInefficiency {
  type: InefficiencyType;
  description: string;
  frequency: number;
  timeWasted: number; // in seconds
  automationPotential: number; // 0-1 scale
}
```

---

## 7. Detailed Workflow Specifications

### User Interaction Flows

#### Complete User Journey Mapping
```typescript
interface UserJourney {
  journeyId: string;
  journeyName: string;
  userPersona: UserPersona;
  stages: JourneyStage[];
  touchpoints: Touchpoint[];
  painPoints: PainPoint[];
  opportunities: Opportunity[];
  successMetrics: SuccessMetric[];
}

interface JourneyStage {
  stageId: string;
  stageName: string;
  description: string;
  userGoals: string[];
  userActions: UserAction[];
  systemResponses: SystemResponse[];
  emotionalState: EmotionalState;
  duration: TimeRange;
  exitCriteria: ExitCriteria[];
}

interface UserAction {
  actionId: string;
  actionType: ActionType;
  description: string;
  triggers: Trigger[];
  preconditions: Precondition[];
  expectedOutcome: string;
  alternativeFlows: AlternativeFlow[];
  errorHandling: ErrorHandling;
}

enum ActionType {
  Navigation,
  ContentCreation,
  ContentEditing,
  AIInteraction,
  StoryBibleManagement,
  Collaboration,
  SystemConfiguration
}

class UserFlowOrchestrator {
  private activeJourneys: Map<string, ActiveJourney> = new Map();
  private flowAnalytics: FlowAnalytics;
  private adaptationEngine: FlowAdaptationEngine;
  
  public async initiateUserFlow(
    userId: string,
    flowType: FlowType,
    context: FlowContext
  ): Promise<FlowSession> {
    // Determine user persona and appropriate journey
    const userPersona = await this.determineUserPersona(userId, context);
    const journey = this.selectOptimalJourney(flowType, userPersona);
    
    // Create active journey session
    const session: FlowSession = {
      sessionId: `flow_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      userId,
      journey,
      currentStage: 0,
      startTime: new Date(),
      context,
      adaptations: [],
      metrics: {
        stageCompletionTimes: [],
        errorCount: 0,
        helpRequestCount: 0,
        satisfactionScore: 0
      }
    };
    
    this.activeJourneys.set(userId, {
      session,
      stageHistory: [],
      currentState: FlowState.Active
    });
    
    // Initialize first stage
    await this.initializeStage(session, journey.stages[0]);
    
    return session;
  }
  
  public async progressFlow(
    userId: string,
    actionResult: ActionResult
  ): Promise<FlowProgressResult> {
    const activeJourney = this.activeJourneys.get(userId);
    if (!activeJourney) {
      throw new Error('No active journey found for user');
    }
    
    const session = activeJourney.session;
    const currentStage = session.journey.stages[session.currentStage];
    
    // Validate action result against current stage
    const validation = await this.validateActionResult(currentStage, actionResult);
    if (!validation.isValid) {
      return await this.handleInvalidAction(session, validation);
    }
    
    // Check if stage completion criteria are met
    const stageComplete = await this.evaluateStageCompletion(currentStage, actionResult);
    
    if (stageComplete.isComplete) {
      // Record stage completion
      session.metrics.stageCompletionTimes.push({
        stageId: currentStage.stageId,
        duration: Date.now() - session.startTime.getTime()
      });
      
      // Check for flow adaptations
      const adaptations = await this.adaptationEngine.analyzeAndSuggestAdaptations(
        session,
        actionResult
      );
      
      if (adaptations.length > 0) {
        await this.applyFlowAdaptations(session, adaptations);
      }
      
      // Progress to next stage or complete journey
      if (session.currentStage < session.journey.stages.length - 1) {
        session.currentStage++;
        const nextStage = session.journey.stages[session.currentStage];
        await this.initializeStage(session, nextStage);
        
        return {
          stageCompleted: true,
          journeyCompleted: false,
          nextStage,
          adaptations,
          recommendations: await this.generateStageRecommendations(session, nextStage)
        };
      } else {
        // Journey completed
        await this.completeJourney(session);
        return {
          stageCompleted: true,
          journeyCompleted: true,
          completionSummary: await this.generateCompletionSummary(session),
          nextJourneyRecommendations: await this.suggestNextJourneys(session)
        };
      }
    } else {
      // Continue current stage with guidance
      return {
        stageCompleted: false,
        journeyCompleted: false,
        guidance: await this.generateStageGuidance(currentStage, actionResult),
        suggestions: await this.generateActionSuggestions(currentStage, actionResult)
      };
    }
  }
  
  private async initializeStage(session: FlowSession, stage: JourneyStage): Promise<void> {
    // Set up stage-specific UI elements
    await this.setupStageUI(stage);
    
    // Initialize stage context
    await this.initializeStageContext(session, stage);
    
    // Show stage introduction if needed
    if (stage.description) {
      await this.showStageIntroduction(stage);
    }
    
    // Set up stage-specific help and guidance
    await this.setupStageHelp(stage);
    
    // Track stage initialization
    this.flowAnalytics.trackStageStart(session.userId, session.sessionId, stage.stageId);
  }
}

// Writing Flow Specifications
class WritingFlowManager {
  public async initiateWritingSession(
    userId: string,
    documentId: number,
    writingGoal: WritingGoal
  ): Promise<WritingSession> {
    const session: WritingSession = {
      sessionId: `writing_${Date.now()}`,
      userId,
      documentId,
      goal: writingGoal,
      startTime: new Date(),
      phases: this.generateWritingPhases(writingGoal),
      currentPhase: 0,
      metrics: {
        wordsWritten: 0,
        aiInteractions: 0,
        editingCycles: 0,
        focusTime: 0,
        distractionCount: 0
      },
      context: await this.buildWritingContext(documentId, writingGoal)
    };
    
    // Initialize first phase
    await this.initializeWritingPhase(session, session.phases[0]);
    
    return session;
  }
  
  private generateWritingPhases(goal: WritingGoal): WritingPhase[] {
    const phases: WritingPhase[] = [];
    
    switch (goal.type) {
      case WritingGoalType.FirstDraft:
        phases.push(
          {
            phaseId: 'planning',
            name: 'Planning & Outlining',
            description: 'Organize your thoughts and create a structure',
            estimatedDuration: 15 * 60, // 15 minutes
            activities: [
              { type: 'brainstorm', name: 'Brainstorm key points' },
              { type: 'outline', name: 'Create basic outline' },
              { type: 'research', name: 'Gather reference materials' }
            ],
            successCriteria: ['outline_created', 'key_points_identified'],
            aiAssistance: {
              suggestedTools: ['brainstorm', 'outline'],
              contextNeeds: ['story_bible', 'character_info']
            }
          },
          {
            phaseId: 'drafting',
            name: 'First Draft Writing',
            description: 'Focus on getting your ideas down without editing',
            estimatedDuration: 45 * 60, // 45 minutes
            activities: [
              { type: 'write', name: 'Write continuously' },
              { type: 'expand', name: 'Develop key scenes' }
            ],
            successCriteria: ['target_word_count_reached', 'main_points_covered'],
            aiAssistance: {
              suggestedTools: ['write', 'expand', 'describe'],
              contextNeeds: ['story_bible', 'previous_chapters']
            }
          },
          {
            phaseId: 'review',
            name: 'Initial Review',
            description: 'Quick review for major issues',
            estimatedDuration: 10 * 60, // 10 minutes
            activities: [
              { type: 'read_through', name: 'Read through draft' },
              { type: 'note_issues', name: 'Note major issues' }
            ],
            successCriteria: ['draft_reviewed', 'issues_identified'],
            aiAssistance: {
              suggestedTools: ['analyze', 'feedback'],
              contextNeeds: ['full_document']
            }
          }
        );
        break;
        
      case WritingGoalType.Revision:
        phases.push(
          {
            phaseId: 'analysis',
            name: 'Content Analysis',
            description: 'Analyze existing content for improvement opportunities',
            estimatedDuration: 20 * 60,
            activities: [
              { type: 'analyze_structure', name: 'Analyze story structure' },
              { type: 'identify_weaknesses', name: 'Identify weak areas' },
              { type: 'plan_improvements', name: 'Plan improvements' }
            ],
            successCriteria: ['analysis_complete', 'improvement_plan_created'],
            aiAssistance: {
              suggestedTools: ['analyze', 'feedback', 'plot_analysis'],
              contextNeeds: ['full_document', 'story_bible']
            }
          },
          {
            phaseId: 'revision',
            name: 'Content Revision',
            description: 'Make targeted improvements to content',
            estimatedDuration: 60 * 60,
            activities: [
              { type: 'rewrite_sections', name: 'Rewrite weak sections' },
              { type: 'enhance_descriptions', name: 'Enhance descriptions' },
              { type: 'improve_dialogue', name: 'Improve dialogue' }
            ],
            successCriteria: ['key_sections_revised', 'quality_improved'],
            aiAssistance: {
              suggestedTools: ['rewrite', 'expand', 'describe', 'dialogue'],
              contextNeeds: ['story_bible', 'character_voices']
            }
          }
        );
        break;
    }
    
    return phases;
  }
}
```

### State Management Architecture

#### Comprehensive Application State Management
```typescript
interface ApplicationState {
  ui: UIState;
  documents: DocumentsState;
  storyBible: StoryBibleState;
  ai: AIState;
  user: UserState;
  system: SystemState;
}

interface StateManager {
  currentState: ApplicationState;
  stateHistory: StateSnapshot[];
  subscribers: StateSubscriber[];
  middleware: StateMiddleware[];
  persistenceLayer: StatePersistence;
}

class CentralizedStateManager {
  private state: ApplicationState;
  private subscribers: Map<string, StateSubscriber[]> = new Map();
  private middleware: StateMiddleware[] = [];
  private stateHistory: StateSnapshot[] = [];
  private maxHistorySize = 50;
  
  constructor() {
    this.state = this.initializeState();
    this.setupMiddleware();
  }
  
  public subscribe<T>(
    selector: StateSelector<T>,
    callback: StateCallback<T>,
    options?: SubscriptionOptions
  ): Unsubscribe {
    const subscription: StateSubscription<T> = {
      id: `sub_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      selector,
      callback,
      options: options || {},
      lastValue: selector(this.state)
    };
    
    const selectorKey = this.getSelectorKey(selector);
    if (!this.subscribers.has(selectorKey)) {
      this.subscribers.set(selectorKey, []);
    }
    this.subscribers.get(selectorKey)!.push(subscription);
    
    return () => this.unsubscribe(subscription.id);
  }
  
  public dispatch(action: StateAction): Promise<void> {
    return new Promise(async (resolve, reject) => {
      try {
        // Create state snapshot for history
        const snapshot: StateSnapshot = {
          timestamp: new Date(),
          state: this.deepClone(this.state),
          action: action
        };
        
        // Apply middleware
        let processedAction = action;
        for (const middleware of this.middleware) {
          processedAction = await middleware.process(processedAction, this.state);
        }
        
        // Apply state changes
        const newState = await this.applyAction(this.state, processedAction);
        
        // Validate state consistency
        const validation = await this.validateState(newState);
        if (!validation.isValid) {
          throw new Error(`State validation failed: ${validation.errors.join(', ')}`);
        }
        
        // Update state
        const previousState = this.state;
        this.state = newState;
        
        // Add to history
        this.addToHistory(snapshot);
        
        // Notify subscribers
        await this.notifySubscribers(previousState, newState);
        
        // Persist state if needed
        if (this.shouldPersistAction(processedAction)) {
          await this.persistState(newState);
        }
        
        resolve();
      } catch (error) {
        reject(error);
      }
    });
  }
  
  private async applyAction(
    currentState: ApplicationState,
    action: StateAction
  ): Promise<ApplicationState> {
    const newState = this.deepClone(currentState);
    
    switch (action.type) {
      case 'DOCUMENT_CONTENT_CHANGED':
        return this.handleDocumentContentChanged(newState, action);
        
      case 'STORY_BIBLE_ELEMENT_UPDATED':
        return this.handleStoryBibleElementUpdated(newState, action);
        
      case 'AI_GENERATION_STARTED':
        return this.handleAIGenerationStarted(newState, action);
        
      case 'AI_GENERATION_COMPLETED':
        return this.handleAIGenerationCompleted(newState, action);
        
      case 'UI_LAYOUT_CHANGED':
        return this.handleUILayoutChanged(newState, action);
        
      case 'USER_PREFERENCES_UPDATED':
        return this.handleUserPreferencesUpdated(newState, action);
        
      default:
        console.warn(`Unknown action type: ${action.type}`);
        return newState;
    }
  }
  
  private handleDocumentContentChanged(
    state: ApplicationState,
    action: DocumentContentChangedAction
  ): ApplicationState {
    const { documentId, content, changeType, range } = action.payload;
    
    // Update document content
    if (state.documents.openDocuments.has(documentId)) {
      const document = state.documents.openDocuments.get(documentId)!;
      document.content = content;
      document.lastModified = new Date();
      document.hasUnsavedChanges = true;
      document.version++;
      
      // Track change for undo/redo
      document.changeHistory.push({
        changeId: `change_${Date.now()}`,
        changeType,
        range,
        previousContent: document.content,
        newContent: content,
        timestamp: new Date()
      });
      
      // Limit change history size
      if (document.changeHistory.length > 100) {
        document.changeHistory = document.changeHistory.slice(-100);
      }
    }
    
    // Update related state
    if (changeType === 'content_edit') {
      // Invalidate AI context cache for this document
      state.ai.contextCache.delete(documentId);
      
      // Mark Story Bible references for re-analysis
      this.markStoryBibleReferencesForUpdate(state, documentId, range);
    }
    
    return state;
  }
  
  private handleStoryBibleElementUpdated(
    state: ApplicationState,
    action: StoryBibleElementUpdatedAction
  ): ApplicationState {
    const { elementType, elementId, updates } = action.payload;
    
    // Update Story Bible element
    switch (elementType) {
      case 'character':
        if (state.storyBible.characters.has(elementId)) {
          const character = state.storyBible.characters.get(elementId)!;
          Object.assign(character, updates);
          character.lastModified = new Date();
        }
        break;
        
      case 'worldbuilding':
        if (state.storyBible.worldbuilding.has(elementId)) {
          const element = state.storyBible.worldbuilding.get(elementId)!;
          Object.assign(element, updates);
          element.lastModified = new Date();
        }
        break;
    }
    
    // Propagate changes to related documents
    const affectedDocuments = this.findDocumentsReferencingElement(
      state,
      elementType,
      elementId
    );
    
    for (const documentId of affectedDocuments) {
      // Mark document for Story Bible sync
      if (state.documents.openDocuments.has(documentId)) {
        const document = state.documents.openDocuments.get(documentId)!;
        document.storyBibleSyncNeeded = true;
      }
      
      // Invalidate AI context cache
      state.ai.contextCache.delete(documentId);
    }
    
    return state;
  }
  
  private async notifySubscribers(
    previousState: ApplicationState,
    newState: ApplicationState
  ): Promise<void> {
    const notificationPromises: Promise<void>[] = [];
    
    for (const [selectorKey, subscriptions] of this.subscribers) {
      for (const subscription of subscriptions) {
        const newValue = subscription.selector(newState);
        const hasChanged = !this.deepEqual(subscription.lastValue, newValue);
        
        if (hasChanged || subscription.options.notifyOnNoChange) {
          subscription.lastValue = newValue;
          
          const notificationPromise = this.notifySubscriber(
            subscription,
            newValue,
            previousState,
            newState
          );
          
          notificationPromises.push(notificationPromise);
        }
      }
    }
    
    await Promise.all(notificationPromises);
  }
  
  private async notifySubscriber<T>(
    subscription: StateSubscription<T>,
    newValue: T,
    previousState: ApplicationState,
    newState: ApplicationState
  ): Promise<void> {
    try {
      if (subscription.options.async) {
        // Asynchronous notification
        setImmediate(() => {
          subscription.callback(newValue, previousState, newState);
        });
      } else {
        // Synchronous notification
        await subscription.callback(newValue, previousState, newState);
      }
    } catch (error) {
      console.error('Error in state subscriber:', error);
      
      // Remove problematic subscriber if configured
      if (subscription.options.removeOnError) {
        this.unsubscribe(subscription.id);
      }
    }
  }
}

// State Persistence Layer
class StatePersistenceManager {
  private storage: StorageAdapter;
  private compressionEnabled = true;
  private encryptionEnabled = true;
  
  public async persistState(state: ApplicationState): Promise<void> {
    try {
      // Create persistable state (exclude non-serializable data)
      const persistableState = this.createPersistableState(state);
      
      // Compress if enabled
      let serializedState = JSON.stringify(persistableState);
      if (this.compressionEnabled) {
        serializedState = await this.compressData(serializedState);
      }
      
      // Encrypt if enabled
      if (this.encryptionEnabled) {
        serializedState = await this.encryptData(serializedState);
      }
      
      // Store with versioning
      await this.storage.store('application_state', serializedState, {
        version: state.system.version,
        timestamp: new Date(),
        checksum: await this.calculateChecksum(serializedState)
      });
      
    } catch (error) {
      console.error('Failed to persist state:', error);
      throw new Error('State persistence failed');
    }
  }
  
  public async loadState(): Promise<ApplicationState | null> {
    try {
      const storedData = await this.storage.retrieve('application_state');
      if (!storedData) {
        return null;
      }
      
      // Verify checksum
      const calculatedChecksum = await this.calculateChecksum(storedData.data);
      if (calculatedChecksum !== storedData.metadata.checksum) {
        throw new Error('State data corruption detected');
      }
      
      // Decrypt if needed
      let serializedState = storedData.data;
      if (this.encryptionEnabled) {
        serializedState = await this.decryptData(serializedState);
      }
      
      // Decompress if needed
      if (this.compressionEnabled) {
        serializedState = await this.decompressData(serializedState);
      }
      
      // Parse and validate
      const persistedState = JSON.parse(serializedState);
      const restoredState = this.restoreState(persistedState);
      
      return restoredState;
      
    } catch (error) {
      console.error('Failed to load state:', error);
      return null;
    }
  }
  
  private createPersistableState(state: ApplicationState): PersistableState {
    return {
      documents: {
        openDocuments: Array.from(state.documents.openDocuments.entries()),
        recentDocuments: state.documents.recentDocuments,
        documentSettings: state.documents.documentSettings
      },
      storyBible: {
        characters: Array.from(state.storyBible.characters.entries()),
        worldbuilding: Array.from(state.storyBible.worldbuilding.entries()),
        plotElements: Array.from(state.storyBible.plotElements.entries()),
        settings: state.storyBible.settings
      },
      ui: {
        layout: state.ui.layout,
        preferences: state.ui.preferences,
        theme: state.ui.theme
      },
      user: {
        preferences: state.user.preferences,
        settings: state.user.settings
      },
      system: {
        version: state.system.version,
        lastSaved: new Date()
      }
    };
  }
}
```

### Error Handling Strategies

#### Comprehensive Error Recovery and User Feedback Systems
```typescript
interface ErrorHandlingStrategy {
  errorTypes: ErrorType[];
  recoveryActions: RecoveryAction[];
  userFeedback: UserFeedbackStrategy;
  escalationRules: EscalationRule[];
  preventionMeasures: PreventionMeasure[];
}

enum ErrorType {
  NetworkError,
  ValidationError,
  AIServiceError,
  DataCorruption,
  PermissionError,
  ResourceExhaustion,
  UserInputError,
  SystemError
}

enum ErrorSeverity {
  Low,      // Minor inconvenience, user can continue
  Medium,   // Affects current operation, user needs to take action
  High,     // Blocks major functionality, requires immediate attention
  Critical  // System unusable, data loss risk
}

class ComprehensiveErrorHandler {
  private errorStrategies: Map<ErrorType, ErrorHandlingStrategy> = new Map();
  private errorHistory: ErrorRecord[] = [];
  private recoveryAttempts: Map<string, RecoveryAttempt[]> = new Map();
  private userFeedbackManager: UserFeedbackManager;
  
  constructor() {
    this.initializeErrorStrategies();
    this.userFeedbackManager = new UserFeedbackManager();
  }
  
  public async handleError(
    error: ApplicationError,
    context: ErrorContext
  ): Promise<ErrorHandlingResult> {
    // Log error immediately
    const errorRecord = await this.logError(error, context);
    
    // Determine error severity and type
    const classification = this.classifyError(error);
    
    // Get handling strategy
    const strategy = this.errorStrategies.get(classification.type);
    if (!strategy) {
      return this.handleUnknownError(error, context);
    }
    
    // Check if we've seen this error before
    const previousAttempts = this.recoveryAttempts.get(error.errorId) || [];
    
    // Select appropriate recovery action
    const recoveryAction = this.selectRecoveryAction(
      strategy,
      classification,
      previousAttempts,
      context
    );
    
    // Provide immediate user feedback
    await this.provideUserFeedback(error, classification, strategy.userFeedback);
    
    // Attempt recovery
    const recoveryResult = await this.attemptRecovery(
      recoveryAction,
      error,
      context
    );
    
    // Record recovery attempt
    this.recordRecoveryAttempt(error.errorId, recoveryAction, recoveryResult);
    
    // Handle recovery result
    if (recoveryResult.success) {
      await this.handleSuccessfulRecovery(error, recoveryResult);
      return {
        handled: true,
        recovered: true,
        userAction: recoveryResult.userActionRequired,
        message: recoveryResult.successMessage
      };
    } else {
      return await this.handleFailedRecovery(error, recoveryResult, strategy);
    }
  }
  
  private initializeErrorStrategies(): void {
    // Network Error Strategy
    this.errorStrategies.set(ErrorType.NetworkError, {
      errorTypes: [ErrorType.NetworkError],
      recoveryActions: [
        {
          actionType: RecoveryActionType.Retry,
          maxAttempts: 3,
          backoffStrategy: BackoffStrategy.Exponential,
          condition: (error, context) => context.isRetryable
        },
        {
          actionType: RecoveryActionType.FallbackService,
          maxAttempts: 1,
          condition: (error, context) => context.hasFallback
        },
        {
          actionType: RecoveryActionType.OfflineMode,
          maxAttempts: 1,
          condition: (error, context) => context.supportsOffline
        }
      ],
      userFeedback: {
        immediate: {
          type: FeedbackType.Toast,
          message: "Connection issue detected. Attempting to reconnect...",
          duration: 5000,
          showProgress: true
        },
        persistent: {
          type: FeedbackType.StatusBar,
          message: "Working offline - changes will sync when connection is restored",
          icon: "offline"
        }
      },
      escalationRules: [
        {
          condition: (attempts) => attempts.length > 3,
          action: EscalationAction.ShowDetailedError
        },
        {
          condition: (attempts) => attempts.length > 5,
          action: EscalationAction.ContactSupport
        }
      ],
      preventionMeasures: [
        {
          type: PreventionType.ConnectionMonitoring,
          description: "Monitor network connectivity"
        },
        {
          type: PreventionType.RequestTimeout,
          description: "Set appropriate request timeouts"
        }
      ]
    });
    
    // AI Service Error Strategy
    this.errorStrategies.set(ErrorType.AIServiceError, {
      errorTypes: [ErrorType.AIServiceError],
      recoveryActions: [
        {
          actionType: RecoveryActionType.RetryWithDifferentModel,
          maxAttempts: 2,
          condition: (error, context) => context.hasAlternativeModels
        },
        {
          actionType: RecoveryActionType.ReduceComplexity,
          maxAttempts: 1,
          condition: (error, context) => error.code === 'CONTEXT_TOO_LARGE'
        },
        {
          actionType: RecoveryActionType.ShowCachedResults,
          maxAttempts: 1,
          condition: (error, context) => context.hasCachedResults
        }
      ],
      userFeedback: {
        immediate: {
          type: FeedbackType.Modal,
          title: "AI Service Temporarily Unavailable",
          message: "We're experiencing issues with the AI service. Trying alternative approaches...",
          actions: [
            { label: "Try Again", action: "retry" },
            { label: "Use Cached Result", action: "use_cache" },
            { label: "Continue Without AI", action: "skip" }
          ]
        }
      },
      escalationRules: [
        {
          condition: (attempts) => attempts.length > 2,
          action: EscalationAction.DisableAIFeatures
        }
      ],
      preventionMeasures: [
        {
          type: PreventionType.ServiceHealthCheck,
          description: "Regular health checks for AI services"
        },
        {
          type: PreventionType.CircuitBreaker,
          description: "Circuit breaker pattern for AI service calls"
        }
      ]
    });
    
    // Data Corruption Strategy
    this.errorStrategies.set(ErrorType.DataCorruption, {
      errorTypes: [ErrorType.DataCorruption],
      recoveryActions: [
        {
          actionType: RecoveryActionType.RestoreFromBackup,
          maxAttempts: 1,
          condition: (error, context) => context.hasBackup
        },
        {
          actionType: RecoveryActionType.RepairData,
          maxAttempts: 1,
          condition: (error, context) => context.isRepairable
        },
        {
          actionType: RecoveryActionType.IsolateCorruption,
          maxAttempts: 1,
          condition: () => true
        }
      ],
      userFeedback: {
        immediate: {
          type: FeedbackType.Modal,
          title: "Data Integrity Issue Detected",
          message: "We've detected a potential issue with your data. We're working to resolve it automatically.",
          severity: FeedbackSeverity.High
        },
        followUp: {
          type: FeedbackType.Notification,
          message: "Data has been successfully recovered from backup",
          actions: [
            { label: "Review Changes", action: "review_recovery" }
          ]
        }
      },
      escalationRules: [
        {
          condition: () => true,
          action: EscalationAction.CreateSupportTicket
        }
      ],
      preventionMeasures: [
        {
          type: PreventionType.DataValidation,
          description: "Validate data integrity on save/load"
        },
        {
          type: PreventionType.AutomaticBackups,
          description: "Regular automatic backups"
        }
      ]
    });
  }
  
  private async attemptRecovery(
    action: RecoveryAction,
    error: ApplicationError,
    context: ErrorContext
  ): Promise<RecoveryResult> {
    const startTime = Date.now();
    
    try {
      switch (action.actionType) {
        case RecoveryActionType.Retry:
          return await this.performRetry(action, error, context);
          
        case RecoveryActionType.FallbackService:
          return await this.performFallback(action, error, context);
          
        case RecoveryActionType.RestoreFromBackup:
          return await this.performBackupRestore(action, error, context);
          
        case RecoveryActionType.RepairData:
          return await this.performDataRepair(action, error, context);
          
        case RecoveryActionType.ReduceComplexity:
          return await this.performComplexityReduction(action, error, context);
          
        default:
          return {
            success: false,
            errorMessage: `Unknown recovery action: ${action.actionType}`,
            duration: Date.now() - startTime
          };
      }
    } catch (recoveryError) {
      return {
        success: false,
        errorMessage: `Recovery failed: ${recoveryError.message}`,
        duration: Date.now() - startTime,
        additionalError: recoveryError
      };
    }
  }
  
  private async performRetry(
    action: RecoveryAction,
    error: ApplicationError,
    context: ErrorContext
  ): Promise<RecoveryResult> {
    const maxAttempts = action.maxAttempts || 3;
    let attempt = 0;
    let lastError: Error | null = null;
    
    while (attempt < maxAttempts) {
      attempt++;
      
      // Calculate delay based on backoff strategy
      const delay = this.calculateBackoffDelay(
        action.backoffStrategy || BackoffStrategy.Linear,
        attempt
      );
      
      if (delay > 0) {
        await this.sleep(delay);
      }
      
      try {
        // Retry the original operation
        const result = await this.retryOriginalOperation(error, context);
        
        return {
          success: true,
          successMessage: `Operation succeeded on attempt ${attempt}`,
          duration: 0,
          attemptsUsed: attempt
        };
      } catch (retryError) {
        lastError = retryError;
        
        // Update user on retry progress
        await this.updateRetryProgress(attempt, maxAttempts, retryError);
      }
    }
    
    return {
      success: false,
      errorMessage: `All ${maxAttempts} retry attempts failed. Last error: ${lastError?.message}`,
      duration: 0,
      attemptsUsed: attempt
    };
  }
  
  private async performBackupRestore(
    action: RecoveryAction,
    error: ApplicationError,
    context: ErrorContext
  ): Promise<RecoveryResult> {
    // Find the most recent valid backup
    const backup = await this.findLatestValidBackup(context.resourceId);
    
    if (!backup) {
      return {
        success: false,
        errorMessage: "No valid backup found for restoration"
      };
    }
    
    // Confirm with user before restoring
    const userConfirmation = await this.requestUserConfirmation({
      title: "Restore from Backup",
      message: `We found a backup from ${backup.timestamp.toLocaleString()}. Restore this version?`,
      details: `This will replace the current corrupted data. Any changes since ${backup.timestamp.toLocaleString()} will be lost.`,
      confirmLabel: "Restore Backup",
      cancelLabel: "Cancel"
    });
    
    if (!userConfirmation) {
      return {
        success: false,
        errorMessage: "User cancelled backup restoration",
        userActionRequired: true
      };
    }
    
    // Perform restoration
    try {
      await this.restoreFromBackup(backup, context.resourceId);
      
      return {
        success: true,
        successMessage: `Data successfully restored from backup (${backup.timestamp.toLocaleString()})`,
        userActionRequired: false
      };
    } catch (restoreError) {
      return {
        success: false,
        errorMessage: `Backup restoration failed: ${restoreError.message}`
      };
    }
  }
}

// User Feedback Management
class UserFeedbackManager {
  private activeNotifications: Map<string, ActiveNotification> = new Map();
  private feedbackQueue: FeedbackItem[] = [];
  private maxConcurrentNotifications = 3;
  
  public async showErrorFeedback(
    error: ApplicationError,
    classification: ErrorClassification,
    feedbackStrategy: UserFeedbackStrategy
  ): Promise<void> {
    // Show immediate feedback
    if (feedbackStrategy.immediate) {
      await this.showImmediateFeedback(feedbackStrategy.immediate, error);
    }
    
    // Set up persistent feedback if needed
    if (feedbackStrategy.persistent) {
      await this.showPersistentFeedback(feedbackStrategy.persistent, error);
    }
    
    // Schedule follow-up feedback if configured
    if (feedbackStrategy.followUp) {
      this.scheduleFeedback(feedbackStrategy.followUp, error, 5000); // 5 second delay
    }
  }
  
  private async showImmediateFeedback(
    feedback: ImmediateFeedback,
    error: ApplicationError
  ): Promise<void> {
    const notificationId = `error_${error.errorId}_${Date.now()}`;
    
    const notification: ActiveNotification = {
      id: notificationId,
      type: feedback.type,
      title: feedback.title || "Error Occurred",
      message: this.personalizeMessage(feedback.message, error),
      severity: feedback.severity || FeedbackSeverity.Medium,
      duration: feedback.duration || 5000,
      actions: feedback.actions || [],
      showProgress: feedback.showProgress || false,
      startTime: Date.now()
    };
    
    // Check if we're at the notification limit
    if (this.activeNotifications.size >= this.maxConcurrentNotifications) {
      // Queue the notification
      this.feedbackQueue.push({
        notification,
        priority: this.calculateFeedbackPriority(feedback.severity || FeedbackSeverity.Medium)
      });
      return;
    }
    
    // Show notification immediately
    await this.displayNotification(notification);
    this.activeNotifications.set(notificationId, notification);
    
    // Auto-dismiss if duration is set
    if (notification.duration > 0) {
      setTimeout(() => {
        this.dismissNotification(notificationId);
      }, notification.duration);
    }
  }
  
  private personalizeMessage(template: string, error: ApplicationError): string {
    return template
      .replace('{errorType}', error.type)
      .replace('{errorCode}', error.code || 'Unknown')
      .replace('{timestamp}', new Date().toLocaleTimeString());
  }
  
  private async displayNotification(notification: ActiveNotification): Promise<void> {
    switch (notification.type) {
      case FeedbackType.Toast:
        await this.showToast(notification);
        break;
        
      case FeedbackType.Modal:
        await this.showModal(notification);
        break;
        
      case FeedbackType.StatusBar:
        await this.showStatusBar(notification);
        break;
        
      case FeedbackType.Notification:
        await this.showSystemNotification(notification);
        break;
    }
  }
}
```

### Performance Optimization Logic

#### Detailed Caching, Loading, and Optimization Strategies
```typescript
interface PerformanceOptimizationEngine {
  cachingStrategies: CachingStrategy[];
  loadingOptimizers: LoadingOptimizer[];
  memoryManagers: MemoryManager[];
  networkOptimizers: NetworkOptimizer[];
  renderingOptimizers: RenderingOptimizer[];
}

class AdvancedCachingSystem {
  private caches: Map<string, Cache> = new Map();
  private cacheStrategies: Map<string, CachingStrategy> = new Map();
  private evictionPolicies: Map<string, EvictionPolicy> = new Map();
  private performanceMonitor: PerformanceMonitor;
  
  constructor() {
    this.performanceMonitor = new PerformanceMonitor();
    this.initializeCachingStrategies();
  }
  
  private initializeCachingStrategies(): void {
    // Document Content Caching
    this.cacheStrategies.set('document_content', {
      name: 'Document Content Cache',
      maxSize: 50 * 1024 * 1024, // 50MB
      maxEntries: 100,
      ttl: 30 * 60 * 1000, // 30 minutes
      evictionPolicy: EvictionPolicy.LRU,
      compressionEnabled: true,
      encryptionEnabled: false,
      persistToDisk: true,
      preloadStrategy: PreloadStrategy.Predictive,
      invalidationRules: [
        {
          trigger: 'document_modified',
          action: InvalidationAction.Remove
        },
        {
          trigger: 'story_bible_updated',
          action: InvalidationAction.MarkStale
        }
      ]
    });
    
    // AI Context Caching
    this.cacheStrategies.set('ai_context', {
      name: 'AI Context Cache',
      maxSize: 100 * 1024 * 1024, // 100MB
      maxEntries: 500,
      ttl: 60 * 60 * 1000, // 1 hour
      evictionPolicy: EvictionPolicy.LFU,
      compressionEnabled: true,
      encryptionEnabled: true,
      persistToDisk: false,
      preloadStrategy: PreloadStrategy.OnDemand,
      invalidationRules: [
        {
          trigger: 'story_bible_modified',
          action: InvalidationAction.Remove
        },
        {
          trigger: 'document_content_changed',
          action: InvalidationAction.MarkStale
        }
      ]
    });
    
    // AI Response Caching
    this.cacheStrategies.set('ai_responses', {
      name: 'AI Response Cache',
      maxSize: 200 * 1024 * 1024, // 200MB
      maxEntries: 1000,
      ttl: 24 * 60 * 60 * 1000, // 24 hours
      evictionPolicy: EvictionPolicy.TLRU, // Time-aware LRU
      compressionEnabled: true,
      encryptionEnabled: false,
      persistToDisk: true,
      preloadStrategy: PreloadStrategy.None,
      invalidationRules: [
        {
          trigger: 'model_updated',
          action: InvalidationAction.RemoveByTag
        }
      ]
    });
  }
  
  public async get<T>(
    cacheKey: string,
    cacheName: string,
    fallbackFn?: () => Promise<T>
  ): Promise<T | null> {
    const cache = this.getCache(cacheName);
    const strategy = this.cacheStrategies.get(cacheName);
    
    if (!cache || !strategy) {
      if (fallbackFn) {
        return await fallbackFn();
      }
      return null;
    }
    
    // Check cache hit
    const startTime = performance.now();
    const cachedItem = await cache.get(cacheKey);
    const cacheCheckTime = performance.now() - startTime;
    
    if (cachedItem && !this.isExpired(cachedItem, strategy)) {
      // Cache hit
      this.performanceMonitor.recordCacheHit(cacheName, cacheCheckTime);
      await this.updateAccessTime(cache, cacheKey);
      
      // Decompress if needed
      if (strategy.compressionEnabled && cachedItem.compressed) {
        const decompressedData = await this.decompress(cachedItem.data);
        return decompressedData as T;
      }
      
      return cachedItem.data as T;
    }
    
    // Cache miss
    this.performanceMonitor.recordCacheMiss(cacheName, cacheCheckTime);
    
    if (fallbackFn) {
      const fallbackStartTime = performance.now();
      const data = await fallbackFn();
      const fallbackTime = performance.now() - fallbackStartTime;
      
      // Store in cache
      await this.set(cacheKey, data, cacheName);
      
      this.performanceMonitor.recordCacheMiss(cacheName, fallbackTime);
      return data;
    }
    
    return null;
  }
  
  public async set<T>(
    cacheKey: string,
    data: T,
    cacheName: string,
    options?: CacheSetOptions
  ): Promise<void> {
    const cache = this.getCache(cacheName);
    const strategy = this.cacheStrategies.get(cacheName);
    
    if (!cache || !strategy) {
      return;
    }
    
    // Check cache size limits
    const dataSize = this.calculateDataSize(data);
    if (dataSize > strategy.maxSize / 10) { // Don't cache items larger than 10% of cache size
      return;
    }
    
    // Prepare cache item
    let processedData = data;
    let compressed = false;
    
    // Compress if enabled and beneficial
    if (strategy.compressionEnabled && dataSize > 1024) { // Only compress if > 1KB
      const compressedData = await this.compress(data);
      if (compressedData.length < dataSize * 0.8) { // Only use if 20%+ compression
        processedData = compressedData as T;
        compressed = true;
      }
    }
    
    const cacheItem: CacheItem<T> = {
      key: cacheKey,
      data: processedData,
      size: this.calculateDataSize(processedData),
      createdAt: new Date(),
      lastAccessed: new Date(),
      accessCount: 1,
      ttl: options?.ttl || strategy.ttl,
      tags: options?.tags || [],
      compressed,
      metadata: options?.metadata || {}
    };
    
    // Check if we need to evict items
    await this.ensureCacheSpace(cache, strategy, cacheItem.size);
    
    // Store item
    await cache.set(cacheKey, cacheItem);
    
    // Persist to disk if enabled
    if (strategy.persistToDisk) {
      await this.persistCacheItem(cacheName, cacheKey, cacheItem);
    }
  }
  
  private async ensureCacheSpace(
    cache: Cache,
    strategy: CachingStrategy,
    requiredSpace: number
  ): Promise<void> {
    const currentSize = await cache.getTotalSize();
    const currentCount = await cache.getItemCount();
    
    // Check size limit
    if (currentSize + requiredSpace > strategy.maxSize) {
      const spaceToFree = (currentSize + requiredSpace) - strategy.maxSize;
      await this.evictBySize(cache, strategy, spaceToFree);
    }
    
    // Check count limit
    if (currentCount >= strategy.maxEntries) {
      const itemsToEvict = (currentCount + 1) - strategy.maxEntries;
      await this.evictByCount(cache, strategy, itemsToEvict);
    }
  }
  
  private async evictBySize(
    cache: Cache,
    strategy: CachingStrategy,
    spaceToFree: number
  ): Promise<void> {
    const evictionPolicy = this.evictionPolicies.get(strategy.evictionPolicy);
    if (!evictionPolicy) {
      return;
    }
    
    const candidates = await evictionPolicy.selectEvictionCandidates(
      cache,
      { type: 'size', target: spaceToFree }
    );
    
    let freedSpace = 0;
    for (const candidate of candidates) {
      if (freedSpace >= spaceToFree) {
        break;
      }
      
      const item = await cache.get(candidate.key);
      if (item) {
        await cache.remove(candidate.key);
        freedSpace += item.size;
        
        this.performanceMonitor.recordEviction(
          strategy.name,
          candidate.key,
          'size_limit'
        );
      }
    }
  }
}

class PredictivePreloader {
  private userBehaviorAnalyzer: UserBehaviorAnalyzer;
  private preloadQueue: PreloadTask[] = [];
  private activePreloads: Map<string, PreloadOperation> = new Map();
  private maxConcurrentPreloads = 3;
  
  constructor() {
    this.userBehaviorAnalyzer = new UserBehaviorAnalyzer();
    this.startPreloadProcessor();
  }
  
  public async analyzeAndPreload(
    userId: string,
    currentContext: UserContext
  ): Promise<PreloadPlan> {
    // Analyze user behavior patterns
    const behaviorPattern = await this.userBehaviorAnalyzer.analyze(userId);
    
    // Predict likely next actions
    const predictions = await this.predictNextActions(behaviorPattern, current
