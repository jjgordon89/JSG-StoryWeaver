import React from 'react';
import ProjectList from '../../components/project/ProjectList';

interface ProjectViewProps {
  onDocumentSelect?: (documentId: number) => void;
}

const ProjectView: React.FC<ProjectViewProps> = ({ onDocumentSelect }) => {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-4">Projects</h1>
      <ProjectList onDocumentSelect={onDocumentSelect} />
    </div>
  );
};

export default ProjectView;
