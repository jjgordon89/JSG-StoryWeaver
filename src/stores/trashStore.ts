import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export enum DeletedItemType {
  Project = 'project',
  Folder = 'folder',
  Document = 'document',
  Series = 'series',
  Character = 'character',
  Location = 'location',
}

export interface DeletedItem {
  id: string;
  item_type: DeletedItemType;
  item_id: string;
  item_data: string; // JSON string of the original item
  parent_id: string | null;
  deletion_reason: string | null;
  deleted_at: string;
  can_restore: boolean;
}

interface TrashState {
  items: DeletedItem[];
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchTrashItems: () => Promise<void>;
  fetchTrashItemsByType: (itemType: DeletedItemType) => Promise<void>;
  fetchTrashItemsByParent: (parentId: string) => Promise<void>;
  restoreItem: (itemId: string) => Promise<void>;
  permanentlyDeleteItem: (itemId: string) => Promise<void>;
  emptyTrash: () => Promise<void>;
}

export const useTrashStore = create<TrashState>((set, get) => ({
  items: [],
  isLoading: false,
  error: null,
  
  fetchTrashItems: async () => {
    set({ isLoading: true, error: null });
    try {
      const items = await invoke<DeletedItem[]>('get_trash_items');
      set({ items, isLoading: false });
    } catch (error) {
      console.error('Failed to fetch trash items:', error);
      set({ error: `Failed to fetch trash items: ${error}`, isLoading: false });
    }
  },
  
  fetchTrashItemsByType: async (itemType: DeletedItemType) => {
    set({ isLoading: true, error: null });
    try {
      const items = await invoke<DeletedItem[]>('get_trash_items_by_type', { itemType });
      set({ items, isLoading: false });
    } catch (error) {
      console.error('Failed to fetch trash items by type:', error);
      set({ error: `Failed to fetch trash items by type: ${error}`, isLoading: false });
    }
  },
  
  fetchTrashItemsByParent: async (parentId: string) => {
    set({ isLoading: true, error: null });
    try {
      const items = await invoke<DeletedItem[]>('get_trash_items_by_parent', { parentId });
      set({ items, isLoading: false });
    } catch (error) {
      console.error('Failed to fetch trash items by parent:', error);
      set({ error: `Failed to fetch trash items by parent: ${error}`, isLoading: false });
    }
  },
  
  restoreItem: async (itemId: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('restore_trash_item', { deletedItemId: itemId });
      // Remove the restored item from the list
      set((state) => ({
        items: state.items.filter(item => item.id !== itemId),
        isLoading: false
      }));
    } catch (error) {
      console.error('Failed to restore item:', error);
      set({ error: `Failed to restore item: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  permanentlyDeleteItem: async (itemId: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('permanently_delete_trash_item', { deletedItemId: itemId });
      // Remove the deleted item from the list
      set((state) => ({
        items: state.items.filter(item => item.id !== itemId),
        isLoading: false
      }));
    } catch (error) {
      console.error('Failed to permanently delete item:', error);
      set({ error: `Failed to permanently delete item: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  emptyTrash: async () => {
    set({ isLoading: true, error: null });
    try {
      await invoke('empty_trash');
      set({ items: [], isLoading: false });
    } catch (error) {
      console.error('Failed to empty trash:', error);
      set({ error: `Failed to empty trash: ${error}`, isLoading: false });
      throw error;
    }
  },
}));
