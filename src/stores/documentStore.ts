import { create } from 'zustand';
import { invoke } from '../utils/tauriSafe';

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
            updated_at: new Date().toISOString(),
          },
        });
      }
    } catch (error) {
      set({ error: error as string });
    }
  },

  createDocument: async (projectId: number, name: string) => {
    set({ isLoading: true, error: null });
    try {
      const document = await invoke<Document>('create_document', { projectId, name });
      set((state) => ({
        documents: [...state.documents, document],
        isLoading: false,
      }));
      return document;
    } catch (error) {
      set({ error: error as string, isLoading: false });
      throw error;
    }
  },

  updateDocument: async (documentId: number, updates: Partial<Document>) => {
    try {
      await invoke('update_document', { documentId, updates });
      
      // Update the documents array
      set((state) => ({
        documents: state.documents.map((doc) =>
          doc.id === documentId ? { ...doc, ...updates } : doc
        ),
      }));
      
      // Update current document if it matches
      const { currentDocument } = get();
      if (currentDocument && currentDocument.id === documentId) {
        set({
          currentDocument: { ...currentDocument, ...updates },
        });
      }
    } catch (error) {
      set({ error: error as string });
    }
  },

  deleteDocument: async (documentId: number) => {
    try {
      await invoke('delete_document', { documentId });
      
      set((state) => ({
        documents: state.documents.filter((doc) => doc.id !== documentId),
        currentDocument:
          state.currentDocument?.id === documentId ? null : state.currentDocument,
      }));
    } catch (error) {
      set({ error: error as string });
    }
  },

  setCurrentDocument: (document: Document | null) => {
    set({ currentDocument: document });
  },
}));

export const useDocumentStore = useStore;
