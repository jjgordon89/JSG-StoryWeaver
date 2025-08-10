import React, { useState } from 'react';
import { Modal } from '../ui/Modal';
import { Button } from '../ui/Button';
import { TextArea } from '../ui/TextArea';
import { Input } from '../ui/input';

interface Template {
  id: string;
  name: string;
  description: string;
  category: string;
  content: string;
}

interface TemplateApplicationDialogProps {
  isOpen: boolean;
  onClose: () => void;
  template: Template | null;
  onApply: (customizedContent: string) => void;
}

const TemplateApplicationDialog: React.FC<TemplateApplicationDialogProps> = ({
  isOpen,
  onClose,
  template,
  onApply
}) => {
  const [customizedContent, setCustomizedContent] = useState('');

  React.useEffect(() => {
    if (template) {
      setCustomizedContent(template.content);
    }
  }, [template]);

  const handleApply = () => {
    if (customizedContent.trim()) {
      onApply(customizedContent);
      onClose();
    }
  };

  const handleClose = () => {
    setCustomizedContent('');
    onClose();
  };

  if (!template) return null;

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      title={`Apply Template: ${template.name}`}
      size="large"
    >
      <div className="space-y-4">
        <div>
          <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Template Description
          </h4>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            {template.description}
          </p>
        </div>

        <div>
          <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Original Content
          </h4>
          <div className="p-3 bg-gray-50 dark:bg-gray-800 rounded-md">
            <p className="text-sm text-gray-600 dark:text-gray-400 whitespace-pre-wrap">
              {template.content}
            </p>
          </div>
        </div>

        <div>
          <h4 className="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Customize Content
          </h4>
          <TextArea
            value={customizedContent}
            onChange={setCustomizedContent}
            placeholder="Customize the template content..."
            rows={8}
          />
        </div>

        <div className="flex justify-end gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
          <Button variant="secondary" onClick={handleClose}>
            Cancel
          </Button>
          <Button 
            variant="primary" 
            onClick={handleApply}
            disabled={!customizedContent.trim()}
          >
            Apply Template
          </Button>
        </div>
      </div>
    </Modal>
  );
};

export default TemplateApplicationDialog;
