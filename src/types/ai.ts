// AI Provider Types
export interface AIProvider {
  id: string;
  name: string;
  status: 'connected' | 'disconnected' | 'error';
  models: string[];
  defaultModel: string;
  apiKey?: string;
  baseUrl?: string;
  maxTokens: number;
  costPerToken: number;
  features: string[];
}

export interface AIModel {
  id: string;
  name: string;
  provider: string;
  contextLength: number;
  costPer1k: number;
  capabilities: string[];
  recommended: boolean;
}

// AI Tool Types
export type AITool = 
  | 'write'
  | 'rewrite'
  | 'expand'
  | 'brainstorm'
  | 'describe'
  | 'visualize'
  | 'quick-edit'
  | 'chat';

export interface AIToolConfig {
  id: AITool;
  name: string;
  description: string;
  icon: string;
  shortcut?: string;
  category: 'writing' | 'editing' | 'creative' | 'utility';
  requiresSelection?: boolean;
  estimatedCredits: number;
}

// Streaming Types
export interface StreamingState {
  isStreaming: boolean;
  isPaused: boolean;
  progress: number;
  estimatedTimeRemaining?: number;
  currentText: string;
  totalExpectedLength?: number;
}

export interface StreamingOptions {
  enableStreaming: boolean;
  chunkSize?: number;
  delayBetweenChunks?: number;
  showProgress?: boolean;
}

// Credit Management Types
export interface CreditUsage {
  operation: string;
  credits: number;
  cost: number;
  timestamp: Date;
  provider: string;
  model: string;
}

export interface CreditBalance {
  total: number;
  used: number;
  remaining: number;
  monthlyLimit: number;
  alertThreshold: number;
  resetDate: Date;
}

export interface CreditEstimate {
  operation: AITool;
  estimatedCredits: number;
  estimatedCost: number;
  provider: string;
  model: string;
}

// AI Settings Types
export interface AISettings {
  // Provider settings
  defaultProvider: string;
  defaultModel: string;
  
  // Behavior settings
  creativity: number; // 0-1
  responseLength: 'short' | 'medium' | 'long' | 'very-long';
  writingStyle: 'formal' | 'casual' | 'balanced' | 'creative' | 'technical';
  customInstructions: string;
  
  // Feature toggles
  autoSave: boolean;
  showCosts: boolean;
  enableStreaming: boolean;
  contextAware: boolean;
  
  // Performance settings
  requestTimeout: number;
  maxConcurrentRequests: number;
  retryAttempts: number;
  cacheDuration: number;
  
  // Security & Privacy
  logRequests: boolean;
  shareAnalytics: boolean;
  dataRetention: number; // days
  
  // Debug settings
  debugMode: boolean;
  mockMode: boolean;
}

// AI Request/Response Types
export interface AIRequest {
  tool: AITool;
  prompt: string;
  context?: string;
  selectedText?: string;
  settings?: Partial<AISettings>;
  provider?: string;
  model?: string;
  streaming?: boolean;
}

export interface AIResponse {
  id: string;
  tool: AITool;
  content: string;
  provider: string;
  model: string;
  creditsUsed: number;
  cost: number;
  timestamp: Date;
  metadata?: {
    wordCount?: number;
    processingTime?: number;
    tokensUsed?: number;
    quality?: number;
  };
}

export interface AIError {
  code: string;
  message: string;
  provider?: string;
  retryable: boolean;
  timestamp: Date;
}

// Context Types
export interface DocumentContext {
  id: string;
  title: string;
  content: string;
  genre?: string;
  characters?: string[];
  setting?: string;
  tone?: string;
  previousContent?: string;
  nextContent?: string;
}

export interface ProjectContext {
  id: string;
  name: string;
  genre: string;
  description: string;
  characters: Character[];
  settings: Setting[];
  themes: string[];
  style: string;
}

export interface Character {
  id: string;
  name: string;
  description: string;
  traits: string[];
  relationships: Record<string, string>;
}

export interface Setting {
  id: string;
  name: string;
  description: string;
  atmosphere: string;
  details: string[];
}

// Hook Return Types
export interface UseAIReturn {
  // Core functions
  autoWrite: (prompt: string, context?: string) => Promise<AIResponse>;
  guidedWrite: (prompt: string, context?: string) => Promise<AIResponse>;
  generateCard: (content: string, type: string) => Promise<void>;
  
  // State
  isLoading: boolean;
  error: AIError | null;
  lastResponse: AIResponse | null;
  
  // Actions
  clearError: () => void;
  cancelRequest: () => void;
}

export interface UseAIStreamingReturn {
  // Streaming functions
  streamAutoWrite: (prompt: string, context?: string) => Promise<void>;
  streamGuidedWrite: (prompt: string, context?: string) => Promise<void>;
  
  // Streaming state
  streamingState: StreamingState;
  
  // Streaming controls
  pauseStream: () => void;
  resumeStream: () => void;
  stopStream: () => void;
}

export interface UseAITextProcessorReturn {
  // Text processing functions
  rewriteText: (text: string, instructions?: string) => Promise<AIResponse>;
  expandText: (text: string, direction?: 'before' | 'after' | 'both') => Promise<AIResponse>;
  quickEdit: (text: string, edit: string) => Promise<AIResponse>;
  
  // State
  isProcessing: boolean;
  processingError: AIError | null;
}

export interface UseAICreativeReturn {
  // Creative functions
  brainstorm: (topic: string, context?: string) => Promise<AIResponse>;
  describeScene: (scene: string, style?: string) => Promise<AIResponse>;
  visualizeScene: (description: string) => Promise<AIResponse>;
  
  // State
  isGenerating: boolean;
  generationError: AIError | null;
}

export interface UseAISettingsReturn {
  // Settings
  settings: AISettings;
  providers: AIProvider[];
  
  // Actions
  updateSettings: (updates: Partial<AISettings>) => void;
  updateProvider: (providerId: string, updates: Partial<AIProvider>) => void;
  testConnection: (providerId: string) => Promise<boolean>;
  resetToDefaults: () => void;
  saveSettings: () => Promise<void>;
}

export interface UseAICreditsReturn {
  // Credit data
  credits: CreditBalance;
  usage: CreditUsage[];
  providers: Record<string, { credits: number; cost: number }>;
  
  // Actions
  refreshCredits: () => Promise<void>;
  updateCreditLimit: (limit: number) => void;
  setLowCreditAlert: (threshold: number) => void;
  getUsageHistory: (days: number) => CreditUsage[];
  getProviderUsage: () => Record<string, { credits: number; cost: number }>;
  estimateCredits: (tool: AITool, inputLength: number) => CreditEstimate;
}

// Event Types
export interface AIEvent {
  type: 'request' | 'response' | 'error' | 'stream_start' | 'stream_chunk' | 'stream_end';
  timestamp: Date;
  data: any;
}

export interface AIEventListener {
  (event: AIEvent): void;
}

// Configuration Types
export interface AIConfiguration {
  providers: Record<string, AIProvider>;
  models: Record<string, AIModel>;
  tools: Record<AITool, AIToolConfig>;
  settings: AISettings;
  features: {
    streaming: boolean;
    contextAwareness: boolean;
    creditTracking: boolean;
    analytics: boolean;
  };
}