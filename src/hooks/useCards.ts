import { useEffect } from 'react';
import { useCardStore } from '../stores/cardStore';

interface UseCardsOptions {
  projectId: number;
  documentId?: number;
  filterType?: string | null;
  showStarredOnly?: boolean;
  sortOrder?: 'newest' | 'oldest';
}

export const useCards = ({
  projectId,
  documentId,
  filterType = null,
  showStarredOnly = false,
  sortOrder = 'newest',
}: UseCardsOptions) => {
  const {
    cards,
    loading,
    error,
    fetchCards,
    toggleCollapse,
    toggleStar,
    deleteCard,
    setFilterType,
    setShowStarredOnly,
    setSortOrder,
  } = useCardStore();

  // Fetch cards when component mounts or when projectId/documentId changes
  useEffect(() => {
    fetchCards(projectId, documentId);
  }, [fetchCards, projectId, documentId]);

  // Apply filters and sorting
  useEffect(() => {
    setFilterType(filterType);
    setShowStarredOnly(showStarredOnly);
    setSortOrder(sortOrder);
  }, [filterType, showStarredOnly, sortOrder, setFilterType, setShowStarredOnly, setSortOrder]);

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
  }, {} as Record<string, typeof cards>);

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

  return {
    cards,
    cardsByFeatureType,
    featureTypes,
    loading,
    error,
    toggleCollapse,
    toggleStar,
    deleteCard,
    setFilterType,
    setShowStarredOnly,
    setSortOrder,
  };
};
