import { create } from 'zustand';
// Import commented out until we implement the actual Tauri backend calls
// import { invoke } from '@tauri-apps/api/tauri';
import { AICard } from '../components/cards/CardSystem';

interface CardState {
  cards: AICard[];
  loading: boolean;
  error: string | null;
  
  // Filters and sorting
  filterType: string | null;
  sortOrder: 'newest' | 'oldest';
  showStarredOnly: boolean;
  
  // Actions
  fetchCards: (projectId: number, documentId?: number) => Promise<void>;
  toggleCollapse: (cardId: number) => Promise<void>;
  toggleStar: (cardId: number) => Promise<void>;
  deleteCard: (cardId: number) => Promise<void>;
  updateCards: (cards: AICard[]) => void;
  setFilterType: (type: string | null) => void;
  setSortOrder: (order: 'newest' | 'oldest') => void;
  setShowStarredOnly: (show: boolean) => void;
}

export const useCardStore = create<CardState>((set, get) => ({
  cards: [],
  loading: false,
  error: null,
  
  filterType: null,
  sortOrder: 'newest',
  showStarredOnly: false,
  
  fetchCards: async (projectId: number, documentId?: number) => {
    try {
      set({ loading: true, error: null });
      
      // In a real implementation, this would be a call to the Tauri backend
      // For now, we'll simulate with mock data
      // const cards = await invoke<AICard[]>('get_ai_response_cards', { projectId, documentId });
      
      // Mock data for development
      const mockCards: AICard[] = [
        {
          id: 1,
          projectId,
          documentId,
          featureType: 'Brainstorm',
          promptContext: 'Help me brainstorm ideas for my protagonist',
          responseText: 'Here are some character ideas for your protagonist:\n\n1. A former detective who left the force after a case went wrong\n2. A botanist who discovers a plant with unusual properties\n3. A librarian who can hear the whispers of books\n4. A chef who can taste emotions in food',
          isStacked: false,
          isStarred: true,
          isCollapsed: false,
          createdAt: new Date().toISOString(),
        },
        {
          id: 2,
          projectId,
          documentId,
          featureType: 'Expand',
          promptContext: 'Expand on the setting description',
          responseText: 'The small coastal town of Harborview sits perched on rocky cliffs overlooking the turbulent Pacific. Victorian houses in faded pastels line narrow streets that wind up from the harbor. The air always carries the scent of salt and pine, and fog rolls in most evenings, transforming familiar landmarks into ghostly silhouettes.',
          isStacked: false,
          isStarred: false,
          isCollapsed: true,
          createdAt: new Date(Date.now() - 86400000).toISOString(), // 1 day ago
        },
        {
          id: 3,
          projectId,
          documentId,
          featureType: 'Rewrite',
          promptContext: 'Rewrite this dialogue to be more tense',
          responseText: '"I told you not to come back here." His voice dropped to a whisper, but the threat in it filled the room.\n\nShe stepped closer, not breaking eye contact. "You don\'t get to decide that anymore."\n\nHis hand twitched toward the drawer. "Last warning."\n\n"Too late for warnings," she said, revealing what she\'d been holding behind her back.',
          isStacked: false,
          isStarred: true,
          isCollapsed: false,
          createdAt: new Date(Date.now() - 172800000).toISOString(), // 2 days ago
        },
      ];
      
      set({ cards: mockCards, loading: false });
    } catch (err) {
      console.error('Error fetching cards:', err);
      set({ error: 'Failed to load AI response cards', loading: false });
    }
  },
  
  toggleCollapse: async (cardId: number) => {
    try {
      const { cards } = get();
      const updatedCards = cards.map(card => 
        card.id === cardId ? { ...card, isCollapsed: !card.isCollapsed } : card
      );
      
      set({ cards: updatedCards });
      
      // In a real implementation, this would update the database
      // await invoke('update_ai_card_collapse', { cardId, isCollapsed: !cards.find(c => c.id === cardId)?.isCollapsed });
    } catch (err) {
      console.error('Error toggling card collapse:', err);
      set({ error: 'Failed to update card' });
    }
  },
  
  toggleStar: async (cardId: number) => {
    try {
      const { cards } = get();
      const updatedCards = cards.map(card => 
        card.id === cardId ? { ...card, isStarred: !card.isStarred } : card
      );
      
      set({ cards: updatedCards });
      
      // In a real implementation, this would update the database
      // await invoke('update_ai_card_star', { cardId, isStarred: !cards.find(c => c.id === cardId)?.isStarred });
    } catch (err) {
      console.error('Error toggling card star:', err);
      set({ error: 'Failed to update card' });
    }
  },
  
  deleteCard: async (cardId: number) => {
    try {
      const { cards } = get();
      const updatedCards = cards.filter(card => card.id !== cardId);
      
      set({ cards: updatedCards });
      
      // In a real implementation, this would delete from the database
      // await invoke('delete_ai_card', { cardId });
    } catch (err) {
      console.error('Error deleting card:', err);
      set({ error: 'Failed to delete card' });
    }
  },
  
  setFilterType: (type: string | null) => set({ filterType: type }),
  
  setSortOrder: (order: 'newest' | 'oldest') => set({ sortOrder: order }),
  
  updateCards: (cards: AICard[]) => set({ cards }),
  
  setShowStarredOnly: (show: boolean) => set({ showStarredOnly: show }),
}));
