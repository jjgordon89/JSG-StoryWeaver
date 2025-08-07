import React, { useState } from 'react';
import BackupManager from './BackupManager';
import TrashManager from './TrashManager';
import { useSettingsStore } from '../../stores/settingsStore';

type SettingsTab = 'backup' | 'trash' | 'performance' | 'security';

const SystemSettings: React.FC = () => {
  const [activeTab, setActiveTab] = useState<SettingsTab>('backup');
  const { appSettings, updateAppSettings } = useSettingsStore();
  
  const handleAutoBackupChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    updateAppSettings({ autoBackup: e.target.checked });
  };
  
  const handleAutoBackupIntervalChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    updateAppSettings({ autoBackupInterval: parseInt(e.target.value, 10) });
  };
  
  const handleMaxBackupCountChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    updateAppSettings({ maxBackupCount: parseInt(e.target.value, 10) });
  };
  
  return (
    <div className="p-6 bg-white dark:bg-gray-800 rounded-lg shadow">
      <h1 className="text-2xl font-bold mb-6 text-gray-900 dark:text-white">System Settings</h1>
      
      <div className="flex border-b border-gray-200 dark:border-gray-700 mb-6">
        <button
          className={`py-2 px-4 font-medium ${
            activeTab === 'backup'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('backup')}
        >
          Backup
        </button>
        <button
          className={`py-2 px-4 font-medium ${
            activeTab === 'trash'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('trash')}
        >
          Trash
        </button>
        <button
          className={`py-2 px-4 font-medium ${
            activeTab === 'performance'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('performance')}
        >
          Performance
        </button>
        <button
          className={`py-2 px-4 font-medium ${
            activeTab === 'security'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('security')}
        >
          Security
        </button>
      </div>
      
      {activeTab === 'backup' && (
        <div>
          <div className="mb-6 bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <h3 className="text-lg font-medium mb-4">Backup Settings</h3>
            
            <div className="mb-4">
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="autoBackup"
                  checked={appSettings.autoBackup}
                  onChange={handleAutoBackupChange}
                  className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="autoBackup" className="ml-2 block text-sm text-gray-900 dark:text-gray-200">
                  Enable automatic backups
                </label>
              </div>
              <p className="text-xs text-gray-500 dark:text-gray-400 mt-1 ml-6">
                Automatically create backups of your database at regular intervals
              </p>
            </div>
            
            <div className="mb-4">
              <label htmlFor="autoBackupInterval" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Backup interval
              </label>
              <select
                id="autoBackupInterval"
                value={appSettings.autoBackupInterval}
                onChange={handleAutoBackupIntervalChange}
                disabled={!appSettings.autoBackup}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white disabled:opacity-50"
              >
                <option value={60}>Every hour</option>
                <option value={60 * 24}>Daily</option>
                <option value={60 * 24 * 7}>Weekly</option>
                <option value={60 * 24 * 30}>Monthly</option>
              </select>
            </div>
            
            <div className="mb-4">
              <label htmlFor="maxBackupCount" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                Maximum number of automatic backups to keep
              </label>
              <select
                id="maxBackupCount"
                value={appSettings.maxBackupCount}
                onChange={handleMaxBackupCountChange}
                disabled={!appSettings.autoBackup}
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white disabled:opacity-50"
              >
                <option value={5}>5</option>
                <option value={10}>10</option>
                <option value={20}>20</option>
                <option value={50}>50</option>
              </select>
              <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                Older backups will be automatically deleted when this limit is reached
              </p>
            </div>
          </div>
          
          <BackupManager />
        </div>
      )}
      
      {activeTab === 'trash' && (
        <div>
          <TrashManager />
        </div>
      )}
      
      {activeTab === 'performance' && (
        <div className="text-center py-8 text-gray-500 dark:text-gray-400">
          Performance settings are available in the Performance Dashboard component.
        </div>
      )}
      
      {activeTab === 'security' && (
        <div className="text-center py-8 text-gray-500 dark:text-gray-400">
          Security settings are available in the Security Settings component.
        </div>
      )}
    </div>
  );
};

export default SystemSettings;
