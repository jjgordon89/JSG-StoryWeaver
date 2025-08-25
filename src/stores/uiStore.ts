import { create } from 'zustand';

export type SaveStatus = 'saved' | 'saving' | 'error';
export type AITool =
  | 'write'
  | 'rewrite'
  | 'expand'
  | 'brainstorm'
  | 'describe'
  | 'visualize'
  | 'quickEdit'
  | 'chat';

interface UIState {
  // Editor status surfaced to the global header
  wordCount: number;
  saveStatus: SaveStatus;

  // Actions to update status
  setEditorStatus: (update: Partial<Pick<UIState, 'wordCount' | 'saveStatus'>>) => void;

  // AI panel orchestration
  aiPanelOpen: boolean;
  aiActiveTool: AITool;

  openAIPanel: (tool?: AITool) => void;
  closeAIPanel: () => void;
  toggleAIPanel: () => void;
  setAIActiveTool: (tool: AITool) => void;
}

export const useUIStore = create<UIState>((set, get) => ({
  wordCount: 0,
  saveStatus: 'saved',

  setEditorStatus: (update) =>
    set((state) => ({
      ...state,
      ...update,
    })),

  aiPanelOpen: false,
  aiActiveTool: 'write',

  openAIPanel: (tool) =>
    set((state) => ({
      aiPanelOpen: true,
      aiActiveTool: tool ?? state.aiActiveTool,
    })),
  closeAIPanel: () => set({ aiPanelOpen: false }),
  toggleAIPanel: () => set((state) => ({ aiPanelOpen: !state.aiPanelOpen })),
  setAIActiveTool: (tool) => set({ aiActiveTool: tool }),
}));

export default useUIStore;
