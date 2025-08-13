import React, { useState, useEffect } from 'react';
import type { CharacterTemplate, WorldBuildingTemplate, ApplyCharacterTemplateRequest, ApplyWorldBuildingTemplateRequest } from '../../types/templates';

interface TemplateApplicationDialogProps {
  open: boolean;
  template: CharacterTemplate | WorldBuildingTemplate | null;
  templateType: 'character' | 'worldbuilding';
  projectId: string;
  isApplying: boolean;
  onApply: (request: ApplyCharacterTemplateRequest | ApplyWorldBuildingTemplateRequest) => void;
  onCancel: () => void;
}

// Simple UI Components
const Dialog: React.FC<{ open: boolean; onClose: () => void; children: React.ReactNode }> = ({ 
  open, 
  onClose, 
  children 
}) => {
  if (!open) return null;
  
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="fixed inset-0 bg-black/50" onClick={onClose} />
      <div className="relative bg-white dark:bg-gray-800 rounded-lg shadow-lg max-w-2xl w-full mx-4 max-h-[90vh] overflow-hidden flex flex-col">
        {children}
      </div>
    </div>
  );
};

const DialogHeader: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <div className="p-6 pb-4 border-b border-gray-200 dark:border-gray-700">
    {children}
  </div>
);

const DialogTitle: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <h2 className="text-lg font-semibold text-gray-900 dark:text-white">{children}</h2>
);

const DialogDescription: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">{children}</p>
);

const DialogContent: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <div className="flex-1 overflow-hidden p-6">
    {children}
  </div>
);

const DialogFooter: React.FC<{ children: React.ReactNode }> = ({ children }) => (
  <div className="p-6 pt-4 border-t border-gray-200 dark:border-gray-700 flex justify-end gap-2">
    {children}
  </div>
);

