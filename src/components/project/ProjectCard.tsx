import React from 'react';

interface Project {
  id: string;
  name: string;
  description: string;
}

interface ProjectCardProps {
  project: Project;
}

const ProjectCard: React.FC<ProjectCardProps> = ({ project }) => {
  return (
    <div className="bg-white dark:bg-gray-800 p-4 rounded-md mb-4 shadow-md">
      <h3 className="text-lg font-bold">{project.name}</h3>
      <p className="text-gray-600 dark:text-gray-400">{project.description}</p>
    </div>
  );
};

export default ProjectCard;
