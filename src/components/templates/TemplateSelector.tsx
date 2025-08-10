import React, { useState } from 'react';
import { Button } from '../ui/Button';
import { Select } from '../ui/select';

interface Template {
  id: string;
  name: string;
  description: string;
  category: string;
  content: string;
}

interface TemplateSelectorProps {
  templates: Template[];
  onSelect: (template: Template) => void;
  category?: string;
  placeholder?: string;
  className?: string;
}

const TemplateSelector: React.FC<TemplateSelectorProps> = ({
  templates,
  onSelect,
  category,
  placeholder = 'Select a template...',
  className = ''
}) => {
  const [selectedTemplateId, setSelectedTemplateId] = useState<string>('');

  const filteredTemplates = category 
    ? templates.filter(t => t.category === category)
    : templates;

  const handleSelect = () => {
    const template = templates.find(t => t.id === selectedTemplateId);
    if (template) {
      onSelect(template);
      setSelectedTemplateId('');
    }
  };

  return (
    <div className={`space-y-3 ${className}`}>
      <div className="flex gap-2">
        <Select
          value={selectedTemplateId}
          onValueChange={setSelectedTemplateId}
        >
          <option value="">{placeholder}</option>
          {filteredTemplates.map((template) => (
            <option key={template.id} value={template.id}>
              {template.name}
            </option>
          ))}
        </Select>
        <Button
          onClick={handleSelect}
          disabled={!selectedTemplateId}
          variant="primary"
          size="sm"
        >
          Select
        </Button>
      </div>
      
      {selectedTemplateId && (
        <div className="p-3 bg-gray-50 dark:bg-gray-800 rounded-md">
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {templates.find(t => t.id === selectedTemplateId)?.description}
          </p>
        </div>
      )}
    </div>
  );
};

export default TemplateSelector;
