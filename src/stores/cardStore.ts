import { create } from 'zustand';
import { invoke } from '../utils/tauriSafe';
import { AICard } from '../components/cards/CardSystem';

interface CardState {
  cards: AICard[];
  loading: boolean;
  error: string | null;
  
  filterType: string | null;
  sortOrder: 'newest' | 'oldest';
  showStarredOnly: boolean;
  
  fetchCards: (projectId: number, documentId?: number) => Promise<void>;
  toggleCollapse: (cardId: number) => Promise<void>;
  toggleStar: (cardId: number) => Promise<void>;
  deleteCard: (cardId: number) => Promise<void>;
  setFilterType: (type: string | null) => void;
  setSortOrder: (order: 'newest' | 'oldest') => void;
  updateCards: (cards: AICard[]) => void;
  setShowStarredOnly: (show: boolean) => void;
  addCard: (cardData: {
    content: string;
    type: string;
    documentId: number;
    projectId: number;
    metadata?: any;
  }) => Promise<void>;
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
      
      let cards: AICard[];
      if (documentId) {
        cards = await invoke<AICard[]>('get_ai_cards_by_document', { documentId });
      } else {
        cards = await invoke<AICard[]>('get_ai_cards_by_project', { projectId });
      }
      
      set({ cards, loading: false });
    } catch (err) {
      console.error('Error fetching cards:', err);
      set({ error: 'Failed to load AI response cards', loading: false });
    }
  },
  
  toggleCollapse: async (cardId: number) => {
    try {
      await invoke('toggle_ai_card_collapse', { cardId });
      
      const { cards } = get();
      const updatedCards = cards.map(card => 
        card.id === cardId ? { ...card, isCollapsed: !card.isCollapsed } : card
      );
      
      set({ cards: updatedCards });
    } catch (err) {
      console.error('Error toggling card collapse:', err);
      set({ error: 'Failed to update card' });
    }
  },
  
  toggleStar: async (cardId: number) => {
    try {
      await invoke('toggle_ai_card_star', { cardId });
      
      const { cards } = get();
      const updatedCards = cards.map(card => 
        card.id === cardId ? { ...card, isStarred: !card.isStarred } : card
      );
      
      set({ cards: updatedCards });
    } catch (err) {
      console.error('Error toggling card star:', err);
      set({ error: 'Failed to update card' });
    }
  },
  
  deleteCard: async (cardId: number) => {
    try {
      await invoke('delete_ai_card', { cardId });
      
      const { cards } = get();
      const updatedCards = cards.filter(card => card.id !== cardId);
      
      set({ cards: updatedCards });
    } catch (err) {
      console.error('Error deleting card:', err);
      set({ error: 'Failed to delete card' });
    }
  },
  
  setFilterType: (type: string | null) => set({ filterType: type }),
  
  setSortOrder: (order: 'newest' | 'oldest') => set({ sortOrder: order }),
  
  updateCards: (cards: AICard[]) => set({ cards }),
  
  setShowStarredOnly: (show: boolean) => set({ showStarredOnly: show }),
  
  addCard: async (cardData: {
    content: string;
    type: string;
    documentId: number;
    projectId: number;
    metadata?: any;
  }) => {
    try {
      const aiCardData = {
        project_id: cardData.projectId,
        document_id: cardData.documentId,
        feature_type: cardData.type,
        prompt_context: cardData.metadata?.prompt || '',
        response_text: cardData.content,
        model_used: 'gpt-4',
        token_count: Math.ceil(cardData.content.length / 4), // Rough estimate
        cost_estimate: 0.001,
        is_stacked: false,
        is_starred: false,
        is_collapsed: false
      };
      
      const result = await invoke('create_ai_card', { cardData: aiCardData });
      if (result.success) {
        // Refresh cards to include the new one
        const { fetchCards } = get();
        await fetchCards(cardData.projectId, cardData.documentId);
      }
    } catch (err) {
      console.error('Error adding card:', err);
      set({ error: 'Failed to add card' });
    }
  },
}));
