import React, { useEffect, useState } from 'react';
import { useSettingsStore } from '../../stores/settingsStore';
import { emitSyncEvent, SyncEventType } from '../../utils/stateSynchronizer';
import { PerformanceSettings } from './PerformanceSettings';
import { PerformanceDashboard } from './PerformanceDashboard';
import SystemSettings from './SystemSettings';
import DataManagement from './DataManagement';

export const AppSettings: React.FC = () => {
  const {
    theme,
    editorSettings,
    accessibilitySettings,
    appSettings,
    setTheme,
    updateEditorSettings,
    updateAccessibilitySettings,
    updateAppSettings,
    syncWithBackend,
    loadFromBackend,
    isSyncing,
    lastSyncTime
  } = useSettingsStore();
  
  const [isSaved, setIsSaved] = useState(false);
  const [activeTab, setActiveTab] = useState<'general' | 'performance' | 'dashboard' | 'system' | 'data-management'>('general');
  
  // Handle form submission
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    await syncWithBackend();
    setIsSaved(true);
    
    // Emit sync event to notify other components about settings update
    emitSyncEvent(SyncEventType.SETTINGS_UPDATED, {
      category: 'app',
      key: 'settings',
      value: {
        theme,
        editorSettings,
        accessibilitySettings,
        appSettings
      }
    }).catch(err => {
      console.error('Failed to emit settings update event:', err);
    });
    
    setTimeout(() => setIsSaved(false), 3000);
  };
  
  // Load settings from backend when component mounts
  useEffect(() => {
    loadFromBackend();
  }, [loadFromBackend]);
  
  return (
    <div className="p-6 max-w-4xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Application Settings</h1>
      
      {isSaved && (
        <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4">
          Settings saved successfully!
        </div>
      )}
      
      <div className="mb-6 border-b border-gray-200">
        <ul className="flex flex-wrap -mb-px text-sm font-medium text-center">
          <li className="mr-2">
            <button
              className={`inline-block p-4 border-b-2 rounded-t-lg ${
                activeTab === 'general'
                  ? 'border-blue-600 text-blue-600'
                  : 'border-transparent hover:text-gray-600 hover:border-gray-300'
              }`}
              onClick={() => setActiveTab('general')}
              type="button"
            >
              General Settings
            </button>
          </li>
          <li className="mr-2">
            <button
              className={`inline-block p-4 border-b-2 rounded-t-lg ${
                activeTab === 'performance'
                  ? 'border-blue-600 text-blue-600'
                  : 'border-transparent hover:text-gray-600 hover:border-gray-300'
              }`}
              onClick={() => setActiveTab('performance')}
              type="button"
            >
              Performance Settings
            </button>
          </li>
          <li className="mr-2">
            <button
              className={`inline-block p-4 border-b-2 rounded-t-lg ${
                activeTab === 'dashboard'
                  ? 'border-blue-600 text-blue-600'
                  : 'border-transparent hover:text-gray-600 hover:border-gray-300'
              }`}
              onClick={() => setActiveTab('dashboard')}
              type="button"
            >
              Performance Dashboard
            </button>
          </li>
          <li className="mr-2">
            <button
              className={`inline-block p-4 border-b-2 rounded-t-lg ${
                activeTab === 'system'
                  ? 'border-blue-600 text-blue-600'
                  : 'border-transparent hover:text-gray-600 hover:border-gray-300'
              }`}
              onClick={() => setActiveTab('system')}
              type="button"
            >
              System
            </button>
          </li>
          <li className="mr-2">
            <button
              className={`inline-block p-4 border-b-2 rounded-t-lg ${
                activeTab === 'data-management'
                  ? 'border-blue-600 text-blue-600'
                  : 'border-transparent hover:text-gray-600 hover:border-gray-300'
              }`}
              onClick={() => setActiveTab('data-management')}
              type="button"
            >
              Data Management
            </button>
          </li>
        </ul>
      </div>
      
      {activeTab === 'general' ? (
        <form onSubmit={handleSubmit}>
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Theme</h2>
          <div className="flex gap-4">
            {['light', 'dark', 'system'].map((themeOption) => (
              <label key={themeOption} className="flex items-center">
                <input
                  type="radio"
                  name="theme"
                  value={themeOption}
                  checked={theme === themeOption}
                  onChange={() => setTheme(themeOption as 'light' | 'dark' | 'system')}
                  className="mr-2"
                />
                {themeOption.charAt(0).toUpperCase() + themeOption.slice(1)}
              </label>
            ))}
          </div>
        </div>
        
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Editor Settings</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label className="block mb-2">Font Size</label>
              <input
                type="number"
                value={editorSettings.fontSize}
                onChange={(e) => updateEditorSettings({ fontSize: parseInt(e.target.value) })}
                className="w-full p-2 border rounded"
                min="8"
                max="32"
              />
            </div>
            
            <div>
              <label className="block mb-2">Line Height</label>
              <select
                value={editorSettings.lineHeight}
                onChange={(e) => updateEditorSettings({ lineHeight: parseFloat(e.target.value) })}
                className="w-full p-2 border rounded"
              >
                <option value="1">Single</option>
                <option value="1.5">1.5</option>
                <option value="2">Double</option>
              </select>
            </div>
            
            <div>
              <label className="block mb-2">Font Family</label>
              <select
                value={editorSettings.fontFamily}
                onChange={(e) => updateEditorSettings({ fontFamily: e.target.value })}
                className="w-full p-2 border rounded"
              >
                <option value="Monaco, Menlo, Consolas, 'Courier New', monospace">Monospace</option>
                <option value="'Segoe UI', Tahoma, Geneva, Verdana, sans-serif">Sans-serif</option>
                <option value="'Times New Roman', Times, serif">Serif</option>
              </select>
            </div>
            
            <div className="flex items-center">
              <input
                type="checkbox"
                id="showLineNumbers"
                checked={editorSettings.showLineNumbers}
                onChange={(e) => updateEditorSettings({ showLineNumbers: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="showLineNumbers">Show Line Numbers</label>
            </div>
            
            <div className="flex items-center">
              <input
                type="checkbox"
                id="wordWrap"
                checked={editorSettings.wordWrap}
                onChange={(e) => updateEditorSettings({ wordWrap: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="wordWrap">Word Wrap</label>
            </div>
            
            <div className="flex items-center">
              <input
                type="checkbox"
                id="autoSave"
                checked={editorSettings.autoSave}
                onChange={(e) => updateEditorSettings({ autoSave: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="autoSave">Auto Save</label>
            </div>
            
            {editorSettings.autoSave && (
              <div>
                <label className="block mb-2">Auto Save Interval (ms)</label>
                <input
                  type="number"
                  value={editorSettings.autoSaveInterval}
                  onChange={(e) => updateEditorSettings({ autoSaveInterval: parseInt(e.target.value) })}
                  className="w-full p-2 border rounded"
                  min="1000"
                  step="1000"
                />
              </div>
            )}
          </div>
        </div>
        
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Accessibility</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="flex items-center">
              <input
                type="checkbox"
                id="highContrast"
                checked={accessibilitySettings.highContrast}
                onChange={(e) => updateAccessibilitySettings({ highContrast: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="highContrast">High Contrast</label>
            </div>
            
            <div className="flex items-center">
              <input
                type="checkbox"
                id="reducedMotion"
                checked={accessibilitySettings.reducedMotion}
                onChange={(e) => updateAccessibilitySettings({ reducedMotion: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="reducedMotion">Reduced Motion</label>
            </div>
            
            <div className="flex items-center">
              <input
                type="checkbox"
                id="largeText"
                checked={accessibilitySettings.largeText}
                onChange={(e) => updateAccessibilitySettings({ largeText: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="largeText">Large Text</label>
            </div>
          </div>
        </div>
        
        <div className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Application Settings</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="flex items-center">
              <input
                type="checkbox"
                id="confirmBeforeDelete"
                checked={appSettings.confirmBeforeDelete}
                onChange={(e) => updateAppSettings({ confirmBeforeDelete: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="confirmBeforeDelete">Confirm Before Delete</label>
            </div>
            
            <div className="flex items-center">
              <input
                type="checkbox"
                id="autoBackup"
                checked={appSettings.autoBackup}
                onChange={(e) => updateAppSettings({ autoBackup: e.target.checked })}
                className="mr-2"
              />
              <label htmlFor="autoBackup">Auto Backup</label>
            </div>
            
            {appSettings.autoBackup && (
              <>
                <div>
                  <label className="block mb-2">Backup Interval (minutes)</label>
                  <input
                    type="number"
                    value={appSettings.autoBackupInterval}
                    onChange={(e) => updateAppSettings({ autoBackupInterval: parseInt(e.target.value) })}
                    className="w-full p-2 border rounded"
                    min="5"
                  />
                </div>
                
                <div>
                  <label className="block mb-2">Max Backup Count</label>
                  <input
                    type="number"
                    value={appSettings.maxBackupCount}
                    onChange={(e) => updateAppSettings({ maxBackupCount: parseInt(e.target.value) })}
                    className="w-full p-2 border rounded"
                    min="1"
                    max="100"
                  />
                </div>
              </>
            )}
          </div>
        </div>
        
        <div className="flex justify-between items-center mt-8">
          <button
            type="submit"
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            disabled={isSyncing}
          >
            {isSyncing ? 'Saving...' : 'Save Settings'}
          </button>
          
          {lastSyncTime && (
            <span className="text-sm text-gray-500">
              Last saved: {new Date(lastSyncTime).toLocaleString()}
            </span>
          )}
        </div>
      </form>
      ) : activeTab === 'performance' ? (
        <PerformanceSettings />
      ) : activeTab === 'dashboard' ? (
        <PerformanceDashboard />
      ) : activeTab === 'system' ? (
        <SystemSettings />
      ) : (
        <DataManagement />
      )}
    </div>
  );
};
