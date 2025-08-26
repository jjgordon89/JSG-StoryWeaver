import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import * as RadioGroup from '@radix-ui/react-radio-group';
import { Canvas, ExportFormat, CanvasExportResult } from '../../types/canvas';
import LoadingSpinner from '../ui/LoadingSpinner';
import ErrorMessage from '../ui/ErrorMessage';
import './CanvasExportDialog.css';

interface CanvasExportDialogProps {
  canvas: Canvas;
  onClose: () => void;
}

export const CanvasExportDialog: React.FC<CanvasExportDialogProps> = ({
  canvas,
  onClose
}) => {
  const [selectedFormat, setSelectedFormat] = useState<ExportFormat>('markdown');
  const [isExporting, setIsExporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [exportResult, setExportResult] = useState<CanvasExportResult | null>(null);

  const exportFormats: { format: ExportFormat; label: string; description: string; icon: string }[] = [
    {
      format: 'markdown',
      label: 'Markdown',
      description: 'Export as structured markdown document',
      icon: '📝'
    },
    {
      format: 'story_bible',
      label: 'Story Bible',
      description: 'Export as comprehensive story bible format',
      icon: '📚'
    },
    {
      format: 'outline',
      label: 'Outline',
      description: 'Export as structured story outline',
      icon: '📋'
    },
    {
      format: 'json',
      label: 'JSON',
      description: 'Export raw canvas data as JSON',
      icon: '🔧'
    },
    {
      format: 'png',
      label: 'PNG Image',
      description: 'Export visual representation as PNG image',
      icon: '🖼️'
    },
    {
      format: 'svg',
      label: 'SVG Vector',
      description: 'Export as scalable vector graphics',
      icon: '🎨'
    },
    {
      format: 'pdf',
      label: 'PDF Document',
      description: 'Export as PDF document',
      icon: '📄'
    }
  ];

  const handleExport = async () => {
    try {
      setIsExporting(true);
      setError(null);

      const result = await invoke<CanvasExportResult>('export_canvas', {
        canvasId: canvas.id,
        format: selectedFormat
      });

      setExportResult(result);
      
      if (['markdown', 'story_bible', 'outline', 'json'].includes(selectedFormat)) {
        const blob = new Blob([result.data], {
          type: getContentType(selectedFormat)
        });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${canvas.name}_${selectedFormat}.${getFileExtension(selectedFormat)}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
      }
      
      if (['png', 'svg', 'pdf'].includes(selectedFormat)) {
        const a = document.createElement('a');
        a.href = `data:${getContentType(selectedFormat)};base64,${result.data}`;
        a.download = `${canvas.name}_export.${getFileExtension(selectedFormat)}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
      }

    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to export canvas');
    } finally {
      setIsExporting(false);
    }
  };

  const getContentType = (format: ExportFormat): string => {
    switch (format) {
      case 'markdown': return 'text/markdown';
      case 'story_bible': return 'text/plain';
      case 'outline': return 'text/plain';
      case 'json': return 'application/json';
      case 'png': return 'image/png';
      case 'svg': return 'image/svg+xml';
      case 'pdf': return 'application/pdf';
      default: return 'text/plain';
    }
  };

  const getFileExtension = (format: ExportFormat): string => {
    switch (format) {
      case 'markdown': return 'md';
      case 'story_bible': return 'txt';
      case 'outline': return 'txt';
      case 'json': return 'json';
      case 'png': return 'png';
      case 'svg': return 'svg';
      case 'pdf': return 'pdf';
      default: return 'txt';
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return (
    <div className="canvas-export-dialog-overlay" onClick={onClose}>
      <div className="canvas-export-dialog" onClick={(e) => e.stopPropagation()}>
        <div className="export-dialog-header">
          <h3>Export Canvas: {canvas.name}</h3>
          <button
            className="close-btn"
            onClick={onClose}
            aria-label="Close export dialog"
          >
            ✕
          </button>
        </div>

        {error && (
          <ErrorMessage
            message={error}
            onDismiss={() => setError(null)}
          />
        )}

        <div className="export-dialog-content">
          <div className="format-selection">
            <h4>Select Export Format</h4>
            <RadioGroup.Root
              className="format-grid"
              value={selectedFormat}
              onValueChange={(value) => setSelectedFormat(value as ExportFormat)}
              aria-label="Export format"
            >
              {exportFormats.map(({ format, label, description, icon }) => (
                <label
                  key={format}
                  className={`format-option ${selectedFormat === format ? 'selected' : ''}`}
                >
                  <div className="format-icon">{icon}</div>
                  <div className="format-info">
                    <div className="format-label">{label}</div>
                    <div className="format-description">{description}</div>
                  </div>
                  <div className="format-radio">
                    <RadioGroup.Item
                      value={format}
                      className="radio-group-item"
                      id={`format-${format}`}
                    >
                      <RadioGroup.Indicator className="radio-group-indicator" />
                    </RadioGroup.Item>
                  </div>
                </label>
              ))}
            </RadioGroup.Root>
          </div>

          {exportResult && (
            <div className="export-result">
              <h4>Export Completed</h4>
              <div className="result-info">
                <div className="result-item">
                  <strong>Format:</strong> {exportResult.format.toUpperCase()}
                </div>
                <div className="result-item">
                  <strong>File Size:</strong> {formatFileSize(exportResult.file_size)}
                </div>
                <div className="result-item">
                  <strong>Exported:</strong> {new Date(exportResult.exported_at).toLocaleString()}
                </div>
              </div>
              <div className="success-message">
                ✅ Canvas exported successfully! The file has been downloaded to your default downloads folder.
              </div>
            </div>
          )}

          <div className="canvas-info">
            <h4>Canvas Information</h4>
            <div className="info-grid">
              <div className="info-item">
                <strong>Name:</strong> {canvas.name}
              </div>
              {canvas.description && (
                <div className="info-item">
                  <strong>Description:</strong> {canvas.description}
                </div>
              )}
              <div className="info-item">
                <strong>Dimensions:</strong> {canvas.width} × {canvas.height}
              </div>
              <div className="info-item">
                <strong>Created:</strong> {new Date(canvas.created_at).toLocaleDateString()}
              </div>
              <div className="info-item">
                <strong>Last Modified:</strong> {new Date(canvas.updated_at).toLocaleDateString()}
              </div>
            </div>
          </div>
        </div>

        <div className="export-dialog-actions">
          <button
            className="btn btn-secondary"
            onClick={onClose}
            disabled={isExporting}
          >
            Cancel
          </button>
          <button
            className="btn btn-primary export-btn"
            onClick={handleExport}
            disabled={isExporting}
          >
            {isExporting ? (
              <>
                <LoadingSpinner size="small" />
                Exporting...
              </>
            ) : (
              <>
                📤 Export {selectedFormat.toUpperCase()}
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
};
