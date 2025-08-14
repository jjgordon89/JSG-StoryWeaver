import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Canvas, CanvasElement, OutlineTemplate, CanvasSnapshot } from '../types/canvas';

export const useCanvas = (projectId: string) => {
  const [canvases, setCanvases] = useState<Canvas[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadCanvases = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      
      const data = await invoke<Canvas[]>('get_project_canvases', { projectId });
      setCanvases(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load canvases');
    } finally {
      setLoading(false);
    }
  }, [projectId]);

  const createCanvas = useCallback(async (name: string, description?: string) => {
    try {
      const newCanvas = await invoke<Canvas>('create_canvas', {
        projectId,
        name,
        description
      });
      
      setCanvases(prev => [newCanvas, ...prev]);
      return newCanvas;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create canvas';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, [projectId]);

  const updateCanvas = useCallback(async (canvasId: number, name?: string, description?: string) => {
    try {
      await invoke('update_canvas', {
        canvasId,
        name,
        description
      });
      
      setCanvases(prev => prev.map(canvas => 
        canvas.id === canvasId 
          ? { ...canvas, name: name || canvas.name, description: description || canvas.description }
          : canvas
      ));
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to update canvas';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  const deleteCanvas = useCallback(async (canvasId: number) => {
    try {
      await invoke('delete_canvas', { canvasId });
      setCanvases(prev => prev.filter(canvas => canvas.id !== canvasId));
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to delete canvas';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  useEffect(() => {
    if (projectId) {
      loadCanvases();
    }
  }, [projectId, loadCanvases]);

  return {
    canvases,
    loading,
    error,
    loadCanvases,
    createCanvas,
    updateCanvas,
    deleteCanvas,
    clearError: () => setError(null)
  };
};

export const useCanvasElements = (canvasId: number | null) => {
  const [elements, setElements] = useState<CanvasElement[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadElements = useCallback(async () => {
    if (!canvasId) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const data = await invoke<CanvasElement[]>('get_canvas_elements', { canvasId });
      setElements(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load canvas elements');
    } finally {
      setLoading(false);
    }
  }, [canvasId]);

  const createElement = useCallback(async (elementData: {
    canvasId: number;
    elementType: string;
    title: string;
    content: string;
    x: number;
    y: number;
    width: number;
    height: number;
    color: string;
    metadata: string;
    connections: string;
    orderIndex: number;
  }) => {
    try {
      const newElement = await invoke<CanvasElement>('create_canvas_element', elementData);
      setElements(prev => [...prev, newElement]);
      return newElement;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create element';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  const updateElement = useCallback(async (elementId: number, updates: {
    x?: number;
    y?: number;
    width?: number;
    height?: number;
    content?: string;
    color?: string;
    title?: string;
    orderIndex?: number;
  }) => {
    try {
      await invoke('update_canvas_element', {
        elementId,
        ...updates
      });
      
      setElements(prev => prev.map(element => 
        element.id === elementId 
          ? { ...element, ...updates }
          : element
      ));
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to update element';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  const deleteElement = useCallback(async (elementId: number) => {
    try {
      await invoke('delete_canvas_element', { elementId });
      setElements(prev => prev.filter(element => element.id !== elementId));
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to delete element';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  useEffect(() => {
    if (canvasId) {
      loadElements();
    } else {
      setElements([]);
    }
  }, [canvasId, loadElements]);

  return {
    elements,
    loading,
    error,
    loadElements,
    createElement,
    updateElement,
    deleteElement,
    clearError: () => setError(null)
  };
};

export const useOutlineTemplates = () => {
  const [templates, setTemplates] = useState<OutlineTemplate[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTemplates = useCallback(async (templateType?: string) => {
    try {
      setLoading(true);
      setError(null);
      
      const data = await invoke<OutlineTemplate[]>('get_outline_templates', {
        templateType: templateType || null
      });
      setTemplates(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load templates');
    } finally {
      setLoading(false);
    }
  }, []);

  const createTemplate = useCallback(async (templateData: {
    name: string;
    description: string;
    templateType: string;
    structure: string;
    isOfficial: boolean;
  }) => {
    try {
      const newTemplate = await invoke<OutlineTemplate>('create_outline_template', templateData);
      setTemplates(prev => [newTemplate, ...prev]);
      return newTemplate;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create template';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, []);

  useEffect(() => {
    loadTemplates();
  }, [loadTemplates]);

  return {
    templates,
    loading,
    error,
    loadTemplates,
    createTemplate,
    clearError: () => setError(null)
  };
};

export const useCanvasSnapshots = (canvasId: number | null) => {
  const [snapshots, setSnapshots] = useState<CanvasSnapshot[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadSnapshots = useCallback(async () => {
    if (!canvasId) return;
    
    try {
      setLoading(true);
      setError(null);
      
      const data = await invoke<CanvasSnapshot[]>('get_canvas_snapshots', { canvasId });
      setSnapshots(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load snapshots');
    } finally {
      setLoading(false);
    }
  }, [canvasId]);

  const createSnapshot = useCallback(async (name: string, snapshotData: string) => {
    if (!canvasId) return;
    
    try {
      const newSnapshot = await invoke<CanvasSnapshot>('create_canvas_snapshot', {
        canvasId,
        name,
        snapshotData
      });
      
      setSnapshots(prev => [newSnapshot, ...prev]);
      return newSnapshot;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create snapshot';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, [canvasId]);

  const restoreSnapshot = useCallback(async (snapshotId: number) => {
    try {
      await invoke('restore_canvas_snapshot', { snapshotId });
      // Reload snapshots to get updated data
      await loadSnapshots();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to restore snapshot';
      setError(errorMessage);
      throw new Error(errorMessage);
    }
  }, [loadSnapshots]);

  useEffect(() => {
    if (canvasId) {
      loadSnapshots();
    } else {
      setSnapshots([]);
    }
  }, [canvasId, loadSnapshots]);

  return {
    snapshots,
    loading,
    error,
    loadSnapshots,
    createSnapshot,
    restoreSnapshot,
    clearError: () => setError(null)
  };
};
