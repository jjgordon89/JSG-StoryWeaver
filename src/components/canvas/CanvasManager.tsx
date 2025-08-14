import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Canvas as CanvasModel } from '../../types/canvas';
import { Canvas } from './Canvas';
import LoadingSpinner from '../ui/LoadingSpinner';
import ErrorMessage from '../ui/ErrorMessage';
import './CanvasManager.css';

interface CanvasManagerProps {
  projectId: string;
}

export const CanvasManager: React.FC<CanvasManagerProps> = ({ projectId }) => {
  const [canvases, setCanvases] = useState<CanvasModel[]>([]);
  const [selectedCanvas, setSelectedCanvas] = useState<CanvasModel | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [newCanvasName, setNewCanvasName] = useState('');
  const [newCanvasDescription, setNewCanvasDescription] = useState('');

  useEffect(() => {
    loadCanvases();
  }, [projectId]);

  const loadCanvases = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const canvasData = await invoke<CanvasModel[]>('get_project_canvases', {
        projectId
      });
      
      setCanvases(canvasData);
      
      // Select the first canvas if none is selected
      if (canvasData.length > 0 && !selectedCanvas) {
        setSelectedCanvas(canvasData[0]);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load canvases');
    } finally {
      setLoading(false);
    }
  };

  const createCanvas = async () => {
    if (!newCanvasName.trim()) {
      setError('Canvas name is required');
      return;
    }

    try {
      const newCanvas = await invoke<CanvasModel>('create_canvas', {
        projectId,
        name: newCanvasName.trim(),
        description: newCanvasDescription.trim() || undefined
      });

      setCanvases(prev => [newCanvas, ...prev]);
      setSelectedCanvas(newCanvas);
      setShowCreateDialog(false);
      setNewCanvasName('');
      setNewCanvasDescription('');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create canvas');
    }
  };

  const deleteCanvas = async (canvasId: number) => {
    if (!confirm('Are you sure you want to delete this canvas? This action cannot be undone.')) {
      return;
    }

    try {
      await invoke('delete_canvas', { canvasId });
      
      setCanvases(prev => prev.filter(c => c.id !== canvasId));
      
      if (selectedCanvas?.id === canvasId) {
        const remainingCanvases = canvases.filter(c => c.id !== canvasId);
        setSelectedCanvas(remainingCanvases.length > 0 ? remainingCanvases[0] : null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete canvas');
    }
  };

  const handleCanvasChange = (canvas: CanvasModel) => {
    setCanvases(prev => prev.map(c => c.id === canvas.id ? canvas : c));
  };

  if (loading) {
    return <LoadingSpinner />;
  }

  return (
    <div className="canvas-manager">
      {error && <ErrorMessage message={error} onDismiss={() => setError(null)} />}
      
      <div className="canvas-sidebar">
        <div className="sidebar-header">
          <h3>Canvases</h3>
          <button
            className="btn btn-primary create-btn"
            onClick={() => setShowCreateDialog(true)}
          >
            + New Canvas
          </button>
        </div>

        <div className="canvas-list">
          {canvases.length === 0 ? (
            <div className="no-canvases">
              <p>No canvases yet.</p>
              <button
                className="btn btn-primary"
                onClick={() => setShowCreateDialog(true)}
              >
                Create your first canvas
              </button>
            </div>
          ) : (
            canvases.map(canvas => (
              <div
                key={canvas.id}
                className={`canvas-item ${selectedCanvas?.id === canvas.id ? 'selected' : ''}`}
                onClick={() => setSelectedCanvas(canvas)}
              >
                <div className="canvas-item-content">
                  <h4 className="canvas-item-name">{canvas.name}</h4>
                  {canvas.description && (
                    <p className="canvas-item-description">{canvas.description}</p>
                  )}
                  <div className="canvas-item-meta">
                    <span className="canvas-size">
                      {canvas.width} × {canvas.height}
                    </span>
                    <span className="canvas-date">
                      {new Date(canvas.updated_at).toLocaleDateString()}
                    </span>
                  </div>
                </div>
                <div className="canvas-item-actions">
                  <button
                    className="action-btn delete-btn"
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteCanvas(canvas.id);
                    }}
                    title="Delete Canvas"
                  >
                    🗑️
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      <div className="canvas-main">
        {selectedCanvas ? (
          <Canvas
            projectId={projectId}
            canvasId={selectedCanvas.id}
            onCanvasChange={handleCanvasChange}
          />
        ) : (
          <div className="no-canvas-selected">
            <h3>Select a Canvas</h3>
            <p>Choose a canvas from the sidebar to start visual planning.</p>
          </div>
        )}
      </div>

      {showCreateDialog && (
        <div className="create-canvas-dialog">
          <div className="dialog-overlay" onClick={() => setShowCreateDialog(false)}></div>
          <div className="dialog-panel">
            <div className="dialog-header">
              <h3>Create New Canvas</h3>
              <button className="close-btn" onClick={() => setShowCreateDialog(false)}>×</button>
            </div>
            
            <div className="dialog-content">
              <div className="form-group">
                <label htmlFor="canvas-name">Canvas Name:</label>
                <input
                  id="canvas-name"
                  type="text"
                  value={newCanvasName}
                  onChange={(e) => setNewCanvasName(e.target.value)}
                  className="form-control"
                  placeholder="Enter canvas name..."
                  autoFocus
                />
              </div>
              
              <div className="form-group">
                <label htmlFor="canvas-description">Description (optional):</label>
                <textarea
                  id="canvas-description"
                  value={newCanvasDescription}
                  onChange={(e) => setNewCanvasDescription(e.target.value)}
                  className="form-control"
                  placeholder="Enter canvas description..."
                  rows={3}
                />
              </div>
            </div>
            
            <div className="dialog-actions">
              <button
                className="btn btn-primary"
                onClick={createCanvas}
                disabled={!newCanvasName.trim()}
              >
                Create Canvas
              </button>
              <button
                className="btn btn-secondary"
                onClick={() => setShowCreateDialog(false)}
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
