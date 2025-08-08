import React, { useState, useEffect } from 'react';
import ProjectCard from './ProjectCard';
import ProjectPreview from './ProjectPreview';
import { invoke } from '../../utils/tauriSafe';
import { useProjectContext } from '../../contexts/ProjectContext';

interface Project {
  id: string;
  name: string;
  description: string;
}

interface Document {
  id: number;
  title: string;
  document_type: string;
}

interface ProjectListProps {
  onDocumentSelect?: (documentId: number) => void;
}

const ProjectList: React.FC<ProjectListProps> = ({ onDocumentSelect }) => {
  const { selectedProjectId, setSelectedProjectId } = useProjectContext();
  const [projects, setProjects] = useState<Project[]>([]);
  const [documents, setDocuments] = useState<Document[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [previewProjectId, setPreviewProjectId] = useState<string | null>(null);

  // Fetch projects on component mount
  useEffect(() => {
    const fetchProjects = async () => {
      try {
        // In a real implementation, this would call the Tauri API
        // For now, using placeholder data
        setProjects([
          { id: '1', name: 'My First Novel', description: 'A story about a brave adventurer.' },
          { id: '2', name: 'Sci-Fi Epic', description: 'A sprawling space opera.' },
          { id: '3', name: 'Fantasy World', description: 'A world of magic and monsters.' },
        ]);
      } catch (error) {
        console.error('Error fetching projects:', error);
      }
    };

    fetchProjects();
  }, []);

  // Fetch documents when a project is selected
  useEffect(() => {
    if (!selectedProjectId) return;

    const fetchDocuments = async () => {
      setLoading(true);
      try {
        // In a real implementation, this would call the Tauri API
        // For now, using placeholder data based on project ID
        const projectDocs = [
          { id: 101, title: 'Chapter 1', document_type: 'chapter' },
          { id: 102, title: 'Chapter 2', document_type: 'chapter' },
          { id: 103, title: 'Character Notes', document_type: 'notes' },
        ];
        setDocuments(projectDocs);
      } catch (error) {
        console.error('Error fetching documents:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchDocuments();
  }, [selectedProjectId]);

  const handleProjectClick = (projectId: string) => {
    setSelectedProjectId(projectId === selectedProjectId ? null : projectId);
  };

  const handleDocumentClick = (documentId: number) => {
    if (onDocumentSelect) {
      onDocumentSelect(documentId);
    }
  };

  const handlePreviewClick = (projectId: string) => {
    setPreviewProjectId(projectId);
  };

  const handlePreviewClose = () => {
    setPreviewProjectId(null);
  };

  const handleOpenProject = () => {
    if (previewProjectId) {
      setSelectedProjectId(previewProjectId);
      setPreviewProjectId(null);
    }
  };

  return (
    <div className="space-y-4">
      {/* Projects list */}
      <div className="space-y-2">
        {projects.map((project) => (
          <div key={project.id} onClick={() => handleProjectClick(project.id)}>
            <ProjectCard 
              project={project} 
              isSelected={project.id === selectedProjectId}
              onPreview={handlePreviewClick}
            />
          </div>
        ))}
      </div>

      {/* Project Preview Modal */}
      {previewProjectId && (
        <ProjectPreview 
          projectId={previewProjectId}
          onClose={handlePreviewClose}
          onOpen={handleOpenProject}
        />
      )}

      {/* Documents list for selected project */}
      {selectedProjectId && (
        <div className="mt-4 border-t pt-4">
          <h3 className="text-lg font-semibold mb-2">Documents</h3>
          {loading ? (
            <p className="text-gray-500">Loading documents...</p>
          ) : (
            <ul className="space-y-1">
              {documents.map((doc) => (
                <li 
                  key={doc.id}
                  onClick={() => handleDocumentClick(doc.id)}
                  className="p-2 hover:bg-gray-300 dark:hover:bg-gray-700 rounded cursor-pointer flex items-center"
                >
                  <span className="text-xs bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200 px-2 py-1 rounded mr-2">
                    {doc.document_type}
                  </span>
                  <span>{doc.title}</span>
                </li>
              ))}
            </ul>
          )}
          <button 
            className="mt-2 text-sm text-blue-600 dark:text-blue-400 hover:underline"
            onClick={() => {
              // This would open a dialog to create a new document
              console.log('Create new document for project', selectedProjectId);
            }}
          >
            + New Document
          </button>
        </div>
      )}
    </div>
  );
};

export default ProjectList;
