import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/tauri';
import { CanvasManager } from '../CanvasManager';
import { Canvas as CanvasModel } from '../../../types/canvas';

import { Mock } from 'vitest';
// Mock the invoke function
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

const mockCanvases: CanvasModel[] = [
  { id: 1, project_id: '1', name: 'Canvas 1', description: 'Desc 1', width: 800, height: 600, created_at: new Date().toISOString(), updated_at: new Date().toISOString(), canvas_data: '{}', zoom_level: 1, viewport_x: 0, viewport_y: 0 },
  { id: 2, project_id: '1', name: 'Canvas 2', description: 'Desc 2', width: 1024, height: 768, created_at: new Date().toISOString(), updated_at: new Date().toISOString(), canvas_data: '{}', zoom_level: 1, viewport_x: 0, viewport_y: 0 },
];

describe('CanvasManager', () => {
  beforeEach(() => {
    (invoke as Mock).mockClear();
  });

  it('should load and display canvases on initial render', async () => {
    (invoke as Mock).mockResolvedValue(mockCanvases);
    render(<CanvasManager projectId="1" />);

    expect(invoke).toHaveBeenCalledWith('get_project_canvases', { projectId: '1' });

    await waitFor(() => {
      expect(screen.getByText('Canvas 1')).toBeInTheDocument();
      expect(screen.getByText('Canvas 2')).toBeInTheDocument();
    });
  });
  
  it('should select a canvas and make an announcement', async () => {
    (invoke as Mock).mockResolvedValue(mockCanvases);
    render(<CanvasManager projectId="1" />);
    
    await waitFor(() => {
      fireEvent.click(screen.getByText('Canvas 2'));
    });
    
    await waitFor(() => {
      const announcer = screen.getByText("Canvas 'Canvas 2' selected.");
      expect(announcer).toBeInTheDocument();
    });
  });

  it('should create a new canvas and make an announcement', async () => {
    (invoke as Mock).mockResolvedValueOnce(mockCanvases); // Initial load
    const newCanvas = { ...mockCanvases[0], id: 3, name: 'New Canvas' };
    (invoke as Mock).mockResolvedValueOnce(newCanvas); // Create canvas

    render(<CanvasManager projectId="1" />);

    fireEvent.click(screen.getByText('+ New Canvas'));

    fireEvent.change(screen.getByLabelText('Canvas Name:'), { target: { value: 'New Canvas' } });
    fireEvent.click(screen.getByText('Create Canvas'));
    
    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('create_canvas', { projectId: '1', name: 'New Canvas', description: '' });
    });

    await waitFor(() => {
      const announcer = screen.getByText("Canvas 'New Canvas' created.");
      expect(announcer).toBeInTheDocument();
      expect(screen.getByText('New Canvas')).toBeInTheDocument();
    });
  });

  it('should delete a canvas and make an announcement', async () => {
    window.confirm = vi.fn(() => true); // Mock confirm dialog
    (invoke as Mock).mockResolvedValue(mockCanvases); // Initial load
    (invoke as Mock).mockResolvedValueOnce(undefined); // Delete canvas

    render(<CanvasManager projectId="1" />);

    await waitFor(() => {
        const deleteButtons = screen.getAllByTitle('Delete Canvas');
        fireEvent.click(deleteButtons[0]);
    });

    await waitFor(() => {
      expect(invoke).toHaveBeenCalledWith('delete_canvas', { canvasId: 1 });
    });

    await waitFor(() => {
      const announcer = screen.getByText("Canvas 'Canvas 1' deleted.");
      expect(announcer).toBeInTheDocument();
      expect(screen.queryByText('Canvas 1')).not.toBeInTheDocument();
    });
  });
});