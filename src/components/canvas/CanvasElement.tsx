import React, { useState, useRef, useEffect } from 'react';
import { CanvasElement as CanvasElementType, CanvasElementType as ElementType } from '../../types/canvas';
import './CanvasElement.css';

interface CanvasElementProps {
  element: CanvasElementType;
  isSelected: boolean;
  onSelect: () => void;
  onUpdate: (updates: Partial<CanvasElementType>) => void;
  onDelete: () => void;
  zoom: number;
}

export const CanvasElement: React.FC<CanvasElementProps> = ({
  element,
  isSelected,
  onSelect,
  onUpdate,
  onDelete,
  zoom
}) => {
  const [isDragging, setIsDragging] = useState(false);
  const [isResizing, setIsResizing] = useState(false);
  const [resizeStart, setResizeStart] = useState({ x: 0, y: 0, width: 0, height: 0 });
  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState(element.content);
  const [editTitle, setEditTitle] = useState(element.title);

  // Use refs for drag metadata to avoid re-renders while dragging
  const dragRef = useRef({ clientX: 0, clientY: 0, initialX: 0, initialY: 0 });

  const elementRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLTextAreaElement>(null);
  const titleRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    setEditContent(element.content);
    setEditTitle(element.title);
  }, [element.content, element.title]);

  const handleMouseDown = (e: React.MouseEvent) => {
    e.stopPropagation();
    onSelect();

    if (e.detail === 2) {
      // Double click to edit
      setIsEditing(true);
      return;
    }

    const rect = elementRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    // Check if clicking on resize handle (bottom-right corner)
    const isResizeHandle = x > rect.width - 20 && y > rect.height - 20;

    if (isResizeHandle) {
      setIsResizing(true);
      setResizeStart({
        x: e.clientX,
        y: e.clientY,
        width: element.width,
        height: element.height
      });
    } else {
      // Initialize drag metadata: store the initial client coords and element logical position
      dragRef.current = {
        clientX: e.clientX,
        clientY: e.clientY,
        initialX: element.position_x,
        initialY: element.position_y
      };
      setIsDragging(true);
    }
  };

  const handleMouseMove = (e: MouseEvent) => {
    if (isDragging) {
      // Calculate delta in client pixels and convert to canvas units using zoom
      const deltaX = (e.clientX - dragRef.current.clientX) / zoom;
      const deltaY = (e.clientY - dragRef.current.clientY) / zoom;

      const newX = Math.max(0, dragRef.current.initialX + deltaX);
      const newY = Math.max(0, dragRef.current.initialY + deltaY);

      onUpdate({
        position_x: newX,
        position_y: newY
      });
    } else if (isResizing) {
      const deltaX = e.clientX - resizeStart.x;
      const deltaY = e.clientY - resizeStart.y;

      const newWidth = Math.max(50, resizeStart.width + deltaX / zoom);
      const newHeight = Math.max(30, resizeStart.height + deltaY / zoom);

      onUpdate({
        width: newWidth,
        height: newHeight
      });
    }
  };

  const handleMouseUp = () => {
    setIsDragging(false);
    setIsResizing(false);
  };

  useEffect(() => {
    if (isDragging || isResizing) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);

      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
  }, [isDragging, isResizing, resizeStart, zoom]);

  const handleSaveEdit = () => {
    onUpdate({
      title: editTitle,
      content: editContent
    });
    setIsEditing(false);
  };

  const handleCancelEdit = () => {
    setEditTitle(element.title);
    setEditContent(element.content);
    setIsEditing(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && e.ctrlKey) {
      handleSaveEdit();
    } else if (e.key === 'Escape') {
      handleCancelEdit();
    } else if (e.key === 'Delete' && isSelected && !isEditing) {
      onDelete();
    }
  };

  const getElementIcon = (type: ElementType) => {
    switch (type) {
      case 'plot_point': return '📍';
      case 'character_arc': return '👤';
      case 'scene': return '🎬';
      case 'chapter': return '📖';
      case 'act': return '🎭';
      case 'note': return '📝';
      case 'timeline_event': return '⏰';
      case 'theme': return '💭';
      case 'conflict': return '⚡';
      case 'sticky_note': return '🗒️';
      default: return '📄';
    }
  };

  const getElementTypeLabel = (type: ElementType) => {
    return type.replace('_', ' ').replace(/\b\w/g, l => l.toUpperCase());
  };

  return (
    <div
      ref={elementRef}
      className={`canvas-element ${element.element_type} ${isSelected ? 'selected' : ''} ${isDragging ? 'dragging' : ''} ${isResizing ? 'resizing' : ''}`}
      style={{
        left: element.position_x,
        top: element.position_y,
        width: element.width,
        height: element.height,
        backgroundColor: element.color,
        zIndex: element.order_index
      }}
      onMouseDown={handleMouseDown}
      onKeyDown={handleKeyDown}
      tabIndex={0}
    >
      <div className="element-header">
        <span className="element-icon">{getElementIcon(element.element_type)}</span>
        {isEditing ? (
          <input
            ref={titleRef}
            type="text"
            value={editTitle}
            onChange={(e) => setEditTitle(e.target.value)}
            className="element-title-input"
            autoFocus
            onBlur={handleSaveEdit}
          />
        ) : (
          <span className="element-title">{element.title}</span>
        )}
        <div className="element-actions">
          <button
            className="element-action-btn edit-btn"
            onClick={(e) => {
              e.stopPropagation();
              setIsEditing(true);
            }}
            title="Edit"
          >
            ✏️
          </button>
          <button
            className="element-action-btn delete-btn"
            onClick={(e) => {
              e.stopPropagation();
              onDelete();
            }}
            title="Delete"
          >
            🗑️
          </button>
        </div>
      </div>

      <div className="element-content">
        {isEditing ? (
          <textarea
            ref={contentRef}
            value={editContent}
            onChange={(e) => setEditContent(e.target.value)}
            className="element-content-input"
            placeholder={`Enter ${getElementTypeLabel(element.element_type)} content...`}
            onBlur={handleSaveEdit}
          />
        ) : (
          <div className="element-content-display">
            {element.content || `Click to add ${getElementTypeLabel(element.element_type)} content...`}
          </div>
        )}
      </div>

      {isSelected && (
        <>
          <div className="selection-handles">
            <div className="handle top-left"></div>
            <div className="handle top-right"></div>
            <div className="handle bottom-left"></div>
            <div className="handle bottom-right resize-handle"></div>
          </div>
          <div className="element-info">
            <span className="element-type-label">{getElementTypeLabel(element.element_type)}</span>
          </div>
        </>
      )}

      {isEditing && (
        <div className="edit-controls">
          <button onClick={handleSaveEdit} className="save-btn">Save</button>
          <button onClick={handleCancelEdit} className="cancel-btn">Cancel</button>
        </div>
      )}
    </div>
  );
};
