import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { invoke } from '../utils/tauriSafe';

interface SettingsState {
  // Focus mode settings
  focusModeEnabled: boolean;
  focusModeOptions: {
    hideNavigation: boolean;
    hideRightPanel: boolean;
    hideHeader: boolean;
    dimUI: boolean;
  };
  
  // Theme settings
  theme: 'light' | 'dark' | 'system';
  
  // Editor settings
  editorSettings: {
    fontSize: number;
    fontFamily: string;
    lineHeight: number;
    showLineNumbers: boolean;
    wordWrap: boolean;
    autoSave: boolean;
    autoSaveInterval: number;
  };
  
  // Accessibility settings
  accessibilitySettings: {
    highContrast: boolean;
    reducedMotion: boolean;
    largeText: boolean;
  };
  
  // Application settings
  appSettings: {
    confirmBeforeDelete: boolean;
    autoBackup: boolean;
    autoBackupInterval: number; // in minutes
    maxBackupCount: number;
  };
  
  // Sync status
  isSyncing: boolean;
  lastSyncTime: number | null;
  
  // Actions
  toggleFocusMode: () => void;
  updateFocusModeOptions: (options: Partial<SettingsState['focusModeOptions']>) => void;
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
  updateEditorSettings: (settings: Partial<SettingsState['editorSettings']>) => void;
  updateAccessibilitySettings: (settings: Partial<SettingsState['accessibilitySettings']>) => void;
  updateAppSettings: (settings: Partial<SettingsState['appSettings']>) => void;
  syncWithBackend: () => Promise<void>;
  loadFromBackend: () => Promise<void>;
}

export const useSettingsStore = create<SettingsState>()(devtools(
  persist(
    (set, get) => ({
      // Default focus mode settings
      focusModeEnabled: false,
      focusModeOptions: {
        hideNavigation: true,
        hideRightPanel: true,
        hideHeader: false,
        dimUI: true,
      },
      
      // Default theme settings
      theme: 'system',
      
      // Default editor settings
      editorSettings: {
        fontSize: 14,
        fontFamily: 'Monaco, Menlo, Consolas, "Courier New", monospace',
        lineHeight: 1.5,
        showLineNumbers: true,
        wordWrap: true,
        autoSave: true,
        autoSaveInterval: 5000, // 5 seconds
      },
      
      // Default accessibility settings
      accessibilitySettings: {
        highContrast: false,
        reducedMotion: false,
        largeText: false,
      },
      
      // Default application settings
      appSettings: {
        confirmBeforeDelete: true,
        autoBackup: true,
        autoBackupInterval: 60, // 60 minutes
        maxBackupCount: 10,
      },
      
      // Sync status
      isSyncing: false,
      lastSyncTime: null,
      
      // Actions
      toggleFocusMode: () => set((state) => ({ 
        focusModeEnabled: !state.focusModeEnabled 
      })),
      
      updateFocusModeOptions: (options) => set((state) => ({
        focusModeOptions: {
          ...state.focusModeOptions,
          ...options
        }
      })),
      
      setTheme: (theme) => set({ theme }),
      
      updateEditorSettings: (settings) => set((state) => ({
        editorSettings: {
          ...state.editorSettings,
          ...settings
        }
      })),
      
      updateAccessibilitySettings: (settings) => set((state) => ({
        accessibilitySettings: {
          ...state.accessibilitySettings,
          ...settings
        }
      })),
      
      updateAppSettings: (settings) => set((state) => ({
        appSettings: {
          ...state.appSettings,
          ...settings
        }
      })),
      
      // Sync settings with backend
      syncWithBackend: async () => {
        try {
          set({ isSyncing: true });
          
          // Get current settings
          const state = get();
          
          // Prepare settings object for backend
          const settings = {
            focusMode: {
              enabled: state.focusModeEnabled,
              options: state.focusModeOptions
            },
            theme: state.theme,
            editor: state.editorSettings,
            accessibility: state.accessibilitySettings,
            app: state.appSettings
          };
          
          // Send to backend
          await invoke('sync_settings', { 
            request: { settings } 
          });
          
          set({ 
            isSyncing: false,
            lastSyncTime: Date.now()
          });
          
          console.log('Settings synced with backend');
        } catch (error) {
          console.error('Failed to sync settings with backend:', error);
          set({ isSyncing: false });
        }
      },
      
      // Load settings from backend
      loadFromBackend: async () => {
        try {
          set({ isSyncing: true });
          
          // Get settings from backend
          const result = await invoke<any>('get_preferences_as_object', { 
            category: 'app' 
          });
          
          if (result && result.data) {
            const backendSettings = result.data;
            
            // Update local settings with backend data
            set((state) => ({
              focusModeEnabled: backendSettings.focusMode?.enabled ?? state.focusModeEnabled,
              focusModeOptions: {
                ...state.focusModeOptions,
                ...backendSettings.focusMode?.options
              },
              theme: backendSettings.theme ?? state.theme,
              editorSettings: {
                ...state.editorSettings,
                ...backendSettings.editor
              },
              accessibilitySettings: {
                ...state.accessibilitySettings,
                ...backendSettings.accessibility
              },
              appSettings: {
                ...state.appSettings,
                ...backendSettings.app
              },
              isSyncing: false,
              lastSyncTime: Date.now()
            }));
            
            console.log('Settings loaded from backend');
          }
        } catch (error) {
          console.error('Failed to load settings from backend:', error);
          set({ isSyncing: false });
        }
      }
    }),
    {
      name: 'storyweaver-settings',
      // When the store is hydrated from localStorage, sync with backend
      onRehydrateStorage: () => (state) => {
        if (state) {
          // Schedule sync after a short delay to ensure app is fully loaded
          setTimeout(() => {
            state.syncWithBackend();
          }, 1000);
        }
      }
    }
  ),
  { name: 'SettingsStore' }
));
