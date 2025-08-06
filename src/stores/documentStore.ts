import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface Document {
  id: number;
  project_id: number;
  name: string;
  content: string;
  word_count: number;
  created_at: string;
  updated_at: string;
}

interface DocumentState {
  documents: Document[];
  currentDocument: Document | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  loadDocument: (documentId: number) => Promise<void>;
  saveDocument: (documentId: number, content: string) => Promise<void>;
  createDocument: (projectId: number, name: string) => Promise<Document>;
  updateDocument: (documentId: number, updates: Partial<Document>) => Promise<void>;
  deleteDocument: (documentId: number) => Promise<void>;
  setCurrentDocument: (document: Document | null) => void;
}

export const useStore = create<DocumentState>((set, get) => ({
  documents: [],
  currentDocument: null,
  isLoading: false,
  error: null,

  loadDocument: async (documentId: number) => {
    set({ isLoading: true, error: null });
    try {
      const document = await invoke<Document>('get_document', { documentId });
      set({ currentDocument: document, isLoading: false });
    } catch (error) {
      set({ error: error as string, isLoading: false });
    }
  },

  saveDocument: async (documentId: number, content: string) => {
    try {
      await invoke('save_document', { documentId, content });
      
      // Update the current document if it matches
      const { currentDocument } = get();
      if (currentDocument && currentDocument.id === documentId) {
        set({
          currentDocument: {
            ...currentDocument,
            content,