const Button: React.FC<{
  variant?: 'outline' | 'solid';
  onClick?: () => void;
  disabled?: boolean;
  children: React.ReactNode;
}> = ({ variant = 'solid', onClick, disabled, children }) => {
  const baseClasses = 'inline-flex items-center justify-center px-4 py-2 rounded-md font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2';
  const variantClasses = variant === 'outline' 
    ? 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700'
    : 'bg-blue-600 text-white hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600';
  const disabledClasses = disabled ? 'opacity-50 cursor-not-allowed' : '';
  
  return (
    <button
      className={`${baseClasses} ${variantClasses} ${disabledClasses}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};

const Input: React.FC<{
  id?: string;
  type?: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  required?: boolean;
}> = ({ id, type = 'text', value, onChange, placeholder, required }) => (
  <input
    id={id}
    type={type}
    value={value}
    onChange={(e) => onChange(e.target.value)}
    placeholder={placeholder}
    required={required}
    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
  />
);

const Textarea: React.FC<{
  id?: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  rows?: number;
}> = ({ id, value, onChange, placeholder, rows = 3 }) => (
  <textarea
    id={id}
    value={value}
    onChange={(e) => onChange(e.target.value)}
    placeholder={placeholder}
    rows={rows}
    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white resize-none"
  />
);

const Label: React.FC<{ htmlFor?: string; children: React.ReactNode }> = ({ htmlFor, children }) => (
  <label htmlFor={htmlFor} className="block text-sm font-medium text-gray-700 dark:text-gray-300">
    {children}
  </label>
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

const Card: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg ${className}`}>
    {children}
  </div>
);

const Separator: React.FC = () => (
  <hr className="border-gray-200 dark:border-gray-700" />
);

const ScrollArea: React.FC<{ className?: string; children: React.ReactNode }> = ({ className = '', children }) => (
  <div className={`overflow-y-auto ${className}`}>
    {children}
  </div>
);

export const TemplateApplicationDialog: React.FC<TemplateApplicationDialogProps> = ({
  open,
  template,
  templateType,
  projectId,
  isApplying,
  onApply,
  onCancel
}) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [overrides, setOverrides] = useState<Record<string, any>>({});
  const [selectedProperties, setSelectedProperties] = useState<Record<string, boolean>>({});

  useEffect(() => {
    if (template && open) {
      resetForm();
    }
  }, [template, open]);

  const resetForm = () => {
    setName('');
    setDescription('');
    setOverrides({});
    setSelectedProperties({});
    
    if (template) {
      if (templateType === 'character') {
        const charTemplate = template as CharacterTemplate;
        const newSelectedProperties: Record<string, boolean> = {};
        const newOverrides: Record<string, any> = {};
        
        charTemplate.default_traits.forEach(trait => {
          newSelectedProperties[trait.trait_name] = trait.is_required;
          if (trait.default_value) {
            newOverrides[trait.trait_name] = trait.default_value;
          }
        });
        
        setSelectedProperties(newSelectedProperties);
        setOverrides(newOverrides);
      } else {
        const worldTemplate = template as WorldBuildingTemplate;
        const newSelectedProperties: Record<string, boolean> = {};
        const newOverrides: Record<string, any> = {};
        
        worldTemplate.default_properties.forEach(prop => {
          newSelectedProperties[prop.property_name] = prop.is_required;
          if (prop.default_value) {
            newOverrides[prop.property_name] = prop.default_value;
          }
        });
        
        setSelectedProperties(newSelectedProperties);
        setOverrides(newOverrides);
      }
    }
  };

  const handleApply = () => {
    if (!template || !name.trim()) return;

    // Filter overrides to only include selected properties
    const filteredOverrides: Record<string, any> = {};
    Object.keys(selectedProperties).forEach(key => {
      if (selectedProperties[key] && overrides[key] !== undefined) {
        filteredOverrides[key] = overrides[key];
      }
    });

    if (templateType === 'character') {
      const request: ApplyCharacterTemplateRequest = {
        template_id: template.id,
        project_id: projectId,
        name: name.trim(),
        description: description.trim() || undefined,
        trait_overrides: Object.keys(filteredOverrides).length > 0 ? filteredOverrides : undefined,
      };
      onApply(request);
    } else {
      const request: ApplyWorldBuildingTemplateRequest = {
        template_id: template.id,
        project_id: projectId,
        name: name.trim(),
        description: description.trim() || undefined,
        property_overrides: Object.keys(filteredOverrides).length > 0 ? filteredOverrides : undefined,
      };
      onApply(request);
    }
  };

  const getTemplateIcon = () => {
    if (!template) return '✨';
    
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

  const getItems = () => {
    if (!template) return [];
    
    if (templateType === 'character') {
      return (template as CharacterTemplate).default_traits;
    } else {
      return (template as WorldBuildingTemplate).default_properties;
    }
  };

  const getItemName = (item: any): string => {
    return templateType === 'character' ? item.trait_name : item.property_name;
  };

  const getItemValue = (item: any): string => {
    return templateType === 'character' ? item.default_value : item.default_value;
  };

  const getItemType = (item: any): string => {
    return templateType === 'character' ? item.trait_type : item.property_type;
  };

  const isRequired = (item: any): boolean => {
    return item.is_required;
  };

  const handlePropertyToggle = (propertyName: string, checked: boolean) => {
    setSelectedProperties(prev => ({
      ...prev,
      [propertyName]: checked
    }));
  };

  const handleOverrideChange = (propertyName: string, value: any) => {
    setOverrides(prev => ({
      ...prev,
      [propertyName]: value
    }));
  };

  return (
    <Dialog open={open} onClose={onCancel}>
      <DialogHeader>
        <div className="flex items-center space-x-2">
          <span className="text-lg">{getTemplateIcon()}</span>
          <DialogTitle>Apply Template: {template?.name || ''}</DialogTitle>
        </div>
        <DialogDescription>
          {template?.description || ''}
        </DialogDescription>
      </DialogHeader>

      <DialogContent>
        <ScrollArea className="h-full pr-4">
          <div className="space-y-6">
            {/* Basic Information */}
            <div className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="name">Name *</Label>
                <Input
                  id="name"
                  value={name}
                  onChange={setName}
                  placeholder={`Enter ${templateType} name`}
                  required
                />
              </div>
              
              <div className="space-y-2">
                <Label htmlFor="description">Description</Label>
                <Textarea
                  id="description"
                  value={description}
                  onChange={setDescription}
                  placeholder="Optional description"
                  rows={2}
                />
              </div>
            </div>

            <Separator />

            {/* Template Properties/Traits */}
            <div className="space-y-4">
              <div className="flex items-center space-x-2">
                <h3 className="text-lg font-semibold">
                  {templateType === 'character' ? 'Character Traits' : 'Element Properties'}
                </h3>
                <Badge variant="secondary" className="text-xs">
                  {getItems().length} items
                </Badge>
              </div>
              
              <div className="space-y-3">
                {getItems().map((item, index) => (
                  <Card key={index} className="p-4">
                    <div className="space-y-3">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-2">
                          <input
                            type="checkbox"
                            checked={selectedProperties[getItemName(item)] || false}
                            onChange={(e) => handlePropertyToggle(getItemName(item), e.target.checked)}
                            disabled={isRequired(item)}
                            className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                          />
                          <Label className="font-medium">
                            {getItemName(item)}
                            {isRequired(item) && (
                              <span className="text-red-500 ml-1">*</span>
                            )}
                          </Label>
                          <Badge variant="outline" className="text-xs">
                            {getItemType(item)}
                          </Badge>
                        </div>
                      </div>
                      
                      <p className="text-sm text-gray-600 dark:text-gray-400">
                        {item.description}
                      </p>
                      
                      {selectedProperties[getItemName(item)] && (
                        <div className="space-y-2">
                          <Label htmlFor={getItemName(item)} className="text-sm">
                            Value
                          </Label>
                          {getItemType(item) === 'text' || getItemType(item) === 'list' ? (
                            <Textarea
                              id={getItemName(item)}
                              value={overrides[getItemName(item)] || ''}
                              onChange={(value) => handleOverrideChange(getItemName(item), value)}
                              placeholder={getItemValue(item) || `Enter ${getItemName(item)}`}
                              rows={2}
                            />
                          ) : getItemType(item) === 'number' ? (
                            <Input
                              id={getItemName(item)}
                              type="number"
                              value={overrides[getItemName(item)] || ''}
                              onChange={(value) => handleOverrideChange(getItemName(item), value)}
                              placeholder={getItemValue(item) || '0'}
                            />
                          ) : getItemType(item) === 'boolean' ? (
                            <input
                              type="checkbox"
                              checked={overrides[getItemName(item)] || false}
                              onChange={(e) => handleOverrideChange(getItemName(item), e.target.checked)}
                              className="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                            />
                          ) : (
                            <Input
                              id={getItemName(item)}
                              value={overrides[getItemName(item)] || ''}
                              onChange={(value) => handleOverrideChange(getItemName(item), value)}
                              placeholder={getItemValue(item) || `Enter ${getItemName(item)}`}
                            />
                          )}
                        </div>
                      )}
                    </div>
                  </Card>
                ))}
              </div>
            </div>
          </div>
        </ScrollArea>
      </DialogContent>

      <DialogFooter>
        <Button variant="outline" onClick={onCancel} disabled={isApplying}>
          Cancel
        </Button>
        <Button onClick={handleApply} disabled={!name.trim() || isApplying}>
          {isApplying && (
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
          )}
          Apply Template
        </Button>
      </DialogFooter>
    </Dialog>
  );
};

export default TemplateApplicationDialog;
