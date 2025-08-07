import { listen } from './tauriSafe';
import { useEffect } from 'react';
import { useSettingsStore } from '../stores/settingsStore';
import { useStore as useDocumentStore } from '../stores/documentStore';
import { useCardStore } from '../stores/cardStore';

// Check if we're running in Tauri environment
const isTauri = typeof window !== 'undefined' && window.__TAURI__ !== undefined;

// Define event types for state synchronization
export enum SyncEventType {
  DOCUMENT_UPDATED = 'document_updated',
  DOCUMENT_CREATED = 'document_created',
  DOCUMENT_DELETED = 'document_deleted',
  SETTINGS_UPDATED = 'settings_updated',
  CARD_UPDATED = 'card_updated',
  CARD_CREATED = 'card_created',
  CARD_DELETED = 'card_deleted',
  PROJECT_UPDATED = 'project_updated',
  FOLDER_UPDATED = 'folder_updated',
  SERIES_UPDATED = 'series_updated'
}

// Define payload types for each event
export interface DocumentSyncPayload {
  documentId: number;
  projectId: number;
  content?: string;
  name?: string;
  wordCount?: number;
  updatedAt?: string;
}

export interface SettingsSyncPayload {
  category: string;
  key: string;
  value: any;
}

export interface CardSyncPayload {
  cardId: number;
  projectId: number;
  documentId?: number;
  isStarred?: boolean;
  isCollapsed?: boolean;
  responseText?: string;
}

export interface ProjectSyncPayload {
  projectId: number;
  name?: string;
  description?: string;
  folderId?: number;
  seriesId?: number;
}

export interface FolderSyncPayload {
  folderId: number;
  name?: string;
  parentFolderId?: number;
}

export interface SeriesSyncPayload {
  seriesId: number;
  name?: string;
  description?: string;
  folderId?: number;
}

// Main hook for subscribing to state synchronization events
export function useStateSynchronization() {
  const documentStore = useDocumentStore();
  const settingsStore = useSettingsStore();
  const cardStore = useCardStore();

  useEffect(() => {
    // Only set up listeners if we're running in Tauri environment
    if (!isTauri) {
      return;
    }

    // Set up listeners for various state change events
    const unsubscribePromises: Promise<() => void>[] = [];

    // Document events
    unsubscribePromises.push(
      listen<DocumentSyncPayload>(SyncEventType.DOCUMENT_UPDATED, ({ payload }) => {
        // If this document is currently loaded, update it
        if (documentStore.currentDocument && documentStore.currentDocument.id === payload.documentId) {
          const updatedDoc = { ...documentStore.currentDocument };
          
          if (payload.content !== undefined) updatedDoc.content = payload.content;
          if (payload.name !== undefined) updatedDoc.name = payload.name;
          if (payload.wordCount !== undefined) updatedDoc.word_count = payload.wordCount;
          if (payload.updatedAt !== undefined) updatedDoc.updated_at = payload.updatedAt;
          
          documentStore.setCurrentDocument(updatedDoc);
        }
      })
    );

    // Settings events
    unsubscribePromises.push(
      listen<SettingsSyncPayload>(SyncEventType.SETTINGS_UPDATED, ({ payload }) => {
        // Update settings based on category
        if (payload.category === 'focusMode') {
          if (payload.key === 'enabled') {
            if (settingsStore.focusModeEnabled !== payload.value) {
              settingsStore.toggleFocusMode();
            }
          } else if (payload.key.startsWith('options.')) {
            const optionKey = payload.key.replace('options.', '') as keyof typeof settingsStore.focusModeOptions;
            settingsStore.updateFocusModeOptions({ [optionKey]: payload.value });
          }
        } else if (payload.category === 'theme') {
          settingsStore.setTheme(payload.value);
        } else if (payload.category === 'editor') {
          settingsStore.updateEditorSettings({ [payload.key]: payload.value });
        } else if (payload.category === 'accessibility') {
          settingsStore.updateAccessibilitySettings({ [payload.key]: payload.value });
        } else if (payload.category === 'app') {
          settingsStore.updateAppSettings({ [payload.key]: payload.value });
        }
      })
    );

    // Card events
    unsubscribePromises.push(
      listen<CardSyncPayload>(SyncEventType.CARD_UPDATED, ({ payload }) => {
        // Find and update the card in the store
        const cards = cardStore.cards;
        const cardIndex = cards.findIndex(card => card.id === payload.cardId);
        
        if (cardIndex !== -1) {
          const updatedCards = [...cards];
          const updatedCard = { ...updatedCards[cardIndex] };
          
          if (payload.isStarred !== undefined) updatedCard.isStarred = payload.isStarred;
          if (payload.isCollapsed !== undefined) updatedCard.isCollapsed = payload.isCollapsed;
          if (payload.responseText !== undefined) updatedCard.responseText = payload.responseText;
          
          updatedCards[cardIndex] = updatedCard;
          // We'll update the cardStore interface to include this method
          cardStore.updateCards(updatedCards);
        }
      })
    );

    // Clean up listeners when component unmounts
    return () => {
      unsubscribePromises.forEach(async (unsubscribePromise) => {
        const unsubscribe = await unsubscribePromise;
        unsubscribe();
      });
    };
  }, [documentStore, settingsStore, cardStore]);
}

// Helper function to emit synchronization events
export async function emitSyncEvent<T>(eventType: SyncEventType, payload: T) {
  // Use safe emit wrapper to avoid Tauri import errors in web environment
  const { emit } = await import('./tauriSafe');
  return emit(eventType, payload);
}

// Middleware for Zustand stores to automatically emit sync events
export function withSyncMiddleware<T extends object>(
  eventType: SyncEventType,
  mapStateToPayload: (state: T) => any
) {
  return (config: any) => (set: any, get: any, api: any) => {
    const originalSet = set;
    
    // Override the set function to emit sync events
    const syncSet = (...args: any[]) => {
      // Call the original set function
      originalSet(...args);
      
      // Get the updated state and map it to a payload
      const state = get() as T;
      const payload = mapStateToPayload(state);
      
      // Emit the sync event
      emitSyncEvent(eventType, payload).catch(err => {
        console.error(`Failed to emit sync event ${eventType}:`, err);
      });
    };
    
    return config(syncSet, get, api);
  };
}

// Helper function to create a payload mapper for sync middleware
export function createPayloadMapper<T>(selector: (state: T) => any) {
  return (state: T): any => selector(state);
}
