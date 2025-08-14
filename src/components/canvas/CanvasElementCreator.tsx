import React, { useState } from 'react';
import { CanvasElement, CanvasElementType } from '../../types/canvas';
import './CanvasElementCreator.css';

interface CanvasElementCreatorProps {
  element: CanvasElement;
  onUpdate: (updates: Partial<CanvasElement>) => void;
  onClose: () => void;
}

export const CanvasElementCreator: React.FC<CanvasElementCreatorProps> = ({
  element,
  onUpdate,
  onClose
}) => {
  const [title, setTitle] = useState(element.title);
  const [content, setContent] = useState(element.content);
  const [color, setColor] = useState(element.color);
  const [elementType, setElementType] = useState(element.element_type);

  const elementTypes: { value: CanvasElementType; label: string }[] = [
    { value: 'text_box', label: 'Text Box' },
    { value: 'sticky_note', label: 'Sticky Note' },
    { value: 'plot_point', label: 'Plot Point' },
    { value: 'character_arc', label: 'Character Arc' },
    { value: 'scene', label: 'Scene' },
    { value: 'chapter', label: 'Chapter' },
    { value: 'act', label: 'Act' },
    { value: 'note', label: 'Note' },
    { value: 'timeline_event', label: 'Timeline Event' },
    { value: 'theme', label: 'Theme' },
    { value: 'conflict', label: 'Conflict' }
  ];

  const colorPresets = [
    '#ffffff', '#f8f9fa', '#e9ecef', '#dee2e6',
    '#ffeaa7', '#fdcb6e', '#e17055', '#d63031',
    '#74b9ff', '#0984e3', '#00b894', '#00cec9',
    '#a29bfe', '#6c5ce7', '#fd79a8', '#e84393',
    '#fdcb6e', '#e17055', '#00b894', '#74b9ff'
  ];

  const handleSave = () => {
    onUpdate({
      title,
      content,
      color,
      element_type: elementType
    });
    onClose();
  };

  const handleCancel = () => {
    onClose();
  };

  return (
    <div className="canvas-element-creator">
      <div className="creator-overlay" onClick={onClose}></div>
      <div className="creator-panel">
        <div className="creator-header">
          <h3>Edit Element</h3>
          <button className="close-btn" onClick={onClose}>×</button>
        </div>

        <div className="creator-content">
          <div className="form-group">
            <label htmlFor="element-type">Element Type:</label>
            <select
              id="element-type"
              value={elementType}
              onChange={(e) => setElementType(e.target.value as CanvasElementType)}
              className="form-control"
            >
              {elementTypes.map(({ value, label }) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </div>

          <div className="form-group">
            <label htmlFor="element-title">Title:</label>
            <input
              id="element-title"
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="form-control"
              placeholder="Enter element title..."
            />
          </div>

          <div className="form-group">
            <label htmlFor="element-content">Content:</label>
            <textarea
              id="element-content"
              value={content}
              onChange={(e) => setContent(e.target.value)}
              className="form-control content-textarea"
              placeholder="Enter element content..."
              rows={6}
            />
          </div>

          <div className="form-group">
            <label>Background Color:</label>
            <div className="color-picker">
              <input
                type="color"
                value={color}
                onChange={(e) => setColor(e.target.value)}
                className="color-input"
              />
              <div className="color-presets">
                {colorPresets.map((preset) => (
                  <button
                    key={preset}
                    className={`color-preset ${color === preset ? 'selected' : ''}`}
                    style={{ backgroundColor: preset }}
                    onClick={() => setColor(preset)}
                    title={preset}
                  />
                ))}
              </div>
            </div>
          </div>

          <div className="form-group">
            <label>Position & Size:</label>
            <div className="position-size-grid">
              <div className="input-group">
                <label>X:</label>
                <input
                  type="number"
                  value={Math.round(element.position_x)}
                  onChange={(e) => onUpdate({ position_x: parseFloat(e.target.value) || 0 })}
                  className="form-control small"
                />
              </div>
              <div className="input-group">
                <label>Y:</label>
                <input
                  type="number"
                  value={Math.round(element.position_y)}
                  onChange={(e) => onUpdate({ position_y: parseFloat(e.target.value) || 0 })}
                  className="form-control small"
                />
              </div>
              <div className="input-group">
                <label>Width:</label>
                <input
                  type="number"
                  value={Math.round(element.width)}
                  onChange={(e) => onUpdate({ width: parseFloat(e.target.value) || 50 })}
                  className="form-control small"
                  min="50"
                />
              </div>
              <div className="input-group">
                <label>Height:</label>
                <input
                  type="number"
                  value={Math.round(element.height)}
                  onChange={(e) => onUpdate({ height: parseFloat(e.target.value) || 30 })}
                  className="form-control small"
                  min="30"
                />
              </div>
            </div>
          </div>
        </div>

        <div className="creator-actions">
          <button className="btn btn-primary" onClick={handleSave}>
            Save Changes
          </button>
          <button className="btn btn-secondary" onClick={handleCancel}>
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
};
