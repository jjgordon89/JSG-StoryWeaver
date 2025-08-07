import React, { useState } from 'react';
import BackupManager from './BackupManager';
import TrashManager from './TrashManager';
import { useSettingsStore } from '../../stores/settingsStore';
import { useVersionStore } from '../../stores/versionStore';

type DataManagementTab = 'backup' | 'trash' | 'version-overview';

const DataManagement: React.FC = () => {
  const [activeTab, setActiveTab] = useState<DataManagementTab>('backup');
  const { appSettings } = useSettingsStore();
  
  return (
    <div className="p-6 bg-white dark:bg-gray-800 rounded-lg shadow">
      <h1 className="text-2xl font-bold mb-6 text-gray-900 dark:text-white">Data Management</h1>
      
      <div className="flex border-b border-gray-200 dark:border-gray-700 mb-6 overflow-x-auto">
        <button
          className={`py-2 px-4 font-medium whitespace-nowrap ${
            activeTab === 'backup'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('backup')}
        >
          Database Backup
        </button>
        <button
          className={`py-2 px-4 font-medium whitespace-nowrap ${
            activeTab === 'trash'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('trash')}
        >
          Trash Management
        </button>
        <button
          className={`py-2 px-4 font-medium whitespace-nowrap ${
            activeTab === 'version-overview'
              ? 'text-blue-600 dark:text-blue-400 border-b-2 border-blue-600 dark:border-blue-400'
              : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
          onClick={() => setActiveTab('version-overview')}
        >
          Version History Overview
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
                  readOnly
                  className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="autoBackup" className="ml-2 block text-sm text-gray-900 dark:text-gray-200">
                  Automatic backups: {appSettings.autoBackup ? 'Enabled' : 'Disabled'}
                </label>
              </div>
              {appSettings.autoBackup && (
                <div className="text-xs text-gray-500 dark:text-gray-400 mt-1 ml-6">
                  Frequency: {getBackupIntervalText(appSettings.autoBackupInterval)}<br />
                  Maximum backups: {appSettings.maxBackupCount}
                </div>
              )}
              <div className="mt-2 text-sm text-gray-600 dark:text-gray-300">
                You can change these settings in the <span className="font-medium">System Settings</span> tab.
              </div>
            </div>
          </div>
          
          <BackupManager />
        </div>
      )}
      
      {activeTab === 'trash' && (
        <div>
          <div className="mb-6 bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
            <h3 className="text-lg font-medium mb-4">Trash Management</h3>
            <p className="text-sm text-gray-600 dark:text-gray-300">
              Deleted items are stored in the trash for recovery. Items in the trash can be restored or permanently deleted.
            </p>
          </div>
          
          <TrashManager />
        </div>
      )}
      
      {activeTab === 'version-overview' && (
        <VersionOverview />
      )}
    </div>
  );
};

// Helper component for version history overview
const VersionOverview: React.FC = () => {
  const { 
    versionStatistics, 
    isLoadingStatistics, 
    error, 
    fetchVersionStatistics 
  } = useVersionStore();
  
  // Load version statistics when component mounts
  React.useEffect(() => {
    fetchVersionStatistics().catch(error => {
      console.error('Failed to load version statistics:', error);
    });
  }, [fetchVersionStatistics]);
  
  const formatDate = (dateString: string | null) => {
    if (!dateString) return 'N/A';
    try {
      return new Date(dateString).toLocaleDateString(undefined, {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    } catch (error) {
      return 'Invalid date';
    }
  };
  
  return (
    <div>
      <div className="mb-6 bg-gray-50 dark:bg-gray-700 p-4 rounded-lg">
        <h3 className="text-lg font-medium mb-4">Version History Overview</h3>
        <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">
          StoryWeaver automatically creates versions of your documents as you work. You can access version history for a specific document from the document editor.
        </p>
        
        {isLoadingStatistics ? (
          <div className="flex justify-center items-center py-8">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-4 border-gray-300 border-t-blue-600"></div>
            <span className="ml-2 text-gray-600 dark:text-gray-400">Loading statistics...</span>
          </div>
        ) : error ? (
          <div className="p-4 text-red-600 dark:text-red-400 bg-red-100 dark:bg-red-900/20 rounded">
            {error}
          </div>
        ) : versionStatistics ? (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
              <div className="text-sm text-gray-500 dark:text-gray-400">Total Versions</div>
              <div className="text-2xl font-bold text-gray-900 dark:text-white">{versionStatistics.totalVersions}</div>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
              <div className="text-sm text-gray-500 dark:text-gray-400">Documents with Versions</div>
              <div className="text-2xl font-bold text-gray-900 dark:text-white">{versionStatistics.documentsWithVersions}</div>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
              <div className="text-sm text-gray-500 dark:text-gray-400">Oldest Version</div>
              <div className="text-lg font-medium text-gray-900 dark:text-white">{formatDate(versionStatistics.oldestVersion)}</div>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
              <div className="text-sm text-gray-500 dark:text-gray-400">Newest Version</div>
              <div className="text-lg font-medium text-gray-900 dark:text-white">{formatDate(versionStatistics.newestVersion)}</div>
            </div>
            
            <div className="bg-white dark:bg-gray-800 p-4 rounded-lg shadow md:col-span-2">
              <div className="text-sm text-gray-500 dark:text-gray-400">Average Versions per Document</div>
              <div className="text-2xl font-bold text-gray-900 dark:text-white">{versionStatistics.averageVersionsPerDocument.toFixed(1)}</div>
            </div>
          </div>
        ) : (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            No version statistics available.
          </div>
        )}
      </div>
      
      <div className="bg-blue-50 dark:bg-blue-900/20 border-l-4 border-blue-500 p-4 rounded">
        <div className="flex">
          <div className="flex-shrink-0">
            <svg className="h-5 w-5 text-blue-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
            </svg>
          </div>
          <div className="ml-3">
            <p className="text-sm text-blue-700 dark:text-blue-300">
              To access version history for a specific document, open the document in the editor and click the version history icon in the toolbar.
            </p>
          </div>
        </div>
      </div>
      
      <div className="mt-6">
        <h3 className="text-lg font-medium mb-4">Version Management Tips</h3>
        <ul className="list-disc pl-5 space-y-2 text-gray-700 dark:text-gray-300">
          <li>Create manual versions before making significant changes to your document</li>
          <li>Use the version comment feature to describe what changed in each version</li>
          <li>Compare versions to see what changed between drafts</li>
          <li>Delete old versions you no longer need to save space</li>
          <li>Restore previous versions if you need to revert changes</li>
        </ul>
      </div>
    </div>
  );
};

// Helper function to format backup interval
const getBackupIntervalText = (minutes: number): string => {
  if (minutes < 60) {
    return `Every ${minutes} minutes`;
  } else if (minutes === 60) {
    return 'Every hour';
  } else if (minutes < 60 * 24) {
    return `Every ${minutes / 60} hours`;
  } else if (minutes === 60 * 24) {
    return 'Daily';
  } else if (minutes === 60 * 24 * 7) {
    return 'Weekly';
  } else if (minutes === 60 * 24 * 30) {
    return 'Monthly';
  } else {
    return `Every ${minutes} minutes`;
  }
};

export default DataManagement;
