import React from 'react';
import ProjectCard from './ProjectCard';

const ProjectList: React.FC = () => {
  // Placeholder data
  const projects = [
    { id: '1', name: 'My First Novel', description: 'A story about a brave adventurer.' },
    { id: '2', name: 'Sci-Fi Epic', description: 'A sprawling space opera.' },
    { id: '3', name: 'Fantasy World', description: 'A world of magic and monsters.' },
  ];

  return (
    <div>
      {projects.map((project) => (
        <ProjectCard key={project.id} project={project} />
      ))}
    </div>
  );
};

export default ProjectList;
