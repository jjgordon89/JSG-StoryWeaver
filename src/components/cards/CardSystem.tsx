import React, { useState, useEffect } from 'react';
import { CardStack } from './CardStack';
import { Button } from '../ui/Button';
import { FilterIcon, SortIcon } from '../ui/Icons';

export interface AICard {
  id: number;
  projectId: number;
  documentId?: number;
  featureType: string;
  promptContext?: string;
  responseText: string;
  isStacked: boolean;
  stackOrder?: number;
  isStarred: boolean;
  isCollapsed: boolean;
  createdAt: string;
}

interface CardSystemProps {
  projectId: number;
  documentId?: number;
  onCardAction?: (action: string, cardId: number) => void;
}

export const CardSystem: React.FC<CardSystemProps> = ({
  projectId,
  documentId,
  onCardAction,
}) => {
  const [cards, setCards] = useState<AICard[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [filterType, setFilterType] = useState<string | null>(null);
  const [sortOrder, setSortOrder] = useState<'newest' | 'oldest'>('newest');
  const [showStarredOnly, setShowStarredOnly] = useState(false);

  // Fetch cards from the database
  useEffect(() => {
    const fetchCards = async () => {
      try {
        setLoading(true);
        // In a real implementation, this would be a call to the Tauri backend
        // For now, we'll simulate with mock data
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
        
        setCards(mockCards);
        setLoading(false);
      } catch (err) {
        console.error('Error fetching cards:', err);
        setError('Failed to load AI response cards');
        setLoading(false);
      }
    };

    fetchCards();
  }, [projectId, documentId]);

  // Handle toggling card collapse state
  const handleToggleCollapse = (cardId: number) => {
    setCards(prevCards =>
      prevCards.map(card =>
        card.id === cardId ? { ...card, isCollapsed: !card.isCollapsed } : card
      )
    );
    
    onCardAction?.('toggle_collapse', cardId);
  };

  // Handle toggling card star state
  const handleToggleStar = (cardId: number) => {
    setCards(prevCards =>
      prevCards.map(card =>
        card.id === cardId ? { ...card, isStarred: !card.isStarred } : card
      )
    );
    
    onCardAction?.('toggle_star', cardId);
  };

  // Handle deleting a card
  const handleDeleteCard = (cardId: number) => {
    setCards(prevCards => prevCards.filter(card => card.id !== cardId));
    
    onCardAction?.('delete', cardId);
  };

  // Group cards by feature type for stacking
  const cardsByFeatureType = cards.reduce((acc, card) => {
    // Apply filters
    if (filterType && card.featureType !== filterType) return acc;
    if (showStarredOnly && !card.isStarred) return acc;
    
    if (!acc[card.featureType]) {
      acc[card.featureType] = [];
    }
    acc[card.featureType].push(card);
    return acc;
  }, {} as Record<string, AICard[]>);

  // Sort cards within each stack
  Object.keys(cardsByFeatureType).forEach(featureType => {
    cardsByFeatureType[featureType].sort((a, b) => {
      if (sortOrder === 'newest') {
        return new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime();
      } else {
        return new Date(a.createdAt).getTime() - new Date(b.createdAt).getTime();
      }
    });
  });

  // Get unique feature types for filter dropdown
  const featureTypes = Array.from(new Set(cards.map(card => card.featureType)));

  if (loading) {
    return <div className="p-4 text-center">Loading cards...</div>;
  }

  if (error) {
    return <div className="p-4 text-center text-red-500">{error}</div>;
  }

  return (
    <div className="card-system p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">AI Responses</h2>
        <div className="flex space-x-2">
          <div className="relative">
            <Button
              variant="outline"
              size="sm"
              className="flex items-center"
              onClick={() => setShowStarredOnly(!showStarredOnly)}
            >
              {showStarredOnly ? 'All Cards' : 'Starred Only'}
            </Button>
          </div>
          
          <div className="relative">
            <Button
              variant="outline"
              size="sm"
              className="flex items-center"
              onClick={() => setSortOrder(sortOrder === 'newest' ? 'oldest' : 'newest')}
            >
              <SortIcon className="h-4 w-4 mr-1" />
              {sortOrder === 'newest' ? 'Newest First' : 'Oldest First'}
            </Button>
          </div>
          
          <div className="relative">
            <Button
              variant="outline"
              size="sm"
              className="flex items-center"
              onClick={() => setFilterType(null)}
            >
              <FilterIcon className="h-4 w-4 mr-1" />
              {filterType || 'All Types'}
            </Button>
            {/* In a real implementation, this would be a dropdown menu */}
          </div>
        </div>
      </div>

      {Object.keys(cardsByFeatureType).length === 0 ? (
        <div className="text-center py-8 text-slate-500">
          No AI response cards found
          {filterType && <div>Try removing the filter</div>}
        </div>
      ) : (
        Object.entries(cardsByFeatureType).map(([featureType, cardsInStack]) => (
          <CardStack
            key={featureType}
            title={featureType}
            cards={cardsInStack}
            onToggleCollapse={handleToggleCollapse}
            onToggleStar={handleToggleStar}
            onDelete={handleDeleteCard}
          />
        ))
      )}
    </div>
  );
};
