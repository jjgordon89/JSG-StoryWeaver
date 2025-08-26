// AI Component Exports
export { default as AISelectionMenu } from './AISelectionMenu';
export { default as StreamingText } from './StreamingText';
export { default as AIWritingPanel } from './AIWritingPanel';
export { default as AIQuickTools } from './AIQuickTools';
export { default as AICreditManager } from './AICreditManager';
export { default as AISettingsPanel } from './AISettingsPanel';
export { GuidedSuggestions } from './GuidedSuggestions';

// Re-export types if needed
export type {
  AITool,
  AIProvider,
  StreamingState,
  CreditUsage
} from '../../types/ai';

// Component props types
export interface AIComponentProps {
  className?: string;
  onInsertText?: (text: string) => void;
  onReplaceText?: (text: string) => void;
  selectedText?: string;
  documentContext?: string;
}

export interface AISelectionMenuProps extends AIComponentProps {
  position: { x: number; y: number };
  onClose: () => void;
  visible: boolean;
}

export interface AIWritingPanelProps extends AIComponentProps {
  isOpen: boolean;
  onToggle: () => void;
}

export interface AIQuickToolsProps extends AIComponentProps {
  compact?: boolean;
}

export interface StreamingTextProps {
  text: string;
  isStreaming: boolean;
  onPause?: () => void;
  onResume?: () => void;
  onStop?: () => void;
  className?: string;
}

export interface AICreditManagerProps {
  className?: string;
}

export interface AISettingsPanelProps {
  className?: string;
}