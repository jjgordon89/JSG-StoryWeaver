import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { OutlineTemplate, OutlineTemplateType } from '../../types/canvas';
import LoadingSpinner from '../ui/LoadingSpinner';
import ErrorMessage from '../ui/ErrorMessage';
import './OutlineTemplateSelector.css';

interface OutlineTemplateSelectorProps {
  onSelect: (template: OutlineTemplate) => void;
  onClose: () => void;
}

export const OutlineTemplateSelector: React.FC<OutlineTemplateSelectorProps> = ({
  onSelect,
  onClose
}) => {
  const [templates, setTemplates] = useState<OutlineTemplate[]>([]);
  const [filteredTemplates, setFilteredTemplates] = useState<OutlineTemplate[]>([]);
  const [selectedType, setSelectedType] = useState<OutlineTemplateType | 'all'>('all');
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [previewTemplate, setPreviewTemplate] = useState<OutlineTemplate | null>(null);

  const templateTypes: { value: OutlineTemplateType | 'all'; label: string }[] = [
    { value: 'all', label: 'All Templates' },
    { value: 'heros_journey', label: "Hero's Journey" },
    { value: 'hollywood_beats', label: 'Hollywood Beats' },
    { value: 'story_circle', label: 'Story Circle' },
    { value: 'romance_outline', label: 'Romance Outline' },
    { value: 'three_act', label: 'Three Act Structure' },
    { value: 'save_the_cat', label: 'Save the Cat' },
    { value: 'snowflake', label: 'Snowflake Method' },
    { value: 'seven_point', label: 'Seven Point Story' },
    { value: 'custom', label: 'Custom Templates' }
  ];

  useEffect(() => {
    loadTemplates();
  }, []);

  useEffect(() => {
    if (selectedType === 'all') {
      setFilteredTemplates(templates);
    } else {
      setFilteredTemplates(templates.filter(t => t.template_type === selectedType));
    }
  }, [templates, selectedType]);

  const loadTemplates = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const templatesData = await invoke<OutlineTemplate[]>('get_outline_templates', {
        templateType: null
      });
      
      setTemplates(templatesData);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load templates');
    } finally {
      setLoading(false);
    }
  };

  const handleTemplateSelect = (template: OutlineTemplate) => {
    onSelect(template);
  };

  const handlePreview = (template: OutlineTemplate) => {
    setPreviewTemplate(template);
  };

  const getTemplateDescription = (template: OutlineTemplate) => {
    const descriptions: Record<OutlineTemplateType, string> = {
      heros_journey: "The classic monomyth structure following a hero's transformative adventure.",
      hollywood_beats: "Industry-standard screenplay structure with key plot points and timing.",
      story_circle: "Dan Harmon's simplified version of the Hero's Journey in 8 steps.",
      romance_outline: "Structure specifically designed for romance novels and stories.",
      three_act: "Traditional three-act structure with setup, confrontation, and resolution.",
      save_the_cat: "Blake Snyder's beat sheet for screenwriting with 15 key beats.",
      snowflake: "Randy Ingermanson's method for developing stories from simple to complex.",
      seven_point: "Dan Wells' structure focusing on character development and plot progression.",
      custom: "User-created templates for specific story needs."
    };
    
    return descriptions[template.template_type] || template.description;
  };

  if (loading) {
    return (
      <div className="template-selector">
        <div className="selector-overlay" onClick={onClose}></div>
        <div className="selector-panel">
          <LoadingSpinner />
        </div>
      </div>
    );
  }

  return (
    <div className="template-selector">
      <div className="selector-overlay" onClick={onClose}></div>
      <div className="selector-panel">
        <div className="selector-header">
          <h3>Choose Outline Template</h3>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        {error && <ErrorMessage message={error} onDismiss={() => setError(null)} />}

        <div className="selector-filters">
          <div className="filter-group">
            <label htmlFor="template-type">Filter by Type:</label>
            <select
              id="template-type"
              value={selectedType}
              onChange={(e) => setSelectedType(e.target.value as OutlineTemplateType | 'all')}
              className="form-control"
            >
              {templateTypes.map(({ value, label }) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div className="templates-grid">
          {filteredTemplates.map((template) => (
            <div key={template.id} className="template-card">
              <div className="template-header">
                <h4 className="template-name">{template.name}</h4>
                {template.is_builtin && (
                  <span className="builtin-badge">Built-in</span>
                )}
              </div>
              
              <div className="template-description">
                {getTemplateDescription(template)}
              </div>

              <div className="template-type">
                <span className="type-label">
                  {templateTypes.find(t => t.value === template.template_type)?.label || template.template_type}
                </span>
              </div>

              <div className="template-actions">
                <button
                  className="btn btn-secondary preview-btn"
                  onClick={() => handlePreview(template)}
                >
                  Preview
                </button>
                <button
                  className="btn btn-primary select-btn"
                  onClick={() => handleTemplateSelect(template)}
                >
                  Use Template
                </button>
              </div>
            </div>
          ))}
        </div>

        {filteredTemplates.length === 0 && !loading && (
          <div className="no-templates">
            <p>No templates found for the selected type.</p>
          </div>
        )}

        {previewTemplate && (
          <div className="template-preview">
            <div className="preview-overlay" onClick={() => setPreviewTemplate(null)}></div>
            <div className="preview-panel">
              <div className="preview-header">
                <h3>{previewTemplate.name}</h3>
                <button className="close-btn" onClick={() => setPreviewTemplate(null)}>×</button>
              </div>
              
              <div className="preview-content">
                <div className="preview-description">
                  <h4>Description:</h4>
                  <p>{getTemplateDescription(previewTemplate)}</p>
                </div>
                
                <div className="preview-structure">
                  <h4>Structure Preview:</h4>
                  <div className="structure-preview">
                    {(() => {
                      try {
                        const structure = JSON.parse(previewTemplate.structure_data);
                        return (
                          <div className="structure-elements">
                            {structure.elements?.map((element: any, index: number) => (
                              <div key={index} className="structure-element">
                                <span className="element-type">{element.type}</span>
                                <span className="element-title">{element.title}</span>
                              </div>
                            )) || <p>No structure preview available</p>}
                          </div>
                        );
                      } catch {
                        return <p>Structure data format not supported for preview</p>;
                      }
                    })()}
                  </div>
                </div>
              </div>
              
              <div className="preview-actions">
                <button
                  className="btn btn-primary"
                  onClick={() => {
                    handleTemplateSelect(previewTemplate);
                    setPreviewTemplate(null);
                  }}
                >
                  Use This Template
                </button>
                <button
                  className="btn btn-secondary"
                  onClick={() => setPreviewTemplate(null)}
                >
                  Close Preview
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
