import { useEffect } from 'react';
import { useCardStore } from '../stores/cardStore';

/**
 * Configuration options for the useCards hook.
 */
interface UseCardsOptions {
  /** The project ID to fetch cards for */
  projectId: number;
  /** Optional document ID to filter cards by specific document */
  documentId?: number;
  /** Filter cards by feature type (e.g., 'write', 'rewrite', 'brainstorm') */
  filterType?: string | null;
  /** Show only starred cards when true */
  showStarredOnly?: boolean;
  /** Sort order for cards within each stack */
  sortOrder?: 'newest' | 'oldest';
}

/**
 * Hook for managing AI cards with filtering, sorting, and grouping functionality.
 * 
 * Provides a comprehensive interface for working with AI-generated cards, including
 * automatic fetching, filtering by type and starred status, sorting, and grouping
 * by feature type for stacked display.
 * 
 * @param {UseCardsOptions} options - Configuration options for card management
 * @returns {Object} Card management interface
 * @returns {Array} cards - All cards for the project/document
 * @returns {Object} cardsByFeatureType - Cards grouped by feature type for stacking
 * @returns {string[]} featureTypes - Unique feature types available for filtering
 * @returns {boolean} loading - Whether cards are currently being fetched
 * @returns {string|null} error - Current error state, if any
 * @returns {Function} toggleCollapse - Toggle collapse state of a card
 * @returns {Function} toggleStar - Toggle starred state of a card
 * @returns {Function} deleteCard - Delete a card
 * @returns {Function} addCard - Add a new card
 * @returns {Function} setFilterType - Update the filter type
 * @returns {Function} setShowStarredOnly - Update starred-only filter
 * @returns {Function} setSortOrder - Update sort order
 * 
 * @example
 * ```tsx
 * const {
 *   cardsByFeatureType,
 *   featureTypes,
 *   loading,
 *   toggleStar,
 *   setFilterType
 * } = useCards({
 *   projectId: 1,
 *   documentId: 5,
 *   filterType: 'write',
 *   showStarredOnly: false,
 *   sortOrder: 'newest'
 * });
 * 
 * // Display cards grouped by feature type
 * Object.entries(cardsByFeatureType).map(([featureType, cards]) => (
 *   <CardStack key={featureType} type={featureType} cards={cards} />
 * ));
 * ```
 */
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
    addCard,
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
    addCard,
    setFilterType,
    setShowStarredOnly,
    setSortOrder,
  };
};
