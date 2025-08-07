import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

export interface BackupInfo {
  id: string;
  filename: string;
  created_at: string;
  is_auto: boolean;
  comment: string | null;
  file_exists: boolean;
  file_size: number | null;
}

interface BackupState {
  backups: BackupInfo[];
  isLoading: boolean;
  error: string | null;
  
  // Actions
  fetchBackups: () => Promise<void>;
  createBackup: (backupName?: string) => Promise<string>;
  restoreBackup: (backupId: string) => Promise<void>;
  deleteBackup: (backupId: string) => Promise<void>;
  createAutoBackup: () => Promise<void>;
  cleanupOldBackups: () => Promise<void>;
}

export const useBackupStore = create<BackupState>((set, get) => ({
  backups: [],
  isLoading: false,
  error: null,
  
  fetchBackups: async () => {
    set({ isLoading: true, error: null });
    try {
      const backups = await invoke<BackupInfo[]>('get_backups');
      set({ backups, isLoading: false });
    } catch (error) {
      console.error('Failed to fetch backups:', error);
      set({ error: `Failed to fetch backups: ${error}`, isLoading: false });
    }
  },
  
  createBackup: async (backupName?: string) => {
    set({ isLoading: true, error: null });
    try {
      const backupPath = await invoke<string>('create_backup', { backupName });
      await get().fetchBackups();
      return backupPath;
    } catch (error) {
      console.error('Failed to create backup:', error);
      set({ error: `Failed to create backup: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  restoreBackup: async (backupId: string) => {
    set({ isLoading: true, error: null });
    try {
      // Find the backup filename from the ID
      const backup = get().backups.find(b => b.id === backupId);
      if (!backup) {
        throw new Error(`Backup with ID ${backupId} not found`);
      }
      
      await invoke('restore_from_backup', { backupFilename: backup.filename });
      set({ isLoading: false });
    } catch (error) {
      console.error('Failed to restore backup:', error);
      set({ error: `Failed to restore backup: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  deleteBackup: async (backupId: string) => {
    set({ isLoading: true, error: null });
    try {
      await invoke('delete_backup', { backupId });
      await get().fetchBackups();
    } catch (error) {
      console.error('Failed to delete backup:', error);
      set({ error: `Failed to delete backup: ${error}`, isLoading: false });
      throw error;
    }
  },
  
  createAutoBackup: async () => {
    try {
      await invoke('create_auto_backup');
      await get().fetchBackups();
    } catch (error) {
      console.error('Failed to create auto backup:', error);
      set({ error: `Failed to create auto backup: ${error}` });
      throw error;
    }
  },
  
  cleanupOldBackups: async () => {
    try {
      await invoke('cleanup_old_backups');
      await get().fetchBackups();
    } catch (error) {
      console.error('Failed to cleanup old backups:', error);
      set({ error: `Failed to cleanup old backups: ${error}` });
      throw error;
    }
  },
}));
