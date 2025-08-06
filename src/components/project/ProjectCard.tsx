import React from 'react';

interface Project {
  id: string;
  name: string;
  description: string;
}

interface ProjectCardProps {
  project: Project;
  isSelected?: boolean;
}

const ProjectCard: React.FC<ProjectCardProps> = ({ project, isSelected = false }) => {
  return (
    <div className={`bg-white dark:bg-gray-800 p-4 rounded-md mb-4 shadow-md transition-colors
      ${isSelected ? 'border-l-4 border-blue-500 dark:border-blue-400' : ''}
      hover:bg-gray-50 dark:hover:bg-gray-750 cursor-pointer`}
    >
      <h3 className="text-lg font-bold">{project.name}</h3>
      <p className="text-gray-600 dark:text-gray-400">{project.description}</p>
      {isSelected && (
        <div className="mt-2 text-xs text-blue-600 dark:text-blue-400">
          Selected
        </div>
      )}
    </div>
  );
};

export default ProjectCard;
