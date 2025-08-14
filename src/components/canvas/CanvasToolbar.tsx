import React, { useState } from 'react';
import { Canvas, CanvasElementType } from '../../types/canvas';
import './CanvasToolbar.css';

interface CanvasToolbarProps {
  canvas: Canvas | null;
  zoom: number;
  onZoomIn: () => void;
  onZoomOut: () => void;
  onResetZoom: () => void;
  onCreateElement: (type: CanvasElementType) => void;
  onShowTemplates: () => void;
  onShowCollaboration: () => void;
  onShowExport: () => void;
  isCreatingElement: boolean;
  onCancelCreate: () => void;
}

export const CanvasToolbar: React.FC<CanvasToolbarProps> = ({
  canvas,
  zoom,
  onZoomIn,
  onZoomOut,
  onResetZoom,
  onCreateElement,
  onShowTemplates,
  onShowCollaboration,
  onShowExport,
  isCreatingElement,
  onCancelCreate
}) => {
  const [showElementMenu, setShowElementMenu] = useState(false);

  const elementTypes: { type: CanvasElementType; label: string; icon: string }[] = [
    { type: 'text_box', label: 'Text Box', icon: '📄' },
    { type: 'sticky_note', label: 'Sticky Note', icon: '🗒️' },
    { type: 'plot_point', label: 'Plot Point', icon: '📍' },
    { type: 'character_arc', label: 'Character Arc', icon: '👤' },
    { type: 'scene', label: 'Scene', icon: '🎬' },
    { type: 'chapter', label: 'Chapter', icon: '📖' },
    { type: 'act', label: 'Act', icon: '🎭' },
    { type: 'note', label: 'Note', icon: '📝' },
    { type: 'timeline_event', label: 'Timeline Event', icon: '⏰' },
    { type: 'theme', label: 'Theme', icon: '💭' },
    { type: 'conflict', label: 'Conflict', icon: '⚡' }
  ];

  const handleElementTypeSelect = (type: CanvasElementType) => {
    onCreateElement(type);
    setShowElementMenu(false);
  };

  return (
    <div className="canvas-toolbar">
      <div className="toolbar-section">
        <h3 className="canvas-title">
          {canvas?.name || 'Canvas'}
        </h3>
      </div>

      <div className="toolbar-section">
        <div className="toolbar-group">
          <label className="toolbar-label">Zoom:</label>
          <button 
            className="toolbar-btn zoom-btn"
            onClick={onZoomOut}
            title="Zoom Out"
            disabled={zoom <= 0.1}
          >
            🔍-
          </button>
          <span className="zoom-level">{Math.round(zoom * 100)}%</span>
          <button 
            className="toolbar-btn zoom-btn"
            onClick={onZoomIn}
            title="Zoom In"
            disabled={zoom >= 3}
          >
            🔍+
          </button>
          <button 
            className="toolbar-btn reset-zoom-btn"
            onClick={onResetZoom}
            title="Reset Zoom"
          >
            🎯
          </button>
        </div>
      </div>

      <div className="toolbar-section">
        <div className="toolbar-group">
          {isCreatingElement ? (
            <div className="creating-element-controls">
              <span className="creating-text">Click on canvas to place element</span>
              <button 
                className="toolbar-btn cancel-btn"
                onClick={onCancelCreate}
              >
                Cancel
              </button>
            </div>
          ) : (
            <div className="element-controls">
              <div className="dropdown-container">
                <button 
                  className="toolbar-btn add-element-btn"
                  onClick={() => setShowElementMenu(!showElementMenu)}
                >
                  ➕ Add Element
                </button>
                {showElementMenu && (
                  <div className="element-menu">
                    {elementTypes.map(({ type, label, icon }) => (
                      <button
                        key={type}
                        className="element-menu-item"
                        onClick={() => handleElementTypeSelect(type)}
                      >
                        <span className="element-icon">{icon}</span>
                        <span className="element-label">{label}</span>
                      </button>
                    ))}
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>

      <div className="toolbar-section">
        <div className="toolbar-group">
          <button 
            className="toolbar-btn template-btn"
            onClick={onShowTemplates}
            title="Apply Template"
          >
            📋 Templates
          </button>
          <button 
            className="toolbar-btn collaboration-btn"
            onClick={onShowCollaboration}
            title="Collaboration"
          >
            👥 Collaborate
          </button>
          <button 
            className="toolbar-btn export-btn"
            onClick={onShowExport}
            title="Export Canvas"
            disabled={!canvas}
          >
            📤 Export
          </button>
        </div>
      </div>

      <div className="toolbar-section">
        <div className="toolbar-group">
          <div className="canvas-info">
            <span className="info-item">
              Size: {canvas?.width || 1920} × {canvas?.height || 1080}
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};
