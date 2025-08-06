import React, { useState } from 'react';
import { CardStack } from './CardStack';
import { Button } from '../ui/Button';
import { FilterIcon, SortIcon, StarIcon } from '../ui/Icons';
import { useCards } from '../../hooks/useCards';
import { DropdownButton } from '../ui/Dropdown';

interface EnhancedCardSystemProps {
  projectId: number;
  documentId?: number;
}

export const EnhancedCardSystem: React.FC<EnhancedCardSystemProps> = ({
  projectId,
  documentId,
}) => {
  const [filterType, setFilterType] = useState<string | null>(null);
  const [sortOrder, setSortOrder] = useState<'newest' | 'oldest'>('newest');
  const [showStarredOnly, setShowStarredOnly] = useState(false);
  
  const {
    cardsByFeatureType,
    featureTypes,
    loading,
    error,
    toggleCollapse,
    toggleStar,
    deleteCard,
  } = useCards({
    projectId,
    documentId,
    filterType,
    sortOrder,
    showStarredOnly,
  });

  const handleFilterChange = (type: string | null) => {
    setFilterType(type === filterType ? null : type);
  };

  const handleSortChange = () => {
    setSortOrder(sortOrder === 'newest' ? 'oldest' : 'newest');
  };

  const handleStarredOnlyToggle = () => {
    setShowStarredOnly(!showStarredOnly);
  };

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
          <Button
            variant="outline"
            size="sm"
            className={`flex items-center ${showStarredOnly ? 'bg-yellow-50 border-yellow-200 dark:bg-yellow-900 dark:border-yellow-700' : ''}`}
            onClick={handleStarredOnlyToggle}
          >
            <StarIcon className={`h-4 w-4 mr-1 ${showStarredOnly ? 'text-yellow-500' : 'text-slate-400'}`} />
            {showStarredOnly ? 'Starred Only' : 'All Cards'}
          </Button>
          
          <Button
            variant="outline"
            size="sm"
            className="flex items-center"
            onClick={handleSortChange}
          >
            <SortIcon className="h-4 w-4 mr-1" />
            {sortOrder === 'newest' ? 'Newest First' : 'Oldest First'}
          </Button>
          
          <DropdownButton
            label={filterType || 'All Types'}
            icon={<FilterIcon className="h-4 w-4" />}
            items={[
              { label: 'All Types', value: 'all', selected: filterType === null },
              ...featureTypes.map(type => ({
                label: type,
                value: type,
                selected: filterType === type,
              })),
            ]}
            onSelect={(value) => handleFilterChange(value === 'all' ? null : value)}
            variant="outline"
            size="sm"
          />
        </div>
      </div>

      {Object.keys(cardsByFeatureType).length === 0 ? (
        <div className="text-center py-8 text-slate-500">
          No AI response cards found
          {filterType && <div>Try removing the filter</div>}
          {showStarredOnly && <div>Try showing all cards</div>}
        </div>
      ) : (
        Object.entries(cardsByFeatureType).map(([featureType, cardsInStack]) => (
          <CardStack
            key={featureType}
            title={featureType}
            cards={cardsInStack}
            onToggleCollapse={toggleCollapse}
            onToggleStar={toggleStar}
            onDelete={deleteCard}
          />
        ))
      )}
    </div>
  );
};
