import React, { useEffect, useState } from 'react';
import { useVersionStore, VersionHistoryItem, DocumentVersion } from '../../stores/versionStore';
import { formatDistanceToNow, format } from 'date-fns';

interface VersionHistoryProps {
  documentId: string;
  onClose: () => void;
  onRestoreVersion: (version: DocumentVersion) => void;
}

const VersionHistory: React.FC<VersionHistoryProps> = ({ 
  documentId, 
  onClose,
  onRestoreVersion
}) => {
  const { 
    versionHistory, 
    isLoading, 
    error, 
    fetchVersionHistory,
    getVersion,
    restoreVersion,
    deleteVersion,
    deleteAllVersions
  } = useVersionStore();
  
  const [selectedVersionId, setSelectedVersionId] = useState<string | null>(null);
  const [selectedVersion, setSelectedVersion] = useState<DocumentVersion | null>(null);
  const [isViewingVersion, setIsViewingVersion] = useState(false);
  const [isRestoring, setIsRestoring] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [isDeletingAll, setIsDeletingAll] = useState(false);
  const [showConfirmDelete, setShowConfirmDelete] = useState(false);
  const [showConfirmDeleteAll, setShowConfirmDeleteAll] = useState(false);
  
  useEffect(() => {
    fetchVersionHistory(documentId);
  }, [fetchVersionHistory, documentId]);
  
  const handleViewVersion = async (versionId: string) => {
    try {
      const version = await getVersion(versionId);
      if (version) {
        setSelectedVersion(version);
        setIsViewingVersion(true);
      }
    } catch (error) {
      console.error('Failed to get version:', error);
    }
  };
  
  const handleRestoreVersion = async () => {
    if (!selectedVersionId) return;
    
    try {
      setIsRestoring(true);
      await restoreVersion(selectedVersionId);
      
      // Get the restored version to pass back to parent
      const restoredVersion = await getVersion(selectedVersionId);
      if (restoredVersion) {
        onRestoreVersion(restoredVersion);
      }
      
      setIsViewingVersion(false);
    } catch (error) {
      console.error('Failed to restore version:', error);
    } finally {
      setIsRestoring(false);
    }
  };
  
  const handleDeleteVersion = async () => {
    if (!selectedVersionId) return;
    
    try {
      setIsDeleting(true);
      await deleteVersion(selectedVersionId);
      setShowConfirmDelete(false);
      setSelectedVersionId(null);
      setSelectedVersion(null);
      setIsViewingVersion(false);
    } catch (error) {
      console.error('Failed to delete version:', error);
    } finally {
      setIsDeleting(false);
    }
  };
  
  const handleDeleteAllVersions = async () => {
    try {
      setIsDeletingAll(true);
      await deleteAllVersions(documentId);
      setShowConfirmDeleteAll(false);
    } catch (error) {
      console.error('Failed to delete all versions:', error);
    } finally {
      setIsDeletingAll(false);
    }
  };
  
  const formatVersionDate = (dateString: string) => {
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
  
  const formatWordCount = (count: number, change: number) => {
    if (change > 0) {
      return `${count} (+${change})`;
    } else if (change < 0) {
      return `${count} (${change})`;
    } else {
      return count.toString();
    }
  };
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg w-full max-w-4xl max-h-[90vh] flex flex-col">
        {/* Header */}
        <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">Document Version History</h2>
          <button
            onClick={onClose}
            className="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        
        {/* Content */}
        <div className="flex flex-1 overflow-hidden">
          {/* Version List */}
          <div className="w-1/3 border-r border-gray-200 dark:border-gray-700 overflow-y-auto">
            <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
              <h3 className="font-medium">Versions</h3>
              {versionHistory.length > 0 && (
                <button
                  onClick={() => setShowConfirmDeleteAll(true)}
                  className="text-sm text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-300"
                >
                  Delete All
                </button>
              )}
            </div>
            
            {isLoading ? (
              <div className="flex items-center justify-center p-8">
                <div className="inline-block animate-spin rounded-full h-8 w-8 border-4 border-gray-300 border-t-blue-600"></div>
                <span className="ml-2 text-gray-600 dark:text-gray-400">Loading versions...</span>
              </div>
            ) : error ? (
              <div className="p-4 text-red-600 dark:text-red-400">
                {error}
              </div>
            ) : versionHistory.length === 0 ? (
              <div className="p-4 text-gray-500 dark:text-gray-400 text-center">
                No version history available.
              </div>
            ) : (
              <ul className="divide-y divide-gray-200 dark:divide-gray-700">
                {versionHistory.map((version) => {
                  const date = formatVersionDate(version.created_at);
                  const isSelected = selectedVersionId === version.id;
                  
                  return (
                    <li 
                      key={version.id}
                      className={`p-4 cursor-pointer hover:bg-gray-50 dark:hover:bg-gray-750 ${
                        isSelected ? 'bg-blue-50 dark:bg-blue-900/20' : ''
                      }`}
                      onClick={() => {
                        setSelectedVersionId(version.id);
                        handleViewVersion(version.id);
                      }}
                    >
                      <div className="flex justify-between items-start">
                        <div>
                          <div className="font-medium text-gray-900 dark:text-white">
                            Version {version.version_number}
                          </div>
                          <div className="text-sm text-gray-500 dark:text-gray-400">
                            {date.relative}
                          </div>
                          <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                            {date.exact}
                          </div>
                        </div>
                        <div className="text-right">
                          <div className="text-sm font-medium text-gray-900 dark:text-white">
                            {formatWordCount(version.word_count, version.word_count_change)}
                          </div>
                          <div className="text-xs text-gray-500 dark:text-gray-400">
                            words
                          </div>
                        </div>
                      </div>
                      
                      {version.comment && (
                        <div className="mt-2 text-sm text-gray-600 dark:text-gray-300 bg-gray-50 dark:bg-gray-700 p-2 rounded">
                          {version.comment}
                        </div>
                      )}
                    </li>
                  );
                })}
              </ul>
            )}
          </div>
          
          {/* Version Preview */}
          <div className="w-2/3 flex flex-col overflow-hidden">
            {isViewingVersion && selectedVersion ? (
              <>
                <div className="p-4 border-b border-gray-200 dark:border-gray-700 flex justify-between items-center">
                  <div>
                    <h3 className="font-medium">Version {selectedVersion.version_number}</h3>
                    <p className="text-sm text-gray-500 dark:text-gray-400">
                      {selectedVersion.word_count} words
                    </p>
                  </div>
                  <div className="flex space-x-2">
                    <button
                      onClick={() => setShowConfirmDelete(true)}
                      className="px-3 py-1 text-sm text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-300 border border-red-600 dark:border-red-400 rounded"
                    >
                      Delete
                    </button>
                    <button
                      onClick={handleRestoreVersion}
                      disabled={isRestoring}
                      className="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
                    >
                      {isRestoring ? 'Restoring...' : 'Restore This Version'}
                    </button>
                  </div>
                </div>
                
                <div className="flex-1 overflow-y-auto p-4">
                  <div className="prose dark:prose-invert max-w-none">
                    {selectedVersion.content.split('\n').map((line, i) => (
                      <p key={i}>{line || <br />}</p>
                    ))}
                  </div>
                </div>
              </>
            ) : (
              <div className="flex-1 flex items-center justify-center text-gray-500 dark:text-gray-400">
                {versionHistory.length > 0 ? (
                  <p>Select a version to preview</p>
                ) : (
                  <p>No version history available</p>
                )}
              </div>
            )}
          </div>
        </div>
      </div>
      
      {/* Delete Version Confirmation Dialog */}
      {showConfirmDelete && selectedVersion && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Delete Version</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to delete version {selectedVersion.version_number}?
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
                onClick={handleDeleteVersion}
                disabled={isDeleting}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50"
              >
                {isDeleting ? 'Deleting...' : 'Delete Version'}
              </button>
            </div>
          </div>
        </div>
      )}
      
      {/* Delete All Versions Confirmation Dialog */}
      {showConfirmDeleteAll && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Delete All Versions</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to delete all version history for this document?
            </p>
            <p className="mb-6 text-red-600 dark:text-red-400 font-medium">
              This action cannot be undone. All version history will be permanently deleted.
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setShowConfirmDeleteAll(false)}
                className="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600"
              >
                Cancel
              </button>
              <button
                onClick={handleDeleteAllVersions}
                disabled={isDeletingAll}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50"
              >
                {isDeletingAll ? 'Deleting...' : 'Delete All Versions'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default VersionHistory;
