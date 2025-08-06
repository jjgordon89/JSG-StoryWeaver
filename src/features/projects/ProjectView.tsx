import React from 'react';
import ProjectList from '../../components/project/ProjectList';

const ProjectView: React.FC = () => {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Projects</h1>
      <ProjectList />
    </div>
  );
};

export default ProjectView;
