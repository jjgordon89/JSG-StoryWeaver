import React, { useState } from 'react';
import ProjectList from '../../components/project/ProjectList';
import StoryBible from '../story-bible/components/react/StoryBible';
import { Book, FileText } from 'lucide-react';
import { useProjectContext } from '../../contexts/ProjectContext';

interface ProjectViewProps {
  onDocumentSelect?: (documentId: number) => void;
}

const ProjectView: React.FC<ProjectViewProps> = ({ onDocumentSelect }) => {
  const { selectedProjectId } = useProjectContext();
  const [activeTab, setActiveTab] = useState<'projects' | 'story-bible'>('projects');

  return (
    <div className="h-full flex flex-col">
      {/* Navigation Tabs */}
      <div className="flex border-b border-gray-300 dark:border-gray-600 mb-4">
        <button
          onClick={() => setActiveTab('projects')}
          className={`flex items-center px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
            activeTab === 'projects'
              ? 'border-blue-500 text-blue-600 dark:text-blue-400'
              : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
        >
          <FileText className="w-4 h-4 mr-2" />
          Projects
        </button>
        <button
          onClick={() => setActiveTab('story-bible')}
          className={`flex items-center px-4 py-2 text-sm font-medium border-b-2 transition-colors ${
            activeTab === 'story-bible'
              ? 'border-blue-500 text-blue-600 dark:text-blue-400'
              : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'
          }`}
        >
          <Book className="w-4 h-4 mr-2" />
          Story Bible
        </button>
      </div>

      {/* Content Area */}
      <div className="flex-1 overflow-hidden">
        {activeTab === 'projects' ? (
          <div>
            <h1 className="text-2xl font-bold mb-4">Projects</h1>
            <ProjectList onDocumentSelect={onDocumentSelect} />
          </div>
        ) : (
          <div className="h-full">
            {selectedProjectId ? (
              <StoryBible projectId={selectedProjectId} />
            ) : (
              <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
                <div className="text-center">
                  <Book className="w-12 h-12 mx-auto mb-4 opacity-50" />
                  <p className="text-lg font-medium mb-2">No Project Selected</p>
                  <p className="text-sm">Please select a project from the Projects tab to access the Story Bible.</p>
                </div>
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

export default ProjectView;
