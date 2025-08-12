import React, { useState } from 'react';
import { Card, CardContent, CardFooter, CardHeader } from '../../ui/components/common';
import { Button } from '../../ui/components/common';
import { StarIcon, ChevronDownIcon, ChevronUpIcon } from '../ui/Icons';

export interface AIResponseCardProps {
  id: number;
  featureType: string;
  promptContext?: string;
  responseText: string;
  isStarred: boolean;
  isCollapsed: boolean;
  createdAt: string;
  onToggleCollapse: (id: number) => void;
  onToggleStar: (id: number) => void;
  onDelete?: (id: number) => void;
}

export const AIResponseCard: React.FC<AIResponseCardProps> = ({
  id,
  featureType,
  promptContext,
  responseText,
  isStarred,
  isCollapsed,
  createdAt,
  onToggleCollapse,
  onToggleStar,
  onDelete,
}) => {
  const [isHovered, setIsHovered] = useState(false);
  
  const formattedDate = new Date(createdAt).toLocaleString();
  
  return (
    <Card
      className={`mb-4 transition-all duration-200 ${isCollapsed ? 'max-h-24 overflow-hidden' : ''}`}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <CardHeader className="flex flex-row items-center justify-between py-2 px-4 bg-slate-100 dark:bg-slate-800">
        <div className="flex items-center">
          <span className="font-medium text-sm">{featureType}</span>
          <span className="ml-2 text-xs text-slate-500">{formattedDate}</span>
        </div>
        <div className="flex items-center">
          <Button
            variant="ghost"
            size="sm"
            className={`p-1 ${isStarred ? 'text-yellow-500' : 'text-slate-400'}`}
            onClick={() => onToggleStar(id)}
            aria-label={isStarred ? "Unstar" : "Star"}
          >
            <StarIcon className="h-4 w-4" />
          </Button>
          <Button
            variant="ghost"
            size="sm"
            className="p-1 text-slate-400"
            onClick={() => onToggleCollapse(id)}
            aria-label={isCollapsed ? "Expand" : "Collapse"}
          >
            {isCollapsed ? (
              <ChevronDownIcon className="h-4 w-4" />
            ) : (
              <ChevronUpIcon className="h-4 w-4" />
            )}
          </Button>
        </div>
      </CardHeader>
      
      <CardContent className="py-3 px-4">
        {promptContext && !isCollapsed && (
          <div className="mb-3 text-sm italic text-slate-600 dark:text-slate-400 bg-slate-50 dark:bg-slate-900 p-2 rounded">
            {promptContext}
          </div>
        )}
        <div className="text-sm whitespace-pre-wrap">
          {isCollapsed ? `${responseText.substring(0, 100)}${responseText.length > 100 ? '...' : ''}` : responseText}
        </div>
      </CardContent>
      
      {!isCollapsed && onDelete && (
        <CardFooter className="py-2 px-4 flex justify-end">
          {isHovered && (
            <Button
              variant="ghost"
              size="sm"
              className="text-red-500 hover:bg-red-50 dark:hover:bg-red-900"
              onClick={() => onDelete(id)}
            >
              Delete
            </Button>
          )}
        </CardFooter>
      )}
    </Card>
  );
};
