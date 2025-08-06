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
        // In a real implementation, this would call the Tauri API
        // For now, using placeholder data
        const folderData: Folder[] = [
          { id: 'f1', name: 'Chapters', parent_folder_id: null, is_series: false, created_at: new Date().toISOString() },
          { id: 'f2', name: 'Characters', parent_folder_id: null, is_series: false, created_at: new Date().toISOString() },
          { id: 'f3', name: 'Act 1', parent_folder_id: 'f1', is_series: false, created_at: new Date().toISOString() },
          { id: 'f4', name: 'Act 2', parent_folder_id: 'f1', is_series: false, created_at: new Date().toISOString() },
          { id: 'f5', name: 'Act 3', parent_folder_id: 'f1', is_series: false, created_at: new Date().toISOString() },
          { id: 'f6', name: 'Main Characters', parent_folder_id: 'f2', is_series: false, created_at: new Date().toISOString() },
          { id: 'f7', name: 'Supporting Characters', parent_folder_id: 'f2', is_series: false, created_at: new Date().toISOString() },
        ];
        
        // Build folder hierarchy
        const rootFolders: Folder[] = [];
        const folderMap = new Map<string, Folder>();
        
        // First pass: create map of all folders
        folderData.forEach(folder => {
          folderMap.set(folder.id, { ...folder, children: [] });
        });
        
        // Second pass: build hierarchy
        folderData.forEach(folder => {
          const folderWithChildren = folderMap.get(folder.id)!;
          
          if (folder.parent_folder_id === null) {
            rootFolders.push(folderWithChildren);
          } else {
            const parent = folderMap.get(folder.parent_folder_id);
            if (parent) {
              parent.children = parent.children || [];
              parent.children.push(folderWithChildren);
            }
          }
        });
        
        setFolders(rootFolders);
        
        // Fetch documents for each folder
        const docsData: Record<string, Document[]> = {
          'f1': [],
          'f3': [
            { id: 'd1', title: 'Chapter 1: Beginning', document_type: 'chapter' },
            { id: 'd2', title: 'Chapter 2: Inciting Incident', document_type: 'chapter' },
          ],
          'f4': [
            { id: 'd3', title: 'Chapter 3: Rising Action', document_type: 'chapter' },
            { id: 'd4', title: 'Chapter 4: Midpoint', document_type: 'chapter' },
          ],
          'f5': [
            { id: 'd5', title: 'Chapter 5: Climax', document_type: 'chapter' },
            { id: 'd6', title: 'Chapter 6: Resolution', document_type: 'chapter' },
          ],
          'f6': [
            { id: 'd7', title: 'Protagonist', document_type: 'notes' },
            { id: 'd8', title: 'Antagonist', document_type: 'notes' },
          ],
          'f7': [
            { id: 'd9', title: 'Sidekick', document_type: 'notes' },
            { id: 'd10', title: 'Mentor', document_type: 'notes' },
          ],
        };
        
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

  // Create a new folder
  const createFolder = async (parentId: string | null, name: string) => {
    try {
      // In a real implementation, this would call the Tauri API
      console.log(`Creating folder "${name}" under parent ${parentId || 'root'}`);
      
      // Refresh folder structure
      // This would be replaced with actual API call and state update
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
        // In a real implementation, this would call the Tauri API to move the folder
        console.log(`Moving folder ${id} to folder ${targetFolderId}`);
      } else {
        // In a real implementation, this would call the Tauri API to move the document
        console.log(`Moving document ${id} to folder ${targetFolderId}`);
      }
      
      // Refresh folder structure
      // This would be replaced with actual API call and state update
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
