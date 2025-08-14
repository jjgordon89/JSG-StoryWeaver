import React, { useState, useEffect, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Canvas as CanvasModel, CanvasElement, CanvasElementType, OutlineTemplate } from '../../types/canvas';
import { CanvasElement as CanvasElementComponent } from './CanvasElement';
import { CanvasToolbar } from './CanvasToolbar';
import { CanvasElementCreator } from './CanvasElementCreator';
import { OutlineTemplateSelector } from './OutlineTemplateSelector';
import { CanvasCollaboration } from './CanvasCollaboration';
import { CanvasExportDialog } from './CanvasExportDialog';
import LoadingSpinner from '../ui/LoadingSpinner';
import ErrorMessage from '../ui/ErrorMessage';
import './Canvas.css';

interface CanvasProps {
  projectId: string;
  canvasId?: number;
  onCanvasChange?: (canvas: CanvasModel) => void;
}

export const Canvas: React.FC<CanvasProps> = ({ projectId, canvasId, onCanvasChange }) => {
  const [canvas, setCanvas] = useState<CanvasModel | null>(null);
  const [elements, setElements] = useState<CanvasElement[]>([]);
  const [selectedElement, setSelectedElement] = useState<CanvasElement | null>(null);
  const [isCreatingElement, setIsCreatingElement] = useState(false);
  const [elementTypeToCreate, setElementTypeToCreate] = useState<CanvasElementType>('text_box');
  const [showTemplateSelector, setShowTemplateSelector] = useState(false);
  const [showCollaboration, setShowCollaboration] = useState(false);
  const [showExport, setShowExport] = useState(false);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const [zoom, setZoom] = useState(1);
  const [viewport, setViewport] = useState({ x: 0, y: 0 });

  const canvasRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Load canvas and elements
  useEffect(() => {
    if (canvasId) {
      loadCanvas();
    } else {
      setLoading(false);
    }
  }, [canvasId]);

  const loadCanvas = async () => {
    try {
      setLoading(true);
      setError(null);

      if (canvasId) {
        const canvasData = await invoke<CanvasModel>('get_canvas', { canvasId });
        if (canvasData) {
          setCanvas(canvasData);
          setZoom(canvasData.zoom_level);
          setViewport({ x: canvasData.viewport_x, y: canvasData.viewport_y });
          
          const elementsData = await invoke<CanvasElement[]>('get_canvas_elements', { canvasId });
          setElements(elementsData);
          
          onCanvasChange?.(canvasData);
        }
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load canvas');
    } finally {
      setLoading(false);
    }
  };

  const createCanvas = async (name: string, description?: string) => {
    try {
      setLoading(true);
      setError(null);

      const newCanvas = await invoke<CanvasModel>('create_canvas', {
        projectId,
        name,
        description
      });

      setCanvas(newCanvas);
      setElements([]);
      onCanvasChange?.(newCanvas);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create canvas');
    } finally {
      setLoading(false);
    }
  };

  const createElement = async (x: number, y: number) => {
    if (!canvas) return;

    try {
      const newElement = await invoke<CanvasElement>('create_canvas_element', {
        canvasId: canvas.id,
        elementType: elementTypeToCreate,
        title: `New ${elementTypeToCreate.replace('_', ' ')}`,
        content: '',
        x,
        y,
        width: 200,
        height: 100,
        color: '#ffffff',
        metadata: '{}',
        connections: '[]',
        orderIndex: elements.length
      });

      setElements(prev => [...prev, newElement]);
      setSelectedElement(newElement);
      setIsCreatingElement(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create element');
    }
  };

  const updateElement = async (elementId: number, updates: Partial<CanvasElement>) => {
    try {
      await invoke('update_canvas_element', {
        elementId,
        x: updates.position_x,
        y: updates.position_y,
        width: updates.width,
        height: updates.height,
        content: updates.content,
        color: updates.color,
        title: updates.title,
        orderIndex: updates.order_index
      });

      setElements(prev => prev.map(el => 
        el.id === elementId ? { ...el, ...updates } : el
      ));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update element');
    }
  };

  const deleteElement = async (elementId: number) => {
    try {
      await invoke('delete_canvas_element', { elementId });
      setElements(prev => prev.filter(el => el.id !== elementId));
      if (selectedElement?.id === elementId) {
        setSelectedElement(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete element');
    }
  };

  const handleCanvasClick = (e: React.MouseEvent) => {
    if (!isCreatingElement) {
      setSelectedElement(null);
      return;
    }

    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = (e.clientX - rect.left - viewport.x) / zoom;
    const y = (e.clientY - rect.top - viewport.y) / zoom;

    createElement(x, y);
  };

  const handleElementSelect = (element: CanvasElement) => {
    setSelectedElement(element);
  };

  const handleElementUpdate = (elementId: number, updates: Partial<CanvasElement>) => {
    updateElement(elementId, updates);
  };

  const handleElementDelete = (elementId: number) => {
    deleteElement(elementId);
  };

  const handleZoom = (delta: number) => {
    const newZoom = Math.max(0.1, Math.min(3, zoom + delta));
    setZoom(newZoom);
    
    if (canvas) {
      // Update canvas zoom in database
      invoke('update_canvas', {
        canvasId: canvas.id,
        name: canvas.name,
        description: canvas.description
      });
    }
  };

  const handlePan = (deltaX: number, deltaY: number) => {
    setViewport(prev => ({
      x: prev.x + deltaX,
      y: prev.y + deltaY
    }));
  };

  // Global keyboard shortcuts:
  // - Escape: cancel create mode, close template selector/collaboration, or deselect element
  // - Delete / Backspace: delete currently selected element (if any)
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (isCreatingElement) {
          setIsCreatingElement(false);
          return;
        }
        if (showTemplateSelector) {
          setShowTemplateSelector(false);
          return;
        }
        if (showCollaboration) {
          setShowCollaboration(false);
          return;
        }
        if (showExport) {
          setShowExport(false);
          return;
        }
        setSelectedElement(null);
      } else if ((e.key === 'Delete' || e.key === 'Backspace') && selectedElement) {
        // fire and forget deletion of selected element
        deleteElement(selectedElement.id);
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isCreatingElement, showTemplateSelector, showCollaboration, selectedElement]);

  const applyTemplate = async (template: OutlineTemplate) => {
    if (!canvas) return;

    try {
      // Parse template structure and create elements
      const structure = JSON.parse(template.structure_data);
      
      // Clear existing elements
      for (const element of elements) {
        await invoke('delete_canvas_element', { elementId: element.id });
      }

      // Create new elements from template
      const newElements: CanvasElement[] = [];
      for (let i = 0; i < structure.elements.length; i++) {
        const templateElement = structure.elements[i];
        const element = await invoke<CanvasElement>('create_canvas_element', {
          canvasId: canvas.id,
          elementType: templateElement.type,
          title: templateElement.title,
          content: templateElement.content || '',
          x: templateElement.x,
          y: templateElement.y,
          width: templateElement.width || 200,
          height: templateElement.height || 100,
          color: templateElement.color || '#ffffff',
          metadata: JSON.stringify(templateElement.metadata || {}),
          connections: JSON.stringify(templateElement.connections || []),
          orderIndex: i
        });
        newElements.push(element);
      }

      setElements(newElements);
      setShowTemplateSelector(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to apply template');
    }
  };

  if (loading) {
    return <LoadingSpinner />;
  }

  if (!canvas && !canvasId) {
    return (
      <div className="canvas-empty">
        <h3>Create a New Canvas</h3>
        <p>Start planning your story visually with a new canvas.</p>
        <button 
          onClick={() => createCanvas('New Canvas')}
          className="btn btn-primary"
        >
          Create Canvas
        </button>
      </div>
    );
  }

  return (
    <div className="canvas-container" ref={containerRef}>
      {error && <ErrorMessage message={error} onDismiss={() => setError(null)} />}
      
      <CanvasToolbar
        canvas={canvas}
        zoom={zoom}
        onZoomIn={() => handleZoom(0.1)}
        onZoomOut={() => handleZoom(-0.1)}
        onResetZoom={() => setZoom(1)}
        onCreateElement={(type) => {
          setElementTypeToCreate(type);
          setIsCreatingElement(true);
        }}
        onShowTemplates={() => setShowTemplateSelector(true)}
        onShowCollaboration={() => setShowCollaboration(true)}
        onShowExport={() => setShowExport(true)}
        isCreatingElement={isCreatingElement}
        onCancelCreate={() => setIsCreatingElement(false)}
      />

      <div 
        className={`canvas-workspace ${isCreatingElement ? 'creating' : ''}`}
        ref={canvasRef}
        onClick={handleCanvasClick}
        style={{
          transform: `scale(${zoom}) translate(${viewport.x}px, ${viewport.y}px)`,
          transformOrigin: '0 0'
        }}
      >
        {elements.map(element => (
          <CanvasElementComponent
            key={element.id}
            element={element}
            isSelected={selectedElement?.id === element.id}
            onSelect={() => handleElementSelect(element)}
            onUpdate={(updates) => handleElementUpdate(element.id, updates)}
            onDelete={() => handleElementDelete(element.id)}
            zoom={zoom}
          />
        ))}
      </div>

      {selectedElement && (
        <CanvasElementCreator
          element={selectedElement}
          onUpdate={(updates) => handleElementUpdate(selectedElement.id, updates)}
          onClose={() => setSelectedElement(null)}
        />
      )}

      {showTemplateSelector && (
        <OutlineTemplateSelector
          onSelect={applyTemplate}
          onClose={() => setShowTemplateSelector(false)}
        />
      )}

      {showCollaboration && canvas && (
        <CanvasCollaboration
          canvas={canvas}
          onClose={() => setShowCollaboration(false)}
        />
      )}

      {showExport && canvas && (
        <CanvasExportDialog
          canvas={canvas}
          onClose={() => setShowExport(false)}
        />
      )}
    </div>
  );
};
