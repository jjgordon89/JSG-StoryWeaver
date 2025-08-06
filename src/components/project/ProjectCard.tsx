import React from 'react';

interface Project {
  id: string;
  name: string;
  description: string;
}

interface ProjectCardProps {
  project: Project;
  isSelected?: boolean;
  onPreview?: (projectId: string) => void;
}

const ProjectCard: React.FC<ProjectCardProps> = ({ project, isSelected = false, onPreview }) => {
  const handlePreviewClick = (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent triggering the parent click handler
    if (onPreview) {
      onPreview(project.id);
    }
  };

  return (
    <div className={`bg-white dark:bg-gray-800 p-4 rounded-md mb-4 shadow-md transition-colors
      ${isSelected ? 'border-l-4 border-blue-500 dark:border-blue-400' : ''}
      hover:bg-gray-50 dark:hover:bg-gray-750 cursor-pointer`}
    >
      <div className="flex justify-between items-start">
        <div>
          <h3 className="text-lg font-bold">{project.name}</h3>
          <p className="text-gray-600 dark:text-gray-400">{project.description}</p>
          {isSelected && (
            <div className="mt-2 text-xs text-blue-600 dark:text-blue-400">
              Selected
            </div>
          )}
        </div>
        <button 
          onClick={handlePreviewClick}
          className="text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 p-1"
          title="Preview Project"
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
            <path fillRule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clipRule="evenodd" />
          </svg>
        </button>
      </div>
    </div>
  );
};

export default ProjectCard;
