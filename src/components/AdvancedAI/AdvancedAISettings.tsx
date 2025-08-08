import React, { useState, useEffect } from 'react';
import { useAdvancedAIStore } from '../../stores/advancedAIStore';
import type { AdvancedAISettings } from '../../types/advancedAI';

const AdvancedAISettings: React.FC = () => {
  const {
    settings,
    loadSettings,
    updateSettings,
    resetSettings,
    exportSettings,
    importSettings
  } = useAdvancedAIStore();

  const [localSettings, setLocalSettings] = useState<AdvancedAISettings>(settings);
  const [hasChanges, setHasChanges] = useState(false);
  const [showResetConfirm, setShowResetConfirm] = useState(false);
  const [showExportModal, setShowExportModal] = useState(false);
  const [showImportModal, setShowImportModal] = useState(false);
  const [importData, setImportData] = useState('');
  const [activeSection, setActiveSection] = useState('general');
  const [isSaving, setIsSaving] = useState(false);

  const sections = [
    { id: 'general', label: 'General', icon: 'fas fa-cog' },
    { id: 'generation', label: 'Generation', icon: 'fas fa-magic' },
    { id: 'saliency', label: 'Saliency Engine', icon: 'fas fa-brain' },
    { id: 'image', label: 'Image Generation', icon: 'fas fa-image' },
    { id: 'brainstorm', label: 'Brainstorming', icon: 'fas fa-lightbulb' },
    { id: 'performance', label: 'Performance', icon: 'fas fa-tachometer-alt' },
    { id: 'privacy', label: 'Privacy', icon: 'fas fa-shield-alt' }
  ];

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  useEffect(() => {
    setLocalSettings(settings);
  }, [settings]);

  useEffect(() => {
    const hasChanges = JSON.stringify(localSettings) !== JSON.stringify(settings);
    setHasChanges(hasChanges);
  }, [localSettings, settings]);

  const handleSettingChange = (path: string, value: any) => {
    setLocalSettings(prev => {
      const newSettings = { ...prev };
      const keys = path.split('.');
      let current: any = newSettings;
      
      for (let i = 0; i < keys.length - 1; i++) {
        if (!(keys[i] in current)) {
          current[keys[i]] = {};
        }
        current = current[keys[i]];
      }
      
      current[keys[keys.length - 1]] = value;
      return newSettings;
    });
  };

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await updateSettings(localSettings);
      setHasChanges(false);
    } catch (error) {
      console.error('Error saving settings:', error);
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = async () => {
    try {
      await resetSettings();
      setShowResetConfirm(false);
      setHasChanges(false);
    } catch (error) {
      console.error('Error resetting settings:', error);
    }
  };

  const handleExport = async () => {
    try {
      const exportedData = await exportSettings();
      const blob = new Blob([JSON.stringify(exportedData, null, 2)], {
        type: 'application/json'
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `storyweaver-ai-settings-${new Date().toISOString().split('T')[0]}.json`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      setShowExportModal(false);
    } catch (error) {
      console.error('Error exporting settings:', error);
    }
  };

  const handleImport = async () => {
    try {
      const parsedData = JSON.parse(importData);
      await importSettings(parsedData);
      setShowImportModal(false);
      setImportData('');
    } catch (error) {
      console.error('Error importing settings:', error);
    }
  };

  const handleFileImport = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        const content = e.target?.result as string;
        setImportData(content);
      };
      reader.readAsText(file);
    }
  };

  const renderGeneralSettings = () => (
    <div className="settings-section">
      <h3>General Settings</h3>
      
      <div className="setting-group">
        <label htmlFor="default-prose-mode">Default Prose Mode</label>
        <select
          id="default-prose-mode"
          value={localSettings.general.defaultProseMode}
          onChange={(e) => handleSettingChange('general.defaultProseMode', e.target.value)}
        >
          <option value="creative">Creative</option>
          <option value="technical">Technical</option>
          <option value="academic">Academic</option>
          <option value="casual">Casual</option>
        </select>
        <span className="setting-description">
          The default writing style for AI generation
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.general.autoSave}
            onChange={(e) => handleSettingChange('general.autoSave', e.target.checked)}
          />
          <span className="checkmark"></span>
          Auto-save generated content
        </label>
        <span className="setting-description">
          Automatically save AI-generated content to your project
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.general.showAdvancedOptions}
            onChange={(e) => handleSettingChange('general.showAdvancedOptions', e.target.checked)}
          />
          <span className="checkmark"></span>
          Show advanced options by default
        </label>
        <span className="setting-description">
          Display advanced settings in generation interfaces
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.general.enableStreaming}
            onChange={(e) => handleSettingChange('general.enableStreaming', e.target.checked)}
          />
          <span className="checkmark"></span>
          Enable streaming responses
        </label>
        <span className="setting-description">
          Show AI responses as they're being generated
        </span>
      </div>
    </div>
  );

  const renderGenerationSettings = () => (
    <div className="settings-section">
      <h3>Generation Settings</h3>
      
      <div className="setting-group">
        <label htmlFor="default-context-length">Default Context Length</label>
        <div className="range-input">
          <input
            type="range"
            id="default-context-length"
            min="1000"
            max="8000"
            step="500"
            value={localSettings.generation.defaultContextLength}
            onChange={(e) => handleSettingChange('generation.defaultContextLength', parseInt(e.target.value))}
          />
          <span className="range-value">{localSettings.generation.defaultContextLength} tokens</span>
        </div>
        <span className="setting-description">
          Amount of context to include in AI requests
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="default-output-length">Default Output Length</label>
        <div className="range-input">
          <input
            type="range"
            id="default-output-length"
            min="100"
            max="2000"
            step="100"
            value={localSettings.generation.defaultOutputLength}
            onChange={(e) => handleSettingChange('generation.defaultOutputLength', parseInt(e.target.value))}
          />
          <span className="range-value">{localSettings.generation.defaultOutputLength} tokens</span>
        </div>
        <span className="setting-description">
          Target length for AI-generated content
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="creativity-level">Creativity Level</label>
        <div className="range-input">
          <input
            type="range"
            id="creativity-level"
            min="0"
            max="1"
            step="0.1"
            value={localSettings.generation.creativityLevel}
            onChange={(e) => handleSettingChange('generation.creativityLevel', parseFloat(e.target.value))}
          />
          <span className="range-value">{(localSettings.generation.creativityLevel * 100).toFixed(0)}%</span>
        </div>
        <span className="setting-description">
          Higher values produce more creative but less predictable results
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.generation.enableClicheDetection}
            onChange={(e) => handleSettingChange('generation.enableClicheDetection', e.target.checked)}
          />
          <span className="checkmark"></span>
          Enable cliché detection
        </label>
        <span className="setting-description">
          Automatically detect and highlight potential clichés
        </span>
      </div>
    </div>
  );

  const renderSaliencySettings = () => (
    <div className="settings-section">
      <h3>Saliency Engine Settings</h3>
      
      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.saliencyEngine.enabled}
            onChange={(e) => handleSettingChange('saliencyEngine.enabled', e.target.checked)}
          />
          <span className="checkmark"></span>
          Enable Saliency Engine
        </label>
        <span className="setting-description">
          Automatically analyze and prioritize story elements
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.saliencyEngine.autoBuild}
            onChange={(e) => handleSettingChange('saliencyEngine.autoBuild', e.target.checked)}
            disabled={!localSettings.saliencyEngine.enabled}
          />
          <span className="checkmark"></span>
          Auto-build saliency maps
        </label>
        <span className="setting-description">
          Automatically update saliency analysis as you write
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="refresh-interval">Refresh Interval</label>
        <select
          id="refresh-interval"
          value={localSettings.saliencyEngine.refreshInterval}
          onChange={(e) => handleSettingChange('saliencyEngine.refreshInterval', parseInt(e.target.value))}
          disabled={!localSettings.saliencyEngine.enabled || !localSettings.saliencyEngine.autoBuild}
        >
          <option value="30">30 seconds</option>
          <option value="60">1 minute</option>
          <option value="300">5 minutes</option>
          <option value="600">10 minutes</option>
        </select>
        <span className="setting-description">
          How often to refresh saliency analysis
        </span>
      </div>

      <div className="setting-group">
        <label>Include in Analysis</label>
        <div className="checkbox-group">
          {['characters', 'plot', 'themes', 'settings', 'conflicts'].map((element) => (
            <label key={element} className="checkbox-label">
              <input
                type="checkbox"
                checked={localSettings.saliencyEngine.includedElements.includes(element)}
                onChange={(e) => {
                  const elements = localSettings.saliencyEngine.includedElements;
                  if (e.target.checked) {
                    handleSettingChange('saliencyEngine.includedElements', [...elements, element]);
                  } else {
                    handleSettingChange('saliencyEngine.includedElements', elements.filter(el => el !== element));
                  }
                }}
                disabled={!localSettings.saliencyEngine.enabled}
              />
              <span className="checkmark"></span>
              {element.charAt(0).toUpperCase() + element.slice(1)}
            </label>
          ))}
        </div>
        <span className="setting-description">
          Story elements to include in saliency analysis
        </span>
      </div>
    </div>
  );

  const renderImageSettings = () => (
    <div className="settings-section">
      <h3>Image Generation Settings</h3>
      
      <div className="setting-group">
        <label htmlFor="default-style">Default Art Style</label>
        <select
          id="default-style"
          value={localSettings.imageGeneration.defaultStyle}
          onChange={(e) => handleSettingChange('imageGeneration.defaultStyle', e.target.value)}
        >
          <option value="realistic">Realistic</option>
          <option value="artistic">Artistic</option>
          <option value="fantasy">Fantasy</option>
          <option value="sci-fi">Sci-Fi</option>
          <option value="cartoon">Cartoon</option>
          <option value="sketch">Sketch</option>
        </select>
        <span className="setting-description">
          Default artistic style for image generation
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="default-resolution">Default Resolution</label>
        <select
          id="default-resolution"
          value={localSettings.imageGeneration.defaultResolution}
          onChange={(e) => handleSettingChange('imageGeneration.defaultResolution', e.target.value)}
        >
          <option value="512x512">512x512 (Square)</option>
          <option value="768x512">768x512 (Landscape)</option>
          <option value="512x768">512x768 (Portrait)</option>
          <option value="1024x1024">1024x1024 (High Quality)</option>
        </select>
        <span className="setting-description">
          Default image dimensions
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="quality-level">Quality Level</label>
        <select
          id="quality-level"
          value={localSettings.imageGeneration.qualityLevel}
          onChange={(e) => handleSettingChange('imageGeneration.qualityLevel', e.target.value)}
        >
          <option value="draft">Draft (Fast)</option>
          <option value="standard">Standard</option>
          <option value="high">High Quality</option>
          <option value="ultra">Ultra (Slow)</option>
        </select>
        <span className="setting-description">
          Balance between speed and image quality
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.imageGeneration.enablePromptEnhancement}
            onChange={(e) => handleSettingChange('imageGeneration.enablePromptEnhancement', e.target.checked)}
          />
          <span className="checkmark"></span>
          Enable prompt enhancement
        </label>
        <span className="setting-description">
          Automatically improve image prompts with AI
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.imageGeneration.useStoryContext}
            onChange={(e) => handleSettingChange('imageGeneration.useStoryContext', e.target.checked)}
          />
          <span className="checkmark"></span>
          Use story context
        </label>
        <span className="setting-description">
          Include story details in image generation
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.imageGeneration.autoSaveImages}
            onChange={(e) => handleSettingChange('imageGeneration.autoSaveImages', e.target.checked)}
          />
          <span className="checkmark"></span>
          Auto-save generated images
        </label>
        <span className="setting-description">
          Automatically save images to project gallery
        </span>
      </div>
    </div>
  );

  const renderBrainstormSettings = () => (
    <div className="settings-section">
      <h3>Brainstorming Settings</h3>
      
      <div className="setting-group">
        <label htmlFor="session-duration">Default Session Duration</label>
        <select
          id="session-duration"
          value={localSettings.brainstorming.defaultSessionDuration}
          onChange={(e) => handleSettingChange('brainstorming.defaultSessionDuration', parseInt(e.target.value))}
        >
          <option value="5">5 minutes</option>
          <option value="10">10 minutes</option>
          <option value="15">15 minutes</option>
          <option value="30">30 minutes</option>
          <option value="60">1 hour</option>
        </select>
        <span className="setting-description">
          Default length for brainstorming sessions
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="ideas-per-generation">Ideas per Generation</label>
        <div className="range-input">
          <input
            type="range"
            id="ideas-per-generation"
            min="3"
            max="15"
            step="1"
            value={localSettings.brainstorming.ideasPerGeneration}
            onChange={(e) => handleSettingChange('brainstorming.ideasPerGeneration', parseInt(e.target.value))}
          />
          <span className="range-value">{localSettings.brainstorming.ideasPerGeneration} ideas</span>
        </div>
        <span className="setting-description">
          Number of ideas to generate at once
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.brainstorming.autoRateIdeas}
            onChange={(e) => handleSettingChange('brainstorming.autoRateIdeas', e.target.checked)}
          />
          <span className="checkmark"></span>
          Auto-rate generated ideas
        </label>
        <span className="setting-description">
          Automatically assign quality ratings to ideas
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.brainstorming.useStoryContext}
            onChange={(e) => handleSettingChange('brainstorming.useStoryContext', e.target.checked)}
          />
          <span className="checkmark"></span>
          Use story context
        </label>
        <span className="setting-description">
          Include current story details in brainstorming
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.brainstorming.autoExportKeepers}
            onChange={(e) => handleSettingChange('brainstorming.autoExportKeepers', e.target.checked)}
          />
          <span className="checkmark"></span>
          Auto-export keeper ideas
        </label>
        <span className="setting-description">
          Automatically add keeper ideas to Story Bible
        </span>
      </div>
    </div>
  );

  const renderPerformanceSettings = () => (
    <div className="settings-section">
      <h3>Performance Settings</h3>
      
      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.performance.enableCaching}
            onChange={(e) => handleSettingChange('performance.enableCaching', e.target.checked)}
          />
          <span className="checkmark"></span>
          Enable response caching
        </label>
        <span className="setting-description">
          Cache AI responses to improve performance
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.performance.preloadModels}
            onChange={(e) => handleSettingChange('performance.preloadModels', e.target.checked)}
          />
          <span className="checkmark"></span>
          Preload AI models
        </label>
        <span className="setting-description">
          Load models in advance for faster generation
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.performance.enableBackgroundProcessing}
            onChange={(e) => handleSettingChange('performance.enableBackgroundProcessing', e.target.checked)}
          />
          <span className="checkmark"></span>
          Enable background processing
        </label>
        <span className="setting-description">
          Process AI tasks in the background
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="concurrent-generations">Concurrent Generations</label>
        <div className="range-input">
          <input
            type="range"
            id="concurrent-generations"
            min="1"
            max="5"
            step="1"
            value={localSettings.performance.maxConcurrentGenerations}
            onChange={(e) => handleSettingChange('performance.maxConcurrentGenerations', parseInt(e.target.value))}
          />
          <span className="range-value">{localSettings.performance.maxConcurrentGenerations}</span>
        </div>
        <span className="setting-description">
          Maximum number of simultaneous AI generations
        </span>
      </div>
    </div>
  );

  const renderPrivacySettings = () => (
    <div className="settings-section">
      <h3>Privacy Settings</h3>
      
      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.privacy.saveGenerationHistory}
            onChange={(e) => handleSettingChange('privacy.saveGenerationHistory', e.target.checked)}
          />
          <span className="checkmark"></span>
          Save generation history
        </label>
        <span className="setting-description">
          Keep a history of AI generations for reference
        </span>
      </div>

      <div className="setting-group">
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={localSettings.privacy.shareUsageData}
            onChange={(e) => handleSettingChange('privacy.shareUsageData', e.target.checked)}
          />
          <span className="checkmark"></span>
          Share anonymous usage data
        </label>
        <span className="setting-description">
          Help improve StoryWeaver by sharing anonymous usage statistics
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="data-retention">Data Retention Period</label>
        <select
          id="data-retention"
          value={localSettings.privacy.dataRetentionDays}
          onChange={(e) => handleSettingChange('privacy.dataRetentionDays', parseInt(e.target.value))}
        >
          <option value="7">1 week</option>
          <option value="30">1 month</option>
          <option value="90">3 months</option>
          <option value="365">1 year</option>
          <option value="-1">Forever</option>
        </select>
        <span className="setting-description">
          How long to keep AI generation data
        </span>
      </div>

      <div className="setting-group">
        <label htmlFor="export-format">Export Format</label>
        <select
          id="export-format"
          value={localSettings.privacy.exportFormat}
          onChange={(e) => handleSettingChange('privacy.exportFormat', e.target.value)}
        >
          <option value="json">JSON</option>
          <option value="csv">CSV</option>
          <option value="txt">Plain Text</option>
        </select>
        <span className="setting-description">
          Default format for data exports
        </span>
      </div>
    </div>
  );

  const renderSection = () => {
    switch (activeSection) {
      case 'general': return renderGeneralSettings();
      case 'generation': return renderGenerationSettings();
      case 'saliency': return renderSaliencySettings();
      case 'image': return renderImageSettings();
      case 'brainstorm': return renderBrainstormSettings();
      case 'performance': return renderPerformanceSettings();
      case 'privacy': return renderPrivacySettings();
      default: return renderGeneralSettings();
    }
  };

  return (
    <div className="advanced-ai-settings">
      <div className="settings-header">
        <div className="header-title">
          <h2>Advanced AI Settings</h2>
          <p>Configure AI behavior and preferences for optimal writing assistance</p>
        </div>
        
        {hasChanges && (
          <div className="unsaved-indicator">
            <i className="fas fa-circle"></i>
            <span>Unsaved changes</span>
          </div>
        )}
      </div>

      <div className="settings-container">
        {/* Settings Navigation */}
        <div className="settings-nav">
          {sections.map((section) => (
            <button
              key={section.id}
              className={`nav-item ${activeSection === section.id ? 'active' : ''}`}
              onClick={() => setActiveSection(section.id)}
            >
              <i className={section.icon}></i>
              <span>{section.label}</span>
            </button>
          ))}
        </div>

        {/* Settings Content */}
        <div className="settings-content">
          {renderSection()}
        </div>
      </div>

      {/* Settings Actions */}
      <div className="settings-actions">
        <div className="primary-actions">
          <button
            className="save-btn"
            onClick={handleSave}
            disabled={!hasChanges || isSaving}
          >
            {isSaving ? (
              <>
                <i className="fas fa-spinner fa-spin"></i>
                Saving...
              </>
            ) : (
              <>
                <i className="fas fa-save"></i>
                Save Changes
              </>
            )}
          </button>
          
          <button
            className="reset-btn"
            onClick={() => setShowResetConfirm(true)}
          >
            <i className="fas fa-undo"></i>
            Reset to Defaults
          </button>
        </div>

        <div className="secondary-actions">
          <button
            className="export-btn"
            onClick={() => setShowExportModal(true)}
          >
            <i className="fas fa-download"></i>
            Export Settings
          </button>
          
          <button
            className="import-btn"
            onClick={() => setShowImportModal(true)}
          >
            <i className="fas fa-upload"></i>
            Import Settings
          </button>
        </div>
      </div>

      {/* Reset Confirmation Modal */}
      {showResetConfirm && (
        <div className="modal-overlay" onClick={() => setShowResetConfirm(false)}>
          <div className="confirm-modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h3>Reset Settings</h3>
            </div>
            <div className="modal-content">
              <p>Are you sure you want to reset all settings to their default values? This action cannot be undone.</p>
            </div>
            <div className="modal-actions">
              <button className="confirm-btn" onClick={handleReset}>
                <i className="fas fa-undo"></i>
                Reset
              </button>
              <button className="cancel-btn" onClick={() => setShowResetConfirm(false)}>
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Export Modal */}
      {showExportModal && (
        <div className="modal-overlay" onClick={() => setShowExportModal(false)}>
          <div className="export-modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h3>Export Settings</h3>
              <button className="close-btn" onClick={() => setShowExportModal(false)}>
                <i className="fas fa-times"></i>
              </button>
            </div>
            <div className="modal-content">
              <p>Export your current AI settings to a file for backup or sharing.</p>
              <div className="export-options">
                <div className="option">
                  <i className="fas fa-cog"></i>
                  <span>All settings and preferences</span>
                </div>
                <div className="option">
                  <i className="fas fa-palette"></i>
                  <span>Style examples and templates</span>
                </div>
                <div className="option">
                  <i className="fas fa-shield-alt"></i>
                  <span>Privacy settings (excluding sensitive data)</span>
                </div>
              </div>
            </div>
            <div className="modal-actions">
              <button className="export-btn" onClick={handleExport}>
                <i className="fas fa-download"></i>
                Export
              </button>
              <button className="cancel-btn" onClick={() => setShowExportModal(false)}>
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Import Modal */}
      {showImportModal && (
        <div className="modal-overlay" onClick={() => setShowImportModal(false)}>
          <div className="import-modal" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h3>Import Settings</h3>
              <button className="close-btn" onClick={() => setShowImportModal(false)}>
                <i className="fas fa-times"></i>
              </button>
            </div>
            <div className="modal-content">
              <p>Import AI settings from a previously exported file.</p>
              
              <div className="import-methods">
                <div className="method">
                  <label htmlFor="file-upload" className="file-upload-btn">
                    <i className="fas fa-file-upload"></i>
                    Choose File
                  </label>
                  <input
                    id="file-upload"
                    type="file"
                    accept=".json"
                    onChange={handleFileImport}
                    style={{ display: 'none' }}
                  />
                </div>
                
                <div className="method">
                  <label htmlFor="paste-data">Or paste settings data:</label>
                  <textarea
                    id="paste-data"
                    value={importData}
                    onChange={(e) => setImportData(e.target.value)}
                    placeholder="Paste exported settings JSON here..."
                    rows={6}
                  />
                </div>
              </div>
            </div>
            <div className="modal-actions">
              <button
                className="import-btn"
                onClick={handleImport}
                disabled={!importData.trim()}
              >
                <i className="fas fa-upload"></i>
                Import
              </button>
              <button className="cancel-btn" onClick={() => setShowImportModal(false)}>
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default AdvancedAISettings;