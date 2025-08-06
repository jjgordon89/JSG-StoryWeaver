import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Folder {
  id: string;
  name: string;
  parent_folder_id: string | null;
  is_series: boolean;
  created_at: string;
  children?: Folder[];
}

interface Document {
  id: string;
  title: string;
  document_type: string;
}

interface FolderHierarchyProps {
  projectId: string;
  onDocumentSelect?: (documentId: string) => void;
}

const FolderHierarchy: React.FC<FolderHierarchyProps> = ({ projectId, onDocumentSelect }) => {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [documents, setDocuments] = useState<Record<string, Document[]>>({});
  const [expandedFolders, setExpandedFolders] = useState<Set<string>>(new Set());
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [draggedItem, setDraggedItem] = useState<{ type: 'folder' | 'document', id: string } | null>(null);

  // Fetch folder structure
  useEffect(() => {
    const fetchFolders = async () => {
      setLoading(true);
      try {
        // Get folder tree from backend
        const response = await invoke<{ data: any[] }>('get_folder_tree');
        
        if (!response.data) {
          throw new Error('Failed to fetch folder tree');
        }
        
        // Convert the folder tree to our format
        const rootFolders = response.data.map(convertFolderTreeNode);
        setFolders(rootFolders);
        
        // Fetch documents for each folder
        const docsData: Record<string, Document[]> = {};
        
        // For each folder, fetch its documents
        const allFolders = getAllFoldersFromTree(rootFolders);
        for (const folder of allFolders) {
          try {
            // This assumes there's a command to get documents by folder ID
            // If not available, you might need to fetch all documents and filter
            const docsResponse = await invoke<{ data: Document[] }>('get_documents_by_folder', { folderId: folder.id });
            if (docsResponse.data) {
              docsData[folder.id] = docsResponse.data;
            }
          } catch (err) {
            console.error(`Error fetching documents for folder ${folder.id}:`, err);
          }
        }
        
        setDocuments(docsData);
      } catch (err) {
        console.error('Error fetching folders:', err);
        setError('Failed to load folder structure');
      } finally {
        setLoading(false);
      }
    };
    
    fetchFolders();
  }, [projectId]);

  // Toggle folder expansion
  const toggleFolder = (folderId: string) => {
    setExpandedFolders(prev => {
      const newSet = new Set(prev);
      if (newSet.has(folderId)) {
        newSet.delete(folderId);
      } else {
        newSet.add(folderId);
      }
      return newSet;
    });
  };

  // Helper function to convert folder tree node from backend format
  const convertFolderTreeNode = (node: any): Folder => {
    return {
      id: node.id,
      name: node.name,
      parent_folder_id: node.parent_folder_id,
      is_series: node.is_series,
      created_at: node.created_at,
      children: node.children ? node.children.map(convertFolderTreeNode) : []
    };
  };
  
  // Helper function to get all folders from tree
  const getAllFoldersFromTree = (folders: Folder[]): Folder[] => {
    let result: Folder[] = [];
    
    for (const folder of folders) {
      result.push(folder);
      if (folder.children && folder.children.length > 0) {
        result = result.concat(getAllFoldersFromTree(folder.children));
      }
    }
    
    return result;
  };

  // Create a new folder
  const createFolder = async (parentId: string | null, name: string) => {
    try {
      // Call the backend API to create a folder
      const response = await invoke<{ data: Folder }>('create_folder', {
        request: {
          name,
          parent_folder_id: parentId,
          is_series: false
        }
      });
      
      if (!response.data) {
        throw new Error('Failed to create folder');
      }
      
      // Refresh folder structure
      const treeResponse = await invoke<{ data: any[] }>('get_folder_tree');
      if (treeResponse.data) {
        const rootFolders = treeResponse.data.map(convertFolderTreeNode);
        setFolders(rootFolders);
      }
    } catch (err) {
      console.error('Error creating folder:', err);
      setError('Failed to create folder');
    }
  };

  // Handle drag start
  const handleDragStart = (e: React.DragEvent, type: 'folder' | 'document', id: string) => {
    setDraggedItem({ type, id });
    e.dataTransfer.setData('text/plain', JSON.stringify({ type, id }));
    e.dataTransfer.effectAllowed = 'move';
  };

  // Handle drag over
  const handleDragOver = (e: React.DragEvent, folderId: string) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
  };

  // Handle drop
  const handleDrop = async (e: React.DragEvent, targetFolderId: string) => {
    e.preventDefault();
    
    if (!draggedItem) return;
    
    try {
      const { type, id } = draggedItem;
      
      if (type === 'folder') {
        // Update the folder's parent
        await invoke('update_folder', {
          request: {
            id,
            parent_folder_id: targetFolderId
          }
        });
      } else {
        // Move document to folder
        await invoke('move_items_to_folder', {
          request: {
            folder_id: targetFolderId,
            project_ids: [],
            document_ids: [id]
          }
        });
      }
      
      // Refresh folder structure
      const treeResponse = await invoke<{ data: any[] }>('get_folder_tree');
      if (treeResponse.data) {
        const rootFolders = treeResponse.data.map(convertFolderTreeNode);
        setFolders(rootFolders);
      }
      
      // Refresh documents
      const allFolders = getAllFoldersFromTree(folders);
      const docsData: Record<string, Document[]> = { ...documents };
      
      // Update documents for the target folder
      const docsResponse = await invoke<{ data: Document[] }>('get_documents_by_folder', { folderId: targetFolderId });
      if (docsResponse.data) {
        docsData[targetFolderId] = docsResponse.data;
        setDocuments(docsData);
      }
    } catch (err) {
      console.error('Error moving item:', err);
      setError('Failed to move item');
    } finally {
      setDraggedItem(null);
    }
  };

  // Render a folder and its children recursively
  const renderFolder = (folder: Folder, depth = 0) => {
    const isExpanded = expandedFolders.has(folder.id);
    const folderDocs = documents[folder.id] || [];
    
    return (
      <div key={folder.id} className="select-none">
        <div 
          className={`flex items-center p-1 rounded cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-700
            ${folder.is_series ? 'text-purple-600 dark:text-purple-400 font-medium' : ''}`}
          style={{ paddingLeft: `${depth * 16 + 8}px` }}
          onClick={() => toggleFolder(folder.id)}
          draggable
          onDragStart={(e) => handleDragStart(e, 'folder', folder.id)}
          onDragOver={(e) => handleDragOver(e, folder.id)}
          onDrop={(e) => handleDrop(e, folder.id)}
        >
          <span className="mr-1">{isExpanded ? '▼' : '►'}</span>
          <span className={folder.is_series ? 'text-purple-600 dark:text-purple-400' : ''}>
            {folder.is_series ? '📚' : '📁'} {folder.name}
          </span>
        </div>
        
        {isExpanded && (
          <div>
            {/* Render documents in this folder */}
            {folderDocs.map(doc => (
              <div 
                key={doc.id}
                className="flex items-center p-1 rounded cursor-pointer hover:bg-gray-200 dark:hover:bg-gray-700"
                style={{ paddingLeft: `${(depth + 1) * 16 + 8}px` }}
                onClick={() => onDocumentSelect && onDocumentSelect(doc.id)}
                draggable
                onDragStart={(e) => handleDragStart(e, 'document', doc.id)}
              >
                <span className="mr-2">
                  {doc.document_type === 'chapter' ? '📄' : '📝'}
                </span>
                <span>{doc.title}</span>
              </div>
            ))}
            
            {/* Render child folders */}
            {folder.children?.map(childFolder => renderFolder(childFolder, depth + 1))}
            
            {/* Add new item button */}
            <div 
              className="flex items-center p-1 text-blue-600 dark:text-blue-400 hover:underline cursor-pointer"
              style={{ paddingLeft: `${(depth + 1) * 16 + 8}px` }}
              onClick={() => {
                const name = prompt('Enter folder name:');
                if (name) createFolder(folder.id, name);
              }}
            >
              <span className="mr-1">+</span>
              <span className="text-sm">New Folder</span>
            </div>
          </div>
        )}
      </div>
    );
  };

  if (loading) {
    return <div className="p-4">Loading folder structure...</div>;
  }

  if (error) {
    return <div className="p-4 text-red-500">{error}</div>;
  }

  return (
    <div className="folder-hierarchy">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-lg font-semibold">Project Structure</h3>
        <button 
          className="text-sm bg-blue-500 hover:bg-blue-700 text-white px-2 py-1 rounded"
          onClick={() => {
            const name = prompt('Enter folder name:');
            if (name) createFolder(null, name);
          }}
        >
          New Root Folder
        </button>
      </div>
      
      <div className="border rounded-md bg-white dark:bg-gray-800 overflow-y-auto max-h-[500px]">
        {folders.length === 0 ? (
          <div className="p-4 text-gray-500">
            No folders yet. Create your first folder to organize your project.
          </div>
        ) : (
          folders.map(folder => renderFolder(folder))
        )}
      </div>
    </div>
  );
};

export default FolderHierarchy;
