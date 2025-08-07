import React, { useEffect, useState } from 'react';
import { useTrashStore, DeletedItem, DeletedItemType } from '../../stores/trashStore';
import { formatDistanceToNow, format } from 'date-fns';

const TrashManager: React.FC = () => {
  const { 
    items, 
    isLoading, 
    error, 
    fetchTrashItems, 
    fetchTrashItemsByType,
    restoreItem, 
    permanentlyDeleteItem,
    emptyTrash
  } = useTrashStore();
  
  const [selectedItemId, setSelectedItemId] = useState<string | null>(null);
  const [selectedItemType, setSelectedItemType] = useState<DeletedItemType | 'all'>('all');
  const [showConfirmRestore, setShowConfirmRestore] = useState(false);
  const [showConfirmDelete, setShowConfirmDelete] = useState(false);
  const [showConfirmEmptyTrash, setShowConfirmEmptyTrash] = useState(false);
  const [isRestoring, setIsRestoring] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [isEmptyingTrash, setIsEmptyingTrash] = useState(false);
  
  useEffect(() => {
    if (selectedItemType === 'all') {
      fetchTrashItems();
    } else {
      fetchTrashItemsByType(selectedItemType);
    }
  }, [fetchTrashItems, fetchTrashItemsByType, selectedItemType]);
  
  const handleRestoreItem = async () => {
    if (!selectedItemId) return;
    
    try {
      setIsRestoring(true);
      await restoreItem(selectedItemId);
      setShowConfirmRestore(false);
      setSelectedItemId(null);
    } catch (error) {
      console.error('Failed to restore item:', error);
    } finally {
      setIsRestoring(false);
    }
  };
  
  const handlePermanentlyDeleteItem = async () => {
    if (!selectedItemId) return;
    
    try {
      setIsDeleting(true);
      await permanentlyDeleteItem(selectedItemId);
      setShowConfirmDelete(false);
      setSelectedItemId(null);
    } catch (error) {
      console.error('Failed to permanently delete item:', error);
    } finally {
      setIsDeleting(false);
    }
  };
  
  const handleEmptyTrash = async () => {
    try {
      setIsEmptyingTrash(true);
      await emptyTrash();
      setShowConfirmEmptyTrash(false);
    } catch (error) {
      console.error('Failed to empty trash:', error);
    } finally {
      setIsEmptyingTrash(false);
    }
  };
  
  const formatDeletedDate = (dateString: string) => {
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
  
  const getItemTypeLabel = (type: DeletedItemType) => {
    switch (type) {
      case DeletedItemType.Project:
        return 'Project';
      case DeletedItemType.Document:
        return 'Document';
      case DeletedItemType.Folder:
        return 'Folder';
      case DeletedItemType.Series:
        return 'Series';
      case DeletedItemType.Character:
        return 'Character';
      case DeletedItemType.Location:
        return 'Location';
      default:
        return type;
    }
  };
  
  const getItemName = (item: DeletedItem) => {
    try {
      const data = JSON.parse(item.item_data);
      return data.name || data.title || 'Unnamed item';
    } catch (error) {
      return 'Unknown item';
    }
  };
  
  const selectedItem = selectedItemId 
    ? items.find(item => item.id === selectedItemId) 
    : null;
  
  return (
    <div className="p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">Trash</h2>
        <button
          onClick={() => setShowConfirmEmptyTrash(true)}
          disabled={items.length === 0 || isLoading}
          className="px-3 py-1 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50"
        >
          Empty Trash
        </button>
      </div>
      
      {error && (
        <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded">
          {error}
        </div>
      )}
      
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
          Filter by type
        </label>
        <select
          value={selectedItemType}
          onChange={(e) => setSelectedItemType(e.target.value as DeletedItemType | 'all')}
          className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
        >
          <option value="all">All items</option>
          <option value={DeletedItemType.Project}>Projects</option>
          <option value={DeletedItemType.Document}>Documents</option>
          <option value={DeletedItemType.Folder}>Folders</option>
          <option value={DeletedItemType.Series}>Series</option>
          <option value={DeletedItemType.Character}>Characters</option>
          <option value={DeletedItemType.Location}>Locations</option>
        </select>
      </div>
      
      {isLoading ? (
        <div className="text-center py-8">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-4 border-gray-300 border-t-blue-600"></div>
          <p className="mt-2 text-gray-600 dark:text-gray-400">Loading trash items...</p>
        </div>
      ) : items.length === 0 ? (
        <div className="text-center py-8 text-gray-500 dark:text-gray-400">
          Trash is empty.
        </div>
      ) : (
        <div className="border dark:border-gray-700 rounded-md overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
            <thead className="bg-gray-50 dark:bg-gray-800">
              <tr>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Item</th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Type</th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Deleted</th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Reason</th>
                <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody className="bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700">
              {items.map((item) => {
                const date = formatDeletedDate(item.deleted_at);
                const isSelected = selectedItemId === item.id;
                const itemName = getItemName(item);
                
                return (
                  <tr 
                    key={item.id} 
                    className={`hover:bg-gray-50 dark:hover:bg-gray-750 ${isSelected ? 'bg-blue-50 dark:bg-blue-900/20' : ''}`}
                    onClick={() => setSelectedItemId(item.id)}
                  >
                    <td className="px-4 py-3 whitespace-nowrap">
                      <div className="font-medium text-gray-900 dark:text-white">
                        {itemName}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">
                        ID: {item.item_id}
                      </div>
                    </td>
                    <td className="px-4 py-3 whitespace-nowrap">
                      <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-300">
                        {getItemTypeLabel(item.item_type)}
                      </span>
                    </td>
                    <td className="px-4 py-3 whitespace-nowrap">
                      <div className="text-sm text-gray-900 dark:text-white">{date.relative}</div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">{date.exact}</div>
                    </td>
                    <td className="px-4 py-3 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                      {item.deletion_reason || 'No reason provided'}
                    </td>
                    <td className="px-4 py-3 whitespace-nowrap text-right text-sm font-medium">
                      {item.can_restore && (
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            setSelectedItemId(item.id);
                            setShowConfirmRestore(true);
                          }}
                          className="text-blue-600 dark:text-blue-400 hover:text-blue-900 dark:hover:text-blue-300 mr-3"
                        >
                          Restore
                        </button>
                      )}
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setSelectedItemId(item.id);
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
      
      {/* Restore Confirmation Dialog */}
      {showConfirmRestore && selectedItem && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Restore</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to restore <strong>{getItemName(selectedItem)}</strong>?
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setShowConfirmRestore(false)}
                className="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600"
              >
                Cancel
              </button>
              <button
                onClick={handleRestoreItem}
                disabled={isRestoring}
                className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
              >
                {isRestoring ? 'Restoring...' : 'Restore Item'}
              </button>
            </div>
          </div>
        </div>
      )}
      
      {/* Delete Confirmation Dialog */}
      {showConfirmDelete && selectedItem && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Permanent Deletion</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to permanently delete <strong>{getItemName(selectedItem)}</strong>?
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
                onClick={handlePermanentlyDeleteItem}
                disabled={isDeleting}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50"
              >
                {isDeleting ? 'Deleting...' : 'Delete Permanently'}
              </button>
            </div>
          </div>
        </div>
      )}
      
      {/* Empty Trash Confirmation Dialog */}
      {showConfirmEmptyTrash && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full">
            <h3 className="text-lg font-medium mb-4">Confirm Empty Trash</h3>
            <p className="mb-4 text-gray-700 dark:text-gray-300">
              Are you sure you want to permanently delete all items in the trash?
            </p>
            <p className="mb-6 text-red-600 dark:text-red-400 font-medium">
              This action cannot be undone. All items in the trash will be permanently deleted.
            </p>
            <div className="flex justify-end gap-3">
              <button
                onClick={() => setShowConfirmEmptyTrash(false)}
                className="px-4 py-2 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600"
              >
                Cancel
              </button>
              <button
                onClick={handleEmptyTrash}
                disabled={isEmptyingTrash}
                className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 disabled:opacity-50"
              >
                {isEmptyingTrash ? 'Emptying Trash...' : 'Empty Trash'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default TrashManager;
