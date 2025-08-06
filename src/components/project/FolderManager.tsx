import React, { useState, useEffect } from 'react';
import { useStore } from '../../stores/projectStore';
import { invoke } from '@tauri-apps/api';
import { v4 as uuidv4 } from 'uuid';

// Define explicit types
interface Folder {
  id: number;
  name: string;
  children?: Folder[];
}

interface FolderManagerProps {
  projectId: number;
}

const FolderManager: React.FC<FolderManagerProps> = ({ projectId }) => {
  const [folders, setFolders] = useState<Folder[]>([]);
  const [newFolderName, setNewFolderName] = useState('');
  
  // Explicitly type store accesses
  const createFolder = useStore((state: any) => state.createFolder);
  const updateFolder = useStore((state: any) => state.updateFolder);
  const deleteFolder = useStore((state: any) => state.deleteFolder);

  useEffect(() => {
    const loadFolders = async () => {
      try {
        const result = await invoke('getProjectFolders', { projectId });
        setFolders(result as Folder[]);
      } catch (error) {
        console.error('Error loading folders:', error);
      }
    };
    
    loadFolders();
  }, [projectId]);

  const handleCreateFolder = async () => {
    if (!newFolderName.trim()) return;
    
    try {
      const newFolder = await createFolder({
        name: newFolderName,
        projectId,
        parentId: null,
      });
      
      setFolders([...folders, newFolder as Folder]);
      setNewFolderName('');
    } catch (error) {
      console.error('Error creating folder:', error);
    }
  };

  const handleRenameFolder = async (folderId: number, newName: string) => {
    try {
      const updatedFolder = await updateFolder(folderId, { name: newName });
      setFolders(folders.map(folder => 
        folder.id === folderId ? (updatedFolder as Folder) : folder
      ));
    } catch (error) {
      console.error('Error renaming folder:', error);
    }
  };

  const handleDeleteFolder = async (folderId: number) => {
    try {
      await deleteFolder(folderId);
      setFolders(folders.filter(folder => folder.id !== folderId));
    } catch (error) {
      console.error('Error deleting folder:', error);
    }
  };

  const handleDrop = async (droppedFolderId: number, targetFolderId: number | null) => {
    try {
      const updatedFolder = await invoke('moveFolder', {
        folderId: droppedFolderId,
        parentId: targetFolderId,
      });
      
      setFolders(folders.map(folder => 
        folder.id === droppedFolderId ? (updatedFolder as Folder) : folder
      ));
    } catch (error) {
      console.error('Error moving folder:', error);
    }
  };

  return (
    <div className="flex flex-col h-full p-4">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-xl font-semibold">Folders</h2>
        <button 
          onClick={handleCreateFolder}
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        >
          New Folder
        </button>
      </div>
      
      <div className="flex-1 overflow-y-auto">
        <ul className="space-y-2">
          {folders.map(folder => (
            <li 
              key={folder.id}
              className="relative p-2 border rounded cursor-move"
            >
              <div className="flex justify-between items-center">
                <input
                  type="text"
                  value={folder.name}
                  onChange={(e) => handleRenameFolder(folder.id, e.target.value)}
                  placeholder="Folder name" // Added placeholder for accessibility
                  className="flex-grow mr-4 p-1 border rounded"
                />
                <button
                  onClick={() => handleDeleteFolder(folder.id)}
                  className="text-red-500"
                >
                  Delete
                </button>
              </div>
              {folder.children && (
                <ul className="ml-4 space-y-1">
                  {folder.children.map((child: Folder) => (
                    <li key={child.id}>{child.name}</li>
                  ))}
                </ul>
              )}
            </li>
          ))}
        </ul>
      </div>
      
      <input
        type="text"
        value={newFolderName}
        onChange={(e) => setNewFolderName(e.target.value)}
        className="mt-4 p-2 border rounded"
        placeholder="New folder name..."
      />
    </div>
  );
};

export default FolderManager;
