import React, { useEffect, useState } from 'react';
import { useBackupStore, BackupInfo } from '../../stores/backupStore';
import { formatDistanceToNow, format } from 'date-fns';

const BackupManager: React.FC = () => {
  const { 
    backups, 
    isLoading, 
    error, 
    fetchBackups, 
    createBackup, 
    restoreBackup, 
    deleteBackup,
    cleanupOldBackups
  } = useBackupStore();
  
  const [backupName, setBackupName] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const [isRestoring, setIsRestoring] = useState(false);
  const [selectedBackupId, setSelectedBackupId] = useState<string | null>(null);
  const [showConfirmRestore, setShowConfirmRestore] = useState(false);
  const [showConfirmDelete, setShowConfirmDelete] = useState(false);
  
  useEffect(() => {
    fetchBackups();
  }, [fetchBackups]);
  
  const handleCreateBackup = async () => {
    try {
      setIsCreating(true);
      await createBackup(backupName || undefined);
      setBackupName('');
    } catch (error) {
      console.error('Failed to create backup:', error);
    } finally {
      setIsCreating(false);
    }
  };
  
  const handleRestoreBackup = async () => {
    if (!selectedBackupId) return;
    
    try {
      setIsRestoring(true);
      await restoreBackup(selectedBackupId);
      setShowConfirmRestore(false);
      // Show success message or notification
    } catch (error) {
      console.error('Failed to restore backup:', error);
    } finally {
      setIsRestoring(false);
    }
  };
  
  const handleDeleteBackup = async () => {
    if (!selectedBackupId) return;
    
    try {
      await deleteBackup(selectedBackupId);
      setSelectedBackupId(null);
      setShowConfirmDelete(false);
    } catch (error) {
      console.error('Failed to delete backup:', error);
    }
  };
  
  const handleCleanupOldBackups = async () => {
    try {
      await cleanupOldBackups();
      // Show success message
    } catch (error) {
      console.error('Failed to cleanup old backups:', error);
    }
  };
  
  const formatBackupDate = (dateString: string) => {
    try {
      const date = new Date(dateString);
      return {
        relative: formatDistanceToNow(date, { addSuffix: true }),
        exact: format(date, 'PPpp')
      };
    } catch (error) {
      return { relative: 'Unknown date', exact: dateString };
    }
  };
  
  const formatFileSize = (bytes: number | null) => {
    if (bytes === null) return 'Unknown';
    
    const units = ['B', 'KB', 'MB', 'GB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  };
  
  const selectedBackup = selectedBackupId 
    ? backups.find(b => b.id === selectedBackupId) 
    : null;
  
  return (
    <div className="p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
      <h2 className="text-xl font-semibold mb-4">Database Backup Manager</h2>
      
      {error && (
        <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded">
          {error}
        </div>
      )}
      
      <div className="mb-6">
        <h3 className="text-lg font-medium mb-2">Create New Backup</h3>
        <div className="flex gap-2">
          <input
            type="text"
            value={backupName}
            onChange={(e) => setBackupName(e.target.value)}
            placeholder="Backup name (optional)"
            className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
          />
          <button
            onClick={handleCreateBackup}
            disabled={isCreating || isLoading}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
          >
            {isCreating ? 'Creating...' : 'Create Backup'}
          </button>
        </div>
        <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
          Creates a snapshot of your database that you can restore later if needed.
        </p>
      </div>
      
      <div className="mb-6">
        <div className="flex justify-between items-center mb-2">
          <h3 className="text-lg font-medium">Available Backups</h3>
          <button
            onClick={handleCleanupOldBackups}
            className="text-sm px-3 py-1 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600"
          >
            Cleanup Old Backups
          </button>
        </div>
        
        {isLoading ? (
          <div className="text-center py-8">
            <div className="inline-block animate-spin rounded-full h-8 w-8 border-4 border-gray-300 border-t-blue-600"></div>
            <p className="mt-2 text-gray-600 dark:text-gray-400">Loading backups...</p>
          </div>
        ) : backups.length === 0 ? (
          <div className="text-center py-8 text-gray-500 dark:text-gray-400">
            No backups available. Create your first backup above.
          </div>
        ) : (
          <div className="border dark:border-gray-700 rounded-md overflow-hidden">
            <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
              <thead className="bg-gray-50 dark:bg-gray-800">
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Name</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Created</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Size</th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Type</th>
                  <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Actions</th>
                </tr>
              </thead>
              <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
                {backups.map((backup) => {
                  const date = formatBackupDate(backup.created_at);
                  const isSelected = selectedBackupId === backup.id;
                  
                  return (
                    <tr 
                      key={backup.id} 
                      className={`hover:bg-gray-50 dark:hover:bg-gray-750 ${isSelected ? 'bg-blue-50 dark:bg-blue-900/20' : ''}`}
                      onClick={() => setSelectedBackupId(backup.id)}
                    >
                      <td className="px-4 py-3 whitespace-nowrap">
                        <div className="font-medium text-gray-900 dark:text-white">
                          {backup.comment || backup.filename.replace(/backup_|\.db$/g, '')}
                        </div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">
                          {backup.filename}
                        </div>
                      </td>
                      <td className="px-4 py-3 whitespace-nowrap">
                        <div className="text-sm text-gray-900 dark:text-white">{date.relative}</div>
                        <div className="text-xs text-gray-500 dark:text-gray-400">{date.exact}</div>
                      </td>
                      <td className="px-4 py-3 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                        {backup.file_exists ? formatFileSize(backup.file_size) : 'File missing'}
                      </td>
                      <td className="px-4 py-3 whitespace-nowrap">
                        <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                          backup.is_auto 
                            ? 'bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-300' 
                            : 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-300'
                        }`}>
                          {backup.is_auto ? 'Auto' : 'Manual'}
                        </span>
                      </td>
                      <td className="px-4 py-3 whitespace-nowrap text-right text-sm font-medium">
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            setSelectedBackupId(backup.id);
                            setShowConfirmRestore(true);
                          }}
                          disabled={!backup.file_exists}
                          className="text-blue-600 dark:text-blue-400 hover:text-blue-900 dark:hover:text-blue-300 mr-3 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                          Restore
                        </button>
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            setSelectedBackupId(backup.id);
                            setShowConfirmDelete(true);
                          }}
                          className="text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300"
                        >
                          Delete
                        </button>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>
      
      {/* Restore Confirmation Dialog */}
      {showConfirmRestore && selectedBackup && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Restore</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to restore from backup <strong>{selectedBackup.comment || selectedBackup.filename}</strong>?
              This will replace your current database with the backup.
            </p>
            <p className="mb-6 text-amber-600 dark:text-amber-400 font-medium">
              This action cannot be undone. The application will restart after restore.
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setShowConfirmRestore(false)}
                className="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600"
              >
                Cancel
              </button>
              <button
                onClick={handleRestoreBackup}
                disabled={isRestoring}
                className="px-4 py-2 bg-amber-600 text-white rounded-md hover:bg-amber-700 focus:outline-none focus:ring-2 focus:ring-amber-500 disabled:opacity-50"
              >
                {isRestoring ? 'Restoring...' : 'Restore Backup'}
              </button>
            </div>
          </div>
        </div>
      )}
      
      {/* Delete Confirmation Dialog */}
      {showConfirmDelete && selectedBackup && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Delete</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to delete the backup <strong>{selectedBackup.comment || selectedBackup.filename}</strong>?
            </p>
            <p className="mb-6 text-red-600 dark:text-red-400 font-medium">
              This action cannot be undone.
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setShowConfirmDelete(false)}
                className="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600"
              >
                Cancel
              </button>
              <button
                onClick={handleDeleteBackup}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
              >
                Delete Backup
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default BackupManager;
