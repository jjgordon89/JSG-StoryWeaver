import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface DocumentVersion {
  id: string;
  document_id: string;
  content: string;
  word_count: number;
  version_number: number;
  created_at: string;
  created_by: string | null;
  comment: string | null;
}

export interface VersionHistoryItem {
  id: string;
  version_number: number;
  word_count: number;
  created_at: string;
  created_by: string | null;
  comment: string | null;
  word_count_change: number;
}

export interface VersionStatistics {
  totalVersions: number;
  documentsWithVersions: number;
  oldestVersion: string | null;
  newestVersion: string | null;
  averageVersionsPerDocument: number;
}

interface VersionState {
  versions: DocumentVersion[];
  versionHistory: VersionHistoryItem[];
  currentDocumentId: string | null;
  selectedVersionId: string | null;
  isLoading: boolean;
  error: string | null;
  versionStatistics: VersionStatistics | null;
  isLoadingStatistics: boolean;
  
  // Actions
  fetchVersions: (documentId: string) => Promise<void>;
  fetchVersionHistory: (documentId: string) => Promise<void>;
  getVersion: (versionId: string) => Promise<DocumentVersion | null>;
  createVersion: (documentId: string, comment?: string) => Promise<DocumentVersion>;
  restoreVersion: (versionId: string) => Promise<void>;
  deleteVersion: (versionId: string) => Promise<void>;
  deleteAllVersions: (documentId: string) => Promise<void>;
  fetchVersionStatistics: () => Promise<VersionStatistics>;
  setCurrentDocumentId: (documentId: string | null) => void;
  setSelectedVersionId: (versionId: string | null) => void;
}

export const useVersionStore = create<VersionState>((set, get) => ({
  versions: [],
  versionHistory: [],
  currentDocumentId: null,
  selectedVersionId: null,
  isLoading: false,
  error: null,
  versionStatistics: null,
  isLoadingStatistics: false,
  
  fetchVersions: async (documentId: string) => {
    set({ isLoading: true, error: null, currentDocumentId: documentId });
    try {
      const versions = await invoke<DocumentVersion[]>('get_document_versions', { documentId });
      set({ versions, isLoading: false });
    } catch (error) {
      console.error('Failed to fetch document versions:', error);
      set({ error: `Failed to fetch document versions: ${error}`, isLoading: false });
    }
  },
  
  fetchVersionHistory: async (documentId: string) => {
    set({ isLoading: true, error: null, currentDocumentId: documentId });
    try {
      const versionHistory = await invoke<VersionHistoryItem[]>('get_version_history', { documentId });
      set({ versionHistory, isLoading: false });
    } catch (error) {
      console.error('Failed to fetch version history:', error);
      set({ error: `Failed to fetch version history: ${error}`, isLoading: false });
    }
  },
  
  getVersion: async (versionId: string) => {
    set({ isLoading: true, error: null });
    try {
      const version = await invoke<DocumentVersion | null>('get_document_version', { versionId });
      set({ isLoading: false });
      return version;
    } catch (error) {
      console.error('Failed to get document version:', error);
      set({ error: `Failed to get document version: ${error}`, isLoading: false });
      return null;
    }
  },
  
  createVersion: async (documentId: string, comment?: string) => {
    set({ isLoading: true, error: null });
    try {
      const version = await invoke<DocumentVersion>('create_document_version', { 
        documentId, 
        comment 
      });
      
      // Update versions list if this is for the current document
      if (get().currentDocumentId === documentId) {
        set((state) => ({
          versions: [version, ...state.versions],
        }));
        
        // Also update version history
        await get().fetchVersionHistory(documentId);
      }
      
      set({ isLoading: false });
      return version;
    } catch (error) {
      console.error('Failed to create document version:', error);
      set({ error: `Failed to create document version: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  restoreVersion: async (versionId: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('restore_document_version', { versionId });
      
      // If we have a current document ID, refresh the versions
      const { currentDocumentId } = get();
      if (currentDocumentId) {
        await get().fetchVersions(currentDocumentId);
        await get().fetchVersionHistory(currentDocumentId);
      }
      
      set({ isLoading: false });
    } catch (error) {
      console.error('Failed to restore document version:', error);
      set({ error: `Failed to restore document version: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  deleteVersion: async (versionId: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_document_version', { versionId });
      
      // Update versions list
      set((state) => ({
        versions: state.versions.filter(v => v.id !== versionId),
        versionHistory: state.versionHistory.filter(v => v.id !== versionId),
        isLoading: false
      }));
      
      // Clear selected version if it was deleted
      if (get().selectedVersionId === versionId) {
        set({ selectedVersionId: null });
      }
    } catch (error) {
      console.error('Failed to delete document version:', error);
      set({ error: `Failed to delete document version: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  deleteAllVersions: async (documentId: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_all_document_versions', { documentId });
      
      // If this is the current document, clear the versions
      if (get().currentDocumentId === documentId) {
        set({ 
          versions: [],
          versionHistory: [],
          selectedVersionId: null,
          isLoading: false
        });
      } else {
        set({ isLoading: false });
      }
    } catch (error) {
      console.error('Failed to delete all document versions:', error);
      set({ error: `Failed to delete all document versions: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  setCurrentDocumentId: (documentId: string | null) => {
    set({ currentDocumentId: documentId });
    if (documentId === null) {
      set({ versions: [], versionHistory: [], selectedVersionId: null });
    }
  },
  
  setSelectedVersionId: (versionId: string | null) => {
    set({ selectedVersionId: versionId });
  },
  
  fetchVersionStatistics: async () => {
    set({ isLoadingStatistics: true, error: null });
    try {
      // In a real implementation, this would call a Tauri command
      // For now, we'll simulate the API call with a delay and mock data
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const statistics: VersionStatistics = {
        totalVersions: 42,
        documentsWithVersions: 15,
        oldestVersion: '2025-05-01T12:00:00Z',
        newestVersion: '2025-08-05T15:30:00Z',
        averageVersionsPerDocument: 2.8
      };
      
      set({ versionStatistics: statistics, isLoadingStatistics: false });
      return statistics;
    } catch (error) {
      console.error('Failed to fetch version statistics:', error);
      set({ error: `Failed to fetch version statistics: ${error}`, isLoadingStatistics: false });
      throw error;
    }
  }
}));
