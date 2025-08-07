import React, { useEffect } from 'react';
import { CardStack } from './CardStack';
import { Button } from '../ui/Button';
import { FilterIcon, SortIcon } from '../ui/Icons';
import { useCardStore } from '../../stores/cardStore';

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
  const {
    cards,
    loading,
    error,
    filterType,
    sortOrder,
    showStarredOnly,
    fetchCards,
    toggleCollapse,
    toggleStar,
    deleteCard,
    setFilterType,
    setSortOrder,
    setShowStarredOnly,
  } = useCardStore();

  // Fetch cards from the database
  useEffect(() => {
    fetchCards(projectId, documentId);
  }, [projectId, documentId, fetchCards]);

  // Handle toggling card collapse state
  const handleToggleCollapse = async (cardId: number) => {
    await toggleCollapse(cardId);
    onCardAction?.('toggle_collapse', cardId);
  };

  // Handle toggling card star state
  const handleToggleStar = async (cardId: number) => {
    await toggleStar(cardId);
    onCardAction?.('toggle_star', cardId);
  };

  // Handle deleting a card
  const handleDeleteCard = async (cardId: number) => {
    await deleteCard(cardId);
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
