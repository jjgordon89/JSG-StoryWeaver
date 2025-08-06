import React, { useState } from 'react';
import { AIResponseCard, AIResponseCardProps } from './AIResponseCard';
import { Card, CardHeader, CardContent } from '../ui/Card';
import { Button } from '../ui/Button';
import { ChevronDownIcon, ChevronUpIcon } from '../ui/Icons';

export interface CardStackProps {
  title: string;
  cards: Omit<AIResponseCardProps, 'onToggleCollapse' | 'onToggleStar' | 'onDelete'>[];
  onToggleCollapse: (id: number) => void;
  onToggleStar: (id: number) => void;
  onDelete?: (id: number) => void;
}

export const CardStack: React.FC<CardStackProps> = ({
  title,
  cards,
  onToggleCollapse,
  onToggleStar,
  onDelete,
}) => {
  const [isExpanded, setIsExpanded] = useState(true);
  
  // Count starred cards
  const starredCount = cards.filter(card => card.isStarred).length;
  
  return (
    <Card className="mb-6">
      <CardHeader className="flex flex-row items-center justify-between py-2 px-4 bg-slate-200 dark:bg-slate-700">
        <div className="flex items-center">
          <h3 className="font-medium">{title}</h3>
          {starredCount > 0 && (
            <span className="ml-2 text-xs bg-yellow-100 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-100 px-2 py-0.5 rounded-full">
              {starredCount} starred
            </span>
          )}
          <span className="ml-2 text-xs text-slate-500">
            {cards.length} {cards.length === 1 ? 'card' : 'cards'}
          </span>
        </div>
        <Button
          variant="ghost"
          size="sm"
          className="p-1"
          onClick={() => setIsExpanded(!isExpanded)}
          aria-label={isExpanded ? "Collapse stack" : "Expand stack"}
        >
          {isExpanded ? (
            <ChevronUpIcon className="h-4 w-4" />
          ) : (
            <ChevronDownIcon className="h-4 w-4" />
          )}
        </Button>
      </CardHeader>
      
      {isExpanded && (
        <CardContent className="py-4 px-4">
          {cards.length === 0 ? (
            <div className="text-center text-slate-500 py-4">
              No cards in this stack
            </div>
          ) : (
            cards.map((card) => (
              <AIResponseCard
                key={card.id}
                {...card}
                onToggleCollapse={onToggleCollapse}
                onToggleStar={onToggleStar}
                onDelete={onDelete}
              />
            ))
          )}
        </CardContent>
      )}
    </Card>
  );
};
