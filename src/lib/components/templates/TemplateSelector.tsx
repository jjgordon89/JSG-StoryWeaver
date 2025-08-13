import React from 'react';
import type { CharacterTemplate, WorldBuildingTemplate } from '../../types/templates';

interface TemplateSelectorProps {
  templates: (CharacterTemplate | WorldBuildingTemplate)[];
  templateType: 'character' | 'worldbuilding';
  selectedTemplateId: string | null;
  isLoading: boolean;
  onSelect: (templateId: string, template: CharacterTemplate | WorldBuildingTemplate) => void;
  onApply: (templateId: string, template: CharacterTemplate | WorldBuildingTemplate) => void;
}

// Simple UI Components
const Card: React.FC<{ 
  className?: string; 
  onClick?: () => void;
  children: React.ReactNode;
}> = ({ className = '', onClick, children }) => (
  <div 
    className={`bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-sm ${onClick ? 'cursor-pointer' : ''} ${className}`}
    onClick={onClick}
  >
    {children}
  </div>
);

const CardHeader: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`p-4 pb-3 ${className}`}>
    {children}
  </div>
);

const CardContent: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`p-4 pt-0 ${className}`}>
    {children}
  </div>
);

const CardTitle: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <h3 className={`font-semibold text-gray-900 dark:text-white ${className}`}>
    {children}
  </h3>
);

const CardDescription: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <p className={`text-gray-600 dark:text-gray-400 ${className}`}>
    {children}
  </p>
);

const Badge: React.FC<{
  variant?: 'secondary' | 'outline';
  className?: string;
  children: React.ReactNode;
}> = ({ variant = 'secondary', className = '', children }) => {
  const variantClasses = variant === 'outline' 
    ? 'border border-gray-300 bg-transparent text-gray-700 dark:border-gray-600 dark:text-gray-300'
    : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200';
  
  return (
    <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${variantClasses} ${className}`}>
      {children}
    </span>
  );
};

const Button: React.FC<{
  size?: 'sm' | 'md';
  variant?: 'ghost' | 'solid';
  className?: string;
  onClick?: (e: React.MouseEvent) => void;
  children: React.ReactNode;
}> = ({ size = 'md', variant = 'solid', className = '', onClick, children }) => {
  const baseClasses = 'inline-flex items-center justify-center rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2';
  const sizeClasses = size === 'sm' ? 'h-8 w-8 p-0' : 'px-4 py-2';
  const variantClasses = variant === 'ghost' 
    ? 'text-gray-600 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700'
    : 'bg-blue-600 text-white hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600';
  
  return (
    <button
      className={`${baseClasses} ${sizeClasses} ${variantClasses} ${className}`}
      onClick={onClick}
    >
      {children}
    </button>
  );
};

const ScrollArea: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`overflow-y-auto ${className}`}>
    {children}
  </div>
);

export const TemplateSelector: React.FC<TemplateSelectorProps> = ({
  templates,
  templateType,
  selectedTemplateId,
  isLoading,
  onSelect,
  onApply
}) => {
  const getTemplateIcon = (template: CharacterTemplate | WorldBuildingTemplate) => {
    if (templateType === 'character') {
      return '👤';
    } else {
      const worldTemplate = template as WorldBuildingTemplate;
      switch (worldTemplate.element_type) {
        case 'location':
          return '🌍';
        case 'organization':
          return '👥';
        case 'culture':
          return '✨';
        case 'magic':
          return '✨';
        case 'technology':
          return '⚙️';
        case 'artifact':
          return '✨';
        default:
          return '🌍';
      }
    }
  };

  const getArchetypeOrType = (template: CharacterTemplate | WorldBuildingTemplate): string => {
    if (templateType === 'character') {
      return (template as CharacterTemplate).archetype;
    } else {
      return (template as WorldBuildingTemplate).element_type;
    }
  };

  const getDefaultItems = (template: CharacterTemplate | WorldBuildingTemplate): string[] => {
    if (templateType === 'character') {
      const charTemplate = template as CharacterTemplate;
      return charTemplate.default_traits.slice(0, 3).map(trait => trait.trait_name);
    } else {
      const worldTemplate = template as WorldBuildingTemplate;
      return worldTemplate.default_properties.slice(0, 3).map(prop => prop.property_name);
    }
  };

  const getTotalItemsCount = (template: CharacterTemplate | WorldBuildingTemplate): number => {
    if (templateType === 'character') {
      return (template as CharacterTemplate).default_traits.length;
    } else {
      return (template as WorldBuildingTemplate).default_properties.length;
    }
  };

  const handleApplyClick = (e: React.MouseEvent, template: CharacterTemplate | WorldBuildingTemplate) => {
    e.stopPropagation();
    onApply(template.id, template);
  };

  return (
    <div className="template-selector w-full">
      {isLoading ? (
        <div className="flex items-center justify-center p-8">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          <span className="ml-2 text-gray-600 dark:text-gray-400">Loading templates...</span>
        </div>
      ) : templates.length === 0 ? (
        <div className="text-center p-8 text-gray-600 dark:text-gray-400">
          <div className="text-4xl mb-4 opacity-50">✨</div>
          <p>No templates available</p>
        </div>
      ) : (
        <ScrollArea className="h-96">
          <div className="space-y-3 p-1">
            {templates.map((template) => (
              <Card 
                key={template.id}
                className={`transition-all hover:shadow-md ${selectedTemplateId === template.id ? 'ring-2 ring-blue-500' : ''}`}
                onClick={() => onSelect(template.id, template)}
              >
                <CardHeader className="pb-3">
                  <div className="flex items-start justify-between">
                    <div className="flex items-center space-x-2">
                      <span className="text-lg">{getTemplateIcon(template)}</span>
                      <div>
                        <CardTitle className="text-base">{template.name}</CardTitle>
                        <div className="flex items-center space-x-2 mt-1">
                          <Badge variant="secondary" className="text-xs">
                            {getArchetypeOrType(template)}
                          </Badge>
                          {template.is_system && (
                            <Badge variant="outline" className="text-xs">
                              System
                            </Badge>
                          )}
                        </div>
                      </div>
                    </div>
                    <Button
                      size="sm"
                      variant="ghost"
                      className="h-8 w-8 p-0"
                      onClick={(e) => handleApplyClick(e, template)}
                    >
                      <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5l7 7-7 7" />
                      </svg>
                    </Button>
                  </div>
                </CardHeader>
                <CardContent className="pt-0">
                  <CardDescription className="text-sm mb-3">
                    {template.description}
                  </CardDescription>
                  
                  <div className="space-y-2">
                    <div className="text-xs font-medium text-gray-600 dark:text-gray-400">
                      {templateType === 'character' ? 'Default Traits:' : 'Default Properties:'}
                    </div>
                    <div className="flex flex-wrap gap-1">
                      {getDefaultItems(template).map((item, index) => (
                        <Badge key={index} variant="outline" className="text-xs">
                          {item}
                        </Badge>
                      ))}
                      {getTotalItemsCount(template) > 3 && (
                        <Badge variant="outline" className="text-xs">
                          +{getTotalItemsCount(template) - 3} more
                        </Badge>
                      )}
                    </div>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </ScrollArea>
      )}
    </div>
  );
};

export default TemplateSelector;
